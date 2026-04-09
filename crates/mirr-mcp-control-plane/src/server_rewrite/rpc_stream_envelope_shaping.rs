#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use serde_json::Value;

use super::mrt_dispatch_invocation_input::InvocationInputBody;
use super::rpc_dispatch_bridge::{
    dispatch_body_to_string, dispatch_rpc_to_handler, RpcDispatchMessage, RpcDispatchResult,
    RpcHandlerMap,
};

pub const STREAM_JSON_RPC_VERSION: &str = "2.0";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StreamRpcEnvelopeInput {
    pub id: Option<Value>,
    pub method: Option<String>,
    pub params: InvocationInputBody,
    pub call_tool_name: Option<String>,
    pub api_key: Option<String>,
    pub meta: BTreeMap<String, String>,
    pub params_api_key: Option<String>,
    pub params_meta: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamRpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamRpcEnvelopeResponse {
    pub status_code: u16,
    pub jsonrpc: &'static str,
    pub id: Option<Value>,
    pub result: Option<String>,
    pub error: Option<StreamRpcError>,
}

pub fn shape_stream_rpc_envelope(
    msg: &StreamRpcEnvelopeInput,
    handlers: &RpcHandlerMap<String>,
) -> StreamRpcEnvelopeResponse {
    let dispatch_result = dispatch_rpc_to_handler(
        &RpcDispatchMessage {
            method: msg.method.clone(),
            params: msg.params.clone(),
            call_tool_name: msg.call_tool_name.clone(),
            api_key: msg.api_key.clone(),
            meta: msg.meta.clone(),
            params_api_key: msg.params_api_key.clone(),
            params_meta: msg.params_meta.clone(),
        },
        handlers,
    );

    shape_stream_rpc_envelope_from_dispatch(msg.id.clone(), Ok(dispatch_result))
}

pub fn shape_stream_rpc_envelope_from_dispatch(
    id: Option<Value>,
    dispatch_result: Result<RpcDispatchResult<String>, String>,
) -> StreamRpcEnvelopeResponse {
    match dispatch_result {
        Ok(result) => {
            if (200..300).contains(&result.status) {
                return StreamRpcEnvelopeResponse {
                    status_code: 200,
                    jsonrpc: STREAM_JSON_RPC_VERSION,
                    id,
                    result: Some(dispatch_body_to_string(result.body)),
                    error: None,
                };
            }

            StreamRpcEnvelopeResponse {
                status_code: result.status,
                jsonrpc: STREAM_JSON_RPC_VERSION,
                id,
                result: None,
                error: Some(StreamRpcError {
                    code: i32::from(result.status),
                    message: dispatch_body_to_string(result.body),
                }),
            }
        }
        Err(err_message) => StreamRpcEnvelopeResponse {
            status_code: 500,
            jsonrpc: STREAM_JSON_RPC_VERSION,
            id,
            result: None,
            error: Some(StreamRpcError { code: 500, message: err_message }),
        },
    }
}
