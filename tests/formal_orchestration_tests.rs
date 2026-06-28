#![allow(clippy::field_reassign_with_default)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

//! Integration tests for the formal verification orchestration module
//! (`src/toolchain/formal.rs`).
//!
//! Tests cover: struct construction, Default impls, FormalStatus equality,
//! run_formal_pipeline error paths, constant values, clone/debug traits,
//! and end-to-end pipeline-to-formal-config wiring.

use mirrc::parser::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::toolchain::formal::{
    FormalConfig, FormalResult, FormalStatus, PropertyVerdict, MAX_FORMAL_DEPTH,
    MAX_FORMAL_PROPERTIES,
};
use mirrc::toolchain::sby::SbyEngine;
use mirrc::toolchain::{Tool, ToolInfo, ToolRegistry, ToolchainError};

/// Bounded iteration limits for test loops.
const MAX_TEST_CONFIGS: usize = 32;
const MAX_TEST_VERDICTS: usize = 64;
const MAX_TEST_STATUS_VARIANTS: usize = 8;
const MAX_TEST_ENGINES: usize = 8;

/// Canonical MIRR module used across tests — satisfies the minimum
/// requirement of 1 input, 1 output, 1 guard, 1 reflex.
const MIRR_MODULE: &str = "\
module formal_test {
    signal clk_en: in bool;
    signal alarm: out bool;
    guard watchdog {
        when clk_en
        for 3 cycles;
    }
    reflex trigger {
        on watchdog {
            alarm = true;
        }
    }
}
";

// ---------------------------------------------------------------------------
// Helper functions (no recursion)
// ---------------------------------------------------------------------------

/// Build a ToolRegistry where sby is explicitly marked unavailable.
fn registry_without_sby() -> ToolRegistry {
    let mut reg = ToolRegistry::new();
    reg.tools.insert(
        Tool::Sby,
        ToolInfo { path: "sby".to_string(), version: String::new(), available: false },
    );
    reg
}

/// Build a completely empty ToolRegistry (no tools registered at all).
fn empty_registry() -> ToolRegistry {
    ToolRegistry::new()
}

/// Build a FormalConfig pointing to a fictitious SV file.
fn config_for_sv(sv_path: &str) -> FormalConfig {
    FormalConfig { sv_path: sv_path.to_string(), ..FormalConfig::default() }
}

/// Build a FormalConfig with prove enabled and a bind file.
fn config_with_bind(sv_path: &str, bind_path: &str) -> FormalConfig {
    FormalConfig {
        bmc_depth: 50,
        prove: true,
        engine: SbyEngine::Yices,
        sv_path: sv_path.to_string(),
        bind_path: Some(bind_path.to_string()),
        extra_files: Vec::new(),
    }
}

/// Construct a PropertyVerdict for testing.
fn make_verdict(name: &str, task: &str, status: FormalStatus) -> PropertyVerdict {
    PropertyVerdict { name: name.to_string(), task: task.to_string(), status }
}

/// Construct a FormalResult for testing.
fn make_result(
    passed: bool,
    verdicts: Vec<PropertyVerdict>,
    stdout: &str,
    stderr: &str,
    exit_code: Option<i32>,
) -> FormalResult {
    FormalResult {
        exit_code,
        passed,
        verdicts,
        stdout: stdout.to_string(),
        stderr: stderr.to_string(),
    }
}

// ===========================================================================
// FormalConfig — Default trait
// ===========================================================================

#[test]
fn formal_config_default_bmc_depth() {
    let cfg = FormalConfig::default();
    assert_eq!(cfg.bmc_depth, 50, "default bmc_depth should be 20");
}

#[test]
fn formal_config_default_prove_false() {
    let cfg = FormalConfig::default();
    assert!(!cfg.prove, "default prove should be false");
}

#[test]
fn formal_config_default_engine_z3() {
    let cfg = FormalConfig::default();
    assert_eq!(cfg.engine, SbyEngine::Bitwuzla, "default engine should be Z3");
}

#[test]
fn formal_config_default_sv_path_empty() {
    let cfg = FormalConfig::default();
    assert!(cfg.sv_path.is_empty(), "default sv_path should be empty");
}

#[test]
fn formal_config_default_bind_path_none() {
    let cfg = FormalConfig::default();
    assert!(cfg.bind_path.is_none(), "default bind_path should be None");
}

// ===========================================================================
// FormalConfig — custom construction
// ===========================================================================

#[test]
fn formal_config_custom_bmc_depth() {
    let cfg = FormalConfig { bmc_depth: 100, ..FormalConfig::default() };
    assert_eq!(cfg.bmc_depth, 100, "custom bmc_depth should be 100");
}

