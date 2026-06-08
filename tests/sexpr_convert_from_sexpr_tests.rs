#![forbid(unsafe_code)]
//! S-expression → AST conversion tests (`sexpr_to_ast`) and roundtrip.
//!
//! NASA P10: bounded iteration, no recursion.

use mirrc::ast::program::{MirrProgram, Module};
use mirrc::sexpr::convert::{ast_to_sexpr, sexpr_to_ast};
use mirrc::sexpr::print_sexpr;
use mirrc::sexpr::types::SExpr;

fn empty_program() -> MirrProgram {
    MirrProgram {
        patterns: Vec::new(),
        imports: Vec::new(),
        module: Module {
            name: "rt_test".to_string(),
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

fn roundtrip(prog: &MirrProgram) -> MirrProgram {
    let sexpr = ast_to_sexpr(prog);
    sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for valid AST")
}

fn single_signal_program_sexpr(signal_ty: SExpr) -> SExpr {
    SExpr::list(vec![
        SExpr::sym("program"),
        SExpr::list(vec![SExpr::sym("patterns")]),
        SExpr::list(vec![
            SExpr::sym("module"),
            SExpr::str_val("fifo_shape_test"),
            SExpr::list(vec![
                SExpr::sym("signals"),
                SExpr::list(vec![
                    SExpr::sym("signal"),
                    SExpr::str_val("q"),
                    SExpr::sym("internal"),
                    signal_ty,
                ]),
            ]),
            SExpr::list(vec![SExpr::sym("guards")]),
            SExpr::list(vec![SExpr::sym("reflexes")]),
            SExpr::list(vec![SExpr::sym("properties")]),
            SExpr::list(vec![SExpr::sym("pattern-calls")]),
            SExpr::list(vec![SExpr::sym("pattern-origins")]),
        ]),
    ])
}

#[test]
fn empty_program_roundtrip_succeeds() {
    let _ = roundtrip(&empty_program());
}

#[test]
fn roundtrip_preserves_module_name() {
    let mut prog = empty_program();
    prog.module.name = "my_module".to_string();
    let rt = roundtrip(&prog);
    assert_eq!(rt.module.name, "my_module", "module name must survive roundtrip");
}

#[test]
fn double_roundtrip_idempotent() {
    let prog = empty_program();
    let rt1 = roundtrip(&prog);
    let rt2 = roundtrip(&rt1);
    let s1 = print_sexpr(&ast_to_sexpr(&rt1));
    let s2 = print_sexpr(&ast_to_sexpr(&rt2));
    assert_eq!(s1, s2, "double roundtrip must be idempotent");
}

#[test]
fn sexpr_to_ast_ok_for_valid_sexpr() {
    let sexpr = ast_to_sexpr(&empty_program());
    assert!(sexpr_to_ast(&sexpr).is_ok(), "must return Ok for valid program");
}

#[test]
fn roundtrip_preserves_signal_count() {
    use mirrc::ast::program::SignalDecl;
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};

    let mut prog = empty_program();
    let decls: &[(&str, SignalKind, SignalType)] = &[
        ("a", SignalKind::Input, SignalType::Unsigned(8)),
        ("b", SignalKind::Output, SignalType::Bool),
        ("c", SignalKind::Input, SignalType::Unsigned(16)),
    ];
    let mut i = 0usize;
    while i < decls.len() {
        let (name, kind, ty) = decls[i].clone();
        prog.module.signals.push(SignalDecl {
            name: name.to_string(),
            kind,
            ty: ExtendedType::from_core(ty),
            origin: None,
            span: None,
        });
        i += 1;
    }
    let rt = roundtrip(&prog);
    assert_eq!(rt.module.signals.len(), 3, "signal count must survive roundtrip");
}

#[test]
fn roundtrip_preserves_signal_name() {
    use mirrc::ast::program::SignalDecl;
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};

    let mut prog = empty_program();
    prog.module.signals.push(SignalDecl {
        name: "pressure_sensor".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(SignalType::Unsigned(16)),
        origin: None,
        span: None,
    });
    let rt = roundtrip(&prog);
    assert_eq!(rt.module.signals[0].name, "pressure_sensor");
}

#[test]
fn roundtrip_serializes_to_same_text() {
    let prog = empty_program();
    let s1 = print_sexpr(&ast_to_sexpr(&prog));
    let rt = roundtrip(&prog);
    let s2 = print_sexpr(&ast_to_sexpr(&rt));
    assert_eq!(s1, s2, "roundtrip must produce identical S-expr text");
}

#[test]
fn roundtrip_guard_count() {
    use mirrc::ast::expr::Expr;
    use mirrc::ast::program::Guard;
    use mirrc::ast::types::LiteralValue;

    let mut prog = empty_program();
    prog.module.guards.push(Guard {
        name: "g1".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    prog.module.guards.push(Guard {
        name: "g2".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(false)),
        cycles: 2,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let rt = roundtrip(&prog);
    assert_eq!(rt.module.guards.len(), 2, "guard count must survive roundtrip");
}

#[test]
fn roundtrip_property_count() {
    use mirrc::ast::expr::Expr;
    use mirrc::ast::program::SignalDecl;
    use mirrc::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};

    let mut prog = empty_program();
    prog.module.signals.push(SignalDecl {
        name: "safe".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(SignalType::Bool),
        origin: None,
        span: None,
    });
    prog.module.properties.push(PropertyDecl {
        name: "safety_prop".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(Expr::Signal("safe".to_string())),
        origin: None,
        span: None,
    });
    let rt = roundtrip(&prog);
    assert_eq!(rt.module.properties.len(), 1, "property count must survive roundtrip");
}

#[test]
fn roundtrip_empty_reflexes() {
    let prog = empty_program();
    let rt = roundtrip(&prog);
    assert!(rt.module.reflexes.is_empty(), "empty reflexes must survive roundtrip");
}

#[test]
fn roundtrip_preserves_fifo_signal_type() {
    use mirrc::ast::program::SignalDecl;
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};

    let mut prog = empty_program();
    prog.module.signals.push(SignalDecl {
        name: "sample_fifo".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Fifo {
            element: Box::new(SignalType::Unsigned(8)),
            depth: 4,
        }),
        origin: None,
        span: None,
    });

    let rt = roundtrip(&prog);
    assert_eq!(
        rt.module.signals[0].ty.core,
        SignalType::Fifo { element: Box::new(SignalType::Unsigned(8)), depth: 4 },
        "fifo type must survive roundtrip",
    );
}

#[test]
fn roundtrip_preserves_nested_fifo_element_type() {
    use mirrc::ast::program::SignalDecl;
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};

    let mut prog = empty_program();
    prog.module.signals.push(SignalDecl {
        name: "nested_fifo".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Fifo {
            element: Box::new(SignalType::Array { element: Box::new(SignalType::Bool), length: 2 }),
            depth: 3,
        }),
        origin: None,
        span: None,
    });

    let rt = roundtrip(&prog);
    assert_eq!(
        rt.module.signals[0].ty.core,
        SignalType::Fifo {
            element: Box::new(SignalType::Array { element: Box::new(SignalType::Bool), length: 2 }),
            depth: 3,
        },
        "nested fifo element type must survive roundtrip",
    );
}

