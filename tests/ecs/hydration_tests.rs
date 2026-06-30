#![forbid(unsafe_code)]
#[allow(unused_imports)]
use mirrc::ast::program::Module;
#[allow(unused_imports)]
use mirrc::ast::types::*;
#[allow(unused_imports)]
use mirrc::ast::BinaryOp;
#[allow(unused_imports)]
use mirrc::ast::Expr;
#[allow(unused_imports)]
use mirrc::ast::UnaryOp;
#[allow(unused_imports)]
use mirrc::ecs::components::*;
#[allow(unused_imports)]
use mirrc::ecs::registry::Registry;
#[allow(unused_imports)]
use mirrc::ecs::systems::*;
#[allow(unused_imports)]
use mirrc::span::Span;

#[test]
fn test_hydration_1() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Bool(true));
    let id = reg.ingest_expr(&expr).unwrap();
    assert!(reg.literals[id.0 as usize].is_some());
}
#[test]
fn test_hydration_2() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Bool(true));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Bool(true)) => (),
        _ => panic!("Expected Literal(true)"),
    }
}
#[test]
fn test_hydration_3() {
    let mut reg = Registry::new();
    let expr = Expr::Signal("some_sig".to_string());
    let id = reg.ingest_expr(&expr).unwrap();
    assert!(reg.pending_signal_refs[id.0 as usize].is_some());
}
#[test]
fn test_hydration_4() {
    let mut reg = Registry::new();
    let expr = Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Literal(LiteralValue::Bool(true))),
    };
    let id = reg.ingest_expr(&expr).unwrap();
    assert!(reg.unary_ops[id.0 as usize].is_some());
}
#[test]
fn test_hydration_5() {
    let mut reg = Registry::new();
    let expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Literal(LiteralValue::Bool(true))),
        right: Box::new(Expr::Literal(LiteralValue::Bool(false))),
    };
    let id = reg.ingest_expr(&expr).unwrap();
    assert!(reg.binary_ops[id.0 as usize].is_some());
}
#[test]
fn test_hydration_6() {
    let mut reg = Registry::new();
    reg.create_entity("s", KindComponent::SIGNAL);
    let expr = Expr::Prev { signal: "s".to_string(), delay: 1 };
    let id = reg.ingest_expr(&expr).unwrap();
    assert!(reg.prev_ops[id.0 as usize].is_some());
}
#[test]
fn test_hydration_7() {
    let mut reg = Registry::new();
    let module = Module {
        name: "M1".to_string(),
        clock_domains: vec![],
        signals: vec![],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };
    let id = reg.ingest_module(&module).unwrap();
    assert_eq!(reg.resolve_name(reg.names[id.0 as usize].as_ref().unwrap().0), "M1");
}
#[test]
fn test_hydration_8() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(7));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(7)) => (),
        _ => panic!("Expected Literal(7)"),
    }
}
#[test]
fn test_hydration_9() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(8));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(8)) => (),
        _ => panic!("Expected Literal(8)"),
    }
}
#[test]
fn test_hydration_10() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(9));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(9)) => (),
        _ => panic!("Expected Literal(9)"),
    }
}
#[test]
fn test_hydration_11() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(10));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(10)) => (),
        _ => panic!("Expected Literal(10)"),
    }
}
#[test]
fn test_hydration_12() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(11));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(11)) => (),
        _ => panic!("Expected Literal(11)"),
    }
}
#[test]
fn test_hydration_13() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(12));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(12)) => (),
        _ => panic!("Expected Literal(12)"),
    }
}
#[test]
fn test_hydration_14() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(13));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(13)) => (),
        _ => panic!("Expected Literal(13)"),
    }
}
#[test]
fn test_hydration_15() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(14));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(14)) => (),
        _ => panic!("Expected Literal(14)"),
    }
}
#[test]
fn test_hydration_16() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(15));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(15)) => (),
        _ => panic!("Expected Literal(15)"),
    }
}
#[test]
fn test_hydration_17() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(16));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(16)) => (),
        _ => panic!("Expected Literal(16)"),
    }
}
#[test]
fn test_hydration_18() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(17));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(17)) => (),
        _ => panic!("Expected Literal(17)"),
    }
}
#[test]
fn test_hydration_19() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(18));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(18)) => (),
        _ => panic!("Expected Literal(18)"),
    }
}
#[test]
fn test_hydration_20() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(19));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(19)) => (),
        _ => panic!("Expected Literal(19)"),
    }
}
#[test]
fn test_hydration_21() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(20));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(20)) => (),
        _ => panic!("Expected Literal(20)"),
    }
}
#[test]
fn test_hydration_22() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(21));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(21)) => (),
        _ => panic!("Expected Literal(21)"),
    }
}
#[test]
fn test_hydration_23() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(22));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(22)) => (),
        _ => panic!("Expected Literal(22)"),
    }
}
#[test]
fn test_hydration_24() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(23));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(23)) => (),
        _ => panic!("Expected Literal(23)"),
    }
}
#[test]
fn test_hydration_25() {
    let mut reg = Registry::new();
    let expr = Expr::Literal(LiteralValue::Integer(24));
    let id = reg.ingest_expr(&expr).unwrap();
    let reified = reg.reify_expr(id).unwrap();
    match reified {
        Expr::Literal(LiteralValue::Integer(24)) => (),
        _ => panic!("Expected Literal(24)"),
    }
}
