#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

fn rust_control_plane_source(relative_path: &str) -> String {
    let path = Path::new("crates/mirr-mcp-control-plane/src").join(relative_path);
    fs::read_to_string(path).expect("control-plane Rust source must be readable")
}

fn mirr_brain_source() -> String {
    let path = Path::new("src/bin/mirr-brain.rs");
    fs::read_to_string(path).expect("mirr-brain.rs must be readable")
}

#[test]
fn rwfi2_mcp_stdio_host_registers_handshake_and_discovery_handlers() {
    let src = rust_control_plane_source("bin/mirr-mcp-stdio-host.rs");

    assert!(src.contains("mcp_initialize"));
    assert!(src.contains("mcp_schema"));
    assert!(src.contains("tools/list"));
    assert!(src.contains("resources/templates/list"));
    assert!(src.contains("ping"));
}

#[test]
fn rwfi2_mcp_protocol_and_catalog_constants_are_declared() {
    let src = rust_control_plane_source("../src/catalog.rs");

    assert!(src.contains("MCP_PROTOCOL_VERSION"));
    assert!(src.contains("MCP_SERVER_NAME"));
    assert!(src.contains("MCP_SERVER_VERSION"));
    assert!(src.contains("CANONICAL_CATALOG_ID"));
}

#[test]
fn rwfi2_rpc_aliases_cover_initialize_and_tools_call() {
    let src = rust_control_plane_source("server_rewrite/rpc_method_aliases.rs");

    assert!(src.contains("INITIALIZE_ALIAS"));
    assert!(src.contains("TOOLS_CALL_METHOD"));
    assert!(src.contains("resolve_method_alias"));
    assert!(src.contains("mcp_initialize"));
}

#[test]
fn rwfi2_stdio_dispatch_limits_and_unknown_method_contract_are_bounded() {
    let src = rust_control_plane_source("server_rewrite/rpc_stdio_message_dispatch.rs");

    assert!(src.contains("MAX_STDIO_BUFFER_BYTES"));
    assert!(src.contains("MAX_STDIO_LINE_BYTES"));
    assert!(src.contains("MCP unknown method rejected"));
}

#[test]
fn rwfi2_transport_bootstrap_supports_stdio_and_stream_startup() {
    let src = rust_control_plane_source("server_rewrite/transport_bootstrap.rs");

    assert!(src.contains("TransportStartupAction"));
    assert!(src.contains("StartStdio"));
    assert!(src.contains("StartStream"));
}

#[test]
fn rwfi2_canonical_tool_catalog_includes_required_mrt_methods() {
    let src = rust_control_plane_source("tooling/canonical_discovery_method_metadata.rs");

    assert!(src.contains("mrt_audit"));
    assert!(src.contains("mrt_brain_get"));
    assert!(src.contains("mrt_general_ci"));
    assert!(src.contains("mrt_general_ci_compile"));
    assert!(src.contains("mrt_general_ci_fast"));
    assert!(src.contains("mrt_wave_dry_run"));
    assert!(src.contains("mrt_wave_apply"));
    assert!(src.contains("mrt_lsp_diagnostics"));
    assert!(src.contains("mrt_compile"));
    assert!(src.contains("mrt_rspu_validate"));
    assert!(src.contains("mrt_rspu_proofs"));
}

#[test]
fn rwfi2_mirr_brain_source_has_bounded_contract_constants_and_root_flag() {
    let src = mirr_brain_source();
    assert!(src.contains("MAX_RESULTS"));
    assert!(src.contains("MAX_ENTRY_SIZE"));
    assert!(src.contains("DEFAULT_KB_ROOT"));
    assert!(src.contains("kb_root"));
    assert!(src.contains(".kb-data"));
}
