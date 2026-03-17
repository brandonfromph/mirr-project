// ---------------------------------------------------------------------------
//! Core type definitions for the MIRR AST.
//!
//! Defines signal kinds (input/output/internal), signal types (bool, unsigned,
//! signed, array, struct, fixed-point, interface bundle), binary and unary
//! operators, and literal values.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

// =========================================================================
// MEGA-10 bounded-data constants (NASA Power-of-10: all collections bounded).
// =========================================================================

/// Maximum number of fields in a struct type.
pub const MAX_STRUCT_FIELDS: usize = 32;

/// Maximum nesting depth for array dimensions (e.g. `[u8; 4][4]` = depth 2).
pub const MAX_ARRAY_DIMS: usize = 4;

/// Maximum total bit width for a fixed-point type.
pub const MAX_FIXED_POINT_BITS: u32 = 64;

/// Maximum number of signals in an interface bundle.
pub const MAX_INTERFACE_SIGNALS: usize = 64;

/// Kind of signal in a MIRR module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    /// Hardware input port (read-only from module perspective).
    Input,
    /// Hardware output port (driven by reflexes).
    Output,
    /// Module-internal signal (persists across clock ticks).
    Internal,
}

/// Type of a signal (boolean, fixed-width unsigned, fixed-width signed,
/// fixed-size array, struct, fixed-point, or interface bundle).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalType {
    /// Single-bit boolean (true/false).
    Bool,
    /// Fixed-width unsigned integer (`u8`, `u16`, `u32`, `u64`).
    Unsigned(u32),
    /// Fixed-width signed two's complement integer (`i8`, `i16`, `i32`, `i64`).
    Signed(u32),
    /// Fixed-size array: `[element; length]`. Compiles to N registers + MUX tree.
    /// Length must be 1..=MAX_TYPE_NAT. Nesting depth bounded by MAX_ARRAY_DIMS.
    Array {
        /// Element type (scalar or nested array/struct).
        element: Box<SignalType>,
        /// Number of elements (statically known, >= 1).
        length: u64,
    },
    /// Named struct: `struct Name { field1: Type, ... }`. Compiles to concatenated wires.
    /// Field count bounded by MAX_STRUCT_FIELDS.
    Struct {
        /// Struct type name (must match a declared struct).
        name: String,
        /// Ordered fields: `(field_name, field_type)`.
        fields: Vec<(String, SignalType)>,
    },
    /// Fixed-point number: `fixed<total_bits, frac_bits>`. Compiles to integer ALU
    /// with implicit radix point. `total_bits` includes `frac_bits`.
    FixedPoint {
        /// Total bit width (integer + fractional).
        total_bits: u32,
        /// Number of fractional bits (must be <= total_bits).
        frac_bits: u32,
    },
    /// Interface bundle reference: `interface Name`. Compiles to a group of ports.
    /// Resolved during elaboration to the interface's constituent signals.
    Bundle(String),
}

impl std::fmt::Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalType::Bool => write!(f, "bool"),
            SignalType::Unsigned(width) => write!(f, "u{}", width),
            SignalType::Signed(width) => write!(f, "i{}", width),
            SignalType::Array { element, length } => write!(f, "{}[{}]", element, length),
            SignalType::Struct { name, .. } => write!(f, "struct {}", name),
            SignalType::FixedPoint { total_bits, frac_bits } => {
                write!(f, "fixed<{},{}>", total_bits, frac_bits)
            }
            SignalType::Bundle(name) => write!(f, "interface {}", name),
        }
    }
}

