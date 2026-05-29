#![forbid(unsafe_code)]

use nasa_rust_project::mrt_host as mrt;

fn runtime_with_limits(stdout_limit: usize, stderr_limit: usize) -> mrt::HostRuntime {
    mrt::HostRuntime::builder()
        .shadow_mode(mrt::ShadowMode::DualExecute)
        .primary_engine(mrt::EngineKind::TypeScript)
        .mismatch_policy(mrt::MismatchPolicy::LogAndReturnTs)
        .rollback_switch(mrt::RollbackSwitch::in_memory(false))
        .output_envelope(mrt::OutputEnvelope::bounded(stdout_limit, stderr_limit))
        .compare_rules(
            mrt::CompareRules::deterministic("wave2-rules-v1")
                .semantic(mrt::SemanticParity::CanonicalJson)
                .bytes(mrt::ByteParity::Exact)
                .ignore_fields(vec!["timestamp_ms", "nonce"]),
        )
        .build_for_test()
}

fn sample_call() -> mrt::ToolCall {
    mrt::ToolCall::new("mrt_general_ci")
        .with_correlation_id("wave2-call-001")
        .with_argument("schema_version", "1")
        .with_argument("tool", "mrt_general_ci")
}

fn semantic(json: &[u8]) -> mrt::SemanticPayload {
    mrt::SemanticPayload::from_json_bytes(json.to_vec()).expect("fixture json must be valid")
}

fn engine_result(
    semantic_payload: mrt::SemanticPayload,
    bytes: &[u8],
    stdout: &[u8],
    stderr: &[u8],
) -> mrt::EngineResult {
    mrt::EngineResult::success()
        .with_semantic(semantic_payload)
        .with_bytes(bytes.to_vec())
        .with_stdout(stdout.to_vec())
        .with_stderr(stderr.to_vec())
}

#[test]
fn shadow_mode_dual_execution_invokes_ts_and_wasm_once_each() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"status":"ok","engine":"ts"}"#),
        br#"{"status":"ok","engine":"ts"}"#,
        b"ts-stdout",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"status":"ok","engine":"wasm"}"#),
        br#"{"status":"ok","engine":"wasm"}"#,
        b"wasm-stdout",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.trace().typescript_invocations(), 1);
    assert_eq!(outcome.trace().wasm_invocations(), 1);
    assert_eq!(outcome.execution_path(), mrt::ExecutionPath::ShadowDual);
}

#[test]
fn shadow_mode_marks_typescript_as_primary_response_engine() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"result":"ts-primary"}"#),
        br#"{"result":"ts-primary"}"#,
        b"ts-primary-stdout",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"result":"wasm-secondary"}"#),
        br#"{"result":"wasm-secondary"}"#,
        b"wasm-secondary-stdout",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.response().engine(), mrt::EngineKind::TypeScript);
    assert_eq!(outcome.response().body_bytes(), br#"{"result":"ts-primary"}"#.to_vec());
}

#[test]
fn shadow_mode_attaches_wasm_shadow_result_when_both_engines_succeed() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"status":"ok","from":"ts"}"#),
        br#"{"status":"ok","from":"ts"}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"status":"ok","from":"wasm"}"#),
        br#"{"status":"ok","from":"wasm"}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    let shadow = outcome.shadow_result().expect("shadow outcome should include wasm result");
    assert_eq!(shadow.engine(), mrt::EngineKind::Wasm);
    assert_eq!(shadow.body_bytes(), br#"{"status":"ok","from":"wasm"}"#.to_vec());
}

#[test]
fn shadow_mode_uses_single_correlation_id_for_ts_and_wasm_runs() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"ok":true}"#),
        br#"{"ok":true}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"ok":true}"#),
        br#"{"ok":true}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.trace().typescript_correlation_id(), "wave2-call-001");
    assert_eq!(outcome.trace().wasm_correlation_id(), "wave2-call-001");
}

#[test]
fn semantic_parity_accepts_canonical_json_equivalence() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"a":1,"b":2}"#),
        br#"{"a":1,"b":2}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"b":2,"a":1}"#),
        br#"{"b":2,"a":1}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert!(outcome.parity().semantic_match());
}

