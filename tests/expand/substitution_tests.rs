#![forbid(unsafe_code)]
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
fn run_expand_only(src: &str) -> Result<(), String> {
    let config = PipelineConfig {
        typecheck: false,
        bootstrap_mode: false,
        simplify: false,
        sat_simplify: false,
        width: false,
        temporal: false,
        ..PipelineConfig::default()
    };
    run_pipeline(src, &config).map_err(|e| e.to_string()).map(|_| ())
}
#[test]
fn test_subst_0() {
    let src = r#"def A_0(s: signal in bool) { reflect { signal my_sig_0: internal bool; guard g { when ${s} for 1 cycles; } reflex r { on g { my_sig_0 = true; } } } } module m { signal sig: in bool; A_0(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_1() {
    let src = r#"def A_1(s: signal in bool) { reflect { signal my_sig_1: internal bool; guard g { when ${s} for 1 cycles; } reflex r { on g { my_sig_1 = true; } } } } module m { signal sig: in bool; A_1(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_2() {
    let src = r#"def A_2(s: signal in bool) { reflect { signal my_sig_2: internal bool; guard g { when ${s} for 1 cycles; } reflex r { on g { my_sig_2 = true; } } } } module m { signal sig: in bool; A_2(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_3() {
    let src = r#"def A_3(s: signal in bool) { reflect { signal my_sig_3: internal bool; guard g { when ${s} for 1 cycles; } reflex r { on g { my_sig_3 = true; } } } } module m { signal sig: in bool; A_3(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_4() {
    let src = r#"def A_4(s: signal in bool) { reflect { signal my_sig_4: internal bool; guard g { when ${s} for 1 cycles; } reflex r { on g { my_sig_4 = true; } } } } module m { signal sig: in bool; A_4(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_5() {
    let src = r#"def A_5(val: signal in u32) { reflect { signal my_sig_5: internal u32; guard g { when my_sig_5 == ${val} for 1 cycles; } } } module m { signal the_val: in u32; A_5(the_val); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_6() {
    let src = r#"def A_6(val: signal in u32) { reflect { signal my_sig_6: internal u32; guard g { when my_sig_6 == ${val} for 1 cycles; } } } module m { signal the_val: in u32; A_6(the_val); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_7() {
    let src = r#"def A_7(val: signal in u32) { reflect { signal my_sig_7: internal u32; guard g { when my_sig_7 == ${val} for 1 cycles; } } } module m { signal the_val: in u32; A_7(the_val); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_8() {
    let src = r#"def A_8(val: signal in u32) { reflect { signal my_sig_8: internal u32; guard g { when my_sig_8 == ${val} for 1 cycles; } } } module m { signal the_val: in u32; A_8(the_val); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_9() {
    let src = r#"def A_9(val: signal in u32) { reflect { signal my_sig_9: internal u32; guard g { when my_sig_9 == ${val} for 1 cycles; } } } module m { signal the_val: in u32; A_9(the_val); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_10() {
    let src = r#"def A_10(b: signal in bool) { reflect { signal my_sig_10: internal bool; guard g { when ${b} for 1 cycles; } } } module m { signal my_bool: in bool; A_10(my_bool); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_11() {
    let src = r#"def A_11(b: signal in bool) { reflect { signal my_sig_11: internal bool; guard g { when ${b} for 1 cycles; } } } module m { signal my_bool: in bool; A_11(my_bool); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_12() {
    let src = r#"def A_12(b: signal in bool) { reflect { signal my_sig_12: internal bool; guard g { when ${b} for 1 cycles; } } } module m { signal my_bool: in bool; A_12(my_bool); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_13() {
    let src = r#"def A_13(b: signal in bool) { reflect { signal my_sig_13: internal bool; guard g { when ${b} for 1 cycles; } } } module m { signal my_bool: in bool; A_13(my_bool); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_14() {
    let src = r#"def A_14(b: signal in bool) { reflect { signal my_sig_14: internal bool; guard g { when ${b} for 1 cycles; } } } module m { signal my_bool: in bool; A_14(my_bool); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_15() {
    let src = r#"def A_15(x: signal in bool) { reflect { } } module m { A_15(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
}
#[test]
fn test_subst_16() {
    let src = r#"def A_16(x: signal in bool) { reflect { } } module m { A_16(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
}
#[test]
fn test_subst_17() {
    let src = r#"def A_17(x: signal in bool) { reflect { } } module m { A_17(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
}
#[test]
fn test_subst_18() {
    let src = r#"def A_18(x: signal in bool) { reflect { } } module m { A_18(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
}
#[test]
fn test_subst_19() {
    let src = r#"def A_19(x: signal in bool) { reflect { } } module m { A_19(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
}
#[test]
fn test_subst_20() {
    let src = r#"def A_20(x: signal in bool, y: signal in bool) { reflect { signal z: internal bool; guard g { when ${x} && ${y} for 1 cycles; } } } module m { signal a: in bool; signal b: in bool; A_20(a, b); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_21() {
    let src = r#"def A_21(x: signal in bool, y: signal in bool) { reflect { signal z: internal bool; guard g { when ${x} && ${y} for 1 cycles; } } } module m { signal a: in bool; signal b: in bool; A_21(a, b); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_22() {
    let src = r#"def A_22(x: signal in bool, y: signal in bool) { reflect { signal z: internal bool; guard g { when ${x} && ${y} for 1 cycles; } } } module m { signal a: in bool; signal b: in bool; A_22(a, b); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_23() {
    let src = r#"def A_23(x: signal in bool, y: signal in bool) { reflect { signal z: internal bool; guard g { when ${x} && ${y} for 1 cycles; } } } module m { signal a: in bool; signal b: in bool; A_23(a, b); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_subst_24() {
    let src = r#"def A_24(x: signal in bool, y: signal in bool) { reflect { signal z: internal bool; guard g { when ${x} && ${y} for 1 cycles; } } } module m { signal a: in bool; signal b: in bool; A_24(a, b); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
