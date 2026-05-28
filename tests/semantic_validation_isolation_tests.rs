#![forbid(unsafe_code)]

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

fn err_str(src: &str) -> String {
    let res = run_pipeline(src, &PipelineConfig::default());
    assert!(res.is_err(), "Expected compilation to fail for source:\n{}", src);
    res.unwrap_err().to_string()
}

fn ok_str(src: &str) {
    let res = run_pipeline(src, &PipelineConfig::default());
    assert!(res.is_ok(), "Expected compilation to succeed but failed: {:?}", res.unwrap_err());
}

#[test]
fn test_semantic_duplicate_signal_e201() {
    let src = r#"
        module m {
            signal a: in bool;
            signal a: out bool;
        }
    "#;
    assert!(err_str(src).contains("E201"));
}

#[test]
fn test_semantic_duplicate_guard_e213() {
    let src = r#"
        module m {
            signal a: in bool;
            guard g { when a for 1 cycles; }
            guard g { when a for 2 cycles; }
        }
    "#;
    assert!(err_str(src).contains("E213"));
}

#[test]
fn test_semantic_duplicate_reflex_e212() {
    let src = r#"
        module m {
            signal a: in bool;
            signal out: out bool;
            guard g { when a for 1 cycles; }
            reflex r { on g { out = true; } }
            reflex r { on g { out = false; } }
        }
    "#;
    assert!(err_str(src).contains("E212"));
}

#[test]
fn test_semantic_missing_guard_for_reflex_e205() {
    let src = r#"
        module m {
            signal out: out bool;
            reflex r { on g_missing { out = true; } }
        }
    "#;
    assert!(err_str(src).contains("E205")); // or E204 depending on how undefined identifiers are handled, usually undefined signal/guard is E204
}

#[test]
fn test_semantic_undefined_signal_in_guard_e204() {
    let src = r#"
        module m {
            guard g { when nonexistent for 1 cycles; }
        }
    "#;
    assert!(err_str(src).contains("E204"));
}

#[test]
fn test_semantic_assign_to_input_signal_e206() {
    let src = r#"
        module m {
            signal a: in bool;
            guard g { when a for 1 cycles; }
            reflex r { on g { a = true; } }
        }
    "#;
    assert!(err_str(src).contains("E206"));
}

#[test]
fn test_semantic_prev_delay_zero_e209() {
    let src = r#"
        module m {
            signal a: in bool;
            guard g { when prev(a, 0) for 1 cycles; }
        }
    "#;
    assert!(err_str(src).contains("E209"));
}

#[test]
fn test_semantic_cross_type_collision() {
    let src = r#"
        module m {
            signal my_name: in bool;
            guard my_name { when my_name for 1 cycles; }
        }
    "#;
    // Should trigger a duplicate name error, either E201 or E213, or some shadowing error.
    let err = err_str(src);
    assert!(err.contains("E201") || err.contains("E213") || err.contains("E200"));
}

#[test]
fn test_semantic_valid_complex_module_ok() {
    let src = r#"
        module m {
            signal sys_clk: in bool;
            signal sys_rst: in bool;
            signal data_in: in u16;
            signal data_out: out u16;
            
            guard g_valid { when sys_rst == false for 1 cycles; }
            
            reflex r_process {
                on g_valid {
                    data_out = data_in + 1;
                }
            }
            
            property p_safe { always(data_out > 0); }
        }
    "#;
    ok_str(src);
}

#[test]
fn test_semantic_multiple_errors_collected() {
    let src = r#"
        module m {
            signal a: in bool;
            signal a: out bool;
            guard g { when a for 1 cycles; }
            guard g { when a for 2 cycles; }
        }
    "#;
    let err = err_str(src);
    assert!(err.contains("E201"));
    assert!(err.contains("E213"));
}

// --- AUTO GENERATED EXPANSION TESTS ---

