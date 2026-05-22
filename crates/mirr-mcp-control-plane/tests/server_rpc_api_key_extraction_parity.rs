use std::collections::BTreeMap;

use mirror::server_rewrite::rpc_api_key_extraction::{
    api_key_from_request_sources, api_key_from_rpc_envelope, parse_api_key_value,
    RequestApiKeyInput, RpcEnvelopeApiKeyInput, RpcParamsApiKeyInput,
};

#[test]
fn parse_api_key_value_matches_ts_behavior() {
    assert_eq!(parse_api_key_value(None), "");
    assert_eq!(parse_api_key_value(Some("   ")), "");
    assert_eq!(parse_api_key_value(Some("token-value")), "token-value");
    assert_eq!(parse_api_key_value(Some("Bearer token-from-header")), "token-from-header");
}

#[test]
fn rpc_envelope_api_key_precedence_matches_ts() {
    let mut envelope =
        RpcEnvelopeApiKeyInput { api_key: Some("direct".to_owned()), ..Default::default() };

    assert_eq!(api_key_from_rpc_envelope(&envelope), "direct");

    envelope.api_key = None;
    envelope.meta.insert("x-mcp-api-key".to_owned(), "meta-header".to_owned());
    assert_eq!(api_key_from_rpc_envelope(&envelope), "meta-header");

    envelope.meta.clear();
    envelope.params = Some(RpcParamsApiKeyInput {
        api_key: Some("params-direct".to_owned()),
        meta: BTreeMap::new(),
        tool_name: None,
    });
    assert_eq!(api_key_from_rpc_envelope(&envelope), "params-direct");

    if let Some(params) = envelope.params.as_mut() {
        params.api_key = None;
        params.meta.insert("authorization".to_owned(), "Bearer params-meta-auth".to_owned());
    }
    assert_eq!(api_key_from_rpc_envelope(&envelope), "params-meta-auth");
}

#[test]
fn request_api_key_precedence_matches_ts() {
    let request = RequestApiKeyInput {
        header_x_mcp_api_key: Some("header-x".to_owned()),
        header_authorization: Some("Bearer header-auth".to_owned()),
        body_api_key: Some("body-direct".to_owned()),
        body_meta: BTreeMap::new(),
        query_api_key: Some("query".to_owned()),
    };
    assert_eq!(api_key_from_request_sources(&request), "header-x");

    let request = RequestApiKeyInput {
        header_x_mcp_api_key: None,
        header_authorization: Some("Bearer header-auth".to_owned()),
        body_api_key: Some("body-direct".to_owned()),
        body_meta: BTreeMap::new(),
        query_api_key: Some("query".to_owned()),
    };
    assert_eq!(api_key_from_request_sources(&request), "header-auth");

    let request = RequestApiKeyInput {
        header_x_mcp_api_key: None,
        header_authorization: None,
        body_api_key: Some("body-direct".to_owned()),
        body_meta: BTreeMap::new(),
        query_api_key: Some("query".to_owned()),
    };
    assert_eq!(api_key_from_request_sources(&request), "body-direct");

    let mut body_meta = BTreeMap::new();
    body_meta.insert("x_mcp_api_key".to_owned(), "body-meta".to_owned());
    let request = RequestApiKeyInput {
        header_x_mcp_api_key: None,
        header_authorization: None,
        body_api_key: None,
        body_meta,
        query_api_key: Some("query".to_owned()),
    };
    assert_eq!(api_key_from_request_sources(&request), "body-meta");

    let request = RequestApiKeyInput {
        header_x_mcp_api_key: None,
        header_authorization: None,
        body_api_key: None,
        body_meta: BTreeMap::new(),
        query_api_key: Some("query".to_owned()),
    };
    assert_eq!(api_key_from_request_sources(&request), "query");
}
