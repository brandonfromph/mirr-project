// ---------------------------------------------------------------------------
//! Top-level program and module AST structures.
//!
//! Defines the core MIRR constructs: signals, guards, reflexes, and the
//! module container. Extended with pattern calls and origin tagging (Phase 7b).
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::expr::Expr;
use super::pattern::{PatternCall, PatternDef, PatternOrigin};
use super::property::PropertyDecl;
use super::types::{SignalKind, SignalType};
use crate::span::Span;

/// A signal declaration: name, direction (in/out/internal), and bit-width type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalDecl {
    /// Signal identifier (unique within the module).
    pub name: String,
    /// Direction: input, output, or internal.
    pub kind: SignalKind,
    /// Bit-width type (bool, u8–u64, i8–i64).
    #[serde(rename = "ty")]
    pub ty: SignalType,
    /// Pattern origin tag for DO-178C traceability (`None` for hand-written signals).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Source span for LSP diagnostics (`None` when unavailable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// A temporal guard: fires when a condition holds for N consecutive clock cycles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guard {
    /// Guard identifier (referenced by reflexes).
    pub name: String,
    /// Boolean condition expression evaluated each clock tick.
    pub condition: Expr,
    /// Number of consecutive cycles the condition must hold before firing.
    pub cycles: u64,
    /// Pattern origin tag for DO-178C traceability (`None` for hand-written guards).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Source span for LSP diagnostics (`None` when unavailable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// A single assignment: `target = value;`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    /// Target signal name (must be output or internal).
    pub target: String,
    /// Value expression to assign.
    pub value: Expr,
    /// Source span for LSP diagnostics (`None` when unavailable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// A reflex block: triggered by a guard, assigns values to output/internal signals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reflex {
    /// Reflex block identifier.
    pub name: String,
    /// List of guard names that trigger this reflex.
    pub guard_names: Vec<String>,
    /// Assignments executed when the reflex fires.
    pub assignments: Vec<Assignment>,
    /// Pattern origin tag for DO-178C traceability (`None` for hand-written reflexes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Source span for LSP diagnostics (`None` when unavailable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// A MIRR module: the top-level container for signals, guards, reflexes, and properties.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    /// Module name (appears after `module` keyword).
    pub name: String,
    /// Signal declarations (inputs, outputs, internals).
    pub signals: Vec<SignalDecl>,
    /// Temporal guard definitions.
    pub guards: Vec<Guard>,
    /// Reflex blocks (guard-triggered assignments).
    pub reflexes: Vec<Reflex>,
    /// Safety property declarations (compiled to SVA).
    #[serde(default)]
    pub properties: Vec<PropertyDecl>,
    /// Pattern instantiation calls (erased after expansion).
    #[serde(default)]
    pub pattern_calls: Vec<PatternCall>,
    /// Provenance tags from pattern expansion (DO-178C traceability).
    #[serde(default)]
    pub pattern_origins: Vec<PatternOrigin>,
    /// Source span for LSP diagnostics (`None` when unavailable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// Root of a parsed MIRR program, with IR version for contract tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrProgram {
    /// Top-level pattern definitions (`def` blocks).
    #[serde(default)]
    pub patterns: Vec<PatternDef>,
    /// The single module in this compilation unit.
    pub module: Module,
}

/// Versioned AST wrapper for canonical JSON serialization.
/// Used by IR contract tests and parity gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrAstJson {
    /// IR version string for contract tracking (currently "1.0").
    pub ir_version: String,
    /// The compiled module AST.
    pub module: Module,
}

impl MirrAstJson {
    /// Wrap a parsed program in the versioned JSON envelope.
    ///
    /// Spans are stripped because they are compiler-internal metadata
    /// and not part of the IR contract.
    pub fn from_program(program: &MirrProgram) -> Self {
        let mut module = program.module.clone();
        module.span = None;
        for sig in &mut module.signals {
            sig.span = None;
        }
        for guard in &mut module.guards {
            guard.span = None;
        }
        for reflex in &mut module.reflexes {
            reflex.span = None;
            for assign in &mut reflex.assignments {
                assign.span = None;
            }
        }
        for prop in &mut module.properties {
            prop.span = None;
        }
        for call in &mut module.pattern_calls {
            call.span = None;
        }
        Self { ir_version: "1.0".to_string(), module }
    }
}
