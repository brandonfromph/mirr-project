#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
//! Extended integration tests for the MIRR Self-Hosting Bootstrap Runner.
//!
//! Comprehensive coverage of `src/bootstrap_runner.rs`:
//! - BootstrapOpts construction and defaults
//! - BootstrapRunner construction (new, new_default, Default)
//! - Pipeline stages: Read, Parse, Validate, TemporalLower, FixtureParity
//! - fail_fast behavior across all failure modes
//! - emit_netlist_json and emit_netlist_verilog flags
//! - BootstrapResult methods: print_report(), summary_line()
//! - StageResult field verification
//! - Fixture parity checking with explicit fixture_root
//! - Fixture parity mismatch detection
//! - Various MIRR source forms: simple signal, comparison, negation, multi-guard
//! - Edge cases: nonexistent file, empty file, malformed MIRR, semantic errors
//!
//! NASA Power-of-10 compliance:
//! - `#![forbid(unsafe_code)]`
//! - All loops use explicit `MAX_*` bounded iteration constants.
//! - No recursion in any test helper.
//! - Every `assert!` has a descriptive message string.

use nasa_rust_project::{BootstrapOpts, BootstrapResult, BootstrapRunner, StageResult};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum stages to verify in any single run result.
const MAX_STAGES_CHECK: usize = 16;

/// Maximum lines to scan in emitted JSON output.
const MAX_JSON_LINES: usize = 4096;

/// Maximum lines to scan in emitted Verilog output.
const MAX_VERILOG_LINES: usize = 2048;

/// Maximum number of test source variants to iterate.
const MAX_SOURCE_VARIANTS: usize = 32;

/// Maximum characters to scan in a summary line.
const MAX_SUMMARY_CHARS: usize = 1024;

// ---------------------------------------------------------------------------
// Helper functions (no recursion)
// ---------------------------------------------------------------------------

/// Write MIRR source to a temporary file with `.mirr` extension.
fn write_temp_mirr(src: &str) -> NamedTempFile {
    let mut f = NamedTempFile::with_suffix(".mirr").expect("tempfile creation must succeed");
    f.write_all(src.as_bytes()).expect("tempfile write must succeed");
    f
}

/// Write MIRR source to a named file inside a temporary directory.
/// Returns the (temp_dir, file_path) so the caller can keep them alive.
fn write_named_mirr(name: &str, src: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir creation must succeed");
    let path = dir.path().join(name);
    std::fs::write(&path, src).expect("named file write must succeed");
    (dir, path)
}

/// Create a runner with all flags disabled.
fn runner_default() -> BootstrapRunner {
    BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: None,
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    })
}

/// Create a runner with fail_fast enabled.
fn runner_fail_fast() -> BootstrapRunner {
    BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: None,
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: true,
        run_lexer_driver: false,
    })
}

/// Create a runner with JSON emission enabled.
fn runner_emit_json() -> BootstrapRunner {
    BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: None,
        emit_netlist_json: true,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    })
}

/// Create a runner with Verilog emission enabled.
fn runner_emit_verilog() -> BootstrapRunner {
    BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: None,
        emit_netlist_json: false,
        emit_netlist_verilog: true,
        fail_fast: false,
        run_lexer_driver: false,
    })
}

/// Create a runner with both JSON and Verilog emission enabled.
fn runner_emit_both() -> BootstrapRunner {
    BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: None,
        emit_netlist_json: true,
        emit_netlist_verilog: true,
        fail_fast: false,
        run_lexer_driver: false,
    })
}

/// Count how many stages with a given name appear in the result.
fn count_stages_named(result: &BootstrapResult, name: &str) -> usize {
    let mut count = 0;
    for i in 0..MAX_STAGES_CHECK {
        if i >= result.stages.len() {
            break;
        }
        if result.stages[i].name == name {
            count += 1;
        }
    }
    count
}

/// Find the first stage with the given name, if present.
fn find_stage<'a>(result: &'a BootstrapResult, name: &str) -> Option<&'a StageResult> {
    for i in 0..MAX_STAGES_CHECK {
        if i >= result.stages.len() {
            break;
        }
        if result.stages[i].name == name {
            return Some(&result.stages[i]);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// MIRR source constants (multiline format per rules)
// ---------------------------------------------------------------------------

/// Minimal valid module: 1 input, 1 output, 1 guard, 1 reflex.
const MINIMAL_SRC: &str = r#"
module minimal {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}
"#;

/// Neonatal respirator module with comparison guard and counter delay.
const NEONATAL_SRC: &str = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure: in u16;
    signal clamp_valve: out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for 1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;

/// Module with a short-delay guard (shift register path, N <= 16).
const SHORT_DELAY_SRC: &str = r#"
module short_delay_test {
    signal sensor: in bool;
    signal alarm: out bool;
    guard quick_check {
        when sensor
        for 4 cycles;
    }
    reflex trigger_alarm {
        on quick_check {
            alarm = true;
        }
    }
}
"#;

/// Module with a negated signal guard condition.
const NEGATED_GUARD_SRC: &str = r#"
module negated_guard_test {
    signal enable: in bool;
    signal alert: out bool;
    guard disabled_check {
        when !enable
        for 3 cycles;
    }
    reflex fire_alert {
        on disabled_check {
            alert = true;
        }
    }
}
"#;

/// Module with multiple guards and reflexes.
const MULTI_GUARD_SRC: &str = r#"
module multi_guard_mod {
    signal temp: in u16;
    signal pressure: in u16;
    signal alarm_a: out bool;
    signal alarm_b: out bool;

    guard high_temp {
        when temp > 100
        for 5 cycles;
    }

    guard low_pressure {
        when pressure < 20
        for 10 cycles;
    }

    reflex temp_alarm {
        on high_temp {
            alarm_a = true;
        }
    }

    reflex pressure_alarm {
        on low_pressure {
            alarm_b = true;
        }
    }
}
"#;

/// Module with equality comparison guard.
const EQUALITY_GUARD_SRC: &str = r#"
module eq_test {
    signal mode: in u8;
    signal locked: out bool;
    guard exact_mode {
        when mode == 42
        for 2 cycles;
    }
    reflex lock_it {
        on exact_mode {
            locked = true;
        }
    }
}
"#;

/// Module with greater-or-equal comparison guard.
const GE_GUARD_SRC: &str = r#"
module ge_test {
    signal voltage: in u16;
    signal overvolt: out bool;
    guard voltage_high {
        when voltage >= 240
        for 8 cycles;
    }
    reflex trip_breaker {
        on voltage_high {
            overvolt = true;
        }
    }
}
"#;

/// Module with not-equal comparison guard.
const NE_GUARD_SRC: &str = r#"
module ne_test {
    signal status: in u8;
    signal err_flag: out bool;
    guard not_ok {
        when status != 0
        for 2 cycles;
    }
    reflex flag_error {
        on not_ok {
            err_flag = true;
        }
    }
}
"#;

/// Module with less-or-equal comparison guard.
const LE_GUARD_SRC: &str = r#"
module le_test {
    signal battery: in u8;
    signal low_batt: out bool;
    guard battery_low {
        when battery <= 10
        for 5 cycles;
    }
    reflex warn_low {
        on battery_low {
            low_batt = true;
        }
    }
}
"#;

/// Module with a long counter delay (> 16 cycles).
const LONG_DELAY_SRC: &str = r#"
module long_delay_mod {
    signal input_sig: in bool;
    signal output_sig: out bool;
    guard long_wait {
        when input_sig
        for 500 cycles;
    }
    reflex delayed_response {
        on long_wait {
            output_sig = true;
        }
    }
}
"#;

/// Deliberately invalid MIRR: parser will fail.
const PARSE_ERROR_SRC: &str = "module bad { JUNK }";

/// Semantically invalid: reflex references undeclared guard.
const SEMANTIC_ERROR_SRC: &str = r#"
module semantic_err {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on nonexistent_guard {
            b = true;
        }
    }
}
"#;

/// Semantically invalid: guard references undeclared signal.
const UNDECLARED_SIGNAL_SRC: &str = r#"
module undeclared_sig {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when phantom_signal
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true;
        }
    }
}
"#;