macro_rules! test_semantic_valid {
    ($name:ident, $src:expr) => {
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use nasa_rust_project::ecs::Registry;
            use nasa_rust_project::parse_mirr;
            let prog = parse_mirr($src)?;
            let mut reg = Registry::new();
            reg.ingest_module(&prog.module)?;
            reg.semantic_validate()?;
            Ok(())
        }
    };
}
test_semantic_valid!(test_semantic_iso_1, "module m_1 { signal x_1: in bool; signal y_1: out bool; guard g_1 { when x_1 for 1 cycles; } reflex r_1 { on g_1 { y_1 = true; } } }");
test_semantic_valid!(test_semantic_iso_2, "module m_2 { signal x_2: in bool; signal y_2: out bool; guard g_2 { when x_2 for 1 cycles; } reflex r_2 { on g_2 { y_2 = true; } } }");
test_semantic_valid!(test_semantic_iso_3, "module m_3 { signal x_3: in bool; signal y_3: out bool; guard g_3 { when x_3 for 1 cycles; } reflex r_3 { on g_3 { y_3 = true; } } }");
test_semantic_valid!(test_semantic_iso_4, "module m_4 { signal x_4: in bool; signal y_4: out bool; guard g_4 { when x_4 for 1 cycles; } reflex r_4 { on g_4 { y_4 = true; } } }");
test_semantic_valid!(test_semantic_iso_5, "module m_5 { signal x_5: in bool; signal y_5: out bool; guard g_5 { when x_5 for 1 cycles; } reflex r_5 { on g_5 { y_5 = true; } } }");
test_semantic_valid!(test_semantic_iso_6, "module m_6 { signal x_6: in bool; signal y_6: out bool; guard g_6 { when x_6 for 1 cycles; } reflex r_6 { on g_6 { y_6 = true; } } }");
test_semantic_valid!(test_semantic_iso_7, "module m_7 { signal x_7: in bool; signal y_7: out bool; guard g_7 { when x_7 for 1 cycles; } reflex r_7 { on g_7 { y_7 = true; } } }");
test_semantic_valid!(test_semantic_iso_8, "module m_8 { signal x_8: in bool; signal y_8: out bool; guard g_8 { when x_8 for 1 cycles; } reflex r_8 { on g_8 { y_8 = true; } } }");
test_semantic_valid!(test_semantic_iso_9, "module m_9 { signal x_9: in bool; signal y_9: out bool; guard g_9 { when x_9 for 1 cycles; } reflex r_9 { on g_9 { y_9 = true; } } }");
test_semantic_valid!(test_semantic_iso_10, "module m_10 { signal x_10: in bool; signal y_10: out bool; guard g_10 { when x_10 for 1 cycles; } reflex r_10 { on g_10 { y_10 = true; } } }");
test_semantic_valid!(test_semantic_iso_11, "module m_11 { signal x_11: in bool; signal y_11: out bool; guard g_11 { when x_11 for 1 cycles; } reflex r_11 { on g_11 { y_11 = true; } } }");
test_semantic_valid!(test_semantic_iso_12, "module m_12 { signal x_12: in bool; signal y_12: out bool; guard g_12 { when x_12 for 1 cycles; } reflex r_12 { on g_12 { y_12 = true; } } }");
test_semantic_valid!(test_semantic_iso_13, "module m_13 { signal x_13: in bool; signal y_13: out bool; guard g_13 { when x_13 for 1 cycles; } reflex r_13 { on g_13 { y_13 = true; } } }");
test_semantic_valid!(test_semantic_iso_14, "module m_14 { signal x_14: in bool; signal y_14: out bool; guard g_14 { when x_14 for 1 cycles; } reflex r_14 { on g_14 { y_14 = true; } } }");
test_semantic_valid!(test_semantic_iso_15, "module m_15 { signal x_15: in bool; signal y_15: out bool; guard g_15 { when x_15 for 1 cycles; } reflex r_15 { on g_15 { y_15 = true; } } }");
test_semantic_valid!(test_semantic_iso_16, "module m_16 { signal x_16: in bool; signal y_16: out bool; guard g_16 { when x_16 for 1 cycles; } reflex r_16 { on g_16 { y_16 = true; } } }");
test_semantic_valid!(test_semantic_iso_17, "module m_17 { signal x_17: in bool; signal y_17: out bool; guard g_17 { when x_17 for 1 cycles; } reflex r_17 { on g_17 { y_17 = true; } } }");
test_semantic_valid!(test_semantic_iso_18, "module m_18 { signal x_18: in bool; signal y_18: out bool; guard g_18 { when x_18 for 1 cycles; } reflex r_18 { on g_18 { y_18 = true; } } }");
test_semantic_valid!(test_semantic_iso_19, "module m_19 { signal x_19: in bool; signal y_19: out bool; guard g_19 { when x_19 for 1 cycles; } reflex r_19 { on g_19 { y_19 = true; } } }");
test_semantic_valid!(test_semantic_iso_20, "module m_20 { signal x_20: in bool; signal y_20: out bool; guard g_20 { when x_20 for 1 cycles; } reflex r_20 { on g_20 { y_20 = true; } } }");
test_semantic_valid!(test_semantic_iso_21, "module m_21 { signal x_21: in bool; signal y_21: out bool; guard g_21 { when x_21 for 1 cycles; } reflex r_21 { on g_21 { y_21 = true; } } }");
test_semantic_valid!(test_semantic_iso_22, "module m_22 { signal x_22: in bool; signal y_22: out bool; guard g_22 { when x_22 for 1 cycles; } reflex r_22 { on g_22 { y_22 = true; } } }");
test_semantic_valid!(test_semantic_iso_23, "module m_23 { signal x_23: in bool; signal y_23: out bool; guard g_23 { when x_23 for 1 cycles; } reflex r_23 { on g_23 { y_23 = true; } } }");
test_semantic_valid!(test_semantic_iso_24, "module m_24 { signal x_24: in bool; signal y_24: out bool; guard g_24 { when x_24 for 1 cycles; } reflex r_24 { on g_24 { y_24 = true; } } }");
test_semantic_valid!(test_semantic_iso_25, "module m_25 { signal x_25: in bool; signal y_25: out bool; guard g_25 { when x_25 for 1 cycles; } reflex r_25 { on g_25 { y_25 = true; } } }");
test_semantic_valid!(test_semantic_iso_26, "module m_26 { signal x_26: in bool; signal y_26: out bool; guard g_26 { when x_26 for 1 cycles; } reflex r_26 { on g_26 { y_26 = true; } } }");
test_semantic_valid!(test_semantic_iso_27, "module m_27 { signal x_27: in bool; signal y_27: out bool; guard g_27 { when x_27 for 1 cycles; } reflex r_27 { on g_27 { y_27 = true; } } }");
test_semantic_valid!(test_semantic_iso_28, "module m_28 { signal x_28: in bool; signal y_28: out bool; guard g_28 { when x_28 for 1 cycles; } reflex r_28 { on g_28 { y_28 = true; } } }");
test_semantic_valid!(test_semantic_iso_29, "module m_29 { signal x_29: in bool; signal y_29: out bool; guard g_29 { when x_29 for 1 cycles; } reflex r_29 { on g_29 { y_29 = true; } } }");
test_semantic_valid!(test_semantic_iso_30, "module m_30 { signal x_30: in bool; signal y_30: out bool; guard g_30 { when x_30 for 1 cycles; } reflex r_30 { on g_30 { y_30 = true; } } }");
test_semantic_valid!(test_semantic_iso_31, "module m_31 { signal x_31: in bool; signal y_31: out bool; guard g_31 { when x_31 for 1 cycles; } reflex r_31 { on g_31 { y_31 = true; } } }");
test_semantic_valid!(test_semantic_iso_32, "module m_32 { signal x_32: in bool; signal y_32: out bool; guard g_32 { when x_32 for 1 cycles; } reflex r_32 { on g_32 { y_32 = true; } } }");
test_semantic_valid!(test_semantic_iso_33, "module m_33 { signal x_33: in bool; signal y_33: out bool; guard g_33 { when x_33 for 1 cycles; } reflex r_33 { on g_33 { y_33 = true; } } }");
test_semantic_valid!(test_semantic_iso_34, "module m_34 { signal x_34: in bool; signal y_34: out bool; guard g_34 { when x_34 for 1 cycles; } reflex r_34 { on g_34 { y_34 = true; } } }");
test_semantic_valid!(test_semantic_iso_35, "module m_35 { signal x_35: in bool; signal y_35: out bool; guard g_35 { when x_35 for 1 cycles; } reflex r_35 { on g_35 { y_35 = true; } } }");
test_semantic_valid!(test_semantic_iso_36, "module m_36 { signal x_36: in bool; signal y_36: out bool; guard g_36 { when x_36 for 1 cycles; } reflex r_36 { on g_36 { y_36 = true; } } }");
test_semantic_valid!(test_semantic_iso_37, "module m_37 { signal x_37: in bool; signal y_37: out bool; guard g_37 { when x_37 for 1 cycles; } reflex r_37 { on g_37 { y_37 = true; } } }");
test_semantic_valid!(test_semantic_iso_38, "module m_38 { signal x_38: in bool; signal y_38: out bool; guard g_38 { when x_38 for 1 cycles; } reflex r_38 { on g_38 { y_38 = true; } } }");
test_semantic_valid!(test_semantic_iso_39, "module m_39 { signal x_39: in bool; signal y_39: out bool; guard g_39 { when x_39 for 1 cycles; } reflex r_39 { on g_39 { y_39 = true; } } }");
test_semantic_valid!(test_semantic_iso_40, "module m_40 { signal x_40: in bool; signal y_40: out bool; guard g_40 { when x_40 for 1 cycles; } reflex r_40 { on g_40 { y_40 = true; } } }");
test_semantic_valid!(test_semantic_iso_41, "module m_41 { signal x_41: in bool; signal y_41: out bool; guard g_41 { when x_41 for 1 cycles; } reflex r_41 { on g_41 { y_41 = true; } } }");
test_semantic_valid!(test_semantic_iso_42, "module m_42 { signal x_42: in bool; signal y_42: out bool; guard g_42 { when x_42 for 1 cycles; } reflex r_42 { on g_42 { y_42 = true; } } }");
test_semantic_valid!(test_semantic_iso_43, "module m_43 { signal x_43: in bool; signal y_43: out bool; guard g_43 { when x_43 for 1 cycles; } reflex r_43 { on g_43 { y_43 = true; } } }");
test_semantic_valid!(test_semantic_iso_44, "module m_44 { signal x_44: in bool; signal y_44: out bool; guard g_44 { when x_44 for 1 cycles; } reflex r_44 { on g_44 { y_44 = true; } } }");
test_semantic_valid!(test_semantic_iso_45, "module m_45 { signal x_45: in bool; signal y_45: out bool; guard g_45 { when x_45 for 1 cycles; } reflex r_45 { on g_45 { y_45 = true; } } }");
test_semantic_valid!(test_semantic_iso_46, "module m_46 { signal x_46: in bool; signal y_46: out bool; guard g_46 { when x_46 for 1 cycles; } reflex r_46 { on g_46 { y_46 = true; } } }");
test_semantic_valid!(test_semantic_iso_47, "module m_47 { signal x_47: in bool; signal y_47: out bool; guard g_47 { when x_47 for 1 cycles; } reflex r_47 { on g_47 { y_47 = true; } } }");
test_semantic_valid!(test_semantic_iso_48, "module m_48 { signal x_48: in bool; signal y_48: out bool; guard g_48 { when x_48 for 1 cycles; } reflex r_48 { on g_48 { y_48 = true; } } }");
test_semantic_valid!(test_semantic_iso_49, "module m_49 { signal x_49: in bool; signal y_49: out bool; guard g_49 { when x_49 for 1 cycles; } reflex r_49 { on g_49 { y_49 = true; } } }");
test_semantic_valid!(test_semantic_iso_50, "module m_50 { signal x_50: in bool; signal y_50: out bool; guard g_50 { when x_50 for 1 cycles; } reflex r_50 { on g_50 { y_50 = true; } } }");
test_semantic_valid!(test_semantic_iso_51, "module m_51 { signal x_51: in bool; signal y_51: out bool; guard g_51 { when x_51 for 1 cycles; } reflex r_51 { on g_51 { y_51 = true; } } }");
test_semantic_valid!(test_semantic_iso_52, "module m_52 { signal x_52: in bool; signal y_52: out bool; guard g_52 { when x_52 for 1 cycles; } reflex r_52 { on g_52 { y_52 = true; } } }");
test_semantic_valid!(test_semantic_iso_53, "module m_53 { signal x_53: in bool; signal y_53: out bool; guard g_53 { when x_53 for 1 cycles; } reflex r_53 { on g_53 { y_53 = true; } } }");
test_semantic_valid!(test_semantic_iso_54, "module m_54 { signal x_54: in bool; signal y_54: out bool; guard g_54 { when x_54 for 1 cycles; } reflex r_54 { on g_54 { y_54 = true; } } }");
test_semantic_valid!(test_semantic_iso_55, "module m_55 { signal x_55: in bool; signal y_55: out bool; guard g_55 { when x_55 for 1 cycles; } reflex r_55 { on g_55 { y_55 = true; } } }");
test_semantic_valid!(test_semantic_iso_56, "module m_56 { signal x_56: in bool; signal y_56: out bool; guard g_56 { when x_56 for 1 cycles; } reflex r_56 { on g_56 { y_56 = true; } } }");
test_semantic_valid!(test_semantic_iso_57, "module m_57 { signal x_57: in bool; signal y_57: out bool; guard g_57 { when x_57 for 1 cycles; } reflex r_57 { on g_57 { y_57 = true; } } }");
test_semantic_valid!(test_semantic_iso_58, "module m_58 { signal x_58: in bool; signal y_58: out bool; guard g_58 { when x_58 for 1 cycles; } reflex r_58 { on g_58 { y_58 = true; } } }");
test_semantic_valid!(test_semantic_iso_59, "module m_59 { signal x_59: in bool; signal y_59: out bool; guard g_59 { when x_59 for 1 cycles; } reflex r_59 { on g_59 { y_59 = true; } } }");
test_semantic_valid!(test_semantic_iso_60, "module m_60 { signal x_60: in bool; signal y_60: out bool; guard g_60 { when x_60 for 1 cycles; } reflex r_60 { on g_60 { y_60 = true; } } }");
test_semantic_valid!(test_semantic_iso_61, "module m_61 { signal x_61: in bool; signal y_61: out bool; guard g_61 { when x_61 for 1 cycles; } reflex r_61 { on g_61 { y_61 = true; } } }");
test_semantic_valid!(test_semantic_iso_62, "module m_62 { signal x_62: in bool; signal y_62: out bool; guard g_62 { when x_62 for 1 cycles; } reflex r_62 { on g_62 { y_62 = true; } } }");
test_semantic_valid!(test_semantic_iso_63, "module m_63 { signal x_63: in bool; signal y_63: out bool; guard g_63 { when x_63 for 1 cycles; } reflex r_63 { on g_63 { y_63 = true; } } }");
test_semantic_valid!(test_semantic_iso_64, "module m_64 { signal x_64: in bool; signal y_64: out bool; guard g_64 { when x_64 for 1 cycles; } reflex r_64 { on g_64 { y_64 = true; } } }");
test_semantic_valid!(test_semantic_iso_65, "module m_65 { signal x_65: in bool; signal y_65: out bool; guard g_65 { when x_65 for 1 cycles; } reflex r_65 { on g_65 { y_65 = true; } } }");
test_semantic_valid!(test_semantic_iso_66, "module m_66 { signal x_66: in bool; signal y_66: out bool; guard g_66 { when x_66 for 1 cycles; } reflex r_66 { on g_66 { y_66 = true; } } }");
test_semantic_valid!(test_semantic_iso_67, "module m_67 { signal x_67: in bool; signal y_67: out bool; guard g_67 { when x_67 for 1 cycles; } reflex r_67 { on g_67 { y_67 = true; } } }");
test_semantic_valid!(test_semantic_iso_68, "module m_68 { signal x_68: in bool; signal y_68: out bool; guard g_68 { when x_68 for 1 cycles; } reflex r_68 { on g_68 { y_68 = true; } } }");
test_semantic_valid!(test_semantic_iso_69, "module m_69 { signal x_69: in bool; signal y_69: out bool; guard g_69 { when x_69 for 1 cycles; } reflex r_69 { on g_69 { y_69 = true; } } }");
test_semantic_valid!(test_semantic_iso_70, "module m_70 { signal x_70: in bool; signal y_70: out bool; guard g_70 { when x_70 for 1 cycles; } reflex r_70 { on g_70 { y_70 = true; } } }");
test_semantic_valid!(test_semantic_iso_71, "module m_71 { signal x_71: in bool; signal y_71: out bool; guard g_71 { when x_71 for 1 cycles; } reflex r_71 { on g_71 { y_71 = true; } } }");
test_semantic_valid!(test_semantic_iso_72, "module m_72 { signal x_72: in bool; signal y_72: out bool; guard g_72 { when x_72 for 1 cycles; } reflex r_72 { on g_72 { y_72 = true; } } }");
test_semantic_valid!(test_semantic_iso_73, "module m_73 { signal x_73: in bool; signal y_73: out bool; guard g_73 { when x_73 for 1 cycles; } reflex r_73 { on g_73 { y_73 = true; } } }");
test_semantic_valid!(test_semantic_iso_74, "module m_74 { signal x_74: in bool; signal y_74: out bool; guard g_74 { when x_74 for 1 cycles; } reflex r_74 { on g_74 { y_74 = true; } } }");
test_semantic_valid!(test_semantic_iso_75, "module m_75 { signal x_75: in bool; signal y_75: out bool; guard g_75 { when x_75 for 1 cycles; } reflex r_75 { on g_75 { y_75 = true; } } }");
test_semantic_valid!(test_semantic_iso_76, "module m_76 { signal x_76: in bool; signal y_76: out bool; guard g_76 { when x_76 for 1 cycles; } reflex r_76 { on g_76 { y_76 = true; } } }");
test_semantic_valid!(test_semantic_iso_77, "module m_77 { signal x_77: in bool; signal y_77: out bool; guard g_77 { when x_77 for 1 cycles; } reflex r_77 { on g_77 { y_77 = true; } } }");
test_semantic_valid!(test_semantic_iso_78, "module m_78 { signal x_78: in bool; signal y_78: out bool; guard g_78 { when x_78 for 1 cycles; } reflex r_78 { on g_78 { y_78 = true; } } }");
test_semantic_valid!(test_semantic_iso_79, "module m_79 { signal x_79: in bool; signal y_79: out bool; guard g_79 { when x_79 for 1 cycles; } reflex r_79 { on g_79 { y_79 = true; } } }");
test_semantic_valid!(test_semantic_iso_80, "module m_80 { signal x_80: in bool; signal y_80: out bool; guard g_80 { when x_80 for 1 cycles; } reflex r_80 { on g_80 { y_80 = true; } } }");
test_semantic_valid!(test_semantic_iso_81, "module m_81 { signal x_81: in bool; signal y_81: out bool; guard g_81 { when x_81 for 1 cycles; } reflex r_81 { on g_81 { y_81 = true; } } }");
test_semantic_valid!(test_semantic_iso_82, "module m_82 { signal x_82: in bool; signal y_82: out bool; guard g_82 { when x_82 for 1 cycles; } reflex r_82 { on g_82 { y_82 = true; } } }");
test_semantic_valid!(test_semantic_iso_83, "module m_83 { signal x_83: in bool; signal y_83: out bool; guard g_83 { when x_83 for 1 cycles; } reflex r_83 { on g_83 { y_83 = true; } } }");
test_semantic_valid!(test_semantic_iso_84, "module m_84 { signal x_84: in bool; signal y_84: out bool; guard g_84 { when x_84 for 1 cycles; } reflex r_84 { on g_84 { y_84 = true; } } }");
test_semantic_valid!(test_semantic_iso_85, "module m_85 { signal x_85: in bool; signal y_85: out bool; guard g_85 { when x_85 for 1 cycles; } reflex r_85 { on g_85 { y_85 = true; } } }");
test_semantic_valid!(test_semantic_iso_86, "module m_86 { signal x_86: in bool; signal y_86: out bool; guard g_86 { when x_86 for 1 cycles; } reflex r_86 { on g_86 { y_86 = true; } } }");
test_semantic_valid!(test_semantic_iso_87, "module m_87 { signal x_87: in bool; signal y_87: out bool; guard g_87 { when x_87 for 1 cycles; } reflex r_87 { on g_87 { y_87 = true; } } }");
test_semantic_valid!(test_semantic_iso_88, "module m_88 { signal x_88: in bool; signal y_88: out bool; guard g_88 { when x_88 for 1 cycles; } reflex r_88 { on g_88 { y_88 = true; } } }");
test_semantic_valid!(test_semantic_iso_89, "module m_89 { signal x_89: in bool; signal y_89: out bool; guard g_89 { when x_89 for 1 cycles; } reflex r_89 { on g_89 { y_89 = true; } } }");
test_semantic_valid!(test_semantic_iso_90, "module m_90 { signal x_90: in bool; signal y_90: out bool; guard g_90 { when x_90 for 1 cycles; } reflex r_90 { on g_90 { y_90 = true; } } }");
