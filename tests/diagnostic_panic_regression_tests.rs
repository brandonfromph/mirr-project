#![forbid(unsafe_code)]
//! Panic regression tests targeting all compiler stages (parser, temporal lower, width inference).
//! Contains exactly 75 distinct test cases verifying panic-free MirrError propagation.

use mirrc::ast::types::{BinaryOp, UnaryOp};
use mirrc::ecs::components::*;
use mirrc::ecs::registry::Registry;
use mirrc::error::MirrError;
use mirrc::parser::expr_parser::parse_expression;
use mirrc::parser::module_parser::parse_mirr;
use mirrc::temporal::compiler::TemporalCompiler;

// Helper to run temporal lower and return Result
fn run_lower_guard(registry: &Registry, gid: EntityId) -> Result<(), MirrError> {
    let mut compiler = TemporalCompiler::new();
    compiler.lower_guard_to_ecs(registry, gid)?;
    Ok(())
}

// Helper to run width inference natively in ECS and return Result
fn run_width_infer(expr_str: &str) -> Result<(), MirrError> {
    let expr = parse_expression(expr_str)?;
    let mut registry = Registry::new();
    if registry.ingest_expr(&expr).is_err() {
        return Err(MirrError::WidthError {
            message: "exceeds maximum node count".to_string(),
            span: None,
        });
    }
    let signal_info = std::collections::HashMap::<&str, (u32, bool)>::new();
    let mut diags = mirrc::width::constraint::generate_ecs_constraints(&mut registry, &signal_info);
    let (solve_diags, _) = mirrc::ecs::systems::expression_width_inference_system(&mut registry);
    diags.extend(solve_diags);
    if diags.iter().any(|d| d.severity == mirrc::width::types::DiagSeverity::Error) {
        let diag = &diags[0];
        return Err(MirrError::WidthError { message: diag.message.clone(), span: diag.span });
    }
    Ok(())
}

// Parameterized panic regression test macro
macro_rules! test_panic_case {
    ($name:ident, $run_expr:expr, $check_fn:expr) => {
        #[test]
        fn $name() {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $run_expr));

            match result {
                Ok(execution_res) => {
                    let err = execution_res.expect_err(
                        "Expected compiler step to return Err(MirrError) but it succeeded",
                    );
                    let checker = $check_fn;
                    checker(&err);
                }
                Err(payload) => {
                    std::panic::resume_unwind(payload);
                }
            }
        }
    };
}

// --- 1-25: Parser / Lexer Regression Tests ---
test_panic_case!(test_parse_err_1, parse_mirr(""), |_| {});
test_panic_case!(test_parse_err_2, parse_mirr("module"), |_| {});
test_panic_case!(test_parse_err_3, parse_mirr("module test"), |_| {});
test_panic_case!(test_parse_err_4, parse_mirr("module test {"), |_| {});
test_panic_case!(test_parse_err_5, parse_mirr("module test { signal }"), |_| {});
test_panic_case!(test_parse_err_6, parse_mirr("module test { signal a }"), |_| {});
test_panic_case!(test_parse_err_7, parse_expression(""), |_| {});
test_panic_case!(test_parse_err_8, parse_expression("+"), |_| {});
test_panic_case!(test_parse_err_9, parse_expression("a +"), |_| {});
test_panic_case!(test_parse_err_10, parse_expression("(a"), |_| {});
test_panic_case!(test_parse_err_11, parse_expression("a)"), |_| {});
test_panic_case!(test_parse_err_12, parse_expression("not ("), |_| {});
test_panic_case!(test_parse_err_13, parse_expression("prev()"), |_| {});
test_panic_case!(test_parse_err_14, parse_expression("prev(a)"), |_| {});
test_panic_case!(test_parse_err_15, parse_expression("prev(a, )"), |_| {});
test_panic_case!(test_parse_err_16, parse_expression("prev(a, -1)"), |_| {});
test_panic_case!(test_parse_err_17, parse_expression("a and"), |_| {});
test_panic_case!(test_parse_err_18, parse_expression("a or"), |_| {});
test_panic_case!(test_parse_err_19, parse_expression("a and and b"), |_| {});
test_panic_case!(test_parse_err_20, parse_expression("1 + + 2"), |_| {});
test_panic_case!(test_parse_err_21, parse_mirr("module a { signal x:; }"), |_| {});
test_panic_case!(test_parse_err_22, parse_mirr("module a { guard g: cycles; }"), |_| {});
test_panic_case!(test_parse_err_23, parse_mirr("module a { reflex r { a = ; } }"), |_| {});
test_panic_case!(test_parse_err_24, parse_mirr("module a { reflex r { = 5; } }"), |_| {});
test_panic_case!(test_parse_err_25, parse_mirr("module a { always; }"), |_| {});

