#![forbid(unsafe_code)]

use mirrc::cross_surface_stress as css;

#[test]
fn fuzz_harness_contract_registers_wasm_host_surface() {
    let harness = css::FuzzHarnessBuilder::new()
        .with_surface(css::Surface::WasmHost)
        .with_seed(7)
        .with_case_budget(1_024)
        .build();

    assert!(harness.targets(css::Surface::WasmHost));
}

#[test]
fn fuzz_harness_contract_registers_lsp_surface() {
    let harness = css::FuzzHarnessBuilder::new()
        .with_surface(css::Surface::Lsp)
        .with_seed(11)
        .with_case_budget(2_048)
        .build();

    assert!(harness.targets(css::Surface::Lsp));
}

#[test]
fn fuzz_harness_contract_registers_daemon_surface() {
    let harness = css::FuzzHarnessBuilder::new()
        .with_surface(css::Surface::Daemon)
        .with_seed(13)
        .with_case_budget(4_096)
        .build();

    assert!(harness.targets(css::Surface::Daemon));
}

#[test]
fn fuzz_harness_contract_executes_campaign_across_all_surfaces() {
    let harness = css::FuzzHarnessBuilder::new()
        .with_surface(css::Surface::WasmHost)
        .with_surface(css::Surface::Lsp)
        .with_surface(css::Surface::Daemon)
        .with_seed(99)
        .with_case_budget(30_000)
        .build();

    let report = harness.run_campaign();
    assert_eq!(report.surface_count(), 3);
    assert_eq!(report.status(), css::StressRunStatus::Completed);
}

#[test]
fn malformed_wasm_payload_contract_is_recoverable() {
    let harness = css::FuzzHarnessBuilder::new().with_surface(css::Surface::WasmHost).build();

    let outcome = harness.inject_malformed(
        css::Surface::WasmHost,
        css::MalformedInput::binary(vec![0xFF, 0x00, 0xAA]),
    );

    assert_eq!(outcome.classification(), css::FailureClass::MalformedInput);
    assert!(outcome.recovered());
}

#[test]
fn malformed_lsp_frame_contract_is_recoverable() {
    let harness = css::FuzzHarnessBuilder::new().with_surface(css::Surface::Lsp).build();

    let outcome = harness.inject_malformed(
        css::Surface::Lsp,
        css::MalformedInput::utf8("{\"jsonrpc\":".to_string()),
    );

    assert_eq!(outcome.classification(), css::FailureClass::MalformedInput);
    assert!(outcome.recovered());
}

#[test]
fn malformed_daemon_command_contract_is_recoverable() {
    let harness = css::FuzzHarnessBuilder::new().with_surface(css::Surface::Daemon).build();

    let outcome = harness.inject_malformed(
        css::Surface::Daemon,
        css::MalformedInput::utf8("run --bad-flag".to_string()),
    );

    assert_eq!(outcome.classification(), css::FailureClass::MalformedInput);
    assert!(outcome.recovered());
}

#[test]
fn mixed_malformed_stream_contract_preserves_campaign_liveness() {
    let harness = css::FuzzHarnessBuilder::new()
        .with_surface(css::Surface::WasmHost)
        .with_surface(css::Surface::Lsp)
        .with_surface(css::Surface::Daemon)
        .build();

    let batch_report = harness.run_malformed_batch(vec![
        (css::Surface::WasmHost, css::MalformedInput::binary(vec![0x00, 0x01])),
        (css::Surface::Lsp, css::MalformedInput::utf8("{\"id\":null".to_string())),
        (css::Surface::Daemon, css::MalformedInput::utf8("--panic-now".to_string())),
    ]);

    assert_eq!(batch_report.status(), css::StressRunStatus::CompletedWithRecoverableFailures);
    assert_eq!(
        batch_report.failure_class_counts().get(&css::FailureClass::MalformedInput),
        Some(&3)
    );
}

#[test]
fn telemetry_sampling_contract_emits_periodic_samples() {
    let mut sampler = css::TelemetrySampler::new(css::SamplingCadence::hertz(20));

    sampler.tick_millis(50);
    sampler.tick_millis(50);

    assert_eq!(sampler.sample_count(), 2);
}

