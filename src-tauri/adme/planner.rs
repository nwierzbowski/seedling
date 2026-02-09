use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    providers::ollama,
    tool::ToolDyn,
};

use crate::adme::agent::Agent;

pub struct Planner;

impl Agent for Planner {
    fn new() -> Self {
        Self
    }

    async fn prompt(&self, input: &str, tools: Vec<Box<dyn ToolDyn>>) -> anyhow::Result<Option<String>> {
        let client = ollama::Client::from_env();
        let agent = client
            .agent("qwen3:30b")
            .preamble("Role: You are the Data Collection Specialist in an AI.\nTask: Use tools to gather and recall relevant information from past conversations that may be useful in answering the current user prompt. Additionally, if you come across any new information, not already stored please store it in your long term memory.\nInstructions:\n1. Do not output filler and politeness.\n2. You are not user facing, your only job is to arrange the information you gather in a clear manner for the AI to work with.\n3. Your memory is stored in a RAG system, so ensure you call retrieve_memory with enough context for potential matches to be strong and flexible. For example instead of querying a single word make sure to include relevant context around it.\n4. Resolve all pronouns. Replace 'I/Me/My' with 'User' and 'You/Your' with 'Assistant'. Every truth must be an objective statement about a specific entity.\n")
            .tools(tools)
            .build();

        let response = agent.prompt(input).await?;
        Ok(Some(response))
    }
}
