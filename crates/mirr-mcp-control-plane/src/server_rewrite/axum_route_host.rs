#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Bytes;
use axum::extract::rejection::BytesRejection;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};

use super::mrt_dispatch_audit_store::{MrtDispatchAuditEventSink, NoopMrtDispatchAuditEventSink};
use super::mrt_dispatch_dual_run_telemetry::{
    DriftCategory, DualRunTelemetry, ParityMetrics, PathExecutionEvent,
};
use super::mrt_dispatch_invocation_executor::{
    execute_mrt_dispatch_invocation, MrtDispatchExecutionConfig, MrtDispatchExecutionError,
    MrtDispatchExecutionResult, MrtRuntimeAdmissionConfig, MrtRuntimeAdmissionError,
    MrtRuntimeAdmissionState,
};
use super::mrt_dispatch_invocation_input::{
    get_body_string, InvocationInputBody, InvocationInputValue,
};
use super::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;
use super::mrt_dispatch_invocation_resolver::resolve_mrt_dispatch_invocation_by_name;
use super::mrt_dispatch_quota_host_boundary::MrtDispatchQuotaHostBoundary;
use super::mrt_dispatch_quota_store::{MrtDispatchQuotaEventSink, NoopMrtDispatchQuotaEventSink};
use super::mrt_dispatch_route_handler::{
    handle_mrt_dispatch_route, MrtDispatchAuditEvent, MrtDispatchPipelineError,
    MrtDispatchRouteResponse, PayloadValidationError,
};
use super::rpc_api_key_extraction::{
    api_key_from_rpc_envelope, parse_api_key_value, RpcEnvelopeApiKeyInput, RpcParamsApiKeyInput,
};
use super::rpc_dispatch_bridge::{stable_error_to_json_string, RpcHandlerMap, RpcStableErrorBody};
use super::rpc_method_normalization::normalize_rpc_method_name;
use super::rpc_role_gate::{require_mrt_dispatch_role, RoleTokenMap};
use super::rpc_stdio_message_dispatch::{
    parse_stdio_rpc_line, StdioRpcDispatchInput, StdioRpcError, StdioRpcResponse, JSON_RPC_VERSION,
};
use super::rpc_stream_envelope_shaping::{shape_stream_rpc_envelope, StreamRpcEnvelopeInput};
use crate::tooling::{discovery_method_by_name, MrtDispatchTool};

pub const DEFAULT_HTTP_BODY_LIMIT_BYTES: usize = 1_048_576;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AxumHostConfig {
    pub max_request_body_bytes: usize,
}

impl Default for AxumHostConfig {
    fn default() -> Self {
        Self { max_request_body_bytes: DEFAULT_HTTP_BODY_LIMIT_BYTES }
    }
}

pub type RpcHandlerFactory = fn() -> RpcHandlerMap<String>;
pub type ResolveInvocationFn =
    fn(&str, &InvocationInputBody) -> Result<MrtDispatchInvocationPlan, String>;
pub type ExecuteInvocationFn = fn(
    &MrtDispatchInvocationPlan,
    &MrtDispatchExecutionConfig,
) -> Result<MrtDispatchExecutionResult, MrtDispatchExecutionError>;
pub type ValidatePayloadFn =
    fn(&str, &InvocationInputBody) -> Result<(), Vec<PayloadValidationError>>;

#[derive(Clone)]
pub struct AxumMcpHostState {
    pub handler_factory: RpcHandlerFactory,
    pub role_tokens: RoleTokenMap,
    pub audit_event_sink: Arc<dyn MrtDispatchAuditEventSink>,
    pub quota_event_sink: Arc<dyn MrtDispatchQuotaEventSink>,
    pub admission_state: Arc<Mutex<MrtRuntimeAdmissionState>>,
    pub admission_config: MrtRuntimeAdmissionConfig,
    pub execution_config: MrtDispatchExecutionConfig,
    pub resolve_invocation: ResolveInvocationFn,
    pub execute_invocation: ExecuteInvocationFn,
    pub validate_payload: ValidatePayloadFn,
}

impl AxumMcpHostState {
    pub fn new(handler_factory: RpcHandlerFactory) -> Self {
        Self::with_role_tokens(handler_factory, RoleTokenMap::default())
    }

