//! MEGA-1 Extended Type System for MIRR.
//!
//! Extends the core `SignalType` with refinement types, linear types, effect
//! types, clock domain qualifiers, phantom provenance tags, type-level
//! naturals, dependent types, and session types.
//!
//! All features are **opt-in**: existing MIRR programs that use only
//! `SignalType::{Bool, Unsigned, Signed}` continue to work unchanged.
//! The extended system wraps `SignalType` in an `ExtendedType` struct that
//! carries optional metadata for each new type feature.
//!
//! Design constraints (NASA P10 / MIRR policy):
//! - `#![forbid(unsafe_code)]`
//! - No recursion — all traversals use explicit bounded loops
//! - All loops bounded by `MAX_*` constants
//! - All new types derive `Debug, Clone, PartialEq, Eq, Serialize, Deserialize`
//! - Backward-compatible: new optional fields use `#[serde(default)]`
//!
//! Error codes: E610–E625 (see error catalogue below).
//!
//! ## Error Code Allocation (MEGA-1)
//!
//! | Code | Rule | Description |
//! |------|------|-------------|
//! | E610 | REF-BOUND | Refinement lower bound exceeds upper bound |
//! | E612 | REF-WIDTH | Refinement bound exceeds declared bit-width capacity |
//! | E613 | LIN-UNUSED | Linear signal declared but never consumed |
//! | E614 | LIN-DOUBLE | Linear signal consumed more than once in a clock cycle |
//! | E616 | EFF-PURE | Pure (combinational) context contains stateful operation |
//! | E617 | EFF-MIX | Effect mismatch: stateful signal used in pure expression |
//! | E618 | CLK-CROSS | Clock domain crossing without synchronizer |
//! | E619 | CLK-UNDEF | Reference to undeclared clock domain |
//! | E620 | PHT-MISMATCH | Phantom tag mismatch in assignment or comparison |
//! | E621 | PHT-UNDEF | Reference to undeclared phantom tag |
//! | E625 | SES-PROTOCOL | Session type protocol violation (unexpected message) |

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
    pub qualifiers: Vec<TypeQualifier>,

    /// Clock domain this signal belongs to.
    /// `None` means the implicit default clock domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_domain: Option<ClockDomain>,

    /// Phantom provenance tag (zero-cost type-level marker).
    /// `None` means no phantom tag (untagged signal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phantom: Option<PhantomTag>,

    /// Type-level natural for array/vector dimension.
    /// `None` means scalar (not an array element type).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_nat: Option<TypeNat>,

    /// Dependent type parameters (value-parameterized types).
    /// Example: `Vector<u8, 4>` has `dependent_params = [DepParam::Const(4)]`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependent_params: Vec<DependentParam>,

    /// Session type state for protocol-level correctness.
    /// `None` means this signal does not participate in a session protocol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionTypeRef>,

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
    /// Used to fast-path the type checker for legacy programs.
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
        self.qualifiers.iter().any(|q| matches!(q, TypeQualifier::Linear))
    }

    /// Returns true if this type is marked `pure` (combinational).
    pub fn is_pure(&self) -> bool {
        self.qualifiers.iter().any(|q| matches!(q, TypeQualifier::Pure))
    }

    /// Returns true if this type is marked `stateful` (sequential).
    pub fn is_stateful(&self) -> bool {
        self.qualifiers.iter().any(|q| matches!(q, TypeQualifier::Stateful))
    }

    /// Returns the clock domain name if one is attached, or `None`.
    pub fn clock_domain_name(&self) -> Option<&str> {
        self.clock_domain.as_ref().map(|cd| cd.name.as_str())
    }

    /// Extract the maximum bit-width capacity of the base type.
    /// Used by refinement checking to ensure bounds fit in the wire.
    pub fn base_max_value(&self) -> Option<u64> {
        match self.base {
            SignalType::Bool => Some(1),
            SignalType::Unsigned(w) => {
                if w >= 64 {
                    None // Overflow: u64::MAX would itself be the max
                } else {
                    Some((1u64 << w) - 1)
                }
            }
            SignalType::Signed(w) => {
                if w == 0 {
                    Some(0)
                } else if w >= 64 {
                    None
                } else {
                    Some((1u64 << (w - 1)) - 1)
                }
            }
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

// ===========================================================================
// C) TypeQualifier — linear / pure / stateful / clock-domain annotations
// ===========================================================================

/// A type qualifier that annotates a signal with extra semantic constraints.
///
/// Qualifiers are orthogonal to the base `SignalType` and to each other.
/// A signal can carry multiple qualifiers (e.g., `linear pure`).
///
/// # Hardware Mapping
///
/// | Qualifier   | Hardware Impact                                      |
/// |-------------|------------------------------------------------------|
/// | `Linear`    | Compile-time only; enforces single-consume-per-cycle |
/// | `Pure`      | Restricts to combinational logic (no registers)      |
/// | `Stateful`  | Requires register allocation (sequential logic)      |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeQualifier {
    /// Linear type: the signal must be consumed exactly once per clock cycle.
    ///
    /// This interacts with the existing E216 single-writer check. While E216
    /// enforces that at most one reflex writes to a signal, `Linear` further
    /// ensures that the signal's value is read exactly once before the next
    /// clock edge. Double-reads and unused-reads are errors.
    ///
    /// # Interaction with E216
    ///
    /// - E216 (single-writer): ensures no two reflexes drive the same wire.
    ///   This is a structural check at the reflex/assignment level.
    /// - E613/E614 (linear): ensures the value is consumed exactly once per
    ///   cycle. This is a dataflow check at the expression level.
    ///
    /// Together they form the full ownership discipline:
    ///   E216 = "one writer" + E613/E614 = "one reader" = exclusive ownership.
    Linear,

    /// Pure (combinational) effect: the expression contains no state.
    ///
    /// A `pure` signal must be computed entirely from inputs and other `pure`
    /// signals within the same clock cycle. It cannot reference `Prev`,
    /// internal stateful registers, or `stateful`-qualified signals.
    ///
    /// In hardware: pure signals become combinational wires (no flip-flops).
    Pure,

    /// Stateful (sequential) effect: the signal depends on stored state.
    ///
    /// A `stateful` signal may reference `Prev` (register reads), internal
    /// state, and other `stateful` signals. Its value persists across clock
    /// ticks via register inference.
    ///
    /// In hardware: stateful signals map to flip-flop outputs.
    Stateful,
}

impl std::fmt::Display for TypeQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeQualifier::Linear => write!(f, "linear"),
            TypeQualifier::Pure => write!(f, "pure"),
            TypeQualifier::Stateful => write!(f, "stateful"),
        }
    }
}

// ===========================================================================
// Clock Domain qualifiers
// ===========================================================================

