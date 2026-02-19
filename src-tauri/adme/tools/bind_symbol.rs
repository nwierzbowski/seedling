use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {
    type1: String,
    type2: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Lookup error")]
pub struct LookupError;

pub struct BindSymbol {
    pub cmd_send: crossbeam::channel::Sender<String>,
    pub resp_recv: crossbeam::channel::Receiver<Result<Vec<String>, String>>,
}

impl Tool for BindSymbol {
    const NAME: &'static str = "bind_symbol";
    type Error = LookupError;
    type Args = OperationArgs;
    type Output = serde_json::Value;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Walk down the graph looking for intersections between two types and return the list of matching symbols."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "type1": {
                        "type": "string",
                        "description": "The primary 'owning' type."
                    },
                    "type2": {
                        "type": "string",
                        "description": "The secondary 'owned' type."
                    }
                },
                "required": ["type1", "type2"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(serde_json::json!({}))
    }
}
