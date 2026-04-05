#![forbid(unsafe_code)]

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

const EXPECTED_BACKEND: &str = "kb-data";
const EXPECTED_RESULT_LIMIT: u64 = 16;
const EXPECTED_ENTRY_SIZE_LIMIT: u64 = 4096;
const EXPECTED_VALUE: &str = "hello";

static NEXT_TEMP_ID: AtomicUsize = AtomicUsize::new(0);

struct TempKbRoot {
    base_dir: PathBuf,
    kb_root: PathBuf,
}

impl TempKbRoot {
    fn new(test_name: &str) -> Self {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let base_dir = std::env::temp_dir().join(format!(
            "mirr_brain_kb_lite_{test_name}_{}_{}",
            std::process::id(),
            id
        ));

        if base_dir.exists() {
            fs::remove_dir_all(&base_dir).expect("remove stale temp kb root");
        }

        let kb_root = base_dir.join(".kb-data");
        let knowledge_lance = kb_root.join("knowledge.lance");
        fs::create_dir_all(&knowledge_lance).expect("create knowledge.lance directory");

        Self { base_dir, kb_root }
    }

    fn kb_root(&self) -> &Path {
        &self.kb_root
    }
}

impl Drop for TempKbRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base_dir);
    }
}

fn run_mirr_brain(args: &[&str]) -> Output {
    let binary = env!("CARGO_BIN_EXE_mirr-brain");
    Command::new(binary).args(args).output().expect("run mirr-brain")
}

fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context} failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn parse_json_stdout(output: &Output, context: &str) -> Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str::<Value>(&stdout)
        .unwrap_or_else(|err| panic!("{context} returned invalid JSON: {err}\nstdout:\n{stdout}"))
}

#[test]
fn mirr_brain_round_trips_through_temp_kb_root() {
    let temp_kb = TempKbRoot::new("round_trip");
    let kb_root = temp_kb.kb_root().to_string_lossy().into_owned();

    let store_output = run_mirr_brain(&[
        "--kb-root",
        &kb_root,
        "store",
        "--key",
        "greeting",
        "--value",
        EXPECTED_VALUE,
    ]);
    assert_success(&store_output, "store");

    let get_output =
        run_mirr_brain(&["--kb-root", &kb_root, "--format", "json", "get", "--key", "greeting"]);
    assert_success(&get_output, "get");

    let parsed = parse_json_stdout(&get_output, "get");
    assert_eq!(parsed["backend"], EXPECTED_BACKEND);
    assert_eq!(parsed["result_limit"].as_u64(), Some(EXPECTED_RESULT_LIMIT));
    assert_eq!(parsed["entry_size_limit"].as_u64(), Some(EXPECTED_ENTRY_SIZE_LIMIT));
    assert_eq!(parsed["value"], EXPECTED_VALUE);
}

#[test]
fn mirr_brain_laws_are_bounded() {
    let temp_kb = TempKbRoot::new("laws_bounded");
    let kb_root = temp_kb.kb_root().to_string_lossy().into_owned();

    let laws_output = run_mirr_brain(&["--kb-root", &kb_root, "--format", "json", "laws"]);
    assert_success(&laws_output, "laws");

    let parsed = parse_json_stdout(&laws_output, "laws");
    let laws = parsed["laws"].as_array().expect("laws must be an array");
    assert!(laws.len() <= EXPECTED_RESULT_LIMIT as usize);
}