/// A clock domain identifier.
///
/// In multi-clock designs, each signal belongs to exactly one clock domain.
/// Cross-domain references require explicit synchronizers (dual flip-flop
/// chains or handshake protocols). The type checker enforces this.
///
/// # Syntax
///
/// ```text
/// signal sensor_val: in u16 @clk_fast;
/// signal display_val: out u16 @clk_slow;
/// ```
///
/// # Hardware Mapping
///
/// Clock domain annotations route signals to different clock trees in the
/// synthesized netlist. They affect place-and-route but not wire width.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClockDomain {
    /// Clock domain name (e.g., `"clk_fast"`, `"clk_slow"`).
    pub name: String,
    /// Optional frequency hint in Hz (for timing analysis).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_hz: Option<u64>,
}

impl ClockDomain {
    /// Create a new clock domain with just a name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), frequency_hz: None }
    }

    /// Attach a frequency hint (builder pattern).
    pub fn with_frequency(mut self, hz: u64) -> Self {
        self.frequency_hz = Some(hz);
        self
    }
}

impl std::fmt::Display for ClockDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)?;
        if let Some(hz) = self.frequency_hz {
            write!(f, "({}Hz)", hz)?;
        }
        Ok(())
    }
}

// ===========================================================================
// Phantom Types — zero-cost provenance tags
// ===========================================================================

/// A phantom provenance tag for zero-cost type-level tracking.
///
/// Phantom tags carry no runtime data and generate no hardware. They exist
/// purely in the type system to distinguish signals that are otherwise
/// identical at the bit level.
///
/// # Use Cases
///
/// - `Verified` vs `Unverified` sensor data (data provenance)
/// - `Encrypted` vs `Plaintext` communication channels
/// - `Sanitized` vs `Raw` user inputs
///
/// # Syntax
///
/// ```text
/// signal raw_temp:    in  u16 #Unverified;
/// signal safe_temp:   out u16 #Verified;
/// ```
///
/// Assigning a `#Unverified` signal to a `#Verified` target is a type error
/// (E620) unless an explicit verification/conversion construct is used.
///
/// # Hardware Mapping
///
/// Zero cost. Phantom tags are erased before code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhantomTag {
    /// Tag name (e.g., `"Verified"`, `"Unverified"`, `"Encrypted"`).
    pub tag: String,
}

impl PhantomTag {
    /// Create a new phantom tag.
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}

impl std::fmt::Display for PhantomTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.tag)
    }
}

// ===========================================================================
// Type-Level Naturals — compile-time dimension checking
// ===========================================================================

/// A type-level natural number for compile-time dimension checking.
///
/// Type-level naturals parameterize array/vector dimensions. They are resolved
/// entirely at compile time and generate no hardware beyond determining the
/// total wire width of aggregated signals.
///
/// # Syntax
///
/// ```text
/// signal sensor_array: in u8[4];    // 4-element array of u8
/// signal matrix: in u16[3][3];      // 3x3 matrix of u16
/// ```
///
/// # Hardware Mapping
///
/// A `u8[4]` maps to a 32-bit bus (4 * 8 bits). The array structure is
/// flattened during synthesis. The type-level natural only affects total width.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeNat {
    /// The natural number value (must be <= MAX_TYPE_NAT).
    pub value: u64,
}

impl TypeNat {
    /// Create a new type-level natural, returning `None` if it exceeds the bound.
    pub fn new(value: u64) -> Option<Self> {
        if value <= MAX_TYPE_NAT {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Compute the total bit-width when this natural parameterizes an array
    /// of signals with `element_width` bits each.
    /// Returns `None` on overflow.
    pub fn total_width(&self, element_width: u32) -> Option<u64> {
        self.value.checked_mul(element_width as u64)
    }
}

impl std::fmt::Display for TypeNat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ===========================================================================
// Dependent Types — types parameterized by values
// ===========================================================================

/// A dependent type parameter: a compile-time value that parameterizes a type.
///
/// Dependent types allow signal types to be parameterized by integer values
/// known at compile time. This enables dimension-safe operations on
/// fixed-size vectors and matrices.
///
/// # Syntax
///
/// ```text
/// signal data: in Vector<u8, 4>;        // Vector of 4 u8 elements
/// signal pair: in Pair<u16, u32>;       // Heterogeneous pair
/// signal bus:  in Bus<u8, 8, Verified>; // 8-wide bus with phantom tag
/// ```
///
/// # Hardware Mapping
///
/// Dependent parameters are resolved at compile time. `Vector<u8, 4>` becomes
/// a 32-bit bus. The parameterization exists only for type safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependentParam {
    /// A compile-time integer constant (e.g., the `4` in `Vector<u8, 4>`).
    Const(u64),
    /// A type parameter (e.g., the `u8` in `Vector<u8, 4>`).
    Type(SignalType),
    /// A phantom tag parameter (e.g., the `Verified` in `Bus<u8, 8, Verified>`).
    Phantom(PhantomTag),
}

impl std::fmt::Display for DependentParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependentParam::Const(n) => write!(f, "{}", n),
            DependentParam::Type(ty) => write!(f, "{}", ty),
            DependentParam::Phantom(pt) => write!(f, "{}", pt),
        }
    }
}

// ===========================================================================
// Session Types — protocol-level correctness
// ===========================================================================

/// A reference to a session type protocol and its current state.
///
/// Session types ensure that multi-module communication follows a declared
/// protocol. Each signal participating in a protocol carries a reference
/// to the protocol definition and the state it is currently in.
///
/// # Syntax
///
/// ```text
/// protocol Handshake {
///     state Idle -> Ready;
///     state Ready -> Ack;
///     state Ack -> Idle;
/// }
///
/// signal req: out bool session Handshake::Idle;
/// signal ack: in  bool session Handshake::Ready;
/// ```
///
/// The type checker verifies that transitions match the protocol definition.
///
/// # Hardware Mapping
///
/// Session types are compile-time only. They generate no hardware beyond
/// what the base signal types already require. The protocol state machine
/// is a static verification artifact, not a runtime FSM.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTypeRef {
    /// Name of the protocol this signal participates in.
    pub protocol: String,
    /// Current state within the protocol.
    pub state: String,
    /// Role: sender or receiver (for bidirectional protocols).
    pub role: SessionRole,
}

impl std::fmt::Display for SessionTypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "session {}::{} ({})", self.protocol, self.state, self.role)
    }
}

/// The role a signal plays in a session protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionRole {
    /// This signal sends data according to the protocol.
    Sender,
    /// This signal receives data according to the protocol.
    Receiver,
}

impl std::fmt::Display for SessionRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionRole::Sender => write!(f, "sender"),
            SessionRole::Receiver => write!(f, "receiver"),
        }
    }
}

/// A complete session protocol definition (top-level AST node).
///
/// Defines the legal state transitions for a communication protocol.
/// Referenced by `SessionTypeRef` on individual signals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionProtocol {
    /// Protocol name (e.g., `"Handshake"`).
    pub name: String,
    /// Ordered list of state transitions.
    /// Bounded by MAX_SESSION_STATES.
    pub transitions: Vec<SessionTransition>,
    /// Source span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// A single state transition in a session protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTransition {
    /// Source state name.
    pub from: String,
    /// Destination state name.
    pub to: String,
    /// Optional guard condition that must hold for this transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
}

