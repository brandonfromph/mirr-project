#![forbid(unsafe_code)]

use crate::envelope::{
    StableError, INSUFFICIENT_ROLE_ERROR, INVALID_API_KEY_ERROR, MISSING_API_KEY_ERROR,
    UNKNOWN_METHOD_ERROR,
};

use super::rpc_role_gate::RoleCheckFailure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoleFailureEnvelope {
    pub status_code: u16,
    pub error: StableError,
}

pub fn role_failure_to_envelope(failure: &RoleCheckFailure) -> RoleFailureEnvelope {
    match failure {
        RoleCheckFailure::MissingApiKey => {
            RoleFailureEnvelope { status_code: 401, error: MISSING_API_KEY_ERROR }
        }
        RoleCheckFailure::InvalidApiKey => {
            RoleFailureEnvelope { status_code: 403, error: INVALID_API_KEY_ERROR }
        }
        RoleCheckFailure::ValidationUnknownMethod => {
            RoleFailureEnvelope { status_code: 400, error: UNKNOWN_METHOD_ERROR }
        }
        RoleCheckFailure::InsufficientRole { .. } => {
            RoleFailureEnvelope { status_code: 403, error: INSUFFICIENT_ROLE_ERROR }
        }
    }
}
