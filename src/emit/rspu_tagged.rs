//! Tagged-word register file for the R-SPU ISA v2.
//!
//! Every register in the R-SPU v2 carries a runtime type tag and provenance
//! annotation alongside its 64-bit value.  This enables the interpreter and
//! verification back-end to detect type-mismatch bugs (e.g. adding a bool
//! to an unsigned) at execution time rather than silently producing garbage.
//!
//! All collections are bounded by `MAX_*` constants (NASA Power-of-10).
//! No recursion. No unsafe code.

#![forbid(unsafe_code)]

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ast::types::SignalType;
use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::{AluOp, PortId, RegId, MAX_REGISTERS};
use crate::error::MirrError;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum bit-width representable in a tagged word.
pub const MAX_TAGGED_WORD_BITS: u32 = 128;

// ---------------------------------------------------------------------------
// TypeTag
// ---------------------------------------------------------------------------

/// Runtime type tag carried alongside every register value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeTag {
    /// Single-bit boolean.
    Bool,
    /// Fixed-width unsigned integer.
    Unsigned { width: u8 },
    /// Fixed-width signed (two's complement) integer.
    Signed { width: u8 },
    /// Register has not been written yet.
    Uninitialized,
    /// Interval-tagged value with lower and upper bounds (MEGA-5).
    Interval { lo: u64, hi: u64 },
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Unsigned { width } => write!(f, "u{width}"),
            TypeTag::Signed { width } => write!(f, "i{width}"),
            TypeTag::Uninitialized => write!(f, "<uninitialized>"),
            TypeTag::Interval { lo, hi } => write!(f, "interval[{lo}, {hi}]"),
        }
    }
}

impl TypeTag {
    /// Returns `true` if the tag represents a numeric type (unsigned or signed).
    fn is_numeric(self) -> bool {
        matches!(self, TypeTag::Unsigned { .. } | TypeTag::Signed { .. } | TypeTag::Interval { .. })
    }
}

// ---------------------------------------------------------------------------
// Provenance
// ---------------------------------------------------------------------------

/// Tracks how a register value was produced, for diagnostic and verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provenance {
    /// Value was loaded from an input port.
    Input(PortId),
    /// Value was computed by an ALU operation.
    Computed,
    /// Value was loaded from an immediate/literal.
    Literal,
    /// Register has not been written (default state).
    Unset,
}

// ---------------------------------------------------------------------------
// TaggedWord
// ---------------------------------------------------------------------------

/// A register value paired with its type tag and provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaggedWord {
    /// The raw 64-bit value stored in the register.
    pub value: u64,
    /// Runtime type tag.
    pub tag: TypeTag,
    /// How this value was produced.
    pub provenance: Provenance,
}

impl TaggedWord {
    /// Create an uninitialized tagged word (default register state).
    pub fn uninitialized() -> Self {
        Self { value: 0, tag: TypeTag::Uninitialized, provenance: Provenance::Unset }
    }

    /// Create a tagged word loaded from an input port.
    pub fn from_input(value: u64, tag: TypeTag, port: PortId) -> Self {
        Self { value, tag, provenance: Provenance::Input(port) }
    }

    /// Create a tagged word from a literal/immediate value.
    pub fn from_literal(value: u64, tag: TypeTag) -> Self {
        Self { value, tag, provenance: Provenance::Literal }
    }

    /// Create a tagged word produced by an ALU computation.
    pub fn from_computed(value: u64, tag: TypeTag) -> Self {
        Self { value, tag, provenance: Provenance::Computed }
    }
}

// ---------------------------------------------------------------------------
// RegisterFile
// ---------------------------------------------------------------------------

/// Tagged register file: exactly `MAX_REGISTERS` entries, each carrying a
/// type tag and provenance annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterFile {
    /// Backing storage; always has exactly `MAX_REGISTERS` entries.
    registers: Vec<TaggedWord>,
}

