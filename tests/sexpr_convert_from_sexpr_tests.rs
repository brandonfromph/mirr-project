#![forbid(unsafe_code)]
//! S-expression → AST conversion tests (`sexpr_to_ast`) and roundtrip.
//!
//! NASA P10: bounded iteration, no recursion.

use nasa_rust_project::ast::program::{MirrProgram, Module};
use nasa_rust_project::sexpr::convert::{ast_to_sexpr, sexpr_to_ast};
use nasa_rust_project::sexpr::print_sexpr;

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
    use nasa_rust_project::ast::program::SignalDecl;
    use nasa_rust_project::ast::types::{ExtendedType, SignalKind, SignalType};

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
    use nasa_rust_project::ast::program::SignalDecl;
    use nasa_rust_project::ast::types::{ExtendedType, SignalKind, SignalType};

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
    use nasa_rust_project::ast::expr::Expr;
    use nasa_rust_project::ast::program::Guard;
    use nasa_rust_project::ast::types::LiteralValue;

    let mut prog = empty_program();
    prog.module.guards.push(Guard {
        name: "g1".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles: 1,
        origin: None,
        span: None,
    });
    prog.module.guards.push(Guard {
        name: "g2".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(false)),
        cycles: 2,
        origin: None,
        span: None,
    });
    let rt = roundtrip(&prog);
    assert_eq!(rt.module.guards.len(), 2, "guard count must survive roundtrip");
}

#[test]
fn roundtrip_property_count() {
    use nasa_rust_project::ast::expr::Expr;
    use nasa_rust_project::ast::program::SignalDecl;
    use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
    use nasa_rust_project::ast::types::{ExtendedType, SignalKind, SignalType};

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
