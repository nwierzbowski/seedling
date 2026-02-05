import { useEffect, useRef } from "react";
import "./TerminalTab.css";
import "@xterm/xterm/css/xterm.css";

function TerminalTab() {
  const containerRef =
    useRef<HTMLDivElement>(null);

  useEffect(() => {
    // 1. CAPTURE the current value immediately
    const container = containerRef.current;
    if (!container) return;

    // Track all disposables in an array or local variables
    let terminal: any;
    let resizeObserver: ResizeObserver;
    let unlistenPty: () => void;

    const init = async () => {
      const { Terminal } =
        await import("@xterm/xterm");
      const { FitAddon } =
        await import("@xterm/addon-fit");
      const { listen } =
        await import("@tauri-apps/api/event");
      const { invoke } =
        await import("@tauri-apps/api/core");

      terminal = new Terminal({
        cursorBlink: true,
      });

      const fitAddon = new FitAddon();
      terminal.loadAddon(fitAddon);
      terminal.open(container);
      fitAddon.fit();

      // Store the unlisten function returned by Tauri
      unlistenPty = await listen(
        "pty-data",
        event => {
          terminal.write(event.payload as string);
        },
      );

      terminal.onData((data: string) => {
        invoke("backend_write_pty", { data });
      });

      resizeObserver = new ResizeObserver(() => {
        fitAddon.fit();
        invoke("resize_pty", {
          cols: terminal.cols,
          rows: terminal.rows,
        });
      });
      resizeObserver.observe(container);

      await invoke("resize_pty", {
        cols: terminal.cols + 1,
        rows: terminal.rows,
      });

      // Immediately tell it the correct size
      // This forces the backend to send two SIGWINCH signals,
      // ensuring the shell redraws its prompt.
      await invoke("resize_pty", {
        cols: terminal.cols,
        rows: terminal.rows,
      });
    };

    init();

    // CLEANUP: This closure remembers 'container', 'terminal', and 'unlistenPty'
    return () => {
      if (unlistenPty) unlistenPty();
      if (resizeObserver)
        resizeObserver.disconnect();
      if (terminal) {
        terminal.dispose();
      }
      console.log(
        "Terminal cleaned up successfully",
      );
    };
  }, []);

  return (
    <div
      ref={containerRef}
      id="terminal-container"
      className="terminal-container"
    ></div>
  );
}

export default TerminalTab;
