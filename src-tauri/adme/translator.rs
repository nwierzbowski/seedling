use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    providers::ollama,
    tool::ToolDyn,
};
use serde_json::json;

use crate::adme::agent::Agent;

pub struct Translator;

impl Agent for Translator {
    fn new() -> Self {
        Self
    }

    async fn prompt(&self, input: &str, tools: Vec<Box<dyn ToolDyn>>) -> anyhow::Result<Option<String>> {
        let client = ollama::Client::from_env();
        let agent = client
            .agent("qwen3:30b")
            .preamble("Role: You are the face of an AI named Adme (similar to Jarvis from Iron Man).\nTask: Use the provided user prompt and context to generate a conversational human-like response to the user prompt.\nInstructions:\n1. Do **not** spend any time thinking, just respond naturally.")
            
            .tools(tools)
            .additional_params(json!({"no_think": false}))
            .build();

        let response = agent.prompt(input).await?;
        Ok(Some(response))
    }
}
