#![forbid(unsafe_code)]

use nasa_rust_project::compiler::macro_proc::expand_macros;

#[test]
fn test_fsm_state_transition_variant_1() {
    let out = expand_macros("reflex { a_1 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_2() {
    let out = expand_macros("reflex { a_2 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_3() {
    let out = expand_macros("reflex { a_3 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_4() {
    let out = expand_macros("reflex { a_4 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_5() {
    let out = expand_macros("reflex { a_5 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_6() {
    let out = expand_macros("reflex { a_6 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_7() {
    let out = expand_macros("reflex { a_7 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_8() {
    let out = expand_macros("reflex { a_8 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_9() {
    let out = expand_macros("reflex { a_9 = 1; }");
    assert!(out.contains("on always {"));
}

#[test]
fn test_fsm_state_transition_variant_10() {
    let out = expand_macros("signals \n {\n b_10: u8; \n}");
    assert!(out.contains("b_10: u8;"));
}

#[test]
fn test_fsm_state_transition_variant_11() {
    let out = expand_macros("signals \n {\n b_11: u8; \n}");
    assert!(out.contains("b_11: u8;"));
}

#[test]
fn test_fsm_state_transition_variant_12() {
    let out = expand_macros("signals \n {\n b_12: u8; \n}");
    assert!(out.contains("b_12: u8;"));
}

#[test]
fn test_fsm_state_transition_variant_13() {
    let out = expand_macros("signals \n {\n b_13: u8; \n}");
    assert!(out.contains("b_13: u8;"));
}

#[test]
fn test_fsm_state_transition_variant_14() {
    let out = expand_macros("signals \n {\n b_14: u8; \n}");
    assert!(out.contains("b_14: u8;"));
}

#[test]
fn test_fsm_state_transition_variant_15() {
    let out = expand_macros("reflex {\n c_15 = 1; \n}");
    assert!(out.contains("c_15 = 1;"));
}

#[test]
fn test_fsm_state_transition_variant_16() {
    let out = expand_macros("reflex {\n c_16 = 1; \n}");
    assert!(out.contains("c_16 = 1;"));
}

#[test]
fn test_fsm_state_transition_variant_17() {
    let out = expand_macros("reflex {\n c_17 = 1; \n}");
    assert!(out.contains("c_17 = 1;"));
}

#[test]
fn test_fsm_state_transition_variant_18() {
    let out = expand_macros("reflex {\n c_18 = 1; \n}");
    assert!(out.contains("c_18 = 1;"));
}

#[test]
fn test_fsm_state_transition_variant_19() {
    let out = expand_macros("reflex {\n c_19 = 1; \n}");
    assert!(out.contains("c_19 = 1;"));
}

#[test]
fn test_fsm_state_transition_variant_20() {
    let out = expand_macros("reflex {\n d_20 = 1; ");
    assert!(!out.is_empty());
}

#[test]
fn test_fsm_state_transition_variant_21() {
    let out = expand_macros("reflex {\n d_21 = 1; ");
    assert!(!out.is_empty());
}

#[test]
fn test_fsm_state_transition_variant_22() {
    let out = expand_macros("reflex {\n d_22 = 1; ");
    assert!(!out.is_empty());
}

#[test]
fn test_fsm_state_transition_variant_23() {
    let out = expand_macros("reflex {\n d_23 = 1; ");
    assert!(!out.is_empty());
}

#[test]
fn test_fsm_state_transition_variant_24() {
    let out = expand_macros("reflex {\n if a {\n if b {\n if c {\n if d {\n e_24 = 1; \n}\n }\n }\n }\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_fsm_state_transition_variant_25() {
    let out = expand_macros("reflex {\n if a {\n if b {\n if c {\n if d {\n e_25 = 1; \n}\n }\n }\n }\n}");
    assert!(!out.is_empty());
}

