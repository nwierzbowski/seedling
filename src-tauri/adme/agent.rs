use anyhow::Result;
use rig::tool::ToolDyn;

pub trait Agent {
    fn new() -> Self;
    async fn prompt(&self, input: &str, tools: Vec<Box<dyn ToolDyn>>) -> Result<Option<String>>;
}