#![forbid(unsafe_code)]
//! Extended SystemVerilog emitter tests.
//!
//! Comprehensive coverage of `src/emit/verilog.rs` (~666 lines):
//! - Module header and boilerplate
//! - Signal declarations (input, output, internal, signed, widths)
//! - Guard logic (shift register, counter, 1-cycle combinational, complex)
//! - Reflex assignments (single guard, multi-guard AND join, defaults)
//! - SVA property generation (all 6 formula variants x 3 directives)
//! - Bind file generation (`emit_sva_bind_file`)
//! - Synthesis-clean mode (`emit_sv_synthesis`)
//! - Prev register handling (`_dN` delay suffix)
//! - Synchronizer chain emission (`emit_synchronizer_chains`)
//! - SVA-only mode (`emit_sva_only`)
//! - Edge cases (empty modules, no guards, no reflexes, no properties)
//!
//! NASA Power-of-10 compliance:
//! - `#![forbid(unsafe_code)]`
//! - Bounded iteration with `MAX_*` constants
//! - No recursion in test helpers
//! - All `assert!` with descriptive messages

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use mirrc::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use mirrc::emit::verilog;
use mirrc::pipeline::{run_pipeline, PipelineConfig, PipelineResult};
use mirrc::temporal::low_level_ir::{
    CompiledGuard, ComplexGuard, ConditionKind, ShiftRegisterGuard, TemporalNetlist,
};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum lines to scan in emitted output.
const MAX_OUTPUT_LINES: usize = 2048;

/// Maximum ports to verify in a module declaration.
const MAX_PORTS_CHECK: usize = 64;

/// Maximum number of property variants to iterate over.
const MAX_PROPERTY_VARIANTS: usize = 16;

// ---------------------------------------------------------------------------
// Helper functions (no recursion)
// ---------------------------------------------------------------------------

fn default_config() -> PipelineConfig {
    PipelineConfig::default()
}

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn lit_int(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}

fn lit_bool(b: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(b))
}

fn gt_expr(lhs: Expr, rhs: u64) -> Expr {
    Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(lhs),
        right: Box::new(Expr::Literal(LiteralValue::Integer(rhs))),
    }
}

fn add_expr(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::Add, left: Box::new(lhs), right: Box::new(rhs) }
}

fn sub_expr(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::Sub, left: Box::new(lhs), right: Box::new(rhs) }
}

fn mul_expr(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::Mul, left: Box::new(lhs), right: Box::new(rhs) }
}

fn not_expr(operand: Expr) -> Expr {
    Expr::Unary { op: mirrc::ast::types::UnaryOp::Not, operand: Box::new(operand) }
}

fn prev_expr(signal: &str, delay: u64) -> Expr {
    Expr::Prev { signal: signal.to_string(), delay }
}

fn signal_decl(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn make_guard(name: &str, condition: Expr, cycles: u64) -> Guard {
    Guard {
        name: name.to_string(),
        condition,
        cycles,
        template_cycles: None,
        origin: None,
        span: None,
    }
}

fn make_reflex(name: &str, guard_names: Vec<&str>, assignments: Vec<Assignment>) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: guard_names.into_iter().map(|s| s.to_string()).collect(),
        assignments,
        origin: None,
        span: None,
    }
}

fn make_assignment(target: &str, value: Expr) -> Assignment {
    Assignment { target: target.to_string(), value, span: None }
}

fn make_property(
    name: &str,
    directive: PropertyDirective,
    formula: PropertyFormula,
) -> PropertyDecl {
    PropertyDecl { name: name.to_string(), directive, formula, origin: None, span: None }
}

/// Build a PipelineResult from a programmatic Module (bypasses parser/validation).
fn result_from_module(module: Module) -> PipelineResult {
    PipelineResult {
        hls_result: None,
        program: MirrProgram { patterns: Vec::new(), imports: Vec::new(), module },
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
    }
}

/// Build a PipelineResult from a Module with a custom temporal netlist.
fn result_with_netlist(module: Module, netlist: TemporalNetlist) -> PipelineResult {
    PipelineResult {
        hls_result: None,
        program: MirrProgram { patterns: Vec::new(), imports: Vec::new(), module },
        simplify_stats: None,
        width_result: None,
        temporal_netlist: Some(netlist),
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
    }
}

/// Count lines in output, bounded by MAX_OUTPUT_LINES.
fn count_lines_bounded(text: &str) -> usize {
    let mut count = 0usize;
    for _line in text.lines() {
        count += 1;
        if count >= MAX_OUTPUT_LINES {
            break;
        }
    }
    count
}

// ---------------------------------------------------------------------------
// MIRR source fixtures
// ---------------------------------------------------------------------------

const MINIMAL_MODULE: &str = r#"
module minimal {
    signal a: in u8;
    signal b: out u8;

    guard g {
        when a > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            b = a;
        }
    }
}
"#;

const MULTI_GUARD_MODULE: &str = r#"
module multi_guard_mod {
    signal x: in bool;
    signal y: in bool;
    signal z: out bool;

    guard g1 {
        when x
        for 3 cycles;
    }

    guard g2 {
        when y
        for 5 cycles;
    }

    reflex join_both {
        on g1 and g2 {
            z = true;
        }
    }
}
"#;

const COUNTER_GUARD_MODULE: &str = r#"
module counter_mod {
    signal enable: in bool;
    signal ready: out bool;

    guard sustained {
        when enable
        for 100 cycles;
    }

    reflex fire {
        on sustained {
            ready = true;
        }
    }
}
"#;

const INTERNAL_SIGNALS_MODULE: &str = r#"
module internals {
    signal sensor: in u16;
    signal result: out u16;
    signal accumulator: internal u16;

    guard active {
        when sensor > 0
        for 2 cycles;
    }

    reflex compute {
        on active {
            accumulator = sensor;
            result = accumulator;
        }
    }
}
"#;

const PROPERTY_ALL_VARIANTS: &str = r#"
module prop_mod {
    signal sensor: in u16;
    signal alarm: out bool;

    guard g {
        when sensor > 100
        for 3 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }

    property p_always {
        always (sensor < 1000);
    }

    property p_never {
        never (alarm && sensor < 50);
    }

    property p_implies {
        always (sensor > 100 -> alarm);
    }

    property p_never_implies {
        never (sensor > 100 -> alarm);
    }

    property p_eventually {
        eventually within 10 (alarm);
    }

    property p_followed_by {
        always (sensor > 200 followed_by 3 alarm);
    }
}
"#;

const NO_GUARD_MODULE: &str = r#"
module bare {
    signal a: in u8;
    signal b: out u8;
}
"#;

const SIGNED_TYPES_MODULE: &str = r#"
module signed_types {
    signal s_in: in i16;
    signal s_out: out i16;
    signal en: in bool;

    guard g {
        when en
        for 1 cycles;
    }

    reflex r {
        on g {
            s_out = s_in;
        }
    }
}
"#;

// ===========================================================================

mod part1;
mod part10;
mod part11;
mod part12;
mod part13;
mod part14;
mod part15;
mod part16;
mod part17;
mod part18;
mod part19;
mod part2;
mod part20;
mod part21;
mod part22;
mod part23;
mod part24;
mod part25;
mod part26;
mod part27;
mod part3;
mod part4;
mod part5;
mod part6;
mod part7;
mod part8;
mod part9;
