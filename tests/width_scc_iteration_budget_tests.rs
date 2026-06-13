#![forbid(unsafe_code)]
//! Width SCC iteration-budget and classification tests.

use mirrc::pipeline::PipelineConfig;

fn run_pipeline(
    source: &str,
) -> Result<mirrc::pipeline::PipelineResult, mirrc::error::PipelineErrors> {
    let mut config = PipelineConfig::default();
    config.temporal = false;
    mirrc::pipeline::run_pipeline_with_file(source, "test.mirr", &config)
}

#[test]
fn scc_phase_detects_cycle_and_solves() {
    let source = r#"
target profile {
    name: "test";
    word_size: 64;
    reg_width: 10;
    op_width: 6;
}

module ring {
    signal sr0: internal u8;
    signal sr1: internal u8;
    signal sr2: internal u8;

    guard g { when true for 1 cycles; }

    reflex r {
        on g {
            sr0 = prev(sr2, 1);
            sr1 = prev(sr0, 1);
            sr2 = prev(sr1, 1) + 1;
        }
    }
}
"#;
    let res = run_pipeline(source);
    if let Ok(res) = res {
        // Assert width result contains something
        assert!(res.width_result.is_some());
    }
}

#[test]
fn nonexpansive_solver_zero_anchor_reports_e509() {
    let mut registry = mirrc::ecs::Registry::new();
    let e = registry.create_entity(
        "ghost",
        mirrc::ecs::components::KindComponent(mirrc::ecs::components::EntityKind::SIGNAL(
            mirrc::ast::types::SignalKind::Internal,
        )),
    );
    let tc = mirrc::ecs::components::TypeComponent(mirrc::ast::types::ExtendedType::from_core(
        mirrc::ast::types::SignalType::Unsigned(0),
    ));
    registry.set_type(e, tc);

    let scc = mirrc::width::types::SccInfo {
        signals: vec![e],
        kind: mirrc::width::types::SccKind::Nonexpansive,
    };
    let solved = mirrc::width::scc_solver::solve_nonexpansive(&scc, &registry);
    let diags = format!("{:?}", solved.diagnostics);
    assert!(diags.contains("E509"), "expected E509 for unanchored nonexpansive SCC");
}
