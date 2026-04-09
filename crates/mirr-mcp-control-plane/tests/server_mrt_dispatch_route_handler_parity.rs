use std::cell::Cell;

use mirror::policy::Role;
use mirror::server_rewrite::mrt_dispatch_invocation_executor::MrtDispatchExecutionError;
use mirror::server_rewrite::mrt_dispatch_invocation_executor::MrtDispatchExecutionResult;
use mirror::server_rewrite::mrt_dispatch_invocation_input::InvocationInputBody;
use mirror::server_rewrite::mrt_dispatch_invocation_plan::MrtDispatchInvocationPlan;
use mirror::server_rewrite::mrt_dispatch_route_handler::{
    handle_mrt_dispatch_route, MrtDispatchPipelineError, MrtDispatchRouteResponse,
    PayloadValidationError,
};
use mirror::server_rewrite::rpc_role_gate::{RoleCheckFailure, VerifiedPrincipal};

#[test]
fn route_handler_success_path_matches_dispatch_contract_shape() {
    let body = InvocationInputBody::default();
    let mut events = Vec::new();

    let response = handle_mrt_dispatch_route(
        "mrt_audit",
        &body,
        6,
        |_tool, _body| Ok(()),
        |_tool| Ok(VerifiedPrincipal { id: "builder-1".to_owned(), role: Role::Builder }),
        |op| op(),
        |_tool, _body| {
            Ok(MrtDispatchInvocationPlan::new(vec![
                "run".to_owned(),
                "--bin".to_owned(),
                "mirr-audit".to_owned(),
            ]))
        },
        |_plan| {
            Ok(MrtDispatchExecutionResult {
                stdout: "abcdefghi".to_owned(),
                stderr: "warn".to_owned(),
                exit_code: 0,
            })
        },
        |event| events.push(event),
    );

    match response {
        MrtDispatchRouteResponse::Success(ok) => {
            assert_eq!(ok.tool, "mrt_audit");
            assert_eq!(ok.exit_code, 0);
            assert_eq!(ok.stdout, "abcdef");
            assert_eq!(ok.stderr, "warn");
            assert!(ok.stdout_truncated);
            assert!(!ok.stderr_truncated);
            assert_eq!(ok.output_limit_bytes, 6);
        }
        other => panic!("expected success response, got: {:?}", other),
    }

    let kinds = events.iter().map(|event| event.kind).collect::<Vec<&str>>();
    assert_eq!(kinds, vec!["mrt_dispatch_start", "mrt_dispatch_complete"]);
}

#[test]
fn route_handler_validation_reject_is_fail_closed_schema_error() {
    let body = InvocationInputBody::default();
    let role_called = Cell::new(false);
    let mut events = Vec::new();

    let response = handle_mrt_dispatch_route(
        "mrt_compile",
        &body,
        16,
        |_tool, _body| {
            Err(vec![PayloadValidationError {
                path: "/source_file".to_owned(),
                message: "is required".to_owned(),
            }])
        },
        |_tool| {
            role_called.set(true);
            Ok(VerifiedPrincipal { id: "ignored".to_owned(), role: Role::Admin })
        },
        |_op| panic!("limiter must not execute after schema rejection"),
        |_tool, _body| panic!("resolver must not execute after schema rejection"),
        |_plan| panic!("executor must not execute after schema rejection"),
        |event| events.push(event),
    );

    assert!(!role_called.get());
    match response {
        MrtDispatchRouteResponse::StableError(err) => {
            assert_eq!(err.status_code, 400);
            assert_eq!(err.error_code, "validation_schema");
            assert_eq!(err.message, "Request body failed schema validation.");
            assert!(err.details.is_some());
        }
        other => panic!("expected stable error response, got: {:?}", other),
    }

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].kind, "validation_reject");
}