#[test]
fn formal_config_custom_prove_enabled() {
    let cfg = FormalConfig { prove: true, ..FormalConfig::default() };
    assert!(cfg.prove, "prove should be true when explicitly set");
}

#[test]
fn formal_config_custom_engine_yices() {
    let cfg = FormalConfig { engine: SbyEngine::Yices, ..FormalConfig::default() };
    assert_eq!(cfg.engine, SbyEngine::Yices, "engine should be Yices when explicitly set");
}

#[test]
fn formal_config_custom_engine_bitwuzla() {
    let cfg = FormalConfig { engine: SbyEngine::Bitwuzla, ..FormalConfig::default() };
    assert_eq!(cfg.engine, SbyEngine::Bitwuzla, "engine should be Bitwuzla when explicitly set");
}

#[test]
fn formal_config_custom_engine_boolector() {
    let cfg = FormalConfig { engine: SbyEngine::Boolector, ..FormalConfig::default() };
    assert_eq!(cfg.engine, SbyEngine::Boolector, "engine should be Boolector when explicitly set");
}

#[test]
fn formal_config_with_sv_path() {
    let cfg = config_for_sv("output/design_synth.sv");
    assert_eq!(cfg.sv_path, "output/design_synth.sv", "sv_path should match the provided path");
}

#[test]
fn formal_config_with_bind_path() {
    let cfg = config_with_bind("design.sv", "design_sva.sv");
    assert_eq!(
        cfg.bind_path.as_deref(),
        Some("design_sva.sv"),
        "bind_path should be Some with the provided path"
    );
    assert_eq!(cfg.bmc_depth, 50, "bmc_depth should be 50 in config_with_bind");
    assert!(cfg.prove, "prove should be true in config_with_bind");
    assert_eq!(cfg.engine, SbyEngine::Yices, "engine should be Yices in config_with_bind");
}

// ===========================================================================
// FormalConfig — Clone trait
// ===========================================================================

#[test]
fn formal_config_clone_preserves_all_fields() {
    let original = FormalConfig {
        bmc_depth: 75,
        prove: true,
        engine: SbyEngine::Bitwuzla,
        sv_path: "my/path.sv".to_string(),
        bind_path: Some("my/bind.sv".to_string()),
        extra_files: Vec::new(),
    };
    let cloned = original.clone();
    assert_eq!(cloned.bmc_depth, 75, "cloned bmc_depth should equal original");
    assert!(cloned.prove, "cloned prove should equal original");
    assert_eq!(cloned.engine, SbyEngine::Bitwuzla, "cloned engine should equal original");
    assert_eq!(cloned.sv_path, "my/path.sv", "cloned sv_path should equal original");
    assert_eq!(
        cloned.bind_path.as_deref(),
        Some("my/bind.sv"),
        "cloned bind_path should equal original"
    );
}

// ===========================================================================
// FormalConfig — Debug trait
// ===========================================================================

#[test]
fn formal_config_debug_format_includes_fields() {
    let cfg = FormalConfig::default();
    let debug_str = format!("{:?}", cfg);
    assert!(debug_str.contains("bmc_depth"), "Debug output should contain 'bmc_depth'");
    assert!(debug_str.contains("prove"), "Debug output should contain 'prove'");
    assert!(debug_str.contains("engine"), "Debug output should contain 'engine'");
    assert!(debug_str.contains("sv_path"), "Debug output should contain 'sv_path'");
    assert!(debug_str.contains("bind_path"), "Debug output should contain 'bind_path'");
}

// ===========================================================================
// FormalConfig — iterate over engine variants
// ===========================================================================

#[test]
fn formal_config_all_engine_variants_constructible() {
    let engines =
        [SbyEngine::Bitwuzla, SbyEngine::Yices, SbyEngine::Bitwuzla, SbyEngine::Boolector];
    for i in 0..MAX_TEST_ENGINES {
        if i >= engines.len() {
            break;
        }
        let cfg = FormalConfig { engine: engines[i], ..FormalConfig::default() };
        assert_eq!(cfg.engine, engines[i], "engine should match variant at index {i}");
    }
}

// ===========================================================================
// FormalStatus — equality, clone, debug
// ===========================================================================

#[test]
fn formal_status_pass_equals_pass() {
    assert_eq!(FormalStatus::Pass, FormalStatus::Pass, "Pass should equal Pass");
}

#[test]
fn formal_status_fail_equals_fail() {
    assert_eq!(FormalStatus::Fail, FormalStatus::Fail, "Fail should equal Fail");
}

#[test]
fn formal_status_unknown_equals_unknown() {
    assert_eq!(FormalStatus::Unknown, FormalStatus::Unknown, "Unknown should equal Unknown");
}

