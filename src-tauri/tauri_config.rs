//! Tauri configuration for the seedling AI development environment.
//!
//! This module defines the configuration and setup for the Tauri-based desktop application
//! that replaces the tmux terminal interface.

use serde::{Deserialize, Serialize};

/// Configuration for the Tauri desktop application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriConfig {
    /// Application name
    pub app_name: String,
    /// Application version
    pub app_version: String,
    /// Window settings
    pub window: WindowConfig,
    /// Agent configurations
    pub agents: AgentsConfig,
    /// GPU configuration
    pub gpu: GpuConfig,
}

/// Window configuration for the Tauri desktop application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    /// Window dimensions
    pub width: u32,
    pub height: u32,
    /// Whether to enable window decorations
    pub decorations: bool,
    /// Whether the window is resizable
    pub resizable: bool,
}

/// Agent configurations for the Tauri desktop application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    /// Engineer agent configuration
    pub engineer: AgentConfig,
    /// Tester agent configuration
    pub tester: AgentConfig,
    /// Auditor agent configuration
    pub auditor: AgentConfig,
}

/// Configuration for a single agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent name
    pub name: String,
    /// Agent model
    pub model: String,
    /// Agent pane identifier
    pub pane_id: String,
    /// Whether agent is enabled
    pub enabled: bool,
}

/// GPU configuration for the Tauri desktop application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Whether to enable GPU safety protocols
    pub safety_protocols_enabled: bool,
    /// Persistence mode setting
    pub persistence_mode: u32,
    /// Power limit in watts
    pub power_limit_watts: u32,
    /// Clock locking in MHz
    pub clock_lock_mhz: u32,
}

impl Default for TauriConfig {
    fn default() -> Self {
        Self {
            app_name: "Seedling AI Development Environment".to_string(),
            app_version: "0.1.0".to_string(),
            window: WindowConfig {
                title: "Seedling War Room".to_string(),
                width: 1200,
                height: 800,
                decorations: true,
                resizable: true,
            },
            agents: AgentsConfig {
                engineer: AgentConfig {
                    name: "Engineer".to_string(),
                    model: "claude-3-opus".to_string(),
                    pane_id: "pane_0".to_string(),
                    enabled: true,
                },
                tester: AgentConfig {
                    name: "Tester".to_string(),
                    model: "claude-3-sonnet".to_string(),
                    pane_id: "pane_1".to_string(),
                    enabled: true,
                },
                auditor: AgentConfig {
                    name: "Auditor".to_string(),
                    model: "claude-3-haiku".to_string(),
                    pane_id: "pane_2".to_string(),
                    enabled: true,
                },
            },
            gpu: GpuConfig {
                safety_protocols_enabled: true,
                persistence_mode: 1,
                power_limit_watts: 300,
                clock_lock_mhz: 2100,
            },
        }
    }
}

impl TauriConfig {
    /// Creates a new Tauri configuration instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads configuration from file or returns defaults.
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // In a real implementation, this would read from a config file
        // For now, return default configuration
        Ok(Self::default())
    }

    /// Saves the current configuration to file.
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would save to a config file
        Ok(())
    }
}

/// Tauri Application State Manager.
pub struct AppState {
    /// Current configuration
    pub config: TauriConfig,
    /// Whether the application is running
    pub is_running: bool,
    /// Current agent in focus
    pub active_agent: Option<String>,
    /// GPU status
    pub gpu_status: GpuStatus,
}

/// GPU Status Information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuStatus {
    /// Whether GPU is idle
    pub is_idle: bool,
    /// GPU utilization percentage
    pub utilization: u32,
    /// GPU memory usage in MB
    pub memory_used_mb: u32,
    /// Total GPU memory in MB
    pub memory_total_mb: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: TauriConfig::new(),
            is_running: false,
            active_agent: None,
            gpu_status: GpuStatus {
                is_idle: true,
                utilization: 0,
                memory_used_mb: 0,
                memory_total_mb: 0,
            },
        }
    }
}

impl AppState {
    /// Creates a new application state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the active agent.
    pub fn set_active_agent(&mut self, agent_name: &str) {
        self.active_agent = Some(agent_name.to_string());
    }

    /// Gets the current active agent.
    pub fn get_active_agent(&self) -> Option<&String> {
        self.active_agent.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tauri_config_creation() {
        let config = TauriConfig::new();
        assert_eq!(config.app_name, "Seedling AI Development Environment");
        assert_eq!(config.app_version, "0.1.0");
    }

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert!(!state.is_running);
        assert!(state.active_agent.is_none());
    }

    #[test]
    fn test_config_loading() {
        let result = TauriConfig::load_from_file("config.toml");
        // This should work but return defaults in our case
        assert!(result.is_ok());
    }

    #[test]
    fn test_gpu_status_default() {
        let status = GpuStatus {
            is_idle: true,
            utilization: 0,
            memory_used_mb: 0,
            memory_total_mb: 0,
        };
        assert!(status.is_idle);
        assert_eq!(status.utilization, 0);
    }
}