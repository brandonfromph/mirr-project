#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use super::mrt_dispatch_invocation_input::InvocationInputBody;
use super::rpc_dispatch_bridge::{
    dispatch_body_to_string, dispatch_rpc_to_handler, RpcDispatchMessage, RpcHandlerMap,
};
use super::rpc_method_normalization::normalize_rpc_method_name;

pub const JSON_RPC_VERSION: &str = "2.0";
pub const STDLIB_FORCE_RESET_ERROR_CODE: i32 = -32000;
pub const STDLIB_FORCE_RESET_ERROR_MESSAGE: &str = "ECONNRESET simulated by MCP_TEST_FORCE_RESET";
pub const MAX_STDIO_BUFFER_BYTES: usize = 1_048_576;
pub const MAX_STDIO_LINE_BYTES: usize = 131_072;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StdioRpcDispatchInput {
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
pub struct StdioRpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StdioRpcResponse {
    pub jsonrpc: &'static str,
    pub id: Option<Value>,
    pub result: Option<String>,
    pub error: Option<StdioRpcError>,
}

fn json_string_field(object: &Map<String, Value>, key: &str) -> Option<String> {
    object.get(key).and_then(Value::as_str).map(ToOwned::to_owned)
}

fn json_string_map(value: Option<&Value>) -> BTreeMap<String, String> {
    let mut out = BTreeMap::<String, String>::new();
    let Some(Value::Object(map)) = value else {
        return out;
    };

    for (key, value) in map {
        if let Some(as_string) = value.as_str() {
            out.insert(key.clone(), as_string.to_owned());
        }
    }

    out
}

fn invocation_body_from_json_params_filtered(
    params: &Map<String, Value>,
    skip_keys: &[&str],
) -> InvocationInputBody {
    let mut body = InvocationInputBody::default();

    for (key, value) in params {
        if skip_keys.contains(&key.as_str()) {
            continue;
        }

        match value {
            Value::String(text) => body.set_string(key, text.clone()),
            Value::Number(number) => {
                if let Some(as_f64) = number.as_f64() {
                    if as_f64.is_finite() {
                        body.set_number(key, as_f64);
                    }
                }
            }
            Value::Array(values) => {
                let mut strings = Vec::<String>::new();
                let mut all_strings = true;

                for entry in values {
                    let Some(text) = entry.as_str() else {
                        all_strings = false;
                        break;
                    };
                    strings.push(text.to_owned());
                }

                if all_strings {
                    body.set_string_array(key, strings);
                }
            }
            _ => {}
        }
    }

    body
}

fn invocation_body_from_json_params(params: &Map<String, Value>) -> InvocationInputBody {
    invocation_body_from_json_params_filtered(params, &[])
}

fn parse_jsonrpc_id(value: Option<&Value>) -> Option<Value> {
    let raw = value?;

    match raw {
        Value::String(_) | Value::Number(_) => Some(raw.clone()),
        _ => None,
    }
}

pub fn parse_stdio_rpc_line(line: &str) -> Option<StdioRpcDispatchInput> {
    if line.len() > MAX_STDIO_LINE_BYTES {
        return None;
    }

    let parsed: Value = serde_json::from_str(line).ok()?;
    let object = parsed.as_object()?;

    let method = json_string_field(object, "method");
    let mut params = InvocationInputBody::default();
    let mut call_tool_name = None::<String>;
    let mut params_api_key = None::<String>;
    let mut params_meta = BTreeMap::<String, String>::new();

    if let Some(Value::Object(params_map)) = object.get("params") {
        call_tool_name = json_string_field(params_map, "name");
        params_api_key = json_string_field(params_map, "apiKey");
        params_meta = json_string_map(params_map.get("meta"));

        if matches!(method.as_deref(), Some("tools/call" | "CallTool" | "callTool")) {
            if let Some(Value::Object(arguments)) = params_map.get("arguments") {
                params = invocation_body_from_json_params(arguments);
            } else {
                params = invocation_body_from_json_params_filtered(
                    params_map,
                    &["name", "arguments", "apiKey", "meta"],
                );
            }
        } else {
            params = invocation_body_from_json_params(params_map);
        }
    }

    Some(StdioRpcDispatchInput {
        id: parse_jsonrpc_id(object.get("id")),
        method,
        params,
        call_tool_name,
        api_key: json_string_field(object, "apiKey"),
        meta: json_string_map(object.get("meta")),
        params_api_key,
        params_meta,
    })
}

pub fn consume_stdio_input_chunk(
    current_buffer: &str,
    chunk: &str,
) -> (String, Vec<StdioRpcDispatchInput>) {
    let mut buffer = String::with_capacity(current_buffer.len() + chunk.len());
    buffer.push_str(current_buffer);
    buffer.push_str(chunk);

    if buffer.len() > MAX_STDIO_BUFFER_BYTES {
        return (String::new(), Vec::new());
    }

    let mut messages = Vec::<StdioRpcDispatchInput>::new();
    while let Some(idx) = buffer.find('\n') {
        let line = buffer[..idx].trim().to_owned();
        buffer = buffer[idx + 1..].to_owned();

        if line.is_empty() {
            continue;
        }

        if let Some(message) = parse_stdio_rpc_line(&line) {
            messages.push(message);
        }
    }

    if buffer.len() > MAX_STDIO_BUFFER_BYTES {
        return (String::new(), Vec::new());
    }

    (buffer, messages)
}

pub fn format_stdio_rpc_output_line(response: &StdioRpcResponse) -> Option<String> {
    let id = response.id.as_ref()?;

    let jsonrpc = if response.jsonrpc.is_empty() { JSON_RPC_VERSION } else { response.jsonrpc };

    let mut out = Map::<String, Value>::new();
    out.insert("jsonrpc".to_owned(), Value::String(jsonrpc.to_owned()));
    out.insert("id".to_owned(), id.clone());

    if let Some(error) = &response.error {
        let mut err = Map::<String, Value>::new();
        err.insert(
            "code".to_owned(),
            Value::Number(serde_json::Number::from(i64::from(error.code))),
        );
        err.insert("message".to_owned(), Value::String(error.message.clone()));
        out.insert("error".to_owned(), Value::Object(err));
    } else if let Some(result) = &response.result {
        let result_value = match serde_json::from_str::<Value>(result) {
            Ok(value) => value,
            Err(_) => Value::String(result.clone()),
        };
        out.insert("result".to_owned(), result_value);
    }

    Some(format!("{}\n", Value::Object(out)))
}

fn unknown_method_name(raw_method: Option<&str>, normalized_method: &str) -> String {
    if let Some(value) = raw_method {
        return value.to_owned();
    }

    if !normalized_method.is_empty() {
        return normalized_method.to_owned();
    }

    "unknown".to_owned()
}

pub fn dispatch_stdio_message(
    msg: &StdioRpcDispatchInput,
    handlers: &RpcHandlerMap<String>,
    force_reset: bool,
) -> StdioRpcResponse {
    if force_reset {
        return StdioRpcResponse {
            jsonrpc: JSON_RPC_VERSION,
            id: msg.id.clone(),
            result: None,
            error: Some(StdioRpcError {
                code: STDLIB_FORCE_RESET_ERROR_CODE,
                message: STDLIB_FORCE_RESET_ERROR_MESSAGE.to_owned(),
            }),
        };
    }

    let known_methods: BTreeSet<String> = handlers.keys().cloned().collect();
    let normalized_method = normalize_rpc_method_name(
        msg.method.as_deref(),
        msg.call_tool_name.as_deref(),
        &known_methods,
    );

    if normalized_method.is_empty() || !handlers.contains_key(&normalized_method) {
        let unknown_name = unknown_method_name(msg.method.as_deref(), &normalized_method);
        let unknown_message = format!("MCP unknown method rejected: {}.", unknown_name);
        return StdioRpcResponse {
            jsonrpc: JSON_RPC_VERSION,
            id: msg.id.clone(),
            result: None,
            error: Some(StdioRpcError { code: 404, message: unknown_message }),
        };
    }

    let dispatch_result = dispatch_rpc_to_handler(
        &RpcDispatchMessage {
            method: Some(normalized_method),
            params: msg.params.clone(),
            call_tool_name: msg.call_tool_name.clone(),
            api_key: msg.api_key.clone(),
            meta: msg.meta.clone(),
            params_api_key: msg.params_api_key.clone(),
            params_meta: msg.params_meta.clone(),
        },
        handlers,
    );

    if (200..300).contains(&dispatch_result.status) {
        return StdioRpcResponse {
            jsonrpc: JSON_RPC_VERSION,
            id: msg.id.clone(),
            result: Some(dispatch_body_to_string(dispatch_result.body)),
            error: None,
        };
    }

    StdioRpcResponse {
        jsonrpc: JSON_RPC_VERSION,
        id: msg.id.clone(),
        result: None,
        error: Some(StdioRpcError {
            code: i32::from(dispatch_result.status),
            message: dispatch_body_to_string(dispatch_result.body),
        }),
    }
}
