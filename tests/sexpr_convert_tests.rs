#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
//! Comprehensive tests for `src/sexpr/convert.rs` — bidirectional AST <-> S-expression conversion.
//!
//! NASA Power-of-10 compliant: bounded iteration, no recursion, descriptive asserts.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::pattern::{
    PatternArg, PatternCall, PatternDef, PatternOrigin, PatternParam, PatternParamKind,
    ReflectBlock,
};
use nasa_rust_project::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{
    BinaryOp, EffectQualifier, ExtendedType, Linearity, LiteralValue, Refinement, SignalKind,
    SignalType, TypeAnnotations, UnaryOp,
};
use nasa_rust_project::sexpr::convert::{ast_to_sexpr, sexpr_to_ast};
use nasa_rust_project::sexpr::types::SExpr;

/// Maximum test iterations for bounded loops (NASA Power-of-10).
const MAX_TEST_ITEMS: usize = 64;

// =========================================================================
// Helper: build a minimal empty program
// =========================================================================

fn empty_module(name: &str) -> Module {
    Module {
        name: name.to_string(),
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn empty_program() -> MirrProgram {
    MirrProgram { patterns: Vec::new(), module: empty_module("test_module") }
}

fn default_annotations() -> TypeAnnotations {
    TypeAnnotations::default()
}

// =========================================================================
// 1. AST -> S-Expr: Full program structure
// =========================================================================

#[test]
fn ast_to_sexpr_empty_program_has_program_head() {
    let program = empty_program();
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().expect("top-level must be a list");
    assert!(
        items.len() >= 3,
        "program list must have at least 3 elements (head, patterns, module)"
    );
    assert_eq!(items[0].as_symbol(), Some("program"), "first element must be 'program' symbol");
}

#[test]
fn ast_to_sexpr_empty_patterns_section() {
    let program = empty_program();
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().unwrap();
    let patterns = items[1].as_list().expect("patterns must be a list");
    assert_eq!(
        patterns[0].as_symbol(),
        Some("patterns"),
        "patterns section must start with 'patterns' symbol"
    );
    assert_eq!(patterns.len(), 1, "empty patterns section should only contain the head symbol");
}

#[test]
fn ast_to_sexpr_module_name_preserved() {
    let program = MirrProgram { patterns: Vec::new(), module: empty_module("my_mod") };
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().unwrap();
    let module_list = items[2].as_list().expect("module must be a list");
    assert_eq!(module_list[0].as_symbol(), Some("module"), "module section head must be 'module'");
    assert_eq!(module_list[1].as_str_val(), Some("my_mod"), "module name must match input");
}

// =========================================================================
// 2. Signal declarations
// =========================================================================

fn make_signal(name: &str, kind: SignalKind, core: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(core),
        origin: None,
        span: None,
    }
}

#[test]
fn ast_to_sexpr_signal_input_bool() {
    let mut program = empty_program();
    program.module.signals.push(make_signal("enable", SignalKind::Input, SignalType::Bool));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    // signals section is the first section after module name
    let signals_section = module_list[2].as_list().expect("signals section must be a list");
    assert_eq!(signals_section[0].as_symbol(), Some("signals"), "head must be 'signals'");
    let sig = signals_section[1].as_list().expect("signal entry must be a list");
    assert_eq!(sig[0].as_symbol(), Some("signal"), "signal entry head must be 'signal'");
    assert_eq!(sig[1].as_str_val(), Some("enable"), "signal name must be 'enable'");
    assert_eq!(sig[2].as_symbol(), Some("input"), "signal kind must be 'input'");
    assert_eq!(sig[3].as_symbol(), Some("bool"), "signal type must be 'bool'");
}

#[test]
fn ast_to_sexpr_signal_output_unsigned() {
    let mut program = empty_program();
    program.module.signals.push(make_signal(
        "data_out",
        SignalKind::Output,
        SignalType::Unsigned(16),
    ));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let signals_section = module_list[2].as_list().unwrap();
    let sig = signals_section[1].as_list().unwrap();
    assert_eq!(sig[1].as_str_val(), Some("data_out"), "signal name must be 'data_out'");
    assert_eq!(sig[2].as_symbol(), Some("output"), "signal kind must be 'output'");
    let ty = sig[3].as_list().expect("unsigned type must be a list");
    assert_eq!(ty[0].as_symbol(), Some("unsigned"), "type head must be 'unsigned'");
    assert_eq!(ty[1].as_integer(), Some(16), "width must be 16");
}

#[test]
fn ast_to_sexpr_signal_internal_signed() {
    let mut program = empty_program();
    program.module.signals.push(make_signal(
        "counter",
        SignalKind::Internal,
        SignalType::Signed(32),
    ));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let signals_section = module_list[2].as_list().unwrap();
    let sig = signals_section[1].as_list().unwrap();
    assert_eq!(sig[2].as_symbol(), Some("internal"), "signal kind must be 'internal'");
    let ty = sig[3].as_list().expect("signed type must be a list");
    assert_eq!(ty[0].as_symbol(), Some("signed"), "type head must be 'signed'");
    assert_eq!(ty[1].as_integer(), Some(32), "width must be 32");
}

#[test]
fn ast_to_sexpr_signal_all_three_kinds() {
    let mut program = empty_program();
    let kinds = [
        ("a", SignalKind::Input, "input"),
        ("b", SignalKind::Output, "output"),
        ("c", SignalKind::Internal, "internal"),
    ];
    for i in 0..3 {
        let (name, kind, _) = kinds[i];
        program.module.signals.push(make_signal(name, kind, SignalType::Bool));
    }
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let signals_section = module_list[2].as_list().unwrap();
    for i in 0..3 {
        let (_, _, expected_sym) = kinds[i];
        let sig = signals_section[i + 1].as_list().unwrap();
        assert_eq!(sig[2].as_symbol(), Some(expected_sym), "signal kind mismatch at index {i}");
    }
}

// =========================================================================
// 3. Type annotations
// =========================================================================

fn make_signal_with_annotations(name: &str, ann: TypeAnnotations) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::new(SignalType::Unsigned(8), ann),
        origin: None,
        span: None,
    }
}

