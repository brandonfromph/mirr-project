#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StableError {
    pub error_code: &'static str,
    pub message: &'static str,
}

impl StableError {
    pub const fn new(error_code: &'static str, message: &'static str) -> Self {
        Self { error_code, message }
    }
}

pub const UNKNOWN_METHOD_ERROR: StableError =
    StableError::new("validation_unknown_method", "MCP unknown method rejected.");
pub const MISSING_API_KEY_ERROR: StableError =
    StableError::new("auth_missing_api_key", "API key is required.");
pub const INVALID_API_KEY_ERROR: StableError =
    StableError::new("auth_invalid_api_key", "API key is invalid.");
pub const INSUFFICIENT_ROLE_ERROR: StableError =
    StableError::new("auth_insufficient_role", "API key role is not allowed.");

pub const SCHEMA_VALIDATION_ERROR: StableError =
    StableError::new("validation_schema", "Request body failed schema validation.");
pub const CONCURRENCY_LIMIT_ERROR: StableError =
    StableError::new("limit_concurrency_exceeded", "Concurrency limit exceeded.");
pub const TOKEN_QUOTA_LIMIT_ERROR: StableError =
    StableError::new("limit_token_quota_exceeded", "Token request quota exceeded.");
pub const MRT_EXECUTION_ERROR: StableError =
    StableError::new("validation_mrt_exec_failed", "MRT execution failed.");

pub const STABLE_ERROR_ENVELOPE_MAP: &[StableError] = &[
    UNKNOWN_METHOD_ERROR,
    MISSING_API_KEY_ERROR,
    INVALID_API_KEY_ERROR,
    INSUFFICIENT_ROLE_ERROR,
    SCHEMA_VALIDATION_ERROR,
    CONCURRENCY_LIMIT_ERROR,
    TOKEN_QUOTA_LIMIT_ERROR,
    MRT_EXECUTION_ERROR,
];

pub fn stable_error_by_code(error_code: &str) -> Option<StableError> {
    STABLE_ERROR_ENVELOPE_MAP.iter().copied().find(|entry| entry.error_code == error_code)
}
