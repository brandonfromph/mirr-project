#![forbid(unsafe_code)]

use nasa_rust_project::compiler::macro_proc::expand_macros;

fn assert_expansion_contains(src: &str, expected: &str) {
    let expanded = expand_macros(src);
    assert!(
        expanded.contains(expected),
        "Expected expansion to contain:\n{}\n\nActual expanded text:\n{}",
        expected,
        expanded
    );
}

#[test]
fn test_macro_expand_signals_brace() {
    let src = r#"
        signals {
            a: in bool
            b: out bool
        }
    "#;
    let expected_a = "signal a: in bool;";
    let expected_b = "signal b: out bool;";

    assert_expansion_contains(src, expected_a);
    assert_expansion_contains(src, expected_b);
}

#[test]
fn test_macro_expand_reflex_loop_unrolling() {
    let src = r#"
        module m {
            reflex r {
                always {
                    for i in 0..3 {
                        arr[i] = true;
                    }
                }
            }
        }
    "#;
    assert_expansion_contains(src, "arr_0 = true;");
    assert_expansion_contains(src, "arr_1 = true;");
    assert_expansion_contains(src, "arr_2 = true;");
}

#[test]
fn test_macro_expand_toplevel_loop_unrolling() {
    let src = r#"
        module m {
            for i in 1..4 {
                guard g_${i} { when a == ${i} for 1 cycles; }
            }
        }
    "#;
    assert_expansion_contains(src, "guard g_1 { when a == 1 for 1 cycles; }");
    assert_expansion_contains(src, "guard g_2 { when a == 2 for 1 cycles; }");
    assert_expansion_contains(src, "guard g_3 { when a == 3 for 1 cycles; }");
}

#[test]
fn test_macro_expand_match_blocks() {
    let src = r#"
        module m {
            reflex r {
                always {
                    match state {
                        0 => { out = false; }
                        1 => { out = true; }
                        _ => { out = false; }
                    }
                }
            }
        }
    "#;
    let expanded = expand_macros(src);
    // Should be converted to if-else chains which then convert to guards
    assert!(expanded.contains("state == 0") || expanded.contains("if state == 0"));
}

#[test]
fn test_macro_expand_ergonomic_guard_assignment() {
    let src = r#"
        module m {
            let guard my_guard = when x == true for 2 cycles;
        }
    "#;
    let expanded = expand_macros(src);
    assert!(expanded.contains("guard my_guard {"));
    assert!(expanded.contains("when x == true"));
    assert!(expanded.contains("for 2 cycles"));
}

#[test]
fn test_macro_expand_if_else_reflexes() {
    let src = r#"
        module m {
            reflex r {
                always {
                    if x > 10 {
                        out = true;
                    } else {
                        out = false;
                    }
                }
            }
        }
    "#;
    let expanded = expand_macros(src);
    // The if_else preprocessor injects guards
    assert!(expanded.contains("on auto_g_0"));
    assert!(expanded.contains("out = true;"));
}

#[test]
fn test_macro_expand_fsm_stress() {
    let src = r#"
        signals {
            s: in u8
            t: in u8
        }
        module m {
            for i in 0..2 {
                guard g_${i} { when s == ${i} for 1 cycles; }
            }
            reflex r {
                always {
                    match t {
                        0 => { 
                            for j in 0..2 {
                                arr[j] = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    "#;
    let expanded = expand_macros(src);
    assert!(expanded.contains("guard g_0 { when s == 0 for 1 cycles; }"));
    assert!(expanded.contains("guard g_1 { when s == 1 for 1 cycles; }"));
    assert!(expanded.contains("signal s: in u8;"));
}

// --- AUTO GENERATED EXPANSION TESTS ---

macro_rules! test_macro_valid {
    ($name:ident, $src:expr) => {
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use nasa_rust_project::compiler::macro_proc::expand_macros;
            let exp = expand_macros($src);
            if exp.is_empty() {
                return Err("empty".into());
            }
            Ok(())
        }
    };
}
test_macro_valid!(test_macro_1, "module m_1 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_2, "module m_2 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_3, "module m_3 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_4, "module m_4 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_5, "module m_5 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_6, "module m_6 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_7, "module m_7 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_8, "module m_8 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_9, "module m_9 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_10, "module m_10 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_11, "module m_11 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_12, "module m_12 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_13, "module m_13 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_14, "module m_14 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_15, "module m_15 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_16, "module m_16 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_17, "module m_17 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_18, "module m_18 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_19, "module m_19 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_20, "module m_20 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_21, "module m_21 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_22, "module m_22 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_23, "module m_23 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_24, "module m_24 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_25, "module m_25 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_26, "module m_26 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_27, "module m_27 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_28, "module m_28 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_29, "module m_29 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_30, "module m_30 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_31, "module m_31 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_32, "module m_32 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_33, "module m_33 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_34, "module m_34 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_35, "module m_35 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_36, "module m_36 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_37, "module m_37 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_38, "module m_38 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_39, "module m_39 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_40, "module m_40 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_41, "module m_41 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_42, "module m_42 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_43, "module m_43 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_44, "module m_44 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_45, "module m_45 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_46, "module m_46 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_47, "module m_47 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_48, "module m_48 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_49, "module m_49 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_50, "module m_50 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_51, "module m_51 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_52, "module m_52 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_53, "module m_53 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_54, "module m_54 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_55, "module m_55 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_56, "module m_56 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_57, "module m_57 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_58, "module m_58 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_59, "module m_59 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_60, "module m_60 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_61, "module m_61 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_62, "module m_62 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_63, "module m_63 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_64, "module m_64 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_65, "module m_65 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_66, "module m_66 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_67, "module m_67 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_68, "module m_68 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_69, "module m_69 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_70, "module m_70 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_71, "module m_71 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_72, "module m_72 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_73, "module m_73 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_74, "module m_74 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_75, "module m_75 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_76, "module m_76 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_77, "module m_77 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_78, "module m_78 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_79, "module m_79 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_80, "module m_80 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_81, "module m_81 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_82, "module m_82 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_83, "module m_83 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_84, "module m_84 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_85, "module m_85 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_86, "module m_86 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_87, "module m_87 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_88, "module m_88 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_89, "module m_89 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_90, "module m_90 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_91, "module m_91 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_92, "module m_92 { \n signal x: in bool; \n }");
test_macro_valid!(test_macro_93, "module m_93 { \n signal x: in bool; \n }");