impl SignalType {
    /// Returns the bit width of this signal type.
    ///
    /// For composite types: arrays return `element_width * length`,
    /// structs return the sum of field widths, fixed-point returns `total_bits`,
    /// bundles return 0 (resolved during elaboration).
    pub fn width(&self) -> u32 {
        match self {
            SignalType::Bool => 1,
            SignalType::Unsigned(w) | SignalType::Signed(w) => *w,
            SignalType::Array { element, length } => element.width().saturating_mul(*length as u32),
            SignalType::Struct { fields, .. } => {
                let mut total: u32 = 0;
                let mut i = 0;
                while i < fields.len() {
                    total = total.saturating_add(fields[i].1.width());
                    i += 1;
                }
                total
            }
            SignalType::FixedPoint { total_bits, .. } => *total_bits,
            SignalType::Bundle(_) => 0,
        }
    }

    /// Returns `(bit_width, is_signed)`.
    pub fn width_and_signed(&self) -> (u32, bool) {
        match self {
            SignalType::Bool => (1, false),
            SignalType::Unsigned(w) => (*w, false),
            SignalType::Signed(w) => (*w, true),
            SignalType::Array { .. }
            | SignalType::Struct { .. }
            | SignalType::FixedPoint { .. }
            | SignalType::Bundle(_) => (self.width(), false),
        }
    }

    /// Returns true if this is a scalar type (Bool, Unsigned, Signed).
    pub fn is_scalar(&self) -> bool {
        matches!(self, SignalType::Bool | SignalType::Unsigned(_) | SignalType::Signed(_))
    }

    /// Returns true if this is a composite type (Array, Struct, FixedPoint, Bundle).
    pub fn is_composite(&self) -> bool {
        !self.is_scalar()
    }
}

/// Binary operator in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    /// Logical AND (`&&`). Requires bool operands.
    And,
    /// Logical OR (`||`). Requires bool operands.
    Or,
    /// Bitwise XOR (`^`). Requires matching types.
    Xor,
    /// Less than (`<`). Returns bool.
    Lt,
    /// Less than or equal (`<=`). Returns bool.
    Le,
    /// Greater than (`>`). Returns bool.
    Gt,
    /// Greater than or equal (`>=`). Returns bool.
    Ge,
    /// Equal (`==`). Returns bool.
    Eq,
    /// Not equal (`!=`). Returns bool.
    Ne,
    /// Addition (`+`). Requires numeric operands, same signedness.
    Add,
    /// Subtraction (`-`). Requires numeric operands, same signedness.
    Sub,
    /// Multiplication (`*`). Requires numeric operands, same signedness.
    Mul,
    /// Left shift (`<<`). Result width = left operand width.
    Shl,
    /// Right shift (`>>`). Result width = left operand width.
    Shr,
}

/// Unary operator in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// Logical/bitwise NOT (`!`). Works on bool, unsigned, and signed.
    Not,
    /// Arithmetic negation (`-`). Unsigned(N) -> Signed(N+1), Signed(N) -> Signed(N).
    Negate,
}

/// Literal value in an expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiteralValue {
    /// Boolean constant (`true` or `false`).
    Bool(bool),
    /// Unsigned integer constant (inferred as `Unsigned(min_bits)`).
    Integer(u64),
}

// =========================================================================
// MEGA-1 extended type annotations (linearity, effects, refinements,
// clock domains, phantom types).
// =========================================================================

/// Linearity qualifier for the MEGA-1 linear type system.
///
/// A `Linear` signal must be consumed exactly once per clock cycle,
/// preventing accidental fan-out of critical one-shot triggers.
/// Default is `Unrestricted` (no linearity constraint).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Linearity {
    /// No linearity constraint (default, backward-compatible).
    #[default]
    Unrestricted,
    /// Must be consumed exactly once per clock cycle.
    Linear,
}

impl Linearity {
    /// Returns true if unrestricted (used by serde `skip_serializing_if`).
    pub fn is_unrestricted(&self) -> bool {
        *self == Self::Unrestricted
    }
}

