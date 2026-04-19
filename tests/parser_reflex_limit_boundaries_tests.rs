#![forbid(unsafe_code)]
//! Parser reflex bound integration tests.
//!
//! Covers MAX_GUARD_NAMES, MAX_ASSIGNMENTS, and MAX_REFLEX_BODY_LINES.

use nasa_rust_project::parse_mirr;

fn parse_err(source: &str) -> String {
    parse_mirr(source).expect_err("should fail parse").to_string()
}

fn source_with_guard_list(guard_count: usize) -> String {
    let mut source = String::from(
        "module reflex_bounds {\n    signal trig: in bool;\n    signal out1: out bool;\n\n    reflex r {\n        on [",
    );

    for i in 0..guard_count {
        if i > 0 {
            source.push_str(" and ");
        }
        source.push_str(&format!("g{i}"));
    }

    source.push_str("] {\n            out1 = true;\n        }\n    }\n}\n");
    source
}

fn source_with_assignments(assignment_count: usize) -> String {
    let mut source = String::from(
        "module reflex_assignments {\n    signal trig: in bool;\n    signal out1: out bool;\n\n    reflex r {\n        on g {\n",
    );

    for i in 0..assignment_count {
        let value = if i % 2 == 0 { "true" } else { "false" };
        source.push_str(&format!("            out1 = {value};\n"));
    }

    source.push_str("        }\n    }\n}\n");
    source
}

fn source_with_large_reflex_body(comment_lines: usize) -> String {
    let mut source = String::from(
        "module reflex_body_limit {\n    signal trig: in bool;\n    signal out1: out bool;\n\n    reflex r {\n        on g {\n",
    );

    for i in 0..comment_lines {
        source.push_str(&format!("            // filler line {i}\n"));
    }

    source.push_str("            out1 = true;\n        }\n    }\n}\n");
    source
}

#[test]
fn reflex_allows_exactly_max_guard_names() {
    let source = source_with_guard_list(64);
    let program = parse_mirr(&source).expect("64 guard names should parse");
    assert_eq!(program.module.reflexes.len(), 1);
    assert_eq!(program.module.reflexes[0].guard_names.len(), 64);
}

#[test]
fn reflex_too_many_guard_names_reports_e141() {
    let source = source_with_guard_list(65);
    let msg = parse_err(&source);
    assert!(msg.contains("[E141]"), "expected E141, got: {msg}");
    assert!(msg.contains("MAX_GUARD_NAMES (64)"), "expected guard-name bound detail, got: {msg}");
}

#[test]
fn reflex_too_many_assignments_reports_e143() {
    let source = source_with_assignments(257);
    let msg = parse_err(&source);
    assert!(msg.contains("[E143]"), "expected E143, got: {msg}");
    assert!(msg.contains("MAX_ASSIGNMENTS (256)"), "expected assignment bound detail, got: {msg}");
}

#[test]
fn reflex_oversized_body_reports_e142() {
    let source = source_with_large_reflex_body(5000);
    let msg = parse_err(&source);
    assert!(msg.contains("[E142]"), "expected E142, got: {msg}");
    assert!(
        msg.contains("MAX_REFLEX_BODY_LINES (4096)"),
        "expected reflex-body bound detail, got: {msg}"
    );
}