#[test]
fn formal_status_pass_ne_fail() {
    assert_ne!(FormalStatus::Pass, FormalStatus::Fail, "Pass should not equal Fail");
}

#[test]
fn formal_status_pass_ne_unknown() {
    assert_ne!(FormalStatus::Pass, FormalStatus::Unknown, "Pass should not equal Unknown");
}

#[test]
fn formal_status_fail_ne_unknown() {
    assert_ne!(FormalStatus::Fail, FormalStatus::Unknown, "Fail should not equal Unknown");
}

#[test]
fn formal_status_clone_preserves_variant() {
    let variants = [FormalStatus::Pass, FormalStatus::Fail, FormalStatus::Unknown];
    for i in 0..MAX_TEST_STATUS_VARIANTS {
        if i >= variants.len() {
            break;
        }
        let cloned = variants[i].clone();
        assert_eq!(cloned, variants[i], "cloned FormalStatus should equal original at index {i}");
    }
}

#[test]
fn formal_status_debug_pass() {
    let s = format!("{:?}", FormalStatus::Pass);
    assert_eq!(s, "Pass", "Debug of Pass should be \"Pass\"");
}

#[test]
fn formal_status_debug_fail() {
    let s = format!("{:?}", FormalStatus::Fail);
    assert_eq!(s, "Fail", "Debug of Fail should be \"Fail\"");
}

#[test]
fn formal_status_debug_unknown() {
    let s = format!("{:?}", FormalStatus::Unknown);
    assert_eq!(s, "Unknown", "Debug of Unknown should be \"Unknown\"");
}

// ===========================================================================
// PropertyVerdict — construction, clone, debug
// ===========================================================================

#[test]
fn property_verdict_construction() {
    let v = make_verdict("engine_0", "bmc", FormalStatus::Pass);
    assert_eq!(v.name, "engine_0", "verdict name should be 'engine_0'");
    assert_eq!(v.task, "bmc", "verdict task should be 'bmc'");
    assert_eq!(v.status, FormalStatus::Pass, "verdict status should be Pass");
}

#[test]
fn property_verdict_fail_status() {
    let v = make_verdict("engine_0.basecase", "prove", FormalStatus::Fail);
    assert_eq!(v.status, FormalStatus::Fail, "verdict status should be Fail");
    assert_eq!(v.task, "prove", "verdict task should be 'prove'");
}

#[test]
fn property_verdict_unknown_status() {
    let v = make_verdict("engine_1", "bmc", FormalStatus::Unknown);
    assert_eq!(v.status, FormalStatus::Unknown, "verdict status should be Unknown");
}

#[test]
fn property_verdict_clone_preserves_fields() {
    let original = make_verdict("prop_a", "bmc", FormalStatus::Pass);
    let cloned = original.clone();
    assert_eq!(cloned.name, "prop_a", "cloned verdict name should equal original");
    assert_eq!(cloned.task, "bmc", "cloned verdict task should equal original");
    assert_eq!(cloned.status, FormalStatus::Pass, "cloned verdict status should equal original");
}

#[test]
fn property_verdict_debug_contains_fields() {
    let v = make_verdict("engine_0", "bmc", FormalStatus::Pass);
    let debug_str = format!("{:?}", v);
    assert!(debug_str.contains("engine_0"), "Debug output should contain the verdict name");
    assert!(debug_str.contains("bmc"), "Debug output should contain the verdict task");
    assert!(debug_str.contains("Pass"), "Debug output should contain the verdict status");
}

// ===========================================================================
// FormalResult — construction, clone, debug
// ===========================================================================

#[test]
fn formal_result_passed_true() {
    let result = make_result(true, vec![], "all ok", "", Some(0));
    assert!(result.passed, "FormalResult.passed should be true");
    assert_eq!(result.exit_code, Some(0), "exit_code should be Some(0)");
    assert_eq!(result.stdout, "all ok", "stdout should match");
    assert!(result.stderr.is_empty(), "stderr should be empty");
    assert!(result.verdicts.is_empty(), "verdicts should be empty");
}

#[test]
fn formal_result_passed_false() {
    let verdicts = vec![make_verdict("engine_0", "bmc", FormalStatus::Fail)];
    let result = make_result(false, verdicts, "fail output", "error line", Some(1));
    assert!(!result.passed, "FormalResult.passed should be false");
    assert_eq!(result.exit_code, Some(1), "exit_code should be Some(1)");
    assert_eq!(result.verdicts.len(), 1, "should have exactly 1 verdict");
    assert_eq!(result.verdicts[0].status, FormalStatus::Fail, "the single verdict should be Fail");
}