#[test]
fn ast_to_sexpr_annotations_linearity_linear() {
    let mut ann = default_annotations();
    ann.linearity = Linearity::Linear;
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("lin_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    // signal, name, kind, type, annotations
    assert!(sig.len() >= 5, "signal with annotations must have at least 5 elements");
    let annotations = sig[4].as_list().expect("annotations must be a list");
    assert_eq!(annotations[0].as_symbol(), Some("annotations"), "head must be 'annotations'");
    let linearity = annotations[1].as_list().expect("linearity annotation must be a list");
    assert_eq!(linearity[0].as_symbol(), Some("linearity"), "annotation head must be 'linearity'");
    assert_eq!(linearity[1].as_symbol(), Some("linear"), "linearity value must be 'linear'");
}

#[test]
fn ast_to_sexpr_annotations_effect_stateful() {
    let mut ann = default_annotations();
    ann.effect = EffectQualifier::Stateful;
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("stateful_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let effect = annotations[1].as_list().expect("effect annotation must be a list");
    assert_eq!(effect[0].as_symbol(), Some("effect"), "annotation head must be 'effect'");
    assert_eq!(effect[1].as_symbol(), Some("stateful"), "effect value must be 'stateful'");
}

#[test]
fn ast_to_sexpr_annotations_effect_pure() {
    let mut ann = default_annotations();
    ann.effect = EffectQualifier::Pure;
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("pure_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let effect = annotations[1].as_list().unwrap();
    assert_eq!(effect[1].as_symbol(), Some("pure"), "effect value must be 'pure'");
}

#[test]
fn ast_to_sexpr_annotations_refinement_range() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Range { lo: 10, hi: 200 });
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("ranged", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let refinement = annotations[1].as_list().expect("refinement annotation must be a list");
    assert_eq!(refinement[0].as_symbol(), Some("refinement"), "head must be 'refinement'");
    let range = refinement[1].as_list().expect("range must be a list");
    assert_eq!(range[0].as_symbol(), Some("range"), "range head must be 'range'");
    assert_eq!(range[1].as_integer(), Some(10), "range lo must be 10");
    assert_eq!(range[2].as_integer(), Some(200), "range hi must be 200");
}

#[test]
fn ast_to_sexpr_annotations_refinement_predicate() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Predicate("value < 1024".to_string()));
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("pred_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let refinement = annotations[1].as_list().unwrap();
    let predicate = refinement[1].as_list().expect("predicate must be a list");
    assert_eq!(predicate[0].as_symbol(), Some("predicate"), "head must be 'predicate'");
    assert_eq!(predicate[1].as_str_val(), Some("value < 1024"), "predicate expression must match");
}

#[test]
fn ast_to_sexpr_annotations_clock_domain() {
    let mut ann = default_annotations();
    ann.clock_domain = Some("fast_clk".to_string());
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("clk_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let clock = annotations[1].as_list().expect("clock-domain annotation must be a list");
    assert_eq!(clock[0].as_symbol(), Some("clock-domain"), "head must be 'clock-domain'");
    assert_eq!(clock[1].as_str_val(), Some("fast_clk"), "clock domain must be 'fast_clk'");
}

#[test]
fn ast_to_sexpr_annotations_phantom_tag() {
    let mut ann = default_annotations();
    ann.phantom_tag = Some("Celsius".to_string());
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("temp_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let phantom = annotations[1].as_list().expect("phantom-tag annotation must be a list");
    assert_eq!(phantom[0].as_symbol(), Some("phantom-tag"), "head must be 'phantom-tag'");
    assert_eq!(phantom[1].as_str_val(), Some("Celsius"), "phantom tag must be 'Celsius'");
}

#[test]
fn ast_to_sexpr_default_annotations_omitted() {
    let mut program = empty_program();
    program.module.signals.push(make_signal("plain", SignalKind::Input, SignalType::Bool));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    // No annotations element should be present (only signal, name, kind, type = 4 elements)
    assert_eq!(sig.len(), 4, "signal with default annotations must have exactly 4 elements");
}

// =========================================================================
// 4. Expression conversion
// =========================================================================

#[test]
fn ast_to_sexpr_guard_with_bool_literal() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_true".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    // guards section is after signals
    let guards_section = module_list[3].as_list().unwrap();
    assert_eq!(guards_section[0].as_symbol(), Some("guards"), "head must be 'guards'");
    let guard = guards_section[1].as_list().unwrap();
    assert_eq!(guard[0].as_symbol(), Some("guard"), "guard head must be 'guard'");
    assert_eq!(guard[1].as_str_val(), Some("g_true"), "guard name must be 'g_true'");
    assert_eq!(guard[2].as_bool(), Some(true), "condition must be bool true");
    assert_eq!(guard[3].as_integer(), Some(1), "cycles must be 1");
}

#[test]
fn ast_to_sexpr_guard_with_integer_literal() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_num".to_string(),
        condition: Expr::Literal(LiteralValue::Integer(42)),
        cycles: 5,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    assert_eq!(guard[2].as_integer(), Some(42), "condition must be integer 42");
    assert_eq!(guard[3].as_integer(), Some(5), "cycles must be 5");
}

#[test]
fn ast_to_sexpr_guard_with_signal_expr() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_sig".to_string(),
        condition: Expr::Signal("enable".to_string()),
        cycles: 3,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let sig_expr = guard[2].as_list().expect("signal expr must be a list");
    assert_eq!(sig_expr[0].as_symbol(), Some("signal"), "signal expr head must be 'signal'");
    assert_eq!(sig_expr[1].as_str_val(), Some("enable"), "signal name must be 'enable'");
}

#[test]
fn ast_to_sexpr_guard_with_prev_expr() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_prev".to_string(),
        condition: Expr::Prev { signal: "temp".to_string(), delay: 2 },
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let prev_expr = guard[2].as_list().expect("prev expr must be a list");
    assert_eq!(prev_expr[0].as_symbol(), Some("prev"), "prev expr head must be 'prev'");
    assert_eq!(prev_expr[1].as_str_val(), Some("temp"), "prev signal name must be 'temp'");
    assert_eq!(prev_expr[2].as_integer(), Some(2), "prev delay must be 2");
}

#[test]
fn ast_to_sexpr_guard_with_unary_not() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_not".to_string(),
        condition: Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Literal(LiteralValue::Bool(false))),
        },
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let not_expr = guard[2].as_list().expect("not expr must be a list");
    assert_eq!(not_expr[0].as_symbol(), Some("not"), "unary expr head must be 'not'");
    assert_eq!(not_expr[1].as_bool(), Some(false), "operand must be false");
}

#[test]
fn ast_to_sexpr_guard_with_unary_negate() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_neg".to_string(),
        condition: Expr::Unary {
            op: UnaryOp::Negate,
            operand: Box::new(Expr::Literal(LiteralValue::Integer(7))),
        },
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let neg_expr = guard[2].as_list().expect("negate expr must be a list");
    assert_eq!(neg_expr[0].as_symbol(), Some("negate"), "unary expr head must be 'negate'");
    assert_eq!(neg_expr[1].as_integer(), Some(7), "operand must be 7");
}

