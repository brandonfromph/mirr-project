#![forbid(unsafe_code)]
#![deny(warnings)]

use std::collections::BTreeMap;
use std::sync::Mutex;

use mirr_mcp_control_plane::policy::Role;
use mirr_mcp_control_plane::server_rewrite::axum_route_host::{
    dispatch_host_rpc_message, AxumMcpHostState,
};
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_audit_store::MrtDispatchAuditEventSink;
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_executor::{
    MrtDispatchExecutionConfig, MrtDispatchExecutionError, MrtDispatchExecutionResult,
};
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_route_handler::MrtDispatchAuditEvent;
use mirr_mcp_control_plane::server_rewrite::rpc_dispatch_bridge::RpcHandlerMap;
use mirr_mcp_control_plane::server_rewrite::rpc_role_gate::VerifiedPrincipal;
use mirr_mcp_control_plane::server_rewrite::rpc_stdio_message_dispatch::StdioRpcDispatchInput;
use serde_json::json;
use std::sync::Arc;

#[derive(Default)]
struct CapturingAuditSink {
    events: Mutex<Vec<MrtDispatchAuditEvent>>,
}

impl CapturingAuditSink {
    fn events(&self) -> Vec<MrtDispatchAuditEvent> {
        self.events.lock().expect("audit lock").clone()
    }
}

impl MrtDispatchAuditEventSink for CapturingAuditSink {
    fn append(&self, event: &MrtDispatchAuditEvent) {
        self.events.lock().expect("audit lock").push(event.clone());
    }
}

fn mock_execute(
    plan: &MrtDispatchInvocationPlan,
    _config: &MrtDispatchExecutionConfig,
) -> Result<MrtDispatchExecutionResult, MrtDispatchExecutionError> {
    let plan_text = plan.args.join(" ");
    let stdout = if plan_text.contains("mirr-brain") {
        "legacy-path".to_string()
    } else {
        "new-path".to_string()
    };

    Ok(MrtDispatchExecutionResult { stdout, stderr: String::new(), exit_code: 0 })
}

fn mock_execute_fallback(
    plan: &MrtDispatchInvocationPlan,
    _config: &MrtDispatchExecutionConfig,
) -> Result<MrtDispatchExecutionResult, MrtDispatchExecutionError> {
    let plan_text = plan.args.join(" ");
    if plan_text.contains("mirrc-kb") {
        return Err(MrtDispatchExecutionError::NonZeroExit {
            message: "new path failed".to_string(),
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 1,
        });
    }

    Ok(MrtDispatchExecutionResult {
        stdout: "legacy-path".to_string(),
        stderr: String::new(),
        exit_code: 0,
    })
}

fn handler_factory() -> RpcHandlerMap<String> {
    RpcHandlerMap::default()
}

fn dual_run_state() -> (AxumMcpHostState, Arc<CapturingAuditSink>) {
    let mut role_tokens = BTreeMap::new();
    role_tokens.insert(
        "admin-token".to_string(),
        VerifiedPrincipal { id: "admin".to_string(), role: Role::Admin },
    );

    let mut state = AxumMcpHostState::with_role_tokens(handler_factory, role_tokens);
    state.execution_config.dual_run_enabled = true;
    state.execute_invocation = mock_execute;
    let capturing_sink = Arc::new(CapturingAuditSink::default());
    state.audit_event_sink = capturing_sink.clone();
    (state, capturing_sink)
}

#[test]
fn mrt_kb_query_dual_run_prefers_new_path_and_logs_telemetry() {
    let (state, capturing_sink) = dual_run_state();
    let headers = axum::http::HeaderMap::from_iter([(
        axum::http::header::HeaderName::from_static("x-mcp-api-key"),
        axum::http::HeaderValue::from_static("admin-token"),
    )]);

    let dispatch_input = StdioRpcDispatchInput {
        id: Some(json!(1)),
        method: Some("mrt_kb_query".to_string()),
        params: {
            let mut body =
                mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_input::InvocationInputBody::default(
                );
            body.set_string("query", "alpha signal");
            body.set_string("mode", "hybrid");
            body.set_number("limit", 4.0);
            body
        },
        call_tool_name: Some("mrt_kb_query".to_string()),
        api_key: None,
        meta: BTreeMap::new(),
        params_api_key: None,
        params_meta: BTreeMap::new(),
    };

    let (_status, response) = dispatch_host_rpc_message(&state, &headers, dispatch_input);
    let result = response.result.expect("dual-run should return a result");
    assert!(result.contains("new-path"), "expected new path result, got: {result}");

    let events = capturing_sink.events();
    assert!(events.iter().any(|event| event.kind == "mrt_dual_run"));
    assert!(events.iter().any(|event| event
        .message
        .as_deref()
        .unwrap_or_default()
        .contains("mrt_kb_query")));
}

#[test]
fn mrt_kb_query_dual_run_falls_back_to_legacy_when_new_path_errors() {
    let (mut state, capturing_sink) = dual_run_state();
    state.execute_invocation = mock_execute_fallback;

    let headers = axum::http::HeaderMap::from_iter([(
        axum::http::header::HeaderName::from_static("x-mcp-api-key"),
        axum::http::HeaderValue::from_static("admin-token"),
    )]);

    let dispatch_input = StdioRpcDispatchInput {
        id: Some(json!(2)),
        method: Some("mrt_kb_query".to_string()),
        params: {
            let mut body =
                mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_input::InvocationInputBody::default(
                );
            body.set_string("query", "beta signal");
            body.set_string("mode", "hybrid");
            body.set_number("limit", 4.0);
            body
        },
        call_tool_name: Some("mrt_kb_query".to_string()),
        api_key: None,
        meta: BTreeMap::new(),
        params_api_key: None,
        params_meta: BTreeMap::new(),
    };

    let (_status, response) = dispatch_host_rpc_message(&state, &headers, dispatch_input);
    let result = response.result.expect("dual-run should fall back to legacy result");
    assert!(result.contains("legacy-path"), "expected legacy fallback result, got: {result}");

    let events = capturing_sink.events();
    assert!(events.iter().any(|event| event.kind == "mrt_dual_run"));
    assert!(events.iter().any(|event| event
        .message
        .as_deref()
        .unwrap_or_default()
        .contains("new=fail")));
}