    pub fn with_role_tokens(handler_factory: RpcHandlerFactory, role_tokens: RoleTokenMap) -> Self {
        Self {
            handler_factory,
            role_tokens,
            audit_event_sink: Arc::new(NoopMrtDispatchAuditEventSink),
            quota_event_sink: Arc::new(NoopMrtDispatchQuotaEventSink),
            admission_state: Arc::new(Mutex::new(MrtRuntimeAdmissionState::default())),
            admission_config: MrtRuntimeAdmissionConfig::default(),
            execution_config: MrtDispatchExecutionConfig::default(),
            resolve_invocation: resolve_mrt_dispatch_invocation_by_name,
            execute_invocation: execute_mrt_dispatch_invocation,
            validate_payload: validate_canonical_payload,
        }
    }

    pub fn hydrate_token_quota_state_from_sink(
        &self,
        max_rows: usize,
    ) -> Result<(), MrtRuntimeAdmissionError> {
        let boundary = MrtDispatchQuotaHostBoundary::new(
            Arc::clone(&self.admission_state),
            self.admission_config.clone(),
            Arc::clone(&self.quota_event_sink),
        );
        boundary.hydrate_from_sink(max_rows)
    }
}

fn status_from_u16(status: u16) -> StatusCode {
    StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

fn fail_closed_jsonrpc_error(status: StatusCode, id: Option<Value>, message: &str) -> Response {
    (
        status,
        Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": status.as_u16(),
                "message": message,
            }
        })),
    )
        .into_response()
}

fn stable_error_payload(
    error_code: &'static str,
    message: String,
    details: Option<String>,
) -> String {
    stable_error_to_json_string(&RpcStableErrorBody { ok: false, error_code, message, details })
}

fn api_key_from_headers(headers: &HeaderMap) -> String {
    for key in ["x-mcp-api-key", "authorization"] {
        let Some(raw) = headers.get(key) else {
            continue;
        };
        let Ok(text) = raw.to_str() else {
            continue;
        };

        let token = parse_api_key_value(Some(text));
        if !token.is_empty() {
            return token;
        }
    }

    String::new()
}