#[test]
fn ast_to_sexpr_all_binary_operators() {
    let ops: [(BinaryOp, &str); 13] = [
        (BinaryOp::And, "and"),
        (BinaryOp::Or, "or"),
        (BinaryOp::Xor, "xor"),
        (BinaryOp::Lt, "<"),
        (BinaryOp::Le, "<="),
        (BinaryOp::Gt, ">"),
        (BinaryOp::Ge, ">="),
        (BinaryOp::Eq, "=="),
        (BinaryOp::Ne, "!="),
        (BinaryOp::Add, "+"),
        (BinaryOp::Sub, "-"),
        (BinaryOp::Mul, "*"),
        (BinaryOp::Shl, "<<"),
    ];
    for i in 0..13 {
        let (op, expected_sym) = ops[i];
        let mut program = empty_program();
        program.module.guards.push(Guard {
            name: format!("g_{i}"),
            condition: Expr::Binary {
                op,
                left: Box::new(Expr::Literal(LiteralValue::Integer(1))),
                right: Box::new(Expr::Literal(LiteralValue::Integer(2))),
            },
            cycles: 1,
            origin: None,
            span: None,
        });
        let sexpr = ast_to_sexpr(&program);
        let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
        let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
        let bin_expr = guard[2].as_list().expect("binary expr must be a list");
        assert_eq!(
            bin_expr[0].as_symbol(),
            Some(expected_sym),
            "binary op symbol mismatch for op index {i}"
        );
        assert_eq!(bin_expr[1].as_integer(), Some(1), "left operand must be 1 for op index {i}");
        assert_eq!(bin_expr[2].as_integer(), Some(2), "right operand must be 2 for op index {i}");
    }
}

#[test]
fn ast_to_sexpr_binary_shr_operator() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_shr".to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Shr,
            left: Box::new(Expr::Literal(LiteralValue::Integer(8))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
        },
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let bin_expr = guard[2].as_list().unwrap();
    assert_eq!(bin_expr[0].as_symbol(), Some(">>"), "shr operator must be '>>'");
}

#[test]
fn ast_to_sexpr_nested_binary_expression() {
    // (a + b) > 10
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_nested".to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Signal("b".to_string())),
            }),
            right: Box::new(Expr::Literal(LiteralValue::Integer(10))),
        },
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let gt_expr = guard[2].as_list().unwrap();
    assert_eq!(gt_expr[0].as_symbol(), Some(">"), "outer op must be '>'");
    let add_expr = gt_expr[1].as_list().expect("left operand of '>' must be add-expression list");
    assert_eq!(add_expr[0].as_symbol(), Some("+"), "inner op must be '+'");
    assert_eq!(gt_expr[2].as_integer(), Some(10), "right operand of '>' must be 10");
}

// =========================================================================
// 5. Reflex conversion
// =========================================================================

#[test]
fn ast_to_sexpr_reflex_with_assignments() {
    let mut program = empty_program();
    program.module.reflexes.push(Reflex {
        name: "r_drive".to_string(),
        guard_names: vec!["g_hot".to_string(), "g_cold".to_string()],
        assignments: vec![
            Assignment {
                target: "alarm".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            },
            Assignment {
                target: "count".to_string(),
                value: Expr::Literal(LiteralValue::Integer(0)),
                span: None,
            },
        ],
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let reflexes_section = module_list[4].as_list().unwrap();
    assert_eq!(reflexes_section[0].as_symbol(), Some("reflexes"), "head must be 'reflexes'");
    let reflex = reflexes_section[1].as_list().unwrap();
    assert_eq!(reflex[0].as_symbol(), Some("reflex"), "reflex head must be 'reflex'");
    assert_eq!(reflex[1].as_str_val(), Some("r_drive"), "reflex name must be 'r_drive'");

    // on-clause
    let on_clause = reflex[2].as_list().expect("on clause must be a list");
    assert_eq!(on_clause[0].as_symbol(), Some("on"), "on-clause head must be 'on'");
    assert_eq!(on_clause[1].as_str_val(), Some("g_hot"), "first guard must be 'g_hot'");
    assert_eq!(on_clause[2].as_str_val(), Some("g_cold"), "second guard must be 'g_cold'");

    // assignments
    let assign1 = reflex[3].as_list().expect("first assignment must be a list");
    assert_eq!(assign1[0].as_symbol(), Some("assign"), "assign head must be 'assign'");
    assert_eq!(assign1[1].as_str_val(), Some("alarm"), "assign target must be 'alarm'");
    assert_eq!(assign1[2].as_bool(), Some(true), "assign value must be true");

    let assign2 = reflex[4].as_list().expect("second assignment must be a list");
    assert_eq!(assign2[1].as_str_val(), Some("count"), "second assign target must be 'count'");
    assert_eq!(assign2[2].as_integer(), Some(0), "second assign value must be 0");
}

// =========================================================================
// 6. Property conversion
// =========================================================================

#[test]
fn ast_to_sexpr_property_always() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_safe".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(Expr::Signal("ok".to_string())),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let props_section = module_list[5].as_list().unwrap();
    assert_eq!(props_section[0].as_symbol(), Some("properties"), "head must be 'properties'");
    let prop = props_section[1].as_list().unwrap();
    assert_eq!(prop[1].as_str_val(), Some("p_safe"), "property name must be 'p_safe'");
    assert_eq!(prop[2].as_symbol(), Some("assert"), "directive must be 'assert'");
    let formula = prop[3].as_list().expect("formula must be a list");
    assert_eq!(formula[0].as_symbol(), Some("always"), "formula head must be 'always'");
}

#[test]
fn ast_to_sexpr_property_never() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_never".to_string(),
        directive: PropertyDirective::Cover,
        formula: PropertyFormula::Never(Expr::Literal(LiteralValue::Bool(false))),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    assert_eq!(prop[2].as_symbol(), Some("cover"), "directive must be 'cover'");
    let formula = prop[3].as_list().unwrap();
    assert_eq!(formula[0].as_symbol(), Some("never"), "formula head must be 'never'");
}

#[test]
fn ast_to_sexpr_property_always_implies() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_impl".to_string(),
        directive: PropertyDirective::Assume,
        formula: PropertyFormula::AlwaysImplies {
            antecedent: Expr::Signal("a".to_string()),
            consequent: Expr::Signal("b".to_string()),
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    assert_eq!(prop[2].as_symbol(), Some("assume"), "directive must be 'assume'");
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("always-implies"),
        "formula head must be 'always-implies'"
    );
    assert_eq!(formula.len(), 3, "always-implies must have head + antecedent + consequent");
}

#[test]
fn ast_to_sexpr_property_never_implies() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_nimpl".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::NeverImplies {
            antecedent: Expr::Signal("x".to_string()),
            consequent: Expr::Signal("y".to_string()),
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("never-implies"),
        "formula head must be 'never-implies'"
    );
}

#[test]
fn ast_to_sexpr_property_eventually_within() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_even".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::EventuallyWithin {
            expr: Expr::Signal("done".to_string()),
            cycles: 10,
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("eventually-within"),
        "formula head must be 'eventually-within'"
    );
    assert_eq!(formula[2].as_integer(), Some(10), "cycles must be 10");
}

