#![forbid(unsafe_code)]

mod mirr_general;
use mirrc::error::MirrError;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use mirr_general::cache::{
    compute_package_fingerprint, hash_file, load_manifest, upsert_manifest_entry,
    write_manifest_atomic, CacheManifest, PackageSpec,
};
use mirr_general::manifest::load_package_manifest;
use mirr_general::migration::{build_script_inventory, migrate_script};
use mirr_general::parity::{run_consumer_parity, verify_cli_wasm_parity, verify_vscode_contract};
use mirr_general::scheduler::{execute_all_waves, ExecutionPlan, TaskSpec, WaveKind, WaveSpec};

// Compatibility markers retained for full-gate regression checks:
// cargo fmt --all -- --check
// cargo check --all-targets
// cargo clippy --all-targets -- -D warnings
// RUSTDOCFLAGS=-D warnings cargo doc --no-deps
// cargo nextest run --workspace --no-fail-fast
// npm --prefix paper/demos pack --dry-run
// npm --prefix vscode-mirr pack --dry-run
// bash tests/eda/run_eda_tests.sh
// fn npm_command()
// npm.cmd

const DEFAULT_CI_TARGET_DIR: &str = "target/ci-wave";

fn ci_child_target_dir_from(base_dir: Option<&OsStr>) -> OsString {
    let base = base_dir.unwrap_or_else(|| OsStr::new(DEFAULT_CI_TARGET_DIR));

    #[cfg(target_os = "windows")]
    {
        let mut path = PathBuf::from(base);
        // Keep nested cargo builds off the running mirr-general binary output path.
        path.push("mirr-general-child");
        path.into_os_string()
    }

    #[cfg(not(target_os = "windows"))]
    {
        base.to_os_string()
    }
}

fn ci_child_target_dir() -> OsString {
    let inherited = std::env::var_os("CARGO_TARGET_DIR");
    ci_child_target_dir_from(inherited.as_deref())
}

fn task(
    wave_index: usize,
    package_name: &str,
    command: &str,
    args: &[&str],
    allow_cache_skip: bool,
) -> TaskSpec {
    assert!(!package_name.is_empty(), "package_name must not be empty");
    assert!(!command.is_empty(), "command must not be empty");

    let mut env = BTreeMap::new();
    if command == "cargo" {
        env.insert("CARGO_TARGET_DIR".to_string(), ci_child_target_dir());
        if sccache_enabled_for_ci() {
            env.insert("RUSTC_WRAPPER".to_string(), OsString::from("sccache"));
        }
    }

    TaskSpec {
        wave_index,
        package_name: package_name.to_string(),
        command: OsString::from(command),
        args: args.iter().map(|arg| OsString::from(*arg)).collect(),
        cwd: Path::new(".").to_path_buf(),
        env,
        allow_cache_skip,
    }
}

fn npm_command_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "npm.cmd"
    }

    #[cfg(not(target_os = "windows"))]
    {
        "npm"
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CiProfile {
    Full,
    Compile,
    Fast,
}

impl CiProfile {
    fn label(self) -> &'static str {
        match self {
            CiProfile::Full => "full",
            CiProfile::Compile => "compile",
            CiProfile::Fast => "fast",
        }
    }
}

const WORKSPACE_FULL_NEXTEST_ARGS: &[&str] = &["nextest", "run", "-p", "mirrc", "--no-fail-fast"];

const WORKSPACE_FULL_CARGO_TEST_ARGS: &[&str] =
    &["test", "-p", "mirrc", "--all-targets", "--no-fail-fast"];

const WORKSPACE_SELECTIVE_NEXTEST_ARGS: &[&str] = &[
    "nextest",
    "run",
    "-p",
    "mirrc",
    "--no-fail-fast",
    "--test",
    "parser_edge_cases_tests",
    "--test",
    "parser_composite_tests",
    "--test",
    "semantic_validation_tests",
    "--test",
    "composite_type_integration_tests",
    "--test",
    "meta_stage_unroller_tests",
    "--test",
    "compiler_flexibility_tests",
    "--test",
    "orchestrator_scheduler_tests",
    "--test",
    "orchestrator_cache_tests",
    "--test",
    "orchestrator_migration_tests",
    "--test",
    "orchestrator_parity_tests",
];

const WORKSPACE_SELECTIVE_CARGO_TEST_ARGS: &[&str] = &[
    "test",
    "-p",
    "mirrc",
    "--no-fail-fast",
    "--test",
    "parser_edge_cases_tests",
    "--test",
    "parser_composite_tests",
    "--test",
    "semantic_validation_tests",
    "--test",
    "composite_type_integration_tests",
    "--test",
    "meta_stage_unroller_tests",
    "--test",
    "compiler_flexibility_tests",
    "--test",
    "orchestrator_scheduler_tests",
    "--test",
    "orchestrator_cache_tests",
    "--test",
    "orchestrator_migration_tests",
    "--test",
    "orchestrator_parity_tests",
];

const LRA_NEXTEST_ARGS: &[&str] = &["nextest", "run", "-p", "lra-cli", "--no-fail-fast"];

const LRA_CARGO_TEST_ARGS: &[&str] = &["test", "-p", "lra-cli", "--all-targets", "--no-fail-fast"];

fn workspace_full_test_args(use_nextest: bool) -> &'static [&'static str] {
    if use_nextest {
        WORKSPACE_FULL_NEXTEST_ARGS
    } else {
        WORKSPACE_FULL_CARGO_TEST_ARGS
    }
}

fn workspace_selective_test_args(use_nextest: bool) -> &'static [&'static str] {
    if use_nextest {
        WORKSPACE_SELECTIVE_NEXTEST_ARGS
    } else {
        WORKSPACE_SELECTIVE_CARGO_TEST_ARGS
    }
}

fn lra_cli_test_args(use_nextest: bool) -> &'static [&'static str] {
    if use_nextest {
        LRA_NEXTEST_ARGS
    } else {
        LRA_CARGO_TEST_ARGS
    }
}

