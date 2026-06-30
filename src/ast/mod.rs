// ---------------------------------------------------------------------------
//! Abstract Syntax Tree module for MIRR.
//!
//! Re-exports AST types: expressions, signals, guards, reflexes,
//! properties, patterns, and the top-level program structure.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

pub mod expr;
pub mod macro_nodes;
pub mod pattern;
pub mod program;
pub mod property;
pub mod types;

// Re-export all AST types at the `ast` level for ergonomic imports.
pub use expr::Expr;
pub use pattern::{
    PatternArg, PatternCall, PatternDef, PatternOrigin, PatternParam, PatternParamKind,
    ReflectBlock,
};
pub use program::{
    Assignment, ClockDomainDecl, Guard, ImportDecl, MirrAstJson, MirrProgram, Module, Reflex,
    SignalDecl,
};
pub use property::{PropertyDecl, PropertyDirective, PropertyFormula};
pub use types::{
    BinaryOp, LiteralValue, SignalKind, SignalType, UnaryOp, MAX_ARRAY_DIMS, MAX_FIXED_POINT_BITS,
    MAX_INTERFACE_SIGNALS, MAX_STRUCT_FIELDS,
};

/// Maximum expression nodes to visit during bounded traversal (NASA P10).
pub(crate) const MAX_EXPR_NODES: usize = 8192;