#[test]
fn ast_to_sexpr_property_always_followed_by() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_follow".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::AlwaysFollowedBy {
            trigger: Expr::Signal("req".to_string()),
            response: Expr::Signal("ack".to_string()),
            delay_cycles: 5,
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("always-followed-by"),
        "formula head must be 'always-followed-by'"
    );
    assert_eq!(formula.len(), 4, "always-followed-by must have head + trigger + response + delay");
    assert_eq!(formula[3].as_integer(), Some(5), "delay must be 5");
}

// =========================================================================
// 7. Pattern definitions
// =========================================================================

#[test]
fn ast_to_sexpr_pattern_def_with_params() {
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "monitor".to_string(),
        params: vec![
            PatternParam {
                name: "sensor".to_string(),
                kind: PatternParamKind::Signal {
                    kind: SignalKind::Input,
                    ty: SignalType::Unsigned(16),
                    annotations: default_annotations(),
                },
            },
            PatternParam {
                name: "threshold".to_string(),
                kind: PatternParamKind::Constant {
                    ty: SignalType::Unsigned(16),
                    annotations: default_annotations(),
                },
            },
            PatternParam { name: "handler".to_string(), kind: PatternParamKind::Pattern },
        ],
        body: ReflectBlock {
            raw_lines: vec!["guard g_${sensor} when ${sensor} > ${threshold} for 3;".to_string()],
        },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().unwrap();
    let patterns_section = items[1].as_list().unwrap();
    let pattern_def = patterns_section[1].as_list().expect("pattern-def must be a list");
    assert_eq!(pattern_def[0].as_symbol(), Some("pattern-def"), "head must be 'pattern-def'");
    assert_eq!(pattern_def[1].as_str_val(), Some("monitor"), "pattern name must be 'monitor'");

    // params section
    let params = pattern_def[2].as_list().expect("params must be a list");
    assert_eq!(params[0].as_symbol(), Some("params"), "params head must be 'params'");
    assert_eq!(params.len(), 4, "must have head + 3 params");

    // First param: signal
    let p0 = params[1].as_list().unwrap();
    assert_eq!(p0[1].as_str_val(), Some("sensor"), "first param name must be 'sensor'");
    assert_eq!(p0[2].as_symbol(), Some("signal"), "first param kind must be 'signal'");
    assert_eq!(p0[3].as_symbol(), Some("input"), "signal kind must be 'input'");

    // Second param: constant
    let p1 = params[2].as_list().unwrap();
    assert_eq!(p1[1].as_str_val(), Some("threshold"), "second param name must be 'threshold'");
    assert_eq!(p1[2].as_symbol(), Some("constant"), "second param kind must be 'constant'");

    // Third param: pattern
    let p2 = params[3].as_list().unwrap();
    assert_eq!(p2[1].as_str_val(), Some("handler"), "third param name must be 'handler'");
    assert_eq!(p2[2].as_symbol(), Some("pattern"), "third param kind must be 'pattern'");

    // Reflect body
    let reflect = pattern_def[3].as_list().expect("reflect must be a list");
    assert_eq!(reflect[0].as_symbol(), Some("reflect"), "reflect head must be 'reflect'");
    assert_eq!(reflect.len(), 2, "reflect must have head + 1 line");
    assert!(
        reflect[1].as_str_val().unwrap().contains("${sensor}"),
        "reflect line must contain template markers"
    );
}

// =========================================================================
// 8. Pattern calls and origins
// =========================================================================

