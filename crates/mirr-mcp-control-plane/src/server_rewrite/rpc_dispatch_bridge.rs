#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use super::mrt_dispatch_invocation_input::InvocationInputBody;
use super::rpc_api_key_extraction::{
    api_key_from_rpc_envelope, RpcEnvelopeApiKeyInput, RpcParamsApiKeyInput,
};
use super::rpc_method_normalization::normalize_rpc_method_name;

pub const MAX_REGISTERED_HANDLERS: usize = 512;
pub const MAX_RPC_METHOD_NAME_BYTES: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcStableErrorBody {
    pub ok: bool,
    pub error_code: &'static str,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RpcDispatchBody<T> {
    StableError(RpcStableErrorBody),
    Payload(T),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcDispatchResult<T> {
    pub status: u16,
    pub body: RpcDispatchBody<T>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RpcDispatchMessage {
    pub method: Option<String>,
    pub params: InvocationInputBody,
    pub call_tool_name: Option<String>,
    pub api_key: Option<String>,
    pub meta: BTreeMap<String, String>,
    pub params_api_key: Option<String>,
    pub params_meta: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RpcDispatchRequestShim {
    pub headers: BTreeMap<String, String>,
    pub body: InvocationInputBody,
    pub query: InvocationInputBody,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcHandlerResponse<T> {
    pub status: u16,
    pub body: T,
}

pub trait RpcDispatchHandler<T> {
    fn call(&self, req: RpcDispatchRequestShim) -> RpcHandlerResponse<T>;
}

impl<T, F> RpcDispatchHandler<T> for F
where
    F: Fn(RpcDispatchRequestShim) -> RpcHandlerResponse<T>,
{
    fn call(&self, req: RpcDispatchRequestShim) -> RpcHandlerResponse<T> {
        self(req)
    }
}

pub type RpcHandlerMap<T> = BTreeMap<String, Box<dyn RpcDispatchHandler<T>>>;

fn escape_json(raw: &str) -> String {
    raw.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub fn stable_error_to_json_string(error: &RpcStableErrorBody) -> String {
    let details = match &error.details {
        Some(value) => format!("\"{}\"", escape_json(value)),
        None => "null".to_owned(),
    };

    format!(
        "{{\"ok\":false,\"error_code\":\"{}\",\"message\":\"{}\",\"details\":{}}}",
        escape_json(error.error_code),
        escape_json(&error.message),
        details
    )
}

pub fn dispatch_body_to_string(body: RpcDispatchBody<String>) -> String {
    match body {
        RpcDispatchBody::Payload(value) => value,
        RpcDispatchBody::StableError(error) => stable_error_to_json_string(&error),
    }
}

fn unknown_method_response<T>(method_name: Option<&str>) -> RpcDispatchResult<T> {
    let unknown = method_name.unwrap_or("unknown");
    RpcDispatchResult {
        status: 404,
        body: RpcDispatchBody::StableError(RpcStableErrorBody {
            ok: false,
            error_code: "validation_unknown_method",
            message: format!("MCP unknown method rejected: {}.", unknown),
            details: None,
        }),
    }
}

pub fn dispatch_rpc_to_handler<T>(
    msg: &RpcDispatchMessage,
    handlers: &RpcHandlerMap<T>,
) -> RpcDispatchResult<T> {
    if handlers.len() > MAX_REGISTERED_HANDLERS {
        return unknown_method_response(msg.method.as_deref());
    }

    if let Some(raw_method) = msg.method.as_deref() {
        if raw_method.len() > MAX_RPC_METHOD_NAME_BYTES {
            return unknown_method_response(Some(raw_method));
        }
    }

    let known_methods: BTreeSet<String> = handlers.keys().cloned().collect();
    let method_name = normalize_rpc_method_name(
        msg.method.as_deref(),
        msg.call_tool_name.as_deref(),
        &known_methods,
    );

    if method_name.is_empty() || method_name.len() > MAX_RPC_METHOD_NAME_BYTES {
        return unknown_method_response(msg.method.as_deref());
    }

    let Some(handler) = handlers.get(&method_name) else {
        return unknown_method_response(msg.method.as_deref());
    };

    let mut req = RpcDispatchRequestShim {
        headers: BTreeMap::new(),
        body: msg.params.clone(),
        query: msg.params.clone(),
    };

    let envelope = RpcEnvelopeApiKeyInput {
        api_key: msg.api_key.clone(),
        meta: msg.meta.clone(),
        params: Some(RpcParamsApiKeyInput {
            api_key: msg.params_api_key.clone(),
            meta: msg.params_meta.clone(),
            tool_name: msg.call_tool_name.clone(),
        }),
    };
    let rpc_token = api_key_from_rpc_envelope(&envelope);

    if !rpc_token.is_empty() {
        req.headers.insert("x-mcp-api-key".to_owned(), rpc_token.clone());
        req.headers.insert("authorization".to_owned(), format!("Bearer {}", rpc_token));
    }

    let response = handler.call(req);

    RpcDispatchResult { status: response.status, body: RpcDispatchBody::Payload(response.body) }
}
