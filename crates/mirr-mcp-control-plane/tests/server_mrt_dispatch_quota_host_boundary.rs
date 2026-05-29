use std::sync::{Arc, Mutex};

use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_executor::{
    MrtRuntimeAdmissionConfig, MrtRuntimeAdmissionError, MrtRuntimeAdmissionState,
};
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_quota_host_boundary::{
    MrtDispatchQuotaHostBoundary, QuotaHostBoundaryAdmissionDecision,
};
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_quota_store::{
    MrtDispatchQuotaEventSink, PersistedTokenQuotaState,
};

#[derive(Default)]
struct RecordingQuotaSink {
    persisted: Mutex<Vec<(String, u64, u32)>>,
    hydrate_rows: Mutex<Vec<PersistedTokenQuotaState>>,
}

impl RecordingQuotaSink {
    fn persisted_rows(&self) -> Vec<(String, u64, u32)> {
        self.persisted.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
    }

    fn set_hydrate_rows(&self, rows: Vec<PersistedTokenQuotaState>) {
        let mut guard = self.hydrate_rows.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *guard = rows;
    }
}

impl MrtDispatchQuotaEventSink for RecordingQuotaSink {
    fn persist_token_quota(
        &self,
        token: &str,
        window_start_ms: u64,
        count: u32,
    ) -> Result<(), String> {
        let mut guard = self.persisted.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.push((token.to_owned(), window_start_ms, count));
        Ok(())
    }

    fn load_recent_token_quota_rows(
        &self,
        limit: usize,
    ) -> Result<Vec<PersistedTokenQuotaState>, String> {
        let guard = self.hydrate_rows.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        Ok(guard.iter().take(limit).cloned().collect())
    }
}

fn host_boundary_for_test(
    sink: Arc<RecordingQuotaSink>,
    config: MrtRuntimeAdmissionConfig,
) -> MrtDispatchQuotaHostBoundary {
    let sink_for_boundary: Arc<dyn MrtDispatchQuotaEventSink> = sink;
    MrtDispatchQuotaHostBoundary::new(
        Arc::new(Mutex::new(MrtRuntimeAdmissionState::default())),
        config,
        sink_for_boundary,
    )
}

#[test]
fn quota_host_boundary_persists_decisions_and_rejects_above_limit() {
    let sink = Arc::new(RecordingQuotaSink::default());
    let boundary = host_boundary_for_test(
        sink.clone(),
        MrtRuntimeAdmissionConfig {
            max_concurrent_per_key: 2,
            max_requests_per_token: 2,
            token_quota_window_ms: 100,
        },
    );

    let first =
        boundary.enforce_for_token("builder-token", 1_000).expect("first request should pass");
    assert_eq!(
        first,
        QuotaHostBoundaryAdmissionDecision {
            token: "builder-token".to_owned(),
            window_start_ms: 1_000,
            count: 1,
        }
    );

    let second = boundary
        .enforce_for_token("builder-token", 1_050)
        .expect("second request in window should pass");
    assert_eq!(second.count, 2);

    let third = boundary
        .enforce_for_token("builder-token", 1_090)
        .expect_err("third request in same window should fail closed");
    assert_eq!(third, MrtRuntimeAdmissionError::TokenQuotaExceeded);

    let persisted = sink.persisted_rows();
    assert_eq!(persisted.len(), 3);
    assert_eq!(persisted[0], ("builder-token".to_owned(), 1_000, 1));
    assert_eq!(persisted[1], ("builder-token".to_owned(), 1_000, 2));
    assert_eq!(persisted[2], ("builder-token".to_owned(), 1_000, 3));
}

#[test]
fn quota_host_boundary_hydrates_from_sink_before_enforcement() {
    let sink = Arc::new(RecordingQuotaSink::default());
    sink.set_hydrate_rows(vec![PersistedTokenQuotaState {
        token: "builder-token".to_owned(),
        window_start_ms: 700,
        count: 9,
    }]);

    let boundary = host_boundary_for_test(
        sink,
        MrtRuntimeAdmissionConfig {
            max_concurrent_per_key: 2,
            max_requests_per_token: 10,
            token_quota_window_ms: 1_000,
        },
    );

    boundary.hydrate_from_sink(32).expect("quota hydration should succeed");

    let decision = boundary
        .enforce_for_token("builder-token", 900)
        .expect("hydrated row should continue from persisted count");

    assert_eq!(decision.window_start_ms, 700);
    assert_eq!(decision.count, 10);
}

#[test]
fn quota_host_boundary_uses_anon_token_for_missing_api_key() {
    let sink = Arc::new(RecordingQuotaSink::default());
    let boundary = host_boundary_for_test(
        sink,
        MrtRuntimeAdmissionConfig {
            max_concurrent_per_key: 2,
            max_requests_per_token: 1,
            token_quota_window_ms: 10,
        },
    );

    let decision = boundary
        .enforce_for_api_key(None, 100)
        .expect("anonymous request should pass first quota check");

    assert_eq!(decision.token, "__anon");
    assert_eq!(decision.count, 1);
}

#[test]
fn quota_host_boundary_fails_closed_on_sink_hydration_error() {
    struct FailingHydrationSink;

    impl MrtDispatchQuotaEventSink for FailingHydrationSink {
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
            _limit: usize,
        ) -> Result<Vec<PersistedTokenQuotaState>, String> {
            Err("simulated_quota_read_failure".to_owned())
        }
    }

    let sink: Arc<dyn MrtDispatchQuotaEventSink> = Arc::new(FailingHydrationSink);
    let boundary = MrtDispatchQuotaHostBoundary::new(
        Arc::new(Mutex::new(MrtRuntimeAdmissionState::default())),
        MrtRuntimeAdmissionConfig {
            max_concurrent_per_key: 2,
            max_requests_per_token: 1,
            token_quota_window_ms: 10,
        },
        sink,
    );

    let error = boundary.hydrate_from_sink(8).expect_err("quota hydration errors must fail closed");
    assert_eq!(error, MrtRuntimeAdmissionError::InvalidRuntimeLimits);
}

#[test]
fn quota_host_boundary_fails_closed_on_sink_persist_error() {
    struct FailingPersistSink;

    impl MrtDispatchQuotaEventSink for FailingPersistSink {
        fn persist_token_quota(
            &self,
            _token: &str,
            _window_start_ms: u64,
            _count: u32,
        ) -> Result<(), String> {
            Err("simulated_quota_write_failure".to_owned())
        }

        fn load_recent_token_quota_rows(
            &self,
            _limit: usize,
        ) -> Result<Vec<PersistedTokenQuotaState>, String> {
            Ok(Vec::new())
        }
    }

    let sink: Arc<dyn MrtDispatchQuotaEventSink> = Arc::new(FailingPersistSink);
    let boundary = MrtDispatchQuotaHostBoundary::new(
        Arc::new(Mutex::new(MrtRuntimeAdmissionState::default())),
        MrtRuntimeAdmissionConfig {
            max_concurrent_per_key: 2,
            max_requests_per_token: 3,
            token_quota_window_ms: 1_000,
        },
        sink,
    );

    let error = boundary
        .enforce_for_token("builder-token", 1_000)
        .expect_err("quota persistence failures must fail closed");
    assert_eq!(error, MrtRuntimeAdmissionError::InvalidRuntimeLimits);
}
