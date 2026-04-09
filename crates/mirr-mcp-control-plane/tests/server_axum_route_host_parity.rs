use std::collections::BTreeMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use hyper::body::to_bytes;
use mirror::policy::Role;
use mirror::server_rewrite::axum_route_host::{
    build_axum_mcp_router, AxumHostConfig, AxumMcpHostState,
};
use mirror::server_rewrite::mrt_dispatch_audit_store::{
    MrtDispatchAuditEventSink, SqliteMrtDispatchAuditEventSink, SqliteMrtDispatchAuditStore,
};
use mirror::server_rewrite::mrt_dispatch_invocation_executor::{
    MrtDispatchExecutionConfig, MrtDispatchExecutionError, MrtDispatchExecutionResult,
    MrtRuntimeAdmissionConfig,
};
use mirror::server_rewrite::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;
use mirror::server_rewrite::mrt_dispatch_quota_store::{
    MrtDispatchQuotaEventSink, SqliteMrtDispatchQuotaEventSink, SqliteMrtDispatchQuotaStore,
};
use mirror::server_rewrite::mrt_dispatch_route_handler::MrtDispatchAuditEvent;
use mirror::server_rewrite::rpc_dispatch_bridge::{RpcHandlerMap, RpcHandlerResponse};
use mirror::server_rewrite::rpc_role_gate::{RoleTokenMap, VerifiedPrincipal};
use serde_json::Value;
use tower::ServiceExt;

#[derive(Default)]
struct RecordingAuditSink {
    events: Mutex<Vec<MrtDispatchAuditEvent>>,
}

impl RecordingAuditSink {
    fn event_kinds(&self) -> Vec<String> {
        let guard = self.events.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.iter().map(|event| event.kind.to_owned()).collect()
    }
}

impl MrtDispatchAuditEventSink for RecordingAuditSink {
    fn append(&self, event: &MrtDispatchAuditEvent) {
        let mut guard = self.events.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.push(event.clone());
    }
}

#[derive(Default)]
struct RecordingQuotaSink {
    rows: Mutex<Vec<(String, u64, u32)>>,
}

impl RecordingQuotaSink {
    fn rows(&self) -> Vec<(String, u64, u32)> {
        let guard = self.rows.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.clone()
    }
}

impl MrtDispatchQuotaEventSink for RecordingQuotaSink {
    fn persist_token_quota(
        &self,
        token: &str,
        window_start_ms: u64,
        count: u32,
    ) -> Result<(), String> {
        let mut guard = self.rows.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.push((token.to_owned(), window_start_ms, count));
        Ok(())
    }
}

fn unique_sqlite_audit_path() -> String {
    let now_nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };

    let mut path = std::env::temp_dir();
    path.push(format!("mirr-mcp-control-plane-audit-{}.db", now_nanos));
    path.to_string_lossy().to_string()
}

fn unique_sqlite_quota_path() -> String {
    let now_nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };

    let mut path = std::env::temp_dir();
    path.push(format!("mirr-mcp-control-plane-quota-{}.db", now_nanos));
    path.to_string_lossy().to_string()
}

fn handler_factory() -> RpcHandlerMap<String> {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    handlers.insert(
        "mcp_schema".to_owned(),
        Box::new(|_req| RpcHandlerResponse { status: 200, body: "schema_ok".to_owned() }),
    );
    handlers
}

fn fake_execute_invocation(
    _plan: &MrtDispatchInvocationPlan,
    _config: &MrtDispatchExecutionConfig,
) -> Result<MrtDispatchExecutionResult, MrtDispatchExecutionError> {
    Ok(MrtDispatchExecutionResult {
        stdout: "audit_stdout".to_owned(),
        stderr: String::new(),
        exit_code: 0,
    })
}

fn host_state_with_tokens() -> AxumMcpHostState {
    let mut role_tokens: RoleTokenMap = BTreeMap::new();
    role_tokens.insert(
        "builder-token".to_owned(),
        VerifiedPrincipal { id: "builder".to_owned(), role: Role::Builder },
    );
    role_tokens.insert(
        "committer-token".to_owned(),
        VerifiedPrincipal { id: "committer".to_owned(), role: Role::Committer },
    );

    let mut state = AxumMcpHostState::with_role_tokens(handler_factory, role_tokens);
    state.execute_invocation = fake_execute_invocation;
    state
}

fn parse_json_body(bytes: Vec<u8>) -> Value {
    serde_json::from_slice(&bytes).expect("response body should be valid JSON")
}

fn parse_stable_error_code(body: &Value) -> Option<String> {
    let message =
        body.get("error").and_then(|value| value.get("message")).and_then(Value::as_str)?;
    let stable: Value = serde_json::from_str(message).ok()?;
    stable.get("error_code").and_then(Value::as_str).map(ToOwned::to_owned)
}

