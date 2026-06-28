//! R-SPU tagged register file tests — exercises `src/emit/rspu_tagged.rs`
//! branches not covered by inline tests:
//!   - check_alu_tags: right operand uninitialized, Interval arithmetic,
//!     Interval comparisons, Interval bitwise, signed arithmetic width merge,
//!     signed comparison, signed bitwise, shift with Interval lhs
//!   - RegisterFile: len/is_empty, new_with_size, get_all_values
//!   - tag_from_signal_type for Array/Struct/FixedPoint/Bundle/Fifo

#![forbid(unsafe_code)]

use mirrc::ast::types::SignalType;
use mirrc::emit::rspu_isa::AluOp;
use mirrc::emit::rspu_tagged::{
    check_alu_tags, tag_from_signal_type, RegisterFile, TaggedWord, TypeTag,
};

// -----------------------------------------------------------------------
// check_alu_tags: right operand uninitialized
// -----------------------------------------------------------------------
#[test]
fn right_operand_uninitialized_fails_e708() {
    let a = TaggedWord::from_literal(1, TypeTag::Unsigned { width: 8 });
    let b = TaggedWord::uninitialized();
    let err = check_alu_tags(&a, &b, AluOp::Add).unwrap_err();
    assert!(err.message().contains("E708"));
    assert!(err.message().contains("right operand"));
}

// -----------------------------------------------------------------------
// Interval arithmetic
// -----------------------------------------------------------------------
#[test]
fn interval_add_produces_unsigned_64() {
    let a = TaggedWord::from_computed(10, TypeTag::Interval { lo: 0, hi: 100 });
    let b = TaggedWord::from_computed(20, TypeTag::Interval { lo: 0, hi: 200 });
    let result = check_alu_tags(&a, &b, AluOp::Add).unwrap();
    assert_eq!(result, TypeTag::Unsigned { width: 64 });
}

#[test]
fn interval_sub_produces_unsigned_64() {
    let a = TaggedWord::from_computed(50, TypeTag::Interval { lo: 0, hi: 100 });
    let b = TaggedWord::from_computed(10, TypeTag::Interval { lo: 0, hi: 50 });
    let result = check_alu_tags(&a, &b, AluOp::Sub).unwrap();
    assert_eq!(result, TypeTag::Unsigned { width: 64 });
}

#[test]
fn interval_mul_produces_unsigned_64() {
    let a = TaggedWord::from_computed(5, TypeTag::Interval { lo: 0, hi: 10 });
    let b = TaggedWord::from_computed(3, TypeTag::Interval { lo: 0, hi: 10 });
    let result = check_alu_tags(&a, &b, AluOp::Mul).unwrap();
    assert_eq!(result, TypeTag::Unsigned { width: 64 });
}

// -----------------------------------------------------------------------
// Interval comparisons
// -----------------------------------------------------------------------
#[test]
fn interval_eq_produces_bool() {
    let a = TaggedWord::from_computed(1, TypeTag::Interval { lo: 0, hi: 10 });
    let b = TaggedWord::from_computed(1, TypeTag::Interval { lo: 0, hi: 10 });
    let result = check_alu_tags(&a, &b, AluOp::Eq).unwrap();
    assert_eq!(result, TypeTag::Bool);
}

#[test]
fn interval_lt_produces_bool() {
    let a = TaggedWord::from_computed(1, TypeTag::Interval { lo: 0, hi: 10 });
    let b = TaggedWord::from_computed(5, TypeTag::Interval { lo: 0, hi: 10 });
    let result = check_alu_tags(&a, &b, AluOp::Lt).unwrap();
    assert_eq!(result, TypeTag::Bool);
}

