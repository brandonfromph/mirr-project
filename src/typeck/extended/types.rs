//! Core type definitions for the MEGA-1 Extended Type System.
//!
//! Contains `ExtendedType`, `RefinementPredicate`, `RefinementBound`,
//! and bounded constants (NASA P10).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::ast::types::SignalType;
use crate::span::Span;

// ---------------------------------------------------------------------------
// Bounded constants (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum number of refinement predicates per type annotation.
pub const MAX_REFINEMENT_PREDICATES: usize = 8;

/// Maximum value for a type-level natural (prevents unbounded compile-time).
pub const MAX_TYPE_NAT: u64 = 65536;

/// Maximum number of clock domains in a single module.
pub const MAX_CLOCK_DOMAINS: usize = 16;

/// Maximum number of phantom tags in a single module.
pub const MAX_PHANTOM_TAGS: usize = 32;

/// Maximum number of session type states in a single protocol.
pub const MAX_SESSION_STATES: usize = 64;

/// Maximum number of dependent type parameters on a single type.
pub const MAX_DEPENDENT_PARAMS: usize = 8;

/// Maximum extended type nodes to visit during bounded traversal.
pub const MAX_EXTENDED_TYPE_NODES: usize = 512;

// ===========================================================================
// A) ExtendedType — wraps SignalType with all MEGA-1 metadata
// ===========================================================================

