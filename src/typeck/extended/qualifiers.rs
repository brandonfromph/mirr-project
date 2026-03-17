//! Type qualifiers, clock domains, phantom tags, session types, error codes,
//! and signal declarations.
//!
//! Part of the MEGA-1 Extended Type System.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::ast::types::SignalType;
use crate::span::Span;

use super::types::*;

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
