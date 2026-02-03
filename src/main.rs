//! Main orchestrator module for the seedling AI development environment.
//!
//! This is the event loop that coordinates between the hardware, process,
//! and tmux modules to manage the complete AI development workflow.

mod hardware;
mod process;
mod tmux;

use std::error::Error;
use tokio::signal;
use anyhow;

/// Main application struct that orchestrates all modules.
pub struct SeedlingApp {
    /// Hardware manager for GPU configuration
    hardware_manager: hardware::NvSmiLockManager,
    /// Process manager for AI servers
    process_manager: process::ProcessManager,
    /// Tmux manager for terminal layout
    tmux_manager: tmux::TmuxManager,
}

impl SeedlingApp {
    /// Creates a new seedling application instance.
    pub fn new() -> Self {
        Self {
            hardware_manager: hardware::NvSmiLockManager::new(),
            process_manager: process::ProcessManager::new(),
            tmux_manager: tmux::TmuxManager::new("seedling-session"),
        }
    }

    /// Main event loop that coordinates all modules.
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("🚀 Starting seedling AI development environment...");

        // Initialize hardware components
        self.initialize_hardware().await?;

        // Start process management
        self.start_processes().await?;

        // Setup tmux layout and agent communication
        self.setup_tmux_layout().await?;

        // Main event loop for monitoring and coordination
        self.event_loop().await?;

        Ok(())
    }

    /// Initializes hardware components including GPU configuration.
    async fn initialize_hardware(&mut self) -> Result<(), Box<dyn Error>> {
        println!("🛡️ Initializing hardware components...");

        // Engage safety locks and configure GPU settings
        hardware::GpuGuard::engage_safety_locks()
            .map_err(|e| anyhow::anyhow!("Failed to engage safety locks: {}", e))?;

        println!("✅ Hardware components initialized successfully");
        Ok(())
    }

    /// Starts the AI processes (llama-swap).
    async fn start_processes(&mut self) -> Result<(), Box<dyn Error>> {
        println!("⚙️ Starting AI processes...");

        // Start llama-swap server
        self.process_manager.start_llama_swap().await?;

        println!("✅ AI processes started successfully");
        Ok(())
    }

    /// Sets up the tmux terminal layout with agent panes.
    async fn setup_tmux_layout(&mut self) -> Result<(), Box<dyn Error>> {
        println!("🖥️ Setting up tmux layout...");

        // Create and configure tmux session
        self.tmux_manager.create_session().await?;

        // Setup 3 pane layout for agents
        self.tmux_manager.setup_layout().await?;

        println!("✅ Tmux layout set up successfully");
        Ok(())
    }

    /// Main event loop that monitors the system and coordinates modules.
    async fn event_loop(&mut self) -> Result<(), Box<dyn Error>> {
        println!("🤖 Entering main event loop...");
        println!("   Press Ctrl+C to shut down safely");

        // Monitor for shutdown signals
        let ctrl_c = signal::ctrl_c();
        tokio::pin!(ctrl_c);

        loop {
            tokio::select! {
                _ = ctrl_c.as_mut() => {
                    println!("🛑 Received shutdown signal, cleaning up...");
                    self.cleanup().await?;
                    break;
                }
                // Add other event handling here
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    // Periodic maintenance tasks including process monitoring
                    self.periodic_maintenance().await?;
                    // Monitor processes for health and restarts
                    self.process_manager.monitor_processes().await?;
                }
            }
        }

        Ok(())
    }

    /// Performs periodic maintenance tasks.
    async fn periodic_maintenance(&mut self) -> Result<(), Box<dyn Error>> {
        // Implement periodic checks and maintenance here
        println!("🔄 Performing periodic maintenance...");

        // Check GPU idle status
        if hardware::GpuGuard::is_gpu_idle() {
            println!("✅ GPU is idle, ready for new tasks");
        } else {
            println!("📈 GPU is currently busy");
        }

        Ok(())
    }

    /// Cleans up all resources when shutting down.
    async fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        println!("🧹 Cleaning up resources...");

        // Stop all processes
        self.process_manager.stop_all().await?;

        // Release hardware locks
        // Note: The lock release is handled by the hardware manager's drop implementation

        println!("✅ Cleanup completed successfully");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🌱 Seedling AI Development Environment");
    println!("========================================");

    let mut app = SeedlingApp::new();
    app.run().await?;

    println!("🏁 Seedling application shutdown complete.");
    Ok(())
}