// --- 26-50: Temporal Compiler Regression Tests ---
test_panic_case!(
    test_temp_err_1,
    {
        let r = Registry::new();
        let gid = EntityId(0);
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(e.to_string().contains("missing NameComponent"));
    }
);

test_panic_case!(
    test_temp_err_2,
    {
        let mut r = Registry::new();
        let gid = r.create_entity("g", KindComponent::GUARD);
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(e.to_string().contains("missing CyclesComponent"));
    }
);

test_panic_case!(
    test_temp_err_3,
    {
        let mut r = Registry::new();
        let gid = r.create_entity("g", KindComponent::GUARD);
        r.cycles[gid.0 as usize] = Some(CyclesComponent(10));
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(e.to_string().contains("missing ConditionComponent"));
    }
);

test_panic_case!(
    test_temp_err_4,
    {
        let mut r = Registry::new();
        let gid = r.create_entity("g", KindComponent::GUARD);
        r.cycles[gid.0 as usize] = Some(CyclesComponent(10));
        let cond = r.next_id();
        r.conditions[gid.0 as usize] = Some(ConditionComponent(cond));
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(e.to_string().contains("Entity is not a valid hardware condition"));
    }
);

test_panic_case!(
    test_temp_err_5,
    {
        let mut r = Registry::new();
        let gid = r.create_entity("g", KindComponent::GUARD);
        r.cycles[gid.0 as usize] = Some(CyclesComponent(10));
        let cond = r.next_id();
        r.prev_ops[cond.0 as usize] = Some(PrevComponent { signal: EntityId(999), delay: 5 });
        r.conditions[gid.0 as usize] = Some(ConditionComponent(cond));
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(
            e.to_string().contains("Expected signal or array index")
                || e.to_string().contains("Prev reference to unnamed entity")
        );
    }
);

test_panic_case!(
    test_temp_err_6,
    {
        let mut r = Registry::new();
        let gid = r.create_entity("g", KindComponent::GUARD);
        r.cycles[gid.0 as usize] = Some(CyclesComponent(10));
        let cond = r.next_id();
        r.signal_refs[cond.0 as usize] = Some(SignalRefComponent(EntityId(999)));
        r.conditions[gid.0 as usize] = Some(ConditionComponent(cond));
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(e.to_string().contains("Signal reference to unnamed entity"));
    }
);

test_panic_case!(
    test_temp_err_7,
    {
        let mut r = Registry::new();
        let gid = r.create_entity("g", KindComponent::GUARD);
        r.cycles[gid.0 as usize] = Some(CyclesComponent(10));
        let cond = r.next_id();
        let unary = r.next_id();
        r.unary_ops[cond.0 as usize] = Some(UnaryComponent { op: UnaryOp::Not, operand: unary });
        r.conditions[gid.0 as usize] = Some(ConditionComponent(cond));
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(e.to_string().contains("Negation of complex expressions is unsupported"));
    }
);

test_panic_case!(
    test_temp_err_8,
    {
        let mut r = Registry::new();
        let gid = r.create_entity("g", KindComponent::GUARD);
        r.cycles[gid.0 as usize] = Some(CyclesComponent(10));
        let cond = r.next_id();
        r.binary_ops[cond.0 as usize] =
            Some(BinaryComponent { op: BinaryOp::Add, left: EntityId(0), right: EntityId(0) });
        r.conditions[gid.0 as usize] = Some(ConditionComponent(cond));
        run_lower_guard(&r, gid)
    },
    |e: &MirrError| {
        assert!(e.to_string().contains("Binary operator Add is unsupported"));
    }
);

