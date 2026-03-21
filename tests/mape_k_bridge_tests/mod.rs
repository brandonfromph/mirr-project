#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

//! Integration tests for the MAPE-K bridge module (`src/mape_k/bridge.rs`).
//!
//! Validates signal/property lowering from `PipelineResult` to `SimConfig`,
//! including sensor extraction, property lowering, action table generation,
//! and error handling for unsupported formulas and resource limits.

use nasa_rust_project::ast::program::{MirrProgram, Module};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::ast::{Expr, SignalDecl};
use nasa_rust_project::mape_k::bridge::{
    bridge_from_pipeline, BridgeError, DEFAULT_KNOWLEDGE_CAPACITY, DEFAULT_WINDOW_SIZE,
    MAX_BRIDGE_PROPERTIES, MAX_BRIDGE_SIGNALS,
};
use nasa_rust_project::mape_k::planner::{AdaptationAction, TriggerCondition};
use nasa_rust_project::mape_k::{SignalPredicate, TemporalProperty};
use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::pipeline::PipelineResult;

// ---------------------------------------------------------------------------
// Constants — bounded iteration limits (NASA P10)
// ---------------------------------------------------------------------------

const MAX_TEST_SENSORS: usize = 64;
const MAX_TEST_ACTION_ENTRIES: usize = 64;

/// PRNG seed base used by the bridge (mirrors bridge.rs constant).
const SEED_BASE: u64 = 1000;

// ---------------------------------------------------------------------------
// Helpers — no recursion, bounded iteration
// ---------------------------------------------------------------------------

/// Build a minimal `PipelineResult` with the given signals and properties.
fn stub_pipeline(signals: Vec<SignalDecl>, properties: Vec<PropertyDecl>) -> PipelineResult {
    let module = Module {
        name: "test_mod".to_string(),
        signals,
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties,
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    PipelineResult {
        program: MirrProgram { patterns: Vec::new(), module },
        simplify_stats: None,
        width_result: None,
        temporal_netlist: None,
        rspu_program: None,
        type_map: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        sat_stats: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
        hls_result: None,
    }
}

/// Parse a MIRR source string into a `PipelineResult` suitable for bridge testing.
fn parse_to_pipeline(source: &str) -> PipelineResult {
    let program = parse_mirr(source).expect("MIRR parse should succeed");
    PipelineResult {
        program,
        simplify_stats: None,
        width_result: None,
        temporal_netlist: None,
        rspu_program: None,
        type_map: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        sat_stats: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
        hls_result: None,
    }
}

fn input_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn output_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Output,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn internal_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn assert_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Assert,
        formula,
        origin: None,
        span: None,
    }
}

fn cover_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Cover,
        formula,
        origin: None,
        span: None,
    }
}

fn assume_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Assume,
        formula,
        origin: None,
        span: None,
    }
}

// ---------------------------------------------------------------------------
// 1. Sensor extraction — through parser
// ---------------------------------------------------------------------------


mod part1;
mod part2;
mod part3;


