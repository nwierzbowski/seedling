use std::sync::Arc;

use rig::agent::Agent;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::{Prompt};
use rig::providers::ollama;
use tokio::sync::Mutex;

pub struct RouterAgent {
    pub agent: Agent<ollama::CompletionModel>,
}

impl RouterAgent {
    pub fn new() -> Self {
        let client = ollama::Client::from_env();
        let planner_agent = client.agent("qwen3:30b").preamble("").build();
        Self {
            agent: planner_agent,
        }
    }

    pub async fn prompt(&self, input: &str) -> Result<String, String> {

        let response = self.agent
            .prompt(input)
            .await
            .map_err(|e| e.to_string())?;
        Ok(response)
    }
}

pub type SharedRouterAgent = Arc<Mutex<Option<RouterAgent>>>;