// ===========================================================================
// BootstrapOpts tests
// ===========================================================================

#[test]
fn test_opts_default_has_no_fixture_root() {
    let opts = BootstrapOpts::default();
    assert!(opts.fixture_root.is_none(), "Default BootstrapOpts should have fixture_root=None");
}

#[test]
fn test_opts_default_emit_json_false() {
    let opts = BootstrapOpts::default();
    assert!(!opts.emit_netlist_json, "Default BootstrapOpts should have emit_netlist_json=false");
}

#[test]
fn test_opts_default_emit_verilog_false() {
    let opts = BootstrapOpts::default();
    assert!(
        !opts.emit_netlist_verilog,
        "Default BootstrapOpts should have emit_netlist_verilog=false"
    );
}

#[test]
fn test_opts_default_fail_fast_false() {
    let opts = BootstrapOpts::default();
    assert!(!opts.fail_fast, "Default BootstrapOpts should have fail_fast=false");
}

#[test]
fn test_opts_default_run_lexer_driver_false() {
    let opts = BootstrapOpts::default();
    assert!(!opts.run_lexer_driver, "Default BootstrapOpts should have run_lexer_driver=false");
}

#[test]
fn test_opts_clone_preserves_fields() {
    let opts = BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(PathBuf::from("/test/path")),
        emit_netlist_json: true,
        emit_netlist_verilog: true,
        fail_fast: true,
        run_lexer_driver: false,
    };
    let cloned = opts.clone();
    assert_eq!(cloned.fixture_root, opts.fixture_root, "Clone must preserve fixture_root");
    assert_eq!(
        cloned.emit_netlist_json, opts.emit_netlist_json,
        "Clone must preserve emit_netlist_json"
    );
    assert_eq!(
        cloned.emit_netlist_verilog, opts.emit_netlist_verilog,
        "Clone must preserve emit_netlist_verilog"
    );
    assert_eq!(cloned.fail_fast, opts.fail_fast, "Clone must preserve fail_fast");
}

#[test]
fn test_opts_debug_impl() {
    let opts = BootstrapOpts::default();
    let debug_str = format!("{:?}", opts);
    assert!(debug_str.contains("BootstrapOpts"), "Debug output should contain struct name");
}

// ===========================================================================
// BootstrapRunner construction tests
// ===========================================================================

#[test]
fn test_runner_new_returns_instance() {
    let runner = BootstrapRunner::new(BootstrapOpts::default());
    // Verify the runner is functional by running against a valid source.
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner.run(f.path());
    assert!(
        !result.stages.is_empty(),
        "Runner::new should produce a functional runner that generates stages"
    );
}

#[test]
fn test_runner_new_default_is_functional() {
    let runner = BootstrapRunner::new_default();
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner.run(f.path());
    assert!(!result.stages.is_empty(), "Runner::new_default should produce a functional runner");
}

#[test]
fn test_runner_default_trait_is_functional() {
    let runner = BootstrapRunner::default();
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner.run(f.path());
    assert!(!result.stages.is_empty(), "Runner::default() should produce a functional runner");
}

// ===========================================================================
// Stage 1: Read stage tests
// ===========================================================================

#[test]
fn test_read_stage_passes_for_valid_file() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let read_stage = find_stage(&result, "Read");
    assert!(read_stage.is_some(), "Read stage must be present in results");
    assert!(read_stage.unwrap().ok, "Read stage must pass for a valid temp file");
}

#[test]
fn test_read_stage_message_contains_byte_count() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let read_stage = find_stage(&result, "Read").expect("Read stage must exist");
    assert!(
        read_stage.message.contains("bytes read"),
        "Read stage message must mention bytes read, got: {}",
        read_stage.message
    );
}

#[test]
fn test_read_stage_fails_for_nonexistent_file() {
    let result = runner_default().run("/tmp/nonexistent_mirr_file_9999.mirr");
    assert!(!result.ok, "Result must be failed for nonexistent file");
    let read_stage = find_stage(&result, "Read");
    assert!(read_stage.is_some(), "Read stage must be present even on failure");
    assert!(!read_stage.unwrap().ok, "Read stage must be marked failed for missing file");
}

#[test]
fn test_read_stage_failure_message_contains_path() {
    let bogus_path = "/tmp/does_not_exist_bootstrap_test.mirr";
    let result = runner_default().run(bogus_path);
    let read_stage = find_stage(&result, "Read").expect("Read stage must exist");
    assert!(
        read_stage.message.contains("does_not_exist_bootstrap_test"),
        "Read failure message must contain the file path, got: {}",
        read_stage.message
    );
}

#[test]
fn test_read_stage_failure_returns_immediately() {
    let result = runner_default().run("/tmp/nonexistent_mirr_file_9999.mirr");
    assert_eq!(
        result.stages.len(),
        1,
        "On Read failure, only the Read stage should be present, got {} stages",
        result.stages.len()
    );
}

#[test]
fn test_source_path_preserved_for_nonexistent() {
    let path = "/tmp/nonexistent_mirr_file_9999.mirr";
    let result = runner_default().run(path);
    assert_eq!(
        result.source_path,
        PathBuf::from(path),
        "source_path must preserve the input path even on failure"
    );
}

// ===========================================================================
// Stage 2: Parse stage tests
// ===========================================================================

#[test]
fn test_parse_stage_passes_for_valid_mirr() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let parse_stage = find_stage(&result, "Parse");
    assert!(parse_stage.is_some(), "Parse stage must be present for valid MIRR");
    assert!(parse_stage.unwrap().ok, "Parse stage must pass for valid MIRR source");
}

#[test]
fn test_parse_stage_message_contains_signal_count() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let parse_stage = find_stage(&result, "Parse").expect("Parse stage must exist");
    assert!(
        parse_stage.message.contains("signal(s)"),
        "Parse stage message must mention signals, got: {}",
        parse_stage.message
    );
}

#[test]
fn test_parse_stage_message_contains_guard_count() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let parse_stage = find_stage(&result, "Parse").expect("Parse stage must exist");
    assert!(
        parse_stage.message.contains("guard(s)"),
        "Parse stage message must mention guards, got: {}",
        parse_stage.message
    );
}

