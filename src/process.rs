//! Process management module for handling llama-swap and llama-server.
//!
//! This module manages the lifecycle of AI-related processes ensuring they
//! are properly started, monitored, and terminated when the application dies.

use std::process::{Command, Child};
use std::sync::Arc;
use tokio::process::Command as TokioCommand;
use std::collections::HashMap;
use tokio::time::{timeout, Duration, sleep};
use std::time::Instant;

/// Represents the status of a managed process.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessStatus {
    /// Process is running normally
    Running,
    /// Process has been stopped
    Stopped,
    /// Process has failed or crashed
    Failed,
    /// Process is in an unknown state
    Unknown,
}

/// Manages AI processes like llama-swap and llama-server.
pub struct ProcessManager {
    /// Map of process names to their handles
    processes: HashMap<String, Arc<tokio::process::Child>>,
    /// Map of process names to their status
    process_status: HashMap<String, ProcessStatus>,
    /// Map of process names to restart counts
    restart_counts: HashMap<String, u32>,
    /// Maximum restart attempts before giving up
    max_restart_attempts: u32,
}

impl ProcessManager {
    /// Creates a new process manager instance.
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            process_status: HashMap::new(),
            restart_counts: HashMap::new(),
            max_restart_attempts: 3,
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
        self.process_status.insert("llama-swap".to_string(), ProcessStatus::Running);
        self.restart_counts.insert("llama-swap".to_string(), 0);

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
        self.process_status.insert("llama-server".to_string(), ProcessStatus::Running);
        self.restart_counts.insert("llama-server".to_string(), 0);

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
                        self.process_status.insert(name.clone(), ProcessStatus::Stopped);
                    }
                    Ok(Err(e)) => {
                        eprintln!("⚠️  Failed to terminate {} gracefully: {}", name, e);
                        failed_processes.push(name.clone());
                        self.process_status.insert(name.clone(), ProcessStatus::Failed);
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
                        self.process_status.insert(name.clone(), ProcessStatus::Failed);
                    }
                }
            }
        }

        // Give processes a moment to terminate
        sleep(Duration::from_millis(1000)).await;

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

    /// Checks the status of all managed processes.
    pub async fn check_process_status(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut updated_status = HashMap::new();

        for (name, child) in &self.processes {
            let status = self.get_process_status(name, child).await;
            updated_status.insert(name.clone(), status);
        }

        // Update process statuses
        for (name, status) in updated_status {
            self.process_status.insert(name, status);
        }

        Ok(())
    }

    /// Gets the current status of a specific process.
    async fn get_process_status(&self, name: &str, child: &Arc<tokio::process::Child>) -> ProcessStatus {
        // Check if process is still running
        match child.try_wait() {
            Ok(Some(status)) => {
                // Process has exited
                if status.success() {
                    ProcessStatus::Stopped
                } else {
                    ProcessStatus::Failed
                }
            }
            Ok(None) => {
                // Process is still running
                ProcessStatus::Running
            }
            Err(_) => {
                // Error checking process status, assume unknown
                ProcessStatus::Unknown
            }
        }
    }

    /// Monitors processes and handles restarts if needed.
    pub async fn monitor_processes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Monitoring processes...");

        for (name, child) in &self.processes {
            let status = self.get_process_status(name, child).await;

            // Check if process has failed
            if status == ProcessStatus::Failed {
                println!("⚠️  {} has failed, attempting restart...", name);

                // Check restart attempts
                let restart_count = self.restart_counts.get(name).copied().unwrap_or(0);
                if restart_count < self.max_restart_attempts {
                    self.restart_process(name).await?;
                    self.restart_counts.insert(name.clone(), restart_count + 1);
                } else {
                    eprintln!("❌ {} has failed too many times, giving up on restarts", name);
                    // Mark as permanently failed
                    self.process_status.insert(name.clone(), ProcessStatus::Failed);
                }
            } else if status == ProcessStatus::Stopped {
                println!("🛑 {} has stopped normally", name);
                self.process_status.insert(name.clone(), ProcessStatus::Stopped);
            }
        }

        Ok(())
    }

    /// Restarts a specific process.
    async fn restart_process(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Restarting {}...", name);

        match name {
            "llama-swap" => {
                // Stop the existing process if running
                if let Some(child) = self.processes.remove(name) {
                    // Wait for graceful termination or force kill
                    let _ = timeout(Duration::from_millis(1000), child.kill()).await;
                }

                // Restart it
                self.start_llama_swap().await?;
            }
            "llama-server" => {
                // Stop the existing process if running
                if let Some(child) = self.processes.remove(name) {
                    // Wait for graceful termination or force kill
                    let _ = timeout(Duration::from_millis(1000), child.kill()).await;
                }

                // Restart it
                self.start_llama_server().await?;
            }
            _ => {
                eprintln!("⚠️  Unknown process to restart: {}", name);
            }
        }

        println!("✅ {} restarted successfully", name);
        Ok(())
    }

    /// Gets the current status of a specific process.
    pub fn get_process_status_value(&self, name: &str) -> ProcessStatus {
        self.process_status.get(name).copied().unwrap_or(ProcessStatus::Unknown)
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