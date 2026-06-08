#![forbid(unsafe_code)]

use std::time::Duration;

use mirrc::mirr_daemon_security::audit::{AuditEvent, AuditEventKind};
use mirrc::mirr_daemon_security::auth::{AuthContext, Role, RuntimeAuthorizer};
use mirrc::mirr_daemon_security::policy::{OutputLimitPolicy, PayloadLimitPolicy, RoutePolicy};
use mirrc::mirr_daemon_security::rate_limit::{TokenBucketConfig, TokenBucketLimiter};

#[test]
fn token_bucket_starts_full_at_capacity() {
    let config = TokenBucketConfig::new(10, 2, Duration::from_secs(1));
    let limiter = TokenBucketLimiter::new(config);

    assert_eq!(limiter.available_tokens(), 10);
}

#[test]
fn token_bucket_allows_consumption_within_available_tokens() {
    let config = TokenBucketConfig::new(8, 2, Duration::from_secs(1));
    let mut limiter = TokenBucketLimiter::new(config);

    assert!(limiter.try_consume(3));
    assert_eq!(limiter.available_tokens(), 5);
}

#[test]
fn token_bucket_rejects_consumption_when_tokens_insufficient() {
    let config = TokenBucketConfig::new(5, 1, Duration::from_secs(1));
    let mut limiter = TokenBucketLimiter::new(config);

    assert!(limiter.try_consume(5));
    assert!(!limiter.try_consume(1));
    assert_eq!(limiter.available_tokens(), 0);
}

#[test]
fn token_bucket_refills_over_time_but_never_exceeds_capacity() {
    let config = TokenBucketConfig::new(10, 3, Duration::from_secs(1));
    let mut limiter = TokenBucketLimiter::new(config);

    assert!(limiter.try_consume(9));
    limiter.advance_time(Duration::from_secs(5));
    assert_eq!(limiter.available_tokens(), 10);
}

#[test]
fn strict_unknown_method_policy_rejects_unmapped_method() {
    let policy = RoutePolicy::strict_fail_closed("wave6-runtime");
    let decision = policy.decide_method("daemon.unknown");

    assert!(decision.is_denied());
    assert_eq!(decision.reason_code(), "UNKNOWN_METHOD");
    assert_eq!(decision.http_status(), 404);
}

#[test]
fn unknown_method_handling_is_fail_closed_by_default() {
    let policy = RoutePolicy::new("wave6-runtime");
    let decision = policy.decide_method("daemon.not_registered");

    assert!(decision.is_denied());
    assert_eq!(decision.reason_code(), "UNKNOWN_METHOD");
}

#[test]
fn strict_unknown_method_policy_allows_registered_method() {
    let policy = RoutePolicy::strict_fail_closed("wave6-runtime").allow_method("daemon.healthz");
    let decision = policy.decide_method("daemon.healthz");

    assert!(decision.is_allowed());
}

#[test]
fn strict_unknown_method_policy_never_uses_compat_fallback() {
    let policy = RoutePolicy::strict_fail_closed("wave6-runtime");
    let decision = policy.decide_method("daemon.legacy_ping");

    assert!(decision.is_denied());
    assert!(!decision.compatibility_fallback_used());
}

#[test]
fn runtime_boundary_denies_requests_without_auth_context() {
    let policy = RoutePolicy::strict_fail_closed("wave6-runtime").allow_method("daemon.rotate_key");
    let authorizer = RuntimeAuthorizer::new(policy);
    let auth = AuthContext::missing();
    let decision = authorizer.authorize(&auth, "daemon.rotate_key", Role::Admin);

    assert!(decision.is_denied());
    assert_eq!(decision.reason_code(), "AUTH_REQUIRED");
}

#[test]
fn runtime_boundary_denies_invalid_token_before_role_eval() {
    let policy = RoutePolicy::strict_fail_closed("wave6-runtime").allow_method("daemon.rotate_key");
    let mut authorizer = RuntimeAuthorizer::new(policy);
    authorizer.register_principal_token("token-admin", "alice", Role::Admin);

    let auth = AuthContext::bearer("token-invalid");
    let decision = authorizer.authorize(&auth, "daemon.rotate_key", Role::Admin);

    assert!(decision.is_denied());
    assert_eq!(decision.reason_code(), "AUTH_INVALID_TOKEN");
}

