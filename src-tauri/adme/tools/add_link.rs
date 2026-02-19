use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {
    parent: String,
    rel: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Lookup error")]
pub struct LookupError;

pub struct AddLink {
    pub cmd_send: crossbeam::channel::Sender<String>,
    pub resp_recv: crossbeam::channel::Receiver<Result<Vec<String>, String>>,
}

impl Tool for AddLink {
    const NAME: &'static str = "add_link";
    type Error = LookupError;
    type Args = OperationArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Link a child relationship to a parent relationship as a super"
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "parent": {
                        "type": "string",
                        "description": "The parent relationship to attach to."
                    },
                    "rel": {
                        "type": "string",
                        "description": "The new relationship to be added."
                    }
                },
                "required": ["parent", "rel"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let metta_cmd = format!(r#"!(add_link &{} "{}" &{})"#, args.rel, args.rel, args.parent);
        let results = match self.cmd_send.send(metta_cmd) {
            Ok(_) => Ok(format!("Successfully added {} as a child of {}", args.rel, args.parent).to_string()),
            Err(_) => Err(LookupError),
        };

        self.resp_recv.recv();
        results
    }
}
