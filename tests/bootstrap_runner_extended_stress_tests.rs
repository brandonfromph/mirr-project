//! Extended stress tests for `BootstrapRunner`.
//!
//! QA coverage: boundary inputs, large programs, repeated execution, all
//! `BootstrapOpts` flag combinations, signal-type variety, and robustness
//! under degenerate inputs.

#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::{BootstrapOpts, BootstrapRunner};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn write_src(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, content).expect("write temp src");
    path
}

fn one_sig_module(name: &str, sig: &str, ty: &str) -> String {
    format!(
        "module {name} {{\n    signal {sig}: in {ty};\n    signal out_{sig}: out bool;\n    property p_{sig} {{\n        always ({sig} > 0);\n    }}\n}}"
    )
}

// ---------------------------------------------------------------------------
// Degenerate / minimal boundary inputs
// ---------------------------------------------------------------------------

// S1 — empty file is rejected gracefully (no panic)
#[test]
fn s1_empty_file_rejected_gracefully() {
    let p = write_src("brs_s1.mirr", "");
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.ok, "S1: empty source must fail");
    assert!(!result.stages.is_empty(), "S1: must still produce at least one stage");
}

// S2 — whitespace-only file is rejected gracefully
#[test]
fn s2_whitespace_only_file_rejected() {
    let p = write_src("brs_s2.mirr", "   \n\t\n   ");
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.ok, "S2: whitespace-only source must fail");
}

// S3 — single-char garbage file is rejected
#[test]
fn s3_single_char_garbage_rejected() {
    let p = write_src("brs_s3.mirr", "x");
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.ok, "S3: single-char garbage must fail");
}

// S4 — module with only a name (no body) is rejected
#[test]
fn s4_module_no_body_rejected() {
    let p = write_src("brs_s4.mirr", "module empty_module");
    let result = BootstrapRunner::new_default().run(&p);
    assert!(!result.ok, "S4: module without body must fail");
}

// S5 — empty braces module is rejected or passes depending on validation rules
#[test]
fn s5_empty_braces_module_does_not_panic() {
    let p = write_src("brs_s5.mirr", "module empty_braces {}");
    // Whether this passes or fails is an implementation detail; what MUST hold is:
    // no panic, stages are non-empty, ok is consistent with stages.
    let result = BootstrapRunner::new_default().run(&p);
    let all_ok = result.stages.iter().all(|s| s.ok);
    assert_eq!(result.ok, all_ok, "S5: ok must be consistent with stages");
    assert!(!result.stages.is_empty(), "S5: must produce at least one stage");
}

// ---------------------------------------------------------------------------
// Signal type variety
// ---------------------------------------------------------------------------

