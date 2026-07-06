use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::process::Child;
use tokio::process::Command;

const START_TIMEOUT: Duration = Duration::from_secs(10);

/// Host-local exec-server fixture that exposes a WebSocket URL.
///
/// This is distinct from the ordinary local stdio executor: callers use it
/// when they need a socket transport they can interpose.
pub(crate) struct LocalWebsocketExecServer {
    _child: Child,
    websocket_url: String,
}

impl LocalWebsocketExecServer {
    pub(crate) async fn start(codex_home: &Path) -> Result<Self> {
        let mut command = if let Ok(exec_server) = codex_utils_cargo_bin::cargo_bin("exec-server") {
            Command::new(exec_server)
        } else {
            // Cargo-backed functional tests do not build the Bazel-only
            // fixture binary. Keep their existing CLI fallback while Bazel
            // tests and macrobenchmarks inject CARGO_BIN_EXE_exec-server.
            let codex = codex_utils_cargo_bin::cargo_bin("codex")
                .context("should find binary for local exec-server fixture")?;
            let mut command = Command::new(codex);
            command.args(["exec-server", "--listen", "ws://127.0.0.1:0"]);
            command
        };
        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::inherit());
        command.current_dir(codex_home);
        command.env("CODEX_HOME", codex_home);
        command.kill_on_drop(true);
        let mut child = command.spawn().context("start local exec-server fixture")?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("local exec-server fixture stdout was not captured"))?;
        let mut lines = BufReader::new(stdout).lines();
        let deadline = tokio::time::Instant::now() + START_TIMEOUT;
        let websocket_url = loop {
            let remaining = deadline
                .checked_duration_since(tokio::time::Instant::now())
                .ok_or_else(|| anyhow!("timed out waiting for local exec-server listen URL"))?;
            let line = tokio::time::timeout(remaining, lines.next_line())
                .await
                .map_err(|_| anyhow!("timed out waiting for local exec-server listen URL"))??
                .ok_or_else(|| {
                    anyhow!("local exec-server exited before emitting its listen URL")
                })?;
            let listen_url = line.trim();
            if listen_url.starts_with("ws://") {
                break listen_url.to_string();
            }
        };
        Ok(Self {
            _child: child,
            websocket_url,
        })
    }

    pub(crate) fn websocket_url(&self) -> &str {
        &self.websocket_url
    }
}