/// Extended type carrying MEGA-1 metadata alongside the core `SignalType`.
///
/// This is the central type representation for the extended type checker.
/// Every field beyond `base` is optional and defaults to `None`/empty,
/// preserving backward compatibility with existing MIRR programs.
///
/// # Hardware Mapping
///
/// | Feature           | Hardware Impact                                 |
/// |-------------------|-------------------------------------------------|
/// | `base`            | Determines wire width (existing behavior)       |
/// | `refinement`      | Compile-time only — no hardware cost            |
/// | `qualifier`       | Affects scheduling and placement, not wires     |
/// | `clock_domain`    | Routes signal to specific clock tree            |
/// | `phantom`         | Compile-time only — zero hardware cost          |
/// | `type_nat`        | Compile-time only — governs array dimensions    |
/// | `dependent_params`| Compile-time only — parameterizes width/count   |
/// | `session`         | Compile-time only — protocol state machine      |
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedType {
    /// The core signal type: Bool, Unsigned(N), or Signed(N).
    /// This is the only mandatory field — all others are opt-in.
    pub base: SignalType,

    /// Refinement predicates constraining the value range.
    /// Example: `u16 where value < 1024` produces a `ValueLt(1024)` bound.
    /// Empty vec means no refinement (unconstrained within bit-width range).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refinements: Vec<RefinementPredicate>,

    /// Type qualifiers: linearity, effect, clock domain.
    /// Empty vec means no qualifiers (default MIRR semantics).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<super::qualifiers::TypeQualifier>,

    /// Clock domain this signal belongs to.
    /// `None` means the implicit default clock domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_domain: Option<super::qualifiers::ClockDomain>,

    /// Phantom provenance tag (zero-cost type-level marker).
    /// `None` means no phantom tag (untagged signal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phantom: Option<super::qualifiers::PhantomTag>,

    /// Type-level natural for array/vector dimension.
    /// `None` means scalar (not an array element type).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_nat: Option<super::qualifiers::TypeNat>,

    /// Dependent type parameters (value-parameterized types).
    /// Example: `Vector<u8, 4>` has `dependent_params = [DepParam::Const(4)]`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependent_params: Vec<super::qualifiers::DependentParam>,

    /// Session type state for protocol-level correctness.
    /// `None` means this signal does not participate in a session protocol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<super::qualifiers::SessionTypeRef>,

    /// Source span where this type annotation was written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

impl ExtendedType {
    /// Create a basic extended type from a core `SignalType` with no extensions.
    /// This is the upgrade path for all existing MIRR signal declarations.
    pub fn from_base(base: SignalType) -> Self {
        Self {
            base,
            refinements: Vec::new(),
            qualifiers: Vec::new(),
            clock_domain: None,
            phantom: None,
            type_nat: None,
            dependent_params: Vec::new(),
            session: None,
            span: None,
        }
    }

    /// Attach a source span (builder pattern).
    pub fn with_span(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    /// Returns true if this type has no extensions beyond the base `SignalType`.
    /// Used to fast-path the type checker for programs without type extensions.
    pub fn is_base_only(&self) -> bool {
        self.refinements.is_empty()
            && self.qualifiers.is_empty()
            && self.clock_domain.is_none()
            && self.phantom.is_none()
            && self.type_nat.is_none()
            && self.dependent_params.is_empty()
            && self.session.is_none()
    }

    /// Returns true if this type carries the `Linear` qualifier.
    pub fn is_linear(&self) -> bool {
        self.qualifiers.iter().any(|q| matches!(q, super::qualifiers::TypeQualifier::Linear))
    }

    /// Returns true if this type is marked `pure` (combinational).
    pub fn is_pure(&self) -> bool {
        self.qualifiers.iter().any(|q| matches!(q, super::qualifiers::TypeQualifier::Pure))
    }

    /// Returns true if this type is marked `stateful` (sequential).
    pub fn is_stateful(&self) -> bool {
        self.qualifiers.iter().any(|q| matches!(q, super::qualifiers::TypeQualifier::Stateful))
    }

    /// Returns the clock domain name if one is attached, or `None`.
    pub fn clock_domain_name(&self) -> Option<&str> {
        self.clock_domain.as_ref().map(|cd| cd.name.as_str())
    }

    /// Extract the maximum bit-width capacity of the base type.
    /// Used by refinement checking to ensure bounds fit in the wire.
    pub fn base_max_value(&self) -> Option<u64> {
        match &self.base {
            SignalType::Bool => Some(1),
            SignalType::Unsigned(w) => {
                if *w >= 64 {
                    None // Overflow: u64::MAX would itself be the max
                } else {
                    Some((1u64 << *w) - 1)
                }
            }
            SignalType::Signed(w) => {
                if *w == 0 {
                    Some(0)
                } else if *w >= 64 {
                    None
                } else {
                    Some((1u64 << (*w - 1)) - 1)
                }
            }
            SignalType::Array { .. }
            | SignalType::Struct { .. }
            | SignalType::FixedPoint { .. }
            | SignalType::Bundle(_)
            | SignalType::Fifo { .. } => None,
        }
    }
}

impl std::fmt::Display for ExtendedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Base type
        write!(f, "{}", self.base)?;

        // Qualifiers
        for q in &self.qualifiers {
            write!(f, " {}", q)?;
        }

        // Clock domain
        if let Some(ref cd) = self.clock_domain {
            write!(f, " @{}", cd.name)?;
        }

        // Phantom tag
        if let Some(ref pt) = self.phantom {
            write!(f, " #{}", pt.tag)?;
        }

        // Refinements
        if !self.refinements.is_empty() {
            write!(f, " where ")?;
            let mut first = true;
            for pred in &self.refinements {
                if !first {
                    write!(f, " && ")?;
                }
                first = false;
                write!(f, "{}", pred)?;
            }
        }

        // Dependent params
        if !self.dependent_params.is_empty() {
            write!(f, "<")?;
            let mut first = true;
            for dp in &self.dependent_params {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{}", dp)?;
            }
            write!(f, ">")?;
        }

        Ok(())
    }
}

// ===========================================================================
// B) RefinementPredicate / RefinementBound — compile-time range constraints
// ===========================================================================

/// A single refinement predicate on a signal's value.
///
/// Refinement predicates express compile-time constraints on the legal
/// values of a signal. They are checked statically and never generate
/// hardware — they exist purely for the type checker.
///
/// # Syntax
///
/// ```text
/// signal x: out u16 where value < 1024;
/// signal y: out u16 where value >= 100 && value <= 500;
/// signal z: out u8  where value != 0;
/// ```
///
/// The `where` keyword is INTERNAL to the type annotation, not a new
/// top-level construct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefinementPredicate {
    /// The bound operator and threshold value.
    pub bound: RefinementBound,
    /// Source span of this predicate (for error reporting).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

impl std::fmt::Display for RefinementPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bound)
    }
}