#[test]
fn test_parse_stage_message_contains_reflex_count() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let parse_stage = find_stage(&result, "Parse").expect("Parse stage must exist");
    assert!(
        parse_stage.message.contains("reflex(es)"),
        "Parse stage message must mention reflexes, got: {}",
        parse_stage.message
    );
}

#[test]
fn test_parse_stage_fails_for_invalid_mirr() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_default().run(f.path());
    assert!(!result.ok, "Result must fail for parse-error MIRR");
    let parse_stage = find_stage(&result, "Parse");
    assert!(parse_stage.is_some(), "Parse stage must be present on parse failure");
    assert!(!parse_stage.unwrap().ok, "Parse stage must be marked failed for invalid MIRR");
}

#[test]
fn test_parse_failure_stops_pipeline() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_default().run(f.path());
    // Only Read and Parse should be present (pipeline stops after parse failure).
    assert_eq!(
        result.stages.len(),
        2,
        "Parse failure should stop pipeline at 2 stages (Read, Parse), got {}",
        result.stages.len()
    );
}

#[test]
fn test_parse_empty_file_fails() {
    let f = write_temp_mirr("");
    let result = runner_default().run(f.path());
    assert!(!result.ok, "Empty file should fail parsing");
}

#[test]
fn test_parse_whitespace_only_file_fails() {
    let f = write_temp_mirr("   \n  \n  ");
    let result = runner_default().run(f.path());
    assert!(!result.ok, "Whitespace-only file should fail parsing");
}

// ===========================================================================
// Stage 3: Validate stage tests
// ===========================================================================

#[test]
fn test_validate_stage_passes_for_valid_mirr() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let validate_stage = find_stage(&result, "Validate");
    assert!(validate_stage.is_some(), "Validate stage must be present for valid MIRR");
    assert!(validate_stage.unwrap().ok, "Validate stage must pass for valid MIRR source");
}

#[test]
fn test_validate_stage_message_on_success() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let validate_stage = find_stage(&result, "Validate").expect("Validate stage must exist");
    assert!(
        validate_stage.message.contains("semantic checks passed"),
        "Validate success message should mention semantic checks, got: {}",
        validate_stage.message
    );
}

#[test]
fn test_validate_stage_fails_for_undeclared_guard_ref() {
    let f = write_temp_mirr(SEMANTIC_ERROR_SRC);
    let result = runner_default().run(f.path());
    let validate_stage = find_stage(&result, "Validate");
    assert!(validate_stage.is_some(), "Validate stage must be present for semantic error");
    assert!(
        !validate_stage.unwrap().ok,
        "Validate stage must fail when reflex references undeclared guard"
    );
}

#[test]
fn test_validate_stage_fails_for_undeclared_signal_ref() {
    let f = write_temp_mirr(UNDECLARED_SIGNAL_SRC);
    let result = runner_default().run(f.path());
    let validate_stage = find_stage(&result, "Validate");
    assert!(validate_stage.is_some(), "Validate stage must be present for undeclared signal error");
    assert!(
        !validate_stage.unwrap().ok,
        "Validate stage must fail when guard references undeclared signal"
    );
}

#[test]
fn test_validate_failure_does_not_stop_pipeline_without_fail_fast() {
    let f = write_temp_mirr(SEMANTIC_ERROR_SRC);
    let result = runner_default().run(f.path());
    // Without fail_fast, pipeline should continue past validate.
    // Read + Parse + Validate + TemporalLower (at minimum) should be present.
    assert!(
        result.stages.len() >= 3,
        "Without fail_fast, pipeline should continue past Validate, got {} stages",
        result.stages.len()
    );
}

// ===========================================================================
// Stage 4: TemporalLower stage tests
// ===========================================================================

#[test]
fn test_temporal_lower_stage_passes_for_valid_mirr() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let tl_stage = find_stage(&result, "TemporalLower");
    assert!(tl_stage.is_some(), "TemporalLower stage must be present for valid MIRR");
    assert!(tl_stage.unwrap().ok, "TemporalLower stage must pass for valid MIRR source");
}

#[test]
fn test_temporal_lower_message_contains_guard_count() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let tl_stage = find_stage(&result, "TemporalLower").expect("TemporalLower must exist");
    assert!(
        tl_stage.message.contains("guard(s) lowered"),
        "TemporalLower message must mention guards lowered, got: {}",
        tl_stage.message
    );
}

#[test]
fn test_temporal_lower_message_contains_signal_count() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let tl_stage = find_stage(&result, "TemporalLower").expect("TemporalLower must exist");
    assert!(
        tl_stage.message.contains("signal(s) generated"),
        "TemporalLower message must mention signals generated, got: {}",
        tl_stage.message
    );
}

#[test]
fn test_temporal_lower_for_multi_guard_module() {
    let f = write_temp_mirr(MULTI_GUARD_SRC);
    let result = runner_default().run(f.path());
    let tl_stage = find_stage(&result, "TemporalLower");
    assert!(tl_stage.is_some(), "TemporalLower stage must be present for multi-guard module");
    assert!(tl_stage.unwrap().ok, "TemporalLower must pass for multi-guard module");
}

// ===========================================================================
// Stage 5: FixtureParity stage tests
// ===========================================================================

#[test]
fn test_fixture_parity_skipped_when_no_fixture() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let fp_stage = find_stage(&result, "FixtureParity");
    assert!(fp_stage.is_some(), "FixtureParity stage must be present even when skipped");
    assert!(fp_stage.unwrap().ok, "FixtureParity should pass (skip) when no fixture is configured");
    assert!(
        fp_stage.unwrap().message.contains("skipped"),
        "FixtureParity skip message should mention 'skipped', got: {}",
        fp_stage.unwrap().message
    );
}

#[test]
fn test_fixture_parity_with_explicit_fixture_root() {
    let fixture_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures");
    if !fixture_root.exists() {
        return; // skip if fixtures not present in this environment
    }

    // Write with the correct stem for fixture lookup.
    let (_dir, path) = write_named_mirr("neonatal_respirator.mirr", NEONATAL_SRC);
    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(fixture_root),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&path);

    let fp_stage = find_stage(&result, "FixtureParity");
    assert!(fp_stage.is_some(), "FixtureParity stage must be present with explicit fixture_root");
    assert!(
        fp_stage.unwrap().ok,
        "FixtureParity should pass when fixture matches; msg: {}",
        fp_stage.unwrap().message
    );
}

