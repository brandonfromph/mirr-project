#![forbid(unsafe_code)]

use std::str::FromStr;

use super::mrt_dispatch_tool_alias::canonical_dispatch_tool_name;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MrtDispatchTool {
    MrtAudit,
    MrtBrainGet,
    MrtGeneralCi,
    MrtGeneralCiCompile,
    MrtGeneralCiFast,
    MrtWaveDryRun,
    MrtWaveApply,
    MrtLspDiagnostics,
    MrtCompile,
    MrtRspuValidate,
    MrtRspuProofs,
    MrtDaemonCoreContract,
    MrtDaemonSecurityContract,
    LraInit,
    LraValidate,
    LraServe,
    LraCheck,
    LraSign,
    LraVerify,
    MrtKbQuery,
    MrtKbIndex,
    MrtKbIndexStatus,
    MrtKbBrief,
    Dynamic(String),
}

impl MrtDispatchTool {
    pub const LEGACY_ALL: &[Self] = &[
        Self::MrtAudit,
        Self::MrtBrainGet,
        Self::MrtGeneralCi,
        Self::MrtGeneralCiCompile,
        Self::MrtGeneralCiFast,
        Self::MrtWaveDryRun,
        Self::MrtWaveApply,
        Self::MrtLspDiagnostics,
        Self::MrtCompile,
        Self::MrtRspuValidate,
        Self::MrtRspuProofs,
        Self::MrtDaemonCoreContract,
        Self::MrtDaemonSecurityContract,
        Self::LraInit,
        Self::LraValidate,
        Self::LraServe,
        Self::LraCheck,
        Self::LraSign,
        Self::LraVerify,
        Self::MrtKbQuery,
        Self::MrtKbIndex,
        Self::MrtKbIndexStatus,
        Self::MrtKbBrief,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MrtAudit => "mrt_audit",
            Self::MrtBrainGet => "mrt_brain_get",
            Self::MrtGeneralCi => "mrt_general_ci",
            Self::MrtGeneralCiCompile => "mrt_general_ci_compile",
            Self::MrtGeneralCiFast => "mrt_general_ci_fast",
            Self::MrtWaveDryRun => "mrt_wave_dry_run",
            Self::MrtWaveApply => "mrt_wave_apply",
            Self::MrtLspDiagnostics => "mrt_lsp_diagnostics",
            Self::MrtCompile => "mrt_compile",
            Self::MrtRspuValidate => "mrt_rspu_validate",
            Self::MrtRspuProofs => "mrt_rspu_proofs",
            Self::MrtDaemonCoreContract => "mrt_daemon_core_contract",
            Self::MrtDaemonSecurityContract => "mrt_daemon_security_contract",
            Self::LraInit => "lra_init",
            Self::LraValidate => "lra_validate",
            Self::LraServe => "lra_serve",
            Self::LraCheck => "lra_check",
            Self::LraSign => "lra_sign",
            Self::LraVerify => "lra_verify",
            Self::MrtKbQuery => "mrt_kb_query",
            Self::MrtKbIndex => "mrt_kb_index",
            Self::MrtKbIndexStatus => "mrt_kb_index_status",
            Self::MrtKbBrief => "mrt_kb_brief",
            Self::Dynamic(_) => "dynamic",
        }
    }
}

impl FromStr for MrtDispatchTool {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match canonical_dispatch_tool_name(value) {
            "mrt_audit" => Ok(Self::MrtAudit),
            "mrt_brain_get" => Ok(Self::MrtBrainGet),
            "mrt_general_ci" => Ok(Self::MrtGeneralCi),
            "mrt_general_ci_compile" => Ok(Self::MrtGeneralCiCompile),
            "mrt_general_ci_fast" => Ok(Self::MrtGeneralCiFast),
            "mrt_wave_dry_run" => Ok(Self::MrtWaveDryRun),
            "mrt_wave_apply" => Ok(Self::MrtWaveApply),
            "mrt_lsp_diagnostics" => Ok(Self::MrtLspDiagnostics),
            "mrt_compile" => Ok(Self::MrtCompile),
            "mrt_rspu_validate" => Ok(Self::MrtRspuValidate),
            "mrt_rspu_proofs" => Ok(Self::MrtRspuProofs),
            "mrt_daemon_core_contract" => Ok(Self::MrtDaemonCoreContract),
            "mrt_daemon_security_contract" => Ok(Self::MrtDaemonSecurityContract),
            "lra_init" => Ok(Self::LraInit),
            "lra_validate" => Ok(Self::LraValidate),
            "lra_serve" => Ok(Self::LraServe),
            "lra_check" => Ok(Self::LraCheck),
            "lra_sign" => Ok(Self::LraSign),
            "lra_verify" => Ok(Self::LraVerify),
            "mrt_kb_query" => Ok(Self::MrtKbQuery),
            "mrt_kb_index" => Ok(Self::MrtKbIndex),
            "mrt_kb_index_status" => Ok(Self::MrtKbIndexStatus),
            "mrt_kb_brief" => Ok(Self::MrtKbBrief),
            _ => {
                // If it's one of our discovered tools, treat it as dynamic
                if ["mirr-compile", "mirr-brain", "mirr-simulate", "mirr-audit"].contains(&value) {
                    Ok(Self::Dynamic(value.to_string()))
                } else {
                    Err(format!("invalid tool: {}", value))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MrtDispatchTool;

    #[test]
    fn all_tools_roundtrip() {
        for tool in MrtDispatchTool::LEGACY_ALL {
            assert_eq!(tool.as_str().parse::<MrtDispatchTool>(), Ok(tool));
        }
    }

    #[test]
    fn unknown_tool_is_none() {
        assert!("mrt_unknown".parse::<MrtDispatchTool>().is_err());
    }

    #[test]
    fn prefixed_lra_route_names_resolve_to_canonical_tools() {
        assert_eq!("mrt_lra_init".parse::<MrtDispatchTool>(), Ok(MrtDispatchTool::LraInit));
        assert_eq!("mrt_lra_validate".parse::<MrtDispatchTool>(), Ok(MrtDispatchTool::LraValidate));
        assert_eq!("mrt_lra_serve".parse::<MrtDispatchTool>(), Ok(MrtDispatchTool::LraServe));
        assert_eq!("mrt_lra_check".parse::<MrtDispatchTool>(), Ok(MrtDispatchTool::LraCheck));
        assert_eq!("mrt_lra_sign".parse::<MrtDispatchTool>(), Ok(MrtDispatchTool::LraSign));
        assert_eq!("mrt_lra_verify".parse::<MrtDispatchTool>(), Ok(MrtDispatchTool::LraVerify));
    }
}
