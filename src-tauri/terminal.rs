//! Terminal session management for the seedling AI development environment.

use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};

use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

use crate::router_agent::{SharedRouterAgent};

pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    _writer: Box<dyn std::io::Write + Send>,
}

pub struct TerminalStateData {
    pub session: Option<PtySession>,
    pub input_buffer: String,
}

impl Default for TerminalStateData {
    fn default() -> Self {
        Self {
            session: None,
            input_buffer: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct TerminalState(pub Arc<std::sync::Mutex<TerminalStateData>>);

pub fn _start(app: AppHandle, state: TerminalState) {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            ..Default::default()
        })
        .unwrap();

    let cmd = CommandBuilder::new("sh");
    pair.slave
        .spawn_command(cmd)
        .expect("Failed to start bash");
    drop(pair.slave); // Close the slave end in the parent

    let reader = pair
        .master
        .try_clone_reader()
        .expect("Failed to clone reader.");
    let writer = pair.master.take_writer().expect("Failed to take writer.");

    let mut state_data = state.0.lock().unwrap();
    state_data.session = Some(PtySession {
        master: pair.master,
        _writer: writer,
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
pub async fn write_to_buffer(
    data: String,
    state: State<'_, TerminalState>,
    agent: State<'_, SharedRouterAgent>,
    app: AppHandle,
) -> Result<(), String> {
    // If no newline, buffer and echo back to terminal
    if !data.contains('\r') {
        let mut state_guard = state.0.lock().unwrap();
        state_guard.input_buffer.push_str(&data);
        drop(state_guard); // Release lock before emitting
        
        // Echo the typed character back to the terminal
        app.emit("pty-data", data).map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Extract complete buffered input
    let input = {
        let mut state_guard = state.0.lock().unwrap();
        state_guard.input_buffer.push_str(&data);
        let input = state_guard.input_buffer.clone();
        state_guard.input_buffer.clear();
        input
    };

    // Echo the newline
    app.emit("pty-data", "\r\n").map_err(|e| e.to_string())?;

    let response = {
        let guard = agent.inner().lock().await;
        if let Some(agent) = guard.as_ref() {
            agent.prompt(&input).await?
        } else {
            return Err("Router agent not initialized".to_string());
        }
    };

    // Convert Unix newlines to terminal newlines
    let terminal_response = response.replace("\n", "\r\n");
    
    app.emit("pty-data", format!("{}\r\n", terminal_response)).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn resize_pty(rows: u16, cols: u16, state: tauri::State<'_, TerminalState>) {
    let mut state_guard = state.0.lock().unwrap();
    if let Some(session) = state_guard.session.as_mut() {
        let size = PtySize {
            rows,
            cols,
            ..Default::default()
        };
        session.master.resize(size).unwrap();
    }
}