#[test]
fn ast_to_sexpr_pattern_calls() {
    let mut program = empty_program();
    program.module.pattern_calls.push(PatternCall {
        pattern_name: "monitor".to_string(),
        arguments: vec![
            PatternArg::SignalRef("pressure".to_string()),
            PatternArg::ConstInt(100),
            PatternArg::ConstBool(true),
            PatternArg::PatternRef("alert".to_string()),
        ],
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let calls_section = module_list[6].as_list().unwrap();
    assert_eq!(calls_section[0].as_symbol(), Some("pattern-calls"), "head must be 'pattern-calls'");
    let call = calls_section[1].as_list().unwrap();
    assert_eq!(call[0].as_symbol(), Some("pattern-call"), "call head must be 'pattern-call'");
    assert_eq!(call[1].as_str_val(), Some("monitor"), "call pattern name must be 'monitor'");

    // Arguments
    let arg0 = call[2].as_list().unwrap();
    assert_eq!(arg0[0].as_symbol(), Some("signal-ref"), "arg0 head must be 'signal-ref'");
    assert_eq!(arg0[1].as_str_val(), Some("pressure"), "arg0 value must be 'pressure'");

    let arg1 = call[3].as_list().unwrap();
    assert_eq!(arg1[0].as_symbol(), Some("const-int"), "arg1 head must be 'const-int'");
    assert_eq!(arg1[1].as_integer(), Some(100), "arg1 value must be 100");

    let arg2 = call[4].as_list().unwrap();
    assert_eq!(arg2[0].as_symbol(), Some("const-bool"), "arg2 head must be 'const-bool'");
    assert_eq!(arg2[1].as_bool(), Some(true), "arg2 value must be true");

    let arg3 = call[5].as_list().unwrap();
    assert_eq!(arg3[0].as_symbol(), Some("pattern-ref"), "arg3 head must be 'pattern-ref'");
    assert_eq!(arg3[1].as_str_val(), Some("alert"), "arg3 value must be 'alert'");
}

#[test]
fn ast_to_sexpr_pattern_origins() {
    let mut program = empty_program();
    program.module.pattern_origins.push(PatternOrigin {
        pattern_name: "monitor".to_string(),
        call_args_summary: "pressure, 100, true, alert".to_string(),
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let origins_section = module_list[7].as_list().unwrap();
    assert_eq!(
        origins_section[0].as_symbol(),
        Some("pattern-origins"),
        "head must be 'pattern-origins'"
    );
    let origin = origins_section[1].as_list().unwrap();
    assert_eq!(
        origin[0].as_symbol(),
        Some("pattern-origin"),
        "origin head must be 'pattern-origin'"
    );
    assert_eq!(origin[1].as_str_val(), Some("monitor"), "origin pattern name must be 'monitor'");
    assert_eq!(
        origin[2].as_str_val(),
        Some("pressure, 100, true, alert"),
        "origin summary must match"
    );
}

// =========================================================================
// 9. S-Expr -> AST: full round-trip
// =========================================================================

#[test]
fn sexpr_to_ast_roundtrip_empty_program() {
    let program = empty_program();
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for empty program");
    assert_eq!(restored.module.name, "test_module", "module name must survive round-trip");
    assert!(restored.patterns.is_empty(), "patterns must be empty");
    assert!(restored.module.signals.is_empty(), "signals must be empty");
    assert!(restored.module.guards.is_empty(), "guards must be empty");
    assert!(restored.module.reflexes.is_empty(), "reflexes must be empty");
    assert!(restored.module.properties.is_empty(), "properties must be empty");
}

#[test]
fn sexpr_to_ast_roundtrip_signals() {
    let mut program = empty_program();
    program.module.signals.push(make_signal("clk", SignalKind::Input, SignalType::Bool));
    program.module.signals.push(make_signal("data", SignalKind::Output, SignalType::Unsigned(8)));
    program.module.signals.push(make_signal("acc", SignalKind::Internal, SignalType::Signed(16)));
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed");
    assert_eq!(restored.module.signals.len(), 3, "must have 3 signals");
    for i in 0..3 {
        assert_eq!(
            restored.module.signals[i].name, program.module.signals[i].name,
            "signal name mismatch at index {i}"
        );
        assert_eq!(
            restored.module.signals[i].kind, program.module.signals[i].kind,
            "signal kind mismatch at index {i}"
        );
        assert_eq!(
            restored.module.signals[i].ty.core, program.module.signals[i].ty.core,
            "signal type mismatch at index {i}"
        );
    }
}

#[test]
fn sexpr_to_ast_roundtrip_annotations() {
    let mut ann = default_annotations();
    ann.linearity = Linearity::Linear;
    ann.effect = EffectQualifier::Stateful;
    ann.refinement = Some(Refinement::Range { lo: 0, hi: 255 });
    ann.clock_domain = Some("sys_clk".to_string());
    ann.phantom_tag = Some("Voltage".to_string());

    let mut program = empty_program();
    program.module.signals.push(SignalDecl {
        name: "full_ann".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::new(SignalType::Unsigned(8), ann.clone()),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for annotated signal");
    let restored_ann = &restored.module.signals[0].ty.annotations;
    assert_eq!(restored_ann.linearity, Linearity::Linear, "linearity must round-trip");
    assert_eq!(restored_ann.effect, EffectQualifier::Stateful, "effect must round-trip");
    assert_eq!(
        restored_ann.refinement,
        Some(Refinement::Range { lo: 0, hi: 255 }),
        "refinement must round-trip"
    );
    assert_eq!(
        restored_ann.clock_domain,
        Some("sys_clk".to_string()),
        "clock_domain must round-trip"
    );
    assert_eq!(
        restored_ann.phantom_tag,
        Some("Voltage".to_string()),
        "phantom_tag must round-trip"
    );
}

#[test]
fn sexpr_to_ast_roundtrip_guards() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g1".to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("temp".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(100))),
        },
        cycles: 3,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for guards");
    assert_eq!(restored.module.guards.len(), 1, "must have 1 guard");
    let g = &restored.module.guards[0];
    assert_eq!(g.name, "g1", "guard name must be 'g1'");
    assert_eq!(g.cycles, 3, "guard cycles must be 3");
    // Verify the condition structure
    match &g.condition {
        Expr::Binary { op, left, right } => {
            assert_eq!(*op, BinaryOp::Gt, "condition op must be Gt");
            match left.as_ref() {
                Expr::Signal(name) => assert_eq!(name, "temp", "left must be signal 'temp'"),
                other => panic!("expected Signal, got {:?}", other),
            }
            match right.as_ref() {
                Expr::Literal(LiteralValue::Integer(n)) => {
                    assert_eq!(*n, 100, "right must be integer 100");
                }
                other => panic!("expected Integer literal, got {:?}", other),
            }
        }
        other => panic!("expected Binary expression, got {:?}", other),
    }
}

#[test]
fn sexpr_to_ast_roundtrip_reflexes() {
    let mut program = empty_program();
    program.module.reflexes.push(Reflex {
        name: "r1".to_string(),
        guard_names: vec!["g1".to_string()],
        assignments: vec![Assignment {
            target: "alarm".to_string(),
            value: Expr::Literal(LiteralValue::Bool(true)),
            span: None,
        }],
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for reflexes");
    assert_eq!(restored.module.reflexes.len(), 1, "must have 1 reflex");
    let r = &restored.module.reflexes[0];
    assert_eq!(r.name, "r1", "reflex name must be 'r1'");
    assert_eq!(r.guard_names, vec!["g1".to_string()], "guard_names must match");
    assert_eq!(r.assignments.len(), 1, "must have 1 assignment");
    assert_eq!(r.assignments[0].target, "alarm", "assignment target must be 'alarm'");
}

#[test]
fn sexpr_to_ast_roundtrip_properties_all_formulas() {
    let mut program = empty_program();
    let sig_a = Expr::Signal("a".to_string());
    let sig_b = Expr::Signal("b".to_string());
    program.module.properties.push(PropertyDecl {
        name: "p_always".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(sig_a.clone()),
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_never".to_string(),
        directive: PropertyDirective::Cover,
        formula: PropertyFormula::Never(sig_a.clone()),
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_ai".to_string(),
        directive: PropertyDirective::Assume,
        formula: PropertyFormula::AlwaysImplies {
            antecedent: sig_a.clone(),
            consequent: sig_b.clone(),
        },
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_ni".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::NeverImplies {
            antecedent: sig_a.clone(),
            consequent: sig_b.clone(),
        },
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_ew".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::EventuallyWithin { expr: sig_a.clone(), cycles: 10 },
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_afb".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::AlwaysFollowedBy {
            trigger: sig_a.clone(),
            response: sig_b.clone(),
            delay_cycles: 5,
        },
        origin: None,
        span: None,
    });

    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for all formula types");
    assert_eq!(restored.module.properties.len(), 6, "must have 6 properties");

    for i in 0..6 {
        assert_eq!(
            restored.module.properties[i].name, program.module.properties[i].name,
            "property name mismatch at index {i}"
        );
        assert_eq!(
            restored.module.properties[i].directive, program.module.properties[i].directive,
            "property directive mismatch at index {i}"
        );
    }

    // Verify specific formula types
    assert!(
        matches!(restored.module.properties[0].formula, PropertyFormula::Always(_)),
        "property 0 must be Always"
    );
    assert!(
        matches!(restored.module.properties[1].formula, PropertyFormula::Never(_)),
        "property 1 must be Never"
    );
    assert!(
        matches!(restored.module.properties[2].formula, PropertyFormula::AlwaysImplies { .. }),
        "property 2 must be AlwaysImplies"
    );
    assert!(
        matches!(restored.module.properties[3].formula, PropertyFormula::NeverImplies { .. }),
        "property 3 must be NeverImplies"
    );
    match &restored.module.properties[4].formula {
        PropertyFormula::EventuallyWithin { cycles, .. } => {
            assert_eq!(*cycles, 10, "EventuallyWithin cycles must be 10");
        }
        other => panic!("property 4 must be EventuallyWithin, got {:?}", other),
    }
    match &restored.module.properties[5].formula {
        PropertyFormula::AlwaysFollowedBy { delay_cycles, .. } => {
            assert_eq!(*delay_cycles, 5, "AlwaysFollowedBy delay must be 5");
        }
        other => panic!("property 5 must be AlwaysFollowedBy, got {:?}", other),
    }
}

#[test]
fn sexpr_to_ast_roundtrip_pattern_def() {
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "watchdog".to_string(),
        params: vec![
            PatternParam {
                name: "sig".to_string(),
                kind: PatternParamKind::Signal {
                    kind: SignalKind::Output,
                    ty: SignalType::Bool,
                    annotations: default_annotations(),
                },
            },
            PatternParam {
                name: "limit".to_string(),
                kind: PatternParamKind::Constant {
                    ty: SignalType::Unsigned(32),
                    annotations: default_annotations(),
                },
            },
        ],
        body: ReflectBlock { raw_lines: vec!["line1".to_string(), "line2".to_string()] },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for pattern defs");
    assert_eq!(restored.patterns.len(), 1, "must have 1 pattern");
    let p = &restored.patterns[0];
    assert_eq!(p.name, "watchdog", "pattern name must be 'watchdog'");
    assert_eq!(p.params.len(), 2, "must have 2 params");
    assert_eq!(p.params[0].name, "sig", "first param name must be 'sig'");
    assert_eq!(p.params[1].name, "limit", "second param name must be 'limit'");
    assert_eq!(p.body.raw_lines.len(), 2, "reflect body must have 2 lines");
    assert_eq!(p.body.raw_lines[0], "line1", "first reflect line must be 'line1'");
}

#[test]
fn sexpr_to_ast_roundtrip_pattern_calls() {
    let mut program = empty_program();
    program.module.pattern_calls.push(PatternCall {
        pattern_name: "test_pattern".to_string(),
        arguments: vec![
            PatternArg::SignalRef("sig1".to_string()),
            PatternArg::ConstInt(42),
            PatternArg::ConstBool(false),
            PatternArg::PatternRef("other_pat".to_string()),
        ],
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for pattern calls");
    assert_eq!(restored.module.pattern_calls.len(), 1, "must have 1 pattern call");
    let c = &restored.module.pattern_calls[0];
    assert_eq!(c.pattern_name, "test_pattern", "pattern_name must be 'test_pattern'");
    assert_eq!(c.arguments.len(), 4, "must have 4 arguments");
    assert_eq!(
        c.arguments[0],
        PatternArg::SignalRef("sig1".to_string()),
        "arg 0 must be SignalRef"
    );
    assert_eq!(c.arguments[1], PatternArg::ConstInt(42), "arg 1 must be ConstInt(42)");
    assert_eq!(c.arguments[2], PatternArg::ConstBool(false), "arg 2 must be ConstBool(false)");
    assert_eq!(
        c.arguments[3],
        PatternArg::PatternRef("other_pat".to_string()),
        "arg 3 must be PatternRef"
    );
}

#[test]
fn sexpr_to_ast_roundtrip_pattern_origins() {
    let mut program = empty_program();
    program.module.pattern_origins.push(PatternOrigin {
        pattern_name: "monitor_sensor".to_string(),
        call_args_summary: "airway_pressure, 10, 200, 500, alarm".to_string(),
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for pattern origins");
    assert_eq!(restored.module.pattern_origins.len(), 1, "must have 1 pattern origin");
    let o = &restored.module.pattern_origins[0];
    assert_eq!(o.pattern_name, "monitor_sensor", "pattern_name must be 'monitor_sensor'");
    assert_eq!(
        o.call_args_summary, "airway_pressure, 10, 200, 500, alarm",
        "call_args_summary must match"
    );
}

// =========================================================================
// 10. Expression round-trip: all variants
// =========================================================================

/// Helper: put an expression in a guard, round-trip, extract the condition.
fn roundtrip_expr(expr: Expr) -> Expr {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_test".to_string(),
        condition: expr,
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("expression round-trip must succeed");
    restored.module.guards[0].condition.clone()
}

#[test]
fn roundtrip_expr_bool_true() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Bool(true)));
    assert_eq!(result, Expr::Literal(LiteralValue::Bool(true)), "bool true must round-trip");
}

#[test]
fn roundtrip_expr_bool_false() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Bool(false)));
    assert_eq!(result, Expr::Literal(LiteralValue::Bool(false)), "bool false must round-trip");
}

#[test]
fn roundtrip_expr_integer_zero() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Integer(0)));
    assert_eq!(result, Expr::Literal(LiteralValue::Integer(0)), "integer 0 must round-trip");
}

