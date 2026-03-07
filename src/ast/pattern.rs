// ---------------------------------------------------------------------------
//! Pattern system AST types for compile-time hardware template expansion.
//!
//! Defines pattern definitions (`def`), reflect blocks, pattern calls,
//! parameter types, and origin metadata for DO-178C traceability.
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

use super::types::{SignalKind, SignalType};

/// The kind of a pattern parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternParamKind {
    /// A signal reference: `signal in u16`, `signal out bool`, etc.
    Signal { kind: SignalKind, ty: SignalType },
    /// A compile-time constant: `u16`, `u32`, `bool`.
    Constant { ty: SignalType },
}

/// A single parameter in a pattern definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternParam {
    pub name: String,
    pub kind: PatternParamKind,
}

/// The body of a `reflect` block, stored as raw text lines.
///
/// Lines contain `${param}` interpolation markers that are substituted
/// at compile time during pattern evaluation. The raw text is re-parsed
/// by the existing parser after substitution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflectBlock {
    pub raw_lines: Vec<String>,
}

/// A top-level pattern definition: `def name(params) { reflect { ... } }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternDef {
    pub name: String,
    pub params: Vec<PatternParam>,
    pub body: ReflectBlock,
}

/// An argument in a pattern call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternArg {
    /// A reference to a signal declared in the parent module.
    SignalRef(String),
    /// A compile-time integer constant.
    ConstInt(u64),
    /// A compile-time boolean constant.
    ConstBool(bool),
}

/// A pattern call inside a module body: `monitor_sensor(a, 50, 200, 1000, b);`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternCall {
    pub pattern_name: String,
    pub arguments: Vec<PatternArg>,
}

/// Metadata about a pattern expansion, carried through to emitters for annotation.
///
/// After compile-time expansion, pattern calls are erased from the module.
/// `PatternOrigin` preserves provenance so emitters can annotate output with
/// `// Pattern: monitor_sensor(airway_pressure, 10, 200, 500, alarm)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternOrigin {
    pub pattern_name: String,
    pub call_args_summary: String,
}