#[test]
fn semantic_parity_rejects_type_divergence_even_with_equal_rendered_text() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"value":1}"#),
        br#"{"value":1}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"value":"1"}"#),
        br#"{"value":"1"}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert!(!outcome.parity().semantic_match());
    assert_eq!(outcome.parity().classification(), mrt::ParityClassification::SemanticMismatch);
}

#[test]
fn byte_parity_requires_exact_payload_identity() {
    let runtime = runtime_with_limits(256, 256);
    let payload = &[0xAA, 0xBB, 0xCC, 0xDD];
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"raw":"same"}"#),
        payload,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"raw":"same"}"#),
        payload,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert!(outcome.parity().byte_match());
}

#[test]
fn byte_parity_reports_first_divergent_offset_for_diagnostics() {
    let runtime = runtime_with_limits(256, 256);
    let ts_bytes = &[0x10, 0x11, 0x12, 0x13];
    let wasm_bytes = &[0x10, 0x11, 0x99, 0x13];
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"raw":"diverge"}"#),
        ts_bytes,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"raw":"diverge"}"#),
        wasm_bytes,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert!(!outcome.parity().byte_match());
    assert_eq!(outcome.parity().byte_mismatch().first_offset(), Some(2));
}

#[test]
fn mismatch_logging_records_event_and_returns_safe_typescript_response() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"safe":true,"engine":"ts"}"#),
        br#"{"safe":true,"engine":"ts"}"#,
        b"safe-ts",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"safe":false,"engine":"wasm"}"#),
        br#"{"safe":false,"engine":"wasm"}"#,
        b"unsafe-wasm",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.response().engine(), mrt::EngineKind::TypeScript);
    assert_eq!(outcome.mismatch_events().len(), 1);
}

#[test]
fn mismatch_logging_includes_semantic_and_byte_statuses() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"x":1}"#),
        br#"{"x":1}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"x":2}"#),
        br#"{"x":2}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    let event = &outcome.mismatch_events()[0];
    assert_eq!(event.semantic_status(), mrt::ParityStatus::Mismatch);
    assert_eq!(event.byte_status(), mrt::ParityStatus::Mismatch);
}

#[test]
fn mismatch_logging_is_side_effect_only_and_never_mutates_ts_payload() {
    let runtime = runtime_with_limits(256, 256);
    let ts_primary = br#"{"result":"safe-ts"}"#;
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(ts_primary),
        ts_primary,
        b"safe-stdout",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"result":"different-wasm"}"#),
        br#"{"result":"different-wasm"}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.mismatch_events().len(), 1);
    assert_eq!(outcome.response().body_bytes(), ts_primary.to_vec());
}

#[test]
fn rollback_switch_disables_wasm_engine_when_flag_is_on() {
    let runtime = runtime_with_limits(256, 256).with_rollback_state(true);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"mode":"rollback"}"#),
        br#"{"mode":"rollback"}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"mode":"shadow"}"#),
        br#"{"mode":"shadow"}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("rollback invocation should succeed");

    assert_eq!(outcome.trace().wasm_invocations(), 0);
    assert_eq!(outcome.execution_path(), mrt::ExecutionPath::TsOnlyRollback);
}

#[test]
fn rollback_switch_reenables_dual_execution_after_flag_is_off() {
    let mut runtime = runtime_with_limits(256, 256);
    runtime.set_rollback_state(true);
    runtime.set_rollback_state(false);

    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"mode":"ts"}"#),
        br#"{"mode":"ts"}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"mode":"wasm"}"#),
        br#"{"mode":"wasm"}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.trace().wasm_invocations(), 1);
    assert_eq!(outcome.execution_path(), mrt::ExecutionPath::ShadowDual);
}

#[test]
fn rollback_switch_precedence_prefers_runtime_flag_over_request_hint() {
    let runtime = runtime_with_limits(256, 256).with_rollback_state(true);
    let call = sample_call().with_shadow_hint(true);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"status":"ts-only"}"#),
        br#"{"status":"ts-only"}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"status":"should-not-run"}"#),
        br#"{"status":"should-not-run"}"#,
        b"",
        b"",
    ));

    let outcome = runtime.invoke_with(call, ts, wasm).expect("rollback invocation should succeed");

    assert_eq!(outcome.trace().wasm_invocations(), 0);
    assert_eq!(outcome.rollback_reason(), Some(mrt::RollbackReason::RuntimeFlag));
}

