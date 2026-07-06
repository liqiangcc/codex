#![allow(clippy::expect_used)]

use std::sync::Arc;
use std::time::Duration;

use app_test_support::TestAppServer;
use codex_app_server_protocol::JSONRPCResponse;
use codex_app_server_protocol::RequestId;
use codex_app_server_protocol::ThreadStartParams;
use divan::Bencher;
use tempfile::TempDir;
use tokio::runtime::Runtime;

const EXEC_SERVER_DELAY: Duration = Duration::from_millis(15);

fn main() {
    require_bazel_binary("codex-app-server");
    require_bazel_binary("exec-server");
    divan::main();
}

#[divan::bench(sample_count = 5, sample_size = 1)]
fn cold_thread_start(bencher: Bencher) {
    let runtime = Arc::new(Runtime::new().expect("benchmark Tokio runtime should start"));
    let setup_runtime = Arc::clone(&runtime);
    let bench_runtime = Arc::clone(&runtime);

    bencher
        .with_inputs(move || setup_runtime.block_on(ColdThreadStartCase::new()))
        .bench_local_values(move |case| bench_runtime.block_on(case.run()));
}

fn require_bazel_binary(binary: &str) {
    let key = format!("CARGO_BIN_EXE_{binary}");
    assert!(
        std::env::var_os(&key).is_some(),
        "{key} is required; run this benchmark through its Bazel target"
    );
}

struct ColdThreadStartCase {
    _codex_home: TempDir,
    app_server: TestAppServer,
}

impl ColdThreadStartCase {
    async fn new() -> Self {
        let codex_home = TempDir::new().expect("benchmark CODEX_HOME should be created");
        let mut app_server = TestAppServer::builder(codex_home.path())
            .with_exec_server_delay(EXEC_SERVER_DELAY)
            .build()
            .await
            .expect("benchmark app-server fixture should start");
        app_server
            .initialize()
            .await
            .expect("benchmark app-server should initialize");
        Self {
            _codex_home: codex_home,
            app_server,
        }
    }

    async fn run(mut self) {
        let request_id = self
            .app_server
            .send_thread_start_request_with_auto_env(ThreadStartParams::default())
            .await
            .expect("thread/start request should be sent");
        let _: JSONRPCResponse = self
            .app_server
            .read_stream_until_response_message(RequestId::Integer(request_id))
            .await
            .expect("thread/start response should arrive");
    }
}
