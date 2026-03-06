// ---------------------------------------------------------------------------
// Program-level AST structures
// ---------------------------------------------------------------------------
// Single responsibility: top-level declarations — signals, guards, reflexes,
// modules, and the program root.
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

use super::expr::Expr;
use super::types::{SignalKind, SignalType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalDecl {
    pub name: String,
    pub kind: SignalKind,
    #[serde(rename = "ty")]
    pub ty: SignalType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guard {
    pub name: String,
    pub condition: Expr,
    pub cycles: u64,
}

/// A single assignment: `target = value;`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    pub target: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reflex {
    pub name: String,
    pub guard_names: Vec<String>,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub signals: Vec<SignalDecl>,
    pub guards: Vec<Guard>,
    pub reflexes: Vec<Reflex>,
}

/// Root of a parsed MIRR program, with IR version for contract tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrProgram {
    pub module: Module,
}

/// Versioned AST wrapper for canonical JSON serialization.
/// Used by IR contract tests and parity gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrAstJson {
    pub ir_version: String,
    pub module: Module,
}

impl MirrAstJson {
    /// Wrap a parsed program in the versioned JSON envelope.
    pub fn from_program(program: &MirrProgram) -> Self {
        Self {
            ir_version: "1.0".to_string(),
            module: program.module.clone(),
        }
    }
}