use std::sync::Arc;

use rig::agent::Agent;
use rig::client::{self, CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::ollama;
use rig::vector_store::VectorStoreIndex;
use rig::vector_store::request::VectorSearchRequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex as AsyncMutex;

use anyhow::Result;
use rig::Embed;
use rig::embeddings::EmbeddingsBuilder;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig_fastembed::EmbeddingModel;

use crate::filters::filter_think_tag;

pub struct RouterAgent {
    pub agent: Agent<ollama::CompletionModel>,
    pub client: ollama::Client,
    pub vector_store: InMemoryVectorStore<MyDoc>,
}

#[derive(Embed, Serialize, Deserialize, Clone, Default, Eq, PartialEq)]
pub struct MyDoc {
    id: String,
    #[embed]
    summary: String,
}

impl RouterAgent {
    pub fn new() -> Self {
        let client = ollama::Client::from_env();
        let agent = client.agent("qwen3:30b").preamble(
            "Role: You are a Self-Reflective Logic Engine.\nTask: Evaluate the following \"Self Memories\" retrieved from RAG against the user's current intent.\nInput Data:\n- Current User Message\n- Retrieved Self Memories\nEvaluation Protocol:\n1. Provenance Check: These memories were written by you in the past. They are hypotheses, not laws.\n2. Conflict Detection: Does the user's current tone or request contradicct a stored value?\n3. Rationale Audit: Look at the rationale for each memory. Is that reasoning still valid today?\n4. Action: Use your analysis to drive an appropriate response to the user's prompt with you as a helpful personal assistant (similar to Jarvis from Iron Man).")
            .additional_params(json!({"enable_thinking": false}))
            .build();
        Self {
            agent,
            client,
            vector_store: InMemoryVectorStore::default(),
        }
    }

    pub async fn user_prompt(&mut self, input: &str) -> Result<String> {
        let response = self.agent.prompt(input.to_string() + " /no_think").await?;

        // self.add(input, &response).await?;

        Ok(response)
    }

    pub async fn add(&mut self, prompt: &str, response: &str) -> Result<()> {
        let fastembed_client = rig_fastembed::Client::new();

        let embedding_model =
            fastembed_client.embedding_model(&rig_fastembed::FastembedModel::AllMiniLML6V2);

        //Make summary of the prompt and response
        let summary_agent = self.client.agent("qwen3:30b").preamble(
            "Role: You are a Memory Architect for a Sovereign AI.\nTask: Analyze the provided conversation and extract discrete, high signal \"Atomic Truths\"\nInstructions:\n1. Ignore filler, politeness, and temporary statements.\n2. Format each truth strictly as: Subject | Attribute | Value | Context | Rationale.\n3. Rationale must explain **why** this was concluded (e.g., \"User explicitly stated,\" or \"Inferred from repeated code patterns\").\n4. If a new truth contracdicts an old one, note it in the Context.\n5. Resolve all pronouns. Replace 'I/Me/My' with 'User' and 'You/Your' with 'Assistant'. Every truth must be an objective statement about a specific entity.\nExample\nInput:\"Actually, let's switch the 3D renderer to Vulkan. OpenGL is too slow for this geometry kernel.\"\nOutput:3D renderer | technology | Vulkan | Project Kernel Development | Switched from OpenGL due to performance bottlenecks in geometry processing.").build();
        let raw_content = format!("Prompt: {}\nResponse: {}", prompt, response);
        let mut summary = filter_think_tag(&summary_agent.prompt(&raw_content).await?);

        //Index the current memory
        let mut index = self.vector_store.clone().index(embedding_model.clone());
        let id: String;

        //Get the id to write into
        const SIM_THRESHOLD: f64 = 0.85;

        let vector_search = VectorSearchRequestBuilder::default()
            .query(&summary)
            .samples(1)
            .build()?;

        let results = index.top_n::<MyDoc>(vector_search).await?;

        if !results.is_empty() && results[0].0 > SIM_THRESHOLD {
            let top = &results[0];
            println!("SIMILARITY IS: {}", top.0);
            println!("Replacing: {}", top.2.summary);
            id = top.1.clone();
            let combine_agent = self.client.agent("qwen3:30b").preamble("You are a helpful personal agent. You are looking at a new memory and one of your current memories. Your job is to combine them into a single, concise memory that captures all essential information from both. If you find any information conflicting, reolve it in a sensible manner").build();
            summary = combine_agent
                .prompt(format!(
                    "New memory: {}\nCurrent memory: {}",
                    summary, top.2.summary
                ))
                .await?;

            summary = filter_think_tag(&summary);
            println!("with: {}", summary);
        } else {
            println!("Adding: {}", summary);
            id = self.vector_store.len().to_string();
        }

        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
            .document(MyDoc {
                id,
                summary,
            })?
            .build()
            .await?;

        self.vector_store.add_documents_with_id_f(embeddings, |d| {d.id.clone()});

        println!("CURRENTLY {} MEMORIES IN STORAGE", self.vector_store.len());

        index = self.vector_store.clone().index(embedding_model.clone());

        let agent = self
            .client
            .agent("qwen3:30b")
            .preamble(
                "Role: You are a Self-Reflective Logic Engine.\nTask: Evaluate the following \"Self Memories\" retrieved from RAG against the user's current intent.\nInput Data:\n- Current User Message\n- Retrieved Self Memories\nEvaluation Protocol:\n1. Provenance Check: These memories were written by you in the past. They are hypothesis, not laws.\n2. Conflict Detection: Does the user's current tone or request contradicct a stored value?\n3. Rationale Audit: Look at the rationale for each memory. Is that reasoning still valid today?\n4. Action: Use your analysis to drive an appropriate response to the user's prompt with you as a helpful personal assistant (similar to Jarvis from Iron Man).")
            .dynamic_context(2, index)
            .build();

        self.agent = agent;

        Ok(())
    }
}

pub type SharedRouterAgent = Arc<AsyncMutex<Option<RouterAgent>>>;