// -----------------------------------------------------------------------
// Interval bitwise
// -----------------------------------------------------------------------
#[test]
fn interval_and_produces_unsigned_64() {
    let a = TaggedWord::from_computed(0xFF, TypeTag::Interval { lo: 0, hi: 255 });
    let b = TaggedWord::from_computed(0x0F, TypeTag::Interval { lo: 0, hi: 255 });
    let result = check_alu_tags(&a, &b, AluOp::And).unwrap();
    assert_eq!(result, TypeTag::Unsigned { width: 64 });
}

#[test]
fn interval_or_produces_unsigned_64() {
    let a = TaggedWord::from_computed(0xF0, TypeTag::Interval { lo: 0, hi: 255 });
    let b = TaggedWord::from_computed(0x0F, TypeTag::Interval { lo: 0, hi: 255 });
    let result = check_alu_tags(&a, &b, AluOp::Or).unwrap();
    assert_eq!(result, TypeTag::Unsigned { width: 64 });
}

#[test]
fn interval_xor_produces_unsigned_64() {
    let a = TaggedWord::from_computed(0xFF, TypeTag::Interval { lo: 0, hi: 255 });
    let b = TaggedWord::from_computed(0x0F, TypeTag::Interval { lo: 0, hi: 255 });
    let result = check_alu_tags(&a, &b, AluOp::Xor).unwrap();
    assert_eq!(result, TypeTag::Unsigned { width: 64 });
}

// -----------------------------------------------------------------------
// Signed arithmetic / comparison / bitwise
// -----------------------------------------------------------------------
#[test]
fn signed_add_width_merge() {
    let a = TaggedWord::from_literal(10, TypeTag::Signed { width: 16 });
    let b = TaggedWord::from_literal(20, TypeTag::Signed { width: 32 });
    let result = check_alu_tags(&a, &b, AluOp::Add).unwrap();
    assert_eq!(result, TypeTag::Signed { width: 32 });
}

#[test]
fn signed_sub_width_merge() {
    let a = TaggedWord::from_literal(10, TypeTag::Signed { width: 32 });
    let b = TaggedWord::from_literal(20, TypeTag::Signed { width: 16 });
    let result = check_alu_tags(&a, &b, AluOp::Sub).unwrap();
    assert_eq!(result, TypeTag::Signed { width: 32 });
}

#[test]
fn signed_comparison_produces_bool() {
    let a = TaggedWord::from_literal(10, TypeTag::Signed { width: 16 });
    let b = TaggedWord::from_literal(20, TypeTag::Signed { width: 16 });
    for op in [AluOp::Eq, AluOp::Ne, AluOp::Lt, AluOp::Le, AluOp::Gt, AluOp::Ge] {
        let result = check_alu_tags(&a, &b, op).unwrap();
        assert_eq!(result, TypeTag::Bool);
    }
}

#[test]
fn signed_bitwise_and_width_merge() {
    let a = TaggedWord::from_literal(0xFF, TypeTag::Signed { width: 8 });
    let b = TaggedWord::from_literal(0x0F, TypeTag::Signed { width: 16 });
    let result = check_alu_tags(&a, &b, AluOp::And).unwrap();
    assert_eq!(result, TypeTag::Signed { width: 16 });
}

// -----------------------------------------------------------------------
// Cross-type errors
// -----------------------------------------------------------------------
#[test]
fn unsigned_vs_signed_arithmetic_fails() {
    let a = TaggedWord::from_literal(1, TypeTag::Unsigned { width: 8 });
    let b = TaggedWord::from_literal(1, TypeTag::Signed { width: 8 });
    let err = check_alu_tags(&a, &b, AluOp::Add).unwrap_err();
    assert!(err.message().contains("E708"));
}

#[test]
fn bool_vs_unsigned_bitwise_fails() {
    let a = TaggedWord::from_literal(1, TypeTag::Bool);
    let b = TaggedWord::from_literal(1, TypeTag::Unsigned { width: 8 });
    let err = check_alu_tags(&a, &b, AluOp::And).unwrap_err();
    assert!(err.message().contains("E708"));
}

