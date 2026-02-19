use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {
    parent: String,
    new_symbol: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Lookup error")]
pub struct LookupError;

pub struct AddSymbol {
    pub cmd_send: crossbeam::channel::Sender<String>,
    pub resp_recv: crossbeam::channel::Receiver<Result<Vec<String>, String>>,
}

impl Tool for AddSymbol {
    const NAME: &'static str = "add_symbol";
    type Error = LookupError;
    type Args = OperationArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Make a new symbol typed by the provided parent symbol.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "parent": {
                        "type": "string",
                        "description": "The parent symbol to attach to."
                    },
                    "new_symbol": {
                        "type": "string",
                        "description": "The new symbol to be added."
                    }
                },
                "required": ["parent", "new_symbol"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {

        let add_atom = format!(r#"!(add_symbol &{} "{}" &{})"#, args.new_symbol, args.new_symbol, args.parent);

        let metta_cmd = [add_atom].concat();

        let result = match self.cmd_send.send(metta_cmd) {
            Ok(_) => Ok(format!("Successfully added {} as a child of {}", args.new_symbol, args.parent).to_string()),
            Err(_) => Err(LookupError),
        };
        self.resp_recv.recv();

        result
        // match  {
            // Ok(Ok(symbols)) => {
            //     println!("Received symbols: {:?}", symbols);
            //     Ok(symbols)
            // }
            // Ok(Err(metta_err)) => Err(anyhow::anyhow!("MeTTa Execution Error: {}", metta_err)),
            // Err(recv_err) => Err(anyhow::anyhow!("Channel Communication Error: {}", recv_err)),
        // }
    }
}
