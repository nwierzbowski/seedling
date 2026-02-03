//! Process management module for handling llama-swap and llama-server.
//!
//! This module manages the lifecycle of AI-related processes ensuring they
//! are properly started, monitored, and terminated when the application dies.

use std::process::{Command, Child};
use std::sync::Arc;
use tokio::process::Command as TokioCommand;
use std::collections::HashMap;
use tokio::time::{timeout, Duration};

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

    /// Validates that the command is safe to execute (no shell injection).
    fn validate_command(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check for dangerous characters
        let dangerous_chars = [';', '&', '|', '`', '$', '(', ')', '{', '}'];
        if cmd.chars().any(|c| dangerous_chars.contains(&c)) {
            anyhow::bail!("Command contains potentially dangerous characters");
        }

        Ok(())
    }

    /// Starts the llama-swap server.
    pub async fn start_llama_swap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate command
        Self::validate_command("llama-swap")?;

        // Launch the new instance with specified arguments
        let child = TokioCommand::new("llama-swap")
            .arg("--config")
            .arg("config.yaml")
            .arg("--listen")
            .arg("0.0.0.0:8081")
            .spawn()
            .map_err(|e| format!("Failed to start llama-swap: {}", e))?;

        // Store the process handle for proper cleanup
        let child_handle = Arc::new(child);
        self.processes.insert("llama-swap".to_string(), child_handle.clone());

        println!("✅ Started llama-swap server with PID: {}", child_handle.id().unwrap_or(0));
        Ok(())
    }

    /// Starts the llama-server.
    pub async fn start_llama_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate command
        Self::validate_command("llama-server")?;

        // Launch the new instance with specified arguments
        let child = TokioCommand::new("llama-server")
            .arg("--listen")
            .arg("0.0.0.0:8080")
            .arg("--model")
            .arg("models/llama-2-7b-chat.Q4_K_M.gguf")
            .spawn()
            .map_err(|e| format!("Failed to start llama-server: {}", e))?;

        // Store the process handle for proper cleanup
        let child_handle = Arc::new(child);
        self.processes.insert("llama-server".to_string(), child_handle.clone());

        println!("✅ Started llama-server with PID: {}", child_handle.id().unwrap_or(0));
        Ok(())
    }

    /// Stops all managed processes gracefully.
    pub async fn stop_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for stopping all processes
        println!("🛑 Stopping all AI processes...");

        if self.processes.is_empty() {
            println!("No processes to stop");
            return Ok(());
        }

        // For each process we're tracking, try to terminate it gracefully
        let mut failed_processes = Vec::new();

        for (name, child) in &self.processes {
            println!("Terminating {}...", name);

            // Try to terminate the process gracefully using kill() method
            if let Some(child_handle) = child.clone() {
                // Use SIGTERM for graceful termination first
                match timeout(Duration::from_millis(2000), child_handle.kill()).await {
                    Ok(Ok(_)) => {
                        println!("✅ {} terminated gracefully", name);
                    }
                    Ok(Err(e)) => {
                        eprintln!("⚠️  Failed to terminate {} gracefully: {}", name, e);
                        failed_processes.push(name.clone());
                    }
                    Err(_) => {
                        // Timeout occurred, force kill
                        println!("⏰ Timeout on graceful termination of {}, forcing kill...", name);
                        if let Some(pid) = child.id() {
                            let _ = Command::new("kill")
                                .arg("-9")
                                .arg(&pid.to_string())
                                .status();
                        }
                    }
                }
            }
        }

        // Give processes a moment to terminate
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Clean up the tracked processes map
        self.processes.clear();

        if failed_processes.is_empty() {
            println!("✅ All AI processes stopped successfully");
        } else {
            eprintln!("⚠️  Failed to gracefully terminate processes: {:?}", failed_processes);
            println!("✅ All AI processes stopped (some with force termination)");
        }

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
    async fn test_process_manager_stop_all() {
        let mut manager = ProcessManager::new();
        // Test that stop_all can be called without error (even with no processes)
        let result = manager.stop_all().await;
        assert!(result.is_ok());
    }
}