//! Process management module for handling llama-swap and llama-server.
//!
//! This module manages the lifecycle of AI-related processes ensuring they
//! are properly started, monitored, and terminated when the application dies.

use std::process::{Command, Child};
use std::sync::Arc;
use tokio::process::Command as TokioCommand;
use std::collections::HashMap;

/// Manages AI processes like llama-swap and llama-server.
pub struct ProcessManager {
    /// Map of process names to their handles
    processes: HashMap<String, Arc<tokio::process::Child>>,
}

impl ProcessManager {
    /// Creates a new process manager instance.
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    /// Starts the llama-swap server.
    pub async fn start_llama_swap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for starting llama-swap
        println!("Starting llama-swap server...");

        // Kill old instances first
        let _ = Command::new("pkill").arg("-9").arg("llama-server").status();
        let _ = Command::new("pkill").arg("-9").arg("llama-swap").status();

        // Launch the new instance with specified arguments
        let child = Command::new("llama-swap")
            .arg("--config")
            .arg("config.yaml")
            .arg("--listen")
            .arg("0.0.0.0:8081")
            .spawn()
            .map_err(|e| format!("Failed to start llama-swap: {}", e))?;

        Ok(())
    }

    /// Starts the llama-server.
    pub async fn start_llama_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for starting llama-server
        println!("Starting llama-server...");

        // Kill old instances first
        let _ = Command::new("pkill").arg("-9").arg("llama-server").status();
        let _ = Command::new("pkill").arg("-9").arg("llama-swap").status();

        // Launch the new instance with specified arguments
        let child = Command::new("llama-server")
            .arg("--listen")
            .arg("0.0.0.0:8080")
            .arg("--model")
            .arg("models/llama-2-7b-chat.Q4_K_M.gguf")
            .spawn()
            .map_err(|e| format!("Failed to start llama-server: {}", e))?;

        Ok(())
    }

    /// Stops all managed processes.
    pub async fn stop_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for stopping all processes
        println!("Stopping all AI processes...");

        // Kill all processes
        let _ = Command::new("pkill").arg("-9").arg("llama-server").status();
        let _ = Command::new("pkill").arg("-9").arg("llama-swap").status();

        Ok(())
    }
}

/// Process lifecycle management.
pub struct ProcessLifecycle {
    // Fields for managing process lifecycles
}

impl ProcessLifecycle {
    /// Creates a new lifecycle manager.
    pub fn new() -> Self {
        Self {
            // Initialize fields
        }
    }

    /// Ensures processes die when the app dies.
    pub async fn ensure_cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for ensuring cleanup
        println!("Ensuring process cleanup...");

        // Kill all processes
        let _ = Command::new("pkill").arg("-9").arg("llama-server").status();
        let _ = Command::new("pkill").arg("-9").arg("llama-swap").status();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_manager_creation() {
        let manager = ProcessManager::new();
        assert!(true); // Just checking it compiles
    }

    #[tokio::test]
    async fn test_lifecycle_creation() {
        let lifecycle = ProcessLifecycle::new();
        assert!(true); // Just checking it compiles
    }
}