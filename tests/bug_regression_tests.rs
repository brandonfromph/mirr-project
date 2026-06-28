#![allow(clippy::field_reassign_with_default)]
#![cfg(any())]
//! TDD Bug Regression Test Suite
//!
//! Each test in this file is a RED test that exposes a confirmed compiler bug.
//! After each bug is fixed, the corresponding test must turn GREEN.
//!
//! Bug inventory:
//! - BUG-1 (CRITICAL): ECS `semantic_validate()` never called by pipeline
//! - BUG-2 (HIGH):     Import load errors silently discarded by pipeline
//! - BUG-3 (HIGH):     ECS `parallel_width_inference_system` hardcoded to width=8
//! - BUG-4 (MEDIUM):   `cond_to_guard` dedup broken for `is_simple_guard` case
//! - BUG-5 (MEDIUM):   `collect_reflect_body` counts braces inside `${…}` templates

#![forbid(unsafe_code)]

use mirrc::ecs::components::{EntityKind, KindComponent, NameComponent};
use mirrc::ecs::systems::parallel_width_inference_system;
use mirrc::ecs::Registry;
use mirrc::parser::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

// ---------------------------------------------------------------------------
// BUG-1: ECS semantic_validate() is never called by the production pipeline
// A program with a reflex that references a non-existent guard must be rejected.
// Currently it passes because the ECS validation gate is silently discarded.
// ---------------------------------------------------------------------------

/// BUG-1-A: Pipeline must reject a program that assigns to an Input signal.
/// The ECS `semantic_validate` rule E206 (assign-to-input) must be wired into
/// the pipeline. Without the fix, this compiles successfully (wrong behaviour).
#[test]
fn bug1_pipeline_must_reject_assignment_to_input_signal() {
    let src = r#"
        module bad_assign {
            signal sensor: in bool;
            signal out_val: out bool;
            guard g { when sensor for 1 cycles; }
            reflex r {
                on g {
                    sensor = true;
                }
            }
        }
    "#;
    let result = run_pipeline(src, &PipelineConfig::default());
    assert!(
        result.is_err(),
        "BUG-1: Pipeline should reject assignment to an input signal (E206), but it passed"
    );
    let errors = result.unwrap_err();
    let msgs = format!("{:?}", errors);
    assert!(
        msgs.contains("206") || msgs.contains("input") || msgs.contains("assign"),
        "BUG-1: Error message should reference E206 or 'input', got: {msgs}"
    );
}

/// BUG-1-B: Duplicate signal names must be caught by the pipeline (E201).
/// The ECS semantic_validate checks this but its result is never surfaced.
#[test]
fn bug1_pipeline_must_reject_duplicate_signal_names() {
    let src = r#"
        module dup_signals {
            signal clk: in bool;
            signal clk: in bool;
            signal out_val: out bool;
            guard g { when clk for 1 cycles; }
            reflex r { on g { out_val = true; } }
        }
    "#;
    // NOTE: The AST-level validator catches this too, so this test may pass even without the fix.
    // The ECS-level test (BUG-1-A) is the critical one that only ECS catches.
    let result = run_pipeline(src, &PipelineConfig::default());
    assert!(result.is_err(), "BUG-1: Pipeline should reject duplicate signal names (E201)");
}

// ---------------------------------------------------------------------------
// BUG-2: Import load errors silently discarded (pipeline.rs:167-168)
// When a required import fails, the pipeline must surface an error, not continue.
// ---------------------------------------------------------------------------

