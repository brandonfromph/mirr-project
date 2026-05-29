use std::collections::BTreeMap;

use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_input::InvocationInputBody;
use mirr_mcp_control_plane::server_rewrite::rpc_dispatch_bridge::{
    RpcDispatchBody, RpcDispatchRequestShim, RpcDispatchResult, RpcHandlerMap, RpcHandlerResponse,
};
use mirr_mcp_control_plane::server_rewrite::rpc_stream_envelope_shaping::{
    shape_stream_rpc_envelope, shape_stream_rpc_envelope_from_dispatch, StreamRpcEnvelopeInput,
    STREAM_JSON_RPC_VERSION,
};
use serde_json::Value;

#[test]
fn stream_success_wraps_dispatch_payload_into_jsonrpc_result() {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    handlers.insert(
        "mrt_audit".to_owned(),
        Box::new(|_req: RpcDispatchRequestShim| RpcHandlerResponse {
            status: 200,
            body: "ok_payload".to_owned(),
        }),
    );

    let input = StreamRpcEnvelopeInput {
        id: Some(Value::String("99".to_owned())),
        method: Some("mrt_audit".to_owned()),
        params: InvocationInputBody::default(),
        ..StreamRpcEnvelopeInput::default()
    };

    let response = shape_stream_rpc_envelope(&input, &handlers);
    assert_eq!(response.status_code, 200);
    assert_eq!(response.jsonrpc, STREAM_JSON_RPC_VERSION);
    assert_eq!(response.id, Some(Value::String("99".to_owned())));
    assert_eq!(response.result, Some("ok_payload".to_owned()));
    assert_eq!(response.error, None);
}

#[test]
fn stream_non_success_status_maps_to_jsonrpc_error() {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    handlers.insert(
        "mrt_general_ci".to_owned(),
        Box::new(|_req: RpcDispatchRequestShim| RpcHandlerResponse {
            status: 429,
            body: "limit_token_quota_exceeded".to_owned(),
        }),
    );

    let input = StreamRpcEnvelopeInput {
        id: Some(Value::String("7".to_owned())),
        method: Some("mrt_general_ci".to_owned()),
        params: InvocationInputBody::default(),
        ..StreamRpcEnvelopeInput::default()
    };

    let response = shape_stream_rpc_envelope(&input, &handlers);
    assert_eq!(response.status_code, 429);
    assert_eq!(response.result, None);
    let err = response.error.expect("non-success status must return jsonrpc error");
    assert_eq!(err.code, 429);
    assert_eq!(err.message, "limit_token_quota_exceeded");
}

#[test]
fn stream_unknown_method_uses_dispatch_bridge_stable_error_body() {
    let handlers: RpcHandlerMap<String> = BTreeMap::new();
    let input = StreamRpcEnvelopeInput {
        id: Some(Value::String("41".to_owned())),
        method: Some("unknown_rpc".to_owned()),
        params: InvocationInputBody::default(),
        ..StreamRpcEnvelopeInput::default()
    };

    let response = shape_stream_rpc_envelope(&input, &handlers);
    assert_eq!(response.status_code, 404);
    assert_eq!(response.result, None);
    let err = response.error.expect("unknown method must map to jsonrpc error envelope");
    assert_eq!(err.code, 404);
    assert!(err.message.contains("\"error_code\":\"validation_unknown_method\""));
}

#[test]
fn stream_dispatch_failure_maps_to_internal_error_envelope() {
    let response = shape_stream_rpc_envelope_from_dispatch(
        Some(Value::String("501".to_owned())),
        Err("stream_dispatch_failure".to_owned()),
    );

    assert_eq!(response.status_code, 500);
    assert_eq!(response.jsonrpc, STREAM_JSON_RPC_VERSION);
    assert_eq!(response.id, Some(Value::String("501".to_owned())));
    assert_eq!(response.result, None);
    let err = response.error.expect("dispatch failure must map to error");
    assert_eq!(err.code, 500);
    assert_eq!(err.message, "stream_dispatch_failure");
}

#[test]
fn stream_dispatch_wrapper_accepts_precomputed_dispatch_result() {
    let response = shape_stream_rpc_envelope_from_dispatch(
        Some(Value::String("1".to_owned())),
        Ok(RpcDispatchResult { status: 403, body: RpcDispatchBody::Payload("denied".to_owned()) }),
    );

    assert_eq!(response.status_code, 403);
    assert_eq!(response.result, None);
    let err = response.error.expect("status 403 should be represented as error");
    assert_eq!(err.code, 403);
    assert_eq!(err.message, "denied");
}
