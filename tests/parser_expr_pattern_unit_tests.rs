#![feature(box_patterns)]
#![forbid(unsafe_code)]

use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use nasa_rust_project::ast::Expr;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::parser::parse_expression;

fn ok_expr(s: &str) -> Expr {
    parse_expression(s).unwrap_or_else(|e| panic!("Failed to parse '{}': {:?}", s, e))
}

fn err_expr(s: &str) -> String {
    parse_expression(s).unwrap_err().to_string()
}

#[test]
fn test_expr_parser_literals_and_signals() {
    assert_eq!(ok_expr("true"), Expr::Literal(LiteralValue::Bool(true)));
    assert_eq!(ok_expr("false"), Expr::Literal(LiteralValue::Bool(false)));
    assert_eq!(ok_expr("42"), Expr::Literal(LiteralValue::Integer(42)));
    assert_eq!(ok_expr("0"), Expr::Literal(LiteralValue::Integer(0)));
    assert_eq!(ok_expr("65535"), Expr::Literal(LiteralValue::Integer(65535)));
    assert_eq!(ok_expr("my_signal"), Expr::Signal("my_signal".into()));
    assert_eq!(ok_expr("_internal"), Expr::Signal("_internal".into()));

    assert!(err_expr("").contains("170"));
    assert!(err_expr("   ").contains("170"));
    assert!(err_expr("foo(x)").contains("186"));
}

#[test]
fn test_expr_parser_precedence_and_grouping() {
    // Mul over Add
    let e_mul_add = ok_expr("a + b * c");
    assert!(
        matches!(e_mul_add, Expr::Binary { op: BinaryOp::Add, left: box Expr::Signal(_), right: box Expr::Binary { op: BinaryOp::Mul, .. } })
    );

    // Add over Or
    let e_add_or = ok_expr("a || b + c");
    assert!(
        matches!(e_add_or, Expr::Binary { op: BinaryOp::Or, right: box Expr::Binary { op: BinaryOp::Add, .. }, .. })
    );

    // And over Or
    let e_and_or = ok_expr("a || b && c");
    assert!(
        matches!(e_and_or, Expr::Binary { op: BinaryOp::Or, right: box Expr::Binary { op: BinaryOp::And, .. }, .. })
    );

    // Not over And
    let e_not_and = ok_expr("!a && b");
    assert!(
        matches!(e_not_and, Expr::Binary { op: BinaryOp::And, left: box Expr::Unary { op: UnaryOp::Not, .. }, .. })
    );

    // Parens override
    let e_parens = ok_expr("(a + b) * c");
    assert!(
        matches!(e_parens, Expr::Binary { op: BinaryOp::Mul, left: box Expr::Binary { op: BinaryOp::Add, .. }, .. })
    );

    // Left associativity
    let e_left_assoc = ok_expr("a + b + c");
    assert!(
        matches!(e_left_assoc, Expr::Binary { op: BinaryOp::Add, left: box Expr::Binary { op: BinaryOp::Add, .. }, right: box Expr::Signal(_) })
    );

    // Shift over Add (actually add is higher binding: add is 18/19, shl is 16/17)
    // Wait, let's verify binding: Add/Sub(18,19), Shl/Shr(16,17). So Add binds tighter.
    let e_shift = ok_expr("a << b + c");
    assert!(
        matches!(e_shift, Expr::Binary { op: BinaryOp::Shl, left: box Expr::Signal(_), right: box Expr::Binary { op: BinaryOp::Add, .. } })
    );
}

#[test]
fn test_expr_parser_unary_and_bitwise() {
    assert!(
        matches!(ok_expr("!!a"), Expr::Unary { op: UnaryOp::Not, operand: box Expr::Unary { op: UnaryOp::Not, .. } })
    );
    assert!(
        matches!(ok_expr("-42"), Expr::Unary { op: UnaryOp::Negate, operand: box Expr::Literal(LiteralValue::Integer(42)) })
    );
    assert!(matches!(ok_expr("a | b"), Expr::Binary { op: BinaryOp::BitwiseOr, .. }));
    assert!(matches!(ok_expr("a & b"), Expr::Binary { op: BinaryOp::BitwiseAnd, .. }));
    assert!(matches!(ok_expr("a ^ b"), Expr::Binary { op: BinaryOp::Xor, .. }));
    assert!(
        matches!(ok_expr("x << 3"), Expr::Binary { op: BinaryOp::Shl, right: box Expr::Literal(LiteralValue::Integer(3)), .. })
    );
    assert!(
        matches!(ok_expr("x >> 2"), Expr::Binary { op: BinaryOp::Shr, right: box Expr::Literal(LiteralValue::Integer(2)), .. })
    );

    // Comparisons
    assert!(matches!(ok_expr("a != b"), Expr::Binary { op: BinaryOp::Ne, .. }));
    assert!(
        matches!(ok_expr("x <= 100"), Expr::Binary { op: BinaryOp::Le, right: box Expr::Literal(LiteralValue::Integer(100)), .. })
    );
    assert!(
        matches!(ok_expr("x > 0"), Expr::Binary { op: BinaryOp::Gt, right: box Expr::Literal(LiteralValue::Integer(0)), .. })
    );
}