#[test]
fn sexpr_to_ast_parses_fifo_type_canonical_shape() {
    use mirrc::ast::types::SignalType;

    let sexpr = single_signal_program_sexpr(SExpr::list(vec![
        SExpr::sym("fifo"),
        SExpr::list(vec![SExpr::sym("unsigned"), SExpr::int(8)]),
        SExpr::int(4),
    ]));

    let parsed = sexpr_to_ast(&sexpr).expect("canonical fifo type shape must parse");
    assert_eq!(
        parsed.module.signals[0].ty.core,
        SignalType::Fifo { element: Box::new(SignalType::Unsigned(8)), depth: 4 },
    );
}

#[test]
fn sexpr_to_ast_accepts_fifo_labeled_shape_and_roundtrips_to_canonical() {
    use mirrc::ast::types::SignalType;

    let sexpr = single_signal_program_sexpr(SExpr::list(vec![
        SExpr::sym("fifo"),
        SExpr::list(vec![
            SExpr::sym("element"),
            SExpr::list(vec![SExpr::sym("array"), SExpr::sym("bool"), SExpr::int(2)]),
        ]),
        SExpr::list(vec![SExpr::sym("depth"), SExpr::int(3)]),
    ]));

    let parsed = sexpr_to_ast(&sexpr).expect("labeled fifo type shape must parse");
    assert_eq!(
        parsed.module.signals[0].ty.core,
        SignalType::Fifo {
            element: Box::new(SignalType::Array { element: Box::new(SignalType::Bool), length: 2 }),
            depth: 3,
        },
    );

    let re_emitted = print_sexpr(&ast_to_sexpr(&parsed));
    assert!(
        re_emitted.contains("(fifo (array bool 2) 3)"),
        "accepted fifo shape must roundtrip to canonical fifo sexpr"
    );
}
