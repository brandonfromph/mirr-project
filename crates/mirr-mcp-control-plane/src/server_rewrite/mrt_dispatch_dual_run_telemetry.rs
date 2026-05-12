#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Categorization of parity drift between dual-run paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DriftCategory {
    /// Both paths returned identical results in same order.
    NoDrift,
    /// Results match but in different order (RRF/reranking variance).
    MinorReordering,
    /// Paths returned different number of results.
    ResultCountMismatch,
    /// Freshness signals differ (fresh vs stale index states).
    FreshnessMismatch,
    /// Results differ in quality or relevance (semantic drift).
    QualityDrift,
}

/// Execution details for a single path in dual-run mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathExecutionEvent {
    /// Name of the path: "legacy" or "new".
    pub path_name: String,

    /// Whether execution succeeded (true) or failed (false).
    pub success: bool,

    /// Execution latency in milliseconds.
    pub latency_ms: u64,

    /// Number of results returned before reranking/truncation.
    pub result_count: usize,

    /// Whether results were truncated due to context budget.
    pub truncated: bool,

    /// Error message if execution failed.
    pub error: Option<String>,
}

/// Parity comparison metrics between both paths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParityMetrics {
    /// Whether both paths returned structurally identical responses.
    pub paths_match: bool,

    /// Whether result counts are equal.
    pub result_count_match: bool,

    /// Signed difference: new_count - legacy_count.
    pub result_count_diff: i32,

    /// Whether truncation status matches between paths.
    pub truncation_match: bool,

    /// Categorization of observed drift.
    pub drift_category: DriftCategory,

    /// Whether top-k results are reordered between paths.
    pub top_k_reordered: bool,

    /// Percentage of legacy results also in new results (0-100).
    pub result_overlap_percent: u32,
}

/// Complete dual-run telemetry capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Builder/configuration struct for creating DualRunTelemetry.
pub struct DualRunTelemetryBuilder {
    pub request_id: String,
    pub tool_name: String,
    pub query_snippet: String,
    pub legacy_path: PathExecutionEvent,
    pub new_path: PathExecutionEvent,
    pub parity_metrics: ParityMetrics,
    pub primary_path_returned: String,
    pub timestamp_ms: u64,
}

pub struct DualRunTelemetry {
    /// Unique request identifier.
    pub request_id: String,

    /// Tool name being invoked.
    pub tool_name: String,

    /// Original query text (truncated to 200 chars for logging).
    pub query_snippet: String,

    /// Execution details for legacy path (mrt_brain_get).
    pub legacy_path: PathExecutionEvent,

    /// Execution details for new path (mrt_kb_query).
    pub new_path: PathExecutionEvent,

    /// Parity comparison metrics.
    pub parity_metrics: ParityMetrics,

    /// Which path was returned to caller: "legacy", "new", or "fallback".
    pub primary_path_returned: String,

    /// Unix timestamp (milliseconds) when request was received.
    pub timestamp_ms: u64,
}

impl DualRunTelemetry {
    /// Create a new dual-run telemetry record from builder.
    pub fn from_builder(builder: DualRunTelemetryBuilder) -> Self {
        Self {
            request_id: builder.request_id,
            tool_name: builder.tool_name,
            query_snippet: builder.query_snippet,
            legacy_path: builder.legacy_path,
            new_path: builder.new_path,
            parity_metrics: builder.parity_metrics,
            primary_path_returned: builder.primary_path_returned,
            timestamp_ms: builder.timestamp_ms,
        }
    }

    /// Summarize execution outcome (for logging).
    pub fn summary(&self) -> String {
        format!(
            "dual-run: {} | legacy={}/{}ms | new={}/{}ms | drift={:?} | returned={}",
            self.tool_name,
            if self.legacy_path.success { "ok" } else { "fail" },
            self.legacy_path.latency_ms,
            if self.new_path.success { "ok" } else { "fail" },
            self.new_path.latency_ms,
            self.parity_metrics.drift_category,
            self.primary_path_returned
        )
    }
}

/// Configuration for dual-run mode.
#[derive(Debug, Clone)]
pub struct DualRunConfig {
    /// Enable dual-run for mrt_kb_query requests.
    pub enabled: bool,

    /// Per-path timeout in milliseconds (default 5000).
    pub per_path_timeout_ms: u64,

    /// If true, log all parity metrics; if false, only log anomalies.
    pub log_all_metrics: bool,

    /// Drift categories to flag as anomalies for alerting.
    pub anomaly_categories: Vec<DriftCategory>,
}

impl Default for DualRunConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            per_path_timeout_ms: 5000,
            log_all_metrics: true,
            anomaly_categories: vec![
                DriftCategory::ResultCountMismatch,
                DriftCategory::QualityDrift,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telemetry_summary_includes_all_details() {
        let legacy = PathExecutionEvent {
            path_name: "legacy".to_string(),
            success: true,
            latency_ms: 100,
            result_count: 5,
            truncated: false,
            error: None,
        };

        let new = PathExecutionEvent {
            path_name: "new".to_string(),
            success: true,
            latency_ms: 150,
            result_count: 5,
            truncated: false,
            error: None,
        };

        let parity = ParityMetrics {
            paths_match: true,
            result_count_match: true,
            result_count_diff: 0,
            truncation_match: true,
            drift_category: DriftCategory::NoDrift,
            top_k_reordered: false,
            result_overlap_percent: 100,
        };

        let telemetry = DualRunTelemetry::from_builder(DualRunTelemetryBuilder {
            request_id: "req123".to_string(),
            tool_name: "mrt_kb_query".to_string(),
            query_snippet: "test query".to_string(),
            legacy_path: legacy,
            new_path: new,
            parity_metrics: parity,
            primary_path_returned: "new".to_string(),
            timestamp_ms: 1234567890,
        });

        let summary = telemetry.summary();
        assert!(summary.contains("mrt_kb_query"));
        assert!(summary.contains("ok/100ms"));
        assert!(summary.contains("NoDrift"));
    }

    #[test]
    fn parity_metrics_detects_count_mismatch() {
        let parity = ParityMetrics {
            paths_match: false,
            result_count_match: false,
            result_count_diff: 3,
            truncation_match: true,
            drift_category: DriftCategory::ResultCountMismatch,
            top_k_reordered: false,
            result_overlap_percent: 80,
        };

        assert_eq!(parity.drift_category, DriftCategory::ResultCountMismatch);
        assert_eq!(parity.result_count_diff, 3);
    }

    #[test]
    fn dual_run_config_defaults() {
        let config = DualRunConfig::default();
        assert!(!config.enabled); // Disabled by default
        assert_eq!(config.per_path_timeout_ms, 5000);
        assert!(config.log_all_metrics);
    }
}
