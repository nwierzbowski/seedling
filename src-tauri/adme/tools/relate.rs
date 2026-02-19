use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {
    rel: String,
    host1: String,
    host2: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Lookup error")]
pub struct LookupError;

pub struct Relate {
    pub cmd_send: crossbeam::channel::Sender<String>,
    pub resp_recv: crossbeam::channel::Receiver<Result<Vec<String>, String>>,
}

impl Tool for Relate {
    const NAME: &'static str = "relate";
    type Error = LookupError;
    type Args = OperationArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Add a relationship between two symbols."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "rel": {
                        "type": "string",
                        "description": "The relationship added between the two symbols."
                    },
                    "host1": {
                        "type": "string",
                        "description": "The higher priority symbol in the relationship."
                    },
                    "host2": {
                        "type": "string",
                        "description": "The lower priority symbol in the relationship."
                    }
                },
                "required": ["host1", "host2", "rel"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let metta_cmd = format!(r#"!(relate &{} "{}" &{})"#, args.rel, args.host1, args.host2);
        let results = match self.cmd_send.send(metta_cmd) {
            Ok(_) => Ok(format!("Successfully added {} between {} and {}", args.rel, args.host1, args.host2).to_string()),
            Err(_) => Err(LookupError),
        };

        self.resp_recv.recv();
        results
    }
}