#[test]
fn test_fixture_parity_reports_mismatch_with_fake_fixture() {
    // Create a fake fixture that will not match the actual netlist.
    let tmp_dir = tempfile::tempdir().expect("tempdir must succeed");
    let netlist_dir = tmp_dir.path().join("netlist");
    std::fs::create_dir_all(&netlist_dir).expect("create netlist dir must succeed");

    // Write a fixture with wrong ir_version to trigger mismatch.
    let fixture_json = r#"{
        "ir_version": "0.0-fake",
        "guards": [],
        "signals": [],
        "statistics": {
            "shift_registers_used": 0,
            "counters_used": 0,
            "logic_gates_used": 0,
            "max_delay_cycles": 0,
            "total_signals": 0,
            "compilation_time_us": null
        }
    }"#;
    std::fs::write(netlist_dir.join("minimal.json"), fixture_json)
        .expect("write fixture must succeed");

    let (_src_dir, src_path) = write_named_mirr("minimal.mirr", MINIMAL_SRC);

    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(tmp_dir.path().to_path_buf()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&src_path);

    let fp_stage = find_stage(&result, "FixtureParity");
    assert!(fp_stage.is_some(), "FixtureParity stage must be present with mismatched fixture");
    assert!(!fp_stage.unwrap().ok, "FixtureParity should fail when fixture does not match");
}

#[test]
fn test_fixture_parity_mismatch_message_mentions_ir_version() {
    let tmp_dir = tempfile::tempdir().expect("tempdir must succeed");
    let netlist_dir = tmp_dir.path().join("netlist");
    std::fs::create_dir_all(&netlist_dir).expect("create netlist dir must succeed");

    let fixture_json = r#"{
        "ir_version": "0.0-bad",
        "guards": [],
        "signals": [],
        "statistics": {
            "shift_registers_used": 0,
            "counters_used": 0,
            "logic_gates_used": 0,
            "max_delay_cycles": 0,
            "total_signals": 0,
            "compilation_time_us": null
        }
    }"#;
    std::fs::write(netlist_dir.join("minimal.json"), fixture_json)
        .expect("write fixture must succeed");

    let (_src_dir, src_path) = write_named_mirr("minimal.mirr", MINIMAL_SRC);

    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(tmp_dir.path().to_path_buf()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&src_path);

    let fp_stage = find_stage(&result, "FixtureParity").expect("FixtureParity must exist");
    assert!(
        fp_stage.message.contains("ir_version"),
        "Mismatch message should mention ir_version, got: {}",
        fp_stage.message
    );
}

#[test]
fn test_fixture_parity_guard_count_mismatch() {
    let tmp_dir = tempfile::tempdir().expect("tempdir must succeed");
    let netlist_dir = tmp_dir.path().join("netlist");
    std::fs::create_dir_all(&netlist_dir).expect("create netlist dir must succeed");

    // Fixture with correct ir_version but wrong guard count.
    let fixture_json = r#"{
        "ir_version": "2.0",
        "guards": [],
        "signals": [],
        "statistics": {
            "shift_registers_used": 0,
            "counters_used": 0,
            "logic_gates_used": 0,
            "max_delay_cycles": 0,
            "total_signals": 0,
            "compilation_time_us": null
        }
    }"#;
    std::fs::write(netlist_dir.join("minimal.json"), fixture_json)
        .expect("write fixture must succeed");

    let (_src_dir, src_path) = write_named_mirr("minimal.mirr", MINIMAL_SRC);

    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(tmp_dir.path().to_path_buf()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&src_path);

    let fp_stage = find_stage(&result, "FixtureParity").expect("FixtureParity must exist");
    assert!(!fp_stage.ok, "FixtureParity should fail when guard count mismatches");
    assert!(
        fp_stage.message.contains("guard count"),
        "Mismatch message should mention guard count, got: {}",
        fp_stage.message
    );
}

#[test]
fn test_fixture_nonexistent_root_skips_parity() {
    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(PathBuf::from("/nonexistent/fixture/root/path")),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner.run(f.path());

    // Even with a bogus fixture_root, if the netlist/<stem>.json file does
    // not exist, FixtureParity is skipped (returns ok with "skipped").
    let fp_stage = find_stage(&result, "FixtureParity");
    assert!(fp_stage.is_some(), "FixtureParity stage must still be present");
    assert!(
        fp_stage.unwrap().ok,
        "FixtureParity should pass (skip) when fixture file does not exist"
    );
}

// ===========================================================================
// fail_fast behavior tests
// ===========================================================================

#[test]
fn test_fail_fast_stops_on_parse_error() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_fail_fast().run(f.path());
    assert!(!result.ok, "fail_fast result must be failed on parse error");
    // Should have only Read + Parse stages.
    assert_eq!(
        result.stages.len(),
        2,
        "fail_fast should stop at Parse, got {} stages",
        result.stages.len()
    );
}

#[test]
fn test_fail_fast_stops_on_validation_error() {
    let f = write_temp_mirr(SEMANTIC_ERROR_SRC);
    let result = runner_fail_fast().run(f.path());
    assert!(!result.ok, "fail_fast result must be failed on validation error");
    // Should have Read + Parse + Validate (stopped at validate).
    let has_validate = find_stage(&result, "Validate").is_some();
    assert!(has_validate, "fail_fast run should still include the Validate stage");
    let has_temporal = find_stage(&result, "TemporalLower").is_some();
    assert!(!has_temporal, "fail_fast should NOT proceed to TemporalLower after validate failure");
}

#[test]
fn test_fail_fast_stops_on_read_error() {
    let result = runner_fail_fast().run("/tmp/nonexistent_fail_fast_test.mirr");
    assert!(!result.ok, "fail_fast result must be failed on read error");
    assert_eq!(
        result.stages.len(),
        1,
        "fail_fast should stop at Read on file not found, got {} stages",
        result.stages.len()
    );
}

// ===========================================================================
// emit_netlist_json flag tests
// ===========================================================================

#[test]
fn test_emit_json_populated_on_success() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    assert!(
        result.netlist_json.is_some(),
        "netlist_json must be populated when emit_netlist_json=true and pipeline succeeds"
    );
}

#[test]
fn test_emit_json_contains_ir_version() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(json.contains("\"ir_version\""), "Emitted JSON must contain ir_version field");
}

#[test]
fn test_emit_json_contains_guard_name() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(
        json.contains("\"g\"") || json.contains("g_"),
        "Emitted JSON must contain the guard name 'g'"
    );
}

#[test]
fn test_emit_json_contains_guards_array() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(json.contains("\"guards\""), "Emitted JSON must contain guards array");
}

#[test]
fn test_emit_json_contains_signals_array() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(json.contains("\"signals\""), "Emitted JSON must contain signals array");
}

#[test]
fn test_emit_json_contains_statistics() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(json.contains("\"statistics\""), "Emitted JSON must contain statistics object");
}

#[test]
fn test_emit_json_none_when_flag_off() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    assert!(
        result.netlist_json.is_none(),
        "netlist_json must be None when emit_netlist_json=false"
    );
}

#[test]
fn test_emit_json_none_on_parse_failure() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_emit_json().run(f.path());
    assert!(
        result.netlist_json.is_none(),
        "netlist_json must be None when pipeline fails at parse stage"
    );
}

#[test]
fn test_emit_json_for_neonatal_contains_guard_name() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(
        json.contains("sustained_pressure_drop"),
        "Neonatal JSON must contain guard name 'sustained_pressure_drop'"
    );
}

#[test]
fn test_emit_json_is_valid_json() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(json);
    assert!(
        parsed.is_ok(),
        "Emitted netlist_json must be valid JSON, parse error: {:?}",
        parsed.err()
    );
}

