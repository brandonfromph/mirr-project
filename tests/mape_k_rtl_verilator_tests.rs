#![cfg(feature = "legacy_ast")]
#![forbid(unsafe_code)]
#![allow(unused_imports)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn tool_available(name: &str) -> bool {
    let flag = if name == "yosys" || name == "icetime" { "-V" } else { "--version" };
    std::process::Command::new(name).arg(flag).output().map(|o| o.status.success()).unwrap_or(false)
}

fn generate_mape_k_rtl() -> String {
    const MIRR_SRC: &str = "module safety_hw {\n    signal pressure: in u8;\n    signal temp: in u8;\n    signal alarm: out bool;\n\n    property p_pressure {\n        always (pressure > 0);\n    }\n\n    property p_temp {\n        always (temp > 0);\n    }\n}";
    let config = PipelineConfig { mape_k: true, emit_mape_k_rtl: true, ..Default::default() };
    let result = run_pipeline(MIRR_SRC, &config).expect("pipeline should succeed");
    result.mape_k_rtl.expect("RTL should be generated")
}

fn write_temp(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, content).expect("write temp file");
    path
}

fn run_verilator_lint(sv_content: &str, top: &str) -> std::process::Output {
    let sv_path = write_temp(&format!("mirr_vlint_{top}.sv"), sv_content);
    std::process::Command::new("verilator")
        .arg("--lint-only")
        .arg("--sv")
        .arg("--top-module")
        .arg(top)
        .arg(&sv_path)
        .output()
        .expect("verilator lint")
}

// E4.1 — lint mirr_mape_k_top (full hierarchy)
#[test]
fn verilator_lint_full_rtl_top() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_mape_k_top");
    assert!(
        out.status.success(),
        "E4.1: verilator lint failed for mirr_mape_k_top:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E4.2 — lint mirr_monitor
#[test]
fn verilator_lint_monitor() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_monitor");
    assert!(
        out.status.success(),
        "E4.2: verilator lint failed for mirr_monitor:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E4.3 — lint mirr_analyze
#[test]
fn verilator_lint_analyze() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_analyze");
    assert!(
        out.status.success(),
        "E4.3: verilator lint failed for mirr_analyze:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E4.4 — lint mirr_plan
#[test]
fn verilator_lint_plan() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_plan");
    assert!(
        out.status.success(),
        "E4.4: verilator lint failed for mirr_plan:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E4.5 — lint mirr_execute
#[test]
fn verilator_lint_execute() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_execute");
    assert!(
        out.status.success(),
        "E4.5: verilator lint failed for mirr_execute:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E4.6 — lint mirr_knowledge
#[test]
fn verilator_lint_knowledge() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_knowledge");
    assert!(
        out.status.success(),
        "E4.6: verilator lint failed for mirr_knowledge:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E4.7 — verify stderr has no lines containing "%Error:"
#[test]
fn verilator_no_error_lines() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_mape_k_top");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let error_lines: Vec<&str> = stderr.lines().filter(|l| l.contains("%Error:")).collect();
    assert!(
        error_lines.is_empty(),
        "E4.7: verilator reported %Error: lines:\n{}",
        error_lines.join("\n")
    );
}

// E4.8 — compile with --language 1800-2012 (SV standard flag)
#[test]
fn verilator_sv_keyword_support() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_vlint_sv2012.sv", &rtl);
    // Try --language 1800-2012; fall back gracefully if flag unsupported.
    let out = std::process::Command::new("verilator")
        .arg("--lint-only")
        .arg("--sv")
        .arg("--language")
        .arg("1800-2012")
        .arg("--top-module")
        .arg("mirr_mape_k_top")
        .arg(&sv_path)
        .output()
        .expect("verilator sv2012");
    // Accept either success or a flag-unknown error (older verilator).
    let stderr = String::from_utf8_lossy(&out.stderr);
    let flag_unknown = stderr.contains("Unknown argument") || stderr.contains("--language");
    assert!(
        out.status.success() || flag_unknown,
        "E4.8: verilator --language 1800-2012 failed unexpectedly:\n{}",
        stderr
    );
}

// E4.9 — exit status is 0 or 1 only (not crash / signal)
#[test]
fn verilator_lint_returns_valid_exit() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_mape_k_top");
    let code = out.status.code().unwrap_or(99);
    assert!(
        code == 0 || code == 1,
        "E4.9: verilator exited with unexpected code {} (expected 0 or 1)",
        code
    );
}

// E4.10 — lint mirr_mape_k_top has no unresolved references
#[test]
fn verilator_full_rtl_has_no_unresolved_references() {
    if !tool_available("verilator") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let out = run_verilator_lint(&rtl, "mirr_mape_k_top");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("Can't find"),
        "E4.10: verilator reports unresolved references:\n{}",
        stderr
    );
}
