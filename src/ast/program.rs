// ---------------------------------------------------------------------------
// Program-level AST structures
// ---------------------------------------------------------------------------
// Single responsibility: top-level declarations — signals, guards, reflexes,
// modules, and the program root.
// ---------------------------------------------------------------------------

use super::expr::Expr;
use super::types::{SignalKind, SignalType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalDecl {
    pub name: String,
    pub kind: SignalKind,
    pub ty: SignalType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guard {
    pub name: String,
    pub condition: Expr,
    pub cycles: u64,
}

/// A single assignment: `target = value;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub target: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reflex {
    pub name: String,
    pub guard_names: Vec<String>,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub signals: Vec<SignalDecl>,
    pub guards: Vec<Guard>,
    pub reflexes: Vec<Reflex>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrProgram {
    pub module: Module,
}