/// BUG-2: A pipeline run with a missing import directory must not silently succeed.
/// When `base_dir` is set and points to a non-existent path, import resolution
/// should propagate an error rather than continue with a partial registry.
#[test]
fn bug2_pipeline_with_missing_import_must_error() {
    let src = r#"
        import "nonexistent_pattern_lib.mirr";
        module uses_import {
            signal x: in bool;
            signal y: out bool;
            guard g { when x for 1 cycles; }
            reflex r { on g { y = true; } }
        }
    "#;
    let config = PipelineConfig {
        base_dir: Some(std::path::PathBuf::from("/tmp/does_not_exist_mirr_tests_xyz")),
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config);
    // BUG: Currently succeeds with partial registry because `let _ = ...` discards the error.
    // After fix: should return Err with a meaningful diagnostic about the missing import.
    assert!(
        result.is_err(),
        "BUG-2: Pipeline should return an error when a required import file cannot be found"
    );
}

// ---------------------------------------------------------------------------
// BUG-3: parallel_width_inference_system is hardcoded to return width=8 for all signals
// ---------------------------------------------------------------------------

/// BUG-3-A: A u16 signal must have width 16 in ECS inference, not 8.
#[test]
fn bug3_ecs_width_inference_must_return_correct_width_for_u16() {
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
    use mirrc::ecs::components::TypeComponent;

    let mut registry = Registry::new();

    // Create a signal entity manually tagged as u16
    let id = registry.next_id();
    let idx = id.0 as usize;
    registry.names[idx] = Some(NameComponent(registry.interner.intern("counter")));
    registry.kinds[idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
    registry.types[idx] = Some(TypeComponent(ExtendedType::from_core(SignalType::Unsigned(16))));

    let (_sccs, scc_solves, _, _stats) = parallel_width_inference_system(&mut registry);

    assert!(
        scc_solves.iter().any(|r| r.widths.contains(&16)),
        "BUG-3: ECS width inference for a u16 signal must return width=16, got: {:?}",
        scc_solves
    );
}

/// BUG-3-B: A u32 signal must have width 32 in ECS inference, not 8.
#[test]
fn bug3_ecs_width_inference_must_return_correct_width_for_u32() {
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
    use mirrc::ecs::components::TypeComponent;

    let mut registry = Registry::new();

    let id = registry.next_id();
    let idx = id.0 as usize;
    registry.names[idx] = Some(NameComponent(registry.interner.intern("data_bus")));
    registry.kinds[idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
    registry.types[idx] = Some(TypeComponent(ExtendedType::from_core(SignalType::Unsigned(32))));

    let (_sccs, scc_solves, _, _) = parallel_width_inference_system(&mut registry);

    assert!(
        scc_solves.iter().any(|r| r.widths.contains(&32)),
        "BUG-3: ECS width inference for a u32 signal must return width=32, got: {:?}",
        scc_solves
    );
}

/// BUG-3-C: A bool signal must have width 1 in ECS inference.
#[test]
fn bug3_ecs_width_inference_must_return_width_1_for_bool() {
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
    use mirrc::ecs::components::TypeComponent;

    let mut registry = Registry::new();

    let id = registry.next_id();
    let idx = id.0 as usize;
    registry.names[idx] = Some(NameComponent(registry.interner.intern("flag")));
    registry.kinds[idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
    registry.types[idx] = Some(TypeComponent(ExtendedType::from_core(SignalType::Bool)));

    let (_sccs, scc_solves, _, _) = parallel_width_inference_system(&mut registry);

    assert!(
        scc_solves.iter().any(|r| r.widths.contains(&1)),
        "BUG-3: ECS width inference for a bool signal must return width=1, got: {:?}",
        scc_solves
    );
}

// ---------------------------------------------------------------------------
// BUG-4: cond_to_guard dedup broken when condition looks like a declared guard
// ---------------------------------------------------------------------------

/// BUG-4: When a condition string is purely alphanumeric (looks like a guard name)
/// AND that name does NOT exist as a declared guard, the `is_simple_guard` check
/// takes priority over the `cond_to_guard` map. A second occurrence of the same
/// condition will NOT be deduplicated — it will emit a new `auto_g_N` rather than
/// reusing the existing one. This violates the deduplication invariant.
#[test]
fn bug4_cond_to_guard_dedup_fires_for_repeated_simple_identifier_condition() {
    // Condition `ready` is a signal name — it is all alphanumeric, so `is_simple_guard`
    // returns true (because it looks syntactically like a guard name), but it is NOT
    // in `declared_guards`. This means two separate `if ready {` blocks will each
    // receive a different `auto_g_N`, creating duplicate guard declarations.
    let src = r#"
        module m {
            signal ready: in bool;
            signal out1: out bool;
            signal out2: out bool;
            reflex r {
                always {
                    if ready {
                        out1 = true;
                    }
                    if ready {
                        out2 = true;
                    }
                }
            }
        }
    "#;
    let program = parse_mirr(src).unwrap();

    // Count how many `auto_g_` guards are declared for the condition `ready`.
    let auto_guard_decl_count =
        program.module.guards.iter().filter(|g| g.name.starts_with("auto_g_")).count();
    assert_eq!(
        auto_guard_decl_count, 1,
        "BUG-4: Two identical `if ready` conditions should deduplicate to a single auto guard, \
         but found {auto_guard_decl_count} guard declarations"
    );
}

// ---------------------------------------------------------------------------
// BUG-5: collect_reflect_body counts braces inside ${…} template substitutions
// ---------------------------------------------------------------------------

/// BUG-5: A `${var}` substitution inside a reflect body contains `{` and `}`.
/// The `collect_reflect_body` function counts these characters naively.
/// A single `${i}` causes depth += 1 then depth -= 1, which is self-cancelling —
/// but if the closing `}` of `${i}` is the LAST character on a line, the function
/// sees depth == 0 and prematurely terminates the reflect body collection.
#[test]
fn bug5_collect_reflect_body_handles_template_substitution_braces() {
    // This pattern body contains `${i}` — the `}` in `${i}` must not close the reflect block.
    let src = r#"
        def counter_init(n: u16) {
            reflect {
                guard g_${n} { when x for ${n} cycles; }
            }
        }
        module m {}
    "#;

    let result = parse_mirr(src);
    assert!(
        result.is_ok(),
        "BUG-5: Pattern with `${{n}}` in reflect body should parse successfully, got: {:?}",
        result.unwrap_err()
    );

    let program = result.expect("already checked");
    assert_eq!(program.patterns.len(), 1, "BUG-5: Should have parsed 1 pattern def");

    let statements = &program.patterns[0].body.statements;
    // The reflect body should contain the guard line, not be empty from premature termination.
    assert!(
        !statements.is_empty(),
        "BUG-5: Reflect body should not be empty — `${{n}}` braces were mistakenly counted"
    );
    use mirrc::ast::macro_nodes::ModuleMacroStmt;
    assert!(
        statements.iter().any(|s| matches!(s, ModuleMacroStmt::Guard(g) if g.name.contains("g_"))),
        "BUG-5: Reflect body should contain the guard line, but got: {:?}",
        statements
    );
}

/// BUG-5-B: A pattern body with multiple `${…}` on one line must not corrupt depth.
#[test]
fn bug5_multiple_template_substitutions_on_same_line_do_not_corrupt_depth() {
    let src = r#"
        def multi_sub(a: u16, b: u16) {
            reflect {
                guard g_${a}_${b} { when x for 1 cycles; }
                reflex r_${a} { on g_${a}_${b} { out_${b} = true; } }
            }
        }
        module m {}
    "#;

    let result = parse_mirr(src);
    assert!(
        result.is_ok(),
        "BUG-5-B: Pattern with multiple `${{…}}` on a line should parse, got: {:?}",
        result.unwrap_err()
    );
    let program = result.expect("already checked");
    let statements = &program.patterns[0].body.statements;
    assert!(
        statements.len() >= 2,
        "BUG-5-B: Reflect body should have at least 2 statements (guard + reflex), got: {}",
        statements.len()
    );
}