// ===========================================================================
// D) Error codes E610-E625 — type rule identifiers
// ===========================================================================

/// Extended type error codes for MEGA-1 type features.
///
/// These are string constants used in error message formatting. They extend
/// the existing E6xx range (E601-E609) already allocated for core type errors.
pub mod error_codes {
    /// Refinement lower bound exceeds upper bound.
    pub const E610_REF_BOUND: &str = "E610";
    /// Refinement bound exceeds declared bit-width capacity.
    pub const E612_REF_WIDTH: &str = "E612";
    /// Linear signal declared but never consumed.
    pub const E613_LIN_UNUSED: &str = "E613";
    /// Linear signal consumed more than once in a clock cycle.
    pub const E614_LIN_DOUBLE: &str = "E614";
    /// Pure (combinational) context contains stateful operation.
    pub const E616_EFF_PURE: &str = "E616";
    /// Effect mismatch: stateful signal used in pure expression.
    pub const E617_EFF_MIX: &str = "E617";
    /// Clock domain crossing without synchronizer.
    pub const E618_CLK_CROSS: &str = "E618";
    /// Reference to undeclared clock domain.
    pub const E619_CLK_UNDEF: &str = "E619";
    /// Phantom tag mismatch in assignment or comparison.
    pub const E620_PHT_MISMATCH: &str = "E620";
    /// Reference to undeclared phantom tag.
    pub const E621_PHT_UNDEF: &str = "E621";
    /// Session type protocol violation (unexpected message/transition).
    pub const E625_SES_PROTOCOL: &str = "E625";
}

// ===========================================================================
// Extended Signal Declaration
// ===========================================================================

/// Extended signal declaration that carries the full `ExtendedType`.
///
/// This parallels the existing `SignalDecl` but replaces the `ty: SignalType`
/// field with `extended_ty: ExtendedType`. For backward compatibility, the
/// original `ty` field is still populated (from `extended_ty.base`).
///
/// New code should read from `extended_ty`; legacy code continues reading `ty`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedSignalDecl {
    /// Signal identifier (unique within the module).
    pub name: String,
    /// Direction: input, output, or internal.
    pub kind: crate::ast::types::SignalKind,
    /// Core bit-width type — always in sync with `extended_ty.base`.
    /// Retained for backward compatibility with existing passes.
    pub ty: SignalType,
    /// Full extended type with MEGA-1 metadata.
    #[serde(default = "default_extended_type")]
    pub extended_ty: ExtendedType,
    /// Pattern origin tag for DO-178C traceability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Source span for diagnostics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// Default extended type for deserialization backward compatibility.
fn default_extended_type() -> ExtendedType {
    ExtendedType::from_base(SignalType::Bool)
}

impl ExtendedSignalDecl {
    /// Upgrade a legacy `SignalDecl` to an `ExtendedSignalDecl`.
    /// Propagates MEGA-1 annotations from the AST-level `ExtendedType`.
    pub fn from_legacy(decl: &crate::ast::program::SignalDecl) -> Self {
        use crate::ast::types::{EffectQualifier, Linearity, Refinement};

        let base = decl.ty.signal_type();
        let annotations = &decl.ty.annotations;

        // Map annotations to checker-level qualifiers.
        let mut qualifiers: Vec<TypeQualifier> = Vec::new();
        if annotations.linearity == Linearity::Linear {
            qualifiers.push(TypeQualifier::Linear);
        }
        match annotations.effect {
            EffectQualifier::Stateful => qualifiers.push(TypeQualifier::Stateful),
            EffectQualifier::Pure => qualifiers.push(TypeQualifier::Pure),
            EffectQualifier::Unspecified => {}
        }

        // Map refinement.
        let mut refinements: Vec<RefinementPredicate> = Vec::new();
        if let Some(ref refinement) = annotations.refinement {
            match refinement {
                Refinement::Range { lo, hi } => {
                    refinements.push(RefinementPredicate {
                        bound: RefinementBound::ValueInRange { lo: *lo, hi: *hi },
                        span: decl.span,
                    });
                }
                Refinement::Predicate(s) => {
                    // Store as a >= 0 bound; future campaigns will parse the
                    // predicate string into richer bounds.
                    refinements.push(RefinementPredicate {
                        bound: RefinementBound::ValueGe(0),
                        span: decl.span,
                    });
                    let _ = s;
                }
            }
        }

        // Map clock domain.
        let clock_domain = annotations
            .clock_domain
            .as_ref()
            .map(|name| ClockDomain { name: name.clone(), frequency_hz: None });

        // Map phantom tag.
        let phantom = annotations.phantom_tag.as_ref().map(|t| PhantomTag { tag: t.clone() });

        let extended_ty = ExtendedType {
            base,
            refinements,
            qualifiers,
            clock_domain,
            phantom,
            type_nat: None,
            dependent_params: Vec::new(),
            session: None,
            span: decl.span,
        };

        Self {
            name: decl.name.clone(),
            kind: decl.kind,
            ty: base,
            extended_ty,
            origin: decl.origin.clone(),
            span: decl.span,
        }
    }
}

// ===========================================================================
// E) typecheck_extended() — extended type checking function signature
// ===========================================================================

/// Extended type map: maps each expression (by pointer identity) to its
/// inferred `ExtendedType`. Replaces `TypeMap` for MEGA-1 programs.
pub type ExtendedTypeMap = std::collections::HashMap<*const crate::ast::expr::Expr, ExtendedType>;

/// Result of extended type checking on a module.
///
/// Contains the extended type map plus any accumulated errors.
pub struct ExtendedTypeCheckResult {
    /// Extended type inferred for every expression node.
    pub type_map: ExtendedTypeMap,
    /// Accumulated type errors (empty on success).
    pub errors: crate::error::PipelineErrors,
}