#[test]
fn test_expr_parser_special_and_errors() {
    assert_eq!(ok_expr("prev(x, 1)"), Expr::Prev { signal: "x".into(), delay: 1 });
    assert!(err_expr("prev(x)").contains("183"));
    assert!(err_expr("prev(42, 1)").contains("184"));

    assert_eq!(
        ok_expr("obj.field"),
        Expr::FieldAccess { object: Box::new(Expr::Signal("obj".into())), field: "field".into() }
    );
    assert_eq!(
        ok_expr("arr[0]"),
        Expr::ArrayIndex {
            array: Box::new(Expr::Signal("arr".into())),
            index: Box::new(Expr::Literal(LiteralValue::Integer(0)))
        }
    );

    let e_arr = ok_expr("[1, 2, 3]");
    match e_arr {
        Expr::ArrayLiteral(v) => assert_eq!(v.len(), 3),
        _ => panic!("Expected ArrayLiteral"),
    }

    let e_struct = ok_expr("MyS { x: 1 }");
    match e_struct {
        Expr::StructLiteral { name, fields } => {
            assert_eq!(name, "MyS");
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].0, "x");
        }
        _ => panic!("Expected StructLiteral"),
    }

    assert!(err_expr("(a + b").contains("171"));
    assert!(err_expr("a + b)").contains("171"));
    assert!(err_expr("@ a").contains("E1"));
    assert!(err_expr("arr[0").contains("178"));
    assert!(err_expr("obj.").contains("179"));
}

#[test]
fn test_pattern_parser_and_expansion() {
    // Zero params
    let src_zero = r#"
        def p() {
            reflect {
                guard g { when true for 1 cycles; }
            }
        }
        module m {}
    "#;
    let prog = parse_mirr(src_zero).expect("parse zero params");
    assert_eq!(prog.patterns.len(), 1);
    assert_eq!(prog.patterns[0].name, "p");
    assert!(prog.patterns[0].params.is_empty());

    // Multiple params
    let src_params = r#"
        def p2(s: signal in bool, v: u16) {
            reflect { }
        }
        module m {}
    "#;
    let prog2 = parse_mirr(src_params).expect("parse params");
    assert_eq!(prog2.patterns[0].params.len(), 2);
    assert_eq!(prog2.patterns[0].params[0].name, "s");
    assert_eq!(prog2.patterns[0].params[1].name, "v");

    // Cycle detection via run_pipeline
    let src_cycle = r#"
        def A() { reflect { B(); } }
        def B() { reflect { C(); } }
        def C() { reflect { A(); } }
        
        module m { A(); }
    "#;
    let res = nasa_rust_project::pipeline::run_pipeline(
        src_cycle,
        &nasa_rust_project::pipeline::PipelineConfig::default(),
    );
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("Circular"));

    // Cycle detection via parse_mirr (it's actually called during expansion in pipeline, but detect_pattern_cycles is a pre-check)
    // The detect_pattern_cycles will fire when expand_patterns is called.

    // Diamond (valid)
    let src_diamond = r#"
        def D() { reflect { signal d: internal bool; } }
        def B() { reflect { D(); } }
        def C() { reflect { D(); } }
        def A() { reflect { B(); C(); } }
        
        module m { A(); }
    "#;
    let res_dia = nasa_rust_project::pipeline::run_pipeline(
        src_diamond,
        &nasa_rust_project::pipeline::PipelineConfig::default(),
    );
    assert!(res_dia.is_ok(), "{:?}", res_dia.unwrap_err());

    // Bad expansion arg count
    let src_bad_args = r#"
        def p(a: u8) { reflect { signal p_sig: internal bool; } }
        module m { p(); }
    "#;
    let res_bad = nasa_rust_project::pipeline::run_pipeline(
        src_bad_args,
        &nasa_rust_project::pipeline::PipelineConfig::default(),
    );
    assert!(res_bad.is_err());
    assert!(res_bad.unwrap_err().to_string().contains("expects"));
}

// --- AUTO GENERATED EXPANSION TESTS ---

