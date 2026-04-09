use std::collections::BTreeMap;

use axum::body::Body;
use axum::http::{HeaderMap, HeaderName, HeaderValue, Request};
use hyper::body::to_bytes;
use mirror::policy::Role;
use mirror::server_rewrite::axum_route_host::{
    build_axum_mcp_router, dispatch_host_rpc_message, AxumHostConfig, AxumMcpHostState,
};
use mirror::server_rewrite::mrt_dispatch_invocation_executor::{
    MrtDispatchExecutionConfig, MrtDispatchExecutionError, MrtDispatchExecutionResult,
    MrtRuntimeAdmissionConfig, TokenQuotaState,
};
use mirror::server_rewrite::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;
use mirror::server_rewrite::rpc_dispatch_bridge::{RpcHandlerMap, RpcHandlerResponse};
use mirror::server_rewrite::rpc_role_gate::{RoleTokenMap, VerifiedPrincipal};
use mirror::server_rewrite::rpc_stdio_message_dispatch::{parse_stdio_rpc_line, StdioRpcResponse};
use serde_json::Value;
use tower::ServiceExt;

#[derive(Clone, Debug, Eq, PartialEq)]
struct HostBoundaryOutcome {
    status: u16,
    id: Option<Value>,
    result_text: Option<String>,
    result_tool: Option<String>,
    error_code: Option<String>,
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

fn host_state_for_case() -> AxumMcpHostState {
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

fn parse_error_code_from_message(message: &str) -> Option<String> {
    let parsed: Value = serde_json::from_str(message).ok()?;
    parsed.get("error_code").and_then(Value::as_str).map(ToOwned::to_owned)
}

fn parse_result_tool(result_text: Option<&str>) -> Option<String> {
    let text = result_text?;
    let parsed: Value = serde_json::from_str(text).ok()?;
    parsed.get("tool").and_then(Value::as_str).map(ToOwned::to_owned)
}

fn outcome_from_stdio(status: u16, response: StdioRpcResponse) -> HostBoundaryOutcome {
    let result_text = response.result;
    HostBoundaryOutcome {
        status,
        id: response.id,
        result_tool: parse_result_tool(result_text.as_deref()),
        error_code: response
            .error
            .as_ref()
            .and_then(|error| parse_error_code_from_message(&error.message)),
        result_text,
    }
}

fn outcome_from_stream(status: u16, response_json: &Value) -> HostBoundaryOutcome {
    let result_text = response_json.get("result").and_then(Value::as_str).map(ToOwned::to_owned);

    let error_code = response_json
        .get("error")
        .and_then(|error| error.get("message"))
        .and_then(Value::as_str)
        .and_then(parse_error_code_from_message);

    HostBoundaryOutcome {
        status,
        id: response_json.get("id").cloned().filter(|value| !value.is_null()),
        result_tool: parse_result_tool(result_text.as_deref()),
        error_code,
        result_text,
    }
}

fn header_map_from_case(case: &Value) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let Some(header_object) = case.get("headers").and_then(Value::as_object) else {
        return headers;
    };

    for (key, value) in header_object {
        let Some(header_text) = value.as_str() else {
            continue;
        };

        let Ok(name) = HeaderName::from_bytes(key.as_bytes()) else {
            continue;
        };
        let Ok(header_value) = HeaderValue::from_str(header_text) else {
            continue;
        };
        headers.insert(name, header_value);
    }

    headers
}

#[tokio::test]
async fn host_boundary_shared_fixture_parity_for_stdio_and_stream() {
    let fixture_json = include_str!("fixtures/host_boundary_transport_matrix.json");
    let fixture: Value = serde_json::from_str(fixture_json)
        .expect("shared host-boundary fixture should be valid JSON");
    let cases = fixture.as_array().expect("host-boundary fixture must be an array");

    for case in cases {
        let case_name =
            case.get("name").and_then(Value::as_str).expect("each fixture case requires a name");
        let payload =
            case.get("request").cloned().expect("each fixture case requires a request payload");
        let payload_text = payload.to_string();
        let headers = header_map_from_case(case);

        let stdio_state = host_state_for_case();
        let stdio_input = parse_stdio_rpc_line(&payload_text)
            .expect("fixture request payload should parse as stdio JSON-RPC input");
        let (stdio_status, stdio_response) =
            dispatch_host_rpc_message(&stdio_state, &headers, stdio_input);
        let stdio_outcome = outcome_from_stdio(stdio_status, stdio_response);

        let stream_state = host_state_for_case();
        let app = build_axum_mcp_router(stream_state, AxumHostConfig::default());
        let mut stream_request = Request::builder()
            .method("POST")
            .uri("/mcp")
            .header("content-type", "application/json")
            .body(Body::from(payload_text.clone()))
            .expect("stream request should build");
        for (name, value) in &headers {
            stream_request.headers_mut().insert(name.clone(), value.clone());
        }

        let stream_response =
            app.oneshot(stream_request).await.expect("stream request should complete");
        let stream_status = stream_response.status().as_u16();
        let stream_body = to_bytes(stream_response.into_body())
            .await
            .expect("stream response body should be readable");
        let stream_json: Value =
            serde_json::from_slice(&stream_body).expect("stream response body should be JSON");
        let stream_outcome = outcome_from_stream(stream_status, &stream_json);

        assert_eq!(
            stdio_outcome, stream_outcome,
            "stdio/stream host-boundary mismatch for fixture case: {}",
            case_name
        );

        let expect =
            case.get("expect").expect("each fixture case requires expected outcome section");
        let expected_status = expect
            .get("status")
            .and_then(Value::as_u64)
            .expect("expected status must be an integer") as u16;
        assert_eq!(
            stdio_outcome.status, expected_status,
            "unexpected status for fixture case: {}",
            case_name
        );

        if let Some(expected_error_code) = expect.get("error_code").and_then(Value::as_str) {
            assert_eq!(
                stdio_outcome.error_code.as_deref(),
                Some(expected_error_code),
                "unexpected stable error code for fixture case: {}",
                case_name
            );
        }

        if let Some(expected_result_tool) = expect.get("result_tool").and_then(Value::as_str) {
            assert_eq!(
                stdio_outcome.result_tool.as_deref(),
                Some(expected_result_tool),
                "unexpected result tool marker for fixture case: {}",
                case_name
            );
        }

        if let Some(expected_result_contains) =
            expect.get("result_contains").and_then(Value::as_str)
        {
            assert!(
                stdio_outcome
                    .result_text
                    .as_deref()
                    .unwrap_or("")
                    .contains(expected_result_contains),
                "result payload mismatch for fixture case: {}",
                case_name
            );
        }
    }
}

#[tokio::test]
async fn host_boundary_quota_rejection_parity_matches_between_stdio_and_stream() {
    let mut stdio_state = host_state_for_case();
    stdio_state.admission_config = MrtRuntimeAdmissionConfig {
        max_concurrent_per_key: 2,
        max_requests_per_token: 1,
        token_quota_window_ms: 60_000,
    };
    {
        let mut guard =
            stdio_state.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.token_quota.insert(
            "builder-token".to_owned(),
            TokenQuotaState { window_start_ms: u64::MAX, count: 1 },
        );
    }

    let payload_text = "{\"id\":\"quota\",\"method\":\"mrt_audit\",\"params\":{}}".to_owned();
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-mcp-api-key"),
        HeaderValue::from_static("builder-token"),
    );