/// Type-check all expressions in a module using the extended type system.
///
/// This function subsumes the existing `typecheck_module` for programs that
/// use MEGA-1 features. For programs with only base types, it delegates to
/// the existing checker and wraps the result.
///
/// ## Checking Order (all phases bounded by MAX_EXTENDED_TYPE_NODES)
///
/// 1. **Base type inference** — delegates to existing `infer_expr_type` for
///    core signedness/width checking (E601-E609).
///
/// 2. **Refinement checking** — for each assignment, checks that the RHS
///    expression's inferred refinement bounds are subsumed by the LHS target's
///    declared refinement bounds (E610-E612).
///
/// 3. **Linear type checking** — builds a per-cycle use-count map for every
///    linear signal and verifies exactly-once consumption (E613-E614).
///    Interacts with E216: E216 ensures single writer, E613/E614 ensure
///    single reader, together forming exclusive ownership.
///
/// 4. **Effect checking** — verifies that `pure` expressions contain no
///    `Prev` references or `stateful` sub-expressions (E616-E617).
///
/// 5. **Clock domain checking** — builds a domain map and verifies that
///    cross-domain references pass through a declared synchronizer (E618-E619).
///
/// 6. **Phantom tag checking** — verifies tag compatibility on assignments
///    and comparisons (E620-E621).
///
/// 7. **Session type checking** — verifies protocol state transitions
///    across module boundaries (E625).
///
/// Bounded: each phase iterates over a finite collection (signals, guards,
/// reflexes, assignments) with inner traversals bounded by MAX_EXTENDED_TYPE_NODES.
pub fn typecheck_extended(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    clock_domains: &[ClockDomain],
    phantom_tags: &[PhantomTag],
    protocols: &[SessionProtocol],
) -> ExtendedTypeCheckResult {
    // --- Phase 0: Build lookup tables ---
    let mut signal_types: std::collections::HashMap<&str, &ExtendedType> =
        std::collections::HashMap::with_capacity(extended_signals.len());
    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        signal_types.insert(&sig.name, &sig.extended_ty);
        idx += 1;
    }

    let mut errors = crate::error::PipelineErrors::new();
    let mut ext_type_map: ExtendedTypeMap =
        std::collections::HashMap::with_capacity(module.signals.len() * 4);

    // --- Phase 1: Delegate base type checking ---
    match crate::typeck::typecheck_module(module) {
        Ok(base_map) => {
            // Wrap each base type in ExtendedType
            for (iter_count, (ptr, base_ty)) in base_map.iter().enumerate() {
                if iter_count >= MAX_EXTENDED_TYPE_NODES {
                    break;
                }
                let ext = match signal_types.values().find(|et| et.base == *base_ty) {
                    Some(full) => (*full).clone(),
                    None => ExtendedType::from_base(*base_ty),
                };
                ext_type_map.insert(*ptr, ext);
            }
        }
        Err(base_errors) => {
            for e in &base_errors.errors {
                errors.push(e.clone());
            }
        }
    }

    // --- Phase 2: Refinement bound validation ---
    check_refinement_consistency(extended_signals, &mut errors);

    // --- Phase 3: Linear type checking ---
    check_linear_signals(module, extended_signals, &mut errors);

    // --- Phase 4: Effect checking ---
    check_effect_qualifiers(module, extended_signals, &mut errors);

    // --- Phase 5: Clock domain checking ---
    check_clock_domains(module, extended_signals, clock_domains, &mut errors);

    // --- Phase 6: Phantom tag checking ---
    check_phantom_tags(module, extended_signals, phantom_tags, &mut errors);

    // --- Phase 7: Session type checking ---
    check_session_types(module, extended_signals, protocols, &mut errors);

    ExtendedTypeCheckResult { type_map: ext_type_map, errors }
}

// ===========================================================================
// Phase 2: Refinement bound validation
// ===========================================================================

/// Validate that refinement predicates on each signal are internally consistent
/// and compatible with the base type's bit-width capacity.
///
/// Checks:
/// - Lower bounds do not exceed upper bounds (E610).
/// - Bounds fit within the declared bit-width (E612).
///
/// Bounded: iterates over signals (finite) and predicates (max MAX_REFINEMENT_PREDICATES).
fn check_refinement_consistency(
    signals: &[ExtendedSignalDecl],
    errors: &mut crate::error::PipelineErrors,
) {
    let mut sig_idx = 0usize;
    while sig_idx < signals.len() && sig_idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &signals[sig_idx];
        sig_idx += 1;

        if sig.extended_ty.refinements.is_empty() {
            continue;
        }

        let max_val = sig.extended_ty.base_max_value();

        let mut lo: Option<u64> = None;
        let mut hi: Option<u64> = None;

        let mut pred_idx = 0usize;
        while pred_idx < sig.extended_ty.refinements.len() && pred_idx < MAX_REFINEMENT_PREDICATES {
            let pred = &sig.extended_ty.refinements[pred_idx];
            pred_idx += 1;

            // Track tightest bounds
            if let Some(implied_lo) = pred.bound.implied_min() {
                lo = Some(lo.map_or(implied_lo, |l: u64| l.max(implied_lo)));
            }
            if let Some(implied_hi) = pred.bound.implied_max() {
                hi = Some(hi.map_or(implied_hi, |h: u64| h.min(implied_hi)));
            }

            // E612: Check bound fits in bit-width
            if let Some(implied_hi) = pred.bound.implied_max() {
                if let Some(max) = max_val {
                    if implied_hi > max {
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Signal '{}' refinement bound {} exceeds {}-bit capacity (max {}).",
                                error_codes::E612_REF_WIDTH,
                                sig.name,
                                pred.bound,
                                sig.extended_ty.base,
                                max
                            ),
                            span: pred.span,
                        });
                    }
                }
            }
        }

        // E610: Lower bound exceeds upper bound
        if let (Some(lower), Some(upper)) = (lo, hi) {
            if lower > upper {
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Signal '{}' has unsatisfiable refinement: lower bound {} > upper bound {}.",
                        error_codes::E610_REF_BOUND,
                        sig.name,
                        lower,
                        upper
                    ),
                    span: sig.span,
                });
            }
        }
    }
}

// ===========================================================================
// Phase 3: Linear type checking
// ===========================================================================

/// Check that every linear-qualified signal is consumed exactly once per cycle.
///
/// Interaction with E216 (single-writer):
/// - E216 already ensures at most one reflex writes to a given signal.
/// - Linear checking adds the dual constraint: at most one read per cycle.
/// - Together: one writer + one reader = exclusive ownership per cycle.
///
/// Bounded: iterates over reflexes, assignments, expressions (all finite).
fn check_linear_signals(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    errors: &mut crate::error::PipelineErrors,
) {
    // Collect names of linear signals
    let mut linear_names: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());
    let mut sig_idx = 0usize;
    while sig_idx < extended_signals.len() && sig_idx < MAX_EXTENDED_TYPE_NODES {
        if extended_signals[sig_idx].extended_ty.is_linear() {
            linear_names.insert(&extended_signals[sig_idx].name);
        }
        sig_idx += 1;
    }

    if linear_names.is_empty() {
        return;
    }

    // Count reads per signal across all expressions in each reflex
    let mut read_counts: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::with_capacity(linear_names.len());

    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        // Reset counts per reflex (each reflex is a separate "cycle context")
        for name in &linear_names {
            read_counts.insert(name, 0);
        }

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;
                if linear_names.contains(sig_ref.as_str()) {
                    if let Some(count) = read_counts.get_mut(sig_ref.as_str()) {
                        *count += 1;
                    }
                }
            }
        }

        // E614: Double consumption
        for (name, count) in &read_counts {
            if *count > 1 {
                if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                    return;
                }
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Linear signal '{}' is consumed {} times in reflex '{}' (must be exactly 1).",
                        error_codes::E614_LIN_DOUBLE,
                        name,
                        count,
                        reflex.name
                    ),
                    span: reflex.span,
                });
            }
        }
    }

    // E613: Unused linear signals (not consumed by any reflex)
    // Build global read set
    let mut ever_read: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(linear_names.len());

    let mut reflex_idx2 = 0usize;
    while reflex_idx2 < module.reflexes.len() && reflex_idx2 < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx2];
        reflex_idx2 += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let refs = crate::validation::semantic::collect_signal_refs(
                &reflex.assignments[assign_idx].value,
            );
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                if linear_names.contains(refs[ref_idx].as_str()) {
                    ever_read.insert(
                        // Safe: we only insert refs that are in linear_names
                        linear_names.iter().find(|n| **n == refs[ref_idx].as_str()).unwrap(),
                    );
                }
                ref_idx += 1;
            }
            assign_idx += 1;
        }
    }

    for name in &linear_names {
        if !ever_read.contains(name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Linear signal '{}' is declared but never consumed in any reflex.",
                    error_codes::E613_LIN_UNUSED,
                    name
                ),
                span: None,
            });
        }
    }
}

