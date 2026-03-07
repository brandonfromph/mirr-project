//! Core types for MIRR Phase 4 bit-width inference.
//!
//! Defines the width-annotated expression tree (`WidthExpr`), diagnostic types,
//! and the flat node representation used during constraint solving.

#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, UnaryOp};

// ---------------------------------------------------------------------------
// Width
// ---------------------------------------------------------------------------

/// Resolved bit-width for a single expression node.
///
/// Valid range: 1..=64.  A width of 0 means "unresolved" during solving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Width(pub u32);

impl Width {
    /// Maximum supported width (64-bit hardware registers).
    pub const MAX: Width = Width(64);

    /// Minimum bits required to represent the unsigned value `v`.
    /// Returns Width(1) for v == 0 (a single bit is needed to hold zero).
    pub fn min_bits_for(v: u64) -> Width {
        if v == 0 {
            return Width(1);
        }
        // 64 - leading_zeros gives the position of the highest set bit + 1.
        Width(64_u32.saturating_sub(v.leading_zeros()))
    }
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "u{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// WidthExpr — width-annotated expression tree
// ---------------------------------------------------------------------------

/// Width-annotated expression node.
///
/// Parallel tree to `crate::ast::Expr`, produced by the width inference pass.
/// Every node carries its resolved `Width`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidthExpr {
    Literal { value: u64, width: Width },
    Signal { name: String, width: Width },
    Unary { op: UnaryOp, operand: Box<WidthExpr>, width: Width },
    Binary { op: BinaryOp, left: Box<WidthExpr>, right: Box<WidthExpr>, width: Width },
}

impl WidthExpr {
    /// Return the width of this node.
    pub fn width(&self) -> Width {
        match self {
            WidthExpr::Literal { width, .. }
            | WidthExpr::Signal { width, .. }
            | WidthExpr::Unary { width, .. }
            | WidthExpr::Binary { width, .. } => *width,
        }
    }
}

// ---------------------------------------------------------------------------
// FlatNode — linearized representation for constraint solving
// ---------------------------------------------------------------------------

/// Maximum number of flat nodes the width pass will process.
/// Matches MAX_SIMPLIFY_DEPTH * 4 from the simplifier (128 * 4 = 512).
pub const MAX_FLAT_NODES: usize = 512;

/// A flattened expression node assigned a `NodeId` for constraint solving.
///
/// The tree is linearized in post-order so that children always have
/// lower indices than their parent.
#[derive(Debug, Clone)]
pub enum FlatNode {
    Literal { value: u64 },
    Signal { name: String },
    Unary { op: UnaryOp, operand: u32 },
    Binary { op: BinaryOp, left: u32, right: u32 },
}

// ---------------------------------------------------------------------------
// Diagnostic severity & type
// ---------------------------------------------------------------------------

/// Severity of a width diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagSeverity {
    /// Hard error — compilation must stop.
    Error,
    /// Informational note — no harm, but worth knowing.
    Info,
}

/// A diagnostic emitted by the width inference pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WidthDiag {
    pub severity: DiagSeverity,
    pub message: String,
}

impl WidthDiag {
    pub fn error(msg: impl Into<String>) -> Self {
        WidthDiag { severity: DiagSeverity::Error, message: msg.into() }
    }
    pub fn info(msg: impl Into<String>) -> Self {
        WidthDiag { severity: DiagSeverity::Info, message: msg.into() }
    }
}

impl std::fmt::Display for WidthDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.severity {
            DiagSeverity::Error => "error",
            DiagSeverity::Info => "info",
        };
        write!(f, "[width:{}] {}", prefix, self.message)
    }
}

// ---------------------------------------------------------------------------
// WidthResult — aggregate output of width inference
// ---------------------------------------------------------------------------

/// Aggregate statistics from a width inference run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WidthStats {
    /// Number of expression nodes analyzed.
    pub nodes_analyzed: usize,
    /// Number of constraint propagation rounds executed.
    pub propagation_rounds: usize,
    /// Number of diagnostics emitted (errors + infos).
    pub diagnostics_count: usize,
}