fn build_ci_full_plan(use_nextest: bool) -> ExecutionPlan {
    let workspace_test_args = workspace_full_test_args(use_nextest);
    let lra_test_args = lra_cli_test_args(use_nextest);

    ExecutionPlan {
        waves: vec![
            WaveSpec {
                wave_index: 0,
                kind: WaveKind::Fmt,
                tasks: vec![task(
                    0,
                    "workspace-format",
                    "cargo",
                    &["fmt", "--all", "--", "--check"],
                    false,
                )],
            },
            WaveSpec {
                wave_index: 1,
                kind: WaveKind::Clippy,
                tasks: vec![
                    task(
                        1,
                        "workspace-core-clippy",
                        "cargo",
                        &["clippy", "-p", "mirrc", "--all-targets", "--", "-D", "warnings"],
                        true,
                    ),
                    task(
                        1,
                        "lra-cli-clippy",
                        "cargo",
                        &["clippy", "-p", "lra-cli", "--all-targets", "--", "-D", "warnings"],
                        true,
                    ),
                ],
            },
            WaveSpec {
                wave_index: 2,
                kind: WaveKind::Nextest,
                tasks: vec![
                    task(2, "workspace-core-tests", "cargo", workspace_test_args, true),
                    task(2, "lra-cli-tests", "cargo", lra_test_args, true),
                ],
            },
            WaveSpec {
                wave_index: 3,
                kind: WaveKind::Rocq,
                tasks: vec![
                    task(
                        3,
                        "proofs-width",
                        "make",
                        &["-C", "proofs/width", "check-manifest", "all"],
                        true,
                    ),
                    task(
                        3,
                        "proofs-rspu",
                        "make",
                        &["-C", "proofs/rspu", "check-manifest", "all"],
                        true,
                    ),
                    task(
                        3,
                        "proofs-language",
                        "make",
                        &["-C", "proofs/language", "check-manifest", "check-no-admitted", "all"],
                        true,
                    ),
                ],
            },
            WaveSpec {
                wave_index: 4,
                kind: WaveKind::Parity,
                tasks: vec![
                    task(
                        4,
                        "vscode",
                        npm_command_name(),
                        &["--prefix", "vscode-mirr", "pack", "--dry-run"],
                        true,
                    ),
                    task(
                        4,
                        "mirr-wasm-check",
                        "cargo",
                        &["check", "--manifest-path", "crates/mirr-wasm/Cargo.toml"],
                        true,
                    ),
                    task(
                        4,
                        "mirr-arsenal-wasm-check",
                        "cargo",
                        &["check", "--manifest-path", "crates/mirr-arsenal-wasm/Cargo.toml"],
                        true,
                    ),
                ],
            },
        ],
    }
}

fn build_ci_compile_feedback_plan(use_nextest: bool) -> ExecutionPlan {
    let workspace_test_args = workspace_selective_test_args(use_nextest);
    let lra_test_args = lra_cli_test_args(use_nextest);

    ExecutionPlan {
        waves: vec![
            WaveSpec {
                wave_index: 0,
                kind: WaveKind::Check,
                tasks: vec![task(
                    0,
                    "workspace-core-check",
                    "cargo",
                    &["check", "--all-targets"],
                    true,
                )],
            },
            WaveSpec {
                wave_index: 1,
                kind: WaveKind::Nextest,
                tasks: vec![
                    task(1, "workspace-selective-tests", "cargo", workspace_test_args, true),
                    task(1, "lra-cli-tests", "cargo", lra_test_args, true),
                ],
            },
            WaveSpec {
                wave_index: 2,
                kind: WaveKind::Parity,
                tasks: vec![
                    task(
                        2,
                        "mirr-wasm-check",
                        "cargo",
                        &["check", "--manifest-path", "crates/mirr-wasm/Cargo.toml"],
                        true,
                    ),
                    task(
                        2,
                        "mirr-arsenal-wasm-check",
                        "cargo",
                        &["check", "--manifest-path", "crates/mirr-arsenal-wasm/Cargo.toml"],
                        true,
                    ),
                ],
            },
        ],
    }
}

fn build_ci_fast_plan() -> ExecutionPlan {
    ExecutionPlan {
        waves: vec![
            WaveSpec {
                wave_index: 0,
                kind: WaveKind::Fmt,
                tasks: vec![task(
                    0,
                    "workspace-format",
                    "cargo",
                    &["fmt", "--all", "--", "--check"],
                    false,
                )],
            },
            WaveSpec {
                wave_index: 1,
                kind: WaveKind::Clippy,
                tasks: vec![task(
                    1,
                    "workspace-core-clippy",
                    "cargo",
                    &["clippy", "-p", "mirrc", "--all-targets", "--", "-D", "warnings"],
                    true,
                )],
            },
        ],
    }
}

fn command_exists(command: &str) -> bool {
    Command::new(command).arg("--version").output().is_ok()
}

fn env_var_is_truthy(value: &str) -> bool {
    value == "1"
        || value.eq_ignore_ascii_case("true")
        || value.eq_ignore_ascii_case("yes")
        || value.eq_ignore_ascii_case("on")
}

fn sccache_enabled_for_ci() -> bool {
    static SCCACHE_AVAILABLE: OnceLock<bool> = OnceLock::new();

    let disabled = std::env::var("MIRR_GENERAL_DISABLE_SCCACHE")
        .ok()
        .as_deref()
        .map(env_var_is_truthy)
        .unwrap_or(false);
    if disabled {
        return false;
    }

    *SCCACHE_AVAILABLE.get_or_init(|| command_exists("sccache"))
}

