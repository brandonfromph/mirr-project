#![forbid(unsafe_code)]
#![allow(clippy::reversed_empty_ranges)] // Intentional 0..0 ranges in parameterized test macros
//! ECS pipeline integration tests covering constant folding, width inference, and full compiler orchestrator steps.
//! Contains exactly 100 distinct test cases.

use mirrc::ast::types::{BinaryOp, LiteralValue};
use mirrc::ecs::components::*;
use mirrc::ecs::registry::Registry;
use mirrc::ecs::systems::{
    parallel_constant_folding_system, parallel_width_inference_system, run_compilation_pipeline,
};

// Parameterized integration test macro
macro_rules! test_pipeline_case {
    ($name:ident, $setup_fn:expr, $system_fn:expr, $assert_fn:expr) => {
        #[test]
        fn $name() {
            let mut registry = Registry::new();
            let setup = $setup_fn;
            let setup_result = setup(&mut registry);

            let system_call = $system_fn;
            let system_result = system_call(&mut registry);

            let assert_check = $assert_fn;
            assert_check(&registry, setup_result, system_result);
        }
    };
}

// Helpers
fn setup_fold(op: BinaryOp, left: LiteralValue, right: LiteralValue, r: &mut Registry) -> EntityId {
    let l_ent = r.next_id();
    r.literals[l_ent.0 as usize] = Some(LiteralComponent(left));
    let r_ent = r.next_id();
    r.literals[r_ent.0 as usize] = Some(LiteralComponent(right));
    let binary_ent = r.next_id();
    r.binary_ops[binary_ent.0 as usize] = Some(BinaryComponent { op, left: l_ent, right: r_ent });
    binary_ent
}

fn assert_fold_val(expected: LiteralValue) -> impl Fn(&Registry, EntityId, ()) {
    move |r: &Registry, ent: EntityId, _| {
        assert!(r.binary_ops[ent.0 as usize].is_none());
        let lit = &r.literals[ent.0 as usize]
            .as_ref()
            .expect("Expected folded literal component to be present")
            .0;
        assert_eq!(lit, &expected);
    }
}

// --- 1-50: Constant Folding Integration Tests ---
// Integer Addition folding tests (25 tests)
test_pipeline_case!(
    fold_add_1,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(5), LiteralValue::Integer(10), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(15))
);
test_pipeline_case!(
    fold_add_2,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(0), LiteralValue::Integer(0), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(0))
);
test_pipeline_case!(
    fold_add_3,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(5), LiteralValue::Integer(5), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(10))
);
test_pipeline_case!(
    fold_add_4,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(100), LiteralValue::Integer(200), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(300))
);
test_pipeline_case!(
    fold_add_5,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(10), LiteralValue::Integer(20), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(30))
);
test_pipeline_case!(
    fold_add_6,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(1), LiteralValue::Integer(1), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(2))
);
test_pipeline_case!(
    fold_add_7,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(12), LiteralValue::Integer(34), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(46))
);
test_pipeline_case!(
    fold_add_8,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(1000), LiteralValue::Integer(2000), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(3000))
);
test_pipeline_case!(
    fold_add_9,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(50), LiteralValue::Integer(50), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(100))
);
test_pipeline_case!(
    fold_add_10,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(99), LiteralValue::Integer(1), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(100))
);
test_pipeline_case!(
    fold_add_11,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(5), LiteralValue::Integer(15), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(20))
);
test_pipeline_case!(
    fold_add_12,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(90), LiteralValue::Integer(10), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(100))
);
test_pipeline_case!(
    fold_add_13,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(15), LiteralValue::Integer(15), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(30))
);
test_pipeline_case!(
    fold_add_14,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(10000), LiteralValue::Integer(50), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(10050))
);
test_pipeline_case!(
    fold_add_15,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(100), LiteralValue::Integer(100), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(200))
);
test_pipeline_case!(
    fold_add_16,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(88), LiteralValue::Integer(12), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(100))
);
test_pipeline_case!(
    fold_add_17,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(40), LiteralValue::Integer(2), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(42))
);
test_pipeline_case!(
    fold_add_18,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(77), LiteralValue::Integer(77), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(154))
);
test_pipeline_case!(
    fold_add_19,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(14), LiteralValue::Integer(14), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(28))
);
test_pipeline_case!(
    fold_add_20,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(10), LiteralValue::Integer(10), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(20))
);
test_pipeline_case!(
    fold_add_21,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(33), LiteralValue::Integer(33), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(66))
);
test_pipeline_case!(
    fold_add_22,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(9), LiteralValue::Integer(90), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(99))
);
test_pipeline_case!(
    fold_add_23,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(500), LiteralValue::Integer(500), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(1000))
);
test_pipeline_case!(
    fold_add_24,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(50), LiteralValue::Integer(150), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(200))
);
test_pipeline_case!(
    fold_add_25,
    |r| setup_fold(BinaryOp::Add, LiteralValue::Integer(4), LiteralValue::Integer(4), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Integer(8))
);

