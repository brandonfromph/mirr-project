#![forbid(unsafe_code)]
#![allow(clippy::len_zero)]
//! Integration test suite for the hybrid Vector+BM25 RAG Knowledge Base (mirr-kb-native).
//! Contains exactly 50 distinct parameter-driven tests.

use mirr_kb_native::adapters::embedding::StubEmbeddingProvider;
use mirr_kb_native::chunking::{compute_hash, estimate_token_count, ChunkType, MirrChunk};
use mirr_kb_native::context::{validate_query_size, ContextBudget};
use mirr_kb_native::evaluation::{Difficulty, GoldenQAPair};
use mirr_kb_native::expansion::{expand_query_variants, ExpansionMode};
use mirr_kb_native::resilience::ResiliencePolicy;
use mirr_kb_native::retrieval::SearchResult;
use mirr_kb_native::validation::{sanitize_text, validate_results};
use std::process::Output;

// Macro to generate sanitization & validation API tests (001 - 020)
macro_rules! generate_api_tests {
    ($($idx:ident, $snippet:expr, $expected_snippet:expr, $score:expr, $expected_score:expr);* $(;)?) => {
        $(
            #[test]
            fn $idx() {
                let sanitized = sanitize_text($snippet);
                assert_eq!(sanitized, $expected_snippet);

                let item = SearchResult {
                    key: "test_key".to_string(),
                    title: "test_title".to_string(),
                    snippet: $snippet.to_string(),
                    score: $score,
                    source: "test_source".to_string(),
                };
                let (res, summary) = validate_results(vec![item]);
                if $snippet.trim().is_empty() {
                    assert_eq!(summary.dropped_empty, 1);
                    assert!(res.is_empty());
                } else {
                    assert_eq!(res.len(), 1);
                    assert_eq!(res[0].score, $expected_score);
                }
            }
        )*
    };
}

// --- 001 - 020: API, sanitization, and config validation tests ---
generate_api_tests! {
    test_kb_api_001, "simple text", "simple text", 0.8, 0.8;
    test_kb_api_002, "text with null \0 byte", "text with null  byte", 0.9, 0.9;
    test_kb_api_003, "text with control \u{0007} bell", "text with control  bell", 0.5, 0.5;
    test_kb_api_004, "text with newlines \n and tabs \t preserved", "text with newlines \n and tabs \t preserved", 0.6, 0.6;
    test_kb_api_005, "", "", 0.7, 0.7; // dropped empty
    test_kb_api_006, "   ", "   ", 0.7, 0.7; // dropped empty trim
    test_kb_api_007, "valid text", "valid text", 1.5, 1.0; // clamped max score
    test_kb_api_008, "valid text", "valid text", -0.5, 0.0; // clamped min score
    test_kb_api_009, "valid text", "valid text", 0.0, 0.0;
    test_kb_api_010, "valid text", "valid text", 1.0, 1.0;
    test_kb_api_011, "a\0\0\0\0b", "ab", 0.8, 0.8;
    test_kb_api_012, "\u{0001}\u{0002}\u{0003}clean", "clean", 0.5, 0.5;
    test_kb_api_013, "long snippet", "long snippet", 0.85, 0.85;
    test_kb_api_014, "null \0 mid \0 text", "null  mid  text", 0.4, 0.4;
    test_kb_api_015, "control \u{001b} esc", "control  esc", 0.3, 0.3;
    test_kb_api_016, "tab\tnewline\n", "tab\tnewline\n", 0.75, 0.75;
    test_kb_api_017, "score clamp high", "score clamp high", 999.0, 1.0;
    test_kb_api_018, "score clamp low", "score clamp low", -999.0, 0.0;
    test_kb_api_019, "normal query text", "normal query text", 0.123, 0.123;
    test_kb_api_020, "null \0 control \u{0008} backspace", "null  control  backspace", 0.99, 0.99;
}

// Additional config, resilience, query size & context budget tests
#[test]
fn test_kb_api_021() {
    let limit = validate_query_size("short query");
    assert!(limit.is_ok());
}

#[test]
fn test_kb_api_022() {
    let limit = validate_query_size("a".repeat(5000).as_str());
    assert!(limit.is_err());
}

#[test]
fn test_kb_api_023() {
    let mut budget = ContextBudget::new();
    assert!(budget.try_add_chunk("hello"));
}

#[test]
fn test_kb_api_024() {
    let budget = ContextBudget::new();
    assert!(!budget.is_exhausted());
}

#[test]
fn test_kb_api_025() {
    let policy = ResiliencePolicy { max_retries: 10, timeout_ms: 100, fallback_to_lexical: true };
    let norm = policy.normalized();
    assert_eq!(norm.max_retries, 5); // clamped to MAX_RETRIES (5)
    assert_eq!(norm.timeout_ms, 1000); // clamped to min timeout_ms (1000)
}

#[test]
fn test_kb_api_026() {
    let policy = ResiliencePolicy { max_retries: 2, timeout_ms: 5000, fallback_to_lexical: false };
    let norm = policy.normalized();
    assert_eq!(norm.max_retries, 2);
    assert_eq!(norm.timeout_ms, 5000);
}

#[tokio::test]
async fn test_kb_api_027() {
    let policy = ResiliencePolicy { max_retries: 1, timeout_ms: 1000, fallback_to_lexical: true };
    let res = mirr_kb_native::resilience::run_with_resilience(policy, || async { Ok(42) }).await;
    assert_eq!(res.unwrap(), 42);
}

#[test]
fn test_kb_api_028() {
    let variants = expand_query_variants("totality check", ExpansionMode::None);
    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0].0, "totality check");
    assert_eq!(variants[0].1, 1.0);
}

