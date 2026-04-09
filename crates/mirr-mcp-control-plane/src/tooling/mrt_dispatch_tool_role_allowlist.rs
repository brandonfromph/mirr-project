#![forbid(unsafe_code)]

use crate::policy::Role;

use super::mrt_dispatch_tool_name::MrtDispatchTool;

const ROLE_ADMIN_ONLY: &[Role] = &[Role::Admin];
const ROLE_BUILDER_ADMIN: &[Role] = &[Role::Builder, Role::Admin];
const ROLE_COMMITTER_ADMIN: &[Role] = &[Role::Committer, Role::Admin];
const ROLE_BUILDER_COMMITTER_ADMIN: &[Role] = &[Role::Builder, Role::Committer, Role::Admin];

impl MrtDispatchTool {
    pub const fn role_allowlist(self) -> &'static [Role] {
        match self {
            Self::MrtAudit => ROLE_BUILDER_COMMITTER_ADMIN,
            Self::MrtBrainGet => ROLE_COMMITTER_ADMIN,
            Self::MrtGeneralCi => ROLE_BUILDER_ADMIN,
            Self::MrtGeneralCiCompile => ROLE_BUILDER_ADMIN,
            Self::MrtGeneralCiFast => ROLE_BUILDER_ADMIN,
            Self::MrtWaveDryRun => ROLE_BUILDER_COMMITTER_ADMIN,
            Self::MrtWaveApply => ROLE_ADMIN_ONLY,
            Self::MrtLspDiagnostics => ROLE_BUILDER_COMMITTER_ADMIN,
            Self::MrtCompile => ROLE_BUILDER_ADMIN,
            Self::MrtRspuValidate => ROLE_BUILDER_COMMITTER_ADMIN,
            Self::MrtRspuProofs => ROLE_BUILDER_ADMIN,
            Self::LraInit => ROLE_COMMITTER_ADMIN,
            Self::LraValidate => ROLE_BUILDER_COMMITTER_ADMIN,
            Self::LraServe => ROLE_BUILDER_COMMITTER_ADMIN,
            Self::LraCheck => ROLE_BUILDER_COMMITTER_ADMIN,
            Self::LraSign => ROLE_COMMITTER_ADMIN,
            Self::LraVerify => ROLE_BUILDER_COMMITTER_ADMIN,
        }
    }
}
