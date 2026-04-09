use std::collections::BTreeMap;

use mirror::policy::Role;
use mirror::server_rewrite::rpc_role_failure_envelope::role_failure_to_envelope;
use mirror::server_rewrite::rpc_role_gate::{
    require_mrt_dispatch_role, require_role, RoleCheckFailure, RoleTokenMap, VerifiedPrincipal,
};

fn token_map_fixture() -> RoleTokenMap {
    let mut map: BTreeMap<String, VerifiedPrincipal> = BTreeMap::new();
    map.insert(
        "builder-token".to_owned(),
        VerifiedPrincipal { id: "builder".to_owned(), role: Role::Builder },
    );
    map.insert(
        "admin-token".to_owned(),
        VerifiedPrincipal { id: "admin".to_owned(), role: Role::Admin },
    );
    map
}

#[test]
fn require_role_matches_ts_failure_modes() {
    let token_map = token_map_fixture();

    assert_eq!(
        require_role(None, &[Role::Admin], &token_map),
        Err(RoleCheckFailure::MissingApiKey)
    );

    assert_eq!(
        require_role(Some(""), &[Role::Admin], &token_map),
        Err(RoleCheckFailure::MissingApiKey)
    );

    assert_eq!(
        require_role(Some("unknown"), &[Role::Admin], &token_map),
        Err(RoleCheckFailure::InvalidApiKey)
    );

    assert_eq!(
        require_role(Some("builder-token"), &[Role::Admin], &token_map),
        Err(RoleCheckFailure::InsufficientRole { role: Role::Builder })
    );

    let verified = require_role(Some("admin-token"), &[Role::Admin], &token_map)
        .expect("admin role should pass role gate");
    assert_eq!(verified.id, "admin");
    assert_eq!(verified.role, Role::Admin);
}

#[test]
fn require_mrt_dispatch_role_matches_ts_unknown_method_behavior() {
    let token_map = token_map_fixture();

    assert_eq!(
        require_mrt_dispatch_role(Some("admin-token"), "mrt_unknown", &token_map),
        Err(RoleCheckFailure::ValidationUnknownMethod)
    );

    let verified = require_mrt_dispatch_role(Some("admin-token"), "mrt_wave_apply", &token_map)
        .expect("admin should pass for mrt_wave_apply");
    assert_eq!(verified.role, Role::Admin);
}

#[test]
fn role_failure_envelope_mapping_matches_ts_send_role_failure_contract() {
    let missing = role_failure_to_envelope(&RoleCheckFailure::MissingApiKey);
    assert_eq!(missing.status_code, 401);
    assert_eq!(missing.error.error_code, "auth_missing_api_key");

    let invalid = role_failure_to_envelope(&RoleCheckFailure::InvalidApiKey);
    assert_eq!(invalid.status_code, 403);
    assert_eq!(invalid.error.error_code, "auth_invalid_api_key");

    let unknown = role_failure_to_envelope(&RoleCheckFailure::ValidationUnknownMethod);
    assert_eq!(unknown.status_code, 400);
    assert_eq!(unknown.error.error_code, "validation_unknown_method");

    let insufficient =
        role_failure_to_envelope(&RoleCheckFailure::InsufficientRole { role: Role::Builder });
    assert_eq!(insufficient.status_code, 403);
    assert_eq!(insufficient.error.error_code, "auth_insufficient_role");
}
