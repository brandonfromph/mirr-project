#![forbid(unsafe_code)]
//! Width SCC iteration-budget and classification tests.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::width;
use nasa_rust_project::width::scc_solver::solve_nonexpansive;
use nasa_rust_project::width::types::{SccInfo, SccKind};

fn sig(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn ring_program() -> MirrProgram {
    MirrProgram {
        patterns: Vec::new(),
        imports: Vec::new(),
        module: Module {
            name: "ring".to_string(),
            signals: vec![
                sig("sr0", SignalKind::Internal, SignalType::Unsigned(8)),
                sig("sr1", SignalKind::Internal, SignalType::Unsigned(8)),
                sig("sr2", SignalKind::Internal, SignalType::Unsigned(8)),
            ],
            guards: vec![Guard {
                name: "g".to_string(),
                condition: Expr::Literal(LiteralValue::Bool(true)),
                cycles: 1,
                origin: None,
                span: None,
            }],
            reflexes: vec![Reflex {
                name: "r".to_string(),
                guard_names: vec!["g".to_string()],
                assignments: vec![
                    Assignment {
                        target: "sr0".to_string(),
                        value: Expr::Prev { signal: "sr2".to_string(), delay: 1 },
                        span: None,
                    },
                    Assignment {
                        target: "sr1".to_string(),
                        value: Expr::Prev { signal: "sr0".to_string(), delay: 1 },
                        span: None,
                    },
                    Assignment {
                        target: "sr2".to_string(),
                        value: Expr::Binary {
                            op: BinaryOp::Add,
                            left: Box::new(Expr::Prev { signal: "sr1".to_string(), delay: 1 }),
                            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
                        },
                        span: None,
                    },
                ],
                origin: None,
                span: None,
            }],
            properties: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            span: None,
        },
    }
}

#[test]
fn scc_phase_detects_cycle_and_solves() {
    let prog = ring_program();
    let result = width::infer_program_widths_with_scc(&prog, None);
    assert!(result.stats.scc_count >= 1, "expected at least one SCC");
    assert!(!result.scc_solves.is_empty(), "expected solved SCC entries");
}

#[test]
fn nonexpansive_solver_zero_anchor_reports_e509() {
    let scc = SccInfo { signal_indices: vec![0], kind: SccKind::Nonexpansive };
    let signals = vec![sig("ghost", SignalKind::Internal, SignalType::Unsigned(0))];
    let solved = solve_nonexpansive(&scc, &signals);
    assert!(
        solved.diagnostics.iter().any(|d| d.message.contains("E509")),
        "expected E509 for unanchored nonexpansive SCC"
    );
}
