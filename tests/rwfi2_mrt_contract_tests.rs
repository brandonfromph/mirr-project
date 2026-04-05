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

fn kb_lite_source() -> String {
    let path = Path::new("mcp_server/src/mrt_kb_lite.ts");
    fs::read_to_string(path).expect("mrt_kb_lite.ts must be readable")
}

fn mirr_brain_source() -> String {
    let path = Path::new("src/bin/mirr-brain.rs");
    fs::read_to_string(path).expect("mirr-brain.rs must be readable")
}

#[test]
fn rwfi2_mrt_has_typed_allowlist() {
    let src = mrt_source();
    assert!(src.contains("type MrtToolName"));
    assert!(src.contains("\"mirr-audit\""));
    assert!(src.contains("\"mirr-brain\""));
    assert!(src.contains("\"mirr-general\""));
    assert!(src.contains("\"mirr-wave\""));
    assert!(src.contains("\"mirr-lsp\""));
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
    assert!(src.contains("case \"mrt_general_ci_compile\""));
    assert!(src.contains("case \"mrt_general_ci_fast\""));
    assert!(src.contains("case \"mrt_wave_dry_run\""));
    assert!(src.contains("case \"mrt_wave_apply\""));
    assert!(src.contains("case \"mrt_lsp_diagnostics\""));
    assert!(src.contains("callMrtInterface"));
}

#[test]
fn rwfi2_mrt_runtime_dispatch_routes_exist() {
    let src = server_source();
    assert!(src.contains("app.post(\"/mrt_audit\""));
    assert!(src.contains("app.post(\"/mrt_brain_get\""));
    assert!(src.contains("app.post(\"/mrt_general_ci\""));
    assert!(src.contains("app.post(\"/mrt_general_ci_compile\""));
    assert!(src.contains("app.post(\"/mrt_general_ci_fast\""));
    assert!(src.contains("app.post(\"/mrt_wave_dry_run\""));
    assert!(src.contains("app.post(\"/mrt_wave_apply\""));
    assert!(src.contains("app.post(\"/mrt_lsp_diagnostics\""));
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
fn rwfi2_mrt_runtime_includes_bounded_output_metadata_markers() {
    let src = server_source();
    assert!(src.contains("output_limit_bytes"));
    assert!(src.contains("stdout_truncated"));
    assert!(src.contains("stderr_truncated"));
    assert!(src.contains("MRT_STRICT_MODE"));
    assert!(src.contains("if (MRT_STRICT_MODE)"));
    assert!(src.contains("MCP unknown method rejected:"));
    assert!(src.contains("MRT_COMPAT_UNKNOWN_METHODS"));
    assert!(src.contains("MRT_ENABLE_EXECUTE_COMPAT"));
    assert!(src.contains("mrt_execute_compat_disabled"));
}

#[test]
fn rwfi2_mrt_runtime_has_single_route_registration_per_tool() {
    let src = server_source();

    assert_eq!(src.matches("app.post(\"/search_files\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/directory_tree\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_audit\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_brain_get\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_general_ci\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_general_ci_compile\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_general_ci_fast\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_wave_dry_run\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_wave_apply\"").count(), 1);
    assert_eq!(src.matches("app.post(\"/mrt_lsp_diagnostics\"").count(), 1);
}

#[test]
fn rwfi2_kb_lite_contract_is_bounded_and_kb_rooted() {
    let src = kb_lite_source();
    assert!(src.contains("KB_ROOT"));
    assert!(src.contains(".kb-data"));
    assert!(src.contains("MAX_KB_KEY_SIZE"));
    assert!(src.contains("MAX_OUTPUT_BYTES"));
    assert!(src.contains("--kb-root"));
}

#[test]
fn rwfi2_mrt_brain_dispatch_uses_brain_get_args() {
    let mrt_src = mrt_source();
    let server_src = server_source();

    assert!(mrt_src.contains("case \"mrt_brain_get\""));
    assert!(mrt_src.contains("brainGetArgs(getStringArg(args, [\"key\"]))"));
    assert!(mrt_src.contains("callMrtInterface(\"mirr-brain\""));

    assert!(server_src.contains("case \"mrt_brain_get\""));
    assert!(server_src.contains("args: brainGetArgs(getBodyString(body, \"key\"))"));
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
