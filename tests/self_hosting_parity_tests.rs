//! MIRR Self-Hosting Parity Tests — CI Gate
//!
//! These integration tests verify that the Rust compiler pipeline produces
//! output matching the golden IR contract fixtures in tests/fixtures/.
//! They form the mandatory CI gate described in the self-hosting plan (Task 9).
//!
//! When the MIRR-in-MIRR interpreter is wired into the bootstrap runner,
//! these same tests will also exercise the MIRR pipeline path, confirming
//! byte-stable or semantically-equivalent output.
//!
//! Ref: docs/self_hosting_ir_contract.md
//!      docs/self_hosting_core_spec.md §4

use std::path::{Path, PathBuf};

use nasa_rust_project::{BootstrapOpts, BootstrapResult, BootstrapRunner};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Resolve the repo-root fixture directory.
fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures")
}

/// Resolve the path to the canonical example source file.
fn neonatal_source() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples").join("neonatal_respirator.mirr")
}

/// Run the bootstrap pipeline on the given source with fixture parity enabled.
fn run_pipeline(source: &Path) -> BootstrapResult {
    let runner = BootstrapRunner::new(BootstrapOpts {
        fixture_root: Some(fixture_root()),
        emit_netlist_json: true,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    runner.run(source)
}

// ===========================================================================
// Full pipeline parity — canonical neonatal_respirator example
// ===========================================================================

#[test]
fn selfhost_neonatal_all_stages_pass() {
    let result = run_pipeline(&neonatal_source());

    // Every stage must succeed.
    for stage in &result.stages {
        assert!(stage.ok, "Stage '{}' failed: {}", stage.name, stage.message,);
    }
    assert!(result.ok, "Overall pipeline must pass");
}

#[test]
fn selfhost_neonatal_fixture_parity_stage_present() {
    let result = run_pipeline(&neonatal_source());

    // The FixtureParity stage must be present and pass (not skipped).
    let parity = result
        .stages
        .iter()
        .find(|s| s.name == "FixtureParity")
        .expect("FixtureParity stage must exist");

    assert!(parity.ok, "FixtureParity stage must pass: {}", parity.message);
    // It must actually have matched the fixture, not just skipped.
    assert!(
        !parity.message.contains("skipped"),
        "FixtureParity must not be skipped — it must match the golden fixture"
    );
}

#[test]
fn selfhost_neonatal_emits_valid_json() {
    let result = run_pipeline(&neonatal_source());

    let json_str = result
        .netlist_json
        .as_ref()
        .expect("netlist_json must be populated when emit_netlist_json=true");

    // Parse as generic JSON to confirm validity.
    let val: serde_json::Value =
        serde_json::from_str(json_str).expect("emitted netlist must be valid JSON");

    // Structural contract assertions.
    assert_eq!(val["ir_version"], "1.0", "ir_version must be 1.0");
    assert!(val["guards"].is_array(), "guards must be an array");
    assert!(val["signals"].is_array(), "signals must be an array");
    assert!(val["statistics"].is_object(), "statistics must be an object");
}

#[test]
fn selfhost_neonatal_guard_strategy_is_counter() {
    let result = run_pipeline(&neonatal_source());
    let json_str = result.netlist_json.as_ref().unwrap();
    let val: serde_json::Value = serde_json::from_str(json_str).unwrap();

    let guards = val["guards"].as_array().unwrap();
    assert_eq!(guards.len(), 1, "neonatal_respirator has exactly 1 guard");

    // The guard must use the Counter strategy (1000 cycles > 16 threshold).
    let guard = &guards[0];
    assert!(
        guard.get("Counter").is_some(),
        "guard must be Counter variant (1000 cycles > SHIFT_REGISTER_THRESHOLD)"
    );

    let counter = &guard["Counter"];
    assert_eq!(counter["name"], "sustained_pressure_drop");
    assert_eq!(counter["target_count"], 1000);
}

#[test]
fn selfhost_neonatal_signal_contract() {
    let result = run_pipeline(&neonatal_source());
    let json_str = result.netlist_json.as_ref().unwrap();
    let val: serde_json::Value = serde_json::from_str(json_str).unwrap();

    let signals = val["signals"].as_array().unwrap();
    assert_eq!(signals.len(), 3, "counter guard produces exactly 3 signals");

    // Expected signals in order.
    let expected = [
        ("sustained_pressure_drop_counter", "Counter"),
        ("sustained_pressure_drop_cmp", "Comparator"),
        ("sustained_pressure_drop_out", "LogicGate"),
    ];

    for (sig, (exp_name, exp_kind)) in signals.iter().zip(expected.iter()) {
        assert_eq!(sig["name"], *exp_name, "signal name mismatch");
        assert_eq!(sig["kind"], *exp_kind, "signal kind mismatch for {exp_name}");
    }

    // Counter signal must be Unsigned(11) — ceil(log2(1000)) + 1 = 11.
    assert_eq!(
        signals[0]["ty"],
        serde_json::json!({"Unsigned": 11}),
        "counter width must be Unsigned(11)"
    );
}

#[test]
fn selfhost_neonatal_statistics_contract() {
    let result = run_pipeline(&neonatal_source());
    let json_str = result.netlist_json.as_ref().unwrap();
    let val: serde_json::Value = serde_json::from_str(json_str).unwrap();

    let stats = &val["statistics"];
    assert_eq!(stats["shift_registers_used"], 0);
    assert_eq!(stats["counters_used"], 1);
    assert_eq!(stats["logic_gates_used"], 1);
    assert_eq!(stats["max_delay_cycles"], 1000);
    assert_eq!(stats["total_signals"], 3);
}

// ===========================================================================
// Failure detection — bad input must be caught
// ===========================================================================

#[test]
fn selfhost_parse_error_fails_pipeline() {
    // Write a malformed MIRR file to a temp location.
    let tmp_dir = tempfile::tempdir().expect("tempdir");
    let bad_file = tmp_dir.path().join("bad_syntax.mirr");
    std::fs::write(&bad_file, "module bad { THIS IS NOT VALID MIRR }").unwrap();

    let runner = BootstrapRunner::new(BootstrapOpts {
        fixture_root: Some(fixture_root()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: true,
        run_lexer_driver: false,
    });
    let result = runner.run(&bad_file);

    assert!(!result.ok, "pipeline must fail on malformed input");

    let parse_stage = result.stages.iter().find(|s| s.name == "Parse");
    assert!(parse_stage.is_some(), "Parse stage must appear");
    assert!(!parse_stage.unwrap().ok, "Parse stage must report failure");
}

#[test]
fn selfhost_missing_file_fails_pipeline() {
    let runner = BootstrapRunner::new(BootstrapOpts {
        fixture_root: Some(fixture_root()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: true,
        run_lexer_driver: false,
    });
    let result = runner.run("nonexistent_file_xyz.mirr");

    assert!(!result.ok, "pipeline must fail on missing file");
    assert_eq!(result.stages[0].name, "Read");
    assert!(!result.stages[0].ok, "Read stage must fail");
}

#[test]
fn selfhost_validation_error_fails_pipeline() {
    // A module with a duplicate signal declaration.
    let tmp_dir = tempfile::tempdir().expect("tempdir");
    let dup_file = tmp_dir.path().join("dup_signals.mirr");
    std::fs::write(
        &dup_file,
        r#"
module dup_test {
    signal x: in bool;
    signal x: out bool;
}
"#,
    )
    .unwrap();

    let runner = BootstrapRunner::new(BootstrapOpts {
        fixture_root: Some(fixture_root()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: false,
        run_lexer_driver: false,
    });
    let result = runner.run(&dup_file);

    assert!(!result.ok, "pipeline must fail on duplicate signals");

    let validate_stage = result.stages.iter().find(|s| s.name == "Validate");
    assert!(validate_stage.is_some(), "Validate stage must appear");
    assert!(!validate_stage.unwrap().ok, "Validate stage must report failure on duplicate signals");
}

// ===========================================================================
// Summary line format (for CI log parsing)
// ===========================================================================

#[test]
fn selfhost_summary_line_ci_format() {
    let result = run_pipeline(&neonatal_source());
    let summary = result.summary_line();

    // CI gate pattern: [SELF-HOST PASS] N/M stages passed — <path>
    assert!(
        summary.starts_with("[SELF-HOST PASS]"),
        "successful run must produce PASS summary, got: {summary}"
    );
    assert!(summary.contains("stages passed"), "summary must include 'stages passed'");
}

#[test]
fn selfhost_failure_summary_says_fail() {
    let runner = BootstrapRunner::new(BootstrapOpts {
        fixture_root: Some(fixture_root()),
        emit_netlist_json: false,
        emit_netlist_verilog: false,
        fail_fast: true,
        run_lexer_driver: false,
    });
    let result = runner.run("nonexistent_xyz.mirr");
    let summary = result.summary_line();

    assert!(
        summary.starts_with("[SELF-HOST FAIL]"),
        "failed run must produce FAIL summary, got: {summary}"
    );
}

// ===========================================================================
// Stage count contract
// ===========================================================================

#[test]
fn selfhost_neonatal_has_five_stages() {
    let result = run_pipeline(&neonatal_source());

    // The pipeline must execute exactly 5 stages:
    // Read → Parse → Validate → TemporalLower → FixtureParity
    assert_eq!(
        result.stages.len(),
        5,
        "full pipeline must have 5 stages; got: {:?}",
        result.stages.iter().map(|s| &s.name).collect::<Vec<_>>()
    );

    let names: Vec<&str> = result.stages.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(
        names,
        vec!["Read", "Parse", "Validate", "TemporalLower", "FixtureParity"],
        "stages must appear in pipeline order"
    );
}

// ===========================================================================
// Cross-check: fixture file matches what the pipeline produces
// ===========================================================================

#[test]
fn selfhost_fixture_json_roundtrip_stable() {
    // Read the golden fixture.
    let fixture_path = fixture_root().join("netlist").join("neonatal_respirator.json");
    let fixture_str = std::fs::read_to_string(&fixture_path).expect("golden fixture must exist");
    let fixture_val: serde_json::Value =
        serde_json::from_str(&fixture_str).expect("golden fixture must be valid JSON");

    // Run pipeline and get actual JSON.
    let result = run_pipeline(&neonatal_source());
    let actual_str = result.netlist_json.as_ref().unwrap();
    let actual_val: serde_json::Value =
        serde_json::from_str(actual_str).expect("pipeline JSON must be valid");

    // Compare all contract fields (excluding compilation_time_us).
    assert_eq!(actual_val["ir_version"], fixture_val["ir_version"]);
    assert_eq!(actual_val["guards"], fixture_val["guards"]);
    assert_eq!(actual_val["signals"], fixture_val["signals"]);

    // Statistics — compare field-by-field to exclude compilation_time_us.
    for field in &[
        "shift_registers_used",
        "counters_used",
        "logic_gates_used",
        "max_delay_cycles",
        "total_signals",
    ] {
        assert_eq!(
            actual_val["statistics"][field], fixture_val["statistics"][field],
            "statistics.{field} mismatch"
        );
    }
}
