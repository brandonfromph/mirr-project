#![forbid(unsafe_code)]

pub mod rate_limit {
    use std::time::Duration;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct TokenBucketConfig {
        capacity: u64,
        refill_tokens: u64,
        refill_period: Duration,
    }

    impl TokenBucketConfig {
        #[must_use]
        pub fn new(capacity: u64, refill_tokens: u64, refill_period: Duration) -> Self {
            Self { capacity, refill_tokens, refill_period }
        }
    }

    #[derive(Clone, Debug)]
    pub struct TokenBucketLimiter {
        config: TokenBucketConfig,
        tokens: u64,
        carry_nanos: u128,
    }

    impl TokenBucketLimiter {
        #[must_use]
        pub fn new(config: TokenBucketConfig) -> Self {
            Self { tokens: config.capacity, config, carry_nanos: 0 }
        }

        #[must_use]
        pub fn available_tokens(&self) -> u64 {
            self.tokens
        }

        pub fn try_consume(&mut self, requested_tokens: u64) -> bool {
            if requested_tokens > self.tokens {
                return false;
            }
            self.tokens -= requested_tokens;
            true
        }

        pub fn advance_time(&mut self, elapsed: Duration) {
            if self.tokens >= self.config.capacity {
                return;
            }
            if self.config.refill_tokens == 0 || self.config.refill_period.is_zero() {
                return;
            }

            self.carry_nanos = self.carry_nanos.saturating_add(elapsed.as_nanos());
            let refill_period_nanos = self.config.refill_period.as_nanos();
            if refill_period_nanos == 0 {
                return;
            }

            let periods_elapsed = self.carry_nanos / refill_period_nanos;
            if periods_elapsed == 0 {
                return;
            }
            self.carry_nanos %= refill_period_nanos;

            let space_remaining = self.config.capacity.saturating_sub(self.tokens);
            if space_remaining == 0 {
                return;
            }

            let refill_amount = periods_elapsed
                .saturating_mul(u128::from(self.config.refill_tokens))
                .min(u128::from(space_remaining));
            self.tokens = self.tokens.saturating_add(refill_amount as u64);
        }
    }
}

pub mod policy {
    use std::collections::BTreeSet;

    const REASON_UNKNOWN_METHOD: &str = "UNKNOWN_METHOD";
    const REASON_ROUTE_DENY_BY_DEFAULT: &str = "ROUTE_DENY_BY_DEFAULT";
    const REASON_ALLOW: &str = "ALLOW";

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RouteDecision {
        policy_id: String,
        method: String,
        allowed: bool,
        reason_code: &'static str,
        http_status: u16,
        compatibility_fallback_used: bool,
    }

    impl RouteDecision {
        #[must_use]
        pub fn is_allowed(&self) -> bool {
            self.allowed
        }

        #[must_use]
        pub fn is_denied(&self) -> bool {
            !self.allowed
        }

        #[must_use]
        pub fn reason_code(&self) -> &str {
            self.reason_code
        }

        #[must_use]
        pub fn http_status(&self) -> u16 {
            self.http_status
        }

        #[must_use]
        pub fn compatibility_fallback_used(&self) -> bool {
            self.compatibility_fallback_used
        }

        #[must_use]
        pub fn policy_id(&self) -> &str {
            &self.policy_id
        }

        #[must_use]
        pub fn method(&self) -> &str {
            &self.method
        }

