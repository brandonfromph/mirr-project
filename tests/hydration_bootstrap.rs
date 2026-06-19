#![cfg(feature = "legacy_ast")]
use mirrc::parser::parse_mirr;
use mirrc::pipeline::{run_pipeline_with_file, PipelineConfig};

fn run_test(source: &str) -> Result<(), String> {
    let config = PipelineConfig { bootstrap_mode: true, ..Default::default() };
    match run_pipeline_with_file(source, "test.mirr", &config) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

// =========================================================================
// Category 1: Indexing & Read Operations on Numeric/Unsigned Types
// =========================================================================

#[test]
fn test_01_u5_indexing_read() {
    let source = "
    module test {
        signal s1: u5;
        signal b: bool;
        reflex r { on always { b = s1[0]; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_02_u5_indexing_expression() {
    let source = "
    module test {
        signal s1: u5;
        signal s2: u5;
        signal b: bool;
        reflex r { on always { b = s1[0] && s2[1]; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_03_u16_indexing_read() {
    let source = "
    module test {
        signal s: u16;
        signal b: bool;
        reflex r { on always { b = s[15]; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_04_i32_indexing_read() {
    let source = "
    module test {
        signal s: i32;
        signal b: bool;
        reflex r { on always { b = s[0]; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_05_u1_indexing_always_bool() {
    let source = "
    module test {
        signal s: u1;
        signal b: bool;
        reflex r { on always { b = s[0]; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

// =========================================================================
// Category 2: Bit Vector Reconstruction & Logical Operators
// =========================================================================

#[test]
fn test_06_vector_construction_from_bits() {
    let source = "
    module test {
        signal b0: bool;
        signal b1: bool;
        signal b2: bool;
        signal s: u3;
        reflex r { on always { s = b0 || (b1 << 1) || (b2 << 2); } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_07_vector_construction_explicit_width() {
    let source = "
    module test {
        signal b0: bool;
        signal b1: bool;
        signal s: u2;
        reflex r { on always { s = b0 || (b1 << 1); } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_08_large_vector_reconstruction() {
    // Keep expression on a single line so MIRR line-based parser parses it correctly
    let source = "
    module test {
        signal b0: bool; signal b1: bool; signal b2: bool; signal b3: bool; signal b4: bool; signal b5: bool; signal b6: bool; signal b7: bool;
        signal s: u8;
        reflex r { on always { s = b0 || (b1 << 1) || (b2 << 2) || (b3 << 3) || (b4 << 4) || (b5 << 5) || (b6 << 6) || (b7 << 7); } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_09_nested_logic_reconstruction() {
    let source = "
    module test {
        signal b0: bool; signal b1: bool; signal b2: bool; signal b3: bool;
        signal s: u4;
        reflex r { on always { s = ((b0 || (b1 << 1)) || (b2 << 2)) || (b3 << 3); } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_10_mux_logic_hydration_pattern() {
    let source = "
    module test {
        signal s: bool;
        signal a: u8;
        signal b: u8;
        signal y: u8;
        reflex r { on always { y = (s && b) || (!s && a); } }
    }
    ";
    assert!(run_test(source).is_ok());
}

// =========================================================================
// Category 3: Shift Operations & Promotions
// =========================================================================

#[test]
fn test_11_bool_as_u1_in_arithmetic() {
    let source = "
    module test {
        signal b: bool;
        signal s: u8;
        reflex r { on always { s = b + 1; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_12_mixed_width_bitwise_and() {
    let source = "
    module test {
        signal s8: u8;
        signal s1: bool;
        signal res: u8;
        reflex r { on always { res = s8 && s1; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_13_shift_by_bool() {
    let source = "
    module test {
        signal s: u8;
        signal b: bool;
        signal res: u8;
        reflex r { on always { res = s << b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_14_indexed_bit_as_shift_operand() {
    let source = "
    module test {
        signal s: u8;
        signal ctrl: u8;
        signal res: u8;
        reflex r { on always { res = s << ctrl[2]; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_15_shift_left_by_u5() {
    let source = "
    module test {
        signal s: u32;
        signal shamt: u5;
        signal res: u32;
        reflex r { on always { res = s << shamt; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_16_shift_right_by_u5() {
    let source = "
    module test {
        signal s: u32;
        signal shamt: u5;
        signal res: u32;
        reflex r { on always { res = s >> shamt; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_17_shift_amount_larger_than_width() {
    let source = "
    module test {
        signal s: u8;
        signal res: u8;
        reflex r { on always { res = s << 10; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

// =========================================================================
// Category 4: Numeric & Assignment Compatibility Rules
// =========================================================================

#[test]
fn test_18_constant_0_1_compatibility() {
    let source = "
    module test {
        signal b: bool;
        signal u: u32;
        reflex r { on always { b = 0; u = 1; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_19_bool_bitwise_and_u1() {
    let source = "
    module test {
        signal a: bool;
        signal b: u1;
        signal res: u1;
        reflex r { on always { res = a && b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_20_bitwise_not_on_bool() {
    let source = "
    module test {
        signal a: bool;
        signal res: bool;
        reflex r { on always { res = !a; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_21_bitwise_not_on_u8() {
    let source = "
    module test {
        signal a: u8;
        signal res: u8;
        reflex r { on always { res = !a; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_22_bitwise_xor_matching_widths() {
    let source = "
    module test {
        signal a: u8;
        signal b: u8;
        signal res: u8;
        reflex r { on always { res = a ^ b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

// =========================================================================
// Category 5: Invalid Assignment & Indexing Rules (Assert Fails)
// =========================================================================

#[test]
fn test_23_bit_level_assignment_internal_rejected() {
    // LHS indexing is not allowed in MIRR (it generates semantic error E207)
    let source = "
    module test {
        signal s: u5;
        signal b: bool;
        reflex r { on always { s[0] = b; } }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_24_bitwise_xor_mismatched_widths_rejected() {
    let source = "
    module test {
        signal a: u8;
        signal b: u16;
        signal res: u8;
        reflex r { on always { res = a ^ b; } }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_25_bool_to_unsigned_assignment_rejected() {
    let source = "
    module test {
        signal b: bool;
        signal u: u8;
        reflex r { on always { u = b; } }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_26_unsigned_to_bool_assignment_rejected() {
    let source = "
    module test {
        signal b: bool;
        signal u: u8;
        reflex r { on always { b = u; } }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_27_write_to_input_signal_rejected() {
    let source = "
    module test {
        signal s: in u8;
        reflex r { on always { s = 1; } }
    }
    ";
    assert!(run_test(source).is_err());
}

// =========================================================================
// Category 6: Temporal & Prev Operators
// =========================================================================

#[test]
fn test_28_prev_on_indexed_bit() {
    let source = "
    module test {
        signal s: u5;
        signal b: bool;
        reflex r { on always { b = prev(s, 1)[0]; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_29_prev_delay_one() {
    let source = "
    module test {
        signal s: u8;
        signal p: u8;
        reflex r { on always { p = prev(s, 1); } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_30_prev_delay_large() {
    let source = "
    module test {
        signal s: u8;
        signal p: u8;
        reflex r { on always { p = prev(s, 5); } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_31_prev_on_expression_rejected() {
    let source = "
    module test {
        signal a: u8;
        signal b: u8;
        signal p: u8;
        reflex r { on always { p = prev(a + b, 1); } }
    }
    ";
    assert!(run_test(source).is_err());
}

// =========================================================================
// Category 7: Guard & Reflex Gating Semantics
// =========================================================================

#[test]
fn test_32_always_guard_sentinel() {
    let source = "
    module test {
        signal s: u8;
        reflex r { on always { s = 5; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_33_temporal_guard_compilation() {
    let source = "
    module test {
        signal clk: bool;
        signal trigger: bool;
        signal s: u8;
        guard g_trigger { when trigger for 3 cycles; }
        reflex r { on g_trigger { s = 10; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_34_guard_condition_must_be_bool_rejected() {
    let source = "
    module test {
        signal s: u8;
        guard g_trigger { when s for 1 cycles; }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_35_undeclared_signal_in_reflex_rejected() {
    let source = "
    module test {
        reflex r { on always { s_undeclared = 5; } }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_36_undeclared_signal_in_rhs_rejected() {
    let source = "
    module test {
        signal s: u8;
        reflex r { on always { s = s_undeclared; } }
    }
    ";
    assert!(run_test(source).is_err());
}

// =========================================================================
// Category 8: Module Signal Definitions & Ownership Rules
// =========================================================================

#[test]
fn test_37_write_to_output_signal() {
    let source = "
    module test {
        signal s: out u8;
        reflex r { on always { s = 5; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_38_write_to_internal_signal() {
    let source = "
    module test {
        signal s: u8;
        reflex r { on always { s = 5; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_39_multiple_writers_conflict_rejected() {
    let source = "
    module test {
        signal s: u8;
        reflex r1 { on always { s = 5; } }
        reflex r2 { on always { s = 10; } }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_40_multiple_writers_no_conflict() {
    let source = "
    module test {
        signal cond: bool;
        signal s: u8;
        guard g1 { when cond for 1 cycles; }
        guard g2 { when !cond for 1 cycles; }
        reflex r1 { on g1 { s = 5; } }
        reflex r2 { on g2 { s = 10; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

// =========================================================================
// Category 9: Comparisons & Arithmetic Operations
// =========================================================================

#[test]
fn test_41_comparison_unsigned() {
    let source = "
    module test {
        signal a: u8;
        signal b: u8;
        signal res: bool;
        reflex r { on always { res = a < b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_42_comparison_signed() {
    let source = "
    module test {
        signal a: i8;
        signal b: i8;
        signal res: bool;
        reflex r { on always { res = a < b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_43_arithmetic_add_unsigned() {
    let source = "
    module test {
        signal a: u8;
        signal b: u8;
        signal res: u8;
        reflex r { on always { res = a + b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_44_arithmetic_sub_unsigned() {
    let source = "
    module test {
        signal a: u8;
        signal b: u8;
        signal res: u8;
        reflex r { on always { res = a - b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_45_arithmetic_mul_unsigned() {
    let source = "
    module test {
        signal a: u8;
        signal b: u8;
        signal res: u8;
        reflex r { on always { res = a * b; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_46_arithmetic_mixed_signedness_rejected() {
    let source = "
    module test {
        signal a: u8;
        signal b: i8;
        signal res: u8;
        reflex r { on always { res = a + b; } }
    }
    ";
    assert!(run_test(source).is_err());
}

// =========================================================================
// Category 10: Advanced Complex Types, Structs & Semantic Gaps
// =========================================================================

#[test]
fn test_47_struct_signal_declaration() {
    let source = "
    struct Point {
        x: u8;
        y: u8;
    }
    module test {
        signal p: struct Point;
        signal x_val: u8;
        reflex r { on always { x_val = p.x; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_48_struct_field_access_invalid_rejected() {
    let source = "
    struct Point {
        x: u8;
        y: u8;
    }
    module test {
        signal p: struct Point;
        signal z_val: u8;
        reflex r { on always { z_val = p.z; } }
    }
    ";
    assert!(run_test(source).is_err());
}

#[test]
fn test_49_complex_expression_typecheck() {
    let source = "
    module test {
        signal a: u8;
        signal b: u8;
        signal c: u8;
        signal res: u8;
        reflex r { on always { res = (a + b) * c; } }
    }
    ";
    assert!(run_test(source).is_ok());
}

#[test]
fn test_50_zero_width_signal_rejected() {
    // Width cannot be 0, parser will reject or typecheck will fail
    let source = "
    module test {
        signal s: u0;
    }
    ";
    assert!(run_test(source).is_err());
}
