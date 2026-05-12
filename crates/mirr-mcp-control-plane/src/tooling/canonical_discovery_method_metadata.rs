#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveryParameter {
    pub name: &'static str,
    pub required: bool,
    pub ty: &'static str,
}

impl DiscoveryParameter {
    pub const fn new(name: &'static str, required: bool, ty: &'static str) -> Self {
        Self { name, required, ty }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveryMethodMetadata {
    pub name: &'static str,
    pub auto_approve: bool,
    pub description: &'static str,
    pub parameters: &'static [DiscoveryParameter],
}

impl DiscoveryMethodMetadata {
    pub const fn new(
        name: &'static str,
        auto_approve: bool,
        description: &'static str,
        parameters: &'static [DiscoveryParameter],
    ) -> Self {
        Self { name, auto_approve, description, parameters }
    }
}

const READ_TEXT_FILE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("head", false, "number"),
    DiscoveryParameter::new("tail", false, "number"),
];

const WRITE_FILE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("content", true, "string"),
];

const EDIT_FILE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("edits", true, "array"),
    DiscoveryParameter::new("dryRun", false, "boolean"),
];

const CREATE_DIRECTORY_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("path", true, "string")];
const LIST_DIRECTORY_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("path", true, "string")];

const LIST_DIRECTORY_WITH_SIZES_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("sortBy", false, "string"),
];

const DIRECTORY_TREE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("excludePatterns", false, "array"),
];

const MOVE_FILE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("source", true, "string"),
    DiscoveryParameter::new("destination", true, "string"),
];

const SEARCH_FILES_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("pattern", false, "string"),
    DiscoveryParameter::new("excludePatterns", false, "array"),
];

const GET_FILE_INFO_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("path", true, "string")];

const MRT_AUDIT_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("mode", false, "string"),
    DiscoveryParameter::new("glob", false, "string"),
];

const MRT_BRAIN_GET_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("key", true, "string")];

const MRT_WAVE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("proposal_id", false, "string"),
    DiscoveryParameter::new("proposalId", false, "string"),
    DiscoveryParameter::new("proposal_file", false, "string"),
    DiscoveryParameter::new("proposalFile", false, "string"),
    DiscoveryParameter::new("max_lines", false, "number"),
    DiscoveryParameter::new("maxLines", false, "number"),
];

const MRT_LSP_DIAGNOSTICS_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("source", true, "string")];

const MRT_COMPILE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("source_file", true, "string"),
    DiscoveryParameter::new("sourceFile", false, "string"),
    DiscoveryParameter::new("target", false, "string"),
    DiscoveryParameter::new("max_size", false, "number"),
    DiscoveryParameter::new("maxSize", false, "number"),
];

const MRT_RSPU_VALIDATE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("proof_path", true, "string"),
    DiscoveryParameter::new("proofPath", false, "string"),
    DiscoveryParameter::new("mode", false, "string"),
];

const MRT_RSPU_PROOFS_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("source_file", true, "string"),
    DiscoveryParameter::new("sourceFile", false, "string"),
    DiscoveryParameter::new("methods", false, "array"),
];

const MRT_DAEMON_CONTRACT_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("test_filter", false, "string")];

const LRA_INIT_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("project_name", true, "string"),
    DiscoveryParameter::new("projectName", false, "string"),
];

const LRA_VALIDATE_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("target_path", false, "string"),
];

const LRA_SERVE_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("port", false, "number")];

const LRA_CHECK_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("target_path", false, "string"),
];

const LRA_SIGN_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("receipt", true, "string"),
    DiscoveryParameter::new("receipt_path", false, "string"),
    DiscoveryParameter::new("key", false, "string"),
    DiscoveryParameter::new("key_path", false, "string"),
];

const LRA_VERIFY_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("path", true, "string"),
    DiscoveryParameter::new("target", false, "string"),
];

const MRT_KB_QUERY_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("query", true, "string"),
    DiscoveryParameter::new("mode", false, "string"),
    DiscoveryParameter::new("limit", false, "number"),
    DiscoveryParameter::new("filter", false, "string"),
    DiscoveryParameter::new("expand_mode", false, "string"),
    DiscoveryParameter::new("retry_count", false, "number"),
    DiscoveryParameter::new("timeout_ms", false, "number"),
];

const MRT_KB_INDEX_PARAMETERS: &[DiscoveryParameter] =
    &[DiscoveryParameter::new("path", false, "string")];

const MRT_KB_BRIEF_PARAMETERS: &[DiscoveryParameter] = &[
    DiscoveryParameter::new("query", true, "string"),
    DiscoveryParameter::new("mode", false, "string"),
    DiscoveryParameter::new("limit", false, "number"),
    DiscoveryParameter::new("scope", false, "string"),
    DiscoveryParameter::new("format", false, "string"),
];