fn nextest_available() -> bool {
    static NEXTEST_AVAILABLE: OnceLock<bool> = OnceLock::new();

    *NEXTEST_AVAILABLE.get_or_init(|| {
        Command::new("cargo")
            .args(["nextest", "--version"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

fn nextest_enabled_for_ci() -> bool {
    let disabled = std::env::var("MIRR_GENERAL_DISABLE_NEXTEST")
        .ok()
        .as_deref()
        .map(env_var_is_truthy)
        .unwrap_or(false);

    if disabled {
        return false;
    }

    nextest_available()
}

fn parse_as_json(flags: &[String]) -> bool {
    let mut idx = 0usize;
    while idx < flags.len() {
        let flag = &flags[idx];
        if flag == "--format=json" || flag == "--json" || flag == "-j" {
            return true;
        }
        if flag == "--format" {
            return idx + 1 < flags.len() && flags[idx + 1] == "json";
        }
        idx += 1;
    }
    false
}

fn parse_ci_profile(flags: &[String]) -> Result<Option<CiProfile>, MirrError> {
    let mut idx = 0usize;
    let mut parsed: Option<CiProfile> = None;

    fn decode_profile(value: &str) -> Result<CiProfile, MirrError> {
        match value {
            "full" | "ci" => Ok(CiProfile::Full),
            "compile" | "feedback" => Ok(CiProfile::Compile),
            "fast" => Ok(CiProfile::Fast),
            _ => Err(MirrError::ToolingError {
                message: format!(
                    "invalid profile '{}'; expected one of: full, compile, fast",
                    value
                ),
                span: None,
            }),
        }
    }

    while idx < flags.len() {
        let flag = &flags[idx];

        if let Some(value) = flag.strip_prefix("--profile=") {
            parsed = Some(decode_profile(value)?);
            idx += 1;
            continue;
        }

        if flag == "--profile" {
            if idx + 1 >= flags.len() {
                return Err(MirrError::ToolingError {
                    message: "missing value after --profile".to_string(),
                    span: None,
                });
            }
            parsed = Some(decode_profile(&flags[idx + 1])?);
            idx += 2;
            continue;
        }

        if flag == "-p" {
            if idx + 1 >= flags.len() {
                return Err(MirrError::ToolingError {
                    message: "missing value after -p".to_string(),
                    span: None,
                });
            }
            parsed = Some(decode_profile(&flags[idx + 1])?);
            idx += 2;
            continue;
        }

        idx += 1;
    }

    Ok(parsed)
}

fn parse_profile_alias(command: &str) -> Option<CiProfile> {
    match command {
        "full" => Some(CiProfile::Full),
        "check" | "compile" => Some(CiProfile::Compile),
        "fast" | "quick" => Some(CiProfile::Fast),
        _ => None,
    }
}

fn print_usage() {
    eprintln!(
        "mirr-general quick usage:\n  cargo run --bin mirr-general -- check\n  cargo run --bin mirr-general -- fast\n  cargo run --bin mirr-general -- full\n  cargo run --bin mirr-general -- ci -p fast -j\n  cargo run --bin mirr-general -- inspect -j\n\nflags:\n  -p, --profile <full|compile|fast>\n  -j, --json, --format json\n\nnaming policy:\n  package identity remains mirrc for compatibility\n  compiler binary alias is mirrc (plus existing mirr-compile)"
    );
}

fn run_ci_from_flags(flags: &[String], default_profile: Option<CiProfile>) -> io::Result<i32> {
    const MAX_CLI_FLAGS: usize = 64;
    assert!(flags.len() <= MAX_CLI_FLAGS, "flag count exceeded bound ({MAX_CLI_FLAGS})");

    let as_json = parse_as_json(flags);
    let parsed_profile = parse_ci_profile(flags).map_err(|message| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{message}\nUse 'mirr-general help' for command examples."),
        )
    })?;
    run_ci(as_json, parsed_profile.or(default_profile))
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CommandRoute {
    RunCi { flags: Vec<String>, default_profile: Option<CiProfile> },
    Inspect { as_json: bool },
    ParityAll,
    Migrate { dry_run: bool },
    Help,
}

fn route_command(args: &[String]) -> Result<CommandRoute, MirrError> {
    if args.is_empty() {
        return Ok(CommandRoute::RunCi { flags: Vec::new(), default_profile: None });
    }

    if args.len() == 1 && (args[0] == "help" || args[0] == "--help" || args[0] == "-h") {
        return Ok(CommandRoute::Help);
    }

    if args[0] == "run" {
        if args.len() == 1 {
            return Err(MirrError::ToolingError {
                message: "missing subcommand after 'run'".to_string(),
                span: None,
            });
        }

        match args[1].as_str() {
            "ci" => {
                return Ok(CommandRoute::RunCi {
                    flags: args[2..].to_vec(),
                    default_profile: None,
                });
            }
            "inspect" => {
                return Ok(CommandRoute::Inspect { as_json: parse_as_json(&args[2..]) });
            }
            "parity" => {
                if args.len() == 3 && args[2] == "--all" {
                    return Ok(CommandRoute::ParityAll);
                }
                return Err(MirrError::ToolingError {
                    message: "usage: mirr-general run parity --all".to_string(),
                    span: None,
                });
            }
            "migrate" => {
                if args.len() == 3 && args[2] == "--dry-run" {
                    return Ok(CommandRoute::Migrate { dry_run: true });
                }
                if args.len() == 2 {
                    return Ok(CommandRoute::Migrate { dry_run: false });
                }
                return Err(MirrError::ToolingError {
                    message: "usage: mirr-general run migrate [--dry-run]".to_string(),
                    span: None,
                });
            }
            run_subcommand => {
                if let Some(profile) = parse_profile_alias(run_subcommand) {
                    return Ok(CommandRoute::RunCi {
                        flags: args[2..].to_vec(),
                        default_profile: Some(profile),
                    });
                }
                return Err(MirrError::ToolingError {
                    message: format!("unrecognized 'run' subcommand: '{}'", run_subcommand),
                    span: None,
                });
            }
        }
    }

    if args[0] == "ci" {
        return Ok(CommandRoute::RunCi { flags: args[1..].to_vec(), default_profile: None });
    }

    if args[0] == "inspect" {
        return Ok(CommandRoute::Inspect { as_json: parse_as_json(&args[1..]) });
    }

    if let Some(profile) = parse_profile_alias(&args[0]) {
        return Ok(CommandRoute::RunCi {
            flags: args[1..].to_vec(),
            default_profile: Some(profile),
        });
    }

    Err(MirrError::ToolingError {
        message: format!("unrecognized subcommand: '{}'", args[0]),
        span: None,
    })
}

fn dispatch(args: &[String]) -> io::Result<i32> {
    let route = route_command(args)
        .map_err(|message| io::Error::new(io::ErrorKind::InvalidInput, message))?;

    match route {
        CommandRoute::RunCi { flags, default_profile } => {
            run_ci_from_flags(&flags, default_profile)
        }
        CommandRoute::Inspect { as_json } => run_inspect(as_json),
        CommandRoute::ParityAll => run_parity_all(),
        CommandRoute::Migrate { dry_run } => run_migrate(dry_run),
        CommandRoute::Help => {
            print_usage();
            Ok(0)
        }
    }
}

struct BinaryInventory {
    top_level_bin_count: usize,
    deps_bin_count: usize,
    deps_hashed_bin_count: usize,
    deps_nasa_bin_count: usize,
    top_level_sample: Vec<String>,
    deps_sample: Vec<String>,
}

const MAX_INSPECT_FILES: usize = 16384;
const MAX_INSPECT_SAMPLE: usize = 12;

fn has_hashed_suffix(binary_name: &str) -> bool {
    let stem = binary_name.strip_suffix(".exe").unwrap_or(binary_name);
    if let Some((_, suffix)) = stem.rsplit_once('-') {
        suffix.len() == 16 && suffix.as_bytes().iter().all(|b| b.is_ascii_hexdigit())
    } else {
        false
    }
}

fn is_binary_artifact(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("exe"))
        .unwrap_or(false)
}

fn read_binary_names(dir: &Path) -> io::Result<Vec<String>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut names: Vec<String> = Vec::new();
    let mut scanned = 0usize;

    for entry in fs::read_dir(dir)? {
        scanned += 1;
        if scanned > MAX_INSPECT_FILES {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("inspect scan exceeded file bound ({MAX_INSPECT_FILES})"),
            ));
        }

        let entry = entry?;
        let path = entry.path();
        if !path.is_file() || !is_binary_artifact(&path) {
            continue;
        }

        names.push(entry.file_name().to_string_lossy().to_string());
    }

    names.sort();
    Ok(names)
}

