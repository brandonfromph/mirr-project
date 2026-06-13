//! Core types for MIRR Phase 4 bit-width inference.
//!
//! Defines the width-annotated expression tree (`WidthExpr`), diagnostic types,
//! and the flat node representation used during constraint solving.

#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, UnaryOp};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Width
// ---------------------------------------------------------------------------

/// Resolved bit-width for a single expression node.
///
/// Valid range: 1..=64.  A width of 0 means "unresolved" during solving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Width(pub u32);

impl Width {
    /// Maximum supported width (64-bit hardware registers).
    pub const MAX: Width = Width(8192);

    /// Minimum bits required to represent the unsigned value `v`.
    /// Returns Width(1) for v == 0 (a single bit is needed to hold zero).
    pub fn min_bits_for(v: u64) -> Width {
        if v == 0 {
            return Width(1);
        }
        // 64 - leading_zeros gives the position of the highest set bit + 1.
        Width(64_u32.saturating_sub(v.leading_zeros()))
    }

    /// Format this width with a sign prefix: `i16` for signed, `u16` for unsigned.
    pub fn display_with_sign(&self, signed: bool) -> String {
        if signed {
            format!("i{}", self.0)
        } else {
            format!("u{}", self.0)
        }
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum WidthExpr {
    /// Literal constant with its minimum bit-width.
    Literal { value: u64, width: Width },
    /// Signal reference with its declared or inferred width.
    Signal { name: String, width: Width },
    /// Unary operation with result width.
    Unary { op: UnaryOp, operand: Box<WidthExpr>, width: Width },
    /// Binary operation with result width.
    Binary { op: BinaryOp, left: Box<WidthExpr>, right: Box<WidthExpr>, width: Width },
    /// Previous-tick reference with signal's width.
    Prev { signal: String, delay: u64, width: Width },
}

impl WidthExpr {
    /// Return the width of this node.
    pub fn width(&self) -> Width {
        match self {
            WidthExpr::Literal { width, .. }
            | WidthExpr::Signal { width, .. }
            | WidthExpr::Unary { width, .. }
            | WidthExpr::Binary { width, .. }
            | WidthExpr::Prev { width, .. } => *width,
        }
    }
}

// ---------------------------------------------------------------------------
// FlatNode — linearized representation for constraint solving
// ---------------------------------------------------------------------------

// #[allow(dead_code, deprecated)]
// #[deprecated]
// /// Maximum number of flat nodes the width pass will process.
// /// Matches MAX_SIMPLIFY_DEPTH * 4 from the simplifier (128 * 4 = 512).
// pub const MAX_FLAT_NODES: usize = 512;
//
// /// A flattened expression node assigned a `NodeId` for constraint solving.
// ///
// /// The tree is linearized in post-order so that children always have
// /// lower indices than their parent.
// #[derive(Debug, Clone)]
// pub enum FlatNode {
//     /// Literal constant value.
//     Literal { value: u64 },
//     /// Signal reference with signedness flag.
//     Signal { name: String, signed: bool },
//     /// Unary operation referencing operand by node index.
//     Unary { op: UnaryOp, operand: u32 },
//     /// Binary operation referencing operands by node indices.
//     Binary { op: BinaryOp, left: u32, right: u32 },
//     /// Previous-tick reference with signedness flag.
//     Prev { signal: String, delay: u64, signed: bool },
//     /// Array indexing referencing array and index nodes.
//     ArrayIndex { array: u32, index: u32, width: u32, signed: bool },
//     /// Struct field access referencing object node.
//     FieldAccess { object: u32, field: String, width: u32, signed: bool },
//     /// Array literal with element node indices. Width = element_width * length.
//     ArrayLiteral { elements: Vec<u32>, width: u32 },
//     /// Struct literal with field node indices. Width = sum of field widths.
//     StructLiteral { name: String, fields: Vec<(String, u32)>, width: u32 },
//     /// Meta-stage unfolding index.
//     UnfoldIndex { name: String },
// }

// ---------------------------------------------------------------------------
// SCC types for Phase 4b
// ---------------------------------------------------------------------------

/// Maximum number of signals the SCC analyzer will process.
pub const MAX_SIGNALS: usize = 1024;

/// Maximum SCC size before emitting a hard error.
pub const MAX_SCC_SIZE: usize = 64;

/// Classification of a strongly connected component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SccKind {
    /// Values can grow (contains Add, Mul, Shl on cycle path).
    Expansive,
    /// Values circulate but don't grow (Prev-only, And/Or/Xor, comparisons).
    Nonexpansive,
}

/// Information about a detected SCC in the width dependency graph.
#[derive(Debug, Clone, Serialize)]
pub struct SccInfo {
    /// Entities belonging to this SCC in the ECS registry.
    pub signals: Vec<crate::ecs::components::EntityId>,
    /// Classification of this SCC.
    pub kind: SccKind,
}

// ---------------------------------------------------------------------------
// Diagnostic severity & type
// ---------------------------------------------------------------------------

/// Severity of a width diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiagSeverity {
    /// Hard error — compilation must stop.
    Error,
    /// Non-fatal warning — something suspicious but not fatal.
    Warning,
    /// Informational note — no harm, but worth knowing.
    Info,
}

