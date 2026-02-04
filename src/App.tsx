import { useEffect } from 'react';
import './App.css'


function App() {

  useEffect(() => {
    setupTerminal(document.getElementById('terminal-container')!);
  }, []);

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>🌱 AIDME (AI Development Management Environment)</h1>
      </header>
      <main className="app-main">
        <div id="terminal-container" style={{ width: '100%', height: '100%' }}></div>
      </main>
    </div>
  )
}
export default App;

import { Terminal } from '@xterm/xterm';
// import { FitAddon } from '@xterm/addon-fit';
// import { spawn } from 'tauri-pty';
import '@xterm/xterm/css/xterm.css';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { FitAddon } from '@xterm/addon-fit';

export async function setupTerminal(container: HTMLElement) {
  const term = new Terminal({
    cursorBlink: true,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
  });

  const fitAddon = new FitAddon();
  term.loadAddon(fitAddon);
  term.open(container);
  fitAddon.fit();

  // 1. Identify the shell for the current OS
  // const shell = "bash";

  listen("pty-data", (event) => {
    term.write(event.payload as string);
  })

  term.onData((data) => {
    invoke("backend_write_pty", { data });
  })

  // // 5. Handle Resizing
  const resizeObserver = new ResizeObserver(() => {
    fitAddon.fit();
    // Notify the backend PTY that the "window" size changed
    invoke("resize_pty", { cols: term.cols, rows: term.rows });
  });
  
  resizeObserver.observe(container);

  return { term };
}