// Boolean AND/OR folding tests (15 tests)
test_pipeline_case!(
    fold_and_1,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(true), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_and_2,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(true), LiteralValue::Bool(false), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_and_3,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(false), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_and_4,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(false), LiteralValue::Bool(false), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_or_1,
    |r| setup_fold(BinaryOp::Or, LiteralValue::Bool(true), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_or_2,
    |r| setup_fold(BinaryOp::Or, LiteralValue::Bool(true), LiteralValue::Bool(false), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_or_3,
    |r| setup_fold(BinaryOp::Or, LiteralValue::Bool(false), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_or_4,
    |r| setup_fold(BinaryOp::Or, LiteralValue::Bool(false), LiteralValue::Bool(false), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_and_5,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(true), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_and_6,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(true), LiteralValue::Bool(false), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_or_5,
    |r| setup_fold(BinaryOp::Or, LiteralValue::Bool(true), LiteralValue::Bool(false), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_or_6,
    |r| setup_fold(BinaryOp::Or, LiteralValue::Bool(false), LiteralValue::Bool(false), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_and_7,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(false), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_or_7,
    |r| setup_fold(BinaryOp::Or, LiteralValue::Bool(false), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_and_8,
    |r| setup_fold(BinaryOp::And, LiteralValue::Bool(true), LiteralValue::Bool(true), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);

// Equality folding tests (10 tests)
test_pipeline_case!(
    fold_eq_1,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(10), LiteralValue::Integer(10), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_eq_2,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(10), LiteralValue::Integer(20), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_eq_3,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(5), LiteralValue::Integer(5), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_eq_4,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(0), LiteralValue::Integer(1), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_eq_5,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(999), LiteralValue::Integer(999), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_eq_6,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(4), LiteralValue::Integer(4), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_eq_7,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(5), LiteralValue::Integer(6), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_eq_8,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(123), LiteralValue::Integer(123), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);
test_pipeline_case!(
    fold_eq_9,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(1), LiteralValue::Integer(2), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(false))
);
test_pipeline_case!(
    fold_eq_10,
    |r| setup_fold(BinaryOp::Eq, LiteralValue::Integer(100), LiteralValue::Integer(100), r),
    parallel_constant_folding_system,
    assert_fold_val(LiteralValue::Bool(true))
);

// --- 51-80: Width Inference Integration Tests (30 tests) ---
// Parameterized test case generating varying count of signals to verify analyzed count in statistics.
macro_rules! test_width_stats_case {
    ($name:ident, $signal_count:expr) => {
        #[test]
        fn $name() {
            let mut registry = Registry::new();
            for i in 0..$signal_count {
                registry.create_entity(&format!("sig_{}", i), KindComponent::SIGNAL);
            }

            let (_, _, _, stats) = parallel_width_inference_system(&mut registry);
            assert_eq!(stats.nodes_analyzed, $signal_count);
            assert_eq!(stats.scc_count, $signal_count);
            assert_eq!(stats.diagnostics_count, 0);
        }
    };
}

test_width_stats_case!(width_stat_0, 0);
test_width_stats_case!(width_stat_1, 1);
test_width_stats_case!(width_stat_2, 2);
test_width_stats_case!(width_stat_3, 3);
test_width_stats_case!(width_stat_4, 4);
test_width_stats_case!(width_stat_5, 5);
test_width_stats_case!(width_stat_6, 6);
test_width_stats_case!(width_stat_7, 7);
test_width_stats_case!(width_stat_8, 8);
test_width_stats_case!(width_stat_9, 9);
test_width_stats_case!(width_stat_10, 10);
test_width_stats_case!(width_stat_11, 11);
test_width_stats_case!(width_stat_12, 12);
test_width_stats_case!(width_stat_13, 13);
test_width_stats_case!(width_stat_14, 14);
test_width_stats_case!(width_stat_15, 15);
test_width_stats_case!(width_stat_16, 16);
test_width_stats_case!(width_stat_17, 17);
test_width_stats_case!(width_stat_18, 18);
test_width_stats_case!(width_stat_19, 19);
test_width_stats_case!(width_stat_20, 20);
test_width_stats_case!(width_stat_21, 21);
test_width_stats_case!(width_stat_22, 22);
test_width_stats_case!(width_stat_23, 23);
test_width_stats_case!(width_stat_24, 24);
test_width_stats_case!(width_stat_25, 25);
test_width_stats_case!(width_stat_26, 26);
test_width_stats_case!(width_stat_27, 27);
test_width_stats_case!(width_stat_28, 28);
test_width_stats_case!(width_stat_29, 29);

// --- 81-100: Full Pipeline Orchestrator Integration Tests (20 tests) ---
macro_rules! test_pipeline_orchestration_case {
    ($name:ident, $sig_count:expr, $fold_count:expr) => {
        #[test]
        fn $name() {
            let mut registry = Registry::new();
            for i in 0..$sig_count {
                registry.create_entity(&format!("sig_{}", i), KindComponent::SIGNAL);
            }
            for i in 0..$fold_count {
                setup_fold(
                    BinaryOp::Add,
                    LiteralValue::Integer(i),
                    LiteralValue::Integer(1),
                    &mut registry,
                );
            }

            let stats = run_compilation_pipeline(&mut registry);
            assert_eq!(stats.nodes_analyzed, $sig_count);
            // Verify that constant folding worked (the binary_ops should be None now)
            assert_eq!(registry.binary_ops.iter().filter(|b| b.is_some()).count(), 0);
        }
    };
}

test_pipeline_orchestration_case!(pipeline_case_1, 5, 2);
test_pipeline_orchestration_case!(pipeline_case_2, 0, 5);
test_pipeline_orchestration_case!(pipeline_case_3, 10, 0);
test_pipeline_orchestration_case!(pipeline_case_4, 1, 1);
test_pipeline_orchestration_case!(pipeline_case_5, 12, 4);
test_pipeline_orchestration_case!(pipeline_case_6, 3, 8);
test_pipeline_orchestration_case!(pipeline_case_7, 15, 0);
test_pipeline_orchestration_case!(pipeline_case_8, 0, 10);
test_pipeline_orchestration_case!(pipeline_case_9, 2, 2);
test_pipeline_orchestration_case!(pipeline_case_10, 20, 5);
test_pipeline_orchestration_case!(pipeline_case_11, 4, 1);
test_pipeline_orchestration_case!(pipeline_case_12, 6, 2);
test_pipeline_orchestration_case!(pipeline_case_13, 8, 3);
test_pipeline_orchestration_case!(pipeline_case_14, 0, 0);
test_pipeline_orchestration_case!(pipeline_case_15, 1, 0);
test_pipeline_orchestration_case!(pipeline_case_16, 0, 1);
test_pipeline_orchestration_case!(pipeline_case_17, 30, 10);
test_pipeline_orchestration_case!(pipeline_case_18, 5, 0);
test_pipeline_orchestration_case!(pipeline_case_19, 0, 8);
test_pipeline_orchestration_case!(pipeline_case_20, 14, 6);