/// A diagnostic emitted by the width inference pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WidthDiag {
    /// Severity level (error, warning, or info).
    pub severity: DiagSeverity,
    /// Human-readable diagnostic message.
    pub message: String,
    /// Machine-readable error code, e.g. "E503".
    pub code: Option<String>,
    /// Source location where the diagnostic was emitted.
    pub span: Option<crate::span::Span>,
    /// Contextual signal name related to this diagnostic.
    pub signal_name: Option<String>,
    /// Fix suggestion or help text.
    pub help: Option<String>,
}

impl WidthDiag {
    /// Create an error-severity diagnostic.
    pub fn error(msg: impl Into<String>) -> Self {
        WidthDiag {
            severity: DiagSeverity::Error,
            message: msg.into(),
            code: None,
            span: None,
            signal_name: None,
            help: None,
        }
    }
    /// Create an info-severity diagnostic.
    pub fn info(msg: impl Into<String>) -> Self {
        WidthDiag {
            severity: DiagSeverity::Info,
            message: msg.into(),
            code: None,
            span: None,
            signal_name: None,
            help: None,
        }
    }

    /// Attach a machine-readable error code (builder pattern).
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    /// Attach a source span (builder pattern).
    pub fn with_span(mut self, span: Option<crate::span::Span>) -> Self {
        self.span = span;
        self
    }

    /// Attach a contextual signal name (builder pattern).
    pub fn with_signal(mut self, name: &str) -> Self {
        self.signal_name = Some(name.to_string());
        self
    }

    /// Attach a help / fix-suggestion string (builder pattern).
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Convert this width diagnostic to a [`crate::diagnostic::Diagnostic`]
    /// for rendering through the central diagnostic engine.
    pub fn to_diagnostic(&self) -> crate::diagnostic::Diagnostic {
        let sev = match self.severity {
            DiagSeverity::Error => crate::diagnostic::Severity::Error,
            DiagSeverity::Warning => crate::diagnostic::Severity::Warning,
            DiagSeverity::Info => crate::diagnostic::Severity::Info,
        };
        let mut diag = crate::diagnostic::Diagnostic {
            severity: sev,
            message: self.message.clone(),
            code: self.code.clone(),
            span: self.span,
            labels: Vec::new(),
        };
        if let Some(ref sig) = self.signal_name {
            diag = diag.with_note(format!("in signal '{}'", sig));
        }
        if let Some(ref help) = self.help {
            diag = diag.with_help(help.clone());
        }
        diag
    }
}

impl std::fmt::Display for WidthDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.severity {
            DiagSeverity::Error => "error",
            DiagSeverity::Warning => "warning",
            DiagSeverity::Info => "info",
        };
        match &self.code {
            Some(code) => write!(f, "[width:{} {}] {}", prefix, code, self.message),
            None => write!(f, "[width:{}] {}", prefix, self.message),
        }
    }
}

// ---------------------------------------------------------------------------
// WidthResult — aggregate output of width inference
// ---------------------------------------------------------------------------

/// Aggregate statistics from a width inference run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WidthStats {
    /// Number of expression nodes analyzed.
    pub nodes_analyzed: usize,
    /// Number of constraint propagation rounds executed.
    pub propagation_rounds: usize,
    /// Number of diagnostics emitted (errors + infos).
    pub diagnostics_count: usize,
    /// Number of non-trivial SCCs detected (Phase 4b).
    pub scc_count: usize,
    /// Number of expansive SCCs.
    pub expansive_count: usize,
    /// Number of nonexpansive SCCs.
    pub nonexpansive_count: usize,
}
