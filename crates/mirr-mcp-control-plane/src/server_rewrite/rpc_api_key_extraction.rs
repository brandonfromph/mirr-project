#![forbid(unsafe_code)]

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RpcParamsApiKeyInput {
    pub api_key: Option<String>,
    pub meta: BTreeMap<String, String>,
    pub tool_name: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RpcEnvelopeApiKeyInput {
    pub api_key: Option<String>,
    pub meta: BTreeMap<String, String>,
    pub params: Option<RpcParamsApiKeyInput>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequestApiKeyInput {
    pub header_x_mcp_api_key: Option<String>,
    pub header_authorization: Option<String>,
    pub body_api_key: Option<String>,
    pub body_meta: BTreeMap<String, String>,
    pub query_api_key: Option<String>,
}

pub fn parse_api_key_value(raw_value: Option<&str>) -> String {
    let Some(value) = raw_value else {
        return String::new();
    };

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if let Some(stripped) = trimmed.strip_prefix("Bearer ") {
        return stripped.trim().to_owned();
    }

    trimmed.to_owned()
}

pub fn api_key_from_meta(meta: &BTreeMap<String, String>) -> String {
    let candidates =
        [meta.get("x-mcp-api-key"), meta.get("x_mcp_api_key"), meta.get("authorization")];

    for candidate in candidates {
        let token = parse_api_key_value(candidate.map(String::as_str));
        if !token.is_empty() {
            return token;
        }
    }

    String::new()
}

pub fn api_key_from_rpc_envelope(envelope: &RpcEnvelopeApiKeyInput) -> String {
    let direct = parse_api_key_value(envelope.api_key.as_deref());
    if !direct.is_empty() {
        return direct;
    }

    let from_meta = api_key_from_meta(&envelope.meta);
    if !from_meta.is_empty() {
        return from_meta;
    }

    if let Some(params) = &envelope.params {
        let params_direct = parse_api_key_value(params.api_key.as_deref());
        if !params_direct.is_empty() {
            return params_direct;
        }

        let params_meta = api_key_from_meta(&params.meta);
        if !params_meta.is_empty() {
            return params_meta;
        }
    }

    String::new()
}

pub fn api_key_from_request_sources(request: &RequestApiKeyInput) -> String {
    let header_candidates =
        [request.header_x_mcp_api_key.as_deref(), request.header_authorization.as_deref()];

    for candidate in header_candidates {
        let token = parse_api_key_value(candidate);
        if !token.is_empty() {
            return token;
        }
    }

    let body_token = {
        let meta_token = api_key_from_meta(&request.body_meta);
        if !meta_token.is_empty() {
            meta_token
        } else {
            parse_api_key_value(request.body_api_key.as_deref())
        }
    };
    if !body_token.is_empty() {
        return body_token;
    }

    let query_token = parse_api_key_value(request.query_api_key.as_deref());
    if !query_token.is_empty() {
        return query_token;
    }

    String::new()
}
