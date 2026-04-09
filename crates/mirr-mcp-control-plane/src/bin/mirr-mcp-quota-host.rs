#![forbid(unsafe_code)]

use std::env;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use mirror::envelope::{MRT_EXECUTION_ERROR, TOKEN_QUOTA_LIMIT_ERROR};
use mirror::server_rewrite::mrt_dispatch_invocation_executor::{
    MrtRuntimeAdmissionConfig, MrtRuntimeAdmissionError, MrtRuntimeAdmissionState,
    DEFAULT_MAX_CONCURRENT_PER_KEY,
};
use mirror::server_rewrite::mrt_dispatch_quota_host_boundary::{
    MrtDispatchQuotaHostBoundary, DEFAULT_QUOTA_HYDRATE_ROWS,
};
use mirror::server_rewrite::mrt_dispatch_quota_store::{
    MrtDispatchQuotaEventSink, SqliteMrtDispatchQuotaEventSink,
};
use serde_json::{json, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
struct QuotaHostCliInput {
    token: String,
    now_ms: u64,
    max_requests_per_token: u32,
    token_quota_window_ms: u64,
    quota_sqlite_path: String,
    hydrate_rows: usize,
}

fn now_unix_millis() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => match u64::try_from(duration.as_millis()) {
            Ok(value) => value,
            Err(_) => u64::MAX,
        },
        Err(_) => 0,
    }
}

fn parse_u32_arg(raw: &str, name: &str) -> Result<u32, String> {
    raw.parse::<u32>().map_err(|_| format!("invalid_{}_value", name))
}

fn parse_u64_arg(raw: &str, name: &str) -> Result<u64, String> {
    raw.parse::<u64>().map_err(|_| format!("invalid_{}_value", name))
}

fn parse_usize_arg(raw: &str, name: &str) -> Result<usize, String> {
    raw.parse::<usize>().map_err(|_| format!("invalid_{}_value", name))
}

fn next_value(args: &[String], index: usize, name: &str) -> Result<String, String> {
    if index + 1 >= args.len() {
        return Err(format!("missing_value_for_{}", name));
    }

    Ok(args[index + 1].clone())
}

fn parse_cli_args(args: &[String]) -> Result<QuotaHostCliInput, String> {
    let mut token = "__anon".to_owned();
    let mut now_ms = now_unix_millis();
    let mut max_requests_per_token = 30u32;
    let mut token_quota_window_ms = 60_000u64;
    let mut quota_sqlite_path = ".mcp_logs/quota_state.sqlite".to_owned();
    let mut hydrate_rows = DEFAULT_QUOTA_HYDRATE_ROWS;

    let mut index = 0usize;
    while index < args.len() {
        let arg = &args[index];
        if arg == "--token" {
            token = next_value(args, index, "token")?;
            index = index.saturating_add(2);
            continue;
        }

        if arg == "--now-ms" {
            let raw = next_value(args, index, "now-ms")?;
            now_ms = parse_u64_arg(&raw, "now_ms")?;
            index = index.saturating_add(2);
            continue;
        }

        if arg == "--max-requests-per-token" {
            let raw = next_value(args, index, "max-requests-per-token")?;
            max_requests_per_token = parse_u32_arg(&raw, "max_requests_per_token")?;
            index = index.saturating_add(2);
            continue;
        }

        if arg == "--token-quota-window-ms" {
            let raw = next_value(args, index, "token-quota-window-ms")?;
            token_quota_window_ms = parse_u64_arg(&raw, "token_quota_window_ms")?;
            index = index.saturating_add(2);
            continue;
        }

        if arg == "--quota-sqlite-path" {
            quota_sqlite_path = next_value(args, index, "quota-sqlite-path")?;
            index = index.saturating_add(2);
            continue;
        }

        if arg == "--hydrate-rows" {
            let raw = next_value(args, index, "hydrate-rows")?;
            hydrate_rows = parse_usize_arg(&raw, "hydrate_rows")?;
            index = index.saturating_add(2);
            continue;
        }

        return Err(format!("unknown_argument_{}", arg));
    }

    if token.is_empty() {
        token = "__anon".to_owned();
    }

    if max_requests_per_token == 0 || token_quota_window_ms == 0 {
        return Err("invalid_runtime_limits".to_owned());
    }

    if quota_sqlite_path.is_empty() {
        return Err("quota_sqlite_path_required".to_owned());
    }

    Ok(QuotaHostCliInput {
        token,
        now_ms,
        max_requests_per_token,
        token_quota_window_ms,
        quota_sqlite_path,
        hydrate_rows,
    })
}

fn error_response(error: MrtRuntimeAdmissionError) -> Value {
    match error {
        MrtRuntimeAdmissionError::TokenQuotaExceeded => json!({
            "ok": false,
            "error_code": TOKEN_QUOTA_LIMIT_ERROR.error_code,
            "message": TOKEN_QUOTA_LIMIT_ERROR.message,
            "details": null,
        }),
        MrtRuntimeAdmissionError::ConcurrencyLimitExceeded
        | MrtRuntimeAdmissionError::InvalidRuntimeLimits => json!({
            "ok": false,
            "error_code": MRT_EXECUTION_ERROR.error_code,
            "message": MRT_EXECUTION_ERROR.message,
            "details": "quota_host_boundary_invalid_runtime_limits",
        }),
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let input = parse_cli_args(&args)?;

    let sqlite_sink = SqliteMrtDispatchQuotaEventSink::open(&input.quota_sqlite_path)?;
    let sink_for_boundary: Arc<dyn MrtDispatchQuotaEventSink> = Arc::new(sqlite_sink);

    let boundary = MrtDispatchQuotaHostBoundary::new(
        Arc::new(Mutex::new(MrtRuntimeAdmissionState::default())),
        MrtRuntimeAdmissionConfig {
            max_concurrent_per_key: DEFAULT_MAX_CONCURRENT_PER_KEY,
            max_requests_per_token: input.max_requests_per_token,
            token_quota_window_ms: input.token_quota_window_ms,
        },
        sink_for_boundary,
    );

    boundary
        .hydrate_from_sink(input.hydrate_rows)
        .map_err(|_| "quota_state_hydration_failed".to_owned())?;

    let output = match boundary.enforce_for_token(&input.token, input.now_ms) {
        Ok(decision) => json!({
            "ok": true,
            "token": decision.token,
            "window_start_ms": decision.window_start_ms,
            "count": decision.count,
        }),
        Err(error) => error_response(error),
    };

    println!("{}", output);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
