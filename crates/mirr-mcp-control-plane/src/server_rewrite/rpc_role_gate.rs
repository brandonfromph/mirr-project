#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use crate::policy::Role;
use crate::tooling::MrtDispatchTool;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPrincipal {
    pub id: String,
    pub role: Role,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoleCheckFailure {
    MissingApiKey,
    InvalidApiKey,
    InsufficientRole { role: Role },
    ValidationUnknownMethod,
}

pub type RoleTokenMap = BTreeMap<String, VerifiedPrincipal>;

pub fn require_role(
    api_key: Option<&str>,
    allowed_roles: &[Role],
    token_map: &RoleTokenMap,
) -> Result<VerifiedPrincipal, RoleCheckFailure> {
    let Some(token) = api_key else {
        return Err(RoleCheckFailure::MissingApiKey);
    };

    if token.is_empty() {
        return Err(RoleCheckFailure::MissingApiKey);
    }

    let Some(entry) = token_map.get(token) else {
        return Err(RoleCheckFailure::InvalidApiKey);
    };

    if !allowed_roles.contains(&entry.role) {
        return Err(RoleCheckFailure::InsufficientRole { role: entry.role });
    }

    Ok(entry.clone())
}

pub fn require_mrt_dispatch_role(
    api_key: Option<&str>,
    tool_name: &str,
    token_map: &RoleTokenMap,
) -> Result<VerifiedPrincipal, RoleCheckFailure> {
    let Ok(tool) = tool_name.parse::<MrtDispatchTool>() else {
        return Err(RoleCheckFailure::ValidationUnknownMethod);
    };

    require_role(api_key, tool.role_allowlist(), token_map)
}
