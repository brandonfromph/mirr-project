#![forbid(unsafe_code)]
//! MEGA-2 S-expr tests D1–D5: types, parsing, printing, depth limits, roundtrip.
//!
//! NASA P10: bounded loops, no recursion.

use mirrc::ast::program::{MirrProgram, Module};
use mirrc::sexpr::types::SExpr;
use mirrc::sexpr::{ast_to_sexpr, parse_sexpr, print_sexpr, sexpr_to_ast};
use mirrc::{parse_mirr, validate_module};

const MAX_TEST_ITER: usize = 8;

fn empty_program() -> MirrProgram {
    MirrProgram {
        patterns: Vec::new(),
        imports: Vec::new(),
        module: Module {
            name: "d_test".to_string(),
            signals: Vec::new(),
            guards: Vec::new(),
            reflexes: Vec::new(),
            properties: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            span: None,
        },
    }
}

fn roundtrip_src(src: &str, label: &str) {
    let parsed = parse_mirr(src).unwrap_or_else(|e| panic!("{label}: parse: {e}"));
    validate_module(&parsed.module).unwrap_or_else(|e| panic!("{label}: validate: {e}"));
    let s1 = ast_to_sexpr(&parsed);
    let rt = sexpr_to_ast(&s1).unwrap_or_else(|e| panic!("{label}: sexpr_to_ast: {e}"));
    let s2 = ast_to_sexpr(&rt);
    assert_eq!(print_sexpr(&s1), print_sexpr(&s2), "{label}: roundtrip mismatch");
}

// D1: SExpr type construction
#[test]
fn d1_atom_prints_correctly() {
    assert_eq!(print_sexpr(&SExpr::sym("hi")), "hi");
}
#[test]
fn d1_integer_prints_correctly() {
    assert_eq!(print_sexpr(&SExpr::int(7)), "7");
}
#[test]
fn d1_bool_true() {
    let t = print_sexpr(&SExpr::bool_val(true));
    assert!(t == "true" || t == "#t");
}
#[test]
fn d1_bool_false() {
    let t = print_sexpr(&SExpr::bool_val(false));
    assert!(t == "false" || t == "#f");
}
#[test]
fn d1_empty_list() {
    let t = print_sexpr(&SExpr::list(Vec::new()));
    assert!(t == "()" || t == "( )");
}
#[test]
fn d1_list_with_items() {
    let l = SExpr::list(vec![SExpr::sym("a"), SExpr::int(1)]);
    let text = print_sexpr(&l);
    assert!(text.contains('a') && text.contains('1'));
}
#[test]
fn d1_as_list_some_for_list() {
    assert!(SExpr::list(Vec::new()).as_list().is_some());
}
#[test]
fn d1_as_list_none_for_atom() {
    assert!(SExpr::sym("x").as_list().is_none());
}

// D2: parse_sexpr
#[test]
fn d2_parse_atom() {
    assert!(parse_sexpr("hello").is_ok());
}
#[test]
fn d2_parse_integer() {
    assert!(parse_sexpr("42").is_ok());
}
#[test]
fn d2_parse_empty_list() {
    assert!(parse_sexpr("()").is_ok());
}
#[test]
fn d2_parse_nested_list() {
    let r = parse_sexpr("(a (b c) d)").unwrap();
    assert_eq!(r.as_list().unwrap().len(), 3);
}
#[test]
fn d2_parse_unbalanced_err() {
    assert!(parse_sexpr("(unclosed").is_err());
}

// D3: print_sexpr
#[test]
fn d3_print_zero() {
    assert_eq!(print_sexpr(&SExpr::int(0)), "0");
}
#[test]
fn d3_print_large_int() {
    assert_eq!(print_sexpr(&SExpr::int(65535)), "65535");
}
#[test]
fn d3_print_parse_roundtrip() {
    let orig = SExpr::list(vec![SExpr::sym("x"), SExpr::int(5)]);
    let text = print_sexpr(&orig);
    let reparsed = parse_sexpr(&text).unwrap();
    assert_eq!(text, print_sexpr(&reparsed));
}

// D4: depth limits
#[test]
fn d4_shallow_nesting_ok() {
    let mut expr = SExpr::sym("leaf");
    let mut d = 0usize;
    while d < 5 {
        expr = SExpr::list(vec![SExpr::sym("n"), expr]);
        d += 1;
    }
    assert!(parse_sexpr(&print_sexpr(&expr)).is_ok());
}

// D5: roundtrip
#[test]
fn d5_empty_program_roundtrip() {
    let prog = empty_program();
    let s1 = print_sexpr(&ast_to_sexpr(&prog));
    let rt = sexpr_to_ast(&ast_to_sexpr(&prog)).unwrap();
    let s2 = print_sexpr(&ast_to_sexpr(&rt));
    assert_eq!(s1, s2);
}
#[test]
fn d5_module_with_signal_roundtrip() {
    roundtrip_src(
        r#"module m {
    signal x: in u8;
    signal y: out bool;
}"#,
        "signal_roundtrip",
    );
}
#[test]
fn d5_module_with_guard_roundtrip() {
    roundtrip_src(
        r#"module g {
    signal x: in u8;
    signal y: out bool;
    guard gd {
        when (x > 10)
        for 1 cycles;
    }
    reflex r {
        on gd {
            y = true;
        }
    }
}"#,
        "guard_roundtrip",
    );
}
#[test]
fn d5_stable_after_multiple_iterations() {
    let prog = empty_program();
    let base = print_sexpr(&ast_to_sexpr(&prog));
    let mut i = 0usize;
    while i < MAX_TEST_ITER {
        let rt = sexpr_to_ast(&ast_to_sexpr(&prog)).unwrap();
        assert_eq!(base, print_sexpr(&ast_to_sexpr(&rt)));
        i += 1;
    }
}