#[test]
fn runtime_boundary_denies_when_role_is_insufficient() {
    let policy = RoutePolicy::strict_fail_closed("wave6-runtime").allow_method("daemon.rotate_key");
    let mut authorizer = RuntimeAuthorizer::new(policy);
    authorizer.register_principal_token("token-operator", "ops-user", Role::Operator);

    let auth = AuthContext::bearer("token-operator");
    let decision = authorizer.authorize(&auth, "daemon.rotate_key", Role::Admin);

    assert!(decision.is_denied());
    assert_eq!(decision.reason_code(), "ROLE_FORBIDDEN");
}

#[test]
fn runtime_boundary_allows_when_token_and_role_match_requirement() {
    let policy = RoutePolicy::strict_fail_closed("wave6-runtime").allow_method("daemon.rotate_key");
    let mut authorizer = RuntimeAuthorizer::new(policy);
    authorizer.register_principal_token("token-admin", "alice", Role::Admin);

    let auth = AuthContext::bearer("token-admin");
    let decision = authorizer.authorize(&auth, "daemon.rotate_key", Role::Admin);

    assert!(decision.is_allowed());
}

#[test]
fn payload_policy_rejects_request_bytes_over_limit() {
    let policy = PayloadLimitPolicy::new(1_048_576);
    let violation = policy
        .validate_request_bytes(1_048_577)
        .expect_err("request payload above limit must be rejected");

    assert_eq!(violation.code(), "PAYLOAD_TOO_LARGE");
    assert_eq!(violation.limit_bytes(), 1_048_576);
}

#[test]
fn payload_policy_accepts_request_bytes_at_limit() {
    let policy = PayloadLimitPolicy::new(1_048_576);

    assert!(policy.validate_request_bytes(1_048_576).is_ok());
}

#[test]
fn output_policy_rejects_response_bytes_over_limit() {
    let policy = OutputLimitPolicy::new(65_536);
    let violation = policy
        .validate_output_bytes(65_537)
        .expect_err("response output above limit must be rejected");

    assert_eq!(violation.code(), "OUTPUT_TOO_LARGE");
    assert_eq!(violation.limit_bytes(), 65_536);
}

#[test]
fn output_policy_accepts_response_bytes_at_limit() {
    let policy = OutputLimitPolicy::new(65_536);

    assert!(policy.validate_output_bytes(65_536).is_ok());
}

#[test]
fn route_policy_denies_unmapped_methods_by_default() {
    let policy = RoutePolicy::deny_by_default("wave6-runtime");
    let decision = policy.decide_method("daemon.unmapped");

    assert!(decision.is_denied());
    assert_eq!(decision.reason_code(), "ROUTE_DENY_BY_DEFAULT");
}

#[test]
fn route_policy_allows_explicitly_mapped_methods_only() {
    let policy = RoutePolicy::deny_by_default("wave6-runtime").allow_method("daemon.healthz");

    assert!(policy.decide_method("daemon.healthz").is_allowed());
    assert!(policy.decide_method("daemon.unmapped").is_denied());
}

#[test]
fn deny_decisions_emit_audit_events_with_actor_and_policy() {
    let policy = RoutePolicy::deny_by_default("wave6-runtime");
    let decision = policy.decide_method("daemon.unmapped");
    let event = AuditEvent::from_route_decision("req-001", "svc-ci", &decision);

    assert_eq!(event.kind(), AuditEventKind::AccessDenied);
    assert_eq!(event.actor_id(), "svc-ci");
    assert_eq!(event.policy_id(), "wave6-runtime");
    assert_eq!(event.method(), "daemon.unmapped");
}

#[test]
fn allow_decisions_emit_audit_events_with_route_and_outcome() {
    let policy = RoutePolicy::deny_by_default("wave6-runtime").allow_method("daemon.healthz");
    let decision = policy.decide_method("daemon.healthz");
    let event = AuditEvent::from_route_decision("req-002", "svc-ci", &decision);

    assert_eq!(event.kind(), AuditEventKind::AccessAllowed);
    assert_eq!(event.method(), "daemon.healthz");
    assert_eq!(event.outcome(), "allow");
}
