#![forbid(unsafe_code)]

use std::sync::{Arc, Mutex};

use super::mrt_dispatch_invocation_executor::{
    enforce_token_quota, read_token_quota_state, resolve_runtime_token, MrtRuntimeAdmissionConfig,
    MrtRuntimeAdmissionError, MrtRuntimeAdmissionState,
};
use super::mrt_dispatch_quota_store::{bounded_recent_quota_rows, MrtDispatchQuotaEventSink};

pub const DEFAULT_QUOTA_HYDRATE_ROWS: usize = 64;
pub const MAX_QUOTA_HYDRATE_ROWS: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuotaHostBoundaryAdmissionDecision {
    pub token: String,
    pub window_start_ms: u64,
    pub count: u32,
}

#[derive(Clone)]
pub struct MrtDispatchQuotaHostBoundary {
    admission_state: Arc<Mutex<MrtRuntimeAdmissionState>>,
    admission_config: MrtRuntimeAdmissionConfig,
    quota_event_sink: Arc<dyn MrtDispatchQuotaEventSink>,
}

impl MrtDispatchQuotaHostBoundary {
    pub fn new(
        admission_state: Arc<Mutex<MrtRuntimeAdmissionState>>,
        admission_config: MrtRuntimeAdmissionConfig,
        quota_event_sink: Arc<dyn MrtDispatchQuotaEventSink>,
    ) -> Self {
        Self { admission_state, admission_config, quota_event_sink }
    }

    pub fn hydrate_from_sink(&self, max_rows: usize) -> Result<(), MrtRuntimeAdmissionError> {
        let bounded_rows = max_rows.clamp(1, MAX_QUOTA_HYDRATE_ROWS);
        let rows = bounded_recent_quota_rows(self.quota_event_sink.as_ref(), bounded_rows)
            .map_err(|_| MrtRuntimeAdmissionError::InvalidRuntimeLimits)?;
        if rows.is_empty() {
            return Ok(());
        }

        let mut guard =
            self.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        for row in rows {
            guard.token_quota.insert(
                row.token,
                super::mrt_dispatch_invocation_executor::TokenQuotaState {
                    window_start_ms: row.window_start_ms,
                    count: row.count,
                },
            );
        }

        Ok(())
    }

    pub fn enforce_for_api_key(
        &self,
        api_key: Option<&str>,
        now_ms: u64,
    ) -> Result<QuotaHostBoundaryAdmissionDecision, MrtRuntimeAdmissionError> {
        let token = resolve_runtime_token(api_key);
        self.enforce_for_token(&token, now_ms)
    }

    pub fn enforce_for_token(
        &self,
        token: &str,
        now_ms: u64,
    ) -> Result<QuotaHostBoundaryAdmissionDecision, MrtRuntimeAdmissionError> {
        let resolved_token = resolve_runtime_token(Some(token));
        let mut guard =
            self.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        let result =
            enforce_token_quota(&resolved_token, now_ms, &mut guard, &self.admission_config);

        if let Some(snapshot) = read_token_quota_state(&resolved_token, &guard) {
            self.quota_event_sink
                .persist_token_quota(&resolved_token, snapshot.window_start_ms, snapshot.count)
                .map_err(|_| MrtRuntimeAdmissionError::InvalidRuntimeLimits)?;
        }

        result?;

        let snapshot = read_token_quota_state(&resolved_token, &guard)
            .ok_or(MrtRuntimeAdmissionError::InvalidRuntimeLimits)?;

        Ok(QuotaHostBoundaryAdmissionDecision {
            token: resolved_token,
            window_start_ms: snapshot.window_start_ms,
            count: snapshot.count,
        })
    }
}
