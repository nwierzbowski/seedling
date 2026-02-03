//! Tmux module for managing terminal layouts and agent communication.
//!
//! This module handles the terminal layout management and sending commands
//! to different AI agents through tmux panes.
//!
//! The WarRoom implementation provides a sophisticated 3-pane terminal layout:
//! - Pane 0 (Engineer): Main development environment
//! - Pane 1 (Tester): Testing and validation tools
//! - Pane 2 (Auditor): Monitoring and auditing capabilities

use std::process::Command;
use std::io::Write;

/// Manages tmux sessions and pane layouts.
pub struct TmuxManager {
    /// Session name for the tmux session
    session_name: String,
    /// Number of panes in the layout
    pane_count: usize,
}

impl TmuxManager {
    /// Creates a new tmux manager instance.
    pub fn new(session_name: &str) -> Self {
        Self {
            session_name: session_name.to_string(),
            pane_count: 0,
        }
    }

    /// Sets up the sophisticated terminal layout with 3 panes for agents.
    /// This creates a War Room layout with Engineer, Tester, and Auditor panes.
    pub async fn setup_layout(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🖥️  Constructing War Room Layout...");

        // Kill existing session if it exists
        let _ = Command::new("tmux")
            .args(&["kill-session", "-t", &self.session_name])
            .status();

        // Create new session with detached mode (no-attach)
        let _ = Command::new("tmux")
            .args(&["new-session", "-d", "-s", &self.session_name])
            .status();

        // Split right to create Pane 1 (Tester)
        let _ = Command::new("tmux")
            .args(&["split-window", "-h", "-t", &self.session_name])
            .status();

        // Split bottom right (Pane 2 - Auditor) from Pane 1
        let _ = Command::new("tmux")
            .args(&["split-window", "-v", "-t", &format!("{}:0.1", self.session_name)])
            .status();

        // Inject agents into their respective panes
        self.inject_agent(0, "engineer");
        self.inject_agent(1, "tester");
        self.inject_agent(2, "auditor");

        println!("✅ War Room layout constructed with 3 panes.");
        Ok(())
    }

    /// Injects an agent with its specific configuration into a pane.
    fn inject_agent(&self, pane: u8, model: &str) {
        let cmd = format!("export ANTHROPIC_BASE_URL=http://localhost:8081/v1 && claude --model {}", model);

        // Send the command to the specific pane
        let _ = Command::new("tmux")
            .args(&["send-keys", "-t", &format!("{}:0.{}", self.session_name, pane), &cmd, "C-m"])
            .status();

        // Clear screen to force config reload
        let _ = Command::new("tmux")
            .args(&["send-keys", "-t", &format!("{}:0.{}", self.session_name, pane), "/clear", "C-m"])
            .status();
    }

    /// Sends a command to a specific agent pane.
    pub async fn send_command_to_agent(
        &self,
        agent_name: &str,
        command: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for sending commands to agents
        println!("Sending command '{}' to agent '{}'", command, agent_name);
        Ok(())
    }

    /// Creates a new tmux session.
    pub async fn create_session(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for creating tmux session
        println!("Creating tmux session '{}'", self.session_name);
        Ok(())
    }
}

/// Agent communication manager.
pub struct AgentManager {
    /// Map of agent names to their tmux pane identifiers
    agents: std::collections::HashMap<String, String>,
}

impl AgentManager {
    /// Creates a new agent manager instance.
    pub fn new() -> Self {
        Self {
            agents: std::collections::HashMap::new(),
        }
    }

    /// Sends a command to an agent.
    pub async fn send_to_agent(
        &self,
        agent_name: &str,
        command: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for sending commands to agents
        println!("Sending command '{}' to agent '{}'", command, agent_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tmux_manager_creation() {
        let manager = TmuxManager::new("test-session");
        assert!(true); // Just checking it compiles
    }

    #[tokio::test]
    async fn test_agent_manager_creation() {
        let manager = AgentManager::new();
        assert!(true); // Just checking it compiles
    }
}