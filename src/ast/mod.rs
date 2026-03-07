// ---------------------------------------------------------------------------
//! Abstract Syntax Tree module for MIRR.
//!
//! Re-exports AST types: expressions, signals, guards, reflexes,
//! properties, patterns, and the top-level program structure.
// ---------------------------------------------------------------------------

pub mod expr;
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
pub use program::{Assignment, Guard, MirrAstJson, MirrProgram, Module, Reflex, SignalDecl};
pub use property::{PropertyDecl, PropertyDirective, PropertyFormula};
pub use types::{BinaryOp, LiteralValue, SignalKind, SignalType, UnaryOp};
