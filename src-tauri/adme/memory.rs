use rig::{
    Embed,
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::ollama,
    tool::ToolDyn,
    vector_store::{
        VectorStoreIndex, in_memory_store::InMemoryVectorStore, request::VectorSearchRequestBuilder,
    },
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{adme::agent::Agent, filters::filter_think_tag};

#[derive(Embed, Serialize, Deserialize, Clone, Default, Eq, PartialEq)]
pub struct MyDoc {
    id: String,
    #[embed]
    pub summary: String,
}

pub struct Memory {
    vector_store: Mutex<InMemoryVectorStore<MyDoc>>,
}

impl Memory {
    pub async fn n_closest_memories(
        &self,
        info: &str,
        n: u64,
    ) -> anyhow::Result<Vec<(f64, std::string::String, MyDoc)>> {
        let fastembed_client = rig_fastembed::Client::new();

        let embedding_model =
            fastembed_client.embedding_model(&rig_fastembed::FastembedModel::AllMiniLML6V2);

        let vector_search = VectorSearchRequestBuilder::default()
            .query(info)
            .samples(n)
            .build()?;

        let guard = self.vector_store.lock().await;

        let index = guard.clone().index(embedding_model.clone());

        let results = index.top_n::<MyDoc>(vector_search).await?;

        Ok(results)
    }

    pub async fn store_memory(&self, mem: &str) -> anyhow::Result<()> {
        println!("Storing memory: {}", mem);
        let fastembed_client = rig_fastembed::Client::new();

        let embedding_model =
            fastembed_client.embedding_model(&rig_fastembed::FastembedModel::AllMiniLML6V2);

        

        let id: String;

        let results = self.n_closest_memories(mem, 1).await?;

        //Get the id to write into
        const SIM_THRESHOLD: f64 = 0.85;

        let mut comb_mem;
        let mut guard = self.vector_store.lock().await;
        if !results.is_empty() && results[0].0 > SIM_THRESHOLD {
            let top = &results[0];
            println!("SIMILARITY IS: {}", top.0);
            println!("Replacing: {}", top.2.summary);
            id = top.1.clone();
            let combine_agent = ollama::Client::from_env().agent("qwen3:30b").preamble("Role: You are looking at a new memory and one of your current memories.\nTask: combine them into a single, concise memory that captures all essential information from both.\nInstructions:\n1. If you find any information conflicting, go with the more up to date info\n2. If there is little to nothing to gain by updating the same don't bother updating it").build();

            comb_mem = combine_agent
                .prompt(format!(
                    "New memory: {}\nCurrent memory: {}",
                    mem, top.2.summary
                ))
                .await?;

            comb_mem = filter_think_tag(&comb_mem);
            println!("with: {}", comb_mem);
        } else {
            comb_mem = mem.to_string();
            println!("Adding: {}", comb_mem);
            id = guard.len().to_string();
        }

        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
            .document(MyDoc {
                id,
                summary: comb_mem,
            })?
            .build()
            .await?;

        guard.add_documents_with_id_f(embeddings, |d| d.id.clone());

        println!("CURRENTLY {} MEMORIES IN STORAGE", guard.len());

        Ok(())
    }
}

impl Agent for Memory {
    fn new() -> Self {
        Self {
            vector_store: Mutex::new(InMemoryVectorStore::default()),
        }
    }

    async fn prompt(&self, input: &str, tools: Vec<Box<dyn ToolDyn>>) -> anyhow::Result<Option<String>> {
        let memory_agent = ollama::Client::from_env().agent("qwen3:30b")
        .preamble(
            "Role: You are a Memory Architect for a Sovereign AI.\nTask: Analyze the provided conversation and extract discrete, high signal \"Atomic Truths\". Call store_store memory with these truths.\nInstructions:\n1. Ignore filler, politeness, and temporary statements.\n2. Format each truth strictly as: Subject | Attribute | Value | Context | Rationale.\n3. Rationale must explain **why** this was concluded (e.g., \"User explicitly stated,\" or \"Inferred from repeated code patterns\").\n4. If a new truth contracdicts an old one, note it in the Context.\n5. Resolve all pronouns. Replace 'I/Me/My' with 'User' and 'You/Your' with 'Assistant'. Every truth must be an objective statement about a specific entity.\nExample\nInput:\"Actually, let's switch the 3D renderer to Vulkan. OpenGL is too slow for this geometry kernel.\"\nOutput:3D renderer | technology | Vulkan | Project Kernel Development | Switched from OpenGL due to performance bottlenecks in geometry processing.")
            .tools(tools)
            .build();

        let mut summary = filter_think_tag(&memory_agent.prompt(input).await?);
        Ok(None)
    }
}
