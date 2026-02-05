//! Main orchestrator module for the seedling AI development environment.
//!
//! This is the Tauri-based desktop application entry point that provides
//! a sophisticated single-window desktop interface replacing the tmux terminal layout.
//!
//! The system maintains all core functionality:
//! - Hardware safety protocols (GPU persistence mode, power limits, clock locking)
//! - Process lifecycle management (llama-swap server)
//! - Agent switching capability in a single window interface

mod hardware;
mod process;

use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::{error::Error, sync::Arc};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::Mutex;

struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn std::io::Write + Send>,
}
#[derive(Clone)]
struct TerminalState(Arc<std::sync::Mutex<Option<PtySession>>>);

fn start_claude_pty(app: AppHandle, state: TerminalState) {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            ..Default::default()
        })
        .unwrap();

    let mut cmd = CommandBuilder::new("claude");
    cmd.args(&["--model", "engineer"]);
    cmd.cwd("/home/nwier/Documents/git/seedling");
    cmd.env("ANTHROPIC_BASE_URL", "http://localhost:8081/v1");
    pair.slave
        .spawn_command(cmd)
        .expect("Failed to start Claude");
    drop(pair.slave); // Close the slave end in the parent

    let reader = pair
        .master
        .try_clone_reader()
        .expect("Failed to clone reader.");
    let writer = pair.master.take_writer().expect("Failed to take writer.");

    let mut session = state.0.lock().unwrap();
    *session = Some(PtySession {
        master: pair.master,
        writer,
    });

    std::thread::spawn(move || {
        let mut reader = reader;
        let mut buffer = [0u8; 1024];
        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 {
                break;
            }

            app.emit("pty-data", String::from_utf8_lossy(&buffer[..n]))
                .unwrap();
        }
    });
}

#[tauri::command]
fn backend_write_pty(data: String, state: State<'_, TerminalState>) {
    let mut state_guard = state.0.lock().unwrap();
    if let Some(session) = state_guard.as_mut() {
        let _ = session.writer.write_all(data.as_bytes());
        let _ = session.writer.flush();
    }
}

#[tauri::command]
fn resize_pty(rows: u16, cols: u16, state: tauri::State<'_, TerminalState>) {
    let mut state_guard = state.0.lock().unwrap();
    if let Some(session) = state_guard.as_mut() {
        let size = PtySize {
            rows,
            cols,
            ..Default::default()
        };
        session.master.resize(size).unwrap();
    }
}

/// Application state for managing seedling's core components
pub struct AppState {
    /// Hardware manager for GPU configuration
    // hardware_manager: Arc<Mutex<hardware::NvSmiLockManager>>,
    /// Process manager for AI servers
    process_manager: Arc<Mutex<process::ProcessManager>>,
}

pub type ManagedState = Arc<AppState>;

impl AppState {
    pub fn new() -> Self {
        Self {
            // hardware_manager: Arc::new(Mutex::new(hardware::NvSmiLockManager::new())),
            process_manager: Arc::new(Mutex::new(process::ProcessManager::new())),
        }
    }

    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        println!("🚀 Starting seedling AI development environment...");

        // Initialize hardware components
        self.initialize_hardware().await?;

        // Start process management
        self.start_processes().await?;

        // Main event loop for monitoring and coordination
        // self.event_loop().await?;

        Ok(())
    }

    /// Initializes hardware components including GPU configuration.
    async fn initialize_hardware(&self) -> Result<(), Box<dyn Error>> {
        println!("🛡️ Initializing hardware components...");

        // Engage safety locks and configure GPU settings
        hardware::GpuGuard::engage_safety_locks()
            .map_err(|e| anyhow::anyhow!("Failed to engage safety locks: {}", e))?;

        println!("✅ Hardware components initialized successfully");
        Ok(())
    }

    /// Starts the AI processes (llama-swap).
    async fn start_processes(&self) -> Result<(), Box<dyn Error>> {
        println!("⚙️ Starting AI processes...");

        // Start llama-swap server
        self.process_manager.lock().await.start_llama_swap().await?;

        println!("✅ AI processes started successfully");
        Ok(())
    }
}

/// Tauri command handlers for frontend communication
// #[tauri::command]
// async fn get_status(state: State<'_, AppState>) -> Result<String, String> {
//     let gpu = state.gpu_status.lock().map_err(|e| e.to_string())?;
//     let agent = state.active_agent.lock().map_err(|e| e.to_string())?;
//     let process = state.process_status.lock().map_err(|e| e.to_string())?;

//     Ok(format!(
//         "GPU: {}, Agent: {}, Process: {}",
//         *gpu, *agent, *process
//     ))
// }

// #[tauri::command]
// async fn switch_agent(agent: String, state: State<'_, AppState>) -> Result<String, String> {
//     state.switch_agent(agent.clone());
//     Ok(format!("Switched to agent: {}", agent))
// }

// #[tauri::command]
// async fn execute_command(agent: String, command: String) -> Result<String, String> {
//     // For now, simulate command execution with agent-specific responses
//     let response = match agent.as_str() {
//         "engineer" => {
//             if command.contains("test") {
//                 "✓ Running tests... All tests passed!".to_string()
//             } else if command.contains("build") {
//                 "⚙️ Building project... Build successful!".to_string()
//             } else {
//                 format!("[engineer] Executing: {}", command)
//             }
//         }
//         "tester" => {
//             if command.contains("test") {
//                 "🧪 Running test suite... 145 tests passed, 0 failed".to_string()
//             } else {
//                 format!("[tester] Validating: {}", command)
//             }
//         }
//         "auditor" => {
//             if command.contains("audit") || command.contains("check") {
//                 "🔍 Security audit complete. No issues found.".to_string()
//             } else {
//                 format!("[auditor] Reviewing: {}", command)
//             }
//         }
//         _ => format!("Unknown agent: {}", agent),
//     };

//     Ok(response)
// }

/// Main application entry point
fn main() {
    println!("AIDE (Tauri Desktop Version)");
    println!("=========================================");

    let app_state: ManagedState = Arc::new(AppState::new());
    let terminal_state = TerminalState(Arc::new(std::sync::Mutex::new(None)));

    tauri::Builder::default()
        .manage(app_state.clone())
        .manage(terminal_state.clone())
        .invoke_handler(tauri::generate_handler![
            // get_status,
            // switch_agent,
            // execute_command,
            backend_write_pty,
            resize_pty
        ])
        .setup(|app| {
            println!("🚀 Initializing Tauri application...");
            println!("✅ Desktop interface ready");
            let handle = app.handle().clone();

            let state = app.state::<ManagedState>().inner().clone();
            let terminal = app.state::<TerminalState>().inner().clone();

            tauri::async_runtime::spawn(async move {
                if let Err(e) = state.init().await {
                    eprintln!("❌ Application initialization failed: {}", e);
                    return;
                };
                start_claude_pty(handle, terminal.clone());
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
        // .plugin(tauri_plugin_pty::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("🏁 Seedling application shutdown complete.");
}
