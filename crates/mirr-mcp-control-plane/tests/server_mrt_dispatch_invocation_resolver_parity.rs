use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_input::InvocationInputBody;
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_resolver::resolve_mrt_dispatch_invocation_by_name;

#[test]
fn resolve_defaults_match_ts_dispatch_contract() {
    let body = InvocationInputBody::default();

    let audit = resolve_mrt_dispatch_invocation_by_name("mrt_audit", &body)
        .expect("mrt_audit should resolve with defaults");
    assert_eq!(
        audit.args,
        vec![
            "run",
            "--bin",
            "mirr-audit",
            "--",
            "--mode",
            "workspace",
            "--glob",
            "src/**/*.rs",
            "--format",
            "json",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<String>>()
    );
    assert_eq!(audit.stdin_data, None);

    let general_ci = resolve_mrt_dispatch_invocation_by_name("mrt_general_ci", &body)
        .expect("mrt_general_ci should resolve");
    assert_eq!(
        general_ci.args,
        vec!["run", "--bin", "mirr-general", "--", "ci", "--format", "json"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<String>>()
    );

    let lra_validate = resolve_mrt_dispatch_invocation_by_name("lra_validate", &body)
        .expect("lra_validate should resolve with default path");
    assert_eq!(
        lra_validate.args,
        vec!["run", "-p", "lra-cli", "--", "validate", "index.html"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<String>>()
    );
}

#[test]
fn resolve_alias_heavy_invocations_match_ts_contract() {
    let mut body = InvocationInputBody::default();
    body.set_string("proposalId", "107");
    body.set_string("proposalFile", "proposals/107-test.md");
    body.set_string("maxLines", "7");

    let wave = resolve_mrt_dispatch_invocation_by_name("mrt_wave_dry_run", &body)
        .expect("mrt_wave_dry_run should resolve with alias keys");
    assert_eq!(
        wave.args,
        vec![
            "run",
            "--bin",
            "mirr-wave",
            "--",
            "--proposal-id",
            "107",
            "--proposal-file",
            "proposals/107-test.md",
            "--max-lines",
            "7",
            "--dry-run",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<String>>()
    );

    let mut lsp_body = InvocationInputBody::default();
    lsp_body.set_string("sourceText", "module sensor {};");
    let lsp = resolve_mrt_dispatch_invocation_by_name("mrt_lsp_diagnostics", &lsp_body)
        .expect("mrt_lsp_diagnostics should resolve sourceText alias");
    assert_eq!(
        lsp.args,
        vec!["run", "--bin", "mirr-lsp", "--"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<String>>()
    );
    let stdin = lsp.stdin_data.expect("lsp invocation should include stdin payload");
    assert!(stdin.contains("textDocument/didOpen"));
    assert!(stdin.contains("module sensor {}"));

    let mut rspu_body = InvocationInputBody::default();
    rspu_body.set_string("source", "examples/demo.mirr");
    rspu_body.set_string("proofMethods", "alpha,beta,alpha");
    let rspu = resolve_mrt_dispatch_invocation_by_name("mrt_rspu_proofs", &rspu_body)
        .expect("mrt_rspu_proofs should resolve methods from CSV alias");
    assert!(rupu_args_contains(&rspu.args, "--methods"));
    assert!(rupu_args_contains(&rspu.args, "alpha,beta"));

    let mut daemon_core_body = InvocationInputBody::default();
    daemon_core_body.set_string("testFilter", "daemon_core_starts_in_stopped_state");
    let daemon_core =
        resolve_mrt_dispatch_invocation_by_name("mrt_daemon_core_contract", &daemon_core_body)
            .expect("mrt_daemon_core_contract should resolve with test filter aliases");
    assert_eq!(
        daemon_core.args,
        vec![
            "test",
            "--test",
            "wave5_daemon_core_architecture_tests",
            "daemon_core_starts_in_stopped_state",
            "--",
            "--nocapture",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<String>>()
    );

    let daemon_security =
        resolve_mrt_dispatch_invocation_by_name("mrt_daemon_security_contract", &body)
            .expect("mrt_daemon_security_contract should resolve without optional filter");
    assert_eq!(
        daemon_security.args,
        vec!["test", "--test", "wave6_daemon_security_runtime_policy_tests", "--", "--nocapture",]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<String>>()
    );
}

#[test]
fn resolve_wave_inputs_match_ts_fail_closed_contract() {
    let mut missing_id = InvocationInputBody::default();
    missing_id.set_string("proposalFile", "proposals/107-test.md");
    let err = resolve_mrt_dispatch_invocation_by_name("mrt_wave_dry_run", &missing_id)
        .expect_err("missing proposal id must be rejected");
    assert_eq!(err, "missing_proposal_id");

    let mut invalid_id = InvocationInputBody::default();
    invalid_id.set_string("proposalId", "12");
    invalid_id.set_string("proposalFile", "proposals/107-test.md");
    let err = resolve_mrt_dispatch_invocation_by_name("mrt_wave_dry_run", &invalid_id)
        .expect_err("proposal id must be at least three digits");
    assert_eq!(err, "invalid_proposal_id");

    let mut missing_file = InvocationInputBody::default();
    missing_file.set_string("proposalId", "107");
    let err = resolve_mrt_dispatch_invocation_by_name("mrt_wave_dry_run", &missing_file)
        .expect_err("missing proposal file must be rejected");
    assert_eq!(err, "missing_proposal_file");

    let mut invalid_file = InvocationInputBody::default();
    invalid_file.set_string("proposalId", "107");
    invalid_file.set_string("proposalFile", "../proposals/107-test.md");
    let err = resolve_mrt_dispatch_invocation_by_name("mrt_wave_dry_run", &invalid_file)
        .expect_err("path traversal-style proposal file must be rejected");
    assert_eq!(err, "invalid_proposal_file");

    let mut normalized = InvocationInputBody::default();
    normalized.set_string("proposalId", "107");
    normalized.set_string("proposalFile", ".\\proposals\\\\107-test.md");
    normalized.set_string("maxLines", "not-a-number");
    let wave_apply = resolve_mrt_dispatch_invocation_by_name("mrt_wave_apply", &normalized)
        .expect("normalized proposal file and fallback max lines should resolve");

    assert!(rupu_args_contains(&wave_apply.args, "proposals/107-test.md"));
    assert!(rupu_args_contains(&wave_apply.args, "128"));
    assert!(!rupu_args_contains(&wave_apply.args, "--dry-run"));
}

#[test]
fn resolve_lra_init_rejects_unsafe_project_names() {
    let mut valid_body = InvocationInputBody::default();
    valid_body.set_string("projectName", "paper_2026");
    let valid = resolve_mrt_dispatch_invocation_by_name("lra_init", &valid_body)
        .expect("safe project name should resolve");
    assert!(rupu_args_contains(&valid.args, "paper_2026"));

    let mut traversal_body = InvocationInputBody::default();
    traversal_body.set_string("projectName", "../outside");
    let traversal_err = resolve_mrt_dispatch_invocation_by_name("lra_init", &traversal_body)
        .expect_err("path traversal style project names must be rejected");
    assert_eq!(traversal_err, "invalid_project_name");

    let mut separator_body = InvocationInputBody::default();
    separator_body.set_string("projectName", "nested/project");
    let separator_err = resolve_mrt_dispatch_invocation_by_name("lra_init", &separator_body)
        .expect_err("path separator project names must be rejected");
    assert_eq!(separator_err, "invalid_project_name");

    let mut symbol_body = InvocationInputBody::default();
    symbol_body.set_string("projectName", "paper name");
    let symbol_err = resolve_mrt_dispatch_invocation_by_name("lra_init", &symbol_body)
        .expect_err("unsafe symbol project names must be rejected");
    assert_eq!(symbol_err, "invalid_project_name");
}

#[test]
fn resolve_unknown_tool_is_fail_closed() {
    let body = InvocationInputBody::default();
    let err = resolve_mrt_dispatch_invocation_by_name("mrt_unknown", &body)
        .expect_err("unknown tools must be rejected");
    assert!(err.contains("MCP unknown method rejected: mrt_unknown."));
}

#[test]
fn resolve_kb_query_supports_phase4_parameters() {
    let mut body = InvocationInputBody::default();
    body.set_string("query", "find module alpha dependencies");
    body.set_string("mode", "graph");
    body.set_number("limit", 7.0);
    body.set_string("filter", "module:alpha chunk_type:Module");
    body.set_string("expand_mode", "synonym");
    body.set_number("retry_count", 2.0);
    body.set_number("timeout_ms", 5000.0);

    let resolved = resolve_mrt_dispatch_invocation_by_name("mrt_kb_query", &body)
        .expect("kb query should resolve with phase4 params");

    assert!(rupu_args_contains(&resolved.args, "--bin"));
    assert!(rupu_args_contains(&resolved.args, "mirr-kb-native"));
    assert!(rupu_args_contains(&resolved.args, "--expand-mode"));
    assert!(rupu_args_contains(&resolved.args, "synonym"));
    assert!(rupu_args_contains(&resolved.args, "--retry-count"));
    assert!(rupu_args_contains(&resolved.args, "2"));
    assert!(rupu_args_contains(&resolved.args, "--timeout-ms"));
    assert!(rupu_args_contains(&resolved.args, "5000"));
}

#[test]
fn resolve_kb_query_rejects_invalid_phase4_parameters() {
    let mut invalid_expand = InvocationInputBody::default();
    invalid_expand.set_string("query", "x");
    invalid_expand.set_string("expand_mode", "wide");
    let expand_err = resolve_mrt_dispatch_invocation_by_name("mrt_kb_query", &invalid_expand)
        .expect_err("invalid expand_mode must fail closed");
    assert_eq!(expand_err, "expand_mode must be one of none|synonym|hyde");

    let mut invalid_retry = InvocationInputBody::default();
    invalid_retry.set_string("query", "x");
    invalid_retry.set_number("retry_count", 9.0);
    let retry_err = resolve_mrt_dispatch_invocation_by_name("mrt_kb_query", &invalid_retry)
        .expect_err("invalid retry_count must fail closed");
    assert_eq!(retry_err, "retry_count must be between 0 and 5");

    let mut invalid_timeout = InvocationInputBody::default();
    invalid_timeout.set_string("query", "x");
    invalid_timeout.set_number("timeout_ms", 10.0);
    let timeout_err = resolve_mrt_dispatch_invocation_by_name("mrt_kb_query", &invalid_timeout)
        .expect_err("invalid timeout must fail closed");
    assert_eq!(timeout_err, "timeout_ms must be between 1000 and 60000");
}

#[test]
fn resolve_kb_index_supports_optional_path() {
    let mut body = InvocationInputBody::default();
    body.set_string("path", "docs");
    let resolved = resolve_mrt_dispatch_invocation_by_name("mrt_kb_index", &body)
        .expect("kb index should resolve with explicit path");

    assert_eq!(
        resolved.args,
        vec!["run", "-p", "mirr-kb-native", "--bin", "mirr-kb-index", "--", "--path", "docs",]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<String>>()
    );
}

#[test]
fn resolve_kb_index_status_invokes_primary_kb_binary() {
    let body = InvocationInputBody::default();
    let resolved = resolve_mrt_dispatch_invocation_by_name("mrt_kb_index_status", &body)
        .expect("kb index status should resolve");

    assert_eq!(
        resolved.args,
        vec!["run", "-p", "mirr-kb-native", "--bin", "mirr-kb-native", "--", "status",]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<String>>()
    );
}

#[test]
fn resolve_kb_brief_supports_grounded_summary_inputs() {
    let mut body = InvocationInputBody::default();
    body.set_string("query", "where is the kb brief tool wired");
    body.set_string("mode", "hybrid");
    body.set_number("limit", 4.0);
    body.set_string("scope", "crates/mirr-mcp-control-plane/src");
    body.set_string("format", "decision");

    let resolved = resolve_mrt_dispatch_invocation_by_name("mrt_kb_brief", &body)
        .expect("kb brief should resolve with briefing inputs");

    assert!(rupu_args_contains(&resolved.args, "brief"));
    assert!(rupu_args_contains(&resolved.args, "--query"));
    assert!(rupu_args_contains(&resolved.args, "where is the kb brief tool wired"));
    assert!(rupu_args_contains(&resolved.args, "--scope"));
    assert!(rupu_args_contains(&resolved.args, "crates/mirr-mcp-control-plane/src"));
    assert!(rupu_args_contains(&resolved.args, "--format"));
    assert!(rupu_args_contains(&resolved.args, "decision"));
}

fn rupu_args_contains(args: &[String], expected: &str) -> bool {
    args.iter().any(|value| value == expected)
}
