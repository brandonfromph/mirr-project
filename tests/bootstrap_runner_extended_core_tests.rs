//! Extended core tests for `BootstrapRunner`.
//!
//! QA coverage: all public methods, all `BootstrapOpts` flags, every pipeline
//! stage, result consistency invariants, and error-path behaviour.
//!
//! Every test writes its own uniquely-named temp file so tests are safe to run
//! in parallel (`cargo test -- --test-threads N`).

#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::{BootstrapOpts, BootstrapRunner};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn write_src(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, content).expect("write temp src");
    path
}

// Minimal valid MIRR: one in signal, one out signal, one Always property.
const MINIMAL_SRC: &str = "module minimal_core {\n    signal x: in u8;\n    signal y: out bool;\n    property p_x {\n        always (x > 0);\n    }\n}";

// Medium: three in signals, two out, three properties.
const MEDIUM_SRC: &str = "module medium_core {\n    signal pressure: in u8;\n    signal temp: in u8;\n    signal rate: in u16;\n    signal alarm: out bool;\n    signal override_valve: out bool;\n    property p_pressure {\n        always (pressure > 0);\n    }\n    property p_temp {\n        always (temp > 0);\n    }\n    property p_rate {\n        always (rate > 0);\n    }\n}";

// Neonatal respirator: canonical safety-critical example.
const NEONATAL_SRC: &str = "module neonatal_respirator {\n    signal airway_pressure: in u8;\n    signal breath_rate: in u8;\n    signal alarm_out: out bool;\n    signal valve_override: out bool;\n    property p_pressure {\n        always (airway_pressure > 0);\n    }\n    property p_rate {\n        always (breath_rate > 0);\n    }\n}";

// ---------------------------------------------------------------------------
// Stage presence and naming
// ---------------------------------------------------------------------------