#[test]
fn roundtrip_expr_integer_large() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Integer(0xFFFF_FFFF)));
    assert_eq!(
        result,
        Expr::Literal(LiteralValue::Integer(0xFFFF_FFFF)),
        "large integer must round-trip"
    );
}

#[test]
fn roundtrip_expr_signal_ref() {
    let result = roundtrip_expr(Expr::Signal("sensor_value".to_string()));
    assert_eq!(result, Expr::Signal("sensor_value".to_string()), "signal ref must round-trip");
}

#[test]
fn roundtrip_expr_prev() {
    let result = roundtrip_expr(Expr::Prev { signal: "temp".to_string(), delay: 5 });
    assert_eq!(
        result,
        Expr::Prev { signal: "temp".to_string(), delay: 5 },
        "prev expression must round-trip"
    );
}

#[test]
fn roundtrip_expr_unary_not() {
    let result = roundtrip_expr(Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Literal(LiteralValue::Bool(true))),
    });
    match &result {
        Expr::Unary { op, operand } => {
            assert_eq!(*op, UnaryOp::Not, "unary op must be Not");
            assert_eq!(
                operand.as_ref(),
                &Expr::Literal(LiteralValue::Bool(true)),
                "operand must be bool true"
            );
        }
        other => panic!("expected Unary, got {:?}", other),
    }
}

#[test]
fn roundtrip_expr_unary_negate() {
    let result = roundtrip_expr(Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Literal(LiteralValue::Integer(42))),
    });
    match &result {
        Expr::Unary { op, operand } => {
            assert_eq!(*op, UnaryOp::Negate, "unary op must be Negate");
            assert_eq!(
                operand.as_ref(),
                &Expr::Literal(LiteralValue::Integer(42)),
                "operand must be integer 42"
            );
        }
        other => panic!("expected Unary, got {:?}", other),
    }
}

#[test]
fn roundtrip_expr_all_binary_ops() {
    let ops: [BinaryOp; 14] = [
        BinaryOp::And,
        BinaryOp::Or,
        BinaryOp::Xor,
        BinaryOp::Lt,
        BinaryOp::Le,
        BinaryOp::Gt,
        BinaryOp::Ge,
        BinaryOp::Eq,
        BinaryOp::Ne,
        BinaryOp::Add,
        BinaryOp::Sub,
        BinaryOp::Mul,
        BinaryOp::Shl,
        BinaryOp::Shr,
    ];
    for i in 0..14 {
        let op = ops[i];
        let expr = Expr::Binary {
            op,
            left: Box::new(Expr::Literal(LiteralValue::Integer(10))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(20))),
        };
        let result = roundtrip_expr(expr);
        match &result {
            Expr::Binary { op: result_op, left, right } => {
                assert_eq!(*result_op, op, "binary op must round-trip at index {i}");
                assert_eq!(
                    left.as_ref(),
                    &Expr::Literal(LiteralValue::Integer(10)),
                    "left operand must be 10 at index {i}"
                );
                assert_eq!(
                    right.as_ref(),
                    &Expr::Literal(LiteralValue::Integer(20)),
                    "right operand must be 20 at index {i}"
                );
            }
            other => panic!("expected Binary at index {i}, got {:?}", other),
        }
    }
}

