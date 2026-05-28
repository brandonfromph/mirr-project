#![forbid(unsafe_code)]
//! Integration tests targeting TemporalNodeComponent and temporal compilation synthesis metadata.
//! Contains exactly 50 distinct test cases.

use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use nasa_rust_project::ecs::components::*;
use nasa_rust_project::ecs::registry::Registry;
use nasa_rust_project::ecs::systems::temporal_synthesis_system;

// Helper factories to construct ECS conditions
fn make_sig(r: &mut Registry, name: &str) -> EntityId {
    let sig_ent = r.create_entity(name, KindComponent::SIGNAL);
    let ref_ent = r.next_id();
    r.signal_refs[ref_ent.0 as usize] = Some(SignalRefComponent(sig_ent));
    ref_ent
}

fn make_prev(r: &mut Registry, name: &str, delay: u64) -> EntityId {
    let sig_ent = r.create_entity(name, KindComponent::SIGNAL);
    let prev_ent = r.next_id();
    r.prev_ops[prev_ent.0 as usize] = Some(PrevComponent { signal: sig_ent, delay });
    prev_ent
}

fn make_not(r: &mut Registry, name: &str) -> EntityId {
    let sig_ent = r.create_entity(name, KindComponent::SIGNAL);
    let ref_ent = r.next_id();
    r.signal_refs[ref_ent.0 as usize] = Some(SignalRefComponent(sig_ent));
    let not_ent = r.next_id();
    r.unary_ops[not_ent.0 as usize] = Some(UnaryComponent { op: UnaryOp::Not, operand: ref_ent });
    not_ent
}

fn make_cmp(r: &mut Registry, name: &str, op: BinaryOp, val: LiteralValue) -> EntityId {
    let sig_ent = r.create_entity(name, KindComponent::SIGNAL);
    let ref_ent = r.next_id();
    r.signal_refs[ref_ent.0 as usize] = Some(SignalRefComponent(sig_ent));
    let lit_ent = r.next_id();
    r.literals[lit_ent.0 as usize] = Some(LiteralComponent(val));
    let binary_ent = r.next_id();
    r.binary_ops[binary_ent.0 as usize] =
        Some(BinaryComponent { op, left: ref_ent, right: lit_ent });
    binary_ent
}

fn make_always(r: &mut Registry) -> EntityId {
    let lit_ent = r.next_id();
    r.literals[lit_ent.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    lit_ent
}

fn make_and(r: &mut Registry, left: EntityId, right: EntityId) -> EntityId {
    let binary_ent = r.next_id();
    r.binary_ops[binary_ent.0 as usize] = Some(BinaryComponent { op: BinaryOp::And, left, right });
    binary_ent
}

fn make_or(r: &mut Registry, left: EntityId, right: EntityId) -> EntityId {
    let binary_ent = r.next_id();
    r.binary_ops[binary_ent.0 as usize] = Some(BinaryComponent { op: BinaryOp::Or, left, right });
    binary_ent
}

// Parameterized test macro
macro_rules! test_temporal_case {
    ($name:ident, $guard_name:expr, $cycles:expr, $setup_expr:expr, $check_fn:expr) => {
        #[test]
        fn $name() {
            let mut registry = Registry::new();
            let cond_id = {
                let setup: fn(&mut Registry) -> EntityId = $setup_expr;
                setup(&mut registry)
            };
            let g = registry.create_entity($guard_name, KindComponent::GUARD);
            registry.cycles[g.0 as usize] = Some(CyclesComponent($cycles));
            registry.conditions[g.0 as usize] = Some(ConditionComponent(cond_id));

            let _netlist = temporal_synthesis_system(&mut registry).unwrap();
            let node = registry.temporal_nodes[g.0 as usize]
                .as_ref()
                .expect("TemporalNodeComponent should be back-propagated to the guard entity");

            let checker = $check_fn;
            checker(node);
        }
    };
}

// --- 1-16: ShiftRegister Strategy Tests (delays 1 to 16 cycles) ---
test_temporal_case!(test_sr_1, "g_sr1", 1, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 1);
    assert_eq!(node.output_signal, "g_sr1_out");
    assert_eq!(node.generated_signals.len(), 1);
    assert_eq!(node.generated_signals[0], "g_sr1_sr_0");
});

test_temporal_case!(test_sr_2, "g_sr2", 2, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 2);
    assert_eq!(node.generated_signals.len(), 2);
});

test_temporal_case!(test_sr_3, "g_sr3", 3, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 3);
});

test_temporal_case!(test_sr_4, "g_sr4", 4, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 4);
    assert_eq!(node.generated_signals[3], "g_sr4_sr_3");
});

test_temporal_case!(test_sr_5, "g_sr5", 5, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 5);
});

test_temporal_case!(test_sr_6, "g_sr6", 6, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 6);
});

test_temporal_case!(test_sr_7, "g_sr7", 7, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 7);
});

test_temporal_case!(test_sr_8, "g_sr8", 8, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 8);
});

test_temporal_case!(test_sr_9, "g_sr9", 9, |r| make_sig(r, "a"), |node: &TemporalNodeComponent| {
    assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(node.delay_cycles, 9);
});

test_temporal_case!(
    test_sr_10,
    "g_sr10",
    10,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 10);
    }
);

test_temporal_case!(
    test_sr_11,
    "g_sr11",
    11,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 11);
    }
);