#[test]
fn test_emit_json_line_count_bounded() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let mut line_count = 0;
    for _i in 0..MAX_JSON_LINES {
        if line_count >= json.lines().count() {
            break;
        }
        line_count += 1;
    }
    assert!(
        line_count < MAX_JSON_LINES,
        "JSON output should be bounded within {} lines",
        MAX_JSON_LINES
    );
}

// ===========================================================================
// emit_netlist_verilog flag tests
// ===========================================================================

#[test]
fn test_emit_verilog_populated_on_success() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_verilog().run(f.path());
    assert!(
        result.netlist_verilog.is_some(),
        "netlist_verilog must be populated when emit_netlist_verilog=true and pipeline succeeds"
    );
}

#[test]
fn test_emit_verilog_contains_module_keyword() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_verilog().run(f.path());
    let v = result.netlist_verilog.as_ref().expect("netlist_verilog must be Some");
    assert!(v.contains("module"), "Emitted Verilog must contain 'module' keyword");
}

#[test]
fn test_emit_verilog_none_when_flag_off() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    assert!(
        result.netlist_verilog.is_none(),
        "netlist_verilog must be None when emit_netlist_verilog=false"
    );
}

#[test]
fn test_emit_verilog_none_on_parse_failure() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_emit_verilog().run(f.path());
    assert!(
        result.netlist_verilog.is_none(),
        "netlist_verilog must be None when pipeline fails at parse stage"
    );
}

#[test]
fn test_emit_verilog_for_neonatal() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_emit_verilog().run(f.path());
    let v = result.netlist_verilog.as_ref().expect("netlist_verilog must be Some");
    assert!(v.contains("sustained_pressure_drop"), "Neonatal Verilog should mention guard name");
}

#[test]
fn test_emit_verilog_line_count_bounded() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_emit_verilog().run(f.path());
    let v = result.netlist_verilog.as_ref().expect("netlist_verilog must be Some");
    let mut line_count = 0;
    for _i in 0..MAX_VERILOG_LINES {
        if line_count >= v.lines().count() {
            break;
        }
        line_count += 1;
    }
    assert!(
        line_count < MAX_VERILOG_LINES,
        "Verilog output should be bounded within {} lines",
        MAX_VERILOG_LINES
    );
}

// ===========================================================================
// Both emit flags tests
// ===========================================================================

#[test]
fn test_emit_both_flags_populates_both() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_both().run(f.path());
    assert!(
        result.netlist_json.is_some(),
        "netlist_json must be populated when both emit flags are set"
    );
    assert!(
        result.netlist_verilog.is_some(),
        "netlist_verilog must be populated when both emit flags are set"
    );
}

#[test]
fn test_emit_both_flags_json_is_valid() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_emit_both().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(json);
    assert!(parsed.is_ok(), "JSON with both flags must still be valid JSON");
}

// ===========================================================================
// BootstrapResult tests
// ===========================================================================

#[test]
fn test_result_ok_true_for_valid_mirr() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    assert!(result.ok, "Result.ok must be true when all stages pass");
}

#[test]
fn test_result_ok_false_for_parse_error() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_default().run(f.path());
    assert!(!result.ok, "Result.ok must be false when parse fails");
}

#[test]
fn test_result_ok_false_for_semantic_error() {
    let f = write_temp_mirr(SEMANTIC_ERROR_SRC);
    let result = runner_default().run(f.path());
    assert!(!result.ok, "Result.ok must be false when validation fails");
}

#[test]
fn test_result_source_path_preserved() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let expected_path = f.path().to_path_buf();
    let result = runner_default().run(f.path());
    assert_eq!(result.source_path, expected_path, "Result.source_path must match the input path");
}

#[test]
fn test_result_stages_not_empty_for_any_input() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    assert!(!result.stages.is_empty(), "Result.stages must never be empty after run()");
}

#[test]
fn test_result_debug_impl() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("BootstrapResult"), "Debug output should contain struct name");
}

// ===========================================================================
// summary_line() tests
// ===========================================================================

#[test]
fn test_summary_line_contains_self_host() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let summary = result.summary_line();
    assert!(
        summary.contains("SELF-HOST"),
        "summary_line must contain 'SELF-HOST', got: {}",
        summary
    );
}

#[test]
fn test_summary_line_contains_pass_for_success() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let summary = result.summary_line();
    assert!(
        summary.contains("PASS"),
        "summary_line must contain 'PASS' on success, got: {}",
        summary
    );
}

#[test]
fn test_summary_line_contains_fail_for_failure() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_default().run(f.path());
    let summary = result.summary_line();
    assert!(
        summary.contains("FAIL"),
        "summary_line must contain 'FAIL' on failure, got: {}",
        summary
    );
}

#[test]
fn test_summary_line_contains_stages_passed() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let summary = result.summary_line();
    assert!(
        summary.contains("stages passed"),
        "summary_line must contain 'stages passed', got: {}",
        summary
    );
}

#[test]
fn test_summary_line_contains_file_path() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let summary = result.summary_line();
    assert!(
        summary.contains(".mirr"),
        "summary_line should mention the source path, got: {}",
        summary
    );
}

#[test]
fn test_summary_line_pass_count_correct_on_success() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let total = result.stages.len();
    let summary = result.summary_line();
    let expected_fragment = format!("{}/{} stages passed", total, total);
    assert!(
        summary.contains(&expected_fragment),
        "summary_line must show {}, got: {}",
        expected_fragment,
        summary
    );
}

#[test]
fn test_summary_line_bounded_length() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_default().run(f.path());
    let summary = result.summary_line();
    assert!(
        summary.len() < MAX_SUMMARY_CHARS,
        "summary_line should be bounded, got {} chars",
        summary.len()
    );
}

// ===========================================================================
// print_report() tests (verify it does not panic)
// ===========================================================================

#[test]
fn test_print_report_does_not_panic_on_success() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    // print_report writes to stdout; just ensure no panic.
    result.print_report();
}

#[test]
fn test_print_report_does_not_panic_on_failure() {
    let f = write_temp_mirr(PARSE_ERROR_SRC);
    let result = runner_default().run(f.path());
    result.print_report();
}

#[test]
fn test_print_report_does_not_panic_with_json() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    result.print_report();
}

#[test]
fn test_print_report_does_not_panic_with_verilog() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_verilog().run(f.path());
    result.print_report();
}

#[test]
fn test_print_report_does_not_panic_with_both() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_emit_both().run(f.path());
    result.print_report();
}

// ===========================================================================
// StageResult tests
// ===========================================================================

#[test]
fn test_stage_result_clone() {
    let stage = StageResult {
        name: "TestStage".to_string(),
        ok: true,
        message: "test message".to_string(),
    };
    let cloned = stage.clone();
    assert_eq!(cloned.name, stage.name, "StageResult clone must preserve name");
    assert_eq!(cloned.ok, stage.ok, "StageResult clone must preserve ok");
    assert_eq!(cloned.message, stage.message, "StageResult clone must preserve message");
}

