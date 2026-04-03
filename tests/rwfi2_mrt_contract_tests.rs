#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

fn mrt_source() -> String {
    let path = Path::new("mcp_server/src/mrt.ts");
    fs::read_to_string(path).expect("mrt.ts must be readable")
}

fn server_source() -> String {
    let path = Path::new("mcp_server/src/server.ts");
    fs::read_to_string(path).expect("server.ts must be readable")
}

#[test]
fn rwfi2_mrt_has_typed_allowlist() {
    let src = mrt_source();
    assert!(src.contains("type MrtToolName"));
    assert!(src.contains("\"mirr-audit\""));
    assert!(src.contains("\"mirr-brain\""));
    assert!(src.contains("\"mirr-general\""));
}

#[test]
fn rwfi2_mrt_has_schema_v1_request_and_error_shape() {
    let src = mrt_source();
    assert!(src.contains("schema_version: \"1\""));
    assert!(src.contains("code: \"MRT_EXEC_ERROR\""));
    assert!(src.contains("message"));
}

#[test]
fn rwfi2_mrt_rejects_unknown_tools() {
    let src = mrt_source();
    assert!(src.contains("MRT_EXEC_ERROR: unknown tool"));
}

#[test]
fn rwfi2_mrt_routes_required_handlers() {
    let src = mrt_source();
    assert!(src.contains("case \"mrt_audit\""));
    assert!(src.contains("case \"mrt_brain_get\""));
    assert!(src.contains("case \"mrt_general_ci\""));
    assert!(src.contains("callMrtInterface"));
}

#[test]
fn rwfi2_mrt_runtime_dispatch_routes_exist() {
    let src = server_source();
    assert!(src.contains("app.post(\"/mrt_audit\""));
    assert!(src.contains("app.post(\"/mrt_brain_get\""));
    assert!(src.contains("app.post(\"/mrt_general_ci\""));
    assert!(src.contains("app.post(\"/mrt_execute\""));
}

#[test]
fn rwfi2_mrt_runtime_enforces_role_gate() {
    let src = server_source();
    assert!(src.contains("requireMrtDispatchRole(req, toolName)"));
    assert!(src.contains("missing_api_key"));
    assert!(src.contains("role: (rr as any).role ?? null"));
}

#[test]
fn rwfi2_mrt_runtime_has_single_route_registration_per_tool() {
    let src = server_source();

    assert_eq!(src.matches("app.post(\"/search_files\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/directory_tree\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_audit\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_brain_get\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_general_ci\"").count(), 1);
}
