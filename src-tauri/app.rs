//! Main application module for the seedling AI development environment.

use std::{env, sync::Arc};

use tauri::Manager;
use tokio::sync::Mutex;

// Import types we need
pub use terminal::TerminalState;

use crate::{hardware, process, router_agent::{RouterAgent, SharedRouterAgent}, telegram, terminal};

/// Application state for managing seedling's core components
pub struct AppState {
    /// Hardware manager for GPU configuration
    // hardware_manager: Arc<Mutex<hardware::NvSmiLockManager>>,
    /// Process manager for AI servers
    process_manager: Arc<Mutex<process::ProcessManager>>,
    router_agent: SharedRouterAgent,
}

pub type ManagedState = Arc<AppState>;

impl AppState {
    pub fn new() -> Self {
        Self {
            // hardware_manager: Arc::new(Mutex::new(hardware::NvSmiLockManager::new())),
            process_manager: Arc::new(Mutex::new(process::ProcessManager::new())),
            router_agent: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting seedling AI development environment...");

        let agent = RouterAgent::new();
        *self.router_agent.lock().await = Some(agent);

        // Initialize hardware components
        self.initialize_hardware().await?;

        // Start process management
        self.start_processes().await?;

        Ok(())
    }

    /// Initializes hardware components including GPU configuration.
    async fn initialize_hardware(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🛡️ Initializing hardware components...");

        // Engage safety locks and configure GPU settings
        hardware::GpuGuard::engage_safety_locks()
            .map_err(|e| anyhow::anyhow!("Failed to engage safety locks: {}", e))?;

        println!("✅ Hardware components initialized successfully");
        Ok(())
    }

    /// Starts the AI processes (llama-swap).
    async fn start_processes(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚙️ Starting AI processes...");

        // Start llama-swap server
        // self.process_manager.lock().await.start_llama_swap().await?;
        self.process_manager.lock().await.start_ollama().await?;

        println!("✅ AI processes started successfully");
        Ok(())
    }
}

/// Main application entry point
pub fn run() {
    let app_state: ManagedState = Arc::new(AppState::new());
    let terminal_state = TerminalState(Arc::new(std::sync::Mutex::new(
        terminal::TerminalStateData::default(),
    )));

    tauri::Builder::default()
        .manage(app_state.clone())
        .manage(app_state.router_agent.clone())
        .manage(terminal_state.clone())
        .invoke_handler(tauri::generate_handler![
            // get_status,
            // switch_agent,
            // execute_command,
            terminal::write_to_buffer,
            terminal::resize_pty
        ])
        .setup(|app| {
            println!("🚀 Initializing Tauri application...");
            println!("✅ Desktop interface ready");

            
            println!("✅ OpenAI environment variables set");

            // let handle = app.handle().clone();

            let state = app.state::<ManagedState>().inner().clone();
            // let terminal = app.state::<TerminalState>().inner().clone();



            tauri::async_runtime::spawn(async move {
                if let Err(e) = state.init().await {
                    eprintln!("❌ Application initialization failed: {}", e);
                    return;
                };
                telegram::start(state.router_agent.clone()).await;
                // terminal::start(handle, terminal.clone());
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let app_state = window.state::<ManagedState>().inner().clone();

                tauri::async_runtime::block_on(async move {
                    let mut manager = app_state.process_manager.lock().await;
                    if let Err(e) = manager.stop_all().await {
                        eprintln!("❌ Failed to stop processes during shutdown: {}", e);
                    }
                })
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("🏁 Seedling application shutdown complete.");
}