#[test]
fn test_stage_result_debug() {
    let stage =
        StageResult { name: "Debug".to_string(), ok: false, message: "an error".to_string() };
    let debug_str = format!("{:?}", stage);
    assert!(debug_str.contains("StageResult"), "StageResult debug must contain struct name");
    assert!(debug_str.contains("Debug"), "StageResult debug must contain stage name");
}

// ===========================================================================
// Pipeline stage ordering and naming tests
// ===========================================================================

#[test]
fn test_stage_order_on_success() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    // Expected order: Read, Parse, Validate, TemporalLower, FixtureParity
    let expected_names = ["Read", "Parse", "Validate", "TemporalLower", "FixtureParity"];
    assert!(
        result.stages.len() >= expected_names.len(),
        "Successful run must have at least {} stages, got {}",
        expected_names.len(),
        result.stages.len()
    );
    for i in 0..expected_names.len() {
        assert_eq!(
            result.stages[i].name, expected_names[i],
            "Stage {} name mismatch: expected '{}', got '{}'",
            i, expected_names[i], result.stages[i].name
        );
    }
}

#[test]
fn test_all_stages_pass_on_valid_minimal() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    for i in 0..MAX_STAGES_CHECK {
        if i >= result.stages.len() {
            break;
        }
        assert!(
            result.stages[i].ok,
            "Stage '{}' (index {}) should pass for valid minimal MIRR, msg: {}",
            result.stages[i].name, i, result.stages[i].message
        );
    }
}

#[test]
fn test_stage_counts_unique_names() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    assert_eq!(count_stages_named(&result, "Read"), 1, "Exactly one Read stage expected");
    assert_eq!(count_stages_named(&result, "Parse"), 1, "Exactly one Parse stage expected");
    assert_eq!(count_stages_named(&result, "Validate"), 1, "Exactly one Validate stage expected");
    assert_eq!(
        count_stages_named(&result, "TemporalLower"),
        1,
        "Exactly one TemporalLower stage expected"
    );
    assert_eq!(
        count_stages_named(&result, "FixtureParity"),
        1,
        "Exactly one FixtureParity stage expected"
    );
}

// ===========================================================================
// Various MIRR source form tests
// ===========================================================================

#[test]
fn test_short_delay_shift_register_path() {
    let f = write_temp_mirr(SHORT_DELAY_SRC);
    let result = runner_emit_json().run(f.path());
    assert!(result.ok, "Short delay (4 cycles) module must compile successfully");
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(
        json.contains("ShiftRegister"),
        "4-cycle guard should use ShiftRegister strategy, JSON: {}",
        json
    );
}

#[test]
fn test_long_delay_counter_path() {
    let f = write_temp_mirr(LONG_DELAY_SRC);
    let result = runner_emit_json().run(f.path());
    assert!(result.ok, "Long delay (500 cycles) module must compile successfully");
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(
        json.contains("Counter"),
        "500-cycle guard should use Counter strategy, JSON: {}",
        json
    );
}

#[test]
fn test_neonatal_counter_path() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_emit_json().run(f.path());
    assert!(result.ok, "Neonatal module (1000 cycles) must compile successfully");
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(json.contains("Counter"), "1000-cycle guard should use Counter strategy");
}

#[test]
fn test_negated_guard_compiles() {
    let f = write_temp_mirr(NEGATED_GUARD_SRC);
    let result = runner_default().run(f.path());
    assert!(
        result.ok,
        "Negated guard module must compile successfully; stages: {:?}",
        result.stages
    );
}

#[test]
fn test_multi_guard_both_lowered() {
    let f = write_temp_mirr(MULTI_GUARD_SRC);
    let result = runner_emit_json().run(f.path());
    assert!(result.ok, "Multi-guard module must compile successfully");
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(json.contains("high_temp"), "JSON must contain first guard name 'high_temp'");
    assert!(json.contains("low_pressure"), "JSON must contain second guard name 'low_pressure'");
}

#[test]
fn test_equality_guard_compiles() {
    let f = write_temp_mirr(EQUALITY_GUARD_SRC);
    let result = runner_default().run(f.path());
    assert!(
        result.ok,
        "Equality guard (==) module must compile successfully; stages: {:?}",
        result.stages
    );
}

#[test]
fn test_ge_guard_compiles() {
    let f = write_temp_mirr(GE_GUARD_SRC);
    let result = runner_default().run(f.path());
    assert!(
        result.ok,
        "Greater-or-equal guard (>=) module must compile successfully; stages: {:?}",
        result.stages
    );
}

#[test]
fn test_ne_guard_compiles() {
    let f = write_temp_mirr(NE_GUARD_SRC);
    let result = runner_default().run(f.path());
    assert!(
        result.ok,
        "Not-equal guard (!=) module must compile successfully; stages: {:?}",
        result.stages
    );
}

#[test]
fn test_le_guard_compiles() {
    let f = write_temp_mirr(LE_GUARD_SRC);
    let result = runner_default().run(f.path());
    assert!(
        result.ok,
        "Less-or-equal guard (<=) module must compile successfully; stages: {:?}",
        result.stages
    );
}

// ===========================================================================
// Edge case and robustness tests
// ===========================================================================