// C1 — Parse stage present in result
#[test]
fn c1_parse_stage_present_in_result() {
    let p = write_src("brc_c1.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    let stage = result.stages.iter().find(|s| s.name == "Parse");
    assert!(stage.is_some(), "C1: Parse stage must be present");
}

// C2 — Validate stage present in result
#[test]
fn c2_validate_stage_present_in_result() {
    let p = write_src("brc_c2.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    let stage = result.stages.iter().find(|s| s.name == "Validate");
    assert!(stage.is_some(), "C2: Validate stage must be present");
}

// C3 — TemporalLower stage present in result
#[test]
fn c3_temporal_lower_stage_present() {
    let p = write_src("brc_c3.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    let stage = result.stages.iter().find(|s| s.name == "TemporalLower");
    assert!(stage.is_some(), "C3: TemporalLower stage must be present");
}

// C4 — every stage has a non-empty name
#[test]
fn c4_all_stage_names_non_empty() {
    let p = write_src("brc_c4.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    for s in &result.stages {
        assert!(!s.name.is_empty(), "C4: stage has empty name");
    }
}

// C5 — every stage has a non-empty message
#[test]
fn c5_all_stage_messages_non_empty() {
    let p = write_src("brc_c5.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    for s in &result.stages {
        assert!(!s.message.is_empty(), "C5: stage '{}' has empty message", s.name);
    }
}

// ---------------------------------------------------------------------------
// Result invariants
// ---------------------------------------------------------------------------

// C6 — valid MIRR returns ok=true
#[test]
fn c6_valid_mirr_returns_ok_true() {
    let p = write_src("brc_c6.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "C6: valid MIRR must return ok=true: {:?}", result.stages);
}

// C7 — source_path in result matches the input path
#[test]
fn c7_source_path_matches_input() {
    let p = write_src("brc_c7.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert_eq!(result.source_path, p, "C7: source_path must match input path");
}

// C8 — result.ok ↔ all stages ok (success case)
#[test]
fn c8_ok_consistent_with_all_stages_succeeding() {
    let p = write_src("brc_c8.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    let all_ok = result.stages.iter().all(|s| s.ok);
    assert_eq!(result.ok, all_ok, "C8: result.ok must equal all stages.ok");
}

// C9 — result.ok=false when any stage fails (failure case)
#[test]
fn c9_ok_false_when_stage_fails() {
    let p = write_src("brc_c9.mirr", "not valid MIRR garbage {{{");
    let result = BootstrapRunner::new_default().run(&p);
    let any_fail = result.stages.iter().any(|s| !s.ok);
    assert!(!result.ok, "C9: ok must be false when a stage fails");
    assert!(any_fail, "C9: at least one stage must be !ok when result is false");
}

// C10 — failed stages carry non-empty error message
#[test]
fn c10_failed_stage_message_non_empty() {
    let p = write_src("brc_c10.mirr", "broken MIRR {{ syntax error");
    let result = BootstrapRunner::new_default().run(&p);
    for s in result.stages.iter().filter(|s| !s.ok) {
        assert!(
            !s.message.is_empty(),
            "C10: failed stage '{}' must have non-empty message",
            s.name
        );
    }
}

// C11 — at least one stage is always present (never empty stage list)
#[test]
fn c11_stage_list_never_empty() {
    let p = write_src("brc_c11.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.stages.is_empty(), "C11: stage list must never be empty");
}

// ---------------------------------------------------------------------------
// summary_line
// ---------------------------------------------------------------------------

// C12 — summary_line contains PASS on success
#[test]
fn c12_summary_line_contains_pass_on_success() {
    let p = write_src("brc_c12.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    let line = result.summary_line();
    assert!(line.contains("PASS"), "C12: summary_line must contain PASS: {line}");
}

// C13 — summary_line contains FAIL on failure
#[test]
fn c13_summary_line_contains_fail_on_error() {
    let p = write_src("brc_c13.mirr", "syntax failure {{}");
    let result = BootstrapRunner::new_default().run(&p);
    let line = result.summary_line();
    assert!(line.contains("FAIL"), "C13: summary_line must contain FAIL: {line}");
}

// C14 — summary_line does not end with a newline
#[test]
fn c14_summary_line_no_trailing_newline() {
    let p = write_src("brc_c14.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.summary_line().ends_with('\n'), "C14: summary must not end with '\\n'");
}

// C15 — summary_line is non-empty
#[test]
fn c15_summary_line_non_empty() {
    let p = write_src("brc_c15.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.summary_line().is_empty(), "C15: summary_line must not be empty");
}

// ---------------------------------------------------------------------------
// print_report
// ---------------------------------------------------------------------------

// C16 — print_report does not panic on success
#[test]
fn c16_print_report_no_panic_success() {
    let p = write_src("brc_c16.mirr", MINIMAL_SRC);
    BootstrapRunner::new_default().run(&p).print_report();
}

// C17 — print_report does not panic on failure
#[test]
fn c17_print_report_no_panic_failure() {
    let p = write_src("brc_c17.mirr", "complete garbage {{ }}");
    BootstrapRunner::new_default().run(&p).print_report();
}

// ---------------------------------------------------------------------------
// BootstrapOpts: emit flags
// ---------------------------------------------------------------------------

// C18 — emit_netlist_json populates netlist_json
#[test]
fn c18_emit_json_opt_populates_field() {
    let opts = BootstrapOpts { emit_netlist_json: true, ..Default::default() };
    let p = write_src("brc_c18.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(result.netlist_json.is_some(), "C18: netlist_json must be Some when opt enabled");
}

// C19 — emitted JSON contains "ir_version"
#[test]
fn c19_emitted_json_contains_ir_version() {
    let opts = BootstrapOpts { emit_netlist_json: true, ..Default::default() };
    let p = write_src("brc_c19.mirr", MINIMAL_SRC);
    let json = BootstrapRunner::new(opts).run(&p).netlist_json.expect("must be Some");
    assert!(json.contains("ir_version"), "C19: JSON must contain 'ir_version'");
}

// C20 — emit_netlist_json disabled yields None
#[test]
fn c20_emit_json_disabled_yields_none() {
    let p = write_src("brc_c20.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.netlist_json.is_none(), "C20: netlist_json must be None when opt disabled");
}

// C21 — emit_netlist_verilog populates netlist_verilog
#[test]
fn c21_emit_verilog_opt_populates_field() {
    let opts = BootstrapOpts { emit_netlist_verilog: true, ..Default::default() };
    let p = write_src("brc_c21.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(result.netlist_verilog.is_some(), "C21: netlist_verilog must be Some when opt enabled");
}

// C22 — emitted Verilog contains "module"
#[test]
fn c22_emitted_verilog_contains_module_keyword() {
    let opts = BootstrapOpts { emit_netlist_verilog: true, ..Default::default() };
    let p = write_src("brc_c22.mirr", MINIMAL_SRC);
    let sv = BootstrapRunner::new(opts).run(&p).netlist_verilog.expect("must be Some");
    assert!(sv.contains("module"), "C22: Verilog output must contain 'module'");
}

// C23 — emit_netlist_verilog disabled yields None
#[test]
fn c23_emit_verilog_disabled_yields_none() {
    let p = write_src("brc_c23.mirr", MINIMAL_SRC);
    assert!(
        BootstrapRunner::new_default().run(&p).netlist_verilog.is_none(),
        "C23: netlist_verilog must be None when opt disabled"
    );
}

// ---------------------------------------------------------------------------
// BootstrapOpts: fail_fast
// ---------------------------------------------------------------------------

// C24 — fail_fast returns ok=false for invalid input
#[test]
fn c24_fail_fast_ok_false_on_invalid() {
    let opts = BootstrapOpts { fail_fast: true, ..Default::default() };
    let p = write_src("brc_c24.mirr", "not valid MIRR at all");
    assert!(!BootstrapRunner::new(opts).run(&p).ok, "C24: fail_fast must be ok=false");
}

// C25 — fail_fast reports <= stages than default (stops on first failure)
#[test]
fn c25_fail_fast_stops_early() {
    let p = write_src("brc_c25.mirr", "broken MIRR {{ garbage }}");
    let default_len = BootstrapRunner::new_default().run(&p).stages.len();
    let fast_opts = BootstrapOpts { fail_fast: true, ..Default::default() };
    let fast_len = BootstrapRunner::new(fast_opts).run(&p).stages.len();
    assert!(
        fast_len <= default_len,
        "C25: fail_fast must have <= stages: fast={fast_len} default={default_len}"
    );
}

// C26 — fail_fast + valid input still passes
#[test]
fn c26_fail_fast_passes_valid_input() {
    let opts = BootstrapOpts { fail_fast: true, ..Default::default() };
    let p = write_src("brc_c26.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(result.ok, "C26: fail_fast must pass for valid MIRR: {:?}", result.stages);
}

// ---------------------------------------------------------------------------
// Error paths
// ---------------------------------------------------------------------------

// C27 — missing file fails at Read stage
#[test]
fn c27_missing_file_fails_read_stage() {
    let p = std::path::PathBuf::from("/nonexistent/path/that/does/not/exist_brc.mirr");
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.ok, "C27: missing file must fail");
    let read = result.stages.iter().find(|s| s.name == "Read");
    assert!(read.is_some(), "C27: Read stage must be present");
    assert!(!read.unwrap().ok, "C27: Read stage must fail for missing file");
}

// C28 — invalid syntax fails at Parse stage
#[test]
fn c28_invalid_syntax_fails_parse_stage() {
    let p = write_src("brc_c28.mirr", "this is not MIRR at all @@@ !!!");
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.ok, "C28: invalid syntax must fail");
    let parse = result.stages.iter().find(|s| s.name == "Parse");
    assert!(parse.is_some(), "C28: Parse stage must be present even on failure");
    assert!(!parse.unwrap().ok, "C28: Parse stage must fail on invalid MIRR");
}

// ---------------------------------------------------------------------------
// Diverse valid programs
// ---------------------------------------------------------------------------

// C29 — medium multi-signal program passes all stages
#[test]
fn c29_medium_program_passes_all_stages() {
    let p = write_src("brc_c29.mirr", MEDIUM_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "C29: medium program must pass: {:?}", result.stages);
}

// C30 — neonatal program passes all stages
#[test]
fn c30_neonatal_program_passes_all_stages() {
    let p = write_src("brc_c30.mirr", NEONATAL_SRC);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "C30: neonatal program must pass: {:?}", result.stages);
}

// C31 — runner is reusable: two independent runs produce independent results
#[test]
fn c31_runner_reusable_independent_results() {
    let runner = BootstrapRunner::new_default();
    let p1 = write_src("brc_c31a.mirr", MINIMAL_SRC);
    let p2 = write_src("brc_c31b.mirr", MEDIUM_SRC);
    let r1 = runner.run(&p1);
    let r2 = runner.run(&p2);
    assert!(r1.ok, "C31: first run must pass");
    assert!(r2.ok, "C31: second run must pass");
    assert_ne!(r1.source_path, r2.source_path, "C31: source_paths must differ");
}

// C32 — both emit flags enabled together work
#[test]
fn c32_both_emit_flags_together() {
    let opts =
        BootstrapOpts { emit_netlist_json: true, emit_netlist_verilog: true, ..Default::default() };
    let p = write_src("brc_c32.mirr", MINIMAL_SRC);
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(result.ok, "C32: both emit flags must not break the run");
    assert!(result.netlist_json.is_some(), "C32: netlist_json must be Some");
    assert!(result.netlist_verilog.is_some(), "C32: netlist_verilog must be Some");
}

// C33 — bool-typed signal compiles successfully
#[test]
fn c33_bool_signal_compiles() {
    let src = "module boolsig {\n    signal flag: in bool;\n    signal out_signal: out bool;\n    property p_flag {\n        always (flag > 0);\n    }\n}";
    let p = write_src("brc_c33.mirr", src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "C33: bool signal must compile: {:?}", result.stages);
}

// C34 — u32-typed signal compiles successfully
#[test]
fn c34_u32_signal_compiles() {
    let src = "module u32sig {\n    signal counter: in u32;\n    signal active: out bool;\n    property p_count {\n        always (counter > 0);\n    }\n}";
    let p = write_src("brc_c34.mirr", src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "C34: u32 signal must compile: {:?}", result.stages);
}
