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

pub struct StoreMemory {
    pub memory: Arc<Memory>
}

impl Tool for StoreMemory {
    const NAME: &'static str = "store_memory";
    type Error = LookupError;
    type Args = OperationArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "store_memory".to_string(),
            description: "Store the provided query string in your long term memory. Use this when you see a piece of information that should be remembered for future reference.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "info": {
                        "type": "string",
                        "description": "The information to store in long term memory."
                    }
                },
                "required": ["info"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = self.memory.store_memory(&args.info).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(LookupError)
        }
    }
}