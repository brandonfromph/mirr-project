#![forbid(unsafe_code)]

use nasa_rust_project::compiler::macro_proc::expand_macros;

#[test]
fn test_ergonomic_assignment_variant_1() {
    let out = expand_macros("let guard x_1 = when y_1 for 2 cycles;");
    assert!(out.contains("guard x_1 {"));
}

#[test]
fn test_ergonomic_assignment_variant_2() {
    let out = expand_macros("let guard x_2 = when y_2 for 2 cycles;");
    assert!(out.contains("guard x_2 {"));
}

#[test]
fn test_ergonomic_assignment_variant_3() {
    let out = expand_macros("let guard x_3 = when y_3 for 2 cycles;");
    assert!(out.contains("guard x_3 {"));
}

#[test]
fn test_ergonomic_assignment_variant_4() {
    let out = expand_macros("let guard x_4 = when y_4 for 2 cycles;");
    assert!(out.contains("guard x_4 {"));
}

#[test]
fn test_ergonomic_assignment_variant_5() {
    let out = expand_macros("let guard x_5 = when y_5 for 2 cycles;");
    assert!(out.contains("guard x_5 {"));
}

#[test]
fn test_ergonomic_assignment_variant_6() {
    let out = expand_macros("let guard x_6 = when y_6 for 2 cycles;");
    assert!(out.contains("guard x_6 {"));
}

#[test]
fn test_ergonomic_assignment_variant_7() {
    let out = expand_macros("let guard x_7 = when y_7 for 2 cycles;");
    assert!(out.contains("guard x_7 {"));
}

#[test]
fn test_ergonomic_assignment_variant_8() {
    let out = expand_macros("let guard x_8 = when y_8 for 2 cycles;");
    assert!(out.contains("guard x_8 {"));
}

#[test]
fn test_ergonomic_assignment_variant_9() {
    let out = expand_macros("let guard x_9 = when y_9 for 2 cycles;");
    assert!(out.contains("guard x_9 {"));
}

#[test]
fn test_ergonomic_assignment_variant_10() {
    let out = expand_macros("let tmp_10: bool = a_10 & b_10;");
    assert!(out.contains("tmp_10 = a_10 & b_10;"));
}

#[test]
fn test_ergonomic_assignment_variant_11() {
    let out = expand_macros("let tmp_11: bool = a_11 & b_11;");
    assert!(out.contains("tmp_11 = a_11 & b_11;"));
}

#[test]
fn test_ergonomic_assignment_variant_12() {
    let out = expand_macros("let tmp_12: bool = a_12 & b_12;");
    assert!(out.contains("tmp_12 = a_12 & b_12;"));
}

#[test]
fn test_ergonomic_assignment_variant_13() {
    let out = expand_macros("let tmp_13: bool = a_13 & b_13;");
    assert!(out.contains("tmp_13 = a_13 & b_13;"));
}

#[test]
fn test_ergonomic_assignment_variant_14() {
    let out = expand_macros("let tmp_14: bool = a_14 & b_14;");
    assert!(out.contains("tmp_14 = a_14 & b_14;"));
}

#[test]
fn test_ergonomic_assignment_variant_15() {
    let out = expand_macros("let tmp_15: bool = a_15 & b_15;");
    assert!(out.contains("tmp_15 = a_15 & b_15;"));
}

#[test]
fn test_ergonomic_assignment_variant_16() {
    let out = expand_macros("let tmp_16: bool = a_16 & b_16;");
    assert!(out.contains("tmp_16 = a_16 & b_16;"));
}

#[test]
fn test_ergonomic_assignment_variant_17() {
    let out = expand_macros("let tmp_17: bool = a_17 & b_17;");
    assert!(out.contains("tmp_17 = a_17 & b_17;"));
}

#[test]
fn test_ergonomic_assignment_variant_18() {
    let out = expand_macros("let tmp_18: bool = a_18 & b_18;");
    assert!(out.contains("tmp_18 = a_18 & b_18;"));
}

#[test]
fn test_ergonomic_assignment_variant_19() {
    let out = expand_macros("let tmp_19: bool = a_19 & b_19;");
    assert!(out.contains("tmp_19 = a_19 & b_19;"));
}

#[test]
fn test_ergonomic_assignment_variant_20() {
    let out = expand_macros("reflex {\nmatch x_20 {\n 0 => {\n a_20 = 1;\n }\n _ => {\n a_20 = 2;\n }\n}\n}");
    assert!(out.contains("on auto_g_"));
    assert!(out.contains("on always"));
}

#[test]
fn test_ergonomic_assignment_variant_21() {
    let out = expand_macros("reflex {\nmatch x_21 {\n 0 => {\n a_21 = 1;\n }\n _ => {\n a_21 = 2;\n }\n}\n}");
    assert!(out.contains("on auto_g_"));
    assert!(out.contains("on always"));
}

#[test]
fn test_ergonomic_assignment_variant_22() {
    let out = expand_macros("reflex {\nmatch x_22 {\n 0 => {\n a_22 = 1;\n }\n _ => {\n a_22 = 2;\n }\n}\n}");
    assert!(out.contains("on auto_g_"));
    assert!(out.contains("on always"));
}

#[test]
fn test_ergonomic_assignment_variant_23() {
    let out = expand_macros("reflex {\nmatch x_23 {\n 0 => {\n a_23 = 1;\n }\n _ => {\n a_23 = 2;\n }\n}\n}");
    assert!(out.contains("on auto_g_"));
    assert!(out.contains("on always"));
}

#[test]
fn test_ergonomic_assignment_variant_24() {
    let out = expand_macros("reflex {\nmatch x_24 {\n 0 => {\n a_24 = 1;\n }\n _ => {\n a_24 = 2;\n }\n}\n}");
    assert!(out.contains("on auto_g_"));
    assert!(out.contains("on always"));
}

#[test]
fn test_ergonomic_assignment_variant_25() {
    let out = expand_macros("reflex {\nmatch x_25 {\n 0 => {\n a_25 = 1;\n }\n _ => {\n a_25 = 2;\n }\n}\n}");
    assert!(out.contains("on auto_g_"));
    assert!(out.contains("on always"));
}

