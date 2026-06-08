#![forbid(unsafe_code)]

use crate::tooling::MrtDispatchTool;

use super::mrt_dispatch_invocation_input::{
    get_body_string, get_body_string_array, InvocationInputBody,
};
use super::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;

const KB_ROOT: &str = ".kb-data";
const MAX_WAVE_LINES: i64 = 128;
const DEFAULT_COMPILE_MAX_SIZE: i64 = 10 * 1024 * 1024;
const MAX_LSP_SOURCE_BYTES: usize = 1_048_576;
const MAX_RSPU_TIMEOUT_MS: i64 = 300_000;
const MAX_LRA_PROJECT_NAME_BYTES: usize = 64;
const MIN_PROPOSAL_ID_DIGITS: usize = 3;

fn first_string(body: &InvocationInputBody, keys: &[&str], fallback: &str) -> String {
    for key in keys {
        let value = get_body_string(body, key, "");
        if !value.is_empty() {
            return value;
        }
    }
    fallback.to_owned()
}

fn first_number(body: &InvocationInputBody, keys: &[&str], fallback: f64) -> f64 {
    for key in keys {
        if let Some(raw) = body.get(key) {
            match raw {
                super::mrt_dispatch_invocation_input::InvocationInputValue::Number(value)
                    if value.is_finite() =>
                {
                    return *value;
                }
                super::mrt_dispatch_invocation_input::InvocationInputValue::String(value) => {
                    if let Ok(parsed) = value.parse::<f64>() {
                        if parsed.is_finite() {
                            return parsed;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fallback
}

fn first_string_array_or_csv(body: &InvocationInputBody, keys: &[&str]) -> Option<Vec<String>> {
    for key in keys {
        if let Some(values) = get_body_string_array(body, key) {
            return Some(values);
        }
    }

    for key in keys {
        let csv = get_body_string(body, key, "");
        if !csv.is_empty() {
            let values = csv
                .split(',')
                .map(str::trim)
                .filter(|entry| !entry.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>();
            return Some(values);
        }
    }

    None
}

fn frame_json_rpc_message(payload: &str) -> String {
    format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload)
}

fn lsp_diagnostics_stdin(source: &str) -> Result<String, String> {
    if source.is_empty() {
        return Err("missing_source".to_owned());
    }
    if source.len() > MAX_LSP_SOURCE_BYTES {
        return Err("lsp_source_too_large".to_owned());
    }

    let initialize = frame_json_rpc_message(
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"processId\":null,\"rootUri\":null,\"capabilities\":{},\"workspaceFolders\":null}}",
    );

    let source_json = serde_json::to_string(source)
        .map_err(|error| format!("invalid_lsp_source_json: {}", error))?;

    let did_open_payload = format!(
        "{{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/didOpen\",\"params\":{{\"textDocument\":{{\"uri\":\"file:///mrt-input.mirr\",\"languageId\":\"mirr\",\"version\":1,\"text\":{}}}}}}}",
        source_json
    );
    let did_open = frame_json_rpc_message(&did_open_payload);

    let shutdown = frame_json_rpc_message(
        "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"shutdown\",\"params\":null}",
    );
    let exit = frame_json_rpc_message("{\"jsonrpc\":\"2.0\",\"method\":\"exit\"}");

    Ok(format!("{}{}{}{}", initialize, did_open, shutdown, exit))
}

fn require_proposal_id(proposal_id: &str) -> Result<String, String> {
    if proposal_id.is_empty() {
        return Err("missing_proposal_id".to_owned());
    }

    if proposal_id.len() < MIN_PROPOSAL_ID_DIGITS
        || !proposal_id.chars().all(|ch| ch.is_ascii_digit())
    {
        return Err("invalid_proposal_id".to_owned());
    }

    Ok(proposal_id.to_owned())
}

fn normalize_slash_runs(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut in_slash_run = false;

    for ch in value.chars() {
        if ch == '/' {
            if !in_slash_run {
                output.push('/');
            }
            in_slash_run = true;
            continue;
        }

        in_slash_run = false;
        output.push(ch);
    }

    output
}

fn require_proposal_file(proposal_file: &str) -> Result<String, String> {
    if proposal_file.is_empty() {
        return Err("missing_proposal_file".to_owned());
    }

    let forward_slashes = proposal_file.replace('\\', "/");
    let mut normalized = normalize_slash_runs(&forward_slashes);
    if normalized.starts_with("./") {
        normalized = normalized[2..].to_owned();
    }

    if !normalized.starts_with("proposals/") {
        return Err("invalid_proposal_file".to_owned());
    }
    if !normalized.ends_with(".md") {
        return Err("invalid_proposal_file".to_owned());
    }
    if normalized.contains("../") {
        return Err("invalid_proposal_file".to_owned());
    }

    Ok(normalized)
}

fn normalize_max_lines(value: f64) -> Result<i64, String> {
    if !value.is_finite() {
        return Err("invalid_max_lines".to_owned());
    }

    let lines = (value.trunc() as i64).clamp(1, MAX_WAVE_LINES);

    Ok(lines)
}

fn normalize_path(value: &str) -> String {
    value.replace('\\', "/")
}

fn wave_args(
    proposal_id: &str,
    proposal_file: &str,
    max_lines: f64,
    dry_run: bool,
) -> Result<Vec<String>, String> {
    let id = require_proposal_id(proposal_id)?;
    let file = require_proposal_file(proposal_file)?;
    let lines = normalize_max_lines(max_lines)?;

    let mut args = vec![
        "--proposal-id".to_owned(),
        id,
        "--proposal-file".to_owned(),
        file,
        "--max-lines".to_owned(),
        lines.to_string(),
    ];

    if dry_run {
        args.push("--dry-run".to_owned());
    }

    Ok(args)
}

fn brain_get_args(key: &str) -> Result<Vec<String>, String> {
    if key.is_empty() {
        return Err("missing_key".to_owned());
    }

    Ok(vec![
        "--kb-root".to_owned(),
        KB_ROOT.to_owned(),
        "--format".to_owned(),
        "json".to_owned(),
        "get".to_owned(),
        "--key".to_owned(),
        key.to_owned(),
    ])
}

fn general_ci_compile_args() -> Vec<String> {
    vec![
        "ci".to_owned(),
        "--profile".to_owned(),
        "compile".to_owned(),
        "--format".to_owned(),
        "json".to_owned(),
    ]
}

fn general_ci_fast_args() -> Vec<String> {
    vec![
        "ci".to_owned(),
        "--profile".to_owned(),
        "fast".to_owned(),
        "--format".to_owned(),
        "json".to_owned(),
    ]
}

fn lra_init_args(project_name: &str) -> Result<Vec<String>, String> {
    if project_name.is_empty() {
        return Err("missing_project_name".to_owned());
    }

    if project_name.len() > MAX_LRA_PROJECT_NAME_BYTES {
        return Err("invalid_project_name".to_owned());
    }

    if project_name == "." || project_name == ".." {
        return Err("invalid_project_name".to_owned());
    }

    if project_name.contains('/') || project_name.contains('\\') || project_name.contains(':') {
        return Err("invalid_project_name".to_owned());
    }

    if project_name.starts_with('.') {
        return Err("invalid_project_name".to_owned());
    }

    if !project_name.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_') {
        return Err("invalid_project_name".to_owned());
    }

    Ok(vec!["init".to_owned(), project_name.to_owned()])
}

fn lra_validate_args(path: &str) -> Result<Vec<String>, String> {
    if path.is_empty() {
        return Err("missing_path".to_owned());
    }
    Ok(vec!["validate".to_owned(), normalize_path(path)])
}

fn lra_serve_args(port: f64) -> Vec<String> {
    let mut args = vec!["serve".to_owned()];
    if port.is_finite() {
        let p = port.trunc() as i64;
        if (1024..=65535).contains(&p) {
            args.push("--port".to_owned());
            args.push(p.to_string());
        }
    }
    args
}

fn lra_check_args(path: &str) -> Result<Vec<String>, String> {
    lra_validate_args(path)
}

fn lra_sign_args(receipt: &str, key_path: &str) -> Result<Vec<String>, String> {
    if receipt.is_empty() {
        return Err("missing_receipt".to_owned());
    }
    if key_path.is_empty() {
        return Err("missing_key_path".to_owned());
    }

    Ok(vec![
        "sign".to_owned(),
        normalize_path(receipt),
        "--key".to_owned(),
        normalize_path(key_path),
    ])
}

fn lra_verify_args(target: &str) -> Result<Vec<String>, String> {
    if target.is_empty() {
        return Err("missing_target".to_owned());
    }
    Ok(vec!["verify".to_owned(), normalize_path(target)])
}

fn mrt_compile_args(
    source_file: &str,
    target: Option<&str>,
    max_size: f64,
) -> Result<Vec<String>, String> {
    if source_file.is_empty() {
        return Err("missing_source_file".to_owned());
    }

    let compile_target = target.filter(|value| !value.is_empty()).unwrap_or("verilog");
    let size =
        if max_size.is_finite() { max_size.trunc() as i64 } else { DEFAULT_COMPILE_MAX_SIZE };

    Ok(vec![
        "run".to_owned(),
        "--bin".to_owned(),
        "mirr-general".to_owned(),
        "--".to_owned(),
        "ci".to_owned(),
        "compile".to_owned(),
        "--source".to_owned(),
        normalize_path(source_file),
        "--target".to_owned(),
        compile_target.to_owned(),
        "--max-size".to_owned(),
        size.max(1).to_string(),
    ])
}

fn mrt_rspu_validate_args(proof_path: &str, mode: &str) -> Result<Vec<String>, String> {
    if proof_path.is_empty() {
        return Err("missing_proof_path".to_owned());
    }

    let normalized_mode = if mode == "permissive" { "permissive" } else { "strict" };

    Ok(vec![
        "run".to_owned(),
        "--test".to_owned(),
        "rwfi2_mrt_contract_tests".to_owned(),
        "--".to_owned(),
        "--proof".to_owned(),
        normalize_path(proof_path),
        "--mode".to_owned(),
        normalized_mode.to_owned(),
        "--timeout".to_owned(),
        MAX_RSPU_TIMEOUT_MS.to_string(),
    ])
}

fn mrt_rspu_proofs_args(
    source_file: &str,
    methods: Option<Vec<String>>,
) -> Result<Vec<String>, String> {
    if source_file.is_empty() {
        return Err("missing_source_file".to_owned());
    }

    let mut args = vec![
        "run".to_owned(),
        "--bin".to_owned(),
        "mirr-wave".to_owned(),
        "--".to_owned(),
        "prove".to_owned(),
        "--source".to_owned(),
        normalize_path(source_file),
    ];

    if let Some(values) = methods {
        let mut normalized = Vec::<String>::new();
        for method in values {
            if !normalized.contains(&method) {
                normalized.push(method);
            }
        }

        if !normalized.is_empty() {
            args.push("--methods".to_owned());
            args.push(normalized.join(","));
        }
    }

    args.push("--timeout".to_owned());
    args.push(MAX_RSPU_TIMEOUT_MS.to_string());

    Ok(args)
}

fn daemon_contract_args(test_target: &str, test_filter: &str) -> Vec<String> {
    let mut args = vec!["test".to_owned(), "--test".to_owned(), test_target.to_owned()];

    if !test_filter.is_empty() {
        args.push(test_filter.to_owned());
    }

    args.push("--".to_owned());
    args.push("--nocapture".to_owned());

    args
}

fn mrt_kb_index_args(path: &str) -> Vec<String> {
    vec![
        "run".to_owned(),
        "-p".to_owned(),
        "mirrc-kb".to_owned(),
        "--bin".to_owned(),
        "mirr-kb-index".to_owned(),
        "--".to_owned(),
        "--path".to_owned(),
        normalize_path(path),
    ]
}

fn mrt_kb_brief_args(
    query: &str,
    mode: &str,
    limit: i64,
    scope: &str,
    format: &str,
) -> Vec<String> {
    let mut args = vec![
        "run".to_owned(),
        "-p".to_owned(),
        "mirrc-kb".to_owned(),
        "--bin".to_owned(),
        "mirrc-kb".to_owned(),
        "--".to_owned(),
        "brief".to_owned(),
        "--query".to_owned(),
        query.to_owned(),
        "--mode".to_owned(),
        mode.to_owned(),
        "--limit".to_owned(),
        limit.to_string(),
        "--format".to_owned(),
        format.to_owned(),
    ];
    if !scope.is_empty() {
        args.push("--scope".to_owned());
        args.push(normalize_path(scope));
    }
    args
}

pub fn resolve_mrt_dispatch_invocation(
    tool: MrtDispatchTool,
    body: &InvocationInputBody,
) -> Result<MrtDispatchInvocationPlan, String> {
    match tool {
        MrtDispatchTool::MrtAudit => {
            let mode = get_body_string(body, "mode", "workspace");
            let glob = get_body_string(body, "glob", "src/**/*.rs");
            Ok(MrtDispatchInvocationPlan::new(vec![
                "run".to_owned(),
                "--bin".to_owned(),
                "mirr-audit".to_owned(),
                "--".to_owned(),
                "--mode".to_owned(),
                mode,
                "--glob".to_owned(),
                glob,
                "--format".to_owned(),
                "json".to_owned(),
            ]))
        }
        MrtDispatchTool::MrtBrainGet => {
            let key = get_body_string(body, "key", "");
            let mut args = vec![
                "run".to_owned(),
                "--bin".to_owned(),
                "mirr-brain".to_owned(),
                "--".to_owned(),
            ];
            args.extend(brain_get_args(&key)?);
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::MrtGeneralCi => Ok(MrtDispatchInvocationPlan::new(vec![
            "run".to_owned(),
            "--bin".to_owned(),
            "mirr-general".to_owned(),
            "--".to_owned(),
            "ci".to_owned(),
            "--format".to_owned(),
            "json".to_owned(),
        ])),
        MrtDispatchTool::MrtGeneralCiCompile => {
            let mut args = vec![
                "run".to_owned(),
                "--bin".to_owned(),
                "mirr-general".to_owned(),
                "--".to_owned(),
            ];
            args.extend(general_ci_compile_args());
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::MrtGeneralCiFast => {
            let mut args = vec![
                "run".to_owned(),
                "--bin".to_owned(),
                "mirr-general".to_owned(),
                "--".to_owned(),
            ];
            args.extend(general_ci_fast_args());
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::MrtWaveDryRun => {
            let proposal_id = first_string(body, &["proposal_id", "proposalId"], "");
            let proposal_file = first_string(body, &["proposal_file", "proposalFile"], "");
            let max_lines = first_number(body, &["max_lines", "maxLines"], MAX_WAVE_LINES as f64);

            let mut args =
                vec!["run".to_owned(), "--bin".to_owned(), "mirr-wave".to_owned(), "--".to_owned()];
            args.extend(wave_args(&proposal_id, &proposal_file, max_lines, true)?);
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::MrtWaveApply => {
            let proposal_id = first_string(body, &["proposal_id", "proposalId"], "");
            let proposal_file = first_string(body, &["proposal_file", "proposalFile"], "");
            let max_lines = first_number(body, &["max_lines", "maxLines"], MAX_WAVE_LINES as f64);

            let mut args =
                vec!["run".to_owned(), "--bin".to_owned(), "mirr-wave".to_owned(), "--".to_owned()];
            args.extend(wave_args(&proposal_id, &proposal_file, max_lines, false)?);
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::MrtLspDiagnostics => {
            let source = first_string(body, &["source", "source_text", "sourceText", "text"], "");
            let stdin_data = lsp_diagnostics_stdin(&source)?;
            Ok(MrtDispatchInvocationPlan::with_stdin(
                vec!["run".to_owned(), "--bin".to_owned(), "mirr-lsp".to_owned(), "--".to_owned()],
                stdin_data,
            ))
        }
        MrtDispatchTool::MrtCompile => {
            let source_file = first_string(body, &["source_file", "sourceFile"], "");
            let target = get_body_string(body, "target", "");
            let max_size =
                first_number(body, &["max_size", "maxSize"], DEFAULT_COMPILE_MAX_SIZE as f64);
            Ok(MrtDispatchInvocationPlan::new(mrt_compile_args(
                &source_file,
                if target.is_empty() { None } else { Some(target.as_str()) },
                max_size,
            )?))
        }
        MrtDispatchTool::MrtRspuValidate => {
            let proof_path = first_string(body, &["proof_path", "proofPath", "path", "proof"], "");
            let mode = first_string(body, &["mode", "validation_mode", "validationMode"], "strict");
            Ok(MrtDispatchInvocationPlan::new(mrt_rspu_validate_args(&proof_path, &mode)?))
        }
        MrtDispatchTool::MrtRspuProofs => {
            let source_file =
                first_string(body, &["source_file", "sourceFile", "source", "path"], "");
            let methods =
                first_string_array_or_csv(body, &["methods", "proof_methods", "proofMethods"]);
            Ok(MrtDispatchInvocationPlan::new(mrt_rspu_proofs_args(&source_file, methods)?))
        }
        MrtDispatchTool::MrtDaemonCoreContract => {
            let test_filter =
                first_string(body, &["test_filter", "testFilter", "filter", "grep"], "");
            Ok(MrtDispatchInvocationPlan::new(daemon_contract_args(
                "wave5_daemon_core_architecture_tests",
                &test_filter,
            )))
        }
        MrtDispatchTool::MrtDaemonSecurityContract => {
            let test_filter =
                first_string(body, &["test_filter", "testFilter", "filter", "grep"], "");
            Ok(MrtDispatchInvocationPlan::new(daemon_contract_args(
                "wave6_daemon_security_runtime_policy_tests",
                &test_filter,
            )))
        }
        MrtDispatchTool::LraInit => {
            let project_name = first_string(body, &["project_name", "projectName", "name"], "");
            let lra_args = lra_init_args(&project_name)?;
            Ok(MrtDispatchInvocationPlan::new(vec![
                "run".to_owned(),
                "-p".to_owned(),
                "lra-cli".to_owned(),
                "--".to_owned(),
                lra_args[0].clone(),
                lra_args[1].clone(),
            ]))
        }
        MrtDispatchTool::LraValidate => {
            let path = first_string(body, &["path", "target_path", "targetPath"], "index.html");
            let mut args =
                vec!["run".to_owned(), "-p".to_owned(), "lra-cli".to_owned(), "--".to_owned()];
            args.extend(lra_validate_args(&path)?);
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::LraServe => {
            let port = first_number(body, &["port", "server_port", "serverPort"], 8080.0);
            let mut args =
                vec!["run".to_owned(), "-p".to_owned(), "lra-cli".to_owned(), "--".to_owned()];
            args.extend(lra_serve_args(port));
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::LraCheck => {
            let path = first_string(body, &["path", "target_path", "targetPath"], "index.html");
            let mut args =
                vec!["run".to_owned(), "-p".to_owned(), "lra-cli".to_owned(), "--".to_owned()];
            args.extend(lra_check_args(&path)?);
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::LraSign => {
            let receipt = first_string(body, &["receipt", "receipt_path", "receiptPath"], "");
            let key_path = first_string(body, &["key", "key_path", "keyPath"], "lra-identity.key");
            let mut args =
                vec!["run".to_owned(), "-p".to_owned(), "lra-cli".to_owned(), "--".to_owned()];
            args.extend(lra_sign_args(&receipt, &key_path)?);
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::LraVerify => {
            let target = first_string(body, &["path", "target", "target_path", "targetPath"], "");
            let mut args =
                vec!["run".to_owned(), "-p".to_owned(), "lra-cli".to_owned(), "--".to_owned()];
            args.extend(lra_verify_args(&target)?);
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::MrtKbQuery => {
            let query = get_body_string(body, "query", "");
            if query.is_empty() {
                return Err("query parameter is required".to_owned());
            }
            let mode = first_string(body, &["mode"], "hybrid");
            if !matches!(mode.as_str(), "lexical" | "semantic" | "hybrid" | "graph" | "temporal") {
                return Err("mode must be one of lexical|semantic|hybrid|graph|temporal".to_owned());
            }
            let limit = first_number(body, &["limit"], 16.0) as i64;
            if !(1..=1000).contains(&limit) {
                return Err("limit must be between 1 and 1000".to_owned());
            }
            let filter = first_string(body, &["filter"], "");
            let expand_mode = first_string(body, &["expand_mode", "expandMode"], "none");
            if !matches!(expand_mode.as_str(), "none" | "synonym" | "hyde") {
                return Err("expand_mode must be one of none|synonym|hyde".to_owned());
            }
            let retry_count = first_number(body, &["retry_count", "retryCount"], 1.0) as i64;
            if !(0..=5).contains(&retry_count) {
                return Err("retry_count must be between 0 and 5".to_owned());
            }
            let timeout_ms = first_number(body, &["timeout_ms", "timeoutMs"], 30_000.0) as i64;
            if !(1_000..=60_000).contains(&timeout_ms) {
                return Err("timeout_ms must be between 1000 and 60000".to_owned());
            }

            let mut args = vec![
                "run".to_owned(),
                "-p".to_owned(),
                "mirrc-kb".to_owned(),
                "--bin".to_owned(),
                "mirrc-kb".to_owned(),
                "--".to_owned(),
                "query".to_owned(),
                "--text".to_owned(),
                query,
                "--mode".to_owned(),
                mode,
                "--limit".to_owned(),
                limit.to_string(),
                "--expand-mode".to_owned(),
                expand_mode,
                "--retry-count".to_owned(),
                retry_count.to_string(),
                "--timeout-ms".to_owned(),
                timeout_ms.to_string(),
            ];
            if !filter.is_empty() {
                args.push("--filter".to_owned());
                args.push(filter);
            }
            Ok(MrtDispatchInvocationPlan::new(args))
        }
        MrtDispatchTool::MrtKbIndex => {
            let path = first_string(body, &["path"], ".");
            Ok(MrtDispatchInvocationPlan::new(mrt_kb_index_args(&path)))
        }
        MrtDispatchTool::MrtKbIndexStatus => Ok(MrtDispatchInvocationPlan::new(vec![
            "run".to_owned(),
            "-p".to_owned(),
            "mirrc-kb".to_owned(),
            "--bin".to_owned(),
            "mirrc-kb".to_owned(),
            "--".to_owned(),
            "status".to_owned(),
        ])),
        MrtDispatchTool::MrtKbBrief => {
            let query = get_body_string(body, "query", "");
            if query.is_empty() {
                return Err("query parameter is required".to_owned());
            }
            let mode = first_string(body, &["mode"], "hybrid");
            if !matches!(mode.as_str(), "lexical" | "semantic" | "hybrid" | "graph" | "temporal") {
                return Err("mode must be one of lexical|semantic|hybrid|graph|temporal".to_owned());
            }
            let limit = first_number(body, &["limit"], 8.0) as i64;
            if !(1..=20).contains(&limit) {
                return Err("limit must be between 1 and 20".to_owned());
            }
            let scope = first_string(body, &["scope"], "");
            let format = first_string(body, &["format"], "brief");
            if !matches!(format.as_str(), "brief" | "bullet" | "decision") {
                return Err("format must be one of brief|bullet|decision".to_owned());
            }
            Ok(MrtDispatchInvocationPlan::new(mrt_kb_brief_args(
                &query, &mode, limit, &scope, &format,
            )))
        }
        MrtDispatchTool::Dynamic(bin_name) => {
            let mut args =
                vec!["run".to_owned(), "--bin".to_owned(), bin_name.clone(), "--".to_owned()];

            // AI-Native Dynamic Dispatch!
            // Map JSON body directly to CLI flags based on discovered metadata.
            if let Some(method) = crate::tooling::discovery_method_by_name(&bin_name) {
                for param in method.parameters {
                    if let Some(val) = body.get(param.name) {
                        // If it can be interpreted as a boolean
                        if let Some(b) = val.as_bool() {
                            if b {
                                args.push(format!("--{}", param.name.replace('_', "-")));
                            }
                        } else {
                            args.push(format!("--{}", param.name.replace('_', "-")));
                            args.push(val.to_string().trim_matches('"').to_owned());
                        }
                    }
                }
            }
            Ok(MrtDispatchInvocationPlan::new(args))
        }
    }
}

pub fn resolve_mrt_dispatch_invocation_by_name(
    tool_name: &str,
    body: &InvocationInputBody,
) -> Result<MrtDispatchInvocationPlan, String> {
    let Ok(tool) = tool_name.parse::<MrtDispatchTool>() else {
        return Err(format!("MCP unknown method rejected: {}.", tool_name));
    };

    resolve_mrt_dispatch_invocation(tool, body)
}