    let stdio_input =
        parse_stdio_rpc_line(&payload_text).expect("request should parse as stdio input");
    let (stdio_status, stdio_response) =
        dispatch_host_rpc_message(&stdio_state, &headers, stdio_input);
    let stdio_outcome = outcome_from_stdio(stdio_status, stdio_response);

    let mut stream_state = host_state_for_case();
    stream_state.admission_config = MrtRuntimeAdmissionConfig {
        max_concurrent_per_key: 2,
        max_requests_per_token: 1,
        token_quota_window_ms: 60_000,
    };
    {
        let mut guard =
            stream_state.admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.token_quota.insert(
            "builder-token".to_owned(),
            TokenQuotaState { window_start_ms: u64::MAX, count: 1 },
        );
    }

    let app = build_axum_mcp_router(stream_state, AxumHostConfig::default());
    let mut stream_request = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .body(Body::from(payload_text))
        .expect("stream request should build");
    for (name, value) in &headers {
        stream_request.headers_mut().insert(name.clone(), value.clone());
    }

    let stream_response =
        app.oneshot(stream_request).await.expect("stream request should complete");
    let stream_status = stream_response.status().as_u16();
    let stream_body = to_bytes(stream_response.into_body())
        .await
        .expect("stream response body should be readable");
    let stream_json: Value =
        serde_json::from_slice(&stream_body).expect("stream response should be JSON");
    let stream_outcome = outcome_from_stream(stream_status, &stream_json);

    assert_eq!(stdio_outcome, stream_outcome);
    assert_eq!(stdio_outcome.status, 429);
    assert_eq!(stdio_outcome.error_code.as_deref(), Some("limit_token_quota_exceeded"));
}
