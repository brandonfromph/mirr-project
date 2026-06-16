#![forbid(unsafe_code)]
//! Typecheck node-budget contract tests.

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};

fn typecheck_module(module: &Module) -> Result<(), mirrc::error::PipelineErrors> {
    let mut registry = mirrc::ecs::Registry::new();
    registry.ingest_module(module).map_err(|e| mirrc::error::PipelineErrors { errors: vec![e] })?;
    registry.semantic_validate()?;
    registry.typecheck(false)
}

const MAX_EXPR_NODES_BUDGET: usize = 8192;

fn deep_add_expression(depth: usize) -> Expr {
    let mut expr = Expr::Signal("n".to_string());
    let mut i = 0usize;
    while i < depth {
        expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(expr),
            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
        };
        i += 1;
    }
    expr
}

fn budget_module_with_guard(condition: Expr) -> Module {
    Module {
        name: "typeck_budget".to_string(),
        signals: vec![
            SignalDecl {
                name: "n".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "out".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition,
            cycles: 1,
            template_cycles: None,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "out".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

#[test]
fn deep_expression_over_budget_reports_e607() {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let m = budget_module_with_guard(deep_add_expression(MAX_EXPR_NODES_BUDGET + 8));
            let errs = typecheck_module(&m).expect_err("typecheck should fail");
            let msg = errs.to_string();
            assert!(msg.contains("[EFATAL]"), "expected EFATAL, got: {msg}");
            assert!(msg.contains("MAX_EXPR_NODES"), "expected bound detail, got: {msg}");
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn large_array_literal_over_budget_reports_e607() {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let mut elems = Vec::with_capacity(MAX_EXPR_NODES_BUDGET + 4);
            let mut i = 0usize;
            while i < MAX_EXPR_NODES_BUDGET + 4 {
                elems.push(Expr::Literal(LiteralValue::Integer(i as u64)));
                i += 1;
            }

            let m = budget_module_with_guard(Expr::ArrayLiteral(elems));
            let errs = typecheck_module(&m).expect_err("typecheck should fail");
            let msg = errs.to_string();
            assert!(msg.contains("[EFATAL]"), "expected EFATAL, got: {msg}");
        })
        .unwrap()
        .join()
        .unwrap();
}
