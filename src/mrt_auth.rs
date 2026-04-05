#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Role {
    Admin,
    Operator,
    Auditor,
    Viewer,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenHasher {
    version: &'static str,
}

impl TokenHasher {
    pub fn sha256_hex_v1() -> Self {
        Self { version: "v1" }
    }

    pub fn hash(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn version(&self) -> &'static str {
        self.version
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HashedTokenRecord {
    id: String,
    version: String,
    token_hash: String,
    role: Role,
}

impl HashedTokenRecord {
    pub fn new(id: &str, token_hash: String, role: Role) -> Self {
        Self { id: id.to_string(), version: "v1".to_string(), token_hash, role }
    }

    pub fn new_with_version(id: &str, version: &str, token_hash: String, role: Role) -> Self {
        Self { id: id.to_string(), version: version.to_string(), token_hash, role }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenValidation {
    Valid { token_id: String, role: Role },
    Invalid { code: &'static str },
}

#[derive(Debug, Clone)]
pub struct HashedTokenValidator {
    hasher: TokenHasher,
    records: Vec<HashedTokenRecord>,
}

impl HashedTokenValidator {
    pub fn new(hasher: TokenHasher, records: Vec<HashedTokenRecord>) -> Self {
        Self { hasher, records }
    }

    pub fn validate(&self, token: &str) -> TokenValidation {
        if token.contains('\n') || token.contains('\r') {
            return TokenValidation::Invalid { code: "token_malformed" };
        }

        let input_hash = self.hasher.hash(token);
        for record in &self.records {
            if record.token_hash == input_hash {
                if record.version != self.hasher.version() {
                    return TokenValidation::Invalid { code: "hash_version_mismatch" };
                }

                return TokenValidation::Valid { token_id: record.id.clone(), role: record.role };
            }
        }

        TokenValidation::Invalid { code: "hash_mismatch" }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoutePolicy {
    path: String,
    roles: Vec<Role>,
}

impl RoutePolicy {
    pub fn new(path: &str, roles: Vec<Role>) -> Self {
        Self { path: path.to_string(), roles }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AuthConfig {
    strict_fail_closed: bool,
}

impl AuthConfig {
    pub fn strict_fail_closed() -> Self {
        Self { strict_fail_closed: true }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AuthInput {
    _method: String,
    path: String,
    bearer_token: Option<String>,
}

impl AuthInput {
    pub fn new(method: &str, path: &str) -> Self {
        Self { _method: method.to_string(), path: path.to_string(), bearer_token: None }
    }

    pub fn with_bearer_token(mut self, token: &str) -> Self {
        self.bearer_token = Some(token.to_string());
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AuthDecision {
    Allow { role: Role, token_id: String },
    Deny { code: &'static str },
}

#[derive(Debug, Clone)]
pub struct DynamicAuthMiddleware {
    config: AuthConfig,
    validator: Option<HashedTokenValidator>,
    route_policies: HashMap<String, Vec<Role>>,
    revoked_ids: HashSet<String>,
}

impl DynamicAuthMiddleware {
    pub fn new(config: AuthConfig) -> Self {
        Self {
            config,
            validator: None,
            route_policies: HashMap::new(),
            revoked_ids: HashSet::new(),
        }
    }

    pub fn with_validator(mut self, validator: HashedTokenValidator) -> Self {
        self.validator = Some(validator);
        self
    }

    pub fn with_route_policy(mut self, policy: RoutePolicy) -> Self {
        self.route_policies.insert(policy.path, policy.roles);
        self
    }

    pub fn set_route_policy(&mut self, policy: RoutePolicy) {
        self.route_policies.insert(policy.path, policy.roles);
    }

    pub fn revoke_token_by_id(&mut self, token_id: &str) {
        self.revoked_ids.insert(token_id.to_string());
    }

    pub fn authorize(&self, input: &AuthInput) -> AuthDecision {
        let Some(token) = input.bearer_token.as_deref() else {
            return AuthDecision::Deny { code: "missing_token" };
        };

        let Some(validator) = &self.validator else {
            return AuthDecision::Deny { code: "validator_not_configured" };
        };

        let validation = validator.validate(token);
        let (token_id, role) = match validation {
            TokenValidation::Valid { token_id, role } => (token_id, role),
            TokenValidation::Invalid { code } => {
                return AuthDecision::Deny { code };
            }
        };

        if self.revoked_ids.contains(&token_id) {
            return AuthDecision::Deny { code: "revoked_token" };
        }

        let Some(allowed_roles) = self.route_policies.get(&input.path) else {
            if self.config.strict_fail_closed {
                return AuthDecision::Deny { code: "route_not_registered" };
            }
            return AuthDecision::Allow { role, token_id };
        };

        if allowed_roles.is_empty() {
            return AuthDecision::Deny { code: "role_policy_empty" };
        }

        if !allowed_roles.contains(&role) {
            return AuthDecision::Deny { code: "role_forbidden" };
        }

        AuthDecision::Allow { role, token_id }
    }
}
