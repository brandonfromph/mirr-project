#![forbid(unsafe_code)]

use nasa_rust_project::zero_debt_closeout::{
    CompatibilityRouteDisablementContract, CutoverDecision, DeferredScope,
    DeferredScopeRollbackMetadata, EnforcementWindow, LegacyPathRemovalManifest, LegacyRoute,
    RemovalMode, RollbackMetadata, RouteResolution, ShadowModeCutoverPlanner, ShadowSample,
    ZeroDebtCloseoutReport, ZeroDebtInvariant,
};

#[test]
fn shadow_planner_requires_full_stability_window_before_cutover() {
    let mut planner = ShadowModeCutoverPlanner::new(EnforcementWindow::new(5, 0.999));

    for epoch in 0..4 {
        planner.record_shadow_sample(ShadowSample::parity_match(epoch));
    }

    let decision = planner.evaluate_cutover(4);
    assert!(matches!(decision, CutoverDecision::RemainInShadow));
}

#[test]
fn shadow_planner_cuts_over_when_window_and_threshold_are_met() {
    let mut planner = ShadowModeCutoverPlanner::new(EnforcementWindow::new(5, 0.999));

    for epoch in 0..5 {
        planner.record_shadow_sample(ShadowSample::parity_match(epoch));
    }

    let decision = planner.evaluate_cutover(5);
    assert!(matches!(
        decision,
        CutoverDecision::CutoverAt {
            epoch: 5,
            rollback_plan_id
        } if !rollback_plan_id.is_empty()
    ));
}

#[test]
fn shadow_planner_resets_stability_after_any_parity_regression() {
    let mut planner = ShadowModeCutoverPlanner::new(EnforcementWindow::new(5, 0.999));

    for epoch in 0..3 {
        planner.record_shadow_sample(ShadowSample::parity_match(epoch));
    }
    planner.record_shadow_sample(ShadowSample::parity_mismatch(3));
    planner.record_shadow_sample(ShadowSample::parity_match(4));
    planner.record_shadow_sample(ShadowSample::parity_match(5));

    let decision = planner.evaluate_cutover(6);
    assert!(matches!(decision, CutoverDecision::RemainInShadow));
}

#[test]
fn shadow_planner_cutover_decision_is_deterministic_for_same_epoch() {
    let mut planner = ShadowModeCutoverPlanner::new(EnforcementWindow::new(3, 1.0));
    planner.record_shadow_sample(ShadowSample::parity_match(0));
    planner.record_shadow_sample(ShadowSample::parity_match(1));
    planner.record_shadow_sample(ShadowSample::parity_match(2));

    let first = planner.evaluate_cutover(3);
    let second = planner.evaluate_cutover(3);
    assert_eq!(first, second);
}

#[test]
fn compatibility_contract_disables_legacy_route_by_default_after_cutover() {
    let contract = CompatibilityRouteDisablementContract::strict_default()
        .with_legacy_route("mrt_execute")
        .disable_on_cutover();

    let resolution = contract.resolve("mrt_execute", true, 10);
    assert!(matches!(
        resolution,
        RouteResolution::Disabled {
            route,
            status_code: 410,
            ..
        } if route == "mrt_execute"
    ));
}

#[test]
fn compatibility_contract_allows_explicit_temporary_reenable_with_ttl() {
    let mut contract = CompatibilityRouteDisablementContract::strict_default()
        .with_legacy_route("mrt_execute")
        .disable_on_cutover();

    contract
        .request_temporary_reenable("mrt_execute", 3, "incident-42")
        .expect("temporary re-enable with bounded ttl should be allowed");

    let within_ttl = contract.resolve("mrt_execute", true, 11);
    let after_ttl = contract.resolve("mrt_execute", true, 15);

    assert!(matches!(within_ttl, RouteResolution::Allowed { .. }));
    assert!(matches!(after_ttl, RouteResolution::Disabled { .. }));
}

#[test]
fn compatibility_contract_rejects_unbounded_temporary_reenable() {
    let mut contract = CompatibilityRouteDisablementContract::strict_default()
        .with_legacy_route("mrt_execute")
        .disable_on_cutover();

    let result = contract.request_temporary_reenable("mrt_execute", u32::MAX, "incident-99");
    assert!(result.is_err());
}

#[test]
fn compatibility_contract_records_audit_event_for_each_route_resolution() {
    let contract = CompatibilityRouteDisablementContract::strict_default()
        .with_legacy_route("mrt_execute")
        .disable_on_cutover();

    let _ = contract.resolve("mrt_execute", true, 10);
    let _ = contract.resolve("mrt_execute", true, 11);

    let audit = contract.route_audit("mrt_execute");
    assert_eq!(audit.len(), 2);
    assert!(audit.iter().all(|event| event.route == "mrt_execute"));
}

#[test]
fn legacy_manifest_rejects_duplicate_legacy_routes() {
    let result = LegacyPathRemovalManifest::from_routes(vec![
        LegacyRoute::new("/mrt_execute", "/mrt_general_ci"),
        LegacyRoute::new("/mrt_execute", "/mrt_general_ci_compile"),
    ]);

    assert!(result.is_err());
}

