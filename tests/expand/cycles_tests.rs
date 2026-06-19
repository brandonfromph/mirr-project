#![cfg(any())]
#![forbid(unsafe_code)]
use mirrc::pipeline::{run_pipeline, PipelineConfig};
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
fn test_cycles_0() {
    let src = r#"def A_0() { reflect { A_0(); } } module m { A_0(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_1() {
    let src = r#"def A_1() { reflect { A_1(); } } module m { A_1(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_2() {
    let src = r#"def A_2() { reflect { A_2(); } } module m { A_2(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_3() {
    let src = r#"def A_3() { reflect { A_3(); } } module m { A_3(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_4() {
    let src = r#"def A_4() { reflect { A_4(); } } module m { A_4(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_5() {
    let src =
        r#"def A_5() { reflect { B_5(); } } def B_5() { reflect { A_5(); } } module m { A_5(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_6() {
    let src =
        r#"def A_6() { reflect { B_6(); } } def B_6() { reflect { A_6(); } } module m { A_6(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_7() {
    let src =
        r#"def A_7() { reflect { B_7(); } } def B_7() { reflect { A_7(); } } module m { A_7(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_8() {
    let src =
        r#"def A_8() { reflect { B_8(); } } def B_8() { reflect { A_8(); } } module m { A_8(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_9() {
    let src =
        r#"def A_9() { reflect { B_9(); } } def B_9() { reflect { A_9(); } } module m { A_9(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_10() {
    let src = r#"def A_10() { reflect { B_10(); } } def B_10() { reflect { C_10(); } } def C_10() { reflect { D_10(); } } def D_10() { reflect { A_10(); } } module m { A_10(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_11() {
    let src = r#"def A_11() { reflect { B_11(); } } def B_11() { reflect { C_11(); } } def C_11() { reflect { D_11(); } } def D_11() { reflect { A_11(); } } module m { A_11(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_12() {
    let src = r#"def A_12() { reflect { B_12(); } } def B_12() { reflect { C_12(); } } def C_12() { reflect { D_12(); } } def D_12() { reflect { A_12(); } } module m { A_12(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_13() {
    let src = r#"def A_13() { reflect { B_13(); } } def B_13() { reflect { C_13(); } } def C_13() { reflect { D_13(); } } def D_13() { reflect { A_13(); } } module m { A_13(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_14() {
    let src = r#"def A_14() { reflect { B_14(); } } def B_14() { reflect { C_14(); } } def C_14() { reflect { D_14(); } } def D_14() { reflect { A_14(); } } module m { A_14(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_15() {
    let src = r#"def A_15(s: signal in bool) { reflect { A_15(s); } } module m { signal sig: in bool; A_15(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_16() {
    let src = r#"def A_16(s: signal in bool) { reflect { A_16(s); } } module m { signal sig: in bool; A_16(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_17() {
    let src = r#"def A_17(s: signal in bool) { reflect { A_17(s); } } module m { signal sig: in bool; A_17(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_18() {
    let src = r#"def A_18(s: signal in bool) { reflect { A_18(s); } } module m { signal sig: in bool; A_18(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_19() {
    let src = r#"def A_19(s: signal in bool) { reflect { A_19(s); } } module m { signal sig: in bool; A_19(sig); }"#;
    let res = run_expand_only(src);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Circular"));
}
#[test]
fn test_cycles_20() {
    let src = r#"def A_20() { reflect { B_20(); C_20(); } } def B_20() { reflect { D_20(); } } def C_20() { reflect { D_20(); } } def D_20() { reflect { signal s: internal bool; } } module m { A_20(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "Diamond dependency should not be a cycle: {:?}", res.err());
}
#[test]
fn test_cycles_21() {
    let src = r#"def A_21() { reflect { B_21(); C_21(); } } def B_21() { reflect { D_21(); } } def C_21() { reflect { D_21(); } } def D_21() { reflect { signal s: internal bool; } } module m { A_21(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "Diamond dependency should not be a cycle: {:?}", res.err());
}
#[test]
fn test_cycles_22() {
    let src = r#"def A_22() { reflect { B_22(); C_22(); } } def B_22() { reflect { D_22(); } } def C_22() { reflect { D_22(); } } def D_22() { reflect { signal s: internal bool; } } module m { A_22(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "Diamond dependency should not be a cycle: {:?}", res.err());
}
#[test]
fn test_cycles_23() {
    let src = r#"def A_23() { reflect { B_23(); C_23(); } } def B_23() { reflect { D_23(); } } def C_23() { reflect { D_23(); } } def D_23() { reflect { signal s: internal bool; } } module m { A_23(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "Diamond dependency should not be a cycle: {:?}", res.err());
}
#[test]
fn test_cycles_24() {
    let src = r#"def A_24() { reflect { B_24(); C_24(); } } def B_24() { reflect { D_24(); } } def C_24() { reflect { D_24(); } } def D_24() { reflect { signal s: internal bool; } } module m { A_24(); }"#;
    let res = run_expand_only(src);
    assert!(res.is_ok(), "Diamond dependency should not be a cycle: {:?}", res.err());
}