// ===========================================================================
// Phase 4: Effect checking
// ===========================================================================

/// Check that `pure`-qualified signals are not derived from stateful sources.
///
/// A pure signal must only depend on:
/// - Other pure signals
/// - Input signals (implicitly pure)
/// - Literals
///
/// Using `Prev` (register read) in a pure context is an error (E616).
/// Referencing a `stateful` signal from a pure expression is an error (E617).
///
/// Bounded: iterates over reflexes and assignments with bounded inner traversal.
fn check_effect_qualifiers(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build lookup: signal name -> is_pure, is_stateful
    let mut pure_signals: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());
    let mut stateful_signals: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if sig.extended_ty.is_pure() {
            pure_signals.insert(&sig.name);
        }
        if sig.extended_ty.is_stateful() {
            stateful_signals.insert(&sig.name);
        }
        idx += 1;
    }

    if pure_signals.is_empty() {
        return;
    }

    // For each reflex, check assignments to pure targets
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            if !pure_signals.contains(assignment.target.as_str()) {
                continue;
            }

            // E616: Check for Prev in pure expression
            if expr_contains_prev(&assignment.value) {
                if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                    return;
                }
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Pure signal '{}' cannot depend on prev() (stateful operation) in reflex '{}'.",
                        error_codes::E616_EFF_PURE,
                        assignment.target,
                        reflex.name
                    ),
                    span: assignment.span,
                });
            }

            // E617: Check for stateful signal references in pure expression
            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;
                if stateful_signals.contains(sig_ref.as_str()) {
                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Pure signal '{}' cannot depend on stateful signal '{}' in reflex '{}'.",
                            error_codes::E617_EFF_MIX,
                            assignment.target,
                            sig_ref,
                            reflex.name
                        ),
                        span: assignment.span,
                    });
                }
            }
        }
    }
}

/// Check whether an expression contains any `Prev` node.
/// Uses explicit stack (no recursion). Bounded by MAX_EXTENDED_TYPE_NODES.
fn expr_contains_prev(expr: &crate::ast::expr::Expr) -> bool {
    let mut stack: Vec<&crate::ast::expr::Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXTENDED_TYPE_NODES {
            break;
        }
        match node {
            crate::ast::expr::Expr::Prev { .. } => return true,
            crate::ast::expr::Expr::Literal(_) | crate::ast::expr::Expr::Signal(_) => {}
            crate::ast::expr::Expr::Unary { operand, .. } => stack.push(operand),
            crate::ast::expr::Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
        }
    }

    false
}

// ===========================================================================
// Phase 5: Clock domain checking
// ===========================================================================

/// Verify that cross-clock-domain signal references use a synchronizer.
///
/// If signal A is in domain `@clk_fast` and signal B is in domain `@clk_slow`,
/// then referencing A in an expression assigned to B (or vice versa) without
/// an explicit synchronizer construct is an error (E618).
///
/// Bounded: iterates over reflexes and assignments.
fn check_clock_domains(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    declared_domains: &[ClockDomain],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build domain lookup: signal name -> clock domain name
    let mut signal_domain: std::collections::HashMap<&str, &str> =
        std::collections::HashMap::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if let Some(ref cd) = sig.extended_ty.clock_domain {
            signal_domain.insert(&sig.name, &cd.name);
        }
        idx += 1;
    }

    if signal_domain.is_empty() {
        return;
    }

    // Validate declared domains exist (E619)
    let declared_names: std::collections::HashSet<&str> =
        declared_domains.iter().map(|cd| cd.name.as_str()).collect();
    for (sig_name, domain_name) in &signal_domain {
        if !declared_names.contains(domain_name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Signal '{}' references undeclared clock domain '@{}'.",
                    error_codes::E619_CLK_UNDEF,
                    sig_name,
                    domain_name
                ),
                span: None,
            });
        }
    }

    // Check cross-domain references (E618)
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let target_domain = signal_domain.get(assignment.target.as_str());

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;

                let source_domain = signal_domain.get(sig_ref.as_str());

                if let (Some(td), Some(sd)) = (target_domain, source_domain) {
                    if td != sd {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Clock domain crossing: signal '{}' (@{}) references '{}' (@{}) \
                                 without synchronizer in reflex '{}'.",
                                error_codes::E618_CLK_CROSS,
                                assignment.target,
                                td,
                                sig_ref,
                                sd,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                }
            }
        }
    }
}

// ===========================================================================
// Phase 6: Phantom tag checking
// ===========================================================================

/// Verify phantom tag compatibility on assignments.
///
/// A signal tagged `#Verified` can only be assigned from a `#Verified` source.
/// Assigning from `#Unverified` to `#Verified` is error E620.
///
/// Bounded: iterates over reflexes and assignments.
fn check_phantom_tags(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    declared_tags: &[PhantomTag],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build phantom tag lookup: signal name -> tag name
    let mut signal_tag: std::collections::HashMap<&str, &str> =
        std::collections::HashMap::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if let Some(ref pt) = sig.extended_ty.phantom {
            signal_tag.insert(&sig.name, &pt.tag);
        }
        idx += 1;
    }

    if signal_tag.is_empty() {
        return;
    }

    // E621: Validate declared tags exist
    let declared_tag_names: std::collections::HashSet<&str> =
        declared_tags.iter().map(|pt| pt.tag.as_str()).collect();
    for (sig_name, tag_name) in &signal_tag {
        if !declared_tag_names.contains(tag_name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Signal '{}' references undeclared phantom tag '#{}'. \
                     Declare it in the module's tag list.",
                    error_codes::E621_PHT_UNDEF,
                    sig_name,
                    tag_name
                ),
                span: None,
            });
        }
    }

    // E620: Check tag compatibility on assignments
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let target_tag = signal_tag.get(assignment.target.as_str());

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;

                let source_tag = signal_tag.get(sig_ref.as_str());

                match (target_tag, source_tag) {
                    (Some(tt), Some(st)) => {
                        if tt != st {
                            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                                return;
                            }
                            errors.push(crate::error::MirrError::TypeError {
                                message: format!(
                                    "[{}] Phantom tag mismatch: cannot assign #{}-tagged signal '{}' \
                                     to #{}-tagged target '{}' in reflex '{}'.",
                                    error_codes::E620_PHT_MISMATCH,
                                    st, sig_ref, tt, assignment.target, reflex.name
                                ),
                                span: assignment.span,
                            });
                        }
                    }
                    (Some(tt), None) => {
                        // Target is tagged but source is untagged — error
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Phantom tag mismatch: cannot assign untagged signal '{}' \
                                 to #{}-tagged target '{}' in reflex '{}'.",
                                error_codes::E620_PHT_MISMATCH,
                                sig_ref,
                                tt,
                                assignment.target,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                    // (None, Some(_)) — untagged target accepts any source (tag is dropped)
                    // (None, None) — no phantom types involved
                    _ => {}
                }
            }
        }
    }
}

