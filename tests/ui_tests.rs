#![forbid(unsafe_code)]
//! UI tests — compile .mirr files and compare diagnostic output against blessed .stderr files.
//!
//! For each .mirr file in tests/ui/, we compile it and compare the error output
//! against the corresponding .stderr file. This catches regressions in error messages.

use std::fs;
use std::path::Path;

/// Compile a .mirr source and return the error string (empty if no errors).
fn compile_and_capture(source: &str) -> String {
    let result = mirrc::parser::parse_mirr(source);
    match result {
        Ok(program) => {
            // Try validation
            match mirrc::validation::validate_module(&program.module) {
                Ok(()) => String::new(),
                Err(e) => format!("{e}"),
            }
        }
        Err(e) => format!("{e}"),
    }
}

/// Find all .mirr files in tests/ui/ subdirectories and test each one.
fn find_ui_tests(dir: &Path) -> Vec<(String, String, String)> {
    let mut tests = Vec::new();
    const MAX_ENTRIES: usize = 256;
    if !dir.exists() {
        return tests;
    }
    let entries: Vec<_> = fs::read_dir(dir)
        .expect("failed to read ui test directory")
        .filter_map(|e| e.ok())
        .collect();
    for (count, entry) in entries.iter().enumerate() {
        if count >= MAX_ENTRIES {
            break;
        }
        let path = entry.path();
        if path.is_dir() {
            tests.extend(find_ui_tests(&path));
        } else if path.extension().is_some_and(|e| e == "mirr") {
            let mirr_path = path.clone();
            let stderr_path = path.with_extension("stderr");
            let name =
                mirr_path.strip_prefix("tests/ui").unwrap_or(&mirr_path).display().to_string();
            let source = fs::read_to_string(&mirr_path)
                .unwrap_or_else(|_| panic!("failed to read {}", mirr_path.display()));
            let expected = if stderr_path.exists() {
                fs::read_to_string(&stderr_path)
                    .unwrap_or_else(|_| panic!("failed to read {}", stderr_path.display()))
            } else {
                String::new()
            };
            tests.push((name, source, expected));
        }
    }
    tests
}

#[test]
fn ui_test_suite() {
    let ui_dir = Path::new("tests/ui");
    let tests = find_ui_tests(ui_dir);
    assert!(!tests.is_empty(), "No UI tests found in tests/ui/ — expected .mirr + .stderr pairs");

    let mut failures = Vec::new();
    const MAX_TESTS: usize = 256;

    for (i, (name, source, expected)) in tests.iter().enumerate() {
        if i >= MAX_TESTS {
            break;
        }
        let actual = compile_and_capture(source);
        let expected_trimmed = expected.trim();
        let actual_trimmed = actual.trim();

        if expected_trimmed.is_empty() {
            // No .stderr file or empty — expect success (no errors)
            if !actual_trimmed.is_empty() {
                failures.push(format!("FAIL {name}: expected success but got:\n{actual_trimmed}"));
            }
        } else {
            // Check that actual output contains the expected error pattern
            if !actual_trimmed.contains(expected_trimmed) {
                failures.push(format!(
                    "FAIL {name}:\n  expected to contain: {expected_trimmed}\n  actual: {actual_trimmed}"
                ));
            }
        }
    }

    if !failures.is_empty() {
        panic!("{} UI test(s) failed:\n{}", failures.len(), failures.join("\n\n"));
    }
}