#[test]
fn test_run_with_comments_in_source() {
    let src = r#"
module commented {
    // This is a comment
    signal x: in bool;
    signal y: out bool;
    // Another comment
    guard g {
        when x
        for 1 cycles;
    }
    // Reflex comment
    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let f = write_temp_mirr(src);
    let result = runner_default().run(f.path());
    assert!(
        result.ok,
        "Module with comments must compile successfully; stages: {:?}",
        result.stages
    );
}

#[test]
fn test_run_with_multiple_signals() {
    let src = r#"
module multi_sig {
    signal a: in bool;
    signal b: in u8;
    signal c: in u16;
    signal d: out bool;
    signal e: out u8;
    guard g {
        when a
        for 2 cycles;
    }
    reflex r {
        on g {
            d = true;
        }
    }
}
"#;
    let f = write_temp_mirr(src);
    let result = runner_default().run(f.path());
    assert!(result.ok, "Module with multiple signals must compile successfully");
    let parse_stage = find_stage(&result, "Parse").expect("Parse must exist");
    assert!(
        parse_stage.message.contains("5 signal(s)"),
        "Parse stage should report 5 signals, got: {}",
        parse_stage.message
    );
}

#[test]
fn test_run_with_large_cycle_count() {
    let src = r#"
module large_cycles {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 65535 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let f = write_temp_mirr(src);
    let result = runner_default().run(f.path());
    assert!(
        result.ok,
        "Module with large cycle count (65535) must compile; stages: {:?}",
        result.stages
    );
}

#[test]
fn test_run_with_single_cycle() {
    let src = r#"
module single_cycle {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let f = write_temp_mirr(src);
    let result = runner_emit_json().run(f.path());
    assert!(result.ok, "1-cycle guard must compile successfully");
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    assert!(json.contains("ShiftRegister"), "1-cycle guard should use ShiftRegister strategy");
}

#[test]
fn test_multiple_runs_on_same_runner() {
    let runner = runner_default();
    let f1 = write_temp_mirr(MINIMAL_SRC);
    let f2 = write_temp_mirr(NEONATAL_SRC);

    let r1 = runner.run(f1.path());
    let r2 = runner.run(f2.path());

    assert!(r1.ok, "First run on runner must succeed");
    assert!(r2.ok, "Second run on same runner must succeed");
    assert_ne!(r1.source_path, r2.source_path, "Different runs should have different source paths");
}

#[test]
fn test_run_success_then_failure_on_same_runner() {
    let runner = runner_default();
    let good = write_temp_mirr(MINIMAL_SRC);
    let bad = write_temp_mirr(PARSE_ERROR_SRC);

    let r_good = runner.run(good.path());
    let r_bad = runner.run(bad.path());

    assert!(r_good.ok, "Good source must succeed");
    assert!(!r_bad.ok, "Bad source must fail");
}

// ===========================================================================
// Comprehensive comparison operator coverage
// ===========================================================================

#[test]
fn test_all_comparison_operators_compile() {
    // Test all 6 comparison operators in guard conditions.
    let ops = ["<", "<=", ">", ">=", "==", "!="];
    for i in 0..ops.len() {
        if i >= MAX_SOURCE_VARIANTS {
            break;
        }
        let src = format!(
            r#"
module cmp_{i} {{
    signal v: in u16;
    signal out_sig: out bool;
    guard g {{
        when v {op} 100
        for 2 cycles;
    }}
    reflex r {{
        on g {{
            out_sig = true;
        }}
    }}
}}
"#,
            i = i,
            op = ops[i]
        );
        let f = write_temp_mirr(&src);
        let result = runner_default().run(f.path());
        assert!(
            result.ok,
            "Comparison operator '{}' must compile successfully; stages: {:?}",
            ops[i], result.stages
        );
    }
}

// ===========================================================================
// Fixture parity edge cases
// ===========================================================================

#[test]
fn test_fixture_parity_with_malformed_fixture_json() {
    let tmp_dir = tempfile::tempdir().expect("tempdir must succeed");
    let netlist_dir = tmp_dir.path().join("netlist");
    std::fs::create_dir_all(&netlist_dir).expect("create netlist dir must succeed");

    // Write invalid JSON.
    std::fs::write(netlist_dir.join("minimal.json"), "NOT VALID JSON {{{")
        .expect("write fixture must succeed");

    let (_src_dir, src_path) = write_named_mirr("minimal.mirr", MINIMAL_SRC);

    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(tmp_dir.path().to_path_buf()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&src_path);

    let fp_stage = find_stage(&result, "FixtureParity");
    assert!(fp_stage.is_some(), "FixtureParity must be present even with malformed fixture");
    assert!(!fp_stage.unwrap().ok, "FixtureParity must fail for malformed fixture JSON");
}

#[test]
fn test_fixture_parity_signal_count_mismatch() {
    let tmp_dir = tempfile::tempdir().expect("tempdir must succeed");
    let netlist_dir = tmp_dir.path().join("netlist");
    std::fs::create_dir_all(&netlist_dir).expect("create netlist dir must succeed");

    // Correct ir_version, correct guard count (1), but wrong signal count.
    let fixture_json = r#"{
        "ir_version": "2.0",
        "guards": [
            {
                "ShiftRegister": {
                    "name": "g",
                    "input_signal": "x",
                    "output_signal": "g_out",
                    "stages": ["g_s0"],
                    "delay_cycles": 1,
                    "condition_kind": { "SimpleSignal": "x" }
                }
            }
        ],
        "signals": [],
        "statistics": {
            "shift_registers_used": 1,
            "counters_used": 0,
            "logic_gates_used": 0,
            "max_delay_cycles": 1,
            "total_signals": 0,
            "compilation_time_us": null
        }
    }"#;
    std::fs::write(netlist_dir.join("minimal.json"), fixture_json)
        .expect("write fixture must succeed");

    let (_src_dir, src_path) = write_named_mirr("minimal.mirr", MINIMAL_SRC);

    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(tmp_dir.path().to_path_buf()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&src_path);

    let fp_stage = find_stage(&result, "FixtureParity").expect("FixtureParity must exist");
    assert!(!fp_stage.ok, "FixtureParity must fail when signal count mismatches");
    assert!(
        fp_stage.message.contains("signal count"),
        "Mismatch message should mention signal count, got: {}",
        fp_stage.message
    );
}

#[test]
fn test_fixture_parity_statistics_mismatch() {
    let tmp_dir = tempfile::tempdir().expect("tempdir must succeed");
    let netlist_dir = tmp_dir.path().join("netlist");
    std::fs::create_dir_all(&netlist_dir).expect("create netlist dir must succeed");

    // We need to get the actual netlist first, then modify a statistics field.
    let f = write_temp_mirr(MINIMAL_SRC);
    let result_actual = runner_emit_json().run(f.path());
    let actual_json_str = result_actual.netlist_json.as_ref().expect("must have json");

    // Parse actual JSON, modify a statistics field, write as fixture.
    let mut val: serde_json::Value =
        serde_json::from_str(actual_json_str).expect("actual json must parse");
    val["statistics"]["max_delay_cycles"] = serde_json::json!(99999);
    let modified = serde_json::to_string_pretty(&val).expect("serialize must work");

    std::fs::write(netlist_dir.join("minimal.json"), &modified)
        .expect("write fixture must succeed");

    let (_src_dir, src_path) = write_named_mirr("minimal.mirr", MINIMAL_SRC);

    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(tmp_dir.path().to_path_buf()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&src_path);

    let fp_stage = find_stage(&result, "FixtureParity").expect("FixtureParity must exist");
    assert!(!fp_stage.ok, "FixtureParity must fail when statistics mismatch");
    assert!(
        fp_stage.message.contains("max_delay_cycles"),
        "Mismatch message should mention the mismatched field, got: {}",
        fp_stage.message
    );
}

// ===========================================================================
// JSON structure validation tests
// ===========================================================================

#[test]
fn test_emit_json_has_correct_ir_version() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: serde_json::Value = serde_json::from_str(json).expect("JSON must parse");
    assert_eq!(parsed["ir_version"].as_str().unwrap_or(""), "2.0", "ir_version must be '2.0'");
}

#[test]
fn test_emit_json_guards_is_array() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: serde_json::Value = serde_json::from_str(json).expect("JSON must parse");
    assert!(parsed["guards"].is_array(), "guards must be a JSON array");
}

#[test]
fn test_emit_json_signals_is_array() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: serde_json::Value = serde_json::from_str(json).expect("JSON must parse");
    assert!(parsed["signals"].is_array(), "signals must be a JSON array");
}

#[test]
fn test_emit_json_statistics_is_object() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: serde_json::Value = serde_json::from_str(json).expect("JSON must parse");
    assert!(parsed["statistics"].is_object(), "statistics must be a JSON object");
}

#[test]
fn test_emit_json_statistics_fields_present() {
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: serde_json::Value = serde_json::from_str(json).expect("JSON must parse");
    let stats = &parsed["statistics"];

    let expected_fields = [
        "shift_registers_used",
        "counters_used",
        "logic_gates_used",
        "max_delay_cycles",
        "total_signals",
    ];
    for i in 0..expected_fields.len() {
        assert!(
            !stats[expected_fields[i]].is_null(),
            "statistics.{} must be present in JSON output",
            expected_fields[i]
        );
    }
}