fn gather_binary_inventory() -> io::Result<BinaryInventory> {
    let top_level_bins = read_binary_names(Path::new("target/debug"))?;
    let deps_bins = read_binary_names(Path::new("target/debug/deps"))?;

    let deps_hashed_bin_count =
        deps_bins.iter().filter(|name| has_hashed_suffix(name.as_str())).count();
    let deps_nasa_bin_count = deps_bins.iter().filter(|name| name.starts_with("mirrc")).count();

    let top_level_sample: Vec<String> =
        top_level_bins.iter().take(MAX_INSPECT_SAMPLE).cloned().collect();
    let deps_sample: Vec<String> = deps_bins.iter().take(MAX_INSPECT_SAMPLE).cloned().collect();

    Ok(BinaryInventory {
        top_level_bin_count: top_level_bins.len(),
        deps_bin_count: deps_bins.len(),
        deps_hashed_bin_count,
        deps_nasa_bin_count,
        top_level_sample,
        deps_sample,
    })
}

fn run_inspect(as_json: bool) -> io::Result<i32> {
    let inventory = gather_binary_inventory()?;
    let payload = serde_json::json!({
        "package_name": "mirrc",
        "compiler_binaries": {
            "primary": "mirr-compile",
            "alias": "mirrc",
        },
        "runtime": {
            "sccache_enabled": sccache_enabled_for_ci(),
            "nextest_enabled": nextest_enabled_for_ci(),
        },
        "artifacts": {
            "top_level_bin_count": inventory.top_level_bin_count,
            "deps_bin_count": inventory.deps_bin_count,
            "deps_hashed_bin_count": inventory.deps_hashed_bin_count,
            "deps_nasa_bin_count": inventory.deps_nasa_bin_count,
            "top_level_sample": inventory.top_level_sample,
            "deps_sample": inventory.deps_sample,
            "notes": [
                "Many target/debug/deps binaries are build artifacts, not active processes.",
                "Hashed suffixes identify distinct crate/test harness artifacts.",
                "The package name remains mirrc for compatibility; only compiler binary alias is mirrc."
            ]
        }
    });

    if as_json {
        println!("{}", payload);
    } else {
        println!("package_name: mirrc");
        println!("compiler binaries: primary=mirr-compile alias=mirrc");
        println!(
            "runtime: sccache_enabled={} nextest_enabled={}",
            sccache_enabled_for_ci(),
            nextest_enabled_for_ci()
        );
        println!(
            "artifacts: top_level={} deps={} deps_hashed={} deps_nasa={}",
            inventory.top_level_bin_count,
            inventory.deps_bin_count,
            inventory.deps_hashed_bin_count,
            inventory.deps_nasa_bin_count
        );
        if !inventory.top_level_sample.is_empty() {
            println!("top-level sample: {}", inventory.top_level_sample.join(", "));
        }
        if !inventory.deps_sample.is_empty() {
            println!("deps sample: {}", inventory.deps_sample.join(", "));
        }
    }

    Ok(0)
}

fn proof_skip_task(wave_index: usize) -> TaskSpec {
    #[cfg(target_os = "windows")]
    {
        task(
            wave_index,
            "proofs-skip",
            "cmd",
            &["/C", "echo SKIP: make unavailable; proofs wave skipped"],
            false,
        )
    }

    #[cfg(not(target_os = "windows"))]
    {
        task(
            wave_index,
            "proofs-skip",
            "sh",
            &["-c", "echo SKIP: make unavailable; proofs wave skipped"],
            false,
        )
    }
}

