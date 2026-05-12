#![forbid(unsafe_code)]

use crate::envelope::{
    StableError, CONCURRENCY_LIMIT_ERROR, MRT_EXECUTION_ERROR, SCHEMA_VALIDATION_ERROR,
    TOKEN_QUOTA_LIMIT_ERROR,
};

use super::mrt_dispatch_invocation_executor::{
    MrtDispatchExecutionError, MrtDispatchExecutionResult,
};
use super::mrt_dispatch_invocation_input::InvocationInputBody;
use super::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;
use super::rpc_role_failure_envelope::role_failure_to_envelope;
use super::rpc_role_gate::{RoleCheckFailure, VerifiedPrincipal};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PayloadValidationError {
    pub path: String,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchAuditEvent {
    pub kind: &'static str,
    pub subject: String,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchSuccessResponse {
    pub tool: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub output_limit_bytes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchStableErrorResponse {
    pub status_code: u16,
    pub error_code: &'static str,
    pub message: &'static str,
    pub details: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MrtDispatchRouteResponse {
    Success(MrtDispatchSuccessResponse),
    StableError(MrtDispatchStableErrorResponse),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MrtDispatchPipelineError {
    ConcurrencyLimitExceeded,
    TokenQuotaExceeded,
    ResolveFailure(String),
    ExecuteFailure(MrtDispatchExecutionError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ClippedOutput {
    text: String,
    truncated: bool,
}

fn clip_output(raw: &str, max_output_bytes: usize) -> ClippedOutput {
    if raw.len() <= max_output_bytes {
        return ClippedOutput { text: raw.to_owned(), truncated: false };
    }

    let mut text = raw.to_owned();
    while text.len() > max_output_bytes {
        text.pop();
    }

    ClippedOutput { text, truncated: true }
}

fn stable_error_response(
    status_code: u16,
    error: StableError,
    details: Option<String>,
) -> MrtDispatchRouteResponse {
    MrtDispatchRouteResponse::StableError(MrtDispatchStableErrorResponse {
        status_code,
        error_code: error.error_code,
        message: error.message,
        details,
    })
}

fn role_failure_reason(failure: &RoleCheckFailure) -> &'static str {
    match failure {
        RoleCheckFailure::MissingApiKey => "missing_api_key",
        RoleCheckFailure::InvalidApiKey => "invalid_api_key",
        RoleCheckFailure::ValidationUnknownMethod => "validation_unknown_method",
        RoleCheckFailure::InsufficientRole { .. } => "insufficient_role",
    }
}

fn execution_error_details(err: &MrtDispatchExecutionError) -> String {
    match err {
        MrtDispatchExecutionError::SpawnSyncError { message, stdout, stderr, .. } => {
            if !stderr.is_empty() {
                return stderr.clone();
            }
            if !stdout.is_empty() {
                return stdout.clone();
            }
            message.clone()
        }
        MrtDispatchExecutionError::NonZeroExit { message, stdout, stderr, .. } => {
            if !stderr.is_empty() {
                return stderr.clone();
            }
            if !stdout.is_empty() {
                return stdout.clone();
            }
            message.clone()
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mrt_dispatch_route<
    FValidate,
    FRequireRole,
    FWithConcurrency,
    FResolve,
    FExecute,
    FAudit,
>(
    tool_name: &str,
    body: &InvocationInputBody,
    output_limit_bytes: usize,
    validate_payload: FValidate,
    require_dispatch_role: FRequireRole,
    with_concurrency_limit: FWithConcurrency,
    resolve_invocation: FResolve,
    execute_invocation: FExecute,
    mut append_audit_event: FAudit,
) -> MrtDispatchRouteResponse
where
    FValidate: Fn(&str, &InvocationInputBody) -> Result<(), Vec<PayloadValidationError>>,
    FRequireRole: Fn(&str) -> Result<VerifiedPrincipal, RoleCheckFailure>,
    FWithConcurrency: FnOnce(
        &mut dyn FnMut() -> Result<MrtDispatchExecutionResult, MrtDispatchPipelineError>,
    ) -> Result<MrtDispatchExecutionResult, MrtDispatchPipelineError>,
    FResolve: Fn(&str, &InvocationInputBody) -> Result<MrtDispatchInvocationPlan, String>,
    FExecute: Fn(
        &MrtDispatchInvocationPlan,
    ) -> Result<MrtDispatchExecutionResult, MrtDispatchExecutionError>,
    FAudit: FnMut(MrtDispatchAuditEvent),
{
    let validation = validate_payload(tool_name, body);
    if let Err(errors) = validation {
        append_audit_event(MrtDispatchAuditEvent {
            kind: "validation_reject",
            subject: tool_name.to_owned(),
            message: Some(format!("validation_errors={}", errors.len())),
        });

        return stable_error_response(
            400,
            SCHEMA_VALIDATION_ERROR,
            Some(format!("route={};errors={}", tool_name, errors.len())),
        );
    }

    let role_check = require_dispatch_role(tool_name);
    let verified = match role_check {
        Ok(entry) => entry,
        Err(failure) => {
            append_audit_event(MrtDispatchAuditEvent {
                kind: "auth_reject",
                subject: tool_name.to_owned(),
                message: Some(role_failure_reason(&failure).to_owned()),
            });

            let envelope = role_failure_to_envelope(&failure);
            let details = match failure {
                RoleCheckFailure::InsufficientRole { role } => {
                    Some(format!("role={}", role.as_str()))
                }
                _ => None,
            };
            return stable_error_response(envelope.status_code, envelope.error, details);
        }
    };

    append_audit_event(MrtDispatchAuditEvent {
        kind: "mrt_dispatch_start",
        subject: tool_name.to_owned(),
        message: Some(format!("role={};token_id={}", verified.role.as_str(), verified.id)),
    });

    let mut execute_pipeline = || {
        let invocation = resolve_invocation(tool_name, body)
            .map_err(MrtDispatchPipelineError::ResolveFailure)?;
        execute_invocation(&invocation).map_err(MrtDispatchPipelineError::ExecuteFailure)
    };

    let exec_result = match with_concurrency_limit(&mut execute_pipeline) {
        Ok(result) => result,
        Err(MrtDispatchPipelineError::ConcurrencyLimitExceeded) => {
            append_audit_event(MrtDispatchAuditEvent {
                kind: "quota_reject",
                subject: tool_name.to_owned(),
                message: None,
            });
            return stable_error_response(429, CONCURRENCY_LIMIT_ERROR, None);
        }
        Err(MrtDispatchPipelineError::TokenQuotaExceeded) => {
            append_audit_event(MrtDispatchAuditEvent {
                kind: "token_quota_reject",
                subject: tool_name.to_owned(),
                message: None,
            });
            return stable_error_response(429, TOKEN_QUOTA_LIMIT_ERROR, None);
        }
        Err(MrtDispatchPipelineError::ResolveFailure(message)) => {
            append_audit_event(MrtDispatchAuditEvent {
                kind: "mrt_dispatch_error",
                subject: tool_name.to_owned(),
                message: Some(message.clone()),
            });
            return stable_error_response(400, MRT_EXECUTION_ERROR, Some(message));
        }
        Err(MrtDispatchPipelineError::ExecuteFailure(err)) => {
            let details = execution_error_details(&err);
            append_audit_event(MrtDispatchAuditEvent {
                kind: "mrt_dispatch_error",
                subject: tool_name.to_owned(),
                message: Some(details.clone()),
            });
            return stable_error_response(400, MRT_EXECUTION_ERROR, Some(details));
        }
    };

    let clipped_stdout = clip_output(&exec_result.stdout, output_limit_bytes);
    let clipped_stderr = clip_output(&exec_result.stderr, output_limit_bytes);

    append_audit_event(MrtDispatchAuditEvent {
        kind: "mrt_dispatch_complete",
        subject: tool_name.to_owned(),
        message: Some(format!(
            "exit_code={};stdout_truncated={};stderr_truncated={}",
            exec_result.exit_code, clipped_stdout.truncated, clipped_stderr.truncated
        )),
    });

    MrtDispatchRouteResponse::Success(MrtDispatchSuccessResponse {
        tool: tool_name.to_owned(),
        exit_code: exec_result.exit_code,
        stdout: clipped_stdout.text,
        stderr: clipped_stderr.text,
        stdout_truncated: clipped_stdout.truncated,
        stderr_truncated: clipped_stderr.truncated,
        output_limit_bytes,
    })
}