#[test]
fn telemetry_sampling_contract_records_heap_and_rss_channels() {
    let mut sampler = css::TelemetrySampler::new(css::SamplingCadence::hertz(10));

    sampler.record(css::TelemetryPoint::new(1_000_000, 2_000_000));
    sampler.record(css::TelemetryPoint::new(1_500_000, 2_500_000));

    let latest = sampler.latest().expect("latest sample must exist");
    assert_eq!(latest.heap_bytes(), 1_500_000);
    assert_eq!(latest.rss_bytes(), 2_500_000);
}

#[test]
fn telemetry_memory_trend_contract_detects_non_increasing_profile() {
    let trend = css::MemoryTrend::from_samples(vec![8_192, 8_000, 7_900, 7_750]);

    assert!(trend.is_non_increasing());
    assert!(trend.leak_slope_bytes_per_minute() <= 0.0);
}

#[test]
fn telemetry_memory_trend_contract_detects_positive_leak_slope() {
    let trend = css::MemoryTrend::from_samples(vec![4_096, 4_800, 5_632, 6_400]);

    assert!(!trend.is_non_increasing());
    assert!(trend.leak_slope_bytes_per_minute() > 0.0);
}

#[test]
fn stress_orchestration_contract_is_seed_reproducible() {
    let config = css::StressRunConfig::new().with_seed(4_242).with_iterations(10_000);

    let left = css::StressOrchestrator::new(config.clone()).dry_run();
    let right = css::StressOrchestrator::new(config).dry_run();

    assert_eq!(left.execution_fingerprint(), right.execution_fingerprint());
}

#[test]
fn stress_orchestration_contract_enforces_backpressure_policy() {
    let config = css::StressRunConfig::new()
        .with_seed(8080)
        .with_max_in_flight(64)
        .with_backpressure(css::BackpressurePolicy::DropNewest);

    let report = css::StressOrchestrator::new(config).run();
    assert_eq!(report.backpressure_policy(), css::BackpressurePolicy::DropNewest);
    assert!(report.max_observed_in_flight() <= 64);
}

#[test]
fn stress_failure_classification_contract_maps_timeout() {
    let classifier = css::FailureClassifier;
    let class = classifier.classify_timeout("wasm host timed out");

    assert_eq!(class, css::FailureClass::Timeout);
}

#[test]
fn stress_failure_classification_contract_maps_protocol_violation() {
    let classifier = css::FailureClassifier;
    let class = classifier.classify_protocol("lsp frame missing jsonrpc");

    assert_eq!(class, css::FailureClass::ProtocolViolation);
}

#[test]
fn stress_failure_classification_contract_maps_resource_exhaustion() {
    let classifier = css::FailureClassifier;
    let class = classifier.classify_resource_exhaustion("oom while fuzzing daemon");

    assert_eq!(class, css::FailureClass::ResourceExhaustion);
}

#[test]
fn leak_budget_report_contract_includes_surface_deltas() {
    let budget = css::LeakBudget::per_surface_bytes([
        (css::Surface::WasmHost, 16_384),
        (css::Surface::Lsp, 8_192),
        (css::Surface::Daemon, 4_096),
    ]);
    let report = css::LeakBudgetReport::from_surface_deltas(
        budget,
        [(css::Surface::WasmHost, 1_024), (css::Surface::Lsp, 2_048), (css::Surface::Daemon, 512)],
    );

    assert_eq!(report.delta_bytes(css::Surface::WasmHost), 1_024);
    assert_eq!(report.delta_bytes(css::Surface::Lsp), 2_048);
    assert_eq!(report.delta_bytes(css::Surface::Daemon), 512);
}

#[test]
fn leak_budget_report_contract_includes_peak_and_net_growth_windows() {
    let report = css::LeakBudgetReport::from_time_window(
        css::LeakBudget::global_bytes(32_768),
        vec![10_000, 10_400, 10_600, 10_550, 10_700],
    );

    assert_eq!(report.peak_bytes(), 10_700);
    assert_eq!(report.net_growth_bytes(), 700);
}

#[test]
fn leak_budget_report_contract_fails_gate_on_budget_exceeded() {
    let report = css::LeakBudgetReport::from_time_window(
        css::LeakBudget::global_bytes(1_024),
        vec![2_000, 2_800, 3_200],
    );

    assert_eq!(report.status(), css::LeakBudgetStatus::Exceeded);
    assert_eq!(report.primary_failure_class(), css::FailureClass::LeakBudgetExceeded);
}