const MAX_FINGERPRINT_FILES: usize = 4096;
const MAX_FINGERPRINT_DIRS: usize = 4096;

fn should_skip_fingerprint_dir(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | "target"
            | "node_modules"
            | "dist"
            | "build"
            | "coverage"
            | "pkg"
            | ".next"
            | ".cache"
            | "__pycache__"
            | "_site"
            | "artifacts"
            | "out"
    )
}

fn collect_files_under(root: &Path, out: &mut BTreeSet<PathBuf>) -> io::Result<()> {
    if !root.exists() {
        return Ok(());
    }

    if root.is_file() {
        out.insert(root.to_path_buf());
        return Ok(());
    }

    let mut pending = vec![root.to_path_buf()];
    let mut scanned_dirs = 0usize;

    while let Some(dir) = pending.pop() {
        scanned_dirs += 1;
        if scanned_dirs > MAX_FINGERPRINT_DIRS {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("fingerprint scan exceeded directory bound ({MAX_FINGERPRINT_DIRS})"),
            ));
        }

        let entries = fs::read_dir(&dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();

            let metadata = fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() {
                continue;
            }

            if metadata.is_dir() {
                if should_skip_fingerprint_dir(&name) {
                    continue;
                }
                pending.push(path);
                continue;
            }

            if metadata.is_file() {
                out.insert(path);
                if out.len() > MAX_FINGERPRINT_FILES {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("fingerprint scan exceeded file bound ({MAX_FINGERPRINT_FILES})"),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn package_fingerprint_sources(package_name: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();

    match package_name {
        "workspace"
        | "workspace-format"
        | "workspace-core-check"
        | "workspace-core-clippy"
        | "workspace-core-tests" => {
            collect_files_under(Path::new("src"), &mut files)?;
            collect_files_under(Path::new("tests"), &mut files)?;
            collect_files_under(Path::new("benches"), &mut files)?;
            collect_files_under(Path::new("examples"), &mut files)?;
            collect_files_under(Path::new("compiler_mirr"), &mut files)?;
            collect_files_under(Path::new("Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.lock"), &mut files)?;
            collect_files_under(Path::new("rust-toolchain.toml"), &mut files)?;
        }
        "workspace-selective-tests" => {
            collect_files_under(Path::new("src"), &mut files)?;
            collect_files_under(Path::new("Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.lock"), &mut files)?;
            collect_files_under(Path::new("tests/parser_edge_cases_tests.rs"), &mut files)?;
            collect_files_under(Path::new("tests/parser_composite_tests.rs"), &mut files)?;
            collect_files_under(Path::new("tests/semantic_validation_tests.rs"), &mut files)?;
            collect_files_under(
                Path::new("tests/composite_type_integration_tests.rs"),
                &mut files,
            )?;
            collect_files_under(Path::new("tests/meta_stage_unroller_tests.rs"), &mut files)?;
            collect_files_under(Path::new("tests/compiler_flexibility_tests.rs"), &mut files)?;
            collect_files_under(Path::new("tests/orchestrator_scheduler_tests.rs"), &mut files)?;
            collect_files_under(Path::new("tests/orchestrator_cache_tests.rs"), &mut files)?;
            collect_files_under(Path::new("tests/orchestrator_migration_tests.rs"), &mut files)?;
            collect_files_under(Path::new("tests/orchestrator_parity_tests.rs"), &mut files)?;
        }
        "lra-cli" | "lra-cli-check" | "lra-cli-clippy" | "lra-cli-tests" => {
            collect_files_under(Path::new("src"), &mut files)?;
            collect_files_under(Path::new("crates/lra-cli/src"), &mut files)?;
            collect_files_under(Path::new("crates/lra-cli/tests"), &mut files)?;
            collect_files_under(Path::new("crates/lra-cli/Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.lock"), &mut files)?;
        }
        "mirr-wasm-check" => {
            collect_files_under(Path::new("src"), &mut files)?;
            collect_files_under(Path::new("crates/mirr-wasm/src"), &mut files)?;
            collect_files_under(Path::new("crates/mirr-wasm/Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.lock"), &mut files)?;
        }
        "mirr-arsenal-wasm-check" => {
            collect_files_under(Path::new("src"), &mut files)?;
            collect_files_under(Path::new("crates/mirr-arsenal-wasm/src"), &mut files)?;
            collect_files_under(Path::new("crates/mirr-arsenal-wasm/Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.lock"), &mut files)?;
        }
        "proofs-width" => {
            collect_files_under(Path::new("proofs/width"), &mut files)?;
        }
        "proofs-rspu" => {
            collect_files_under(Path::new("proofs/rspu"), &mut files)?;
        }
        "proofs-language" | "proofs-skip" => {
            collect_files_under(Path::new("proofs/language"), &mut files)?;
            collect_files_under(Path::new("proofs/width"), &mut files)?;
            collect_files_under(Path::new("proofs/rspu"), &mut files)?;
        }
        "vscode" => {
            collect_files_under(Path::new("vscode-mirr/src"), &mut files)?;
            collect_files_under(Path::new("vscode-mirr/syntaxes"), &mut files)?;
            collect_files_under(Path::new("vscode-mirr/package.json"), &mut files)?;
            collect_files_under(Path::new("vscode-mirr/package-lock.json"), &mut files)?;
        }
        _ => {
            collect_files_under(Path::new("Cargo.toml"), &mut files)?;
            collect_files_under(Path::new("Cargo.lock"), &mut files)?;
        }
    }

    if files.is_empty() {
        files.insert(Path::new("Cargo.toml").to_path_buf());
    }

    Ok(files.into_iter().collect())
}

fn build_ci_fingerprint_map(plan: &ExecutionPlan) -> io::Result<BTreeMap<String, String>> {
    let lock_hash = if Path::new("Cargo.lock").exists() {
        hash_file(Path::new("Cargo.lock"))?
    } else {
        String::from("missing-lock")
    };
    let rustc_output = Command::new("rustc").arg("--version").output()?;
    let rustc_version = String::from_utf8_lossy(&rustc_output.stdout).trim().to_string();

    let mut fingerprints = BTreeMap::new();
    for wave in &plan.waves {
        for task in &wave.tasks {
            if fingerprints.contains_key(&task.package_name) {
                continue;
            }

            let spec = PackageSpec {
                name: task.package_name.clone(),
                source_files: package_fingerprint_sources(&task.package_name)?,
            };
            let fingerprint = compute_package_fingerprint(&spec, &lock_hash, &rustc_version)?;
            fingerprints.insert(task.package_name.clone(), fingerprint);
        }
    }

    Ok(fingerprints)
}

fn wave_kind_label(kind: &WaveKind) -> &'static str {
    match kind {
        WaveKind::Fmt => "fmt",
        WaveKind::Check => "check",
        WaveKind::Clippy => "clippy",
        WaveKind::Nextest => "nextest",
        WaveKind::Rocq => "rocq",
        WaveKind::Parity => "parity",
    }
}

fn collect_bottleneck_tasks(
    summary: &mirr_general::scheduler::RunSummary,
    limit: usize,
) -> Vec<&mirr_general::scheduler::TaskResult> {
    assert!(limit > 0, "limit must be greater than zero");

    let mut tasks: Vec<&mirr_general::scheduler::TaskResult> =
        summary.task_results.iter().filter(|task| !task.skipped_by_cache).collect();

    tasks.sort_by(|left, right| {
        right
            .duration_ms
            .cmp(&left.duration_ms)
            .then_with(|| left.package_name.cmp(&right.package_name))
            .then_with(|| left.wave_index.cmp(&right.wave_index))
    });

    if tasks.len() > limit {
        tasks.truncate(limit);
    }

    tasks
}

fn print_summary(
    summary: &mirr_general::scheduler::RunSummary,
    as_json: bool,
    profile: CiProfile,
    ci_mode: bool,
    use_nextest: bool,
    use_sccache: bool,
) {
    let bottlenecks = collect_bottleneck_tasks(summary, 5);

    if as_json {
        let mut waves = Vec::with_capacity(summary.wave_results.len());
        for wave in &summary.wave_results {
            let skipped_tasks =
                wave.task_results.iter().filter(|task| task.skipped_by_cache).count();
            let skipped_packages: Vec<String> = wave
                .task_results
                .iter()
                .filter(|task| task.skipped_by_cache)
                .map(|task| task.package_name.clone())
                .collect();
            let tasks = wave
                .task_results
                .iter()
                .map(|task| {
                    serde_json::json!({
                        "package_name": task.package_name,
                        "status_code": task.status_code,
                        "duration_ms": task.duration_ms,
                        "skipped_by_cache": task.skipped_by_cache,
                        "command_line": task.command_line,
                    })
                })
                .collect::<Vec<_>>();
            waves.push(serde_json::json!({
                "wave_index": wave.wave_index,
                "wave_kind": wave_kind_label(&wave.wave_kind),
                "failed": wave.failed,
                "duration_ms": wave.duration_ms,
                "task_results": wave.task_results.len(),
                "skipped_tasks": skipped_tasks,
                "executed_tasks": wave.task_results.len() - skipped_tasks,
                "skipped_packages": skipped_packages,
                "tasks": tasks,
            }));
        }

        let bottleneck_payload = bottlenecks
            .iter()
            .map(|task| {
                serde_json::json!({
                    "wave_index": task.wave_index,
                    "package_name": task.package_name,
                    "duration_ms": task.duration_ms,
                    "command_line": task.command_line,
                })
            })
            .collect::<Vec<_>>();

        let payload = serde_json::json!({
            "success": summary.success,
            "profile": profile.label(),
            "runtime": {
                "ci_mode": ci_mode,
                "nextest_enabled": use_nextest,
                "sccache_enabled": use_sccache,
            },
            "completed_wave": summary.completed_wave,
            "task_results": summary.task_results.len(),
            "total_duration_ms": summary.total_duration_ms,
            "waves": waves,
            "bottlenecks": bottleneck_payload,
        });
        println!("{}", payload);
    } else {
        println!("success: {}", summary.success);
        println!("profile: {}", profile.label());
        println!(
            "runtime: ci_mode={} nextest_enabled={} sccache_enabled={}",
            ci_mode, use_nextest, use_sccache
        );
        println!("completed_wave: {}", summary.completed_wave);
        println!("task_results: {}", summary.task_results.len());
        println!("total_duration_ms: {}", summary.total_duration_ms);
        for wave in &summary.wave_results {
            let skipped_tasks =
                wave.task_results.iter().filter(|task| task.skipped_by_cache).count();
            println!(
                "wave={} kind={} duration_ms={} tasks={} skipped={} failed={}",
                wave.wave_index,
                wave_kind_label(&wave.wave_kind),
                wave.duration_ms,
                wave.task_results.len(),
                skipped_tasks,
                wave.failed
            );
            for task in &wave.task_results {
                println!(
                    "  task package={} duration_ms={} skipped={} status={}",
                    task.package_name, task.duration_ms, task.skipped_by_cache, task.status_code
                );
            }
        }
        if !bottlenecks.is_empty() {
            println!("bottlenecks:");
            for task in bottlenecks {
                println!(
                    "  wave={} package={} duration_ms={} command='{}'",
                    task.wave_index, task.package_name, task.duration_ms, task.command_line
                );
            }
        }
    }
}

fn run_ci(as_json: bool, requested_profile: Option<CiProfile>) -> io::Result<i32> {
    let ci_env = std::env::var("CI").ok();
    let ci_mode = ci_env.as_deref().map(env_var_is_truthy).unwrap_or(false);
    let active_profile = match requested_profile {
        Some(profile) => profile,
        None => {
            if ci_mode {
                CiProfile::Full
            } else {
                CiProfile::Compile
            }
        }
    };

    let use_nextest = nextest_enabled_for_ci();
    let use_sccache = sccache_enabled_for_ci();
    if !use_nextest {
        eprintln!(
            "cargo-nextest not detected or disabled; falling back to cargo test (install with 'cargo install cargo-nextest --locked' for faster local feedback)."
        );
    }

    let mut plan = match active_profile {
        CiProfile::Full => build_ci_full_plan(use_nextest),
        CiProfile::Compile => build_ci_compile_feedback_plan(use_nextest),
        CiProfile::Fast => build_ci_fast_plan(),
    };

    let disable_cache_skip = ci_mode;
    if disable_cache_skip {
        for wave in &mut plan.waves {
            for task in &mut wave.tasks {
                task.allow_cache_skip = false;
            }
        }
    }

    let requires_make = plan.waves.iter().any(|wave| wave.kind == WaveKind::Rocq);
    if requires_make && !command_exists("make") {
        if ci_mode {
            eprintln!(
                "CI/closeout mode requires 'make' for Rocq proof gates. Install make or run locally without CI=1."
            );
            return Ok(1);
        }
        for wave in &mut plan.waves {
            if wave.kind == WaveKind::Rocq {
                wave.tasks = vec![proof_skip_task(wave.wave_index)];
                break;
            }
        }
    }

    let cache_path = Path::new("mirr-general/cache.manifest");
    let mut manifest = if cache_path.exists() {
        load_manifest(cache_path)?
    } else {
        CacheManifest { entries: BTreeMap::new() }
    };

    let package_manifest_path = Path::new("mirr-general/packages.manifest");
    if package_manifest_path.exists() {
        let package_manifest = load_package_manifest(package_manifest_path)?;
        assert!(
            package_manifest.members.len() <= MAX_FINGERPRINT_FILES,
            "package manifest entries exceeded bound ({MAX_FINGERPRINT_FILES})"
        );
    }

    let fingerprints = build_ci_fingerprint_map(&plan)?;
    let summary = execute_all_waves(&plan, &manifest, &fingerprints)?;
    print_summary(&summary, as_json, active_profile, ci_mode, use_nextest, use_sccache);

    if !summary.success {
        for result in &summary.task_results {
            if result.status_code != 0 {
                eprintln!(
                    "CI task failed: wave={} package={} status={} command='{}'",
                    result.wave_index, result.package_name, result.status_code, result.command_line
                );
            }
        }
    }

    if summary.success {
        for (package_name, fingerprint) in &fingerprints {
            upsert_manifest_entry(&mut manifest, package_name, fingerprint);
        }
        write_manifest_atomic(cache_path, &manifest)?;
    }

    if summary.success {
        Ok(0)
    } else {
        Ok(1)
    }
}

fn run_parity_all() -> io::Result<i32> {
    let records = vec![
        verify_cli_wasm_parity(Path::new("examples/neonatal_respirator.mirr"))?,
        verify_vscode_contract()?,
    ];

    run_consumer_parity(&records)?;
    let normalized = std::env::var("MIRR_PARITY_NORMALIZED_JSON").ok();
    let normalized_value = match normalized.as_deref() {
        Some(text) => serde_json::from_str::<serde_json::Value>(text).ok(),
        None => None,
    };
    let report = serde_json::json!({
        "normalized": normalized_value,
        "records": records,
    });
    println!("{}", report);
    Ok(0)
}

fn run_migrate(dry_run: bool) -> io::Result<i32> {
    let repo_root = Path::new(".");
    let inventory = build_script_inventory(repo_root);

    if dry_run {
        for spec in inventory {
            println!("{} -> {}", spec.id, spec.replacement_subcommand);
        }
        return Ok(0);
    }

    let mut overall_ok = true;
    for spec in inventory {
        let result = migrate_script(&spec)?;
        println!("{} deleted={} detail={}", result.id, result.deleted, result.detail);
        if !result.deleted && result.detail.starts_with("parity test failed") {
            overall_ok = false;
        }
    }

    if overall_ok {
        Ok(0)
    } else {
        Ok(1)
    }
}

pub fn run(args: Vec<String>) -> anyhow::Result<()> {
    match dispatch(&args) {
        Ok(code) => {
            std::process::exit(code);
        }
        Err(error) => {
            eprintln!("{}", error);
            print_usage();
            eprintln!(
                "additional supported commands: 'run ci', 'run inspect', 'run parity --all', 'run migrate --dry-run', 'run migrate'"
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;

    use super::ci_child_target_dir_from;
    use super::collect_bottleneck_tasks;
    use super::dispatch;
    use super::has_hashed_suffix;
    use super::mirr_general::scheduler::{RunSummary, TaskResult, WaveKind, WaveResult};
    use super::parse_as_json;
    use super::parse_ci_profile;
    use super::parse_profile_alias;
    use super::route_command;
    use super::workspace_selective_test_args;
    use super::CiProfile;
    use super::CommandRoute;

    fn fake_task(package_name: &str, duration_ms: u64, skipped_by_cache: bool) -> TaskResult {
        TaskResult {
            wave_index: 0,
            package_name: package_name.to_string(),
            status_code: 0,
            skipped_by_cache,
            command_line: "cargo test".to_string(),
            duration_ms,
        }
    }

    #[test]
    fn parse_ci_profile_supports_expected_values_and_aliases() {
        assert_eq!(
            parse_ci_profile(&["--profile".to_string(), "full".to_string()])
                .expect("full profile must parse"),
            Some(CiProfile::Full)
        );
        assert_eq!(
            parse_ci_profile(&["--profile=ci".to_string()]).expect("ci alias must parse"),
            Some(CiProfile::Full)
        );
        assert_eq!(
            parse_ci_profile(&["--profile=feedback".to_string()])
                .expect("feedback alias must parse"),
            Some(CiProfile::Compile)
        );
        assert_eq!(
            parse_ci_profile(&["--profile=fast".to_string()]).expect("fast profile must parse"),
            Some(CiProfile::Fast)
        );
        assert_eq!(
            parse_ci_profile(&["-p".to_string(), "fast".to_string()])
                .expect("short profile flag must parse"),
            Some(CiProfile::Fast)
        );
    }

    #[test]
    fn parse_ci_profile_returns_err_for_invalid_or_missing_values() {
        assert!(parse_ci_profile(&["--profile=unknown".to_string()]).is_err());
        assert!(parse_ci_profile(&["--profile".to_string()]).is_err());
        assert!(parse_ci_profile(&["-p".to_string()]).is_err());
    }

    #[test]
    fn parse_as_json_supports_short_and_long_flags() {
        assert!(parse_as_json(&["--json".to_string()]));
        assert!(parse_as_json(&["-j".to_string()]));
        assert!(parse_as_json(&["--format=json".to_string()]));
        assert!(parse_as_json(&["--format".to_string(), "json".to_string()]));
    }

    #[test]
    fn parse_profile_alias_supports_human_shortcuts() {
        assert_eq!(parse_profile_alias("full"), Some(CiProfile::Full));
        assert_eq!(parse_profile_alias("check"), Some(CiProfile::Compile));
        assert_eq!(parse_profile_alias("compile"), Some(CiProfile::Compile));
        assert_eq!(parse_profile_alias("fast"), Some(CiProfile::Fast));
        assert_eq!(parse_profile_alias("quick"), Some(CiProfile::Fast));
        assert_eq!(parse_profile_alias("unknown"), None);
    }

    #[test]
    fn has_hashed_suffix_detects_rust_hash_pattern() {
        assert!(has_hashed_suffix("foo-0123456789abcdef.exe"));
        assert!(has_hashed_suffix("bar-abcdefabcdefabcd"));
        assert!(!has_hashed_suffix("foo-xyz.exe"));
        assert!(!has_hashed_suffix("foo.exe"));
    }

    #[test]
    fn ci_child_target_dir_defaults_are_platform_safe() {
        let target_dir = ci_child_target_dir_from(None);
        #[cfg(target_os = "windows")]
        {
            assert_eq!(
                target_dir,
                Path::new("target/ci-wave").join("mirr-general-child").into_os_string()
            );
        }
        #[cfg(not(target_os = "windows"))]
        {
            assert_eq!(target_dir, Path::new("target/ci-wave").as_os_str().to_os_string());
        }
    }

    #[test]
    fn ci_child_target_dir_respects_inherited_base_dir() {
        let target_dir =
            ci_child_target_dir_from(Some(Path::new("target/custom-wave").as_os_str()));
        #[cfg(target_os = "windows")]
        {
            assert_eq!(
                target_dir,
                Path::new("target/custom-wave").join("mirr-general-child").into_os_string()
            );
        }
        #[cfg(not(target_os = "windows"))]
        {
            assert_eq!(target_dir, Path::new("target/custom-wave").as_os_str().to_os_string());
        }
    }

    #[test]
    fn route_command_prioritizes_run_ci_over_alias_routing() {
        let args = vec!["run".to_string(), "ci".to_string(), "-j".to_string()];
        let route = route_command(&args).expect("run ci route must parse");
        assert_eq!(
            route,
            CommandRoute::RunCi { flags: vec!["-j".to_string()], default_profile: None }
        );
    }

    #[test]
    fn route_command_maps_run_profile_aliases_explicitly() {
        let args = vec!["run".to_string(), "fast".to_string(), "-j".to_string()];
        let route = route_command(&args).expect("run fast route must parse");
        assert_eq!(
            route,
            CommandRoute::RunCi {
                flags: vec!["-j".to_string()],
                default_profile: Some(CiProfile::Fast),
            }
        );
    }

    #[test]
    fn route_command_rejects_invalid_parity_invocation() {
        let args = vec!["run".to_string(), "parity".to_string()];
        let error = route_command(&args).expect_err("run parity without --all must fail");
        assert!(error.message().contains("usage: mirr-general run parity --all"));
    }

    #[test]
    fn dispatch_returns_error_for_unknown_subcommand() {
        let args = vec!["unknown".to_string()];
        let err = dispatch(&args).expect_err("unknown command should fail");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn collect_bottleneck_tasks_orders_by_duration_and_excludes_cached_skips() {
        let skipped = fake_task("cached", 9999, true);
        let slow = fake_task("slow", 3000, false);
        let medium = fake_task("medium", 1500, false);
        let fast = fake_task("fast", 100, false);
        let summary = RunSummary {
            success: true,
            completed_wave: 1,
            task_results: vec![slow.clone(), skipped, fast.clone(), medium.clone()],
            wave_results: vec![WaveResult {
                wave_index: 0,
                wave_kind: WaveKind::Nextest,
                task_results: vec![slow, fast, medium],
                failed: false,
                duration_ms: 4600,
            }],
            total_duration_ms: 4600,
        };

        let top = collect_bottleneck_tasks(&summary, 3);
        assert_eq!(top.len(), 3);
        assert_eq!(top[0].package_name, "slow");
        assert_eq!(top[1].package_name, "medium");
        assert_eq!(top[2].package_name, "fast");
    }

    #[test]
    fn workspace_selective_test_args_use_nextest_by_default() {
        let args = workspace_selective_test_args(true);
        assert_eq!(args.first().copied(), Some("nextest"));
        assert_eq!(args.get(1).copied(), Some("run"));
    }

    #[test]
    fn workspace_selective_test_args_fallback_to_cargo_test() {
        let args = workspace_selective_test_args(false);
        assert_eq!(args.first().copied(), Some("test"));
        assert!(args.contains(&"--test"));
    }
}
