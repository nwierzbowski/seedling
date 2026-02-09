use std::sync::Arc;

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize};
use serde_json::json;

use crate::adme::memory::Memory;

#[derive(Deserialize)]
pub struct OperationArgs {
    info: String
}

#[derive(Debug, thiserror::Error)]
#[error("Lookup error")]
pub struct LookupError;

pub struct RetrieveMemory {
    pub memory: Arc<Memory>
}

impl Tool for RetrieveMemory {
    const NAME: &'static str = "retrieve_memory";
    type Error = LookupError;
    type Args = OperationArgs;
    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "retrieve_memory".to_string(),
            description: "Search your long term memories of past conversations related to the provided query string. Use this when the user mentions something not in the current conversation.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "info": {
                        "type": "string",
                        "description": "The search term or topic to remember."
                    }
                },
                "required": ["info"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Retrieving memory for query: {}", args.info);
        let result = match self.memory.n_closest_memories(&args.info, 2).await {
            Ok(res) => res,
            Err(_) => return Err(LookupError),
        };
        Ok(result.iter().map(|f| {f.2.summary.clone()}).collect::<Vec<_>>())
    }
}