/// Effect annotation for MEGA-1 effect tracking.
///
/// Signals can be annotated as `Stateful` (carries state across clock
/// cycles, implies a register) or `Pure` (purely combinational, no
/// state — suitable for feed-through wiring).
/// Default is `Unspecified` (no effect annotation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EffectQualifier {
    /// No effect annotation (default, backward-compatible).
    #[default]
    Unspecified,
    /// Signal carries state across clock cycles (register-backed).
    Stateful,
    /// Signal is purely combinational (no register).
    Pure,
}

impl EffectQualifier {
    /// Returns true if unspecified (used by serde `skip_serializing_if`).
    pub fn is_unspecified(&self) -> bool {
        *self == Self::Unspecified
    }
}

/// Refinement constraint on a signal type (MEGA-1).
///
/// Constrains the valid range or values of a signal beyond its base type.
/// Refinements are checked at compile time where possible, and emitted as
/// SystemVerilog assertions at synthesis time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refinement {
    /// Inclusive range constraint: `where lo..hi` means `lo <= value <= hi`.
    Range {
        /// Minimum value (inclusive).
        lo: u64,
        /// Maximum value (inclusive).
        hi: u64,
    },
    /// Predicate constraint: `where value < 1024` stored as raw expression string.
    /// Semantic validation is deferred to the type-checking pass.
    Predicate(String),
}

/// Collected MEGA-1 type annotations beyond the base signal type.
///
/// All fields default to their zero/none values for backward compatibility.
/// When all fields are at their defaults, serde omits the annotations
/// entirely to preserve IR contract stability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeAnnotations {
    /// Linearity qualifier: `linear` keyword before the base type.
    #[serde(default, skip_serializing_if = "Linearity::is_unrestricted")]
    pub linearity: Linearity,
    /// Effect annotation: `stateful` or `pure` keyword before the base type.
    #[serde(default, skip_serializing_if = "EffectQualifier::is_unspecified")]
    pub effect: EffectQualifier,
    /// Refinement constraint: `where <range-or-predicate>` after the base type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refinement: Option<Refinement>,
    /// Clock domain assignment: `@<domain_name>` after the base type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_domain: Option<String>,
    /// Phantom type tag: `#<Tag>` after the base type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phantom_tag: Option<String>,
}

impl TypeAnnotations {
    /// Returns true if all annotations are at their default (empty) values.
    pub fn is_default(&self) -> bool {
        self.linearity == Linearity::Unrestricted
            && self.effect == EffectQualifier::Unspecified
            && self.refinement.is_none()
            && self.clock_domain.is_none()
            && self.phantom_tag.is_none()
    }
}

/// IR version. Bumped when the type representation changes.
pub const IR_VERSION: &str = "2.0";

/// Extended type wrapper: `SignalType` core + all MEGA-1 metadata.
///
/// `SignalType` remains the hardware-synthesizable core (Bool/Unsigned/Signed).
/// `ExtendedType` carries annotation metadata for type checking.
/// Emission and width inference access the core via `signal_type()`.
///
/// All optional fields use `#[serde(default)]` for backward compatibility:
/// deserializing v1 IR produces an `ExtendedType` with empty metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedType {
    /// The core hardware-synthesizable signal type.
    pub core: SignalType,
    /// MEGA-1 annotations parsed from the signal declaration.
    #[serde(default, skip_serializing_if = "TypeAnnotations::is_default")]
    pub annotations: TypeAnnotations,
}

impl ExtendedType {
    /// Create from a bare `SignalType` with default annotations.
    pub fn from_core(core: SignalType) -> Self {
        Self { core, annotations: TypeAnnotations::default() }
    }

    /// Create from a `SignalType` and pre-parsed `TypeAnnotations`.
    pub fn new(core: SignalType, annotations: TypeAnnotations) -> Self {
        Self { core, annotations }
    }

    /// Extract the core `SignalType` for emission and width inference.
    pub fn signal_type(&self) -> SignalType {
        self.core.clone()
    }
}

impl From<SignalType> for ExtendedType {
    fn from(core: SignalType) -> Self {
        Self::from_core(core)
    }
}
