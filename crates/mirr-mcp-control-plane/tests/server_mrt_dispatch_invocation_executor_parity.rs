use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_executor::{
    enforce_token_quota, execute_mrt_dispatch_invocation_with_runner, resolve_runtime_token,
    with_token_concurrency_limit, MrtDispatchExecutionConfig, MrtDispatchExecutionError,
    MrtDispatchInvocationRunner, MrtDispatchRunnerOutcome, MrtRuntimeAdmissionConfig,
    MrtRuntimeAdmissionError, MrtRuntimeAdmissionState, TokenQuotaState,
};
use mirr_mcp_control_plane::server_rewrite::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;

#[derive(Clone)]
struct FakeRunner {
    outcome: MrtDispatchRunnerOutcome,
}

impl MrtDispatchInvocationRunner for FakeRunner {
    fn run(
        &self,
        _plan: &MrtDispatchInvocationPlan,
        _config: &MrtDispatchExecutionConfig,
    ) -> MrtDispatchRunnerOutcome {
        self.outcome.clone()
    }
}

#[test]
fn execute_success_preserves_stdout_stderr_and_exit_code() {
    let plan = MrtDispatchInvocationPlan::new(vec!["run".to_owned()]);
    let config = MrtDispatchExecutionConfig::default();
    let runner = FakeRunner {
        outcome: MrtDispatchRunnerOutcome {
            stdout: "ok-json".to_owned(),
            stderr: "diagnostic".to_owned(),
            status: Some(0),
            error_message: None,
        },
    };

    let result = execute_mrt_dispatch_invocation_with_runner(&runner, &plan, &config)
        .expect("status 0 should return execution result");
    assert_eq!(result.stdout, "ok-json");
    assert_eq!(result.stderr, "diagnostic");
    assert_eq!(result.exit_code, 0);
}

#[test]
fn execute_nonzero_exit_returns_stable_error_pattern() {
    let plan = MrtDispatchInvocationPlan::new(vec!["run".to_owned()]);
    let config = MrtDispatchExecutionConfig::default();
    let runner = FakeRunner {
        outcome: MrtDispatchRunnerOutcome {
            stdout: "partial".to_owned(),
            stderr: "tool failed".to_owned(),
            status: Some(42),
            error_message: None,
        },
    };

    let err = execute_mrt_dispatch_invocation_with_runner(&runner, &plan, &config)
        .expect_err("nonzero exit must fail closed");
    match err {
        MrtDispatchExecutionError::NonZeroExit { message, stdout, stderr, exit_code } => {
            assert_eq!(message, "mrt_exec_failed_exit_42");
            assert_eq!(stdout, "partial");
            assert_eq!(stderr, "tool failed");
            assert_eq!(exit_code, 42);
        }
        other => panic!("unexpected error variant: {:?}", other),
    }
}

#[test]
fn execute_spawn_error_preserves_status_stdout_stderr() {
    let plan = MrtDispatchInvocationPlan::new(vec!["run".to_owned()]);
    let config = MrtDispatchExecutionConfig::default();
    let runner = FakeRunner {
        outcome: MrtDispatchRunnerOutcome {
            stdout: "captured out".to_owned(),
            stderr: "captured err".to_owned(),
            status: Some(127),
            error_message: Some("spawnSync cargo ENOENT".to_owned()),
        },
    };

    let err = execute_mrt_dispatch_invocation_with_runner(&runner, &plan, &config)
        .expect_err("spawn failure must be propagated");
    match err {
        MrtDispatchExecutionError::SpawnSyncError { message, stdout, stderr, status } => {
            assert_eq!(message, "spawnSync cargo ENOENT");
            assert_eq!(stdout, "captured out");
            assert_eq!(stderr, "captured err");
            assert_eq!(status, Some(127));
        }
        other => panic!("unexpected error variant: {:?}", other),
    }
}

#[test]
fn token_quota_windowing_matches_ts_state_progression() {
    let config = MrtRuntimeAdmissionConfig {
        max_concurrent_per_key: 2,
        max_requests_per_token: 2,
        token_quota_window_ms: 100,
    };
    let mut state = MrtRuntimeAdmissionState::default();

    enforce_token_quota("token-a", 1_000, &mut state, &config)
        .expect("first request in window should pass");
    enforce_token_quota("token-a", 1_050, &mut state, &config)
        .expect("second request in window should pass");

    let third = enforce_token_quota("token-a", 1_090, &mut state, &config)
        .expect_err("third request in same window should fail");
    assert_eq!(third, MrtRuntimeAdmissionError::TokenQuotaExceeded);

    enforce_token_quota("token-a", 1_150, &mut state, &config)
        .expect("window rollover should reset count");
    let quota = state.token_quota.get("token-a").expect("quota state should persist for token");
    assert_eq!(quota.window_start_ms, 1_150);
    assert_eq!(quota.count, 1);
}

#[test]
fn concurrency_gate_rolls_back_on_reject_and_decrements_on_exit() {
    let config = MrtRuntimeAdmissionConfig {
        max_concurrent_per_key: 1,
        max_requests_per_token: 30,
        token_quota_window_ms: 60_000,
    };
    let mut state = MrtRuntimeAdmissionState::default();
    state.concurrency.insert("token-b".to_owned(), 1);

    let rejected = with_token_concurrency_limit("token-b", 0, &mut state, &config, || Ok(()))
        .expect_err("overflowing concurrency should reject fail-closed");
    assert_eq!(rejected, MrtRuntimeAdmissionError::ConcurrencyLimitExceeded);
    assert_eq!(state.concurrency.get("token-b"), Some(&1));

    state.concurrency.remove("token-b");
    let done = with_token_concurrency_limit("token-b", 10, &mut state, &config, || Ok("ok"))
        .expect("single in-flight operation should execute");
    assert_eq!(done, "ok");
    assert_eq!(state.concurrency.get("token-b"), None);
}

#[test]
fn quota_check_happens_before_concurrency_increment() {
    let config = MrtRuntimeAdmissionConfig {
        max_concurrent_per_key: 1,
        max_requests_per_token: 1,
        token_quota_window_ms: 60_000,
    };
    let mut state = MrtRuntimeAdmissionState::default();
    state.concurrency.insert("token-c".to_owned(), 1);
    state
        .token_quota
        .insert("token-c".to_owned(), TokenQuotaState { window_start_ms: 0, count: 1 });

    let err = with_token_concurrency_limit("token-c", 10, &mut state, &config, || Ok(()))
        .expect_err("quota overflow should reject before concurrency increment");
    assert_eq!(err, MrtRuntimeAdmissionError::TokenQuotaExceeded);
    assert_eq!(state.concurrency.get("token-c"), Some(&1));
    assert_eq!(state.token_quota.get("token-c").map(|value| value.count), Some(2));
}

#[test]
fn runtime_token_resolution_matches_ts_fallback_behavior() {
    assert_eq!(resolve_runtime_token(None), "__anon");
    assert_eq!(resolve_runtime_token(Some("")), "__anon");
    assert_eq!(resolve_runtime_token(Some("token-77")), "token-77");
}