#[tokio::test]
async fn mcp_route_dispatches_through_canonical_pipeline_for_mrt_tools() {
    let app = build_axum_mcp_router(host_state_with_tokens(), AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":1,\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body()).await.expect("response body should be readable");
    let parsed = parse_json_body(body.to_vec());
    assert_eq!(parsed.get("jsonrpc").and_then(Value::as_str), Some("2.0"));
    assert_eq!(parsed.get("id").and_then(Value::as_i64), Some(1));

    let result_text = parsed
        .get("result")
        .and_then(Value::as_str)
        .expect("canonical route result should be encoded as a JSON string");
    let result_payload: Value = serde_json::from_str(result_text)
        .expect("canonical route result should decode into JSON payload");
    assert_eq!(result_payload.get("tool").and_then(Value::as_str), Some("mrt_audit"));
    assert_eq!(result_payload.get("exit_code").and_then(Value::as_i64), Some(0));
}

#[tokio::test]
async fn mcp_stream_route_is_fail_closed_for_unknown_method() {
    let app = build_axum_mcp_router(host_state_with_tokens(), AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp_stream")
        .header("content-type", "application/json")
        .body(Body::from("{\"id\":\"x\",\"method\":\"unknown_method\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = to_bytes(response.into_body()).await.expect("response body should be readable");
    let parsed = parse_json_body(body.to_vec());
    assert_eq!(parse_stable_error_code(&parsed).as_deref(), Some("validation_unknown_method"));
}

#[tokio::test]
async fn canonical_route_requires_api_key_for_mrt_dispatch() {
    let app = build_axum_mcp_router(host_state_with_tokens(), AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .body(Body::from("{\"id\":\"auth\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = to_bytes(response.into_body()).await.expect("response body should be readable");
    let parsed = parse_json_body(body.to_vec());
    assert_eq!(parse_stable_error_code(&parsed).as_deref(), Some("auth_missing_api_key"));
}

#[tokio::test]
async fn canonical_route_enforces_schema_before_execution() {
    let app = build_axum_mcp_router(host_state_with_tokens(), AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "committer-token")
        .body(Body::from("{\"id\":\"schema\",\"method\":\"mrt_brain_get\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = to_bytes(response.into_body()).await.expect("response body should be readable");
    let parsed = parse_json_body(body.to_vec());
    assert_eq!(parse_stable_error_code(&parsed).as_deref(), Some("validation_schema"));
}

#[tokio::test]
async fn canonical_route_enforces_token_quota_limits() {
    let mut state = host_state_with_tokens();
    state.admission_config = MrtRuntimeAdmissionConfig {
        max_concurrent_per_key: 2,
        max_requests_per_token: 1,
        token_quota_window_ms: 60_000,
    };

    let app = build_axum_mcp_router(state, AxumHostConfig::default());

    let request_1 = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"q1\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");
    let response_1 = app.clone().oneshot(request_1).await.expect("request should complete");
    assert_eq!(response_1.status(), StatusCode::OK);

    let request_2 = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"q2\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");
    let response_2 = app.oneshot(request_2).await.expect("request should complete");
    assert_eq!(response_2.status(), StatusCode::TOO_MANY_REQUESTS);

    let body = to_bytes(response_2.into_body()).await.expect("response body should be readable");
    let parsed = parse_json_body(body.to_vec());
    assert_eq!(parse_stable_error_code(&parsed).as_deref(), Some("limit_token_quota_exceeded"));
}

#[tokio::test]
async fn route_host_fail_closes_on_invalid_json_payload() {
    let app = build_axum_mcp_router(host_state_with_tokens(), AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .body(Body::from("this-is-not-json"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = to_bytes(response.into_body()).await.expect("response body should be readable");
    let text = String::from_utf8(body.to_vec()).expect("response body should be utf8");
    assert!(text.contains("Invalid JSON-RPC request body."));
}

#[tokio::test]
async fn route_host_enforces_bounded_body_limit() {
    let app = build_axum_mcp_router(
        host_state_with_tokens(),
        AxumHostConfig { max_request_body_bytes: 16 },
    );

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .body(Body::from("{\"id\":1,\"method\":\"mrt_audit\",\"params\":{\"x\":\"1234567890\"}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn canonical_route_appends_audit_events_to_configured_sink() {
    let mut state = host_state_with_tokens();
    let recording_sink = Arc::new(RecordingAuditSink::default());
    let sink_for_state: Arc<dyn MrtDispatchAuditEventSink> = recording_sink.clone();
    state.audit_event_sink = sink_for_state;

    let app = build_axum_mcp_router(state, AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"audit\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::OK);

    let kinds = recording_sink.event_kinds();
    assert!(kinds.contains(&"mrt_dispatch_start".to_owned()));
    assert!(kinds.contains(&"mrt_dispatch_complete".to_owned()));
}

#[tokio::test]
async fn canonical_route_persists_audit_events_in_sqlite_sink() {
    let sqlite_path = unique_sqlite_audit_path();

    let mut state = host_state_with_tokens();
    let sqlite_sink = SqliteMrtDispatchAuditEventSink::open(&sqlite_path)
        .expect("sqlite audit sink should initialize");
    state.audit_event_sink = Arc::new(sqlite_sink);

    let app = build_axum_mcp_router(state, AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"sqlite\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::OK);

    let store =
        SqliteMrtDispatchAuditStore::open(&sqlite_path).expect("sqlite audit store should re-open");
    let rows = store.recent_rows(8).expect("sqlite audit rows should load");
    let kinds = rows.iter().map(|row| row.kind.clone()).collect::<Vec<String>>();

    assert!(kinds.contains(&"mrt_dispatch_start".to_owned()));
    assert!(kinds.contains(&"mrt_dispatch_complete".to_owned()));

    let _ = fs::remove_file(sqlite_path);
}

#[tokio::test]
async fn canonical_route_persists_quota_updates_to_configured_sink() {
    let mut state = host_state_with_tokens();
    let recording_sink = Arc::new(RecordingQuotaSink::default());
    let sink_for_state: Arc<dyn MrtDispatchQuotaEventSink> = recording_sink.clone();
    state.quota_event_sink = sink_for_state;

    let app = build_axum_mcp_router(state, AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"quota\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::OK);

    let rows = recording_sink.rows();
    assert!(!rows.is_empty());
    assert_eq!(rows[0].0, "builder-token");
    assert!(rows[0].2 >= 1);
}

#[tokio::test]
async fn canonical_route_persists_quota_updates_in_sqlite_sink() {
    let sqlite_path = unique_sqlite_quota_path();

    let mut state = host_state_with_tokens();
    let sqlite_sink = SqliteMrtDispatchQuotaEventSink::open(&sqlite_path)
        .expect("sqlite quota sink should initialize");
    state.quota_event_sink = Arc::new(sqlite_sink);

    let app = build_axum_mcp_router(state, AxumHostConfig::default());

    let request_1 = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"quota-sqlite-1\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");
    let response_1 = app.clone().oneshot(request_1).await.expect("request should complete");
    assert_eq!(response_1.status(), StatusCode::OK);

    let request_2 = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"quota-sqlite-2\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");
    let response_2 = app.oneshot(request_2).await.expect("request should complete");
    assert_eq!(response_2.status(), StatusCode::OK);

    let store =
        SqliteMrtDispatchQuotaStore::open(&sqlite_path).expect("sqlite quota store should re-open");
    let row = store
        .read_token_quota("builder-token")
        .expect("sqlite quota row should be readable")
        .expect("builder-token quota row should exist");

    assert!(row.count >= 2);
    assert!(row.window_start_ms > 0);

    let _ = fs::remove_file(sqlite_path);
}

#[test]
fn host_state_hydrates_token_quota_from_sqlite_sink() {
    let sqlite_path = unique_sqlite_quota_path();

    let sqlite_sink = SqliteMrtDispatchQuotaEventSink::open(&sqlite_path)
        .expect("sqlite quota sink should initialize");
    sqlite_sink
        .persist_token_quota("builder-token", 777, 9)
        .expect("sqlite seed quota should persist");

    let mut state = host_state_with_tokens();
    state.quota_event_sink = Arc::new(sqlite_sink);

    state
        .hydrate_token_quota_state_from_sink(16)
        .expect("quota hydration should succeed");

    let guard = state.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let quota = guard
        .token_quota
        .get("builder-token")
        .expect("builder token quota should hydrate from sqlite sink");

    assert_eq!(quota.window_start_ms, 777);
    assert_eq!(quota.count, 9);

    let _ = fs::remove_file(sqlite_path);
}

#[tokio::test]
async fn canonical_route_enforces_hydrated_quota_before_first_request() {
    let sqlite_path = unique_sqlite_quota_path();

    let sqlite_sink = SqliteMrtDispatchQuotaEventSink::open(&sqlite_path)
        .expect("sqlite quota sink should initialize");
    sqlite_sink
        .persist_token_quota("builder-token", u64::MAX, 1)
        .expect("sqlite seed quota should persist");

    let mut state = host_state_with_tokens();
    state.admission_config = MrtRuntimeAdmissionConfig {
        max_concurrent_per_key: 2,
        max_requests_per_token: 1,
        token_quota_window_ms: 60_000,
    };
    state.quota_event_sink = Arc::new(sqlite_sink);
    state
        .hydrate_token_quota_state_from_sink(16)
        .expect("quota hydration should succeed");

    let app = build_axum_mcp_router(state, AxumHostConfig::default());

    let request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("x-mcp-api-key", "builder-token")
        .body(Body::from("{\"id\":\"hydrated-quota\",\"method\":\"mrt_audit\",\"params\":{}}"))
        .expect("request should build");

    let response = app.oneshot(request).await.expect("request should complete");
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);

    let body = to_bytes(response.into_body()).await.expect("response body should be readable");
    let parsed = parse_json_body(body.to_vec());
    assert_eq!(parse_stable_error_code(&parsed).as_deref(), Some("limit_token_quota_exceeded"));

    let _ = fs::remove_file(sqlite_path);
}
