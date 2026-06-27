use std::io;
use std::net::SocketAddr;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::process::Child;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tokio::time::sleep_until;
use tokio::time::timeout;

const EXEC_SERVER_START_TIMEOUT: Duration = Duration::from_secs(10);
const FORWARD_BUFFER_BYTES: usize = 64 * 1024;
const FORWARD_QUEUE_CHUNKS: usize = 16;

pub(crate) struct DelayedExecServer {
    _exec_server: Child,
    interposer: TcpDelayInterposer,
}

impl DelayedExecServer {
    pub(crate) async fn start(codex_home: &Path, added_delay: Duration) -> Result<Self> {
        let codex = codex_utils_cargo_bin::cargo_bin("codex")
            .context("should find binary for local exec-server fixture")?;
        let mut child = Command::new(codex);
        child.args(["exec-server", "--listen", "ws://127.0.0.1:0"]);
        child.stdin(Stdio::null());
        child.stdout(Stdio::piped());
        child.stderr(Stdio::inherit());
        child.current_dir(codex_home);
        child.env("CODEX_HOME", codex_home);
        child.kill_on_drop(true);
        let mut child = child.spawn().context("start local exec-server fixture")?;
        let upstream_url = read_exec_server_url(&mut child).await?;
        let interposer = TcpDelayInterposer::start(&upstream_url, added_delay).await?;
        Ok(Self {
            _exec_server: child,
            interposer,
        })
    }

    pub(crate) fn websocket_url(&self) -> &str {
        self.interposer.websocket_url()
    }
}

struct TcpDelayInterposer {
    websocket_url: String,
    accept_task: JoinHandle<()>,
}

impl TcpDelayInterposer {
    async fn start(upstream_url: &str, added_delay: Duration) -> Result<Self> {
        let upstream = websocket_socket_addr(upstream_url)?;
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .context("bind RPC delay interposer")?;
        let websocket_url = format!("ws://{}", listener.local_addr()?);
        let accept_task = tokio::spawn(async move {
            loop {
                let Ok((downstream, _peer)) = listener.accept().await else {
                    break;
                };
                tokio::spawn(async move {
                    let Ok(upstream) = TcpStream::connect(upstream).await else {
                        return;
                    };
                    let _ = proxy_connection(downstream, upstream, added_delay).await;
                });
            }
        });
        Ok(Self {
            websocket_url,
            accept_task,
        })
    }

    fn websocket_url(&self) -> &str {
        &self.websocket_url
    }
}

impl Drop for TcpDelayInterposer {
    fn drop(&mut self) {
        self.accept_task.abort();
    }
}

async fn read_exec_server_url(child: &mut Child) -> Result<String> {
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("local exec-server fixture stdout was not captured"))?;
    let mut lines = BufReader::new(stdout).lines();
    let deadline = Instant::now() + EXEC_SERVER_START_TIMEOUT;

    loop {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .ok_or_else(|| anyhow!("timed out waiting for local exec-server listen URL"))?;
        let line = timeout(remaining, lines.next_line())
            .await
            .map_err(|_| anyhow!("timed out waiting for local exec-server listen URL"))??
            .ok_or_else(|| anyhow!("local exec-server exited before emitting its listen URL"))?;
        let listen_url = line.trim();
        if listen_url.starts_with("ws://") {
            return Ok(listen_url.to_string());
        }
    }
}

fn websocket_socket_addr(websocket_url: &str) -> Result<SocketAddr> {
    let authority = websocket_url
        .strip_prefix("ws://")
        .ok_or_else(|| anyhow!("RPC delay requires a ws:// exec-server URL"))?
        .split('/')
        .next()
        .ok_or_else(|| anyhow!("RPC delay exec-server URL has no authority"))?;
    authority
        .parse()
        .with_context(|| format!("parse RPC delay upstream address {authority}"))
}

async fn proxy_connection(
    downstream: TcpStream,
    upstream: TcpStream,
    added_delay: Duration,
) -> io::Result<()> {
    let (downstream_read, downstream_write) = downstream.into_split();
    let (upstream_read, upstream_write) = upstream.into_split();
    let client_to_server = forward_direction(downstream_read, upstream_write, added_delay);
    let server_to_client = forward_direction(upstream_read, downstream_write, added_delay);
    tokio::try_join!(client_to_server, server_to_client)?;
    Ok(())
}

async fn forward_direction<R, W>(
    mut reader: R,
    mut writer: W,
    added_delay: Duration,
) -> io::Result<()>
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    let (tx, mut rx) = mpsc::channel::<DelayedChunk>(FORWARD_QUEUE_CHUNKS);
    let reader_task = tokio::spawn(async move {
        loop {
            let mut bytes = vec![0; FORWARD_BUFFER_BYTES];
            let read = reader.read(&mut bytes).await?;
            if read == 0 {
                break;
            }
            bytes.truncate(read);
            let chunk = DelayedChunk {
                deliver_at: Instant::now() + added_delay,
                bytes,
            };
            if tx.send(chunk).await.is_err() {
                break;
            }
        }
        Ok::<(), io::Error>(())
    });

    while let Some(chunk) = rx.recv().await {
        sleep_until(chunk.deliver_at).await;
        writer.write_all(&chunk.bytes).await?;
    }
    writer.shutdown().await?;
    reader_task
        .await
        .map_err(|err| io::Error::other(format!("RPC delay reader task failed: {err}")))??;
    Ok(())
}

struct DelayedChunk {
    deliver_at: Instant,
    bytes: Vec<u8>,
}
