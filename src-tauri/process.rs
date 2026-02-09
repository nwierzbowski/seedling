//! Process management module for handling llama-swap server.
//!
//! This module manages the lifecycle of the llama-swap process ensuring it
//! is properly started, monitored, and terminated when the application dies.

use anyhow::Result;
use std::collections::HashMap;
use std::{
    process::Command,
};
use tokio::process::{Child, Command as TokioCommand};
use tokio::time::{Duration, sleep, timeout};

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

/// Manages AI processes like llama-swap.
pub struct ProcessManager {
    /// Map of process names to their handles
    processes: HashMap<String, Child>,
    /// Map of process names to their status
    process_status: HashMap<String, ProcessStatus>,
    // /// Map of process names to restart counts
    // restart_counts: HashMap<String, u32>,
    // /// Maximum restart attempts before giving up
    // max_restart_attempts: u32,
}

impl ProcessManager {
    /// Creates a new process manager instance.
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            process_status: HashMap::new(),
            // restart_counts: HashMap::new(),
            // max_restart_attempts: 3,
        }
    }

    /// Starts the llama-swap server.
    pub async fn _start_llama_swap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Launch the new instance with specified arguments
        let child = TokioCommand::new("/home/nwier/.local/bin/llama-swap")
            .arg("--config")
            .arg("/home/nwier/models/llama-swap-config.yaml")
            .arg("--listen")
            .arg("0.0.0.0:8081")
            .spawn()
            .map_err(|e| format!("Failed to start llama-swap: {}", e))?;

        let pid = child.id();

        // Store the process handle for proper cleanup
        self.processes.insert("llama-swap".to_string(), child);
        self.process_status
            .insert("llama-swap".to_string(), ProcessStatus::Running);
        // self.restart_counts.insert("llama-swap".to_string(), 0);

        println!("✅ Started llama-swap server with PID: {:?}", pid);
        Ok(())
    }

    pub async fn start_ollama(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        // Launch the new instance with specified arguments
        let child = TokioCommand::new("/usr/local/bin/ollama")
            .arg("serve")
            .env("OLLAMA_KV_CACHE_TYPE", "q4_0")
            .env("OLLAMA_FLASH_ATTENTION", "1")
            .env("OLLAMA_CONTEXT_LENGTH", "24576")
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| format!("Failed to start ollama: {}", e))?;

        let pid = child.id();

        // Store the process handle for proper cleanup
        self.processes.insert("ollama".to_string(), child);
        self.process_status
            .insert("ollama".to_string(), ProcessStatus::Running);
        // self.restart_counts.insert("ollama".to_string(), 0);

        println!("✅ Started ollama server with PID: {:?}", pid);
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

        // Collect names first to avoid borrowing issues
        let process_names: Vec<String> = self.processes.keys().cloned().collect();

        for name in process_names {
            if let Some(child) = self.processes.get_mut(&name) {
                println!("Terminating {}...", &name);
                // Try to terminate the process gracefully using kill() method
                // Use SIGTERM for graceful termination first
                match timeout(Duration::from_millis(2000), child.kill()).await {
                    Ok(Ok(_)) => {
                        println!("✅ {} terminated gracefully", &name);
                        self.process_status
                            .insert(name.clone(), ProcessStatus::Stopped);
                    }
                    Ok(Err(e)) => {
                        eprintln!("⚠️  Failed to terminate {} gracefully: {}", &name, e);
                        failed_processes.push(name.clone());
                        self.process_status
                            .insert(name.clone(), ProcessStatus::Failed);
                    }
                    Err(_) => {
                        // Timeout occurred, force kill
                        println!(
                            "⏰ Timeout on graceful termination of {}, forcing kill...",
                            &name
                        );
                        if let Some(pid) = child.id() {
                            let _ = Command::new("kill")
                                .arg("-9")
                                .arg(&pid.to_string())
                                .status();
                        }
                        self.process_status
                            .insert(name.clone(), ProcessStatus::Failed);
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
            eprintln!(
                "⚠️  Failed to gracefully terminate processes: {:?}",
                failed_processes
            );
            println!("✅ All AI processes stopped (some with force termination)");
        }

        Ok(())
    }

    /// Monitors all managed processes for their current status.
    pub async fn _monitor_processes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Monitoring AI processes...");

        // For each process we're tracking, check its status
        let process_names: Vec<String> = self.processes.keys().cloned().collect();

        for name in process_names {
            if let Some(child) = self.processes.get_mut(&name) {
                // Check if the process has exited by trying to get its exit status
                match child.try_wait() {
                    Ok(Some(status)) => {
                        // Process has exited, update status
                        println!("⚠️  {} has exited with status: {}", &name, status);
                        self.process_status
                            .insert(name.clone(), ProcessStatus::Failed);
                    }
                    Ok(None) => {
                        // Process is still running
                        self.process_status
                            .insert(name.clone(), ProcessStatus::Running);
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to check status of {}: {}", &name, e);
                        self.process_status
                            .insert(name.clone(), ProcessStatus::Unknown);
                    }
                }
            }
        }

        println!("✅ AI processes monitoring completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_manager_creation() {
        let _manager = ProcessManager::new();
        assert!(true); // Just checking it compiles
    }
}
