mod agent;
mod memory;
mod metta;
mod planner;
mod tools;
mod translator;

use std::sync::Arc;

use agent::Agent;
use anyhow::Error;
use planner::Planner;
use tokio::sync::Mutex;

use crate::adme::{
    memory::Memory,
    metta::spawn_metta_thread,
    tools::{AddLink, AddSymbol, BindSymbol, GetSymbols, Relate},
    translator::Translator,
};

#[derive(Clone)]
pub struct Adme {
    inner: Arc<Mutex<AdmeInner>>,
}

struct AdmeInner {
    planner: Planner,
    translator: Translator,
    memory: Arc<Memory>,
    cmd_sender: Option<crossbeam::channel::Sender<String>>,
    metta_recv: Option<crossbeam::channel::Receiver<Result<Vec<String>, String>>>,
    metta_handle: Vec<std::thread::JoinHandle<anyhow::Result<()>>>,
}

impl Adme {
    pub fn new() -> Self {
        let memory = Arc::new(memory::Memory::new());
        let (send, recv, handle) = match spawn_metta_thread() {
            Ok(result) => result,
            Err(e) => panic!("Failed to spawn metta thread: {}", e),
        };
        Self {
            inner: Arc::new(Mutex::new(AdmeInner {
                memory: memory.clone(),
                planner: Planner::new(),
                translator: Translator::new(),
                cmd_sender: Some(send),
                metta_recv: Some(recv),
                metta_handle: vec![handle],
            })),
        }
    }

    pub async fn shutdown(&self) {
        let mut guard = self.inner.lock().await;
        let handles = std::mem::take(&mut guard.metta_handle);
        drop(guard.cmd_sender.take());
        drop(guard.metta_recv.take());
        drop(guard);
        for handle in handles {
            handle.join().expect("Failed to join metta thread");
        }
    }

    

    pub async fn prompt(&self, prompt: &str) -> anyhow::Result<String> {
        let guard = self.inner.lock().await;

        let (sender, receiver) = match (guard.cmd_sender.as_ref(), guard.metta_recv.as_ref()) {
            (Some(sender), Some(receiver)) => (sender, receiver),
            _ => {
                return Err(Error::msg("Prompt called when Adme is not initialized"));
            }
        };

        let memory = guard.memory.clone();

        let known_symbols = get_symbols(sender.clone(), receiver.clone())?;

        let mut known_symbols_str = if known_symbols.is_empty() {
            "None".to_string()
        } else {
            known_symbols.join(", ")
        };

        known_symbols_str = format!("Known Symbols: {}", known_symbols_str);

        let mut response = match guard
            .planner
            .prompt(
                &format!("{}\n{}", known_symbols_str, prompt),
                vec![
                    // Box::new(RetrieveMemory {
                    //     memory: memory.clone(),
                    // }),
                    // Box::new(StoreMemory {
                    //     memory: memory.clone(),
                    // }),
                    Box::new(AddSymbol {
                        //Used to add a new instance symbol with a parent type
                        cmd_send: sender.clone(),
                        resp_recv: receiver.clone(),
                    }),
                    // Box::new(BindSymbol { //Takes two symbols as coords and walks down to find all intersection matches returning the list
                    //     cmd_send: sender.clone(),
                    //     resp_recv: receiver.clone(),
                    // }),
                    Box::new(AddLink {
                        //Associates two symbols with an owning relationship
                        cmd_send: sender.clone(),
                        resp_recv: receiver.clone(),
                    }),
                    Box::new(Relate {
                        cmd_send: sender.clone(),
                        resp_recv: receiver.clone(),
                    })
                    // Box::new(GetSymbols { //Gets all applicable aliases for graph symbols
                    //     cmd_send: sender.clone(),
                    //     resp_recv: receiver.clone(),
                    // }),
                ],
            )
            .await
        {
            Ok(Some(res)) => res,
            _ => String::from("Error processing prompt"),
        };

        // response = filter_think_tag(&response);

        // let processor_prompt = format!("Context: {}\nUser Prompt: {}", response, prompt);

        // response = match guard.translator.prompt(&processor_prompt, vec![]).await {
        //     Ok(Some(res)) => res,
        //     _ => String::from("Error processing prompt"),
        // };

        // let memory_entry = format!("User Prompt: {}\nAgent Response: {}", prompt, filter_think_tag(&response));

        // let _ = memory.prompt(&memory_entry, vec![Box::new(StoreMemory { memory: memory.clone() })]).await;
        Ok(response)
    }

    
}

pub fn get_symbols(
        cmd_send: crossbeam::channel::Sender<String>,
        resp_recv: crossbeam::channel::Receiver<Result<Vec<String>, String>>,
    ) -> anyhow::Result<Vec<String>> {
        let query = "!(match &self (Alias $symbol $string) $string)".to_string();
        // let query = "!(get-atoms &self)".to_string();

        // 2. Send the command
        cmd_send
            .send(query)
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        // 3. Receive and flatten the result logic
        match resp_recv.recv() {
            Ok(Ok(symbols)) => {
                println!("Received symbols: {:?}", symbols);
                Ok(symbols)
            }
            Ok(Err(metta_err)) => Err(anyhow::anyhow!("MeTTa Execution Error: {}", metta_err)),
            Err(recv_err) => Err(anyhow::anyhow!("Channel Communication Error: {}", recv_err)),
        }
    }
