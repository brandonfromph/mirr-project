use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use mirror::server_rewrite::mrt_dispatch_invocation_input::InvocationInputBody;
use mirror::server_rewrite::rpc_dispatch_bridge::{
    RpcDispatchRequestShim, RpcHandlerMap, RpcHandlerResponse,
};
use mirror::server_rewrite::rpc_stdio_message_dispatch::{
    consume_stdio_input_chunk, dispatch_stdio_message, format_stdio_rpc_output_line,
    parse_stdio_rpc_line, StdioRpcDispatchInput, StdioRpcError, StdioRpcResponse, JSON_RPC_VERSION,
    MAX_STDIO_BUFFER_BYTES, MAX_STDIO_LINE_BYTES, STDLIB_FORCE_RESET_ERROR_CODE,
    STDLIB_FORCE_RESET_ERROR_MESSAGE,
};
use serde_json::Value;

#[test]
fn stdio_force_reset_returns_expected_protocol_error() {
    let handlers: RpcHandlerMap<String> = BTreeMap::new();
    let input = StdioRpcDispatchInput {
        id: Some(Value::String("17".to_owned())),
        method: Some("mrt_audit".to_owned()),
        params: InvocationInputBody::default(),
        ..StdioRpcDispatchInput::default()
    };

    let response = dispatch_stdio_message(&input, &handlers, true);
    assert_eq!(response.jsonrpc, JSON_RPC_VERSION);
    assert_eq!(response.id, Some(Value::String("17".to_owned())));
    assert_eq!(response.result, None);
    let err = response.error.expect("forced reset must return an error envelope");
    assert_eq!(err.code, STDLIB_FORCE_RESET_ERROR_CODE);
    assert_eq!(err.message, STDLIB_FORCE_RESET_ERROR_MESSAGE);
}

#[test]
fn stdio_unknown_method_is_fail_closed() {
    let handlers: RpcHandlerMap<String> = BTreeMap::new();
    let input = StdioRpcDispatchInput {
        id: Some(Value::String("77".to_owned())),
        method: Some("mrt_unknown".to_owned()),
        params: InvocationInputBody::default(),
        ..StdioRpcDispatchInput::default()
    };

    let response = dispatch_stdio_message(&input, &handlers, false);
    assert_eq!(response.jsonrpc, JSON_RPC_VERSION);
    assert_eq!(response.result, None);
    let err = response.error.expect("unknown method must return an error envelope");
    assert_eq!(err.code, 404);
    assert_eq!(err.message, "MCP unknown method rejected: mrt_unknown.");
}

#[test]
fn stdio_dispatch_normalizes_method_and_injects_api_key_headers() {
    let observed_headers = Rc::new(RefCell::new(BTreeMap::<String, String>::new()));

    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    {
        let observed_headers_ref = observed_headers.clone();
        handlers.insert(
            "mcp_schema".to_owned(),
            Box::new(move |req: RpcDispatchRequestShim| {
                *observed_headers_ref.borrow_mut() = req.headers.clone();
                RpcHandlerResponse { status: 200, body: "schema_result".to_owned() }
            }),
        );
    }

    let input = StdioRpcDispatchInput {
        id: Some(Value::String("11".to_owned())),
        method: Some("ListTools".to_owned()),
        params: InvocationInputBody::default(),
        params_api_key: Some("Bearer stdio-token".to_owned()),
        ..StdioRpcDispatchInput::default()
    };

    let response = dispatch_stdio_message(&input, &handlers, false);
    assert_eq!(response.jsonrpc, JSON_RPC_VERSION);
    assert_eq!(response.id, Some(Value::String("11".to_owned())));
    assert_eq!(response.result, Some("schema_result".to_owned()));
    assert_eq!(response.error, None);

    let headers = observed_headers.borrow();
    assert_eq!(headers.get("x-mcp-api-key"), Some(&"stdio-token".to_owned()));
    assert_eq!(headers.get("authorization"), Some(&"Bearer stdio-token".to_owned()));
}

