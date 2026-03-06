// ---------------------------------------------------------------------------
// AST module — public interface
// ---------------------------------------------------------------------------
// NASA/JPL Rule: Explicit module boundaries with a clean public interface.
// Internal implementation is hidden behind re-exports.
// ---------------------------------------------------------------------------

pub mod expr;
pub mod program;
pub mod types;

// Re-export all AST types at the `ast` level for ergonomic imports.
pub use expr::Expr;
pub use program::{Assignment, Guard, MirrAstJson, MirrProgram, Module, Reflex, SignalDecl};
pub use types::{BinaryOp, LiteralValue, SignalKind, SignalType, UnaryOp};