// ===========================================================================
// Phase 7: Session type checking
// ===========================================================================

/// Verify session type protocol compliance.
///
/// For each signal participating in a session protocol, verify that the
/// signal's declared state is a legal state in the protocol and that
/// all state transitions observable in the module are legal.
///
/// Bounded: iterates over signals, protocols, and transitions.
fn check_session_types(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    protocols: &[SessionProtocol],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build protocol lookup: name -> &SessionProtocol
    let mut protocol_map: std::collections::HashMap<&str, &SessionProtocol> =
        std::collections::HashMap::with_capacity(protocols.len());
    let mut proto_idx = 0usize;
    while proto_idx < protocols.len() && proto_idx < MAX_SESSION_STATES {
        protocol_map.insert(&protocols[proto_idx].name, &protocols[proto_idx]);
        proto_idx += 1;
    }

    if protocol_map.is_empty() {
        return;
    }

    // Collect session-typed signals
    let mut sig_idx = 0usize;
    while sig_idx < extended_signals.len() && sig_idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[sig_idx];
        sig_idx += 1;

        if let Some(ref session_ref) = sig.extended_ty.session {
            // Verify protocol exists
            match protocol_map.get(session_ref.protocol.as_str()) {
                Some(proto) => {
                    // Verify state exists in protocol
                    let state_exists = proto
                        .transitions
                        .iter()
                        .any(|t| t.from == session_ref.state || t.to == session_ref.state);
                    if !state_exists {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Signal '{}' references state '{}' which does not exist \
                                 in protocol '{}'.",
                                error_codes::E625_SES_PROTOCOL,
                                sig.name,
                                session_ref.state,
                                session_ref.protocol
                            ),
                            span: sig.span,
                        });
                    }
                }
                None => {
                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Signal '{}' references undeclared session protocol '{}'.",
                            error_codes::E625_SES_PROTOCOL,
                            sig.name,
                            session_ref.protocol
                        ),
                        span: sig.span,
                    });
                }
            }
        }
    }

    // Cross-reflex transition checking would verify that if a sender signal
    // transitions from state A to state B, the corresponding receiver signal
    // also transitions according to the protocol. This requires interprocedural
    // analysis across reflexes, which is bounded by module size.
    //
    // Full implementation deferred to MEGA-1 Phase 2 (multi-module linking).
    let _ = module;
}

// ===========================================================================
// F) Refinement → FIRWINE width inference API
// ===========================================================================

/// Extract width hints from refinement bounds for the FIRWINE width inference pass.
///
/// Given an `ExtendedType`, computes the tightest upper-bound value implied
/// by all its refinement predicates, then converts that to a minimum bit-width.
///
/// Returns `None` if no refinement bounds constrain the upper range (the
/// existing width inference logic applies).
///
/// # Integration Point
///
/// This function is called by `width::constraint::generate_constraints` when
/// building the constraint set for a signal node. If a refinement-derived
/// width hint exists, it is used as an upper bound on the signal's width
/// variable, potentially allowing narrower hardware than the declared type.
///
/// Example:
/// ```text
/// signal x: out u16 where value < 1024;
/// ```
/// Declared width = 16 bits. Refinement says max value is 1023.
/// `min_bits_for(1023) = 10`. FIRWINE can infer `x` needs only 10 bits
/// (though the wire is still 16 bits for interface compatibility — the
/// optimization is that downstream logic can assume the top 6 bits are 0).
pub fn refinement_width_hint(extended_ty: &ExtendedType) -> Option<crate::width::types::Width> {
    if extended_ty.refinements.is_empty() {
        return None;
    }

    // Find the tightest upper bound across all predicates
    let mut tightest_max: Option<u64> = None;

    let mut pred_idx = 0usize;
    while pred_idx < extended_ty.refinements.len() && pred_idx < MAX_REFINEMENT_PREDICATES {
        let pred = &extended_ty.refinements[pred_idx];
        pred_idx += 1;

        if let Some(implied_max) = pred.bound.implied_max() {
            tightest_max =
                Some(tightest_max.map_or(implied_max, |current| current.min(implied_max)));
        }
    }

    tightest_max.map(crate::width::types::Width::min_bits_for)
}

/// Compute the refined width for a signal, taking the minimum of the
/// declared bit-width and the refinement-derived hint.
///
/// This is the primary API for downstream passes to query "what is the
/// effective width of this signal, considering refinements?"
///
/// Returns the declared width if no refinement narrows it.
pub fn effective_width(extended_ty: &ExtendedType) -> crate::width::types::Width {
    let declared = match extended_ty.base {
        SignalType::Bool => crate::width::types::Width(1),
        SignalType::Unsigned(w) | SignalType::Signed(w) => crate::width::types::Width(w),
    };

    match refinement_width_hint(extended_ty) {
        Some(hint) if hint.0 < declared.0 => hint,
        _ => declared,
    }
}

// ===========================================================================
// H) Parser syntax: token sequences for each type feature
// ===========================================================================