#[test]
fn route_handler_role_failure_uses_existing_role_envelope_mapping() {
    let body = InvocationInputBody::default();
    let mut events = Vec::new();

    let response = handle_mrt_dispatch_route(
        "mrt_wave_apply",
        &body,
        16,
        |_tool, _body| Ok(()),
        |_tool| Err(RoleCheckFailure::InsufficientRole { role: Role::Builder }),
        |_op| panic!("limiter must not execute after role failure"),
        |_tool, _body| panic!("resolver must not execute after role failure"),
        |_plan| panic!("executor must not execute after role failure"),
        |event| events.push(event),
    );

    match response {
        MrtDispatchRouteResponse::StableError(err) => {
            assert_eq!(err.status_code, 403);
            assert_eq!(err.error_code, "auth_insufficient_role");
            assert_eq!(err.message, "API key role is not allowed.");
            assert_eq!(err.details, Some("role=builder".to_owned()));
        }
        other => panic!("expected stable error response, got: {:?}", other),
    }

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].kind, "auth_reject");
}

#[test]
fn route_handler_maps_concurrency_and_token_quota_errors() {
    let body = InvocationInputBody::default();

    let mut concurrency_events = Vec::new();
    let concurrency_response = handle_mrt_dispatch_route(
        "mrt_audit",
        &body,
        64,
        |_tool, _body| Ok(()),
        |_tool| Ok(VerifiedPrincipal { id: "builder-2".to_owned(), role: Role::Builder }),
        |_op| Err(MrtDispatchPipelineError::ConcurrencyLimitExceeded),
        |_tool, _body| panic!("resolver must not execute when limiter rejects"),
        |_plan| panic!("executor must not execute when limiter rejects"),
        |event| concurrency_events.push(event),
    );

    match concurrency_response {
        MrtDispatchRouteResponse::StableError(err) => {
            assert_eq!(err.status_code, 429);
            assert_eq!(err.error_code, "limit_concurrency_exceeded");
        }
        other => panic!("expected stable error response, got: {:?}", other),
    }
    assert_eq!(
        concurrency_events.iter().map(|event| event.kind).collect::<Vec<&str>>(),
        vec!["mrt_dispatch_start", "quota_reject"]
    );

    let mut quota_events = Vec::new();
    let quota_response = handle_mrt_dispatch_route(
        "mrt_audit",
        &body,
        64,
        |_tool, _body| Ok(()),
        |_tool| Ok(VerifiedPrincipal { id: "builder-3".to_owned(), role: Role::Builder }),
        |_op| Err(MrtDispatchPipelineError::TokenQuotaExceeded),
        |_tool, _body| panic!("resolver must not execute when token quota rejects"),
        |_plan| panic!("executor must not execute when token quota rejects"),
        |event| quota_events.push(event),
    );

    match quota_response {
        MrtDispatchRouteResponse::StableError(err) => {
            assert_eq!(err.status_code, 429);
            assert_eq!(err.error_code, "limit_token_quota_exceeded");
        }
        other => panic!("expected stable error response, got: {:?}", other),
    }
    assert_eq!(
        quota_events.iter().map(|event| event.kind).collect::<Vec<&str>>(),
        vec!["mrt_dispatch_start", "token_quota_reject"]
    );
}

#[test]
fn route_handler_maps_execution_failure_to_stable_error_with_details() {
    let body = InvocationInputBody::default();
    let mut events = Vec::new();

    let response = handle_mrt_dispatch_route(
        "mrt_general_ci",
        &body,
        64,
        |_tool, _body| Ok(()),
        |_tool| Ok(VerifiedPrincipal { id: "builder-4".to_owned(), role: Role::Builder }),
        |op| op(),
        |_tool, _body| Ok(MrtDispatchInvocationPlan::new(vec!["run".to_owned()])),
        |_plan| {
            Err(MrtDispatchExecutionError::NonZeroExit {
                message: "mrt_exec_failed_exit_7".to_owned(),
                stdout: "fallback stdout".to_owned(),
                stderr: "stderr details".to_owned(),
                exit_code: 7,
            })
        },
        |event| events.push(event),
    );

    match response {
        MrtDispatchRouteResponse::StableError(err) => {
            assert_eq!(err.status_code, 400);
            assert_eq!(err.error_code, "validation_mrt_exec_failed");
            assert_eq!(err.message, "MRT execution failed.");
            assert_eq!(err.details, Some("stderr details".to_owned()));
        }
        other => panic!("expected stable error response, got: {:?}", other),
    }

    assert_eq!(
        events.iter().map(|event| event.kind).collect::<Vec<&str>>(),
        vec!["mrt_dispatch_start", "mrt_dispatch_error"]
    );
}