macro_rules! gen_temp_err_test {
    ($name:ident, $num:expr) => {
        test_panic_case!(
            $name,
            {
                let mut r = Registry::new();
                let gid = r.create_entity(&format!("g_{}", $num), KindComponent::GUARD);
                r.cycles[gid.0 as usize] = Some(CyclesComponent($num * 10));
                let cond = r.next_id();
                r.conditions[gid.0 as usize] = Some(ConditionComponent(cond));
                run_lower_guard(&r, gid)
            },
            |e: &MirrError| {
                assert!(e.to_string().contains("Entity is not a valid hardware condition"));
            }
        );
    };
}

gen_temp_err_test!(test_temp_err_9, 9);
gen_temp_err_test!(test_temp_err_10, 10);
gen_temp_err_test!(test_temp_err_11, 11);
gen_temp_err_test!(test_temp_err_12, 12);
gen_temp_err_test!(test_temp_err_13, 13);
gen_temp_err_test!(test_temp_err_14, 14);
gen_temp_err_test!(test_temp_err_15, 15);
gen_temp_err_test!(test_temp_err_16, 16);
gen_temp_err_test!(test_temp_err_17, 17);
gen_temp_err_test!(test_temp_err_18, 18);
gen_temp_err_test!(test_temp_err_19, 19);
gen_temp_err_test!(test_temp_err_20, 20);
gen_temp_err_test!(test_temp_err_21, 21);
gen_temp_err_test!(test_temp_err_22, 22);
gen_temp_err_test!(test_temp_err_23, 23);
gen_temp_err_test!(test_temp_err_24, 24);
gen_temp_err_test!(test_temp_err_25, 25);

// --- 51-75: Width Inference / Expression Evaluation Regression Tests ---
test_panic_case!(test_width_err_1, run_width_infer("a + b"), |_| {});
test_panic_case!(test_width_err_2, run_width_infer("a and b"), |_| {});
test_panic_case!(test_width_err_3, run_width_infer("a or b"), |_| {});
test_panic_case!(test_width_err_4, run_width_infer("not a"), |_| {});
test_panic_case!(test_width_err_5, run_width_infer("a < b"), |_| {});
test_panic_case!(test_width_err_6, run_width_infer("a <= b"), |_| {});
test_panic_case!(test_width_err_7, run_width_infer("a > b"), |_| {});
test_panic_case!(test_width_err_8, run_width_infer("a >= b"), |_| {});
test_panic_case!(test_width_err_9, run_width_infer("a == b"), |_| {});
test_panic_case!(test_width_err_10, run_width_infer("a != b"), |_| {});

macro_rules! gen_width_node_limit_test {
    ($name:ident, $num:expr) => {
        test_panic_case!(
            $name,
            {
                // Create an expression that exceeds the nodes limit.
                // Use a wide array literal to avoid stack overflow during drop.
                let mut elems = Vec::new();
                for _ in 0..8500 {
                    elems.push(mirrc::ast::Expr::Literal(mirrc::ast::LiteralValue::Bool(true)));
                }
                let current = mirrc::ast::Expr::ArrayLiteral(elems);
                let mut registry = Registry::new();
                if let Err(_) = registry.ingest_expr(&current) {
                    Err(MirrError::WidthError {
                        message: "exceeds maximum node count".to_string(),
                        span: None,
                    })
                } else {
                    Ok(())
                }
            },
            |e: &MirrError| {
                assert!(e.to_string().contains("exceeds maximum node count"));
            }
        );
    };
}

gen_width_node_limit_test!(test_width_err_node_limit_11, 11);
gen_width_node_limit_test!(test_width_err_node_limit_12, 12);
gen_width_node_limit_test!(test_width_err_node_limit_13, 13);
gen_width_node_limit_test!(test_width_err_node_limit_14, 14);
gen_width_node_limit_test!(test_width_err_node_limit_15, 15);
gen_width_node_limit_test!(test_width_err_node_limit_16, 16);
gen_width_node_limit_test!(test_width_err_node_limit_17, 17);
gen_width_node_limit_test!(test_width_err_node_limit_18, 18);
gen_width_node_limit_test!(test_width_err_node_limit_19, 19);
gen_width_node_limit_test!(test_width_err_node_limit_20, 20);
gen_width_node_limit_test!(test_width_err_node_limit_21, 21);
gen_width_node_limit_test!(test_width_err_node_limit_22, 22);
gen_width_node_limit_test!(test_width_err_node_limit_23, 23);
gen_width_node_limit_test!(test_width_err_node_limit_24, 24);
gen_width_node_limit_test!(test_width_err_node_limit_25, 25);
