use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use mirror::server_rewrite::axum_route_host::{AxumHostConfig, AxumMcpHostState};
use mirror::server_rewrite::mrt_dispatch_quota_store::{
    MrtDispatchQuotaEventSink, PersistedTokenQuotaState,
};
use mirror::server_rewrite::rpc_dispatch_bridge::RpcHandlerMap;
use mirror::server_rewrite::transport_bootstrap::{
    bootstrap_transport_with_runner, TransportStartupRunner,
};
use mirror::server_rewrite::transport_mode_resolution::TransportMode;

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RecoveryRunnerRecord {
    observed_builder_count: Option<u32>,
    stdio_starts: u32,
    stream_starts: u32,
}

#[derive(Clone, Default)]
struct RecoveryRunner {
    record: Arc<Mutex<RecoveryRunnerRecord>>,
}

impl RecoveryRunner {
    fn record(&self) -> RecoveryRunnerRecord {
        self.record.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
    }
}

impl TransportStartupRunner for RecoveryRunner {
    fn start_stdio(&self, state: AxumMcpHostState) -> Result<(), String> {
        let observed_builder_count = {
            let guard =
                state.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.token_quota.get("builder-token").map(|row| row.count)
        };

        let mut record = self.record.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        record.stdio_starts = record.stdio_starts.saturating_add(1);
        record.observed_builder_count = observed_builder_count;
        Ok(())
    }

    fn start_stream(
        &self,
        _bind_addr: SocketAddr,
        state: AxumMcpHostState,
        _config: AxumHostConfig,
    ) -> Result<(), String> {
        let observed_builder_count = {
            let guard =
                state.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.token_quota.get("builder-token").map(|row| row.count)
        };

        let mut record = self.record.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        record.stream_starts = record.stream_starts.saturating_add(1);
        record.observed_builder_count = observed_builder_count;
        Ok(())
    }
}

fn empty_handler_factory() -> RpcHandlerMap<String> {
    BTreeMap::new()
}

#[test]
fn bootstrap_hydrates_quota_state_from_sink_before_startup() {
    let mut state = AxumMcpHostState::new(empty_handler_factory);
    let sink = Arc::new(SeededQuotaSink::with_rows(vec![PersistedTokenQuotaState {
        token: "builder-token".to_owned(),
        window_start_ms: 444,
        count: 7,
    }]));
    let sink_for_state: Arc<dyn MrtDispatchQuotaEventSink> = sink;
    state.quota_event_sink = sink_for_state;

    let runner = RecoveryRunner::default();

    let decision = bootstrap_transport_with_runner(
        &BTreeMap::new(),
        3333,
        state,
        AxumHostConfig::default(),
        &runner,
    )
    .expect("bootstrap should succeed with seeded sink");

    assert_eq!(decision.transport.mode, TransportMode::Stdio);

    let record = runner.record();
    assert_eq!(record.stdio_starts, 1);
    assert_eq!(record.stream_starts, 0);
    assert_eq!(record.observed_builder_count, Some(7));
}

#[test]
fn bootstrap_ignores_invalid_hydration_rows_and_starts_cleanly() {
    let mut state = AxumMcpHostState::new(empty_handler_factory);
    let sink = Arc::new(SeededQuotaSink::with_rows(vec![PersistedTokenQuotaState {
        token: String::new(),
        window_start_ms: 444,
        count: 7,
    }]));
    let sink_for_state: Arc<dyn MrtDispatchQuotaEventSink> = sink;
    state.quota_event_sink = sink_for_state;

    let runner = RecoveryRunner::default();

    bootstrap_transport_with_runner(
        &BTreeMap::new(),
        3333,
        state,
        AxumHostConfig::default(),
        &runner,
    )
    .expect("bootstrap should stay fail-closed and ignore invalid sink rows");

    let record = runner.record();
    assert_eq!(record.stdio_starts, 1);
    assert_eq!(record.observed_builder_count, None);
}