#[test]
fn roundtrip_expr_nested_binary() {
    // (not (a and (b or c)))
    let inner = Expr::Binary {
        op: BinaryOp::Or,
        left: Box::new(Expr::Signal("b".to_string())),
        right: Box::new(Expr::Signal("c".to_string())),
    };
    let mid = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("a".to_string())),
        right: Box::new(inner),
    };
    let outer = Expr::Unary { op: UnaryOp::Not, operand: Box::new(mid) };
    let result = roundtrip_expr(outer.clone());
    assert_eq!(result, outer, "deeply nested expression must round-trip exactly");
}

// =========================================================================
// 11. Error paths in sexpr_to_ast
// =========================================================================

#[test]
fn sexpr_to_ast_rejects_non_list() {
    let sexpr = SExpr::sym("not-a-program");
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject non-list input");
}

#[test]
fn sexpr_to_ast_rejects_empty_list() {
    let sexpr = SExpr::list(Vec::new());
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject empty list");
}

#[test]
fn sexpr_to_ast_rejects_wrong_head() {
    let sexpr = SExpr::list(vec![
        SExpr::sym("not-program"),
        SExpr::list(vec![SExpr::sym("patterns")]),
        SExpr::list(vec![SExpr::sym("module"), SExpr::str_val("m")]),
    ]);
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject wrong head symbol");
}

#[test]
fn sexpr_to_ast_rejects_too_short_program() {
    let sexpr = SExpr::list(vec![SExpr::sym("program"), SExpr::list(vec![SExpr::sym("patterns")])]);
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject program with missing module");
}

// =========================================================================
// 12. Bounded iteration: multiple signals (NASA Power-of-10)
// =========================================================================

#[test]
fn bounded_multiple_signals_roundtrip() {
    let mut program = empty_program();
    let count = 32;
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        program.module.signals.push(make_signal(
            &format!("sig_{i}"),
            SignalKind::Input,
            SignalType::Unsigned(8),
        ));
    }
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("must succeed for multiple signals");
    assert_eq!(
        restored.module.signals.len(),
        count,
        "signal count must be preserved after round-trip"
    );
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        assert_eq!(
            restored.module.signals[i].name,
            format!("sig_{i}"),
            "signal name must match at index {i}"
        );
    }
}

#[test]
fn bounded_multiple_guards_roundtrip() {
    let mut program = empty_program();
    let count = 16;
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        program.module.guards.push(Guard {
            name: format!("g_{i}"),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles: (i as u64) + 1,
            origin: None,
            span: None,
        });
    }
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("must succeed for multiple guards");
    assert_eq!(restored.module.guards.len(), count, "guard count must be preserved");
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        assert_eq!(
            restored.module.guards[i].name,
            format!("g_{i}"),
            "guard name must match at index {i}"
        );
        assert_eq!(
            restored.module.guards[i].cycles,
            (i as u64) + 1,
            "guard cycles must match at index {i}"
        );
    }
}

// =========================================================================
// 13. Full program round-trip (comprehensive)
// =========================================================================

#[test]
fn full_program_roundtrip() {
    let program = MirrProgram {
        patterns: vec![PatternDef {
            name: "sensor_monitor".to_string(),
            params: vec![
                PatternParam {
                    name: "input_sig".to_string(),
                    kind: PatternParamKind::Signal {
                        kind: SignalKind::Input,
                        ty: SignalType::Unsigned(16),
                        annotations: default_annotations(),
                    },
                },
                PatternParam {
                    name: "thresh".to_string(),
                    kind: PatternParamKind::Constant {
                        ty: SignalType::Unsigned(16),
                        annotations: default_annotations(),
                    },
                },
            ],
            body: ReflectBlock {
                raw_lines: vec![
                    "guard g_${input_sig} when ${input_sig} > ${thresh} for 3;".to_string()
                ],
            },
            span: None,
        }],
        module: Module {
            name: "ventilator_ctrl".to_string(),
            signals: vec![
                make_signal("pressure", SignalKind::Input, SignalType::Unsigned(16)),
                make_signal("alarm", SignalKind::Output, SignalType::Bool),
                make_signal("count", SignalKind::Internal, SignalType::Unsigned(8)),
            ],
            guards: vec![Guard {
                name: "g_high_pressure".to_string(),
                condition: Expr::Binary {
                    op: BinaryOp::Gt,
                    left: Box::new(Expr::Signal("pressure".to_string())),
                    right: Box::new(Expr::Literal(LiteralValue::Integer(500))),
                },
                cycles: 3,
                origin: None,
                span: None,
            }],
            reflexes: vec![Reflex {
                name: "r_alarm".to_string(),
                guard_names: vec!["g_high_pressure".to_string()],
                assignments: vec![
                    Assignment {
                        target: "alarm".to_string(),
                        value: Expr::Literal(LiteralValue::Bool(true)),
                        span: None,
                    },
                    Assignment {
                        target: "count".to_string(),
                        value: Expr::Binary {
                            op: BinaryOp::Add,
                            left: Box::new(Expr::Signal("count".to_string())),
                            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
                        },
                        span: None,
                    },
                ],
                origin: None,
                span: None,
            }],
            properties: vec![PropertyDecl {
                name: "p_alarm_response".to_string(),
                directive: PropertyDirective::Assert,
                formula: PropertyFormula::AlwaysFollowedBy {
                    trigger: Expr::Binary {
                        op: BinaryOp::Gt,
                        left: Box::new(Expr::Signal("pressure".to_string())),
                        right: Box::new(Expr::Literal(LiteralValue::Integer(500))),
                    },
                    response: Expr::Signal("alarm".to_string()),
                    delay_cycles: 5,
                },
                origin: None,
                span: None,
            }],
            pattern_calls: vec![PatternCall {
                pattern_name: "sensor_monitor".to_string(),
                arguments: vec![
                    PatternArg::SignalRef("pressure".to_string()),
                    PatternArg::ConstInt(500),
                ],
                span: None,
            }],
            pattern_origins: vec![PatternOrigin {
                pattern_name: "sensor_monitor".to_string(),
                call_args_summary: "pressure, 500".to_string(),
            }],
            span: None,
        },
    };

    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("full program round-trip must succeed");

    // Verify all sections survived
    assert_eq!(restored.patterns.len(), 1, "must have 1 pattern");
    assert_eq!(restored.patterns[0].name, "sensor_monitor", "pattern name must match");
    assert_eq!(restored.module.name, "ventilator_ctrl", "module name must match");
    assert_eq!(restored.module.signals.len(), 3, "must have 3 signals");
    assert_eq!(restored.module.guards.len(), 1, "must have 1 guard");
    assert_eq!(restored.module.reflexes.len(), 1, "must have 1 reflex");
    assert_eq!(restored.module.properties.len(), 1, "must have 1 property");
    assert_eq!(restored.module.pattern_calls.len(), 1, "must have 1 pattern call");
    assert_eq!(restored.module.pattern_origins.len(), 1, "must have 1 pattern origin");

    // Spot-check deep structure
    assert_eq!(restored.module.guards[0].cycles, 3, "guard cycles must be 3");
    assert_eq!(restored.module.reflexes[0].assignments.len(), 2, "reflex must have 2 assignments");
    assert_eq!(
        restored.module.pattern_calls[0].arguments.len(),
        2,
        "pattern call must have 2 arguments"
    );
}

