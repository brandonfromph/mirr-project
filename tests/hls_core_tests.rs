//! MEGA-12: HLS core integration tests.
//!
//! Tests the full HLS pass: DAG construction, ASAP/ALAP scheduling,
//! resource sharing, operation binding, and pipeline integration.

#![forbid(unsafe_code)]

use mirrc::hls::binding::bind_operations;
use mirrc::hls::schedule::{alap_schedule, asap_schedule, compute_mobility, ScheduleOp};
use mirrc::hls::sharing::find_shareable_ops;
use mirrc::hls::{run_hls_pass, HlsConfig, OpDag, ResourceKind};

// =========================================================================
// DAG construction tests
// =========================================================================

#[test]
fn test_dag_build_empty() {
    let dag = OpDag::new();
    assert_eq!(dag.ops.len(), 0);
}

#[test]
fn test_dag_build_single_op() {
    let mut dag = OpDag::new();
    let id = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);
    assert_eq!(id, Some(0));
    assert_eq!(dag.ops.len(), 1);
}

#[test]
fn test_dag_build_chain() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
    let c = dag.add_op(ResourceKind::And, 8, vec![16, 8], vec![]).unwrap();
    dag.add_edge(a, b);
    dag.add_edge(b, c);

    assert_eq!(dag.ops.len(), 3);
    assert_eq!(dag.ops[b as usize].predecessors.len(), 1);
    assert_eq!(dag.ops[c as usize].predecessors.len(), 1);
}

// =========================================================================
// ASAP scheduling tests
// =========================================================================

#[test]
fn test_asap_single_op() {
    let mut dag = OpDag::new();
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);

    let schedule = asap_schedule(&dag).unwrap();
    assert_eq!(schedule.len(), 1);
    assert_eq!(schedule[0].earliest, 0);
}

#[test]
fn test_asap_chain() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
    let c = dag.add_op(ResourceKind::And, 8, vec![16, 8], vec![]).unwrap();
    dag.add_edge(a, b);
    dag.add_edge(b, c);

    let schedule = asap_schedule(&dag).unwrap();
    assert_eq!(schedule[0].earliest, 0);
    assert_eq!(schedule[1].earliest, 1);
    assert_eq!(schedule[2].earliest, 2);
}

// =========================================================================
// ALAP scheduling tests
// =========================================================================

#[test]
fn test_alap_chain() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
    dag.add_edge(a, b);

    let schedule = alap_schedule(&dag, 2).unwrap();
    assert_eq!(schedule[b as usize].latest, 1);
    assert_eq!(schedule[a as usize].latest, 0);
}

// =========================================================================
// Mobility tests
// =========================================================================

#[test]
fn test_mobility_zero() {
    let asap = vec![ScheduleOp { op_id: 0, earliest: 0, latest: 0, resource: ResourceKind::Add }];
    let alap = vec![ScheduleOp { op_id: 0, earliest: 0, latest: 0, resource: ResourceKind::Add }];

    let mobility = compute_mobility(&asap, &alap);
    assert_eq!(mobility[0], 0);
}

// =========================================================================
// Resource sharing tests
// =========================================================================

#[test]
fn test_sharing_no_overlap() {
    let schedule = vec![
        ScheduleOp { op_id: 0, earliest: 0, latest: 1, resource: ResourceKind::Add },
        ScheduleOp { op_id: 1, earliest: 2, latest: 3, resource: ResourceKind::Add },
    ];

    let groups = find_shareable_ops(&schedule);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].len(), 2);
}

#[test]
fn test_sharing_overlap() {
    let schedule = vec![
        ScheduleOp { op_id: 0, earliest: 0, latest: 2, resource: ResourceKind::Add },
        ScheduleOp { op_id: 1, earliest: 1, latest: 3, resource: ResourceKind::Add },
    ];

    let groups = find_shareable_ops(&schedule);
    assert_eq!(groups.len(), 0);
}

// =========================================================================
// Binding tests
// =========================================================================

#[test]
fn test_binding_no_overlap() {
    let schedule = vec![
        ScheduleOp { op_id: 0, earliest: 0, latest: 1, resource: ResourceKind::Add },
        ScheduleOp { op_id: 1, earliest: 2, latest: 3, resource: ResourceKind::Add },
    ];

    let bindings = bind_operations(&schedule);
    assert_eq!(bindings[0], bindings[1]); // Should share resource.
}

#[test]
fn test_binding_overlap() {
    let schedule = vec![
        ScheduleOp { op_id: 0, earliest: 0, latest: 2, resource: ResourceKind::Add },
        ScheduleOp { op_id: 1, earliest: 1, latest: 3, resource: ResourceKind::Add },
    ];

    let bindings = bind_operations(&schedule);
    assert_ne!(bindings[0], bindings[1]); // Different resources.
}

// =========================================================================
// Full HLS pass tests
// =========================================================================

#[test]
fn test_hls_pass_single_op() {
    let mut dag = OpDag::new();
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);

    let config = HlsConfig::default();
    let result = run_hls_pass(&dag, &config).unwrap();
    assert_eq!(result.schedule.len(), 1);
}

#[test]
fn test_hls_pass_chain() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    dag.add_edge(a, b);

    let config = HlsConfig::default();
    let result = run_hls_pass(&dag, &config).unwrap();
    assert_eq!(result.schedule.len(), 2);
    assert_eq!(result.schedule[a as usize].earliest, 0);
    assert_eq!(result.schedule[b as usize].earliest, 1);
}

#[test]
fn test_hls_pass_empty_dag() {
    let dag = OpDag::new();
    let config = HlsConfig::default();
    let result = run_hls_pass(&dag, &config);
    assert!(result.is_err());
}

#[test]
fn test_hls_pass_sharing_disabled() {
    let mut dag = OpDag::new();
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);

    let config = HlsConfig { latency: 1, sharing: false, binding: true, fifo: true };
    let result = run_hls_pass(&dag, &config).unwrap();
    assert_eq!(result.sharing_groups.len(), 0);
}

// =========================================================================
// Pipeline integration tests
// =========================================================================

#[test]
fn test_pipeline_with_hls_disabled() {
    let source = r#"
module test {
    signal x: in u8;
    signal y: out u8;
    signal trigger: in u8;
    guard tick {
        when trigger == 1
        for 1 cycles;
    }
    reflex compute {
        on tick {
            y = x + 1;
        }
    }
}
"#;

    let config =
        mirrc::PipelineConfig { hls: false, rspu: false, mape_k: false, ..Default::default() };

    let result = mirrc::run_pipeline(source, &config).unwrap();
    assert!(result.hls_result.is_none());
}

#[test]
fn test_pipeline_with_hls_enabled() {
    let source = r#"
module test {
    signal x: in u8;
    signal y: out u8;
    signal trigger: in u8;
    guard tick {
        when trigger == 1
        for 1 cycles;
    }
    reflex compute {
        on tick {
            y = x + 1;
        }
    }
}
"#;

    let config =
        mirrc::PipelineConfig { hls: true, rspu: false, mape_k: false, ..Default::default() };

    let result = mirrc::run_pipeline(source, &config).unwrap();
    // HLS may or may not produce results depending on the DAG.
    // The pass should not crash.
    let _ = result.hls_result;
}