// S6 — u8 signal compiles
#[test]
fn s6_u8_signal_compiles() {
    let src = one_sig_module("m_u8", "val_u8", "u8");
    let p = write_src("brs_s6.mirr", &src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S6: u8 signal must compile: {:?}", result.stages);
}

// S7 — u16 signal compiles
#[test]
fn s7_u16_signal_compiles() {
    let src = one_sig_module("m_u16", "val_u16", "u16");
    let p = write_src("brs_s7.mirr", &src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S7: u16 signal must compile: {:?}", result.stages);
}

// S8 — u32 signal compiles
#[test]
fn s8_u32_signal_compiles() {
    let src = one_sig_module("m_u32", "val_u32", "u32");
    let p = write_src("brs_s8.mirr", &src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S8: u32 signal must compile: {:?}", result.stages);
}

// S9 — bool signal compiles
#[test]
fn s9_bool_signal_compiles() {
    let src = one_sig_module("m_bool", "flag_bool", "bool");
    let p = write_src("brs_s9.mirr", &src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S9: bool signal must compile: {:?}", result.stages);
}

// ---------------------------------------------------------------------------
// Scale: many signals and properties
// ---------------------------------------------------------------------------

// S10 — module with 10 in-signals and 5 properties compiles
#[test]
fn s10_ten_signals_five_properties_compiles() {
    let mut src = String::from("module wide_module {\n");
    for i in 0..10 {
        src.push_str(&format!("    signal s{i}: in u8;\n"));
    }
    src.push_str("    signal out_alarm: out bool;\n");
    for i in 0..5 {
        src.push_str(&format!("    property p{i} {{\n        always (s{i} > 0);\n    }}\n"));
    }
    src.push('}');
    let p = write_src("brs_s10.mirr", &src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S10: 10-signal 5-property module must compile: {:?}", result.stages);
}

// S11 — module with 20 in-signals compiles
#[test]
fn s11_twenty_signals_compiles() {
    let mut src = String::from("module twenty_sig {\n");
    for i in 0..20 {
        src.push_str(&format!("    signal s{i}: in u8;\n"));
    }
    src.push_str("    signal alarm: out bool;\n");
    src.push_str("    property p0 {\n        always (s0 > 0);\n    }\n");
    src.push('}');
    let p = write_src("brs_s11.mirr", &src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S11: 20-signal module must compile: {:?}", result.stages);
}

// S12 — module with 8 properties compiles
#[test]
fn s12_eight_properties_compiles() {
    let mut src = String::from("module eight_props {\n");
    for i in 0..8 {
        src.push_str(&format!("    signal s{i}: in u8;\n"));
    }
    src.push_str("    signal alarm: out bool;\n");
    for i in 0..8 {
        src.push_str(&format!("    property p{i} {{\n        always (s{i} > 0);\n    }}\n"));
    }
    src.push('}');
    let p = write_src("brs_s12.mirr", &src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S12: 8-property module must compile: {:?}", result.stages);
}

// ---------------------------------------------------------------------------
// Repeated execution
// ---------------------------------------------------------------------------

// S13 — same runner used 5 times returns consistent results
#[test]
fn s13_five_consecutive_runs_consistent() {
    const SRC: &str = "module repeatme {\n    signal x: in u8;\n    signal y: out bool;\n    property p {\n        always (x > 0);\n    }\n}";
    let runner = BootstrapRunner::new_default();
    for i in 0..5 {
        let p = write_src(&format!("brs_s13_{i}.mirr"), SRC);
        let result = runner.run(&p);
        assert!(result.ok, "S13: run {i} must pass: {:?}", result.stages);
    }
}

// S14 — alternating pass/fail runs are independent
#[test]
fn s14_alternating_pass_fail_independent() {
    const GOOD: &str = "module good_alt {\n    signal x: in u8;\n    signal y: out bool;\n    property p {\n        always (x > 0);\n    }\n}";
    const BAD: &str = "complete garbage @@!!##";
    let runner = BootstrapRunner::new_default();
    for i in 0..4 {
        let (src, expect_ok) = if i % 2 == 0 { (GOOD, true) } else { (BAD, false) };
        let p = write_src(&format!("brs_s14_{i}.mirr"), src);
        let result = runner.run(&p);
        assert_eq!(
            result.ok, expect_ok,
            "S14: run {i} expected ok={expect_ok}: {:?}",
            result.stages
        );
    }
}

// ---------------------------------------------------------------------------
// BootstrapOpts combinations
// ---------------------------------------------------------------------------

// S15 — all opts disabled (explicit Default) behaves like new_default()
#[test]
fn s15_all_opts_disabled_same_as_default() {
    const SRC: &str = "module opts_compare {\n    signal x: in u8;\n    signal y: out bool;\n    property p {\n        always (x > 0);\n    }\n}";
    let p_a = write_src("brs_s15a.mirr", SRC);
    let p_b = write_src("brs_s15b.mirr", SRC);
    let r_default = BootstrapRunner::new_default().run(&p_a);
    let r_explicit = BootstrapRunner::new(BootstrapOpts::default()).run(&p_b);
    assert_eq!(r_default.ok, r_explicit.ok, "S15: default and explicit-default must match");
    assert_eq!(r_default.stages.len(), r_explicit.stages.len(), "S15: stage count must match");
}

// S16 — emit_json + fail_fast combination: passes valid input, populates JSON
#[test]
fn s16_emit_json_and_fail_fast_valid_input() {
    let opts = BootstrapOpts { emit_netlist_json: true, fail_fast: true, ..Default::default() };
    let src = "module combo_16 {\n    signal x: in u8;\n    signal y: out bool;\n    property p {\n        always (x > 0);\n    }\n}";
    let p = write_src("brs_s16.mirr", src);
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(result.ok, "S16: emit_json+fail_fast must pass for valid input");
    assert!(result.netlist_json.is_some(), "S16: JSON must be populated");
}

// S17 — emit_verilog + fail_fast combination: passes valid input, populates SV
#[test]
fn s17_emit_verilog_and_fail_fast_valid_input() {
    let opts = BootstrapOpts { emit_netlist_verilog: true, fail_fast: true, ..Default::default() };
    let src = "module combo_17 {\n    signal x: in u8;\n    signal y: out bool;\n    property p {\n        always (x > 0);\n    }\n}";
    let p = write_src("brs_s17.mirr", src);
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(result.ok, "S17: emit_verilog+fail_fast must pass for valid input");
    assert!(result.netlist_verilog.is_some(), "S17: Verilog must be populated");
}

// S18 — all three emit+fail_fast flags: all outputs present on valid input
#[test]
fn s18_all_output_flags_combined() {
    let opts = BootstrapOpts {
        emit_netlist_json: true,
        emit_netlist_verilog: true,
        fail_fast: true,
        ..Default::default()
    };
    let src = "module combo_18 {\n    signal x: in u8;\n    signal y: out bool;\n    property p {\n        always (x > 0);\n    }\n}";
    let p = write_src("brs_s18.mirr", src);
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(result.ok, "S18: all-flags must pass valid input: {:?}", result.stages);
    assert!(result.netlist_json.is_some(), "S18: JSON field must be Some");
    assert!(result.netlist_verilog.is_some(), "S18: Verilog field must be Some");
}

// S19 — fail_fast on bad src: both emit fields stay None (no partial output)
#[test]
fn s19_fail_fast_bad_src_no_partial_output() {
    let opts = BootstrapOpts {
        emit_netlist_json: true,
        emit_netlist_verilog: true,
        fail_fast: true,
        ..Default::default()
    };
    let p = write_src("brs_s19.mirr", "garbage input for failure test @@#!");
    let result = BootstrapRunner::new(opts).run(&p);
    assert!(!result.ok, "S19: bad input must fail");
    // Outputs should not be populated when pipeline aborts early.
    assert!(result.netlist_json.is_none(), "S19: JSON must be None on pipeline failure");
    assert!(result.netlist_verilog.is_none(), "S19: Verilog must be None on pipeline failure");
}

// ---------------------------------------------------------------------------
// Mixed in/out topology
// ---------------------------------------------------------------------------

// S20 — module with multiple in AND out signals of mixed types
#[test]
fn s20_mixed_in_out_signals_compile() {
    let src = "module mixed_io {\n    signal pressure: in u8;\n    signal temp: in u16;\n    signal enabled: in bool;\n    signal alarm: out bool;\n    signal status: out u8;\n    property p_p {\n        always (pressure > 0);\n    }\n    property p_t {\n        always (temp > 0);\n    }\n}";
    let p = write_src("brs_s20.mirr", src);
    let result = BootstrapRunner::new_default().run(&p);
    assert!(result.ok, "S20: mixed in/out module must compile: {:?}", result.stages);
}
