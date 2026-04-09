#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::env;
use std::io::{self, BufRead, BufReader, Read, Write};

use mirror::catalog::{
    CANONICAL_CATALOG_ID, CATALOG_ALIASES, MCP_PROTOCOL_VERSION, MCP_SERVER_NAME,
    MCP_SERVER_VERSION,
};
use mirror::policy::Role;
use mirror::server_rewrite::axum_route_host::{dispatch_host_stdio_line, AxumMcpHostState};
use mirror::server_rewrite::rpc_dispatch_bridge::{RpcHandlerMap, RpcHandlerResponse};
use mirror::server_rewrite::rpc_role_gate::{RoleTokenMap, VerifiedPrincipal};
use mirror::server_rewrite::rpc_stdio_message_dispatch::{
    format_stdio_rpc_output_line, MAX_STDIO_LINE_BYTES,
};
use mirror::tooling::{discovery_method_by_name, DiscoveryParameter, MrtDispatchTool};
use serde_json::{json, Map, Value};

const MAX_STDIO_INPUT_LINES: usize = 1_000_000;
const MAX_STDIO_HEADER_LINES: usize = 64;
const MAX_STDIO_FRAME_BYTES: usize = MAX_STDIO_LINE_BYTES;
const MAX_SCHEMA_PARAMETERS: usize = 128;

fn json_type_from_parameter_type(parameter_type: &str) -> &'static str {
    match parameter_type {
        "string" => "string",
        "number" => "number",
        "array" => "array",
        "boolean" => "boolean",
        _ => "string",
    }
}

fn build_input_schema(parameters: &[DiscoveryParameter]) -> Value {
    let mut properties = Map::<String, Value>::new();
    let mut required = Vec::<String>::new();

    for parameter in parameters.iter().take(MAX_SCHEMA_PARAMETERS) {
        properties.insert(
            parameter.name.to_owned(),
            json!({ "type": json_type_from_parameter_type(parameter.ty) }),
        );

        if parameter.required {
            required.push(parameter.name.to_owned());
        }
    }

    json!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": false,
    })
}

fn build_initialize_result() -> String {
    json!({
        "protocolVersion": MCP_PROTOCOL_VERSION,
        "serverInfo": {
            "name": MCP_SERVER_NAME,
            "version": MCP_SERVER_VERSION,
        },
        "capabilities": {
            "tools": {
                "listChanged": false,
            },
            "resources": {
                "listChanged": false,
                "subscribe": false,
            },
            "logging": {},
        },
    })
    .to_string()
}

fn build_tools_list_result() -> String {
    let mut tools = Vec::<Value>::new();
    for tool in MrtDispatchTool::ALL {
        if let Some(method) = discovery_method_by_name(tool.as_str()) {
            tools.push(json!({
                "name": method.name,
                "description": method.description,
                "inputSchema": build_input_schema(method.parameters),
                "annotations": {
                    "readOnlyHint": method.auto_approve,
                },
            }));
        }
    }

    json!({ "tools": tools }).to_string()
}

fn build_schema_result() -> String {
    let mut methods = Map::<String, Value>::new();

    for tool in MrtDispatchTool::ALL {
        if let Some(method) = discovery_method_by_name(tool.as_str()) {
            methods.insert(
                method.name.to_owned(),
                json!({
                    "autoApprove": method.auto_approve,
                    "description": method.description,
                    "parameters": method
                        .parameters
                        .iter()
                        .take(MAX_SCHEMA_PARAMETERS)
                        .map(|parameter| {
                            json!({
                                "name": parameter.name,
                                "required": parameter.required,
                                "type": json_type_from_parameter_type(parameter.ty),
                            })
                        })
                        .collect::<Vec<Value>>(),
                }),
            );
        }
    }

    json!({
        "name": CANONICAL_CATALOG_ID,
        "aliases": CATALOG_ALIASES,
        "methods": methods,
    })
    .to_string()
}

fn build_resource_templates_result() -> String {
    json!({ "resourceTemplates": [] }).to_string()
}

fn build_ping_result() -> String {
    json!({}).to_string()
}

fn handler_factory() -> RpcHandlerMap<String> {
    let mut handlers: RpcHandlerMap<String> = BTreeMap::new();

    handlers.insert(
        "mcp_initialize".to_owned(),
        Box::new(|_req| RpcHandlerResponse { status: 200, body: build_initialize_result() }),
    );

    handlers.insert(
        "mcp_schema".to_owned(),
        Box::new(|_req| RpcHandlerResponse { status: 200, body: build_schema_result() }),
    );

    handlers.insert(
        "tools/list".to_owned(),
        Box::new(|_req| RpcHandlerResponse { status: 200, body: build_tools_list_result() }),
    );

    handlers.insert(
        "resources/templates/list".to_owned(),
        Box::new(|_req| RpcHandlerResponse {
            status: 200,
            body: build_resource_templates_result(),
        }),
    );

    handlers.insert(
        "ping".to_owned(),
        Box::new(|_req| RpcHandlerResponse { status: 200, body: build_ping_result() }),
    );

    handlers
}

fn insert_env_role_token(tokens: &mut RoleTokenMap, key: &str, role: Role) {
    let Ok(raw) = env::var(key) else {
        return;
    };

    let token = raw.trim();
    if token.is_empty() {
        return;
    }

    let principal = VerifiedPrincipal {
        id: format!("env_{}", key.to_ascii_lowercase()),
        role,
    };

    tokens
        .entry(token.to_owned())
        .and_modify(|existing| {
            if principal.role < existing.role {
                *existing = principal.clone();
            }
        })
        .or_insert(principal);
}

