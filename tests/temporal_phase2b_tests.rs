#![forbid(unsafe_code)]

//! Tests for temporal Phase 2b: dynamic delays, retiming, clock domain crossing.

use nasa_rust_project::temporal::clock_domain::{
    ClockDomain, DomainCrossing, DEFAULT_SYNC_STAGES, MAX_CLOCK_DOMAINS, MAX_CROSSINGS,
};
use nasa_rust_project::temporal::low_level_ir::{
    CompiledGuard, ConditionKind, DynamicCounterGuard, TemporalNetlist, MAX_DYNAMIC_DELAY,
};
use nasa_rust_project::temporal::retiming::{
    RetimingConfig, RetimingStats, MAX_RETIMING_NODES, MAX_RETIMING_PASSES,
};

// ── DynamicCounterGuard tests ──

#[test]
fn dynamic_counter_guard_creation() {
    let ck = ConditionKind::SimpleSignal("sensor".to_string());
    let delay_expr = nasa_rust_project::ast::Expr::Signal("threshold".to_string());
    let dc = DynamicCounterGuard::new("dyn_guard".to_string(), ck.clone(), delay_expr, 1000);
    assert_eq!(dc.name, "dyn_guard");
    assert_eq!(dc.output_signal, "dyn_guard_out");
    assert_eq!(dc.counter_signal, "dyn_guard_dyn_counter");
    assert_eq!(dc.max_delay, 1000);
    assert_eq!(dc.condition_kind, ck);
}

#[test]
fn dynamic_counter_width_1() {
    let ck = ConditionKind::SimpleSignal("s".to_string());
    let dc = DynamicCounterGuard::new(
        "g".to_string(),
        ck,
        nasa_rust_project::ast::Expr::Signal("x".to_string()),
        1,
    );
    assert_eq!(dc.counter_width(), 1);
}

#[test]
fn dynamic_counter_width_8bit() {
    let ck = ConditionKind::SimpleSignal("s".to_string());
    let dc = DynamicCounterGuard::new(
        "g".to_string(),
        ck,
        nasa_rust_project::ast::Expr::Signal("x".to_string()),
        255,
    );
    assert_eq!(dc.counter_width(), 8);
}

#[test]
fn dynamic_counter_width_20bit() {
    let ck = ConditionKind::SimpleSignal("s".to_string());
    let dc = DynamicCounterGuard::new(
        "g".to_string(),
        ck,
        nasa_rust_project::ast::Expr::Signal("x".to_string()),
        MAX_DYNAMIC_DELAY,
    );
    assert_eq!(dc.counter_width(), 20);
}

#[test]
fn dynamic_counter_in_netlist() {
    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::SimpleSignal("clk_en".to_string());
    let dc = DynamicCounterGuard::new(
        "dyn".to_string(),
        ck,
        nasa_rust_project::ast::Expr::Signal("delay_val".to_string()),
        500,
    );
    netlist.add_guard(CompiledGuard::DynamicCounter(dc));
    assert_eq!(netlist.guards.len(), 1);
    assert_eq!(netlist.statistics.max_delay_cycles, 500);
}

#[test]
fn max_dynamic_delay_constant() {
    assert_eq!(MAX_DYNAMIC_DELAY, 1_048_576);
}

// ── Retiming tests ──

#[test]
fn retiming_config_default() {
    let config = RetimingConfig::default();
    assert!(!config.enabled);
    assert_eq!(config.max_passes, 4);
}

#[test]
fn retiming_bounded_passes() {
    assert_eq!(MAX_RETIMING_PASSES, 8);
}

#[test]
fn retiming_bounded_nodes() {
    assert_eq!(MAX_RETIMING_NODES, 1024);
}

#[test]
fn retiming_stats_fields() {
    let stats = RetimingStats {
        registers_moved: 0,
        critical_path_before: 10,
        critical_path_after: 10,
        passes_used: 0,
    };
    assert_eq!(stats.registers_moved, 0);
    assert_eq!(stats.passes_used, 0);
}

// ── Clock domain tests ──

#[test]
fn clock_domain_default() {
    let cd = ClockDomain { name: "clk".to_string(), frequency_hint: None };
    assert_eq!(cd.name, "clk");
    assert!(cd.frequency_hint.is_none());
}

#[test]
fn clock_domain_with_frequency() {
    let cd = ClockDomain { name: "fast_clk".to_string(), frequency_hint: Some(100_000_000) };
    assert_eq!(cd.name, "fast_clk");
    assert_eq!(cd.frequency_hint, Some(100_000_000));
}

#[test]
fn clock_domain_crossing_creation() {
    let dc = DomainCrossing {
        signal: "data_bus".to_string(),
        from_domain: "clk_a".to_string(),
        to_domain: "clk_b".to_string(),
        sync_stages: DEFAULT_SYNC_STAGES,
    };
    assert_eq!(dc.signal, "data_bus");
    assert_eq!(dc.sync_stages, 2);
}

#[test]
fn clock_domain_bounded() {
    assert_eq!(MAX_CLOCK_DOMAINS, 16);
    assert_eq!(MAX_CROSSINGS, 128);
    assert_eq!(DEFAULT_SYNC_STAGES, 2);
}
