#![forbid(unsafe_code)]

use mirrc::mrt_auth::{
    AuthConfig, AuthDecision, AuthInput, DynamicAuthMiddleware, HashedTokenRecord,
    HashedTokenValidator, Role, RoutePolicy, TokenHasher, TokenValidation,
};
use mirrc::mrt_schema::{
    BoundedFieldSpec, BoundedJsonInput, PreDispatchDecision, PreDispatchInput, SchemaContract,
    SchemaPipeline, SchemaPolicy, SchemaStage,
};

fn base_validator() -> HashedTokenValidator {
    let hasher = TokenHasher::sha256_hex_v1();
    let records = vec![
        HashedTokenRecord::new("admin-token", hasher.hash("admin-secret"), Role::Admin),
        HashedTokenRecord::new("operator-token", hasher.hash("operator-secret"), Role::Operator),
        HashedTokenRecord::new("auditor-token", hasher.hash("auditor-secret"), Role::Auditor),
        HashedTokenRecord::new("viewer-token", hasher.hash("viewer-secret"), Role::Viewer),
    ];
    HashedTokenValidator::new(hasher, records)
}

fn base_middleware() -> DynamicAuthMiddleware {
    DynamicAuthMiddleware::new(AuthConfig::strict_fail_closed())
        .with_validator(base_validator())
        .with_route_policy(RoutePolicy::new("/mrt_wave_apply", vec![Role::Admin]))
        .with_route_policy(RoutePolicy::new("/mrt_operator_tool", vec![Role::Operator]))
}

fn base_pipeline() -> SchemaPipeline {
    let mut pipeline = SchemaPipeline::new(SchemaPolicy::strict_fail_closed())
        .with_stage(SchemaStage::parse_json())
        .with_stage(SchemaStage::enforce_field_bounds())
        .with_stage(SchemaStage::enforce_types());

    let contract = SchemaContract::new("/mrt_wave_apply")
        .with_required_field(BoundedFieldSpec::string("api_key", 64))
        .with_required_field(BoundedFieldSpec::string("path", 256))
        .with_optional_field(BoundedFieldSpec::u64("timeout_ms", 1, 60_000));

    pipeline.register_contract(contract);
    pipeline
}

#[test]
fn dynamic_auth_allows_admin_token_for_admin_route() {
    let middleware = base_middleware();
    let request = AuthInput::new("POST", "/mrt_wave_apply").with_bearer_token("admin-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Allow { role: Role::Admin, .. }));
}

#[test]
fn dynamic_auth_denies_missing_token_fail_closed() {
    let middleware = base_middleware();
    let request = AuthInput::new("POST", "/mrt_wave_apply");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Deny { code: "missing_token" }));
}