fn role_tokens_from_env() -> RoleTokenMap {
    let mut tokens = RoleTokenMap::new();
    for (key, role) in [
        ("MCP_AUTH_TOKEN", Role::Builder),
        ("AUTH_TOKEN", Role::Builder),
        ("MCP_READER_AUTH_TOKEN", Role::Reader),
        ("MCP_BUILDER_AUTH_TOKEN", Role::Builder),
        ("MCP_COMMITTER_AUTH_TOKEN", Role::Committer),
        ("MCP_ADMIN_AUTH_TOKEN", Role::Admin),
    ] {
        insert_env_role_token(&mut tokens, key, role);
    }

    tokens
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StdioInputMode {
    JsonLine,
    Framed,
}

fn trim_line_endings(raw: &str) -> &str {
    raw.trim_end_matches(['\r', '\n'])
}

fn parse_content_length_from_headers(header_lines: &[String]) -> Result<Option<usize>, String> {
    let mut content_length = None::<usize>;

    for line in header_lines {
        let Some((name, raw_value)) = line.split_once(':') else {
            continue;
        };

        if !name.trim().eq_ignore_ascii_case("content-length") {
            continue;
        }

        let parsed = raw_value
            .trim()
            .parse::<usize>()
            .map_err(|error| format!("invalid_content_length_header: {}", error))?;

        if let Some(existing) = content_length {
            if existing != parsed {
                return Err("conflicting_content_length_headers".to_owned());
            }
        } else {
            content_length = Some(parsed);
        }
    }

    Ok(content_length)
}

fn write_response<W: Write>(
    writer: &mut W,
    response: &mirror::server_rewrite::rpc_stdio_message_dispatch::StdioRpcResponse,
    mode: StdioInputMode,
) -> Result<(), String> {
    let Some(output_line) = format_stdio_rpc_output_line(response) else {
        return Ok(());
    };

    match mode {
        StdioInputMode::JsonLine => {
            writer
                .write_all(output_line.as_bytes())
                .map_err(|error| format!("failed_to_write_stdout: {}", error))?;
        }
        StdioInputMode::Framed => {
            let payload = output_line.strip_suffix('\n').unwrap_or(output_line.as_str());
            let framed = format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload);
            writer
                .write_all(framed.as_bytes())
                .map_err(|error| format!("failed_to_write_stdout: {}", error))?;
        }
    }

    writer.flush().map_err(|error| format!("failed_to_flush_stdout: {}", error))
}

fn dispatch_payload_and_write<W: Write>(
    state: &AxumMcpHostState,
    writer: &mut W,
    payload: &str,
    mode: StdioInputMode,
) -> Result<(), String> {
    let Some((_status_code, response)) = dispatch_host_stdio_line(state, payload) else {
        return Ok(());
    };

    write_response(writer, &response, mode)
}

fn run_stdio_host() -> Result<(), String> {
    let state = AxumMcpHostState::with_role_tokens(handler_factory, role_tokens_from_env());

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let mut line = String::new();
    let mut observed_lines = 0usize;
    while observed_lines < MAX_STDIO_INPUT_LINES {
        line.clear();
        let bytes_read = reader
            .read_line(&mut line)
            .map_err(|error| format!("failed_to_read_stdin_line_{}: {}", observed_lines, error))?;
        if bytes_read == 0 {
            break;
        }

        observed_lines = observed_lines.saturating_add(1);
        let trimmed = trim_line_endings(&line);
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('{') {
            dispatch_payload_and_write(&state, &mut writer, trimmed, StdioInputMode::JsonLine)?;
            continue;
        }

        let mut header_lines = vec![trimmed.to_owned()];
        let mut found_header_separator = false;

        for _ in 0..MAX_STDIO_HEADER_LINES {
            if observed_lines >= MAX_STDIO_INPUT_LINES {
                return Err("stdio_input_line_limit_exceeded".to_owned());
            }

            line.clear();
            let header_bytes = reader.read_line(&mut line).map_err(|error| {
                format!("failed_to_read_stdin_header_line_{}: {}", observed_lines, error)
            })?;
            if header_bytes == 0 {
                break;
            }

            observed_lines = observed_lines.saturating_add(1);
            let header_line = trim_line_endings(&line);
            if header_line.is_empty() {
                found_header_separator = true;
                break;
            }
            header_lines.push(header_line.to_owned());
        }

        if !found_header_separator {
            return Err("incomplete_stdio_header_block".to_owned());
        }

        let Some(content_length) = parse_content_length_from_headers(&header_lines)? else {
            continue;
        };

        if content_length > MAX_STDIO_FRAME_BYTES {
            return Err(format!(
                "stdio_content_length_exceeded_limit: {} > {}",
                content_length, MAX_STDIO_FRAME_BYTES
            ));
        }

        let mut payload_bytes = vec![0_u8; content_length];
        reader.read_exact(&mut payload_bytes).map_err(|error| {
            format!(
                "failed_to_read_framed_stdio_payload_{}_bytes: {}",
                content_length, error
            )
        })?;

        let payload = std::str::from_utf8(&payload_bytes)
            .map_err(|error| format!("invalid_utf8_stdio_payload: {}", error))?;

        dispatch_payload_and_write(&state, &mut writer, payload, StdioInputMode::Framed)?;
    }

    Ok(())
}

fn main() {
    if let Err(error) = run_stdio_host() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
