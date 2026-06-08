#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use mirrc::width::scc_solver::solve_expansive;
use mirrc::width::types::{SccInfo, SccKind};

fn make_signal(name: &str, width: u32) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from(SignalType::Unsigned(width)),
        span: None,
        origin: None,
    }
}

fn make_guard(name: &str, cycles: u64) -> Guard {
    Guard {
        name: name.to_string(),
        cycles,
        condition: Expr::Literal(LiteralValue::Bool(true)),
        span: None,
        template_cycles: None,
        origin: None,
    }
}

#[test]
fn test_solve_expansive_simple_accumulator() {
    // 1. Arrange: A = prev(A) + 5 gated by guard g (10 cycles).
    // Expected max value: 5 * 10 = 50.
    // min_bits_for(50) = 6 bits (u6).
    let sig = make_signal("a", 0); // No declared width, trigger inference.
    let signals = vec![sig.clone()];
    let guard = make_guard("g", 10);

    let reflex = Reflex {
        name: "r".to_string(),
        guard_names: vec!["g".to_string()],
        assignments: vec![Assignment {
            target: "a".to_string(),
            value: Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Prev { signal: "a".to_string(), delay: 1 }),
                right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
            },
            span: None,
        }],
        span: None,
        origin: None,
    };

    let program = MirrProgram {
        imports: vec![],
        patterns: vec![],
        module: Module {
            name: "top".to_string(),
            signals: signals.clone(),
            guards: vec![guard.clone()],
            reflexes: vec![reflex],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        },
    };

    let scc = SccInfo { signal_indices: vec![0], kind: SccKind::Expansive };

    // 2. Act
    let result = solve_expansive(&scc, &signals, &[guard], &program);

    // 3. Assert
    assert!(
        result
            .diagnostics
            .iter()
            .all(|d| d.severity != mirrc::width::types::DiagSeverity::Error),
        "Expected no error diagnostics: {:?}",
        result.diagnostics
    );
    assert_eq!(result.widths.len(), 1);
    assert_eq!(result.widths[0], 6, "Expected inferred width of 6 bits for max value of 50");
}

#[test]
fn test_solve_expansive_reversed_operand_accumulator() {
    // 1. Arrange: A = 3 + prev(A) gated by guard g (100 cycles).
    // Expected max value: 3 * 100 = 300.
    // min_bits_for(300) = 9 bits (u9).
    let sig = make_signal("a", 0);
    let signals = vec![sig.clone()];
    let guard = make_guard("g", 100);

    let reflex = Reflex {
        name: "r".to_string(),
        guard_names: vec!["g".to_string()],
        assignments: vec![Assignment {
            target: "a".to_string(),
            value: Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Literal(LiteralValue::Integer(3))),
                right: Box::new(Expr::Prev { signal: "a".to_string(), delay: 1 }),
            },
            span: None,
        }],
        span: None,
        origin: None,
    };

    let program = MirrProgram {
        imports: vec![],
        patterns: vec![],
        module: Module {
            name: "top".to_string(),
            signals: signals.clone(),
            guards: vec![guard.clone()],
            reflexes: vec![reflex],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        },
    };

    let scc = SccInfo { signal_indices: vec![0], kind: SccKind::Expansive };

    // 2. Act
    let result = solve_expansive(&scc, &signals, &[guard], &program);

    // 3. Assert
    assert!(
        result
            .diagnostics
            .iter()
            .all(|d| d.severity != mirrc::width::types::DiagSeverity::Error),
        "Expected no error diagnostics: {:?}",
        result.diagnostics
    );
    assert_eq!(result.widths.len(), 1);
    assert_eq!(result.widths[0], 9, "Expected inferred width of 9 bits for max value of 300");
}

#[test]
fn test_solve_expansive_multiple_accumulators_takes_first_valid() {
    // 1. Arrange: Multiple reflexes assigning to signal 'a'.
    // Reflex r1: A = prev(A) + 2 gated by g1 (5 cycles) => 10 => 4 bits
    // Reflex r2: A = prev(A) + 8 gated by g2 (10 cycles) => 80 => 7 bits
    let sig = make_signal("a", 0);
    let signals = vec![sig.clone()];
    let guard1 = make_guard("g1", 5);
    let guard2 = make_guard("g2", 10);

    let reflex1 = Reflex {
        name: "r1".to_string(),
        guard_names: vec!["g1".to_string()],
        assignments: vec![Assignment {
            target: "a".to_string(),
            value: Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Prev { signal: "a".to_string(), delay: 1 }),
                right: Box::new(Expr::Literal(LiteralValue::Integer(2))),
            },
            span: None,
        }],
        span: None,
        origin: None,
    };

    let reflex2 = Reflex {
        name: "r2".to_string(),
        guard_names: vec!["g2".to_string()],
        assignments: vec![Assignment {
            target: "a".to_string(),
            value: Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Prev { signal: "a".to_string(), delay: 1 }),
                right: Box::new(Expr::Literal(LiteralValue::Integer(8))),
            },
            span: None,
        }],
        span: None,
        origin: None,
    };

    let program = MirrProgram {
        imports: vec![],
        patterns: vec![],
        module: Module {
            name: "top".to_string(),
            signals: signals.clone(),
            guards: vec![guard1.clone(), guard2.clone()],
            reflexes: vec![reflex1, reflex2],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        },
    };

    let scc = SccInfo { signal_indices: vec![0], kind: SccKind::Expansive };

    // 2. Act
    let result = solve_expansive(&scc, &signals, &[guard1, guard2], &program);

    // 3. Assert
    assert!(
        result
            .diagnostics
            .iter()
            .all(|d| d.severity != mirrc::width::types::DiagSeverity::Error),
        "Expected no error diagnostics: {:?}",
        result.diagnostics
    );
    assert_eq!(result.widths.len(), 1);
    // Note: The loop in `infer_bound_from_guards` iterates through reflexes in module order.
    // The first one it finds matching the accumulator will be returned.
    // reflex1 is first, so 2 * 5 = 10 max value => min_bits_for(10) = 4.
    assert_eq!(result.widths[0], 4, "Expected first valid matching reflex to determine width");
}
