#![forbid(unsafe_code)]

use mirrc::cross_surface_stress::{
    FailureClass, FuzzHarnessBuilder, LeakBudget, LeakBudgetReport, LeakBudgetStatus, MemoryTrend,
    StressRunConfig, Surface,
};

#[test]
fn default_fuzz_harness_builder() {
    let hb = FuzzHarnessBuilder::default();
    let _harness = hb.build();
}

#[test]
fn run_malformed_batch_with_empty_vec() {
    let hb = FuzzHarnessBuilder::default();
    let harness = hb.build();
    let report = harness.run_malformed_batch(vec![]);
    assert_eq!(report.status(), mirrc::cross_surface_stress::StressRunStatus::Completed);
}

#[test]
fn memory_trend_from_single_sample() {
    let trend = MemoryTrend::from_samples(vec![100]);
    assert!(trend.is_non_increasing());
    assert_eq!(trend.leak_slope_bytes_per_minute(), 0.0);
}

#[test]
fn default_stress_run_config() {
    let _cfg = StressRunConfig::default();
}

#[test]
fn leak_budget_exceeded_per_surface() {
    let budget = LeakBudget::per_surface_bytes([(Surface::Daemon, 100)]);
    let budget_report =
        LeakBudgetReport::from_surface_deltas(budget.clone(), [(Surface::Daemon, 150)]);
    assert_eq!(budget_report.status(), LeakBudgetStatus::Exceeded);
    assert_eq!(budget_report.primary_failure_class(), FailureClass::LeakBudgetExceeded);
}

#[test]
fn leak_budget_exceeded_global() {
    let budget_global = LeakBudget::global_bytes(200);
    let budget_report =
        LeakBudgetReport::from_surface_deltas(budget_global.clone(), [(Surface::Daemon, 250)]);
    assert_eq!(budget_report.status(), LeakBudgetStatus::Exceeded);
    assert_eq!(budget_report.primary_failure_class(), FailureClass::LeakBudgetExceeded);
}

#[test]
fn leak_budget_from_time_window_empty_samples() {
    let budget = LeakBudget::per_surface_bytes([(Surface::Daemon, 100)]);
    let budget_empty = LeakBudgetReport::from_time_window(budget.clone(), vec![]);
    assert_eq!(budget_empty.status(), LeakBudgetStatus::WithinBudget);
    assert_eq!(budget_empty.primary_failure_class(), FailureClass::ResourceExhaustion);
}

#[test]
fn leak_budget_from_time_window_exceeded_global() {
    let budget_global = LeakBudget::global_bytes(200);
    let budget_report = LeakBudgetReport::from_time_window(budget_global.clone(), vec![50, 300]);
    assert_eq!(budget_report.status(), LeakBudgetStatus::Exceeded);
    assert_eq!(budget_report.primary_failure_class(), FailureClass::LeakBudgetExceeded);
}

#[test]
fn leak_budget_from_time_window_exceeded_per_surface() {
    let budget = LeakBudget::per_surface_bytes([(Surface::Daemon, 100)]);
    let budget_report = LeakBudgetReport::from_time_window(budget.clone(), vec![10, 150]);
    assert_eq!(budget_report.status(), LeakBudgetStatus::Exceeded);
    assert_eq!(budget_report.primary_failure_class(), FailureClass::LeakBudgetExceeded);
}

#[test]
fn leak_budget_within_budget_resource_exhaustion() {
    let budget_global = LeakBudget::global_bytes(200);
    let budget_safe =
        LeakBudgetReport::from_surface_deltas(budget_global.clone(), [(Surface::Daemon, 50)]);
    assert_eq!(budget_safe.status(), LeakBudgetStatus::WithinBudget);
    assert_eq!(budget_safe.primary_failure_class(), FailureClass::ResourceExhaustion);
}
