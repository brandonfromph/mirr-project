// ---------------------------------------------------------------------------
//! Top-level program and module AST structures.
//!
//! Defines the core MIRR constructs: signals, guards, reflexes, and the
//! module container. Extended with pattern calls and origin tagging (Phase 7b).
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

use super::expr::Expr;
use super::pattern::{PatternCall, PatternDef, PatternOrigin};
use super::property::PropertyDecl;
use super::types::{SignalKind, SignalType};

/// A signal declaration: name, direction (in/out/internal), and bit-width type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalDecl {
    pub name: String,
    pub kind: SignalKind,
    #[serde(rename = "ty")]
    pub ty: SignalType,
    /// Pattern origin tag for DO-178C traceability (`None` for hand-written signals).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// A temporal guard: fires when a condition holds for N consecutive clock cycles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guard {
    pub name: String,
    pub condition: Expr,
    pub cycles: u64,
    /// Pattern origin tag for DO-178C traceability (`None` for hand-written guards).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// A single assignment: `target = value;`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    pub target: String,
    pub value: Expr,
}

/// A reflex block: triggered by a guard, assigns values to output/internal signals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reflex {
    pub name: String,
    pub guard_names: Vec<String>,
    pub assignments: Vec<Assignment>,
    /// Pattern origin tag for DO-178C traceability (`None` for hand-written reflexes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// A MIRR module: the top-level container for signals, guards, reflexes, and properties.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub signals: Vec<SignalDecl>,
    pub guards: Vec<Guard>,
    pub reflexes: Vec<Reflex>,
    #[serde(default)]
    pub properties: Vec<PropertyDecl>,
    #[serde(default)]
    pub pattern_calls: Vec<PatternCall>,
    #[serde(default)]
    pub pattern_origins: Vec<PatternOrigin>,
}

/// Root of a parsed MIRR program, with IR version for contract tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrProgram {
    #[serde(default)]
    pub patterns: Vec<PatternDef>,
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
        Self { ir_version: "1.0".to_string(), module: program.module.clone() }
    }
}
