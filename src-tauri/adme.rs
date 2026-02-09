mod agent;
mod memory;
mod planner;
mod tools;
mod translator;

use std::sync::Arc;

use agent::Agent;
use planner::Planner;
use tokio::sync::Mutex;

use crate::{
    adme::{
        memory::Memory,
        tools::{RetrieveMemory, StoreMemory},
        translator::Translator,
    },
    filters::filter_think_tag,
};

#[derive(Clone)]
pub struct Adme {
    inner: Arc<Mutex<AdmeInner>>,
}

struct AdmeInner {
    planner: Planner,
    translator: Translator,
    memory: Arc<Memory>,
}

impl Adme {
    pub fn new() -> Self {
        let memory = Arc::new(memory::Memory::new());
        Self {
            inner: Arc::new(Mutex::new(AdmeInner {
                memory: memory.clone(),
                planner: Planner::new(),
                translator: Translator::new(),
            })),
        }
    }

    pub async fn prompt(&self, prompt: &str) -> String {
        let guard = self.inner.lock().await;
        let memory = guard.memory.clone();

        let mut response = match guard
            .planner
            .prompt(
                prompt,
                vec![
                    Box::new(RetrieveMemory {
                        memory: memory.clone(),
                    }),
                    Box::new(StoreMemory {
                        memory: memory.clone(),
                    }),
                ],
            )
            .await
        {
            Ok(Some(res)) => res,
            _ => String::from("Error processing prompt"),
        };

        response = filter_think_tag(&response);

        let processor_prompt = format!("Context: {}\nUser Prompt: {}", response, prompt);

        response = match guard.translator.prompt(&processor_prompt, vec![]).await {
            Ok(Some(res)) => res,
            _ => String::from("Error processing prompt"),
        };

        // let memory_entry = format!("User Prompt: {}\nAgent Response: {}", prompt, filter_think_tag(&response));

        // let _ = memory.prompt(&memory_entry, vec![Box::new(StoreMemory { memory: memory.clone() })]).await;
        response
    }
}