fn api_key_from_dispatch_input(
    dispatch_input: &StdioRpcDispatchInput,
    header_api_key: String,
) -> Option<String> {
    if !header_api_key.is_empty() {
        return Some(header_api_key);
    }

    let token = api_key_from_rpc_envelope(&RpcEnvelopeApiKeyInput {
        api_key: dispatch_input.api_key.clone(),
        meta: dispatch_input.meta.clone(),
        params: Some(RpcParamsApiKeyInput {
            api_key: dispatch_input.params_api_key.clone(),
            meta: dispatch_input.params_meta.clone(),
            tool_name: dispatch_input.call_tool_name.clone(),
        }),
    });

    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

fn stream_input_from_dispatch(
    dispatch_input: &StdioRpcDispatchInput,
    header_api_key: String,
) -> StreamRpcEnvelopeInput {
    let effective_api_key = if header_api_key.is_empty() {
        dispatch_input.api_key.clone()
    } else {
        Some(header_api_key)
    };

    StreamRpcEnvelopeInput {
        id: dispatch_input.id.clone(),
        method: dispatch_input.method.clone(),
        params: dispatch_input.params.clone(),
        call_tool_name: dispatch_input.call_tool_name.clone(),
        api_key: effective_api_key,
        meta: dispatch_input.meta.clone(),
        params_api_key: dispatch_input.params_api_key.clone(),
        params_meta: dispatch_input.params_meta.clone(),
    }
}

fn invocation_value_matches(expected_type: &str, value: &InvocationInputValue) -> bool {
    match (expected_type, value) {
        ("string", InvocationInputValue::String(_)) => true,
        ("number", InvocationInputValue::Number(_)) => true,
        ("array", InvocationInputValue::StringArray(_)) => true,
        _ => false,
    }
}

fn validate_canonical_payload(
    tool_name: &str,
    body: &InvocationInputBody,
) -> Result<(), Vec<PayloadValidationError>> {
    let Some(metadata) = discovery_method_by_name(tool_name) else {
        return Err(vec![PayloadValidationError {
            path: "/method".to_owned(),
            message: "unknown_method".to_owned(),
        }]);
    };

    let mut errors = Vec::<PayloadValidationError>::new();
    let mut expected_parameters = BTreeMap::<&str, &str>::new();
    for parameter in metadata.parameters {
        expected_parameters.insert(parameter.name, parameter.ty);

        if parameter.required && !body.contains_key(parameter.name) {
            errors.push(PayloadValidationError {
                path: format!("/{}", parameter.name),
                message: "missing_required_parameter".to_owned(),
            });
        }
    }

    for (key, value) in body.iter() {
        let Some(expected_type) = expected_parameters.get(key).copied() else {
            errors.push(PayloadValidationError {
                path: format!("/{}", key),
                message: "unexpected_parameter".to_owned(),
            });
            continue;
        };

        if !invocation_value_matches(expected_type, value) {
            errors.push(PayloadValidationError {
                path: format!("/{}", key),
                message: format!("invalid_parameter_type_{}", expected_type),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
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

fn map_runtime_admission_error(error: MrtRuntimeAdmissionError) -> MrtDispatchPipelineError {
    match error {
        MrtRuntimeAdmissionError::TokenQuotaExceeded => {
            MrtDispatchPipelineError::TokenQuotaExceeded
        }
        MrtRuntimeAdmissionError::ConcurrencyLimitExceeded
        | MrtRuntimeAdmissionError::InvalidRuntimeLimits => {
            MrtDispatchPipelineError::ConcurrencyLimitExceeded
        }
    }
}

fn with_runtime_admission(
    admission_state: &Arc<Mutex<MrtRuntimeAdmissionState>>,
    admission_config: &MrtRuntimeAdmissionConfig,
    quota_event_sink: &Arc<dyn MrtDispatchQuotaEventSink>,
    api_key: Option<&str>,
    operation: &mut dyn FnMut() -> Result<MrtDispatchExecutionResult, MrtDispatchPipelineError>,
) -> Result<MrtDispatchExecutionResult, MrtDispatchPipelineError> {
    let now_ms = now_unix_millis();

    let quota_boundary = MrtDispatchQuotaHostBoundary::new(
        Arc::clone(admission_state),
        admission_config.clone(),
        Arc::clone(quota_event_sink),
    );
    let quota_decision =
        quota_boundary.enforce_for_api_key(api_key, now_ms).map_err(map_runtime_admission_error)?;
    let token = quota_decision.token;

    {
        let mut guard = admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        let current = guard.concurrency.get(&token).copied().unwrap_or(0);
        let next = current.saturating_add(1);
        if next > admission_config.max_concurrent_per_key {
            return Err(MrtDispatchPipelineError::ConcurrencyLimitExceeded);
        }

        guard.concurrency.insert(token.clone(), next);
    }

    let result = operation();

    {
        let mut guard = admission_state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let after = guard.concurrency.get(&token).copied().unwrap_or(1).saturating_sub(1);

        if after == 0 {
            guard.concurrency.remove(&token);
        } else {
            guard.concurrency.insert(token, after);
        }
    }

    result
}

fn known_methods_for_dispatch(handlers: &RpcHandlerMap<String>) -> BTreeSet<String> {
    let mut known = handlers.keys().cloned().collect::<BTreeSet<String>>();
    for tool in MrtDispatchTool::ALL {
        known.insert(tool.as_str().to_owned());
    }
    known
}

fn normalize_host_method_name(
    dispatch_input: &StdioRpcDispatchInput,
    known_methods: &BTreeSet<String>,
) -> String {
    normalize_rpc_method_name(
        dispatch_input.method.as_deref(),
        dispatch_input.call_tool_name.as_deref(),
        known_methods,
    )
}

fn unknown_method_response(
    id: Option<Value>,
    method_name: Option<&str>,
) -> (u16, StdioRpcResponse) {
    let unknown_method = method_name.unwrap_or("unknown");
    let message = format!("MCP unknown method rejected: {}.", unknown_method);
    let stable_payload = stable_error_payload("validation_unknown_method", message, None);

    (
        404,
        StdioRpcResponse {
            jsonrpc: JSON_RPC_VERSION,
            id,
            result: None,
            error: Some(StdioRpcError { code: 404, message: stable_payload }),
        },
    )
}

fn route_response_to_stdio(
    id: Option<Value>,
    route_response: MrtDispatchRouteResponse,
) -> (u16, StdioRpcResponse) {
    match route_response {
        MrtDispatchRouteResponse::Success(success) => (
            200,
            StdioRpcResponse {
                jsonrpc: JSON_RPC_VERSION,
                id,
                result: Some(
                    json!({
                        "tool": success.tool,
                        "exit_code": success.exit_code,
                        "stdout": success.stdout,
                        "stderr": success.stderr,
                        "stdout_truncated": success.stdout_truncated,
                        "stderr_truncated": success.stderr_truncated,
                        "output_limit_bytes": success.output_limit_bytes,
                    })
                    .to_string(),
                ),
                error: None,
            },
        ),
        MrtDispatchRouteResponse::StableError(stable_error) => {
            let payload = stable_error_payload(
                stable_error.error_code,
                stable_error.message.to_owned(),
                stable_error.details,
            );

            (
                stable_error.status_code,
                StdioRpcResponse {
                    jsonrpc: JSON_RPC_VERSION,
                    id,
                    result: None,
                    error: Some(StdioRpcError {
                        code: i32::from(stable_error.status_code),
                        message: payload,
                    }),
                },
            )
        }
    }
}

fn dispatch_canonical_route(
    state: &AxumMcpHostState,
    dispatch_input: &StdioRpcDispatchInput,
    tool_name: &str,
    api_key: Option<String>,
) -> (u16, StdioRpcResponse) {
    if state.execution_config.dual_run_enabled && tool_name == "mrt_kb_query" {
        return dispatch_dual_run_route(state, dispatch_input, tool_name, api_key);
    }

    dispatch_single_route(state, dispatch_input, tool_name, api_key)
}

fn dispatch_single_route(
    state: &AxumMcpHostState,
    dispatch_input: &StdioRpcDispatchInput,
    tool_name: &str,
    api_key: Option<String>,
) -> (u16, StdioRpcResponse) {
    let token_map = state.role_tokens.clone();
    let admission_state = Arc::clone(&state.admission_state);
    let admission_config = state.admission_config.clone();
    let execution_config = state.execution_config.clone();
    let validate_payload = state.validate_payload;
    let resolve_invocation = state.resolve_invocation;
    let execute_invocation = state.execute_invocation;
    let audit_event_sink = Arc::clone(&state.audit_event_sink);
    let quota_event_sink = Arc::clone(&state.quota_event_sink);
    let role_api_key = api_key.clone();
    let runtime_api_key = api_key;

    let route_response = handle_mrt_dispatch_route(
        tool_name,
        &dispatch_input.params,
        execution_config.max_output_bytes,
        validate_payload,
        move |requested_tool| {
            require_mrt_dispatch_role(role_api_key.as_deref(), requested_tool, &token_map)
        },
        |operation| {
            with_runtime_admission(
                &admission_state,
                &admission_config,
                &quota_event_sink,
                runtime_api_key.as_deref(),
                operation,
            )
        },
        |resolved_tool_name, body| resolve_invocation(resolved_tool_name, body),
        |plan| execute_invocation(plan, &execution_config),
        move |event| audit_event_sink.append(&event),
    );

    route_response_to_stdio(dispatch_input.id.clone(), route_response)
}

fn dispatch_dual_run_route(
    state: &AxumMcpHostState,
    dispatch_input: &StdioRpcDispatchInput,
    tool_name: &str,
    api_key: Option<String>,
) -> (u16, StdioRpcResponse) {
    let legacy_dispatch_input = synthesize_legacy_dispatch_input(dispatch_input);
    let legacy_state = state.clone();
    let new_state = state.clone();
    let legacy_api_key = api_key.clone();
    let new_api_key = api_key;
    let legacy_tool_name = String::from("mrt_brain_get");
    let new_tool_name = tool_name.to_owned();

    let legacy_handle = thread::spawn(move || {
        dispatch_single_route(
            &legacy_state,
            &legacy_dispatch_input,
            &legacy_tool_name,
            legacy_api_key,
        )
    });
    let new_dispatch_input = dispatch_input.clone();
    let new_handle = thread::spawn(move || {
        dispatch_single_route(&new_state, &new_dispatch_input, &new_tool_name, new_api_key)
    });

    let legacy_result = legacy_handle.join().ok();
    let new_result = new_handle.join().ok();

    let legacy_result = legacy_result.unwrap_or_else(|| {
        (
            500,
            StdioRpcResponse {
                jsonrpc: JSON_RPC_VERSION,
                id: dispatch_input.id.clone(),
                result: None,
                error: Some(StdioRpcError {
                    code: 500,
                    message: "Legacy dual-run path failed.".to_owned(),
                }),
            },
        )
    });
    let new_result = new_result.unwrap_or_else(|| {
        (
            500,
            StdioRpcResponse {
                jsonrpc: JSON_RPC_VERSION,
                id: dispatch_input.id.clone(),
                result: None,
                error: Some(StdioRpcError {
                    code: 500,
                    message: "New dual-run path failed.".to_owned(),
                }),
            },
        )
    });

    let legacy_response = &legacy_result.1;
    let new_response = &new_result.1;

    let legacy_event = build_path_event("legacy", legacy_response);
    let new_event = build_path_event("new", new_response);
    let parity_metrics = build_parity_metrics(legacy_response, new_response);
    let primary_path_returned = if new_response.result.is_some() {
        String::from("new")
    } else if legacy_response.result.is_some() {
        String::from("legacy")
    } else {
        String::from("fallback")
    };

    let telemetry = DualRunTelemetry::new(
        dispatch_input.id.as_ref().map(ToString::to_string).unwrap_or_default(),
        tool_name.to_owned(),
        extract_query_snippet(&dispatch_input.params),
        legacy_event,
        new_event,
        parity_metrics,
        primary_path_returned,
        current_unix_millis(),
    );

    state.audit_event_sink.append(&MrtDispatchAuditEvent {
        kind: "mrt_dual_run",
        subject: tool_name.to_owned(),
        message: Some(telemetry.summary()),
    });

    let selected = if new_response.result.is_some() { new_result } else { legacy_result };
    let selected_status = selected.0;
    let selected_response = selected.1;

    (selected_status, selected_response)
}

fn synthesize_legacy_dispatch_input(
    dispatch_input: &StdioRpcDispatchInput,
) -> StdioRpcDispatchInput {
    let mut legacy_params = InvocationInputBody::default();
    let key = get_body_string(&dispatch_input.params, "key", "");
    if !key.is_empty() {
        legacy_params.set_string("key", key);
    } else {
        let query = get_body_string(&dispatch_input.params, "query", "");
        if !query.is_empty() {
            legacy_params.set_string("key", query);
        }
    }

    let mut legacy_input = dispatch_input.clone();
    legacy_input.params = legacy_params;
    legacy_input.call_tool_name = Some("mrt_brain_get".to_owned());
    legacy_input
}

fn build_path_event(path_name: &str, response: &StdioRpcResponse) -> PathExecutionEvent {
    let success = response.result.is_some();
    let result_count = response.result.as_deref().map(count_json_items).unwrap_or(0);
    PathExecutionEvent {
        path_name: path_name.to_owned(),
        success,
        latency_ms: 0,
        result_count,
        truncated: false,
        error: response.error.as_ref().map(|error| error.message.clone()),
    }
}

fn build_parity_metrics(
    legacy_response: &StdioRpcResponse,
    new_response: &StdioRpcResponse,
) -> ParityMetrics {
    let legacy_count = legacy_response.result.as_deref().map(count_json_items).unwrap_or(0);
    let new_count = new_response.result.as_deref().map(count_json_items).unwrap_or(0);
    let count_match = legacy_count == new_count;
    let overlap_percent = if legacy_count == 0 && new_count == 0 {
        100
    } else {
        let smaller = legacy_count.min(new_count) as u32;
        let larger = legacy_count.max(new_count) as u32;
        if larger == 0 {
            0
        } else {
            (smaller.saturating_mul(100)) / larger
        }
    };

    ParityMetrics {
        paths_match: legacy_response.result == new_response.result,
        result_count_match: count_match,
        result_count_diff: new_count as i32 - legacy_count as i32,
        truncation_match: true,
        drift_category: if legacy_response.result == new_response.result {
            DriftCategory::NoDrift
        } else if count_match {
            DriftCategory::MinorReordering
        } else {
            DriftCategory::ResultCountMismatch
        },
        top_k_reordered: legacy_response.result != new_response.result && count_match,
        result_overlap_percent: overlap_percent,
    }
}

fn count_json_items(value: &str) -> usize {
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(serde_json::Value::Array(items)) => items.len(),
        Ok(serde_json::Value::Object(map)) => map.len(),
        Ok(_) => 1,
        Err(_) => 1,
    }
}

fn extract_query_snippet(body: &InvocationInputBody) -> String {
    let query = get_body_string(body, "query", "");
    if !query.is_empty() {
        return query.chars().take(128).collect();
    }

    let key = get_body_string(body, "key", "");
    key.chars().take(128).collect()
}

fn current_unix_millis() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => u64::try_from(duration.as_millis()).unwrap_or(u64::MAX),
        Err(_) => 0,
    }
}

pub fn dispatch_host_rpc_message(
    state: &AxumMcpHostState,
    headers: &HeaderMap,
    dispatch_input: StdioRpcDispatchInput,
) -> (u16, StdioRpcResponse) {
    let handlers = (state.handler_factory)();
    let known_methods = known_methods_for_dispatch(&handlers);
    let normalized_method_name = normalize_host_method_name(&dispatch_input, &known_methods);

    if normalized_method_name.is_empty() {
        return unknown_method_response(
            dispatch_input.id.clone(),
            dispatch_input.method.as_deref(),
        );
    }

    if MrtDispatchTool::from_str(&normalized_method_name).is_some() {
        let header_api_key = api_key_from_headers(headers);
        let api_key = api_key_from_dispatch_input(&dispatch_input, header_api_key);
        return dispatch_canonical_route(state, &dispatch_input, &normalized_method_name, api_key);
    }

    let stream_input = stream_input_from_dispatch(&dispatch_input, api_key_from_headers(headers));
    let shaped = shape_stream_rpc_envelope(&stream_input, &handlers);

    if let Some(result) = shaped.result {
        return (
            200,
            StdioRpcResponse {
                jsonrpc: shaped.jsonrpc,
                id: shaped.id,
                result: Some(result),
                error: None,
            },
        );
    }

    let (error_code, error_message) = if let Some(error) = shaped.error {
        (error.code, error.message)
    } else {
        (
            i32::from(shaped.status_code),
            stable_error_payload(
                "transport_stream_dispatch_failed",
                "Unhandled stream dispatch failure.".to_owned(),
                None,
            ),
        )
    };

    (
        shaped.status_code,
        StdioRpcResponse {
            jsonrpc: shaped.jsonrpc,
            id: shaped.id,
            result: None,
            error: Some(StdioRpcError { code: error_code, message: error_message }),
        },
    )
}

pub fn dispatch_host_stdio_line(
    state: &AxumMcpHostState,
    line: &str,
) -> Option<(u16, StdioRpcResponse)> {
    let dispatch_input = parse_stdio_rpc_line(line)?;
    let headers = HeaderMap::new();
    Some(dispatch_host_rpc_message(state, &headers, dispatch_input))
}

pub async fn handle_stream_rpc_request(
    State(state): State<AxumMcpHostState>,
    headers: HeaderMap,
    raw_body: Result<Bytes, BytesRejection>,
) -> Response {
    let body_bytes = match raw_body {
        Ok(value) => value,
        Err(_) => {
            return fail_closed_jsonrpc_error(
                StatusCode::PAYLOAD_TOO_LARGE,
                None,
                "Request body exceeded configured limit.",
            )
        }
    };

    let body_text = match std::str::from_utf8(&body_bytes) {
        Ok(value) => value,
        Err(_) => {
            return fail_closed_jsonrpc_error(
                StatusCode::BAD_REQUEST,
                None,
                "Invalid request encoding.",
            )
        }
    };

    let Some(dispatch_input) = parse_stdio_rpc_line(body_text) else {
        return fail_closed_jsonrpc_error(
            StatusCode::BAD_REQUEST,
            None,
            "Invalid JSON-RPC request body.",
        );
    };

    let (status_code, host_response) = dispatch_host_rpc_message(&state, &headers, dispatch_input);

    if let Some(result) = host_response.result {
        return (
            StatusCode::OK,
            Json(json!({
                "jsonrpc": host_response.jsonrpc,
                "id": host_response.id,
                "result": result,
            })),
        )
            .into_response();
    }

    let (error_code, error_message) = if let Some(error) = host_response.error {
        (error.code, error.message)
    } else {
        (i32::from(status_code), "Unhandled stream dispatch failure.".to_owned())
    };

    (
        status_from_u16(status_code),
        Json(json!({
            "jsonrpc": host_response.jsonrpc,
            "id": host_response.id,
            "error": {
                "code": error_code,
                "message": error_message,
            }
        })),
    )
        .into_response()
}

pub fn build_axum_mcp_router(state: AxumMcpHostState, config: AxumHostConfig) -> Router {
    Router::new()
        .route("/mcp", post(handle_stream_rpc_request))
        .route("/mcp_stream", post(handle_stream_rpc_request))
        .layer(DefaultBodyLimit::max(config.max_request_body_bytes))
        .with_state(state)
}

pub async fn run_axum_mcp_host(
    bind_addr: SocketAddr,
    state: AxumMcpHostState,
    config: AxumHostConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_axum_mcp_router(state, config);
    axum::Server::bind(&bind_addr)
        .serve(app.into_make_service())
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
}
