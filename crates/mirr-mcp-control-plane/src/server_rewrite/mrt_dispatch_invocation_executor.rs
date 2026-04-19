#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use super::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;

pub const DEFAULT_EXEC_TIMEOUT_MS: u64 = 120_000;
pub const DEFAULT_EXEC_MAX_OUTPUT_BYTES: usize = 65_536;
pub const DEFAULT_MAX_CONCURRENT_PER_KEY: u32 = 2;
pub const DEFAULT_MAX_REQUESTS_PER_TOKEN: u32 = 30;
pub const DEFAULT_TOKEN_QUOTA_WINDOW_MS: u64 = 60_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchExecutionConfig {
    pub command: String,
    pub workspace_root: String,
    pub timeout_ms: u64,
    pub max_output_bytes: usize,
    pub dual_run_enabled: bool,
}

impl Default for MrtDispatchExecutionConfig {
    fn default() -> Self {
        Self {
            command: "cargo".to_owned(),
            workspace_root: ".".to_owned(),
            timeout_ms: DEFAULT_EXEC_TIMEOUT_MS,
            max_output_bytes: DEFAULT_EXEC_MAX_OUTPUT_BYTES,
            dual_run_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MrtDispatchExecutionError {
    SpawnSyncError { message: String, stdout: String, stderr: String, status: Option<i32> },
    NonZeroExit { message: String, stdout: String, stderr: String, exit_code: i32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchRunnerOutcome {
    pub stdout: String,
    pub stderr: String,
    pub status: Option<i32>,
    pub error_message: Option<String>,
}

pub trait MrtDispatchInvocationRunner {
    fn run(
        &self,
        plan: &MrtDispatchInvocationPlan,
        config: &MrtDispatchExecutionConfig,
    ) -> MrtDispatchRunnerOutcome;
}

pub struct CargoProcessRunner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtRuntimeAdmissionConfig {
    pub max_concurrent_per_key: u32,
    pub max_requests_per_token: u32,
    pub token_quota_window_ms: u64,
}

impl Default for MrtRuntimeAdmissionConfig {
    fn default() -> Self {
        Self {
            max_concurrent_per_key: DEFAULT_MAX_CONCURRENT_PER_KEY,
            max_requests_per_token: DEFAULT_MAX_REQUESTS_PER_TOKEN,
            token_quota_window_ms: DEFAULT_TOKEN_QUOTA_WINDOW_MS,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenQuotaState {
    pub window_start_ms: u64,
    pub count: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MrtRuntimeAdmissionState {
    pub concurrency: BTreeMap<String, u32>,
    pub token_quota: BTreeMap<String, TokenQuotaState>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MrtRuntimeAdmissionError {
    InvalidRuntimeLimits,
    TokenQuotaExceeded,
    ConcurrencyLimitExceeded,
}

fn runtime_limits_are_valid(config: &MrtRuntimeAdmissionConfig) -> bool {
    config.max_concurrent_per_key > 0
        && config.max_requests_per_token > 0
        && config.token_quota_window_ms > 0
}

pub fn resolve_runtime_token(api_key: Option<&str>) -> String {
    match api_key {
        Some(value) if !value.is_empty() => value.to_owned(),
        _ => "__anon".to_owned(),
    }
}

pub fn enforce_token_quota(
    token: &str,
    now_ms: u64,
    state: &mut MrtRuntimeAdmissionState,
    config: &MrtRuntimeAdmissionConfig,
) -> Result<(), MrtRuntimeAdmissionError> {
    if !runtime_limits_are_valid(config) {
        return Err(MrtRuntimeAdmissionError::InvalidRuntimeLimits);
    }

    let mut quota = state
        .token_quota
        .get(token)
        .cloned()
        .unwrap_or(TokenQuotaState { window_start_ms: now_ms, count: 0 });

    if now_ms.saturating_sub(quota.window_start_ms) >= config.token_quota_window_ms {
        quota.window_start_ms = now_ms;
        quota.count = 0;
    }

    quota.count = quota.count.saturating_add(1);
    state.token_quota.insert(token.to_owned(), quota.clone());

    if quota.count > config.max_requests_per_token {
        return Err(MrtRuntimeAdmissionError::TokenQuotaExceeded);
    }

    Ok(())
}

pub fn read_token_quota_state(
    token: &str,
    state: &MrtRuntimeAdmissionState,
) -> Option<TokenQuotaState> {
    state.token_quota.get(token).cloned()
}

pub fn with_token_concurrency_limit<T, F>(
    token: &str,
    now_ms: u64,
    state: &mut MrtRuntimeAdmissionState,
    config: &MrtRuntimeAdmissionConfig,
    operation: F,
) -> Result<T, MrtRuntimeAdmissionError>
where
    F: FnOnce() -> Result<T, MrtRuntimeAdmissionError>,
{
    if !runtime_limits_are_valid(config) {
        return Err(MrtRuntimeAdmissionError::InvalidRuntimeLimits);
    }

    enforce_token_quota(token, now_ms, state, config)?;

    let current = state.concurrency.get(token).copied().unwrap_or(0);
    let next = current.saturating_add(1);
    state.concurrency.insert(token.to_owned(), next);

    if next > config.max_concurrent_per_key {
        let decremented = next.saturating_sub(1);
        if decremented == 0 {
            state.concurrency.remove(token);
        } else {
            state.concurrency.insert(token.to_owned(), decremented);
        }
        return Err(MrtRuntimeAdmissionError::ConcurrencyLimitExceeded);
    }

    let result = operation();

    let after = state.concurrency.get(token).copied().unwrap_or(1).saturating_sub(1);
    if after == 0 {
        state.concurrency.remove(token);
    } else {
        state.concurrency.insert(token.to_owned(), after);
    }

    result
}

fn clip_output(raw: String, max_output_bytes: usize) -> String {
    if raw.len() <= max_output_bytes {
        return raw;
    }

    let mut clipped = raw;
    while clipped.len() > max_output_bytes {
        clipped.pop();
    }

    clipped
}

impl MrtDispatchInvocationRunner for CargoProcessRunner {
    fn run(
        &self,
        plan: &MrtDispatchInvocationPlan,
        config: &MrtDispatchExecutionConfig,
    ) -> MrtDispatchRunnerOutcome {
        let mut command = Command::new(&config.command);
        command
            .args(&plan.args)
            .current_dir(&config.workspace_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let spawn_result = command.spawn();
        let mut child = match spawn_result {
            Ok(child) => child,
            Err(err) => {
                return MrtDispatchRunnerOutcome {
                    stdout: String::new(),
                    stderr: String::new(),
                    status: None,
                    error_message: Some(err.to_string()),
                }
            }
        };

        if let Some(stdin_data) = &plan.stdin_data {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(stdin_data.as_bytes());
            }
        }

        let timeout = Duration::from_millis(config.timeout_ms);
        let started = Instant::now();

        loop {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => {
                    if started.elapsed() >= timeout {
                        let _ = child.kill();
                        let output = child.wait_with_output();
                        return match output {
                            Ok(out) => MrtDispatchRunnerOutcome {
                                stdout: clip_output(
                                    String::from_utf8_lossy(&out.stdout).to_string(),
                                    config.max_output_bytes,
                                ),
                                stderr: clip_output(
                                    String::from_utf8_lossy(&out.stderr).to_string(),
                                    config.max_output_bytes,
                                ),
                                status: out.status.code(),
                                error_message: Some("mrt_exec_timeout".to_owned()),
                            },
                            Err(err) => MrtDispatchRunnerOutcome {
                                stdout: String::new(),
                                stderr: String::new(),
                                status: None,
                                error_message: Some(err.to_string()),
                            },
                        };
                    }
                    thread::sleep(Duration::from_millis(5));
                }
                Err(err) => {
                    return MrtDispatchRunnerOutcome {
                        stdout: String::new(),
                        stderr: String::new(),
                        status: None,
                        error_message: Some(err.to_string()),
                    }
                }
            }
        }

        match child.wait_with_output() {
            Ok(output) => MrtDispatchRunnerOutcome {
                stdout: clip_output(
                    String::from_utf8_lossy(&output.stdout).to_string(),
                    config.max_output_bytes,
                ),
                stderr: clip_output(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                    config.max_output_bytes,
                ),
                status: output.status.code(),
                error_message: None,
            },
            Err(err) => MrtDispatchRunnerOutcome {
                stdout: String::new(),
                stderr: String::new(),
                status: None,
                error_message: Some(err.to_string()),
            },
        }
    }
}

pub fn execute_mrt_dispatch_invocation_with_runner<R: MrtDispatchInvocationRunner>(
    runner: &R,
    plan: &MrtDispatchInvocationPlan,
    config: &MrtDispatchExecutionConfig,
) -> Result<MrtDispatchExecutionResult, MrtDispatchExecutionError> {
    let outcome = runner.run(plan, config);

    if let Some(message) = outcome.error_message {
        return Err(MrtDispatchExecutionError::SpawnSyncError {
            message,
            stdout: outcome.stdout,
            stderr: outcome.stderr,
            status: outcome.status,
        });
    }

    let exit_code = outcome.status.unwrap_or(1);
    if exit_code != 0 {
        return Err(MrtDispatchExecutionError::NonZeroExit {
            message: format!("mrt_exec_failed_exit_{}", exit_code),
            stdout: outcome.stdout,
            stderr: outcome.stderr,
            exit_code,
        });
    }

    Ok(MrtDispatchExecutionResult { stdout: outcome.stdout, stderr: outcome.stderr, exit_code })
}

pub fn execute_mrt_dispatch_invocation(
    plan: &MrtDispatchInvocationPlan,
    config: &MrtDispatchExecutionConfig,
) -> Result<MrtDispatchExecutionResult, MrtDispatchExecutionError> {
    execute_mrt_dispatch_invocation_with_runner(&CargoProcessRunner, plan, config)
}