#[test]
fn bool_arithmetic_fails() {
    let a = TaggedWord::from_literal(1, TypeTag::Bool);
    let b = TaggedWord::from_literal(0, TypeTag::Bool);
    let err = check_alu_tags(&a, &b, AluOp::Add).unwrap_err();
    assert!(err.message().contains("E708"));
}

// -----------------------------------------------------------------------
// Shift with Interval lhs
// -----------------------------------------------------------------------
#[test]
fn shift_interval_lhs_preserves_interval_type() {
    let a = TaggedWord::from_computed(8, TypeTag::Interval { lo: 0, hi: 100 });
    let b = TaggedWord::from_literal(2, TypeTag::Unsigned { width: 8 });
    let result = check_alu_tags(&a, &b, AluOp::Shl).unwrap();
    assert_eq!(result, TypeTag::Interval { lo: 0, hi: 100 });
}

#[test]
fn shift_signed_lhs_preserves_signed_type() {
    let a = TaggedWord::from_literal(8, TypeTag::Signed { width: 32 });
    let b = TaggedWord::from_literal(2, TypeTag::Unsigned { width: 8 });
    let result = check_alu_tags(&a, &b, AluOp::Shr).unwrap();
    assert_eq!(result, TypeTag::Signed { width: 32 });
}

// -----------------------------------------------------------------------
// RegisterFile: len, is_empty, new_with_size, get_all_values
// -----------------------------------------------------------------------
#[test]
fn register_file_len_and_is_empty() {
    let rf = RegisterFile::new();
    assert_eq!(rf.len(), 1024);
    assert!(!rf.is_empty());

    let small = RegisterFile::new_with_size(0);
    assert_eq!(small.len(), 0);
    assert!(small.is_empty());
}

#[test]
fn register_file_custom_size() {
    let rf = RegisterFile::new_with_size(4);
    assert_eq!(rf.len(), 4);
    for i in 0..4 {
        assert_eq!(rf.read(i).tag, TypeTag::Uninitialized);
    }
}

#[test]
fn register_file_get_all_values() {
    let mut rf = RegisterFile::new_with_size(2);
    rf.write(0, TaggedWord::from_literal(42, TypeTag::Unsigned { width: 8 }));
    rf.write(1, TaggedWord::from_literal(99, TypeTag::Bool));
    let all = rf.get_all_values();
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].value, 42);
    assert_eq!(all[1].value, 99);
}

// -----------------------------------------------------------------------
// tag_from_signal_type for aggregate types
// -----------------------------------------------------------------------
#[test]
fn tag_from_array_type_is_unsigned_0() {
    let ty = SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 4 };
    let tag = tag_from_signal_type(&ty);
    assert_eq!(tag, TypeTag::Unsigned { width: 0 });
}

#[test]
fn tag_from_struct_type_is_unsigned_0() {
    let ty = SignalType::Struct {
        name: "TestStruct".to_string(),
        fields: vec![("a".to_string(), SignalType::Bool)],
    };
    let tag = tag_from_signal_type(&ty);
    assert_eq!(tag, TypeTag::Unsigned { width: 0 });
}

#[test]
fn tag_from_fixed_point_is_unsigned_0() {
    let ty = SignalType::FixedPoint { total_bits: 16, frac_bits: 8 };
    let tag = tag_from_signal_type(&ty);
    assert_eq!(tag, TypeTag::Unsigned { width: 0 });
}

#[test]
fn tag_from_bundle_is_unsigned_0() {
    let ty = SignalType::Bundle("TestBundle".to_string());
    let tag = tag_from_signal_type(&ty);
    assert_eq!(tag, TypeTag::Unsigned { width: 0 });
}

#[test]
fn tag_from_fifo_is_unsigned_0() {
    let ty = SignalType::Fifo { element: Box::new(SignalType::Unsigned(8)), depth: 16 };
    let tag = tag_from_signal_type(&ty);
    assert_eq!(tag, TypeTag::Unsigned { width: 0 });
}
