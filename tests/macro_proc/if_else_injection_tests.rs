#![forbid(unsafe_code)]

use nasa_rust_project::compiler::macro_proc::expand_macros;

#[test]
fn test_if_else_injection_variant_1() {
    let out = expand_macros("reflex {\n if a_1 {\n b_1 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_2() {
    let out = expand_macros("reflex {\n if a_2 {\n b_2 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_3() {
    let out = expand_macros("reflex {\n if a_3 {\n b_3 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_4() {
    let out = expand_macros("reflex {\n if a_4 {\n b_4 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_5() {
    let out = expand_macros("reflex {\n if a_5 {\n b_5 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_6() {
    let out = expand_macros("reflex {\n if a_6 {\n b_6 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_7() {
    let out = expand_macros("reflex {\n if a_7 {\n b_7 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_8() {
    let out = expand_macros("reflex {\n if a_8 {\n b_8 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_9() {
    let out = expand_macros("reflex {\n if a_9 {\n b_9 = 1; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_10() {
    let out = expand_macros("reflex {\n if true {\n b_10 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_11() {
    let out = expand_macros("reflex {\n if true {\n b_11 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_12() {
    let out = expand_macros("reflex {\n if true {\n b_12 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_13() {
    let out = expand_macros("reflex {\n if true {\n b_13 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_14() {
    let out = expand_macros("reflex {\n if true {\n b_14 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_15() {
    let out = expand_macros("reflex {\n if true {\n b_15 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_16() {
    let out = expand_macros("reflex {\n if true {\n b_16 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_17() {
    let out = expand_macros("reflex {\n if true {\n b_17 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_18() {
    let out = expand_macros("reflex {\n if true {\n b_18 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_19() {
    let out = expand_macros("reflex {\n if true {\n b_19 = 1; \n} \n}");
    assert!(out.contains("always"));
}

#[test]
fn test_if_else_injection_variant_20() {
    let out = expand_macros("reflex {\n if c_20 {\n b_20 = 1; \n} else if d_20 {\n b_20 = 2; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_21() {
    let out = expand_macros("reflex {\n if c_21 {\n b_21 = 1; \n} else if d_21 {\n b_21 = 2; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_22() {
    let out = expand_macros("reflex {\n if c_22 {\n b_22 = 1; \n} else if d_22 {\n b_22 = 2; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_23() {
    let out = expand_macros("reflex {\n if c_23 {\n b_23 = 1; \n} else if d_23 {\n b_23 = 2; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_24() {
    let out = expand_macros("reflex {\n if c_24 {\n b_24 = 1; \n} else if d_24 {\n b_24 = 2; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

#[test]
fn test_if_else_injection_variant_25() {
    let out = expand_macros("reflex {\n if c_25 {\n b_25 = 1; \n} else if d_25 {\n b_25 = 2; \n} \n}");
    assert!(out.contains("on auto_g_"));
}