/// A compile-time bound on a signal value.
///
/// These bounds are used during type checking to verify that assignments
/// and arithmetic results stay within declared ranges. They flow into
/// the FIRWINE width inference pass as upper-bound hints.
///
/// # Variants
///
/// Each variant captures a comparison operator and a constant threshold.
/// The implicit subject is always `value` (the signal's runtime value).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefinementBound {
    /// `value < threshold` — strict upper bound (exclusive).
    ValueLt(u64),
    /// `value <= threshold` — inclusive upper bound.
    ValueLe(u64),
    /// `value > threshold` — strict lower bound (exclusive).
    ValueGt(u64),
    /// `value >= threshold` — inclusive lower bound.
    ValueGe(u64),
    /// `value == constant` — exact value constraint (singleton type).
    ValueEq(u64),
    /// `value != constant` — exclusion constraint.
    ValueNe(u64),
    /// `value in lo..=hi` — closed range constraint.
    /// This is sugar for `value >= lo && value <= hi`.
    ValueInRange { lo: u64, hi: u64 },
    /// `value % divisor == remainder` — modular arithmetic constraint.
    /// Useful for alignment requirements in memory-mapped I/O.
    ValueMod { divisor: u64, remainder: u64 },
}

impl RefinementBound {
    /// Compute the tightest upper bound implied by this constraint.
    /// Returns `None` if this bound does not constrain the upper range.
    /// Used by FIRWINE width inference to derive minimum bit-widths.
    pub fn implied_max(&self) -> Option<u64> {
        match self {
            RefinementBound::ValueLt(n) => n.checked_sub(1),
            RefinementBound::ValueLe(n) => Some(*n),
            RefinementBound::ValueEq(n) => Some(*n),
            RefinementBound::ValueInRange { hi, .. } => Some(*hi),
            // These don't constrain the upper bound:
            RefinementBound::ValueGt(_)
            | RefinementBound::ValueGe(_)
            | RefinementBound::ValueNe(_)
            | RefinementBound::ValueMod { .. } => None,
        }
    }

    /// Compute the tightest lower bound implied by this constraint.
    /// Returns `None` if this bound does not constrain the lower range.
    pub fn implied_min(&self) -> Option<u64> {
        match self {
            RefinementBound::ValueGt(n) => n.checked_add(1),
            RefinementBound::ValueGe(n) => Some(*n),
            RefinementBound::ValueEq(n) => Some(*n),
            RefinementBound::ValueInRange { lo, .. } => Some(*lo),
            // These don't constrain the lower bound:
            RefinementBound::ValueLt(_)
            | RefinementBound::ValueLe(_)
            | RefinementBound::ValueNe(_)
            | RefinementBound::ValueMod { .. } => None,
        }
    }

    /// Check whether a given concrete value satisfies this bound.
    /// Used during constant-folding and compile-time evaluation.
    pub fn satisfied_by(&self, value: u64) -> bool {
        match self {
            RefinementBound::ValueLt(n) => value < *n,
            RefinementBound::ValueLe(n) => value <= *n,
            RefinementBound::ValueGt(n) => value > *n,
            RefinementBound::ValueGe(n) => value >= *n,
            RefinementBound::ValueEq(n) => value == *n,
            RefinementBound::ValueNe(n) => value != *n,
            RefinementBound::ValueInRange { lo, hi } => value >= *lo && value <= *hi,
            RefinementBound::ValueMod { divisor, remainder } => {
                if *divisor == 0 {
                    false
                } else {
                    value % *divisor == *remainder
                }
            }
        }
    }
}

impl std::fmt::Display for RefinementBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefinementBound::ValueLt(n) => write!(f, "value < {}", n),
            RefinementBound::ValueLe(n) => write!(f, "value <= {}", n),
            RefinementBound::ValueGt(n) => write!(f, "value > {}", n),
            RefinementBound::ValueGe(n) => write!(f, "value >= {}", n),
            RefinementBound::ValueEq(n) => write!(f, "value == {}", n),
            RefinementBound::ValueNe(n) => write!(f, "value != {}", n),
            RefinementBound::ValueInRange { lo, hi } => {
                write!(f, "value in {}..={}", lo, hi)
            }
            RefinementBound::ValueMod { divisor, remainder } => {
                write!(f, "value % {} == {}", divisor, remainder)
            }
        }
    }
}
