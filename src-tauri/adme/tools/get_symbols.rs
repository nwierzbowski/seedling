use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {}

#[derive(Debug, thiserror::Error)]
#[error("Lookup error")]
pub struct LookupError;

pub struct GetSymbols {
    pub cmd_send: crossbeam::channel::Sender<String>,
    pub resp_recv: crossbeam::channel::Receiver<Result<Vec<String>, String>>,
}

impl Tool for GetSymbols {
    const NAME: &'static str = "get_symbols";
    type Error = LookupError;
    type Args = OperationArgs;
    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Find aliases for all available symbols".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                },
                "required": []
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let query = "!(match &self (Alias $symbol $string) $string)".to_string();

        // 2. Send the command
        self.cmd_send.send(query).map_err(|_| LookupError)?;

        // 3. Receive and flatten the result logic
        match self.resp_recv.recv() {
            Ok(Ok(symbols)) => {
                println!("Received symbols: {:?}", symbols);
                Ok(symbols)
            }
            Ok(Err(metta_err)) => {
                eprintln!("MeTTa Execution Error: {}", metta_err);
                Err(LookupError)
            }
            Err(recv_err) => {
                eprintln!("Channel Communication Error: {}", recv_err);
                Err(LookupError)
            }
        }
    }
}