        #[must_use]
        pub fn outcome(&self) -> &str {
            if self.allowed {
                "allow"
            } else {
                "deny"
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum UnknownRouteBehavior {
        UnknownMethod,
        DenyByDefault,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RoutePolicy {
        policy_id: String,
        unknown_behavior: UnknownRouteBehavior,
        unknown_http_status: u16,
        allowed_methods: BTreeSet<String>,
    }

    impl RoutePolicy {
        #[must_use]
        pub fn new(policy_id: &str) -> Self {
            Self {
                policy_id: policy_id.to_owned(),
                unknown_behavior: UnknownRouteBehavior::UnknownMethod,
                unknown_http_status: 403,
                allowed_methods: BTreeSet::new(),
            }
        }

        #[must_use]
        pub fn strict_fail_closed(policy_id: &str) -> Self {
            Self {
                policy_id: policy_id.to_owned(),
                unknown_behavior: UnknownRouteBehavior::UnknownMethod,
                unknown_http_status: 404,
                allowed_methods: BTreeSet::new(),
            }
        }

        #[must_use]
        pub fn deny_by_default(policy_id: &str) -> Self {
            Self {
                policy_id: policy_id.to_owned(),
                unknown_behavior: UnknownRouteBehavior::DenyByDefault,
                unknown_http_status: 403,
                allowed_methods: BTreeSet::new(),
            }
        }

        #[must_use]
        pub fn allow_method(mut self, method: &str) -> Self {
            self.allowed_methods.insert(method.to_owned());
            self
        }

        #[must_use]
        pub fn decide_method(&self, method: &str) -> RouteDecision {
            if self.allowed_methods.contains(method) {
                return RouteDecision {
                    policy_id: self.policy_id.clone(),
                    method: method.to_owned(),
                    allowed: true,
                    reason_code: REASON_ALLOW,
                    http_status: 200,
                    compatibility_fallback_used: false,
                };
            }

            let reason = match self.unknown_behavior {
                UnknownRouteBehavior::UnknownMethod => REASON_UNKNOWN_METHOD,
                UnknownRouteBehavior::DenyByDefault => REASON_ROUTE_DENY_BY_DEFAULT,
            };

            RouteDecision {
                policy_id: self.policy_id.clone(),
                method: method.to_owned(),
                allowed: false,
                reason_code: reason,
                http_status: self.unknown_http_status,
                compatibility_fallback_used: false,
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct LimitViolation {
        code: &'static str,
        limit_bytes: u64,
    }

    impl LimitViolation {
        #[must_use]
        pub fn code(&self) -> &str {
            self.code
        }

        #[must_use]
        pub fn limit_bytes(&self) -> u64 {
            self.limit_bytes
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct PayloadLimitPolicy {
        limit_bytes: u64,
    }

    impl PayloadLimitPolicy {
        #[must_use]
        pub fn new(limit_bytes: u64) -> Self {
            Self { limit_bytes }
        }

        pub fn validate_request_bytes(
            &self,
            request_size_bytes: u64,
        ) -> Result<(), LimitViolation> {
            if request_size_bytes > self.limit_bytes {
                return Err(LimitViolation {
                    code: "PAYLOAD_TOO_LARGE",
                    limit_bytes: self.limit_bytes,
                });
            }
            Ok(())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct OutputLimitPolicy {
        limit_bytes: u64,
    }

    impl OutputLimitPolicy {
        #[must_use]
        pub fn new(limit_bytes: u64) -> Self {
            Self { limit_bytes }
        }

        pub fn validate_output_bytes(&self, output_size_bytes: u64) -> Result<(), LimitViolation> {
            if output_size_bytes > self.limit_bytes {
                return Err(LimitViolation {
                    code: "OUTPUT_TOO_LARGE",
                    limit_bytes: self.limit_bytes,
                });
            }
            Ok(())
        }
    }
}

pub mod auth {
    use std::collections::BTreeMap;

    use super::policy::RoutePolicy;

    const REASON_ALLOW: &str = "ALLOW";
    const REASON_AUTH_REQUIRED: &str = "AUTH_REQUIRED";
    const REASON_AUTH_INVALID_TOKEN: &str = "AUTH_INVALID_TOKEN";
    const REASON_ROLE_FORBIDDEN: &str = "ROLE_FORBIDDEN";

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Role {
        Operator,
        Admin,
    }

    impl Role {
        #[must_use]
        fn level(self) -> u8 {
            match self {
                Self::Operator => 1,
                Self::Admin => 2,
            }
        }

        #[must_use]
        fn satisfies(self, required: Role) -> bool {
            self.level() >= required.level()
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum AuthContext {
        Missing,
        BearerToken(String),
    }

    impl AuthContext {
        #[must_use]
        pub fn missing() -> Self {
            Self::Missing
        }

        #[must_use]
        pub fn bearer(token: &str) -> Self {
            Self::BearerToken(token.to_owned())
        }

        #[must_use]
        fn token(&self) -> Option<&str> {
            match self {
                Self::Missing => None,
                Self::BearerToken(token) => Some(token),
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct AuthorizationDecision {
        allowed: bool,
        reason_code: String,
        http_status: u16,
        principal_id: Option<String>,
    }

    impl AuthorizationDecision {
        #[must_use]
        pub fn is_allowed(&self) -> bool {
            self.allowed
        }

        #[must_use]
        pub fn is_denied(&self) -> bool {
            !self.allowed
        }

        #[must_use]
        pub fn reason_code(&self) -> &str {
            &self.reason_code
        }

        #[must_use]
        pub fn http_status(&self) -> u16 {
            self.http_status
        }

        #[must_use]
        pub fn principal_id(&self) -> Option<&str> {
            self.principal_id.as_deref()
        }

        #[must_use]
        fn allow(principal_id: String) -> Self {
            Self {
                allowed: true,
                reason_code: REASON_ALLOW.to_owned(),
                http_status: 200,
                principal_id: Some(principal_id),
            }
        }

        #[must_use]
        fn deny(reason_code: &str, http_status: u16) -> Self {
            Self {
                allowed: false,
                reason_code: reason_code.to_owned(),
                http_status,
                principal_id: None,
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct PrincipalRecord {
        principal_id: String,
        role: Role,
    }

    #[derive(Clone, Debug)]
    pub struct RuntimeAuthorizer {
        policy: RoutePolicy,
        principals_by_token: BTreeMap<String, PrincipalRecord>,
    }

    impl RuntimeAuthorizer {
        #[must_use]
        pub fn new(policy: RoutePolicy) -> Self {
            Self { policy, principals_by_token: BTreeMap::new() }
        }

        pub fn register_principal_token(&mut self, token: &str, principal: &str, role: Role) {
            self.principals_by_token.insert(
                token.to_owned(),
                PrincipalRecord { principal_id: principal.to_owned(), role },
            );
        }

        #[must_use]
        pub fn authorize(
            &self,
            auth: &AuthContext,
            method: &str,
            required_role: Role,
        ) -> AuthorizationDecision {
            let token = match auth.token() {
                Some(token) => token,
                None => return AuthorizationDecision::deny(REASON_AUTH_REQUIRED, 401),
            };

            let principal = match self.principals_by_token.get(token) {
                Some(principal) => principal,
                None => return AuthorizationDecision::deny(REASON_AUTH_INVALID_TOKEN, 401),
            };

            if !principal.role.satisfies(required_role) {
                return AuthorizationDecision::deny(REASON_ROLE_FORBIDDEN, 403);
            }

            let route_decision = self.policy.decide_method(method);
            if route_decision.is_denied() {
                return AuthorizationDecision::deny(
                    route_decision.reason_code(),
                    route_decision.http_status(),
                );
            }

            AuthorizationDecision::allow(principal.principal_id.clone())
        }
    }
}

pub mod audit {
    use super::policy::RouteDecision;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AuditEventKind {
        AccessDenied,
        AccessAllowed,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct AuditEvent {
        kind: AuditEventKind,
        actor_id: String,
        policy_id: String,
        method: String,
        outcome: &'static str,
    }

    impl AuditEvent {
        #[must_use]
        pub fn from_route_decision(
            _request_id: &str,
            actor_id: &str,
            decision: &RouteDecision,
        ) -> Self {
            if decision.is_allowed() {
                return Self {
                    kind: AuditEventKind::AccessAllowed,
                    actor_id: actor_id.to_owned(),
                    policy_id: decision.policy_id().to_owned(),
                    method: decision.method().to_owned(),
                    outcome: "allow",
                };
            }

            Self {
                kind: AuditEventKind::AccessDenied,
                actor_id: actor_id.to_owned(),
                policy_id: decision.policy_id().to_owned(),
                method: decision.method().to_owned(),
                outcome: "deny",
            }
        }

        #[must_use]
        pub fn kind(&self) -> AuditEventKind {
            self.kind
        }

        #[must_use]
        pub fn actor_id(&self) -> &str {
            &self.actor_id
        }

        #[must_use]
        pub fn policy_id(&self) -> &str {
            &self.policy_id
        }

        #[must_use]
        pub fn method(&self) -> &str {
            &self.method
        }

        #[must_use]
        pub fn outcome(&self) -> &str {
            self.outcome
        }
    }
}