#[test]
fn test_kb_api_029() {
    let variants = expand_query_variants("totality check", ExpansionMode::Synonym);
    assert!(variants.len() >= 1);
}

#[test]
fn test_kb_api_030() {
    let hash = compute_hash("sample text content");
    assert!(!hash.is_empty());
}

#[test]
fn test_kb_api_031() {
    let count = estimate_token_count("five words sample string text");
    assert!(count > 0);
}

#[test]
fn test_kb_api_032() {
    let chunk = MirrChunk::new(
        "k".to_string(),
        ChunkType::Module,
        "s".to_string(),
        "m".to_string(),
        None,
        (1, 10),
    );
    assert_eq!(chunk.chunk_type, ChunkType::Module);
    assert_eq!(chunk.id, "k");
}

#[test]
fn test_kb_api_033() {
    let pair = GoldenQAPair {
        query: "q".to_string(),
        expected_chunks: vec!["c".to_string()],
        expected_answer: "a".to_string(),
        difficulty: Difficulty::Easy,
    };
    assert_eq!(pair.difficulty, Difficulty::Easy);
}

#[tokio::test]
async fn test_kb_api_034() {
    let embed = mirr_kb_native::config::EmbeddingProvider::Local;
    assert_eq!(embed, mirr_kb_native::config::EmbeddingProvider::Local);
}

#[tokio::test]
async fn test_kb_api_035() {
    use mirr_kb_native::adapters::embedding::EmbeddingProvider;
    let provider = StubEmbeddingProvider;
    let res = provider.embed("text").await;
    assert!(res.is_err());
}

// --- Helper Functions for Category 4 (CLI binary execution) ---
fn get_kb_bin_path(name: &str) -> std::path::PathBuf {
    let mut exe = std::env::current_exe().expect("Failed to get current executable path");
    exe.pop(); // remove test binary name
    exe.pop(); // remove 'deps' directory
    let bin_name = if cfg!(windows) { format!("{}.exe", name) } else { name.to_string() };
    exe.join(bin_name)
}

fn run_kb_cli_no_file(args: &[&str]) -> Output {
    let bin_path = get_kb_bin_path("mirr-kb-native");
    std::process::Command::new(bin_path)
        .args(args)
        .output()
        .expect("Failed to execute mirr-kb-native")
}

fn run_kb_cli_with_env(args: &[&str], kb_root: &str) -> Output {
    let bin_path = get_kb_bin_path("mirr-kb-native");
    std::process::Command::new(bin_path)
        .args(args)
        .env("MIRR_KB_ROOT", kb_root)
        .output()
        .expect("Failed to execute mirr-kb-native with env")
}

// --- 036 - 050: CLI Binary Integration Tests ---
#[test]
fn test_kb_api_036() {
    let out = run_kb_cli_no_file(&["--help"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("MIRR Knowledge Base"));
}

#[test]
fn test_kb_api_037() {
    let out = run_kb_cli_no_file(&["-h"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("MIRR Knowledge Base"));
}

#[test]
fn test_kb_api_038() {
    let out = run_kb_cli_no_file(&["--help-json"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("mirrc-kb"));
}

#[test]
fn test_kb_api_039() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["status"], kb_root);
    assert!(out.status.success());
    let stdout_str = String::from_utf8_lossy(&out.stdout);
    assert!(stdout_str.contains("total_chunks") || stdout_str.contains("is_stale"));
}

#[test]
fn test_kb_api_040() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["query", "--text", "empty_query"], kb_root);
    assert!(out.status.success());
    let stdout_str = String::from_utf8_lossy(&out.stdout);
    assert!(stdout_str.contains("results") && stdout_str.contains("freshness"));
}

#[test]
fn test_kb_api_041() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["query", "--text", "search", "--mode", "lexical"], kb_root);
    assert!(out.status.success());
}

#[test]
fn test_kb_api_042() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["query", "--text", "search", "--mode", "semantic"], kb_root);
    assert!(out.status.success());
}

#[test]
fn test_kb_api_043() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["query", "--text", "search", "--mode", "hybrid"], kb_root);
    assert!(out.status.success());
}

#[test]
fn test_kb_api_044() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["brief", "--query", "search", "--format", "brief"], kb_root);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("answer"));
}

#[test]
fn test_kb_api_045() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["brief", "--query", "search", "--format", "bullet"], kb_root);
    assert!(out.status.success());
}

#[test]
fn test_kb_api_046() {
    let temp_dir = tempfile::tempdir().unwrap();
    let kb_root = temp_dir.path().to_str().unwrap();
    let out = run_kb_cli_with_env(&["brief", "--query", "search", "--format", "decision"], kb_root);
    assert!(out.status.success());
}

#[test]
fn test_kb_api_047() {
    let out = run_kb_cli_no_file(&["invalid_subcommand_xyz"]);
    assert!(!out.status.success());
}

#[test]
fn test_kb_api_048() {
    let out = run_kb_cli_no_file(&["query"]); // missing required --text
    assert!(!out.status.success());
}

#[test]
fn test_kb_api_049() {
    let bin_path = get_kb_bin_path("mirr-kb-index");
    let out = std::process::Command::new(bin_path)
        .arg("--help")
        .output()
        .expect("Failed to execute mirr-kb-index");
    assert!(out.status.success());
}

#[test]
fn test_kb_api_050() {
    let bin_path = get_kb_bin_path("mirr-kb-hydrate");
    let out = std::process::Command::new(bin_path)
        .arg("--help")
        .output()
        .expect("Failed to execute mirr-kb-hydrate");
    assert!(out.status.success());
}