impl RegisterFile {
    /// Create a new register file with all registers uninitialized.
    pub fn new() -> Self {
        let mut registers = Vec::with_capacity(MAX_REGISTERS);
        // Bounded: exactly MAX_REGISTERS iterations.
        for _i in 0..MAX_REGISTERS {
            registers.push(TaggedWord::uninitialized());
        }
        Self { registers }
    }

    /// Read the tagged word at `reg`. Panics if `reg` is out of bounds
    /// (should never happen with valid R-SPU programs, as RegId is u8 and
    /// MAX_REGISTERS is 256).
    pub fn read(&self, reg: RegId) -> &TaggedWord {
        let idx = reg as usize;
        assert!(
            idx < MAX_REGISTERS,
            "RegisterFile::read: index {idx} out of bounds (max {MAX_REGISTERS})"
        );
        &self.registers[idx]
    }

    /// Write a tagged word to `reg`. Panics if `reg` is out of bounds.
    pub fn write(&mut self, reg: RegId, word: TaggedWord) {
        let idx = reg as usize;
        assert!(
            idx < MAX_REGISTERS,
            "RegisterFile::write: index {idx} out of bounds (max {MAX_REGISTERS})"
        );
        self.registers[idx] = word;
    }

    /// Convenience: read just the type tag of a register.
    pub fn read_tag(&self, reg: RegId) -> TypeTag {
        self.read(reg).tag
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ALU tag checking
// ---------------------------------------------------------------------------

/// Validate that the operand tags are compatible with the given ALU operation,
/// and return the result type tag.
///
/// Returns `Err` with `[E708]` if either operand is uninitialized or if the
/// types are incompatible for the requested operation.
pub fn check_alu_tags(a: &TaggedWord, b: &TaggedWord, op: AluOp) -> Result<TypeTag, MirrError> {
    // Reject uninitialized operands.
    if a.tag == TypeTag::Uninitialized {
        return Err(rspu_err("[E708] tag violation: left operand is uninitialized".to_string()));
    }
    if b.tag == TypeTag::Uninitialized {
        return Err(rspu_err("[E708] tag violation: right operand is uninitialized".to_string()));
    }

    match op {
        // -- Comparison ops: both must be numeric or both bool; result is Bool --
        AluOp::Eq | AluOp::Ne | AluOp::Lt | AluOp::Le | AluOp::Gt | AluOp::Ge => {
            let compatible = matches!(
                (a.tag, b.tag),
                (TypeTag::Bool, TypeTag::Bool)
                    | (TypeTag::Unsigned { .. }, TypeTag::Unsigned { .. })
                    | (TypeTag::Signed { .. }, TypeTag::Signed { .. })
                    | (TypeTag::Interval { .. }, TypeTag::Interval { .. })
            );
            if !compatible {
                return Err(rspu_err(format!(
                    "[E708] tag violation: comparison requires matching types, got {} and {}",
                    a.tag, b.tag
                )));
            }
            Ok(TypeTag::Bool)
        }

        // -- Arithmetic ops: both must be numeric, same signedness; max width --
        AluOp::Add | AluOp::Sub | AluOp::Mul => match (a.tag, b.tag) {
            (TypeTag::Unsigned { width: wa }, TypeTag::Unsigned { width: wb }) => {
                Ok(TypeTag::Unsigned { width: wa.max(wb) })
            }
            (TypeTag::Signed { width: wa }, TypeTag::Signed { width: wb }) => {
                Ok(TypeTag::Signed { width: wa.max(wb) })
            }
            (TypeTag::Interval { .. }, TypeTag::Interval { .. }) => {
                Ok(TypeTag::Unsigned { width: 64 })
            }
            _ => Err(rspu_err(format!(
                "[E708] tag violation: arithmetic requires matching numeric types, got {} and {}",
                a.tag, b.tag
            ))),
        },

        // -- Bitwise ops: both must be matching types; max width --
        AluOp::And | AluOp::Or | AluOp::Xor => match (a.tag, b.tag) {
            (TypeTag::Bool, TypeTag::Bool) => Ok(TypeTag::Bool),
            (TypeTag::Unsigned { width: wa }, TypeTag::Unsigned { width: wb }) => {
                Ok(TypeTag::Unsigned { width: wa.max(wb) })
            }
            (TypeTag::Signed { width: wa }, TypeTag::Signed { width: wb }) => {
                Ok(TypeTag::Signed { width: wa.max(wb) })
            }
            (TypeTag::Interval { .. }, TypeTag::Interval { .. }) => {
                Ok(TypeTag::Unsigned { width: 64 })
            }
            _ => Err(rspu_err(format!(
                "[E708] tag violation: bitwise op requires matching types, got {} and {}",
                a.tag, b.tag
            ))),
        },

        // -- Shift ops: lhs must be numeric; result has lhs type --
        AluOp::Shl | AluOp::Shr => {
            if !a.tag.is_numeric() {
                return Err(rspu_err(format!(
                    "[E708] tag violation: shift requires numeric left operand, got {}",
                    a.tag
                )));
            }
            Ok(a.tag)
        }
    }
}

// ---------------------------------------------------------------------------
// Signal type conversion
// ---------------------------------------------------------------------------

/// Convert a MIRR `SignalType` to a runtime `TypeTag`.
pub fn tag_from_signal_type(ty: &SignalType) -> TypeTag {
    match ty {
        SignalType::Bool => TypeTag::Bool,
        SignalType::Unsigned(w) => TypeTag::Unsigned { width: *w as u8 },
        SignalType::Signed(w) => TypeTag::Signed { width: *w as u8 },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_file_new_all_uninitialized() {
        let rf = RegisterFile::new();
        // Bounded: MAX_REGISTERS iterations.
        for i in 0..MAX_REGISTERS {
            let word = rf.read(i as RegId);
            assert_eq!(word.value, 0);
            assert_eq!(word.tag, TypeTag::Uninitialized);
            assert_eq!(word.provenance, Provenance::Unset);
        }
    }

    #[test]
    fn test_register_file_write_read() {
        let mut rf = RegisterFile::new();
        let word = TaggedWord::from_literal(42, TypeTag::Unsigned { width: 8 });
        rf.write(10, word.clone());
        let read_back = rf.read(10);
        assert_eq!(read_back.value, 42);
        assert_eq!(read_back.tag, TypeTag::Unsigned { width: 8 });
        assert_eq!(read_back.provenance, Provenance::Literal);
    }

    #[test]
    fn test_check_alu_tags_unsigned_add() {
        let a = TaggedWord::from_literal(10, TypeTag::Unsigned { width: 8 });
        let b = TaggedWord::from_literal(20, TypeTag::Unsigned { width: 16 });
        let result = check_alu_tags(&a, &b, AluOp::Add).unwrap();
        assert_eq!(result, TypeTag::Unsigned { width: 16 });
    }

    #[test]
    fn test_check_alu_tags_uninit_fails() {
        let a = TaggedWord::uninitialized();
        let b = TaggedWord::from_literal(1, TypeTag::Unsigned { width: 8 });
        let result = check_alu_tags(&a, &b, AluOp::Add);
        assert!(result.is_err());
        let msg = result.unwrap_err().message().to_string();
        assert!(msg.contains("E708"));
        assert!(msg.contains("uninitialized"));
    }

    #[test]
    fn test_tag_from_signal_type() {
        assert_eq!(tag_from_signal_type(&SignalType::Bool), TypeTag::Bool);
        assert_eq!(
            tag_from_signal_type(&SignalType::Unsigned(32)),
            TypeTag::Unsigned { width: 32 }
        );
        assert_eq!(tag_from_signal_type(&SignalType::Signed(16)), TypeTag::Signed { width: 16 });
    }

    #[test]
    fn test_tagged_word_constructors() {
        // uninitialized
        let w = TaggedWord::uninitialized();
        assert_eq!(w.value, 0);
        assert_eq!(w.tag, TypeTag::Uninitialized);
        assert_eq!(w.provenance, Provenance::Unset);

        // from_input
        let w = TaggedWord::from_input(0xFF, TypeTag::Unsigned { width: 8 }, 3);
        assert_eq!(w.value, 0xFF);
        assert_eq!(w.tag, TypeTag::Unsigned { width: 8 });
        assert_eq!(w.provenance, Provenance::Input(3));

        // from_literal
        let w = TaggedWord::from_literal(1, TypeTag::Bool);
        assert_eq!(w.value, 1);
        assert_eq!(w.tag, TypeTag::Bool);
        assert_eq!(w.provenance, Provenance::Literal);

        // from_computed
        let w = TaggedWord::from_computed(999, TypeTag::Signed { width: 16 });
        assert_eq!(w.value, 999);
        assert_eq!(w.tag, TypeTag::Signed { width: 16 });
        assert_eq!(w.provenance, Provenance::Computed);
    }

    #[test]
    fn test_type_tag_display() {
        assert_eq!(format!("{}", TypeTag::Bool), "bool");
        assert_eq!(format!("{}", TypeTag::Unsigned { width: 8 }), "u8");
        assert_eq!(format!("{}", TypeTag::Signed { width: 32 }), "i32");
        assert_eq!(format!("{}", TypeTag::Uninitialized), "<uninitialized>");
        assert_eq!(format!("{}", TypeTag::Interval { lo: 0, hi: 255 }), "interval[0, 255]");
    }

    #[test]
    fn test_comparison_requires_matching_types() {
        let a = TaggedWord::from_literal(1, TypeTag::Bool);
        let b = TaggedWord::from_literal(2, TypeTag::Unsigned { width: 8 });
        assert!(check_alu_tags(&a, &b, AluOp::Eq).is_err());
    }

    #[test]
    fn test_comparison_bool_produces_bool() {
        let a = TaggedWord::from_literal(1, TypeTag::Bool);
        let b = TaggedWord::from_literal(0, TypeTag::Bool);
        let result = check_alu_tags(&a, &b, AluOp::Eq).unwrap();
        assert_eq!(result, TypeTag::Bool);
    }

    #[test]
    fn test_arithmetic_mixed_signedness_fails() {
        let a = TaggedWord::from_literal(10, TypeTag::Unsigned { width: 8 });
        let b = TaggedWord::from_literal(20, TypeTag::Signed { width: 8 });
        assert!(check_alu_tags(&a, &b, AluOp::Add).is_err());
    }

    #[test]
    fn test_bitwise_bool_ok() {
        let a = TaggedWord::from_literal(1, TypeTag::Bool);
        let b = TaggedWord::from_literal(0, TypeTag::Bool);
        let result = check_alu_tags(&a, &b, AluOp::And).unwrap();
        assert_eq!(result, TypeTag::Bool);
    }

    #[test]
    fn test_shift_non_numeric_fails() {
        let a = TaggedWord::from_literal(1, TypeTag::Bool);
        let b = TaggedWord::from_literal(2, TypeTag::Unsigned { width: 8 });
        assert!(check_alu_tags(&a, &b, AluOp::Shl).is_err());
    }

    #[test]
    fn test_shift_preserves_lhs_type() {
        let a = TaggedWord::from_literal(0x10, TypeTag::Unsigned { width: 16 });
        let b = TaggedWord::from_literal(4, TypeTag::Unsigned { width: 8 });
        let result = check_alu_tags(&a, &b, AluOp::Shl).unwrap();
        assert_eq!(result, TypeTag::Unsigned { width: 16 });
    }

    #[test]
    fn test_register_file_read_tag() {
        let mut rf = RegisterFile::new();
        assert_eq!(rf.read_tag(0), TypeTag::Uninitialized);
        rf.write(0, TaggedWord::from_literal(7, TypeTag::Unsigned { width: 8 }));
        assert_eq!(rf.read_tag(0), TypeTag::Unsigned { width: 8 });
    }
}