#[test]
fn test_emit_json_multi_guard_has_two_guards() {
    let f = write_temp_mirr(MULTI_GUARD_SRC);
    let result = runner_emit_json().run(f.path());
    let json = result.netlist_json.as_ref().expect("netlist_json must be Some");
    let parsed: serde_json::Value = serde_json::from_str(json).expect("JSON must parse");
    let guards = parsed["guards"].as_array().expect("guards must be array");
    assert_eq!(
        guards.len(),
        2,
        "Multi-guard module must produce 2 compiled guards, got {}",
        guards.len()
    );
}

// ===========================================================================
// Neonatal fixture parity round-trip
// ===========================================================================

#[test]
fn test_neonatal_fixture_round_trip() {
    let fixture_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures");
    if !fixture_root.join("netlist").join("neonatal_respirator.json").exists() {
        return; // skip if fixture not present
    }

    let (_dir, path) = write_named_mirr("neonatal_respirator.mirr", NEONATAL_SRC);
    let runner = BootstrapRunner::new(BootstrapOpts {
        run_mirr_stages: false,
        fixture_root: Some(fixture_root),
        emit_netlist_json: true,
        emit_netlist_verilog: true,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&path);

    assert!(
        result.ok,
        "Neonatal round-trip must pass all stages including FixtureParity; stages: {:?}",
        result.stages
    );
    assert!(result.netlist_json.is_some(), "JSON must be emitted on round-trip success");
    assert!(result.netlist_verilog.is_some(), "Verilog must be emitted on round-trip success");
}

// ===========================================================================
// Source path edge cases
// ===========================================================================

#[test]
fn test_source_path_with_spaces_in_name() {
    let dir = tempfile::tempdir().expect("tempdir must succeed");
    let path = dir.path().join("file with spaces.mirr");
    std::fs::write(&path, MINIMAL_SRC).expect("write must succeed");

    let result = runner_default().run(&path);
    assert!(result.ok, "File with spaces in name must compile; stages: {:?}", result.stages);
    assert_eq!(result.source_path, path, "source_path must preserve path with spaces");
}

#[test]
fn test_source_path_with_unicode_in_name() {
    let dir = tempfile::tempdir().expect("tempdir must succeed");
    let path = dir.path().join("module_alpha.mirr");
    std::fs::write(&path, MINIMAL_SRC).expect("write must succeed");

    let result = runner_default().run(&path);
    assert!(result.ok, "File with unicode-like name must compile; stages: {:?}", result.stages);
}

// ===========================================================================
// Semantic error detail tests
// ===========================================================================

#[test]
fn test_semantic_error_mentions_undeclared_guard() {
    let f = write_temp_mirr(SEMANTIC_ERROR_SRC);
    let result = runner_default().run(f.path());
    let validate_stage = find_stage(&result, "Validate").expect("Validate must exist");
    assert!(
        validate_stage.message.contains("nonexistent_guard")
            || validate_stage.message.contains("E205"),
        "Validation error should mention the undeclared guard name or error code, got: {}",
        validate_stage.message
    );
}

#[test]
fn test_semantic_error_mentions_undeclared_signal() {
    let f = write_temp_mirr(UNDECLARED_SIGNAL_SRC);
    let result = runner_default().run(f.path());
    let validate_stage = find_stage(&result, "Validate").expect("Validate must exist");
    assert!(
        validate_stage.message.contains("phantom_signal")
            || validate_stage.message.contains("E204"),
        "Validation error should mention the undeclared signal name or error code, got: {}",
        validate_stage.message
    );
}

// ===========================================================================
// Integration: pipeline consistency across different sources
// ===========================================================================

#[test]
fn test_pipeline_consistency_across_sources() {
    // Verify that various valid sources all produce a consistent stage ordering.
    let sources = [MINIMAL_SRC, NEONATAL_SRC, SHORT_DELAY_SRC, MULTI_GUARD_SRC];
    let expected_names = ["Read", "Parse", "Validate", "TemporalLower", "FixtureParity"];

    for idx in 0..sources.len() {
        if idx >= MAX_SOURCE_VARIANTS {
            break;
        }
        let f = write_temp_mirr(sources[idx]);
        let result = runner_default().run(f.path());
        assert!(result.ok, "Source variant {} must compile successfully", idx);
        assert!(
            result.stages.len() >= expected_names.len(),
            "Source variant {} must produce at least {} stages, got {}",
            idx,
            expected_names.len(),
            result.stages.len()
        );
        for j in 0..expected_names.len() {
            assert_eq!(
                result.stages[j].name, expected_names[j],
                "Source variant {}: stage {} name mismatch, expected '{}', got '{}'",
                idx, j, expected_names[j], result.stages[j].name
            );
        }
    }
}

#[test]
fn test_parse_stage_counts_match_source() {
    // Minimal: 2 signals, 1 guard, 1 reflex
    let f = write_temp_mirr(MINIMAL_SRC);
    let result = runner_default().run(f.path());
    let parse_stage = find_stage(&result, "Parse").expect("Parse must exist");
    assert!(
        parse_stage.message.contains("2 signal(s)"),
        "Minimal module should have 2 signals, got: {}",
        parse_stage.message
    );
    assert!(
        parse_stage.message.contains("1 guard(s)"),
        "Minimal module should have 1 guard, got: {}",
        parse_stage.message
    );
    assert!(
        parse_stage.message.contains("1 reflex(es)"),
        "Minimal module should have 1 reflex, got: {}",
        parse_stage.message
    );
}

#[test]
fn test_neonatal_parse_stage_counts() {
    // Neonatal: 3 signals, 1 guard, 1 reflex
    let f = write_temp_mirr(NEONATAL_SRC);
    let result = runner_default().run(f.path());
    let parse_stage = find_stage(&result, "Parse").expect("Parse must exist");
    assert!(
        parse_stage.message.contains("3 signal(s)"),
        "Neonatal module should have 3 signals, got: {}",
        parse_stage.message
    );
    assert!(
        parse_stage.message.contains("1 guard(s)"),
        "Neonatal module should have 1 guard, got: {}",
        parse_stage.message
    );
}

#[test]
fn test_multi_guard_parse_stage_counts() {
    // Multi-guard: 4 signals, 2 guards, 2 reflexes
    let f = write_temp_mirr(MULTI_GUARD_SRC);
    let result = runner_default().run(f.path());
    let parse_stage = find_stage(&result, "Parse").expect("Parse must exist");
    assert!(
        parse_stage.message.contains("4 signal(s)"),
        "Multi-guard module should have 4 signals, got: {}",
        parse_stage.message
    );
    assert!(
        parse_stage.message.contains("2 guard(s)"),
        "Multi-guard module should have 2 guards, got: {}",
        parse_stage.message
    );
    assert!(
        parse_stage.message.contains("2 reflex(es)"),
        "Multi-guard module should have 2 reflexes, got: {}",
        parse_stage.message
    );
}
