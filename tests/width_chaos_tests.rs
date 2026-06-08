#![forbid(unsafe_code)]

use mirrc::ast::program::{Guard, MirrProgram, Module, SignalDecl};
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::width::scc_solver::solve_nonexpansive;
use mirrc::width::types::{SccInfo, SccKind};

#[test]
fn test_width_chaos_nonexpansive_chain_performance() {
    // Create a chain of 500 signals: S0 -> S1 -> S2 -> ... -> S499
    // Only S499 has a width (u32).
    let mut signal_indices = Vec::new();
    let mut signals = Vec::new();

    for i in 0..500 {
        let name = format!("s{}", i);
        let width = if i == 499 { 32 } else { 0 };
        signals.push(SignalDecl {
            name: name.clone(),
            kind: SignalKind::Internal,
            ty: ExtendedType::from(SignalType::Unsigned(width)),
            span: None,
            origin: None,
        });
        signal_indices.push(i);
    }

    let scc = SccInfo { signal_indices, kind: SccKind::Nonexpansive };

    // Performance test: ensure it doesn't hang and converges
    let result = solve_nonexpansive(&scc, &signals);
    assert!(result.diagnostics.is_empty(), "Errors: {:?}", result.diagnostics);
    for w in result.widths {
        assert_eq!(w, 32);
    }
}

#[test]
fn test_width_chaos_unbounded_expansive_loop() {
    // Expansive loop: A = A + 1. No temporal guard.
    // This should be caught by Strategy 3 (Hard Error E510).
    let signals = vec![SignalDecl {
        name: "a".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from(SignalType::Unsigned(0)), // No width
        span: None,
        origin: None,
    }];

    let scc = SccInfo { signal_indices: vec![0], kind: SccKind::Expansive };

    let program = MirrProgram {
        imports: vec![],
        patterns: vec![],
        module: Module {
            name: "top".to_string(),
            signals: signals.clone(),
            guards: vec![],
            reflexes: vec![],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        },
    };

    let result = mirrc::width::scc_solver::solve_expansive(&scc, &signals, &[], &program);
    assert!(!result.diagnostics.is_empty());
    let err_str = format!("{:?}", result.diagnostics);
    assert!(err_str.contains("E510"), "Expected E510, got: {}", err_str);
}

#[test]
fn test_width_chaos_overflowing_inference() {
    // Guard: for 2^60 cycles
    // Reflex: A = prev(A) + 2^10
    // Resulting width: 60 + 10 = 70 bits.
    // This exceeds u64 (64 bits), so Strategy 2 should FAIL (return None) and trigger E510.

    let sig_a = SignalDecl {
        name: "a".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from(SignalType::Unsigned(0)),
        span: None,
        origin: None,
    };
    let signals = vec![sig_a.clone()];

    let guard = Guard {
        name: "g".to_string(),
        cycles: 1u64 << 60,
        condition: mirrc::ast::Expr::Literal(mirrc::ast::types::LiteralValue::Bool(true)),
        span: None,
        template_cycles: None,
        origin: None,
    };

    use mirrc::ast::program::Assignment;
    use mirrc::ast::program::Reflex;
    use mirrc::ast::Expr;

    let reflex = Reflex {
        name: "r".to_string(),
        guard_names: vec!["g".to_string()],
        assignments: vec![Assignment {
            target: "a".to_string(),
            value: Expr::Binary {
                op: mirrc::ast::types::BinaryOp::Add,
                left: Box::new(Expr::Prev { signal: "a".to_string(), delay: 1 }),
                right: Box::new(Expr::Literal(mirrc::ast::types::LiteralValue::Integer(1024))),
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

    let result = mirrc::width::scc_solver::solve_expansive(&scc, &signals, &[guard], &program);
    assert!(!result.diagnostics.is_empty());
    let err_str = format!("{:?}", result.diagnostics);
    // Should fail Strategy 2 because bits > 64
    assert!(
        err_str.contains("E510"),
        "Expected E510 due to width overflow (>64 bits), got: {}",
        err_str
    );
}