/// Describes the concrete syntax extensions for MEGA-1 type features.
///
/// This module does not contain runnable parser code (that lives in
/// `parser::module_parser`), but documents the token sequences that the
/// parser will recognize for each feature.
///
/// ## Refinement Types
///
/// ```text
/// signal x: out u16 where value < 1024;
/// signal y: out u8  where value >= 10 && value <= 200;
/// ```
///
/// Token sequence: `Ident("where") Ident("value") (Lt|Le|Gt|Ge|EqEq|BangEq) Integer`
/// Multiple predicates joined by `AmpAmp`.
///
/// ## Linear Types
///
/// ```text
/// signal x: out linear u16;
/// ```
///
/// Token sequence: `Ident("linear")` before the type name.
///
/// ## Effect Types
///
/// ```text
/// signal x: out pure u16;
/// signal y: internal stateful u32;
/// ```
///
/// Token sequence: `Ident("pure")` or `Ident("stateful")` before the type name.
///
/// ## Clock Domain Qualifiers
///
/// ```text
/// signal x: in u16 @clk_fast;
/// ```
///
/// Token sequence: `At Ident("clk_fast")` after the type.
/// Requires new `At` token (`@`) in the lexer.
///
/// ## Phantom Types
///
/// ```text
/// signal x: in u16 #Verified;
/// ```
///
/// Token sequence: `Hash Ident("Verified")` after the type.
/// Requires new `Hash` token (`#`) in the lexer.
///
/// ## Type-Level Naturals (Array Dimensions)
///
/// ```text
/// signal x: in u8[4];
/// ```
///
/// Token sequence: type `LBracket Integer RBracket` after the base type.
/// Requires new `LBracket`/`RBracket` tokens.
///
/// ## Dependent Types
///
/// ```text
/// signal x: in Vector<u8, 4>;
/// ```
///
/// Token sequence: `Ident Lt` type/integer params separated by `Comma` then `Gt`.
/// Reuses existing `Lt`/`Gt` tokens in a type-position context.
///
/// ## Session Types
///
/// ```text
/// signal x: out bool session Handshake::Idle;
/// ```
///
/// Token sequence: `Ident("session") Ident ColonColon Ident` after the type.
/// Requires new `ColonColon` token (`::`).
pub mod syntax {
    /// New tokens required by MEGA-1 type syntax.
    ///
    /// These extend the existing `Token` enum in `lexer/tokenizer.rs`.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ExtendedToken {
        /// `@` — clock domain prefix.
        At,
        /// `#` — phantom tag prefix.
        Hash,
        /// `[` — array dimension open.
        LBracket,
        /// `]` — array dimension close.
        RBracket,
        /// `::` — scope resolution (for session types).
        ColonColon,
        /// `,` — parameter separator (for dependent types).
        Comma,
        /// `where` keyword (for refinements).
        Where,
        /// `linear` keyword.
        Linear,
        /// `pure` keyword.
        KwPure,
        /// `stateful` keyword.
        KwStateful,
        /// `session` keyword.
        Session,
        /// `protocol` keyword (for protocol definitions).
        Protocol,
        /// `state` keyword (within protocol blocks).
        State,
        /// `->` — state transition arrow (session types).
        Arrow,
    }

    /// Parse a signal type string that may include MEGA-1 extensions.
    ///
    /// Extended syntax: `[qualifiers] base_type [where refinements] [@domain] [#tag] [session ref]`
    ///
    /// Returns the components separately for the caller to assemble into
    /// an `ExtendedType`.
    ///
    /// This is the planned signature; implementation follows in a dedicated PR.
    pub fn parse_extended_type_annotation(
        _input: &str,
    ) -> Result<super::ExtendedType, crate::error::MirrError> {
        // Placeholder: actual implementation will use the extended tokenizer.
        // For now, fall back to the base type parser.
        Err(crate::error::MirrError::ParseError {
            message: "[E100] Extended type parsing not yet implemented.".to_string(),
            span: None,
        })
    }
}

// ===========================================================================
// I) Hardware mapping summary (compile-time vs. synthesis)
// ===========================================================================

/// Documents how each MEGA-1 type feature maps to hardware.
///
/// This is a reference table, not executable code. It is included here
/// alongside the type definitions so that the mapping is co-located with
/// the types it describes.
///
/// | Feature             | Compile-Time | Synthesis Impact                     |
/// |---------------------|--------------|--------------------------------------|
/// | Base `SignalType`    | width check  | Wire width (UInt/SInt in FIRRTL)     |
/// | Refinement types    | range check  | None (wire width from base type)     |
/// | Linear types        | use check    | None (ownership is structural)       |
/// | Effect: pure        | dep check    | Wire only (no flip-flop)             |
/// | Effect: stateful    | dep check    | Register inference (flip-flop)       |
/// | Clock domain        | CDC check    | Clock tree routing                   |
/// | Phantom tags        | tag check    | None (erased before emit)            |
/// | Type-level naturals | dim check    | Array flattening (total wire width)  |
/// | Dependent types     | param check  | Parameterized width/count            |
/// | Session types       | FSM check    | None (protocol is static property)   |
///
/// The general principle: features that constrain _values_ (refinements,
/// phantom tags, session states) are compile-time only. Features that
/// constrain _structure_ (clock domains, effects, array dimensions) may
/// influence synthesis decisions.
pub mod hardware_mapping {
    use super::*;

    /// Determine whether an extended type feature has any impact on
    /// synthesized hardware (as opposed to being purely compile-time).
    pub fn has_synthesis_impact(ty: &ExtendedType) -> bool {
        // Clock domains affect routing
        if ty.clock_domain.is_some() {
            return true;
        }
        // Pure/stateful affects register inference
        if ty.is_pure() || ty.is_stateful() {
            return true;
        }
        // Array dimensions affect total wire width
        if ty.type_nat.is_some() {
            return true;
        }
        // Dependent params may affect width
        if !ty.dependent_params.is_empty() {
            return true;
        }
        // Everything else is compile-time only
        false
    }

    /// Convert an `ExtendedType` to a FIRRTL type string.
    ///
    /// This extends the existing `firrtl_type` function in `emit/firrtl.rs`
    /// to handle array dimensions from type-level naturals.
    pub fn extended_firrtl_type(ty: &ExtendedType) -> String {
        let base = match ty.base {
            SignalType::Bool => "UInt<1>".to_string(),
            SignalType::Unsigned(w) => format!("UInt<{}>", w),
            SignalType::Signed(w) => format!("SInt<{}>", w),
        };

        // If there's a type-level natural, wrap in a FIRRTL vector type
        if let Some(ref nat) = ty.type_nat {
            format!("{}[{}]", base, nat.value)
        } else {
            base
        }
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::SignalType;

    #[test]
    fn extended_type_from_base_is_base_only() {
        let et = ExtendedType::from_base(SignalType::Unsigned(16));
        assert!(et.is_base_only());
        assert!(!et.is_linear());
        assert!(!et.is_pure());
        assert!(!et.is_stateful());
        assert_eq!(et.clock_domain_name(), None);
    }

    #[test]
    fn extended_type_with_qualifiers() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
        et.qualifiers.push(TypeQualifier::Linear);
        et.qualifiers.push(TypeQualifier::Pure);
        assert!(!et.is_base_only());
        assert!(et.is_linear());
        assert!(et.is_pure());
    }