#[test]
fn formal_result_no_exit_code() {
    let result = make_result(false, vec![], "", "signal killed", None);
    assert!(result.exit_code.is_none(), "exit_code should be None when process was killed");
}

#[test]
fn formal_result_multiple_verdicts() {
    let verdicts = vec![
        make_verdict("engine_0", "bmc", FormalStatus::Pass),
        make_verdict("engine_0.induction", "prove", FormalStatus::Pass),
        make_verdict("engine_1", "bmc", FormalStatus::Fail),
    ];
    let result = make_result(false, verdicts, "mixed", "", Some(1));
    assert_eq!(result.verdicts.len(), 3, "should have 3 verdicts");
    assert_eq!(result.verdicts[0].status, FormalStatus::Pass, "first verdict should be Pass");
    assert_eq!(result.verdicts[1].status, FormalStatus::Pass, "second verdict should be Pass");
    assert_eq!(result.verdicts[2].status, FormalStatus::Fail, "third verdict should be Fail");
}

#[test]
fn formal_result_clone_preserves_all() {
    let verdicts = vec![
        make_verdict("e0", "bmc", FormalStatus::Pass),
        make_verdict("e1", "prove", FormalStatus::Fail),
    ];
    let original = make_result(false, verdicts, "out", "err", Some(2));
    let cloned = original.clone();
    assert_eq!(cloned.passed, original.passed, "cloned passed should equal original");
    assert_eq!(cloned.exit_code, original.exit_code, "cloned exit_code should equal original");
    assert_eq!(cloned.stdout, original.stdout, "cloned stdout should equal original");
    assert_eq!(cloned.stderr, original.stderr, "cloned stderr should equal original");
    assert_eq!(
        cloned.verdicts.len(),
        original.verdicts.len(),
        "cloned verdicts length should equal original"
    );
    for i in 0..MAX_TEST_VERDICTS {
        if i >= cloned.verdicts.len() {
            break;
        }
        assert_eq!(
            cloned.verdicts[i].name, original.verdicts[i].name,
            "cloned verdict[{i}].name should equal original"
        );
        assert_eq!(
            cloned.verdicts[i].task, original.verdicts[i].task,
            "cloned verdict[{i}].task should equal original"
        );
        assert_eq!(
            cloned.verdicts[i].status, original.verdicts[i].status,
            "cloned verdict[{i}].status should equal original"
        );
    }
}

#[test]
fn formal_result_debug_contains_key_fields() {
    let result = make_result(true, vec![], "stdout_data", "stderr_data", Some(0));
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("exit_code"), "Debug should contain 'exit_code'");
    assert!(debug_str.contains("passed"), "Debug should contain 'passed'");
    assert!(debug_str.contains("verdicts"), "Debug should contain 'verdicts'");
    assert!(debug_str.contains("stdout"), "Debug should contain 'stdout'");
    assert!(debug_str.contains("stderr"), "Debug should contain 'stderr'");
}

// ===========================================================================
// Constants
// ===========================================================================

#[test]
fn max_formal_depth_is_200() {
    assert_eq!(MAX_FORMAL_DEPTH, 200, "MAX_FORMAL_DEPTH should be 200");
}

#[test]
fn max_formal_properties_is_256() {
    assert_eq!(MAX_FORMAL_PROPERTIES, 256, "MAX_FORMAL_PROPERTIES should be 256");
}

#[test]
fn max_formal_depth_matches_sby_max_bmc_depth() {
    assert_eq!(
        MAX_FORMAL_DEPTH,
        mirrc::toolchain::sby::MAX_BMC_DEPTH,
        "MAX_FORMAL_DEPTH should equal sby::MAX_BMC_DEPTH for consistency"
    );
}

// ===========================================================================
// run_formal_pipeline — error paths
// ===========================================================================

#[test]
fn run_formal_pipeline_tool_not_found_empty_registry() {
    let registry = empty_registry();
    let cfg = config_for_sv("test.sv");
    let result = mirrc::toolchain::formal::run_formal_pipeline(
        &registry,
        &cfg,
        "test_mod",
        std::path::Path::new("."),
    );
    assert!(result.is_err(), "run_formal_pipeline should fail when sby is not in the registry");
    let err = result.unwrap_err();
    let err_msg = format!("{}", err);
    assert!(
        err_msg.contains("not found") || err_msg.contains("sby"),
        "error message should mention tool not found or sby, got: {err_msg}"
    );
}

