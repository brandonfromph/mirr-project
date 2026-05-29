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
fn test_renaming_0() {
    let src = r#"def A_0() { reflect { signal s: internal bool; guard my_guard_0 { when s for 1 cycles; } } } module m { A_0(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_1() {
    let src = r#"def A_1() { reflect { signal s: internal bool; guard my_guard_1 { when s for 1 cycles; } } } module m { A_1(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_2() {
    let src = r#"def A_2() { reflect { signal s: internal bool; guard my_guard_2 { when s for 1 cycles; } } } module m { A_2(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_3() {
    let src = r#"def A_3() { reflect { signal s: internal bool; guard my_guard_3 { when s for 1 cycles; } } } module m { A_3(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_4() {
    let src = r#"def A_4() { reflect { signal s: internal bool; guard my_guard_4 { when s for 1 cycles; } } } module m { A_4(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_5() {
    let src = r#"def A_5() { reflect { signal s: internal bool; guard my_guard_5 { when s for 1 cycles; } } } module m { A_5(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_6() {
    let src = r#"def A_6() { reflect { signal s: internal bool; guard my_guard_6 { when s for 1 cycles; } } } module m { A_6(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_7() {
    let src = r#"def A_7() { reflect { signal s: internal bool; guard my_guard_7 { when s for 1 cycles; } } } module m { A_7(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_8() {
    let src = r#"def A_8() { reflect { signal s: internal bool; guard my_guard_8 { when s for 1 cycles; } } } module m { A_8(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_9() {
    let src = r#"def A_9() { reflect { signal s: internal bool; guard my_guard_9 { when s for 1 cycles; } } } module m { A_9(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_10() {
    let src = r#"def A_10() { reflect { signal s: internal bool; guard my_guard_10 { when s for 1 cycles; } } } module m { A_10(); A_10(); A_10(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_11() {
    let src = r#"def A_11() { reflect { signal s: internal bool; guard my_guard_11 { when s for 1 cycles; } } } module m { A_11(); A_11(); A_11(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_12() {
    let src = r#"def A_12() { reflect { signal s: internal bool; guard my_guard_12 { when s for 1 cycles; } } } module m { A_12(); A_12(); A_12(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_13() {
    let src = r#"def A_13() { reflect { signal s: internal bool; guard my_guard_13 { when s for 1 cycles; } } } module m { A_13(); A_13(); A_13(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_14() {
    let src = r#"def A_14() { reflect { signal s: internal bool; guard my_guard_14 { when s for 1 cycles; } } } module m { A_14(); A_14(); A_14(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_15() {
    let src = r#"def A_15() { reflect { signal s: internal bool; guard my_guard_15 { when s for 1 cycles; } } } module m { A_15(); A_15(); A_15(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_16() {
    let src = r#"def A_16() { reflect { signal s: internal bool; guard my_guard_16 { when s for 1 cycles; } } } module m { A_16(); A_16(); A_16(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_17() {
    let src = r#"def A_17() { reflect { signal s: internal bool; guard my_guard_17 { when s for 1 cycles; } } } module m { A_17(); A_17(); A_17(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_18() {
    let src = r#"def A_18() { reflect { signal s: internal bool; guard my_guard_18 { when s for 1 cycles; } } } module m { A_18(); A_18(); A_18(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_19() {
    let src = r#"def A_19() { reflect { signal s: internal bool; guard my_guard_19 { when s for 1 cycles; } } } module m { A_19(); A_19(); A_19(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_20() {
    let src = r#"def A_20() { reflect { signal s: internal bool; guard my_guard_20 { when s for 1 cycles; } } } module m { A_20(); A_20(); A_20(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_21() {
    let src = r#"def A_21() { reflect { signal s: internal bool; guard my_guard_21 { when s for 1 cycles; } } } module m { A_21(); A_21(); A_21(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_22() {
    let src = r#"def A_22() { reflect { signal s: internal bool; guard my_guard_22 { when s for 1 cycles; } } } module m { A_22(); A_22(); A_22(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_23() {
    let src = r#"def A_23() { reflect { signal s: internal bool; guard my_guard_23 { when s for 1 cycles; } } } module m { A_23(); A_23(); A_23(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
#[test]
fn test_renaming_24() {
    let src = r#"def A_24() { reflect { signal s: internal bool; guard my_guard_24 { when s for 1 cycles; } } } module m { A_24(); A_24(); A_24(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "{:?}", res.err());
}