#[test]
fn dynamic_auth_denies_unregistered_route_in_strict_mode() {
    let middleware = base_middleware();
    let request = AuthInput::new("POST", "/mrt_unknown_route").with_bearer_token("admin-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Deny { code: "route_not_registered" }));
}

#[test]
fn dynamic_auth_applies_runtime_route_policy_updates() {
    let mut middleware = base_middleware();
    let request = AuthInput::new("POST", "/mrt_wave_apply").with_bearer_token("operator-secret");
    let first = middleware.authorize(&request);

    assert!(matches!(first, AuthDecision::Deny { code: "role_forbidden" }));

    middleware.set_route_policy(RoutePolicy::new("/mrt_wave_apply", vec![Role::Operator]));
    let second = middleware.authorize(&request);

    assert!(matches!(second, AuthDecision::Allow { role: Role::Operator, .. }));
}

#[test]
fn dynamic_auth_supports_multi_role_route_policies() {
    let mut middleware = base_middleware();
    middleware
        .set_route_policy(RoutePolicy::new("/mrt_wave_apply", vec![Role::Admin, Role::Operator]));
    let request = AuthInput::new("POST", "/mrt_wave_apply").with_bearer_token("operator-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Allow { role: Role::Operator, .. }));
}

#[test]
fn dynamic_auth_rejects_revoked_tokens() {
    let mut middleware = base_middleware();
    middleware.revoke_token_by_id("operator-token");
    let request = AuthInput::new("POST", "/mrt_operator_tool").with_bearer_token("operator-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Deny { code: "revoked_token" }));
}

#[test]
fn hashed_validation_accepts_known_sha256_token() {
    let validator = base_validator();
    let outcome = validator.validate("admin-secret");

    assert!(matches!(
        outcome,
        TokenValidation::Valid {
            token_id,
            role: Role::Admin
        } if token_id == "admin-token"
    ));
}

#[test]
fn hashed_validation_rejects_plaintext_mismatch() {
    let validator = base_validator();
    let outcome = validator.validate("wrong-secret");

    assert!(matches!(outcome, TokenValidation::Invalid { code: "hash_mismatch" }));
}

#[test]
fn hashed_validation_rejects_malformed_token_input() {
    let validator = base_validator();
    let outcome = validator.validate("bad\nsecret");

    assert!(matches!(outcome, TokenValidation::Invalid { code: "token_malformed" }));
}

#[test]
fn hashed_validation_rejects_hash_version_mismatch() {
    let hasher = TokenHasher::sha256_hex_v1();
    let record = HashedTokenRecord::new_with_version(
        "legacy-token",
        "v2",
        hasher.hash("legacy-secret"),
        Role::Auditor,
    );
    let validator = HashedTokenValidator::new(hasher, vec![record]);
    let outcome = validator.validate("legacy-secret");

    assert!(matches!(outcome, TokenValidation::Invalid { code: "hash_version_mismatch" }));
}

#[test]
fn role_gate_allows_exact_required_role() {
    let middleware = base_middleware();
    let request = AuthInput::new("POST", "/mrt_operator_tool").with_bearer_token("operator-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Allow { role: Role::Operator, .. }));
}

#[test]
fn role_gate_denies_lower_privilege_role() {
    let middleware = base_middleware();
    let request = AuthInput::new("POST", "/mrt_wave_apply").with_bearer_token("viewer-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Deny { code: "role_forbidden" }));
}

#[test]
fn role_gate_allows_any_role_in_allowlist() {
    let mut middleware = base_middleware();
    middleware.set_route_policy(RoutePolicy::new(
        "/mrt_read_audit_log",
        vec![Role::Admin, Role::Auditor],
    ));
    let request = AuthInput::new("POST", "/mrt_read_audit_log").with_bearer_token("auditor-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Allow { role: Role::Auditor, .. }));
}

#[test]
fn role_gate_rejects_empty_role_policy_fail_closed() {
    let mut middleware = base_middleware();
    middleware.set_route_policy(RoutePolicy::new("/mrt_no_roles", vec![]));
    let request = AuthInput::new("POST", "/mrt_no_roles").with_bearer_token("admin-secret");
    let decision = middleware.authorize(&request);

    assert!(matches!(decision, AuthDecision::Deny { code: "role_policy_empty" }));
}

#[test]
fn schema_pipeline_rejects_unregistered_route_pre_dispatch() {
    let pipeline = base_pipeline();
    let payload =
        BoundedJsonInput::new().with_string("api_key", "k1").with_string("path", "/tmp/a");
    let input = PreDispatchInput::new("/mrt_unknown_route", payload);
    let decision = pipeline.pre_dispatch(&input);

    assert!(matches!(decision, PreDispatchDecision::Reject { code: "route_schema_not_found", .. }));
}

#[test]
fn schema_pipeline_rejects_missing_required_field_pre_dispatch() {
    let pipeline = base_pipeline();
    let payload = BoundedJsonInput::new().with_string("path", "/tmp/a");
    let input = PreDispatchInput::new("/mrt_wave_apply", payload);
    let decision = pipeline.pre_dispatch(&input);

    assert!(matches!(
        decision,
        PreDispatchDecision::Reject {
            code: "missing_required_field",
            field: Some(field),
            ..
        } if field == "api_key"
    ));
}

#[test]
fn schema_pipeline_rejects_field_type_mismatch_pre_dispatch() {
    let pipeline = base_pipeline();
    let payload = BoundedJsonInput::new()
        .with_string("api_key", "k1")
        .with_string("path", "/tmp/a")
        .with_string("timeout_ms", "fast");
    let input = PreDispatchInput::new("/mrt_wave_apply", payload);
    let decision = pipeline.pre_dispatch(&input);

    assert!(matches!(
        decision,
        PreDispatchDecision::Reject {
            code: "field_type_mismatch",
            field: Some(field),
            ..
        } if field == "timeout_ms"
    ));
}

#[test]
fn schema_pipeline_rejects_string_field_over_max_length() {
    let pipeline = base_pipeline();
    let long_path = "x".repeat(300);
    let payload =
        BoundedJsonInput::new().with_string("api_key", "k1").with_string("path", &long_path);
    let input = PreDispatchInput::new("/mrt_wave_apply", payload);
    let decision = pipeline.pre_dispatch(&input);

    assert!(matches!(
        decision,
        PreDispatchDecision::Reject {
            code: "field_too_long",
            field: Some(field),
            ..
        } if field == "path"
    ));
}

#[test]
fn schema_pipeline_rejects_payload_exceeding_max_field_count() {
    let mut pipeline = SchemaPipeline::new(SchemaPolicy::strict_fail_closed().with_max_fields(3))
        .with_stage(SchemaStage::parse_json())
        .with_stage(SchemaStage::enforce_field_bounds())
        .with_stage(SchemaStage::enforce_types());

    let contract = SchemaContract::new("/mrt_wave_apply")
        .with_required_field(BoundedFieldSpec::string("api_key", 64))
        .with_required_field(BoundedFieldSpec::string("path", 256))
        .with_optional_field(BoundedFieldSpec::u64("timeout_ms", 1, 60_000))
        .with_optional_field(BoundedFieldSpec::string("correlation_id", 32));
    pipeline.register_contract(contract);

    let payload = BoundedJsonInput::new()
        .with_string("api_key", "k1")
        .with_string("path", "/tmp/a")
        .with_u64("timeout_ms", 1000)
        .with_string("correlation_id", "c-123");
    let input = PreDispatchInput::new("/mrt_wave_apply", payload);
    let decision = pipeline.pre_dispatch(&input);

    assert!(matches!(decision, PreDispatchDecision::Reject { code: "too_many_fields", .. }));
}

#[test]
fn schema_pipeline_fail_closed_on_invalid_json_fragment() {
    let pipeline = base_pipeline();
    let payload = BoundedJsonInput::from_raw_fragment("{\"api_key\":\"k1\",\"path\":\"/tmp/a\"");
    let input = PreDispatchInput::new("/mrt_wave_apply", payload);
    let decision = pipeline.pre_dispatch(&input);

    assert!(matches!(decision, PreDispatchDecision::Reject { code: "invalid_json_fragment", .. }));
}