// =========================================================================
// 14. Annotation combinations
// =========================================================================

#[test]
fn roundtrip_annotations_predicate_refinement() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Predicate("value != 0".to_string()));
    let mut program = empty_program();
    program.module.signals.push(SignalDecl {
        name: "nonzero".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::new(SignalType::Unsigned(8), ann),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("predicate annotation round-trip must succeed");
    let restored_ann = &restored.module.signals[0].ty.annotations;
    match &restored_ann.refinement {
        Some(Refinement::Predicate(expr)) => {
            assert_eq!(expr, "value != 0", "predicate must match");
        }
        other => panic!("expected Predicate refinement, got {:?}", other),
    }
}

#[test]
fn roundtrip_annotations_pure_effect() {
    let mut ann = default_annotations();
    ann.effect = EffectQualifier::Pure;
    let mut program = empty_program();
    program.module.signals.push(SignalDecl {
        name: "combinational".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::new(SignalType::Bool, ann),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("pure effect annotation round-trip must succeed");
    assert_eq!(
        restored.module.signals[0].ty.annotations.effect,
        EffectQualifier::Pure,
        "pure effect must round-trip"
    );
}

// =========================================================================
// 15. Pattern param with annotations
// =========================================================================

#[test]
fn roundtrip_pattern_param_signal_with_annotations() {
    let mut ann = default_annotations();
    ann.linearity = Linearity::Linear;
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "annotated_pat".to_string(),
        params: vec![PatternParam {
            name: "s".to_string(),
            kind: PatternParamKind::Signal {
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(8),
                annotations: ann,
            },
        }],
        body: ReflectBlock { raw_lines: vec!["body".to_string()] },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("annotated pattern param round-trip must succeed");
    match &restored.patterns[0].params[0].kind {
        PatternParamKind::Signal { annotations, .. } => {
            assert_eq!(
                annotations.linearity,
                Linearity::Linear,
                "linearity must survive pattern param round-trip"
            );
        }
        other => panic!("expected Signal param kind, got {:?}", other),
    }
}

#[test]
fn roundtrip_pattern_param_constant_with_annotations() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Range { lo: 1, hi: 100 });
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "const_pat".to_string(),
        params: vec![PatternParam {
            name: "n".to_string(),
            kind: PatternParamKind::Constant { ty: SignalType::Unsigned(16), annotations: ann },
        }],
        body: ReflectBlock { raw_lines: vec!["body".to_string()] },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("annotated constant param round-trip must succeed");
    match &restored.patterns[0].params[0].kind {
        PatternParamKind::Constant { annotations, .. } => {
            assert_eq!(
                annotations.refinement,
                Some(Refinement::Range { lo: 1, hi: 100 }),
                "refinement must survive constant param round-trip"
            );
        }
        other => panic!("expected Constant param kind, got {:?}", other),
    }
}

// =========================================================================
// 16. Edge cases
// =========================================================================

#[test]
fn roundtrip_empty_guard_names_in_reflex() {
    // A reflex with no guard names (unusual but representable)
    let mut program = empty_program();
    program.module.reflexes.push(Reflex {
        name: "r_empty".to_string(),
        guard_names: Vec::new(),
        assignments: vec![Assignment {
            target: "out".to_string(),
            value: Expr::Literal(LiteralValue::Bool(false)),
            span: None,
        }],
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("reflex with empty guard names must round-trip");
    assert!(restored.module.reflexes[0].guard_names.is_empty(), "guard_names must remain empty");
}

#[test]
fn roundtrip_pattern_no_params() {
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "noop".to_string(),
        params: Vec::new(),
        body: ReflectBlock { raw_lines: Vec::new() },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("pattern with no params must round-trip");
    assert!(restored.patterns[0].params.is_empty(), "params must remain empty");
    assert!(restored.patterns[0].body.raw_lines.is_empty(), "body must remain empty");
}

#[test]
fn roundtrip_multiple_patterns() {
    let mut program = empty_program();
    let count = 4;
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration at index {i}");
        program.patterns.push(PatternDef {
            name: format!("pat_{i}"),
            params: vec![PatternParam { name: "p".to_string(), kind: PatternParamKind::Pattern }],
            body: ReflectBlock { raw_lines: vec![format!("line_{i}")] },
            span: None,
        });
    }
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("multiple patterns must round-trip");
    assert_eq!(restored.patterns.len(), count, "pattern count must be preserved");
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration at index {i}");
        assert_eq!(
            restored.patterns[i].name,
            format!("pat_{i}"),
            "pattern name must match at index {i}"
        );
    }
}

#[test]
fn roundtrip_prev_delay_one() {
    let result = roundtrip_expr(Expr::Prev { signal: "x".to_string(), delay: 1 });
    assert_eq!(
        result,
        Expr::Prev { signal: "x".to_string(), delay: 1 },
        "prev with delay=1 must round-trip"
    );
}

#[test]
fn roundtrip_zero_width_unsigned() {
    // SignalType::Unsigned(0) is unusual but representable
    let mut program = empty_program();
    program.module.signals.push(make_signal("zero_w", SignalKind::Input, SignalType::Unsigned(0)));
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("zero-width unsigned must round-trip");
    assert_eq!(
        restored.module.signals[0].ty.core,
        SignalType::Unsigned(0),
        "zero-width must survive"
    );
}

#[test]
fn roundtrip_pattern_call_no_args() {
    let mut program = empty_program();
    program.module.pattern_calls.push(PatternCall {
        pattern_name: "no_arg_pat".to_string(),
        arguments: Vec::new(),
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("pattern call with no args must round-trip");
    assert_eq!(restored.module.pattern_calls[0].pattern_name, "no_arg_pat", "name must match");
    assert!(restored.module.pattern_calls[0].arguments.is_empty(), "args must remain empty");
}
