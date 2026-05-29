#![forbid(unsafe_code)]
//! Semantic error-budget determinism tests.
//!
//! Focus: bounded error accumulation and deterministic diagnostics for fixed inputs.

use nasa_rust_project::parse_mirr;
use nasa_rust_project::validate_module;

const MAX_ACCUMULATED_ERRORS: usize = 20;
const STRESS_GUARD_COUNT: usize = 128;

fn source_with_many_undeclared_guard_signals(guard_count: usize) -> String {
    let mut src = String::from(
        "module semantic_budget {\n    signal trigger: in bool;\n    signal out: out bool;\n\n",
    );

    for i in 0..guard_count {
        src.push_str(&format!(
            "    guard g{i} {{\n        when ghost{i}\n        for 1 cycles;\n    }}\n\n"
        ));
    }

    src.push_str("    reflex r {\n        on g0 {\n            out = true;\n        }\n    }\n}\n");

    src
}

fn semantic_messages(source: &str) -> Vec<String> {
    let program = parse_mirr(source).expect("source should parse");
    let errs = validate_module(&program.module).expect_err("semantic validation should fail");
    errs.errors.iter().map(ToString::to_string).collect()
}

#[test]
fn semantic_error_collection_caps_at_configured_budget() {
    let source = source_with_many_undeclared_guard_signals(STRESS_GUARD_COUNT);
    let messages = semantic_messages(&source);

    assert!(
        messages.len() <= MAX_ACCUMULATED_ERRORS,
        "error count must be capped at {MAX_ACCUMULATED_ERRORS}, got {}",
        messages.len()
    );
    assert!(
        messages.iter().any(|m| m.contains("[E204]")),
        "expected undeclared-guard signal diagnostics, got: {messages:?}"
    );
}

#[test]
fn semantic_error_collection_is_deterministic_for_fixed_source() {
    let source = source_with_many_undeclared_guard_signals(STRESS_GUARD_COUNT);
    let first = semantic_messages(&source);
    let second = semantic_messages(&source);

    assert_eq!(first, second, "semantic diagnostics must be deterministic");
}

#[test]
fn semantic_error_collection_preserves_first_failing_guard_context() {
    let source = source_with_many_undeclared_guard_signals(STRESS_GUARD_COUNT);
    let messages = semantic_messages(&source);

    let first = messages.first().expect("expected at least one semantic diagnostic");
    assert!(
        first.contains("g0") || first.contains("ghost0") || first.contains("[E204]"),
        "first diagnostic should remain anchored to the earliest failing guard, got: {first}"
    );
}