#[test]
fn stdio_non_success_status_maps_to_jsonrpc_error_shape() {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    handlers.insert(
        "mrt_general_ci".to_owned(),
        Box::new(|_req: RpcDispatchRequestShim| RpcHandlerResponse {
            status: 429,
            body: "limit_concurrency_exceeded".to_owned(),
        }),
    );

    let input = StdioRpcDispatchInput {
        id: Some(Value::String("3".to_owned())),
        method: Some("mrt_general_ci".to_owned()),
        params: InvocationInputBody::default(),
        ..StdioRpcDispatchInput::default()
    };

    let response = dispatch_stdio_message(&input, &handlers, false);
    assert_eq!(response.result, None);
    let err = response.error.expect("429 result should map to error envelope");
    assert_eq!(err.code, 429);
    assert_eq!(err.message, "limit_concurrency_exceeded");
}

#[test]
fn stdio_chunk_framing_parses_messages_and_skips_invalid_json_lines() {
    let (buffer_after_partial, first_messages) = consume_stdio_input_chunk(
        "",
        "{\"id\":1,\"method\":\"mrt_audit\",\"params\":{\"apiKey\":\"Bearer one\"}}",
    );
    assert!(first_messages.is_empty());
    assert!(!buffer_after_partial.is_empty());

    let (buffer_after_full, second_messages) = consume_stdio_input_chunk(
        &buffer_after_partial,
        "\nnot-json\n{\"id\":2,\"method\":\"ListTools\",\"params\":{\"name\":\"mcp_schema\"}}\n\n",
    );
    assert_eq!(buffer_after_full, "");
    assert_eq!(second_messages.len(), 2);
    assert_eq!(second_messages[0].id, Some(Value::from(1)));
    assert_eq!(second_messages[0].method, Some("mrt_audit".to_owned()));
    assert_eq!(second_messages[1].id, Some(Value::from(2)));
    assert_eq!(second_messages[1].method, Some("ListTools".to_owned()));
    assert_eq!(second_messages[1].call_tool_name, Some("mcp_schema".to_owned()));
}

#[test]
fn stdio_chunk_framing_fail_closed_on_buffer_and_line_limits() {
    let huge_chunk = "x".repeat(MAX_STDIO_BUFFER_BYTES + 1);
    let (buffer, messages) = consume_stdio_input_chunk("", &huge_chunk);
    assert_eq!(buffer, "");
    assert!(messages.is_empty());

    let overlong_line = "{".to_owned() + &"x".repeat(MAX_STDIO_LINE_BYTES + 1) + "}\n";
    let (buffer, messages) = consume_stdio_input_chunk("", &overlong_line);
    assert_eq!(buffer, "");
    assert!(messages.is_empty());

    assert!(parse_stdio_rpc_line("not-json").is_none());
}

#[test]
fn stdio_output_line_matches_sendrpc_id_gate_and_jsonrpc_default() {
    let notification = StdioRpcResponse {
        jsonrpc: JSON_RPC_VERSION,
        id: None,
        result: Some("ignored".to_owned()),
        error: None,
    };
    assert!(format_stdio_rpc_output_line(&notification).is_none());

    let success = StdioRpcResponse {
        jsonrpc: "",
        id: Some(Value::from(5)),
        result: Some("ok".to_owned()),
        error: None,
    };
    let success_line = format_stdio_rpc_output_line(&success)
        .expect("responses with id should serialize to stdout line");
    assert!(success_line.contains("\"jsonrpc\":\"2.0\""));
    assert!(success_line.contains("\"id\":5"));
    assert!(success_line.contains("\"result\":\"ok\""));
    assert!(success_line.ends_with('\n'));

    let failure = StdioRpcResponse {
        jsonrpc: JSON_RPC_VERSION,
        id: Some(Value::String("6".to_owned())),
        result: None,
        error: Some(StdioRpcError { code: 404, message: "missing".to_owned() }),
    };
    let failure_line = format_stdio_rpc_output_line(&failure)
        .expect("error response with id should serialize to stdout line");
    assert!(failure_line.contains("\"error\""));
    assert!(failure_line.contains("\"code\":404"));
    assert!(failure_line.contains("\"message\":\"missing\""));
}