#[test]
fn output_envelope_truncates_stdout_to_configured_bound() {
    let runtime = runtime_with_limits(4, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"stream":"stdout"}"#),
        br#"{"stream":"stdout"}"#,
        b"123456789",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"stream":"stdout"}"#),
        br#"{"stream":"stdout"}"#,
        b"123456789",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.response().stdout_bytes().len(), 4);
    assert!(outcome.response().stdout_truncated());
}

#[test]
fn output_envelope_truncates_stderr_to_configured_bound() {
    let runtime = runtime_with_limits(256, 3);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"stream":"stderr"}"#),
        br#"{"stream":"stderr"}"#,
        b"",
        b"abcdef",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"stream":"stderr"}"#),
        br#"{"stream":"stderr"}"#,
        b"",
        b"abcdef",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.response().stderr_bytes().len(), 3);
    assert!(outcome.response().stderr_truncated());
}

#[test]
fn output_envelope_exposes_truncation_flags_and_limit_metadata() {
    let runtime = runtime_with_limits(5, 2);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"bounded":true}"#),
        br#"{"bounded":true}"#,
        b"abcdefgh",
        b"wxyz",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"bounded":true}"#),
        br#"{"bounded":true}"#,
        b"abcdefgh",
        b"wxyz",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.response().output_limit_stdout_bytes(), 5);
    assert_eq!(outcome.response().output_limit_stderr_bytes(), 2);
    assert!(outcome.response().stdout_truncated());
    assert!(outcome.response().stderr_truncated());
}

#[test]
fn deterministic_compare_ignores_declared_nondeterministic_fields() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"status":"ok","timestamp_ms":100,"nonce":"a"}"#),
        br#"{"status":"ok","timestamp_ms":100,"nonce":"a"}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"status":"ok","timestamp_ms":200,"nonce":"b"}"#),
        br#"{"status":"ok","timestamp_ms":200,"nonce":"b"}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert!(outcome.parity().semantic_match());
}

#[test]
fn deterministic_compare_returns_stable_decision_across_repeated_runs() {
    let runtime = runtime_with_limits(256, 256);

    let first = runtime
        .invoke_with(
            sample_call(),
            mrt::MockEngine::typescript().returns(engine_result(
                semantic(br#"{"stable":true,"value":7}"#),
                br#"{"stable":true,"value":7}"#,
                b"",
                b"",
            )),
            mrt::MockEngine::wasm().returns(engine_result(
                semantic(br#"{"stable":true,"value":7}"#),
                br#"{"stable":true,"value":7}"#,
                b"",
                b"",
            )),
        )
        .expect("first invocation should succeed");

    let second = runtime
        .invoke_with(
            sample_call(),
            mrt::MockEngine::typescript().returns(engine_result(
                semantic(br#"{"stable":true,"value":7}"#),
                br#"{"stable":true,"value":7}"#,
                b"",
                b"",
            )),
            mrt::MockEngine::wasm().returns(engine_result(
                semantic(br#"{"stable":true,"value":7}"#),
                br#"{"stable":true,"value":7}"#,
                b"",
                b"",
            )),
        )
        .expect("second invocation should succeed");

    assert_eq!(first.parity().decision(), second.parity().decision());
}

#[test]
fn deterministic_compare_uses_explicit_rule_version_for_reproducibility() {
    let runtime = runtime_with_limits(256, 256);
    let ts = mrt::MockEngine::typescript().returns(engine_result(
        semantic(br#"{"versioned":true}"#),
        br#"{"versioned":true}"#,
        b"",
        b"",
    ));
    let wasm = mrt::MockEngine::wasm().returns(engine_result(
        semantic(br#"{"versioned":true}"#),
        br#"{"versioned":true}"#,
        b"",
        b"",
    ));

    let outcome =
        runtime.invoke_with(sample_call(), ts, wasm).expect("shadow invocation should succeed");

    assert_eq!(outcome.parity().rule_version(), "wave2-rules-v1");
    assert!(outcome.parity().deterministic());
}