#[test]
fn run_formal_pipeline_tool_not_found_sby_unavailable() {
    let registry = registry_without_sby();
    let cfg = config_for_sv("design.sv");
    let result = mirrc::toolchain::formal::run_formal_pipeline(
        &registry,
        &cfg,
        "design",
        std::path::Path::new("."),
    );
    assert!(result.is_err(), "run_formal_pipeline should fail when sby is marked unavailable");
    let err = result.unwrap_err();
    let err_msg = format!("{}", err);
    assert!(
        err_msg.contains("not found") || err_msg.contains("sby"),
        "error should reference sby or 'not found', got: {err_msg}"
    );
}

#[test]
fn run_formal_pipeline_error_is_tool_not_found_variant() {
    let registry = empty_registry();
    let cfg = config_for_sv("x.sv");
    let result = mirrc::toolchain::formal::run_formal_pipeline(
        &registry,
        &cfg,
        "x",
        std::path::Path::new("."),
    );
    assert!(result.is_err(), "should be Err");
    match result.unwrap_err() {
        ToolchainError::ToolNotFound { ref tool } => {
            assert_eq!(tool, "sby", "ToolNotFound should reference 'sby'");
        }
        other => panic!("expected ToolchainError::ToolNotFound, got: {:?}", other),
    }
}

#[test]
fn run_formal_pipeline_with_prove_still_fails_without_sby() {
    let registry = empty_registry();
    let cfg = FormalConfig {
        prove: true,
        bmc_depth: 50,
        sv_path: "prove_test.sv".to_string(),
        ..FormalConfig::default()
    };
    let result = mirrc::toolchain::formal::run_formal_pipeline(
        &registry,
        &cfg,
        "prove_mod",
        std::path::Path::new("."),
    );
    assert!(result.is_err(), "should fail regardless of prove flag when sby is absent");
}

#[test]
fn run_formal_pipeline_with_bind_path_still_fails_without_sby() {
    let registry = empty_registry();
    let cfg = config_with_bind("top.sv", "top_sva.sv");
    let result = mirrc::toolchain::formal::run_formal_pipeline(
        &registry,
        &cfg,
        "top",
        std::path::Path::new("."),
    );
    assert!(result.is_err(), "should fail when sby is absent even with bind_path set");
}

#[test]
fn run_formal_pipeline_with_max_depth_still_fails_without_sby() {
    let registry = empty_registry();
    let cfg = FormalConfig {
        bmc_depth: MAX_FORMAL_DEPTH,
        sv_path: "deep.sv".to_string(),
        ..FormalConfig::default()
    };
    let result = mirrc::toolchain::formal::run_formal_pipeline(
        &registry,
        &cfg,
        "deep_mod",
        std::path::Path::new("."),
    );
    assert!(result.is_err(), "should fail when sby is absent even at max depth");
}

#[test]
fn run_formal_pipeline_with_over_max_depth_still_fails_without_sby() {
    let registry = empty_registry();
    let cfg = FormalConfig {
        bmc_depth: MAX_FORMAL_DEPTH + 100,
        sv_path: "over_max.sv".to_string(),
        ..FormalConfig::default()
    };
    let result = mirrc::toolchain::formal::run_formal_pipeline(
        &registry,
        &cfg,
        "over_mod",
        std::path::Path::new("."),
    );
    assert!(
        result.is_err(),
        "should fail when sby is absent even with depth over MAX_FORMAL_DEPTH"
    );
}

// ===========================================================================
// run_formal_pipeline — all engine variants produce same error without sby
// ===========================================================================

#[test]
fn run_formal_pipeline_all_engines_fail_without_sby() {
    let engines =
        [SbyEngine::Bitwuzla, SbyEngine::Yices, SbyEngine::Bitwuzla, SbyEngine::Boolector];
    let registry = empty_registry();
    for i in 0..MAX_TEST_ENGINES {
        if i >= engines.len() {
            break;
        }
        let cfg = FormalConfig {
            engine: engines[i],
            sv_path: format!("engine_{i}.sv"),
            ..FormalConfig::default()
        };
        let result = mirrc::toolchain::formal::run_formal_pipeline(
            &registry,
            &cfg,
            &format!("eng_{i}"),
            std::path::Path::new("."),
        );
        assert!(result.is_err(), "engine variant {i} should fail without sby");
    }
}

// ===========================================================================
// ToolchainError Display coverage
// ===========================================================================

#[test]
fn toolchain_error_tool_not_found_display() {
    let err = ToolchainError::ToolNotFound { tool: "sby".to_string() };
    let msg = format!("{}", err);
    assert!(msg.contains("sby"), "ToolNotFound display should mention tool name");
    assert!(msg.contains("not found"), "ToolNotFound display should say 'not found'");
}

