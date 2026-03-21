#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
//! Extended module parser tests.
//!
//! ~30+ tests covering module declarations, signal parsing (all kinds and types),
//! guards, reflexes, properties (all formula + directive variants), pattern defs,
//! error recovery, and edge cases.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::property::{PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{
    BinaryOp, EffectQualifier, Linearity, LiteralValue, SignalKind, SignalType, UnaryOp,
};
use nasa_rust_project::parse_mirr;

// =========================================================================
// Bounded iteration constants (NASA Power-of-10)
// =========================================================================

/// Maximum signals inspected in any single test loop.
const MAX_SIGNALS: usize = 64;

/// Maximum guards inspected in any single test loop.
const MAX_GUARDS: usize = 32;

/// Maximum reflexes inspected in any single test loop.
const MAX_REFLEXES: usize = 32;

/// Maximum assignments inspected in any single test loop.
const MAX_ASSIGNMENTS: usize = 64;

/// Maximum properties inspected in any single test loop.
const MAX_PROPERTIES: usize = 32;

/// Maximum pattern definitions inspected in any single test loop.
const MAX_PATTERNS: usize = 64;

// =========================================================================
// Helpers (no recursion, all bounded)
// =========================================================================

fn assert_parse_ok(source: &str) -> nasa_rust_project::MirrProgram {
    parse_mirr(source).expect("expected parse to succeed")
}

fn assert_parse_err(source: &str, msg_contains: &str) {
    let err = parse_mirr(source).expect_err("expected parse to fail");
    assert!(
        err.to_string().contains(msg_contains),
        "error '{}' should contain '{}'",
        err,
        msg_contains
    );
}

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn int(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}

fn bool_lit(v: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(v))
}

fn bin(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    Expr::Binary { op, left: Box::new(left), right: Box::new(right) }
}

fn not(e: Expr) -> Expr {
    Expr::Unary { op: UnaryOp::Not, operand: Box::new(e) }
}

/// Wrap a body of declarations inside a minimal module with base signals and a guard.
fn wrap_module(body: &str) -> String {
    format!(
        r#"
module test_mod {{
    signal x: in bool;
    signal y: out bool;
    signal z: in u16;

    guard g {{
        when x
        for 2 cycles;
    }}

    reflex r {{
        on g {{
            y = true;
        }}
    }}

    {body}
}}
"#
    )
}

// =========================================================================
// 1. Module declaration parsing
// =========================================================================

mod sub1;
mod sub2;

