//! MEGA-4 Subsystem Verification Test Suite — Totality Engine + Proof Certificates.
//!
//! NASA-style verification tests for the MIRR totality engine (5 analyses),
//! proof certificate format (serialize/deserialize), and pipeline integration.
//!
//! Covers:
//! - F1: Resource bounds analysis (check_resource_bounds)
//! - F2: Output completeness (check_output_completeness)
//! - F3: Guard coverage (check_guard_coverage)
//! - F4: Temporal bound (check_temporal_bound)
//! - F5: Dependency acyclicity (check_dependency_acyclicity)
//! - F6: Aggregate totality (run_totality_check — all 5 pass)
//! - F7: Property summary (build_property_summary)
//! - F8: Proof certificate serialize/deserialize roundtrip
//! - F9: TerminationStrategy variants
//! - F10: Pipeline integration (totality flag in PipelineConfig)
//! - F11: TotalityError variant in MirrError
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::cert::{ProofCertificate, TerminationStrategy};
use nasa_rust_project::emit::rspu_isa::{MAX_GUARDS, MAX_INSTRUCTIONS, MAX_REGISTERS};
use nasa_rust_project::error::MirrError;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::totality::run_totality_check;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum test iterations in any bounded loop.
const _MAX_TEST_ITERATIONS: usize = 256;

// ---------------------------------------------------------------------------
// AST Helpers — build Module directly for unit tests
// ---------------------------------------------------------------------------

fn make_signal(name: &str, kind: SignalKind) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(SignalType::Bool),
        origin: None,
        span: None,
    }
}

fn _make_signal_u16(name: &str, kind: SignalKind) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(SignalType::Unsigned(16)),
        origin: None,
        span: None,
    }
}

fn make_guard(name: &str, cycles: u64) -> Guard {
    Guard {
        name: name.to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles,
        template_cycles: None,
        origin: None,
        span: None,
    }
}

fn _make_guard_on_signal(name: &str, signal: &str, cycles: u64) -> Guard {
    Guard {
        name: name.to_string(),
        condition: Expr::Signal(signal.to_string()),
        cycles,
        template_cycles: None,
        origin: None,
        span: None,
    }
}

fn make_reflex(name: &str, guard: &str, target: &str) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![guard.to_string()],
        assignments: vec![Assignment {
            target: target.to_string(),
            value: Expr::Literal(LiteralValue::Bool(true)),
            span: None,
        }],
        origin: None,
        span: None,
    }
}

fn make_reflex_with_expr(name: &str, guard: &str, target: &str, value: Expr) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![guard.to_string()],
        assignments: vec![Assignment { target: target.to_string(), value, span: None }],
        origin: None,
        span: None,
    }
}

fn make_module(signals: Vec<SignalDecl>, guards: Vec<Guard>, reflexes: Vec<Reflex>) -> Module {
    Module {
        name: "test".to_string(),
        signals,
        guards,
        reflexes,
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

fn make_module_with_properties(
    signals: Vec<SignalDecl>,
    guards: Vec<Guard>,
    reflexes: Vec<Reflex>,
    properties: Vec<PropertyDecl>,
) -> Module {
    Module {
        name: "test".to_string(),
        signals,
        guards,
        reflexes,
        properties,
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

/// Shorthand: a well-formed total module with 1 input, 1 output, 1 guard, 1 reflex.
fn total_module() -> Module {
    make_module(
        vec![
            make_signal("input_a", SignalKind::Input),
            make_signal("output_b", SignalKind::Output),
        ],
        vec![make_guard("g1", 3)],
        vec![make_reflex("r1", "g1", "output_b")],
    )
}

// ===========================================================================
// F1: Resource bounds analysis
// ===========================================================================

mod sub1;
mod sub2;
mod sub3;
