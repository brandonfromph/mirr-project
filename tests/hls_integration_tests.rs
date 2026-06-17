//! Integration tests for HLS (High-Level Synthesis) optimization pass.

#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::hls::{HlsConfig, OpDag, ResourceKind, MAX_HLS_OPERATIONS};

#[test]
fn hls_single_operation_schedule() {
    let mut dag = OpDag::new();
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);

    let config = HlsConfig { latency: 1, ..Default::default() };
    let result = mirrc::hls::run_hls_pass(&dag, &config);
    assert!(result.is_ok(), "[E1203] HLS scheduling failed: {:?}", result.err());

    let hls = result.unwrap();
    assert_eq!(hls.schedule.len(), 1);
    assert_eq!(hls.schedule[0].earliest, 0);
}

#[test]
fn hls_chain_schedule() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let c = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    dag.add_edge(a, b);
    dag.add_edge(b, c);

    let config = HlsConfig { latency: 3, ..Default::default() };
    let result = mirrc::hls::run_hls_pass(&dag, &config);
    assert!(result.is_ok(), "[E1203] HLS scheduling failed: {:?}", result.err());

    let hls = result.unwrap();
    assert_eq!(hls.schedule.len(), 3);
    assert_eq!(hls.schedule[0].earliest, 0);
    assert_eq!(hls.schedule[1].earliest, 1);
    assert_eq!(hls.schedule[2].earliest, 2);
}

#[test]
fn hls_parallel_schedule() {
    let mut dag = OpDag::new();
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);
    dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);

    let config = HlsConfig { latency: 1, ..Default::default() };
    let result = mirrc::hls::run_hls_pass(&dag, &config);
    assert!(result.is_ok(), "[E1203] HLS scheduling failed: {:?}", result.err());

    let hls = result.unwrap();
    for op in &hls.schedule {
        assert_eq!(op.earliest, 0, "Independent ops should schedule to same cycle");
    }
}

#[test]
fn hls_max_operations_reject() {
    let mut dag = OpDag::new();
    for _ in 0..MAX_HLS_OPERATIONS {
        dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);
    }
    let result = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);
    assert_eq!(result, None, "DAG should reject operations beyond MAX_HLS_OPERATIONS");
}

#[test]
fn hls_empty_dag_rejected() {
    let dag = OpDag::new();
    let config = HlsConfig::default();
    let result = mirrc::hls::run_hls_pass(&dag, &config);
    assert!(result.is_err(), "Empty DAG should be rejected");
}

#[test]
fn hls_full_pipeline() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
    let c = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    dag.add_edge(a, b);
    dag.add_edge(a, c);

    let config = HlsConfig { latency: 3, sharing: true, binding: true, fifo: true };
    let result = mirrc::hls::run_hls_pass(&dag, &config);
    assert!(result.is_ok(), "[E1203] HLS scheduling failed: {:?}", result.err());

    let hls = result.unwrap();
    assert_eq!(hls.schedule.len(), 3);
    assert!(!hls.bindings.is_empty());
    assert!(!hls.resource_count.is_empty());
}

#[test]
fn hls_dag_add_edge() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
    dag.add_edge(a, b);

    assert!(dag.ops[b as usize].predecessors.contains(&a));
    assert!(dag.ops[a as usize].successors.contains(&b));
}

#[test]
fn hls_dag_duplicate_edge() {
    let mut dag = OpDag::new();
    let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
    let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
    dag.add_edge(a, b);
    dag.add_edge(a, b); // duplicate

    assert_eq!(dag.ops[b as usize].predecessors.len(), 1, "Duplicate edges should not be added");
}

#[test]
fn hls_resource_kinds_display() {
    assert_eq!(format!("{}", ResourceKind::Add), "add");
    assert_eq!(format!("{}", ResourceKind::Mul), "mul");
    assert_eq!(format!("{}", ResourceKind::And), "and");
}

#[test]
fn hls_default_config() {
    let config = HlsConfig::default();
    assert_eq!(config.latency, 1);
    assert!(config.sharing);
    assert!(config.binding);
    assert!(config.fifo);
}