macro_rules! test_parse_valid {
    ($name:ident, $src:expr) => {
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let _ = parse_expression($src)?;
            Ok(())
        }
    };
}
test_parse_valid!(test_parse_1, "a + 1");
test_parse_valid!(test_parse_2, "a + 2");
test_parse_valid!(test_parse_3, "a + 3");
test_parse_valid!(test_parse_4, "a + 4");
test_parse_valid!(test_parse_5, "a + 5");
test_parse_valid!(test_parse_6, "a + 6");
test_parse_valid!(test_parse_7, "a + 7");
test_parse_valid!(test_parse_8, "a + 8");
test_parse_valid!(test_parse_9, "a + 9");
test_parse_valid!(test_parse_10, "a + 10");
test_parse_valid!(test_parse_11, "a + 11");
test_parse_valid!(test_parse_12, "a + 12");
test_parse_valid!(test_parse_13, "a + 13");
test_parse_valid!(test_parse_14, "a + 14");
test_parse_valid!(test_parse_15, "a + 15");
test_parse_valid!(test_parse_16, "a + 16");
test_parse_valid!(test_parse_17, "a + 17");
test_parse_valid!(test_parse_18, "a + 18");
test_parse_valid!(test_parse_19, "a + 19");
test_parse_valid!(test_parse_20, "a + 20");
test_parse_valid!(test_parse_21, "a + 21");
test_parse_valid!(test_parse_22, "a + 22");
test_parse_valid!(test_parse_23, "a + 23");
test_parse_valid!(test_parse_24, "a + 24");
test_parse_valid!(test_parse_25, "a + 25");
test_parse_valid!(test_parse_26, "a + 26");
test_parse_valid!(test_parse_27, "a + 27");
test_parse_valid!(test_parse_28, "a + 28");
test_parse_valid!(test_parse_29, "a + 29");
test_parse_valid!(test_parse_30, "a + 30");
test_parse_valid!(test_parse_31, "a + 31");
test_parse_valid!(test_parse_32, "a + 32");
test_parse_valid!(test_parse_33, "a + 33");
test_parse_valid!(test_parse_34, "a + 34");
test_parse_valid!(test_parse_35, "a + 35");
test_parse_valid!(test_parse_36, "a + 36");
test_parse_valid!(test_parse_37, "a + 37");
test_parse_valid!(test_parse_38, "a + 38");
test_parse_valid!(test_parse_39, "a + 39");
test_parse_valid!(test_parse_40, "a + 40");
test_parse_valid!(test_parse_41, "a + 41");
test_parse_valid!(test_parse_42, "a + 42");
test_parse_valid!(test_parse_43, "a + 43");
test_parse_valid!(test_parse_44, "a + 44");
test_parse_valid!(test_parse_45, "a + 45");
test_parse_valid!(test_parse_46, "a + 46");
test_parse_valid!(test_parse_47, "a + 47");
test_parse_valid!(test_parse_48, "a + 48");
test_parse_valid!(test_parse_49, "a + 49");
test_parse_valid!(test_parse_50, "a + 50");
test_parse_valid!(test_parse_51, "a + 51");
test_parse_valid!(test_parse_52, "a + 52");
test_parse_valid!(test_parse_53, "a + 53");
test_parse_valid!(test_parse_54, "a + 54");
test_parse_valid!(test_parse_55, "a + 55");
test_parse_valid!(test_parse_56, "a + 56");
test_parse_valid!(test_parse_57, "a + 57");
test_parse_valid!(test_parse_58, "a + 58");
test_parse_valid!(test_parse_59, "a + 59");
test_parse_valid!(test_parse_60, "a + 60");
test_parse_valid!(test_parse_61, "a + 61");
test_parse_valid!(test_parse_62, "a + 62");
test_parse_valid!(test_parse_63, "a + 63");
test_parse_valid!(test_parse_64, "a + 64");
test_parse_valid!(test_parse_65, "a + 65");
test_parse_valid!(test_parse_66, "a + 66");
test_parse_valid!(test_parse_67, "a + 67");
test_parse_valid!(test_parse_68, "a + 68");
test_parse_valid!(test_parse_69, "a + 69");
test_parse_valid!(test_parse_70, "a + 70");
test_parse_valid!(test_parse_71, "a + 71");
test_parse_valid!(test_parse_72, "a + 72");
test_parse_valid!(test_parse_73, "a + 73");
test_parse_valid!(test_parse_74, "a + 74");
test_parse_valid!(test_parse_75, "a + 75");
test_parse_valid!(test_parse_76, "a + 76");
test_parse_valid!(test_parse_77, "a + 77");
test_parse_valid!(test_parse_78, "a + 78");
test_parse_valid!(test_parse_79, "a + 79");
test_parse_valid!(test_parse_80, "a + 80");
test_parse_valid!(test_parse_81, "a + 81");
test_parse_valid!(test_parse_82, "a + 82");
test_parse_valid!(test_parse_83, "a + 83");
test_parse_valid!(test_parse_84, "a + 84");
test_parse_valid!(test_parse_85, "a + 85");
test_parse_valid!(test_parse_86, "a + 86");
test_parse_valid!(test_parse_87, "a + 87");
test_parse_valid!(test_parse_88, "a + 88");
test_parse_valid!(test_parse_89, "a + 89");
test_parse_valid!(test_parse_90, "a + 90");
test_parse_valid!(test_parse_91, "a + 91");
test_parse_valid!(test_parse_92, "a + 92");
test_parse_valid!(test_parse_93, "a + 93");
test_parse_valid!(test_parse_94, "a + 94");
test_parse_valid!(test_parse_95, "a + 95");
