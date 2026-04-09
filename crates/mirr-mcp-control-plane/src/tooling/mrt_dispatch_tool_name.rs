#![forbid(unsafe_code)]

use super::mrt_dispatch_tool_alias::canonical_dispatch_tool_name;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
    LraInit,
    LraValidate,
    LraServe,
    LraCheck,
    LraSign,
    LraVerify,
}

impl MrtDispatchTool {
    pub const ALL: [Self; 17] = [
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
        Self::LraInit,
        Self::LraValidate,
        Self::LraServe,
        Self::LraCheck,
        Self::LraSign,
        Self::LraVerify,
    ];

    pub const fn as_str(self) -> &'static str {
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
            Self::LraInit => "lra_init",
            Self::LraValidate => "lra_validate",
            Self::LraServe => "lra_serve",
            Self::LraCheck => "lra_check",
            Self::LraSign => "lra_sign",
            Self::LraVerify => "lra_verify",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match canonical_dispatch_tool_name(value) {
            "mrt_audit" => Some(Self::MrtAudit),
            "mrt_brain_get" => Some(Self::MrtBrainGet),
            "mrt_general_ci" => Some(Self::MrtGeneralCi),
            "mrt_general_ci_compile" => Some(Self::MrtGeneralCiCompile),
            "mrt_general_ci_fast" => Some(Self::MrtGeneralCiFast),
            "mrt_wave_dry_run" => Some(Self::MrtWaveDryRun),
            "mrt_wave_apply" => Some(Self::MrtWaveApply),
            "mrt_lsp_diagnostics" => Some(Self::MrtLspDiagnostics),
            "mrt_compile" => Some(Self::MrtCompile),
            "mrt_rspu_validate" => Some(Self::MrtRspuValidate),
            "mrt_rspu_proofs" => Some(Self::MrtRspuProofs),
            "lra_init" => Some(Self::LraInit),
            "lra_validate" => Some(Self::LraValidate),
            "lra_serve" => Some(Self::LraServe),
            "lra_check" => Some(Self::LraCheck),
            "lra_sign" => Some(Self::LraSign),
            "lra_verify" => Some(Self::LraVerify),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MrtDispatchTool;

    #[test]
    fn all_tools_roundtrip() {
        for tool in MrtDispatchTool::ALL {
            assert_eq!(MrtDispatchTool::from_str(tool.as_str()), Some(tool));
        }
    }

    #[test]
    fn unknown_tool_is_none() {
        assert_eq!(MrtDispatchTool::from_str("mrt_unknown"), None);
    }

    #[test]
    fn prefixed_lra_route_names_resolve_to_canonical_tools() {
        assert_eq!(MrtDispatchTool::from_str("mrt_lra_init"), Some(MrtDispatchTool::LraInit));
        assert_eq!(
            MrtDispatchTool::from_str("mrt_lra_validate"),
            Some(MrtDispatchTool::LraValidate)
        );
        assert_eq!(MrtDispatchTool::from_str("mrt_lra_serve"), Some(MrtDispatchTool::LraServe));
        assert_eq!(MrtDispatchTool::from_str("mrt_lra_check"), Some(MrtDispatchTool::LraCheck));
        assert_eq!(MrtDispatchTool::from_str("mrt_lra_sign"), Some(MrtDispatchTool::LraSign));
        assert_eq!(MrtDispatchTool::from_str("mrt_lra_verify"), Some(MrtDispatchTool::LraVerify));
    }
}