const MRT_KB_INDEX_STATUS_PARAMETERS: &[DiscoveryParameter] = &[];
pub const CANONICAL_DISCOVERY_METHOD_METADATA: &[DiscoveryMethodMetadata] = &[
    DiscoveryMethodMetadata::new(
        "read_text_file",
        true,
        "Read the complete contents of a file from the file system as text. Handles various encodings and provides detailed errors. Use head/tail parameters.",
        READ_TEXT_FILE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "write_file",
        true,
        "Create or overwrite a file with new content. Operates within allowed dirs.",
        WRITE_FILE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "edit_file",
        true,
        "Perform line-based edits to a text file and return a git-style diff.",
        EDIT_FILE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "create_directory",
        true,
        "Ensure a directory exists, creating parent directories as needed.",
        CREATE_DIRECTORY_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "list_directory",
        true,
        "List files and directories at a path, marking types.",
        LIST_DIRECTORY_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "list_directory_with_sizes",
        true,
        "Like list_directory but include sizes, with optional sorting.",
        LIST_DIRECTORY_WITH_SIZES_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "directory_tree",
        true,
        "Return JSON tree of directories/files recursively.",
        DIRECTORY_TREE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "move_file",
        true,
        "Move or rename a file or directory within allowed paths.",
        MOVE_FILE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "search_files",
        true,
        "Recursively search using glob patterns starting at path.",
        SEARCH_FILES_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "get_file_info",
        true,
        "Retrieve metadata about a file or directory.",
        GET_FILE_INFO_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "list_allowed_directories",
        true,
        "Return directories the server may access according to config.",
        &[],
    ),
    DiscoveryMethodMetadata::new(
        "mrt_audit",
        false,
        "Run mirr-audit with MRT role allowlist enforcement.",
        MRT_AUDIT_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_brain_get",
        false,
        "Run mirr-brain get with MRT role allowlist enforcement.",
        MRT_BRAIN_GET_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_general_ci",
        false,
        "Run mirr-general ci with MRT role allowlist enforcement.",
        &[],
    ),
    DiscoveryMethodMetadata::new(
        "mrt_general_ci_compile",
        false,
        "Run mirr-general ci compile gate with MRT role allowlist enforcement.",
        &[],
    ),
    DiscoveryMethodMetadata::new(
        "mrt_general_ci_fast",
        false,
        "Run mirr-general ci fast gate with MRT role allowlist enforcement.",
        &[],
    ),
    DiscoveryMethodMetadata::new(
        "mrt_wave_dry_run",
        false,
        "Run mirr-wave in dry-run mode with bounded proposal inputs.",
        MRT_WAVE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_wave_apply",
        false,
        "Run mirr-wave apply mode with bounded proposal inputs.",
        MRT_WAVE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_lsp_diagnostics",
        false,
        "Run mirr-lsp diagnostics on a bounded source string through stdin.",
        MRT_LSP_DIAGNOSTICS_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_compile",
        false,
        "Invoke compiler pipeline for MIRR source with bounded options (target, size limit).",
        MRT_COMPILE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_rspu_validate",
        false,
        "Validate R-SPU (Rust SPU) proofs with strict or permissive mode.",
        MRT_RSPU_VALIDATE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_rspu_proofs",
        false,
        "Execute proof synthesis with optional method restrictions.",
        MRT_RSPU_PROOFS_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_daemon_core_contract",
        false,
        "Run daemon core architecture contract tests (wave5) with optional test filter.",
        MRT_DAEMON_CONTRACT_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_daemon_security_contract",
        false,
        "Run daemon security/runtime policy contract tests (wave6) with optional test filter.",
        MRT_DAEMON_CONTRACT_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "lra_init",
        false,
        "Initialize a new LRA (Living Research Artifact) project with scaffold.",
        LRA_INIT_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "lra_validate",
        false,
        "Validate LRA compliance (Bronze/Silver/Gold) for an HTML paper.",
        LRA_VALIDATE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "lra_serve",
        false,
        "Start a local LRA dev server with live reload.",
        LRA_SERVE_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "lra_check",
        false,
        "Check LRA compliance (alias for validate).",
        LRA_CHECK_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "lra_sign",
        false,
        "Sign a verification receipt with an Ed25519 keypair.",
        LRA_SIGN_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "lra_verify",
        false,
        "Verify a deployed LRA paper and validate content integrity.",
        LRA_VERIFY_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_kb_query",
        false,
        "Query KB using lexical, semantic, or hybrid retrieval with deterministic fallback.",
        MRT_KB_QUERY_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_kb_index",
        false,
        "Build or refresh the KB index from files under a target path.",
        MRT_KB_INDEX_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_kb_index_status",
        false,
        "Get KB index freshness, size, and last refresh timestamp.",
        MRT_KB_INDEX_STATUS_PARAMETERS,
    ),
    DiscoveryMethodMetadata::new(
        "mrt_kb_brief",
        false,
        "Produce a grounded KB briefing with cited evidence, gaps, and follow-up queries.",
        MRT_KB_BRIEF_PARAMETERS,
    ),
];

pub fn discovery_method_by_name(name: &str) -> Option<&'static DiscoveryMethodMetadata> {
    CANONICAL_DISCOVERY_METHOD_METADATA.iter().find(|method| method.name == name)
}
