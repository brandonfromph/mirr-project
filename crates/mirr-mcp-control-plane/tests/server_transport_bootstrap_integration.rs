use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use mirror::server_rewrite::axum_route_host::{AxumHostConfig, AxumMcpHostState};
use mirror::server_rewrite::mrt_dispatch_quota_store::{
    MrtDispatchQuotaEventSink, PersistedTokenQuotaState,
};
use mirror::server_rewrite::rpc_dispatch_bridge::RpcHandlerMap;
use mirror::server_rewrite::transport_bootstrap::{
    bootstrap_transport_with_runner, TransportStartupAction, TransportStartupRunner,
};
use mirror::server_rewrite::transport_mode_resolution::{
    TransportMode, TRANSPORT_STREAM_FEATURE_FLAG, TRANSPORT_STREAM_PORT_KEY,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RunnerRecord {
    stdio_starts: u32,
    stream_starts: u32,
    last_stream_bind: Option<SocketAddr>,
    observed_builder_quota_count: Option<u32>,
}

#[derive(Clone, Default)]
struct FakeStartupRunner {
    record: Arc<Mutex<RunnerRecord>>,
}

impl FakeStartupRunner {
    fn record(&self) -> RunnerRecord {
        self.record.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
    }
}

impl TransportStartupRunner for FakeStartupRunner {
    fn start_stdio(&self, state: AxumMcpHostState) -> Result<(), String> {
        let observed = {
            let guard =
                state.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.token_quota.get("builder-token").map(|entry| entry.count)
        };

        let mut record = self.record.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        record.stdio_starts = record.stdio_starts.saturating_add(1);
        record.observed_builder_quota_count = observed;
        Ok(())
    }

    fn start_stream(
        &self,
        bind_addr: SocketAddr,
        state: AxumMcpHostState,
        _config: AxumHostConfig,
    ) -> Result<(), String> {
        let observed = {
            let guard =
                state.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.token_quota.get("builder-token").map(|entry| entry.count)
        };

        let mut record = self.record.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        record.stream_starts = record.stream_starts.saturating_add(1);
        record.last_stream_bind = Some(bind_addr);
        record.observed_builder_quota_count = observed;
        Ok(())
    }
}

#[derive(Default)]
struct SeededQuotaSink {
    rows: Mutex<Vec<PersistedTokenQuotaState>>,
}

impl SeededQuotaSink {
    fn with_rows(rows: Vec<PersistedTokenQuotaState>) -> Self {
        Self { rows: Mutex::new(rows) }
    }
}

impl MrtDispatchQuotaEventSink for SeededQuotaSink {
    fn persist_token_quota(
        &self,
        _token: &str,
        _window_start_ms: u64,
        _count: u32,
    ) -> Result<(), String> {
        Ok(())
    }

    fn load_recent_token_quota_rows(
        &self,
        limit: usize,
    ) -> Result<Vec<PersistedTokenQuotaState>, String> {
        let guard = self.rows.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        Ok(guard.iter().take(limit).cloned().collect())
    }
}

fn empty_handler_factory() -> RpcHandlerMap<String> {
    BTreeMap::new()
}

#[test]
fn bootstrap_seam_exercises_mode_resolution_and_startup_branching() {
    let runner = FakeStartupRunner::default();
    let state = AxumMcpHostState::new(empty_handler_factory);
    let host_config = AxumHostConfig::default();

    let mut stream_env = BTreeMap::<String, String>::new();
    stream_env.insert(TRANSPORT_STREAM_FEATURE_FLAG.to_owned(), "true".to_owned());
    stream_env.insert(TRANSPORT_STREAM_PORT_KEY.to_owned(), "4555".to_owned());

    let stream_decision = bootstrap_transport_with_runner(
        &stream_env,
        3333,
        state.clone(),
        host_config.clone(),
        &runner,
    )
    .expect("stream bootstrap branch should start successfully");

    assert_eq!(stream_decision.transport.mode, TransportMode::Stream);
    match stream_decision.action {
        TransportStartupAction::StartStream { bind_addr } => {
            assert_eq!(bind_addr.port(), 4555);
        }
        TransportStartupAction::StartStdio => panic!("expected stream startup action"),
    }

    let stdio_decision =
        bootstrap_transport_with_runner(&BTreeMap::new(), 3333, state, host_config, &runner)
            .expect("stdio bootstrap branch should start successfully");

    assert_eq!(stdio_decision.transport.mode, TransportMode::Stdio);
    assert_eq!(stdio_decision.action, TransportStartupAction::StartStdio);

    let record = runner.record();
    assert_eq!(record.stream_starts, 1);
    assert_eq!(record.stdio_starts, 1);
    assert_eq!(record.last_stream_bind.map(|addr| addr.port()), Some(4555));
    assert_eq!(record.observed_builder_quota_count, None);
}

#[test]
fn bootstrap_hydrates_quota_rows_before_startup_runner() {
    let runner = FakeStartupRunner::default();
    let mut state = AxumMcpHostState::new(empty_handler_factory);
    let seeded_sink = Arc::new(SeededQuotaSink::with_rows(vec![PersistedTokenQuotaState {
        token: "builder-token".to_owned(),
        window_start_ms: 200,
        count: 5,
    }]));
    let sink_for_state: Arc<dyn MrtDispatchQuotaEventSink> = seeded_sink;
    state.quota_event_sink = sink_for_state;

    let host_config = AxumHostConfig::default();

    let decision =
        bootstrap_transport_with_runner(&BTreeMap::new(), 3333, state, host_config, &runner)
            .expect("bootstrap should hydrate seeded quota rows before startup");

    assert_eq!(decision.transport.mode, TransportMode::Stdio);

    let record = runner.record();
    assert_eq!(record.stdio_starts, 1);
    assert_eq!(record.observed_builder_quota_count, Some(5));
}