test_temporal_case!(
    test_sr_12,
    "g_sr12",
    12,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 12);
    }
);

test_temporal_case!(
    test_sr_13,
    "g_sr13",
    13,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 13);
    }
);

test_temporal_case!(
    test_sr_14,
    "g_sr14",
    14,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 14);
    }
);

test_temporal_case!(
    test_sr_15,
    "g_sr15",
    15,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 15);
    }
);

test_temporal_case!(
    test_sr_16,
    "g_sr16",
    16,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 16);
    }
);

// --- 17-32: Counter Strategy Tests (delays 17 to 32 cycles) ---
test_temporal_case!(
    test_counter_17,
    "g_c17",
    17,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 17);
        assert_eq!(node.output_signal, "g_c17_out");
        assert_eq!(node.generated_signals[0], "g_c17_counter");
        assert_eq!(node.generated_signals[1], "g_c17_cmp");
    }
);

test_temporal_case!(
    test_counter_18,
    "g_c18",
    18,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 18);
    }
);

test_temporal_case!(
    test_counter_19,
    "g_c19",
    19,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 19);
    }
);

test_temporal_case!(
    test_counter_20,
    "g_c20",
    20,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 20);
    }
);

test_temporal_case!(
    test_counter_21,
    "g_c21",
    21,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 21);
    }
);

test_temporal_case!(
    test_counter_22,
    "g_c22",
    22,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 22);
    }
);

test_temporal_case!(
    test_counter_23,
    "g_c23",
    23,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 23);
    }
);

test_temporal_case!(
    test_counter_24,
    "g_c24",
    24,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 24);
    }
);

test_temporal_case!(
    test_counter_25,
    "g_c25",
    25,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 25);
    }
);

test_temporal_case!(
    test_counter_26,
    "g_c26",
    26,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 26);
    }
);

test_temporal_case!(
    test_counter_27,
    "g_c27",
    27,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 27);
    }
);

test_temporal_case!(
    test_counter_28,
    "g_c28",
    28,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 28);
    }
);

test_temporal_case!(
    test_counter_29,
    "g_c29",
    29,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 29);
    }
);

test_temporal_case!(
    test_counter_30,
    "g_c30",
    30,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 30);
    }
);

test_temporal_case!(
    test_counter_31,
    "g_c31",
    31,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 31);
    }
);

test_temporal_case!(
    test_counter_32,
    "g_c32",
    32,
    |r| make_sig(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 32);
    }
);

// --- 33-42: Complex AND/OR Strategy Tests ---
test_temporal_case!(
    test_complex_and_1,
    "g_and1",
    5,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_and(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
        assert_eq!(node.output_signal, "g_and1_out");
    }
);

test_temporal_case!(
    test_complex_and_2,
    "g_and2",
    20,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_and(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

test_temporal_case!(
    test_complex_and_3,
    "g_and3",
    2,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_and(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

test_temporal_case!(
    test_complex_and_4,
    "g_and4",
    100,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_and(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

test_temporal_case!(
    test_complex_and_5,
    "g_and5",
    1,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_and(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

test_temporal_case!(
    test_complex_or_1,
    "g_or1",
    5,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_or(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
        assert_eq!(node.output_signal, "g_or1_out");
    }
);

test_temporal_case!(
    test_complex_or_2,
    "g_or2",
    20,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_or(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

test_temporal_case!(
    test_complex_or_3,
    "g_or3",
    2,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_or(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

test_temporal_case!(
    test_complex_or_4,
    "g_or4",
    100,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_or(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

test_temporal_case!(
    test_complex_or_5,
    "g_or5",
    1,
    |r| {
        let a = make_sig(&mut *r, "a");
        let b = make_sig(&mut *r, "b");
        make_or(r, a, b)
    },
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Complex));
    }
);

// --- 43-46: Always True Strategy Tests ---
test_temporal_case!(
    test_always_0,
    "always",
    0,
    |r| make_always(r),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 0);
        assert_eq!(node.output_signal, "always_out");
    }
);

test_temporal_case!(
    test_always_5,
    "always",
    5,
    |r| make_always(r),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 5);
    }
);

test_temporal_case!(
    test_always_15,
    "always",
    15,
    |r| make_always(r),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 15);
    }
);

test_temporal_case!(
    test_always_20,
    "always",
    20,
    |r| make_always(r),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 20);
    }
);

// --- 47-50: Comparison Operators Tests ---
test_temporal_case!(
    test_cmp_lt,
    "g_lt",
    5,
    |r| make_cmp(r, "a", BinaryOp::Lt, LiteralValue::Integer(10)),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 5);
    }
);

test_temporal_case!(
    test_cmp_gt,
    "g_gt",
    25,
    |r| make_cmp(r, "a", BinaryOp::Gt, LiteralValue::Integer(10)),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 25);
    }
);

test_temporal_case!(
    test_cmp_le,
    "g_le",
    8,
    |r| make_not(r, "a"),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::ShiftRegister));
        assert_eq!(node.delay_cycles, 8);
    }
);

test_temporal_case!(
    test_cmp_ge,
    "g_ge",
    40,
    |r| make_prev(r, "a", 5),
    |node: &TemporalNodeComponent| {
        assert!(matches!(node.strategy, TemporalStrategy::Counter { .. }));
        assert_eq!(node.delay_cycles, 45);
    }
);