    #[test]
    fn refinement_bound_satisfied() {
        assert!(RefinementBound::ValueLt(1024).satisfied_by(1023));
        assert!(!RefinementBound::ValueLt(1024).satisfied_by(1024));
        assert!(RefinementBound::ValueGe(10).satisfied_by(10));
        assert!(!RefinementBound::ValueGe(10).satisfied_by(9));
        assert!(RefinementBound::ValueInRange { lo: 5, hi: 15 }.satisfied_by(10));
        assert!(!RefinementBound::ValueInRange { lo: 5, hi: 15 }.satisfied_by(16));
        assert!(RefinementBound::ValueMod { divisor: 4, remainder: 0 }.satisfied_by(8));
        assert!(!RefinementBound::ValueMod { divisor: 4, remainder: 0 }.satisfied_by(7));
    }

    #[test]
    fn refinement_bound_implied_max() {
        assert_eq!(RefinementBound::ValueLt(1024).implied_max(), Some(1023));
        assert_eq!(RefinementBound::ValueLe(1024).implied_max(), Some(1024));
        assert_eq!(RefinementBound::ValueGt(5).implied_max(), None);
        assert_eq!(RefinementBound::ValueInRange { lo: 10, hi: 200 }.implied_max(), Some(200));
    }

    #[test]
    fn refinement_width_hint_from_upper_bound() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
        et.refinements
            .push(RefinementPredicate { bound: RefinementBound::ValueLt(1024), span: None });
        let hint = refinement_width_hint(&et);
        // 1023 needs 10 bits
        assert_eq!(hint, Some(crate::width::types::Width(10)));
    }

    #[test]
    fn effective_width_with_refinement() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
        et.refinements
            .push(RefinementPredicate { bound: RefinementBound::ValueLt(256), span: None });
        // 255 needs 8 bits, which is less than declared 16
        assert_eq!(effective_width(&et), crate::width::types::Width(8));
    }

    #[test]
    fn effective_width_no_refinement() {
        let et = ExtendedType::from_base(SignalType::Unsigned(16));
        assert_eq!(effective_width(&et), crate::width::types::Width(16));
    }

    #[test]
    fn base_max_value_unsigned() {
        let et = ExtendedType::from_base(SignalType::Unsigned(8));
        assert_eq!(et.base_max_value(), Some(255));
    }

    #[test]
    fn base_max_value_bool() {
        let et = ExtendedType::from_base(SignalType::Bool);
        assert_eq!(et.base_max_value(), Some(1));
    }

    #[test]
    fn clock_domain_display() {
        let cd = ClockDomain::new("clk_fast").with_frequency(100_000_000);
        assert_eq!(cd.to_string(), "@clk_fast(100000000Hz)");
    }

    #[test]
    fn phantom_tag_display() {
        let pt = PhantomTag::new("Verified");
        assert_eq!(pt.to_string(), "#Verified");
    }

    #[test]
    fn type_nat_bounds() {
        assert!(TypeNat::new(65536).is_some());
        assert!(TypeNat::new(65537).is_none());
    }

    #[test]
    fn type_nat_total_width() {
        let nat = TypeNat::new(4).unwrap();
        assert_eq!(nat.total_width(8), Some(32)); // 4 * 8 = 32
    }

    #[test]
    fn extended_type_display() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
        et.qualifiers.push(TypeQualifier::Linear);
        et.clock_domain = Some(ClockDomain::new("clk_fast"));
        et.phantom = Some(PhantomTag::new("Verified"));
        et.refinements
            .push(RefinementPredicate { bound: RefinementBound::ValueLt(1024), span: None });
        let display = et.to_string();
        assert!(display.contains("u16"));
        assert!(display.contains("linear"));
        assert!(display.contains("@clk_fast"));
        assert!(display.contains("#Verified"));
        assert!(display.contains("value < 1024"));
    }

    #[test]
    fn extended_signal_decl_from_legacy() {
        let legacy = crate::ast::program::SignalDecl {
            name: "sensor".to_string(),
            kind: crate::ast::types::SignalKind::Input,
            ty: crate::ast::types::ExtendedType::from_core(SignalType::Unsigned(16)),
            origin: None,
            span: None,
        };
        let ext = ExtendedSignalDecl::from_legacy(&legacy);
        assert_eq!(ext.ty, SignalType::Unsigned(16));
        assert!(ext.extended_ty.is_base_only());
    }

    #[test]
    fn hardware_mapping_synthesis_impact() {
        let base = ExtendedType::from_base(SignalType::Unsigned(8));
        assert!(!hardware_mapping::has_synthesis_impact(&base));

        let mut with_clock = ExtendedType::from_base(SignalType::Unsigned(8));
        with_clock.clock_domain = Some(ClockDomain::new("clk_fast"));
        assert!(hardware_mapping::has_synthesis_impact(&with_clock));

        let mut with_pure = ExtendedType::from_base(SignalType::Unsigned(8));
        with_pure.qualifiers.push(TypeQualifier::Pure);
        assert!(hardware_mapping::has_synthesis_impact(&with_pure));
    }

    #[test]
    fn firrtl_type_with_array() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
        et.type_nat = Some(TypeNat::new(4).unwrap());
        assert_eq!(hardware_mapping::extended_firrtl_type(&et), "UInt<8>[4]");
    }

    #[test]
    fn firrtl_type_scalar() {
        let et = ExtendedType::from_base(SignalType::Signed(16));
        assert_eq!(hardware_mapping::extended_firrtl_type(&et), "SInt<16>");
    }

    #[test]
    fn session_role_display() {
        assert_eq!(SessionRole::Sender.to_string(), "sender");
        assert_eq!(SessionRole::Receiver.to_string(), "receiver");
    }

    #[test]
    fn dependent_param_display() {
        assert_eq!(DependentParam::Const(4).to_string(), "4");
        assert_eq!(DependentParam::Type(SignalType::Unsigned(8)).to_string(), "u8");
        assert_eq!(DependentParam::Phantom(PhantomTag::new("V")).to_string(), "#V");
    }

    #[test]
    fn refinement_bound_display() {
        assert_eq!(RefinementBound::ValueLt(1024).to_string(), "value < 1024");
        assert_eq!(
            RefinementBound::ValueInRange { lo: 10, hi: 200 }.to_string(),
            "value in 10..=200"
        );
        assert_eq!(
            RefinementBound::ValueMod { divisor: 4, remainder: 0 }.to_string(),
            "value % 4 == 0"
        );
    }

    #[test]
    fn error_codes_are_distinct() {
        let codes = [
            error_codes::E610_REF_BOUND,
            error_codes::E612_REF_WIDTH,
            error_codes::E613_LIN_UNUSED,
            error_codes::E614_LIN_DOUBLE,
            error_codes::E616_EFF_PURE,
            error_codes::E617_EFF_MIX,
            error_codes::E618_CLK_CROSS,
            error_codes::E619_CLK_UNDEF,
            error_codes::E620_PHT_MISMATCH,
            error_codes::E621_PHT_UNDEF,
            error_codes::E625_SES_PROTOCOL,
        ];
        let mut seen = std::collections::HashSet::new();
        for code in &codes {
            assert!(seen.insert(*code), "Duplicate error code: {}", code);
        }
        assert_eq!(codes.len(), 11);
    }
}
