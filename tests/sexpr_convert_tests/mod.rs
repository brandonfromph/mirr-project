#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
//! Comprehensive tests for `src/sexpr/convert.rs` — bidirectional AST <-> S-expression conversion.
//!
//! NASA Power-of-10 compliant: bounded iteration, no recursion, descriptive asserts.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::pattern::{
    PatternArg, PatternCall, PatternDef, PatternOrigin, PatternParam, PatternParamKind,
    ReflectBlock,
};
use nasa_rust_project::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{
    BinaryOp, EffectQualifier, ExtendedType, Linearity, LiteralValue, Refinement, SignalKind,
    SignalType, TypeAnnotations, UnaryOp,
};
use nasa_rust_project::sexpr::convert::{ast_to_sexpr, sexpr_to_ast};
use nasa_rust_project::sexpr::types::SExpr;

/// Maximum test iterations for bounded loops (NASA Power-of-10).
const MAX_TEST_ITEMS: usize = 64;

// =========================================================================
// Helper: build a minimal empty program
// =========================================================================

fn empty_module(name: &str) -> Module {
    Module {
        name: name.to_string(),
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn empty_program() -> MirrProgram {
    MirrProgram { patterns: Vec::new(), module: empty_module("test_module") }
}

fn default_annotations() -> TypeAnnotations {
    TypeAnnotations::default()
}

// =========================================================================
// 1. AST -> S-Expr: Full program structure
// =========================================================================


mod part1;
mod part2;
mod part3;