#[test]
fn legacy_manifest_requires_replacement_route_for_each_removed_path() {
    let result = LegacyPathRemovalManifest::from_routes(vec![LegacyRoute::without_replacement(
        "/mrt_execute",
    )]);

    assert!(result.is_err());
}

#[test]
fn legacy_manifest_becomes_immutable_after_irreversible_freeze() {
    let mut manifest =
        LegacyPathRemovalManifest::from_routes(vec![LegacyRoute::new("/legacy/a", "/mrt_audit")])
            .expect("seed manifest should be valid");

    manifest.freeze(RemovalMode::Irreversible).expect("freeze should succeed exactly once");

    let add_result = manifest.add_route(LegacyRoute::new("/legacy/b", "/mrt_wave_apply"));
    assert!(add_result.is_err());
}

#[test]
fn legacy_manifest_exports_removed_routes_in_deterministic_order() {
    let manifest = LegacyPathRemovalManifest::from_routes(vec![
        LegacyRoute::new("/legacy/z", "/mrt_wave_apply"),
        LegacyRoute::new("/legacy/a", "/mrt_audit"),
        LegacyRoute::new("/legacy/m", "/mrt_general_ci"),
    ])
    .expect("manifest should normalize route ordering");

    let ordered = manifest.ordered_removed_routes();
    assert_eq!(ordered, vec!["/legacy/a", "/legacy/m", "/legacy/z"]);
}

#[test]
fn closeout_report_fails_when_any_zero_debt_invariant_is_unmet() {
    let mut report = ZeroDebtCloseoutReport::new("proposal-104");
    report.record_invariant(ZeroDebtInvariant::NoWrapperFunctions, true);
    report.record_invariant(ZeroDebtInvariant::NoDeprecatedAliases, true);
    report.record_invariant(ZeroDebtInvariant::NoBackwardCompatShims, false);

    assert!(!report.is_closeout_ready());
}

#[test]
fn closeout_report_requires_planner_route_and_manifest_alignment() {
    let planner = ShadowModeCutoverPlanner::empty_ready_state();
    let contract = CompatibilityRouteDisablementContract::strict_default();
    let manifest = LegacyPathRemovalManifest::empty();

    let report =
        ZeroDebtCloseoutReport::from_components("proposal-104", planner, contract, manifest);
    assert!(report.validate_component_alignment().is_ok());
}

#[test]
fn closeout_report_requires_zero_debt_score_before_finalize() {
    let mut report = ZeroDebtCloseoutReport::new("proposal-104");
    report.set_debt_score(1);

    let result = report.finalize();
    assert!(result.is_err());
}

#[test]
fn closeout_report_requires_nonempty_evidence_ids_for_each_section() {
    let report = ZeroDebtCloseoutReport::new("proposal-104")
        .with_section_evidence("planner", "evidence-1")
        .with_section_evidence("routes", "evidence-2")
        .with_section_evidence("manifest", "evidence-3")
        .with_section_evidence("metadata", "evidence-4");

    assert!(report.section_evidence_ids().values().all(|value| !value.is_empty()));
}

#[test]
fn deferred_scope_requires_owner_and_reason() {
    let deferred = DeferredScope::new("full-daemon-cutover", "", "");
    assert!(deferred.is_err());
}

#[test]
fn rollback_metadata_rejects_unbounded_step_count() {
    let steps: Vec<String> = (0..257).map(|idx| format!("step-{idx}")).collect();
    let result = RollbackMetadata::new("rb-unbounded", steps);

    assert!(result.is_err());
}

#[test]
fn metadata_model_roundtrip_preserves_deterministic_hash() {
    let deferred =
        DeferredScope::new("wasm-host-cutover", "wave-9", "runtime backend staging not complete")
            .expect("deferred scope should be valid");
    let rollback = RollbackMetadata::new(
        "rb-104",
        vec!["restore-route-map".to_string(), "restore-cutover-guard".to_string()],
    )
    .expect("rollback metadata should be valid");

    let metadata = DeferredScopeRollbackMetadata::new(vec![deferred], rollback)
        .expect("metadata model should be constructible");

    let encoded = metadata.to_canonical_bytes();
    let decoded = DeferredScopeRollbackMetadata::from_canonical_bytes(&encoded)
        .expect("roundtrip decode should succeed");

    assert_eq!(metadata.deterministic_hash(), decoded.deterministic_hash());
}

#[test]
fn metadata_model_requires_one_to_one_scope_to_rollback_strategy() {
    let deferred_scopes = vec![
        DeferredScope::new("full-daemon-cutover", "wave-9", "daemon runtime landing deferred")
            .expect("first deferred scope should be valid"),
        DeferredScope::new("wasm-host-cutover", "wave-9", "wasm host migration deferred")
            .expect("second deferred scope should be valid"),
    ];

    let rollback = RollbackMetadata::new(
        "rb-104",
        vec!["restore-router-state".to_string(), "restore-compat-policy".to_string()],
    )
    .expect("rollback metadata should be valid")
    .with_strategy_for("full-daemon-cutover", "daemon-rollback")
    .with_strategy_for("wasm-host-cutover", "wasm-rollback");

    let metadata = DeferredScopeRollbackMetadata::new(deferred_scopes, rollback)
        .expect("metadata model should be constructible");

    assert!(metadata.validate_one_to_one_scope_to_strategy().is_ok());
}
