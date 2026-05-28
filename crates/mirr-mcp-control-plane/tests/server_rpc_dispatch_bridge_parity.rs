use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_input::InvocationInputBody;
use mirr_mcp_control_plane::server_rewrite::rpc_dispatch_bridge::{
    dispatch_rpc_to_handler, RpcDispatchBody, RpcDispatchMessage, RpcDispatchRequestShim,
    RpcHandlerMap, RpcHandlerResponse, MAX_REGISTERED_HANDLERS,
};

#[test]
fn dispatch_bridge_rejects_unknown_methods_fail_closed() {
    let called = Rc::new(Cell::new(false));
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    {
        let called_ref = called.clone();
        handlers.insert(
            "mcp_schema".to_owned(),
            Box::new(move |_req| {
                called_ref.set(true);
                RpcHandlerResponse { status: 200, body: "schema".to_owned() }
            }),
        );
    }

    let message = RpcDispatchMessage {
        method: Some("unknown_method".to_owned()),
        params: InvocationInputBody::default(),
        ..RpcDispatchMessage::default()
    };

    let result = dispatch_rpc_to_handler(&message, &handlers);
    assert_eq!(result.status, 404);
    assert!(!called.get());

    match result.body {
        RpcDispatchBody::StableError(error) => {
            assert_eq!(error.error_code, "validation_unknown_method");
            assert!(error.message.contains("unknown_method"));
        }
        other => panic!("expected stable error body, got: {:?}", other),
    }
}

#[test]
fn dispatch_bridge_uses_normalized_method_name_for_handler_lookup() {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    handlers.insert(
        "mcp_schema".to_owned(),
        Box::new(|_req| RpcHandlerResponse { status: 200, body: "schema_ok".to_owned() }),
    );

    let message = RpcDispatchMessage {
        method: Some("ListTools".to_owned()),
        params: InvocationInputBody::default(),
        ..RpcDispatchMessage::default()
    };

    let result = dispatch_rpc_to_handler(&message, &handlers);
    assert_eq!(result.status, 200);
    assert_eq!(result.body, RpcDispatchBody::Payload("schema_ok".to_owned()));
}

#[test]
fn dispatch_bridge_injects_both_api_key_headers() {
    let captured_headers = Rc::new(RefCell::new(BTreeMap::<String, String>::new()));
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    {
        let captured_headers_ref = captured_headers.clone();
        handlers.insert(
            "mrt_audit".to_owned(),
            Box::new(move |req: RpcDispatchRequestShim| {
                *captured_headers_ref.borrow_mut() = req.headers.clone();
                RpcHandlerResponse { status: 200, body: "ok".to_owned() }
            }),
        );
    }

    let message = RpcDispatchMessage {
        method: Some("mrt_audit".to_owned()),
        params: InvocationInputBody::default(),
        params_api_key: Some("Bearer token-123".to_owned()),
        ..RpcDispatchMessage::default()
    };

    let result = dispatch_rpc_to_handler(&message, &handlers);
    assert_eq!(result.status, 200);

    let headers = captured_headers.borrow();
    assert_eq!(headers.get("x-mcp-api-key"), Some(&"token-123".to_owned()));
    assert_eq!(headers.get("authorization"), Some(&"Bearer token-123".to_owned()));
}

#[test]
fn dispatch_bridge_passthroughs_handler_status_and_body() {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    handlers.insert(
        "mrt_general_ci".to_owned(),
        Box::new(|_req| RpcHandlerResponse {
            status: 503,
            body: "upstream_unavailable".to_owned(),
        }),
    );

    let message = RpcDispatchMessage {
        method: Some("mrt_general_ci".to_owned()),
        params: InvocationInputBody::default(),
        ..RpcDispatchMessage::default()
    };

    let result = dispatch_rpc_to_handler(&message, &handlers);
    assert_eq!(result.status, 503);
    assert_eq!(result.body, RpcDispatchBody::Payload("upstream_unavailable".to_owned()));
}

#[test]
fn dispatch_bridge_rejects_oversized_handler_registry() {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();
    for idx in 0..(MAX_REGISTERED_HANDLERS + 1) {
        handlers.insert(
            format!("method_{}", idx),
            Box::new(|_req| RpcHandlerResponse { status: 200, body: "ok".to_owned() }),
        );
    }

    let message = RpcDispatchMessage {
        method: Some("method_0".to_owned()),
        params: InvocationInputBody::default(),
        ..RpcDispatchMessage::default()
    };

    let result = dispatch_rpc_to_handler(&message, &handlers);
    assert_eq!(result.status, 404);
    match result.body {
        RpcDispatchBody::StableError(error) => {
            assert_eq!(error.error_code, "validation_unknown_method");
        }
        other => panic!("expected stable error body, got: {:?}", other),
    }
}