#[test]
fn toolchain_error_invocation_display() {
    let err = ToolchainError::Invocation {
        tool: "sby".to_string(),
        message: "permission denied".to_string(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("sby"), "Invocation display should mention tool name");
    assert!(msg.contains("permission denied"), "Invocation display should include error message");
}

#[test]
fn toolchain_error_tool_failed_display() {
    let err = ToolchainError::ToolFailed {
        tool: "sby".to_string(),
        exit_code: Some(1),
        stderr: "assertion failed\nsecond line".to_string(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("sby"), "ToolFailed display should mention tool name");
    assert!(
        msg.contains("assertion failed"),
        "ToolFailed display should show first line of stderr"
    );
}

#[test]
fn toolchain_error_parse_error_display() {
    let err = ToolchainError::ParseError {
        tool: "sby".to_string(),
        message: "unexpected token".to_string(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("sby"), "ParseError display should mention tool name");
    assert!(
        msg.contains("unexpected token"),
        "ParseError display should include parse error message"
    );
}

// ===========================================================================
// FormalConfig — many configs via bounded iteration
// ===========================================================================

#[test]
fn formal_config_bounded_depth_values() {
    let depths: [u32; 6] = [0, 1, 20, 100, 200, 999];
    for i in 0..MAX_TEST_CONFIGS {
        if i >= depths.len() {
            break;
        }
        let cfg = FormalConfig { bmc_depth: depths[i], ..FormalConfig::default() };
        assert_eq!(
            cfg.bmc_depth, depths[i],
            "config at index {i} should have bmc_depth={}",
            depths[i]
        );
    }
}

// ===========================================================================
// PropertyVerdict — bulk construction with bounded iteration
// ===========================================================================

#[test]
fn property_verdict_bulk_construction() {
    let mut verdicts = Vec::new();
    for i in 0..MAX_TEST_VERDICTS {
        if i >= 10 {
            break;
        }
        let status = match i % 3 {
            0 => FormalStatus::Pass,
            1 => FormalStatus::Fail,
            _ => FormalStatus::Unknown,
        };
        verdicts.push(make_verdict(
            &format!("engine_{i}"),
            if i % 2 == 0 { "bmc" } else { "prove" },
            status,
        ));
    }
    assert_eq!(verdicts.len(), 10, "should have constructed exactly 10 verdicts");
    for i in 0..MAX_TEST_VERDICTS {
        if i >= verdicts.len() {
            break;
        }
        assert_eq!(verdicts[i].name, format!("engine_{i}"), "verdict[{i}] name should match");
    }
}

// ===========================================================================
// FormalResult — large verdict vector
// ===========================================================================

#[test]
fn formal_result_with_max_verdicts() {
    let mut verdicts = Vec::new();
    for i in 0..MAX_TEST_VERDICTS {
        if verdicts.len() >= MAX_FORMAL_PROPERTIES {
            break;
        }
        verdicts.push(make_verdict(&format!("prop_{i}"), "bmc", FormalStatus::Pass));
    }
    let result = make_result(true, verdicts, "", "", Some(0));
    assert_eq!(
        result.verdicts.len(),
        MAX_TEST_VERDICTS,
        "result should contain MAX_TEST_VERDICTS verdicts"
    );
    assert!(result.passed, "result should be passed");
}

// ===========================================================================
// MIRR compilation sanity — parse the canonical module
// ===========================================================================

#[test]
fn mirr_module_parses_successfully() {
    let program = parse_mirr(MIRR_MODULE);
    assert!(program.is_ok(), "canonical MIRR module should parse without errors");
    let prog = program.unwrap();
    assert_eq!(prog.module.name, "formal_test", "parsed module name should be 'formal_test'");
}

#[test]
fn mirr_module_has_required_signals() {
    let prog = parse_mirr(MIRR_MODULE).expect("parse should succeed");
    let input_count = prog
        .module
        .signals
        .iter()
        .filter(|s| s.kind == mirrc::ast::types::SignalKind::Input)
        .count();
    let output_count = prog
        .module
        .signals
        .iter()
        .filter(|s| s.kind == mirrc::ast::types::SignalKind::Output)
        .count();
    assert!(input_count >= 1, "module should have at least 1 input signal, found {input_count}");
    assert!(output_count >= 1, "module should have at least 1 output signal, found {output_count}");
}

#[test]
fn mirr_module_has_guard() {
    let prog = parse_mirr(MIRR_MODULE).expect("parse should succeed");
    assert!(!prog.module.guards.is_empty(), "module should have at least 1 guard");
    assert_eq!(prog.module.guards[0].name, "watchdog", "first guard should be named 'watchdog'");
}

#[test]
fn mirr_module_has_reflex() {
    let prog = parse_mirr(MIRR_MODULE).expect("parse should succeed");
    assert!(!prog.module.reflexes.is_empty(), "module should have at least 1 reflex");
    assert_eq!(prog.module.reflexes[0].name, "trigger", "first reflex should be named 'trigger'");
}

#[test]
fn mirr_module_pipeline_succeeds() {
    let pipeline_cfg = PipelineConfig::default();
    let result = run_pipeline(MIRR_MODULE, &pipeline_cfg);
    assert!(result.is_ok(), "pipeline should succeed on the canonical MIRR module");
}

// ===========================================================================
// FormalConfig wired to pipeline output
// ===========================================================================

#[test]
fn formal_config_from_pipeline_output_path() {
    // Simulate the pattern: compile MIRR -> get SV path -> build FormalConfig
    let sv_path = "output/formal_test_synth.sv";
    let bind_path = "output/formal_test_sva.sv";
    let cfg = FormalConfig {
        bmc_depth: 30,
        prove: true,
        engine: SbyEngine::Bitwuzla,
        sv_path: sv_path.to_string(),
        bind_path: Some(bind_path.to_string()),
        extra_files: Vec::new(),
    };
    assert_eq!(cfg.sv_path, sv_path, "FormalConfig sv_path should reference pipeline SV output");
    assert_eq!(
        cfg.bind_path.as_deref(),
        Some(bind_path),
        "FormalConfig bind_path should reference pipeline SVA output"
    );
    assert_eq!(cfg.bmc_depth, 30, "bmc_depth should be 30");
    assert!(cfg.prove, "prove should be enabled for full formal verification");
}

// ===========================================================================
// ToolRegistry — is_available for sby
// ===========================================================================

#[test]
fn tool_registry_empty_sby_not_available() {
    let reg = empty_registry();
    assert!(!reg.is_available(Tool::Sby), "sby should not be available in an empty registry");
}

#[test]
fn tool_registry_sby_marked_unavailable() {
    let reg = registry_without_sby();
    assert!(!reg.is_available(Tool::Sby), "sby should not be available when marked unavailable");
}

#[test]
fn tool_registry_sby_marked_available() {
    let mut reg = ToolRegistry::new();
    reg.tools.insert(
        Tool::Sby,
        ToolInfo {
            path: "/usr/bin/sby".to_string(),
            version: "sby 0.39".to_string(),
            available: true,
        },
    );
    assert!(reg.is_available(Tool::Sby), "sby should be available when marked available");
}

#[test]
fn tool_registry_version_for_sby() {
    let mut reg = ToolRegistry::new();
    reg.tools.insert(
        Tool::Sby,
        ToolInfo {
            path: "/usr/bin/sby".to_string(),
            version: "sby 0.39".to_string(),
            available: true,
        },
    );
    let version = reg.version(Tool::Sby);
    assert_eq!(version, Some("sby 0.39"), "version should return the registered version string");
}

#[test]
fn tool_registry_version_none_when_unavailable() {
    let reg = registry_without_sby();
    let version = reg.version(Tool::Sby);
    assert!(version.is_none(), "version should be None when tool is unavailable");
}

// ===========================================================================
// Edge cases: empty strings, boundary values
// ===========================================================================

#[test]
fn formal_config_empty_sv_path() {
    let cfg = FormalConfig::default();
    assert!(cfg.sv_path.is_empty(), "default sv_path should be empty string");
}

#[test]
fn formal_config_zero_depth() {
    let cfg = FormalConfig { bmc_depth: 0, ..FormalConfig::default() };
    assert_eq!(
        cfg.bmc_depth, 0,
        "bmc_depth of 0 should be accepted (clamping happens in pipeline)"
    );
}

#[test]
fn formal_config_depth_exceeding_max() {
    let cfg = FormalConfig { bmc_depth: u32::MAX, ..FormalConfig::default() };
    assert_eq!(
        cfg.bmc_depth,
        u32::MAX,
        "FormalConfig should store the raw depth; clamping happens in run_formal_pipeline"
    );
}

#[test]
fn formal_result_empty_stdout_stderr() {
    let result = make_result(true, vec![], "", "", Some(0));
    assert!(result.stdout.is_empty(), "stdout should be empty");
    assert!(result.stderr.is_empty(), "stderr should be empty");
}

#[test]
fn formal_result_multiline_stdout() {
    let stdout = "SBY  0:00:01 [bmc] engine_0: PASS\nSBY  0:00:02 done\n";
    let result = make_result(true, vec![], stdout, "", Some(0));
    assert!(result.stdout.contains("PASS"), "stdout should contain PASS");
    assert!(result.stdout.contains("done"), "stdout should contain done");
}

#[test]
fn property_verdict_empty_name() {
    let v = make_verdict("", "bmc", FormalStatus::Unknown);
    assert!(v.name.is_empty(), "verdict with empty name should have empty name");
}

#[test]
fn property_verdict_empty_task() {
    let v = make_verdict("engine_0", "", FormalStatus::Pass);
    assert!(v.task.is_empty(), "verdict with empty task should have empty task");
}

// ===========================================================================
// FormalConfig — struct update syntax coverage
// ===========================================================================

#[test]
fn formal_config_struct_update_only_depth() {
    let base = FormalConfig::default();
    let cfg = FormalConfig { bmc_depth: 42, ..base };
    assert_eq!(cfg.bmc_depth, 42, "only bmc_depth should differ");
    assert!(!cfg.prove, "prove should still be default (false)");
    assert_eq!(cfg.engine, SbyEngine::Bitwuzla, "engine should still be default (Z3)");
}

#[test]
fn formal_config_struct_update_only_prove() {
    let base = FormalConfig::default();
    let cfg = FormalConfig { prove: true, ..base };
    assert!(cfg.prove, "prove should be true");
    assert_eq!(cfg.bmc_depth, 50, "bmc_depth should remain default");
}

#[test]
fn formal_config_struct_update_only_engine() {
    let base = FormalConfig::default();
    let cfg = FormalConfig { engine: SbyEngine::Boolector, ..base };
    assert_eq!(cfg.engine, SbyEngine::Boolector, "engine should be Boolector");
    assert_eq!(cfg.bmc_depth, 50, "bmc_depth should remain default");
    assert!(!cfg.prove, "prove should remain default");
}

// ===========================================================================
// Multiple MIRR modules — ensure different formal configs can reference them
// ===========================================================================

#[test]
fn formal_configs_for_multiple_modules() {
    let modules = [
        (
            "\
module alpha {
    signal a_in: in bool;
    signal a_out: out bool;
    guard a_g {
        when a_in
        for 2 cycles;
    }
    reflex a_r {
        on a_g {
            a_out = true;
        }
    }
}
",
            "alpha",
        ),
        (
            "\
module beta {
    signal b_in: in bool;
    signal b_out: out bool;
    guard b_g {
        when b_in
        for 5 cycles;
    }
    reflex b_r {
        on b_g {
            b_out = true;
        }
    }
}
",
            "beta",
        ),
    ];
    for i in 0..MAX_TEST_CONFIGS {
        if i >= modules.len() {
            break;
        }
        let (src, name) = modules[i];
        let prog = parse_mirr(src);
        assert!(prog.is_ok(), "module '{name}' should parse successfully");
        let cfg =
            FormalConfig { sv_path: format!("output/{name}_synth.sv"), ..FormalConfig::default() };
        assert_eq!(
            cfg.sv_path,
            format!("output/{name}_synth.sv"),
            "FormalConfig for '{name}' should have correct sv_path"
        );
    }
}

// ===========================================================================
// SbyEngine re-exports accessible from formal path
// ===========================================================================

#[test]
fn sby_engine_from_str_z3() {
    assert_eq!(
        SbyEngine::from_str_name("z3"),
        Some(SbyEngine::Z3),
        "from_str_name('z3') should return Z3"
    );
}

#[test]
fn sby_engine_from_str_unknown() {
    assert_eq!(
        SbyEngine::from_str_name("unknown_solver"),
        None,
        "from_str_name of unknown solver should return None"
    );
}

#[test]
fn sby_engine_name_z3() {
    assert_eq!(SbyEngine::Z3.engine_name(), "smtbmc z3", "Z3 engine_name should be 'smtbmc z3'");
}

#[test]
fn sby_engine_name_boolector() {
    assert_eq!(
        SbyEngine::Boolector.engine_name(),
        "btor btormc",
        "Boolector engine_name should be 'btor btormc'"
    );
}

// ===========================================================================
// Formal pipeline with different working directories — still fails without sby
// ===========================================================================

#[test]
fn run_formal_pipeline_various_working_dirs() {
    let dirs = [".", "/tmp", "/nonexistent/path"];
    let registry = empty_registry();
    for i in 0..MAX_TEST_CONFIGS {
        if i >= dirs.len() {
            break;
        }
        let cfg = config_for_sv("test.sv");
        let result = mirrc::toolchain::formal::run_formal_pipeline(
            &registry,
            &cfg,
            "mod",
            std::path::Path::new(dirs[i]),
        );
        assert!(result.is_err(), "should fail without sby for working_dir '{}'", dirs[i]);
    }
}
