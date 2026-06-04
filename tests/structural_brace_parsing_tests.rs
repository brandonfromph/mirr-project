//! Unit and integration tests for structural brace parsing in the presence of template/macro structures.

#![forbid(unsafe_code)]

// Re-export compiler dependencies so that `crate::ast`, `crate::parser`, and `crate::simplify`
// resolve correctly when the compiler source code is included via `#[path]`.
pub mod ast {
    pub use nasa_rust_project::ast::*;
}
pub mod parser {
    pub use nasa_rust_project::parser::*;
}
pub mod simplify {
    pub use nasa_rust_project::simplify::*;
}

#[path = "../src/compiler/mod.rs"]
mod compiler;

use compiler::macro_proc::expand_macros;
use nasa_rust_project::parser::parse_mirr;

// --- Algorithmic Unit Tests of the Brace-Splitting Logic ---

/// Duplicate of `split_at_structural_brace` for direct unit testing of its algorithm.
fn split_at_structural_brace(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;
    while i < len {
        if bytes[i] == b'$' && i + 1 < len && bytes[i + 1] == b'{' {
            i += 2;
            while i < len && bytes[i] != b'}' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'{' {
            return Some((&s[..i], &s[i + 1..]));
        }
        i += 1;
    }
    None
}

/// Duplicate of `find_structural_brace` for direct unit testing of its algorithm.
fn find_structural_brace(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() {
        if bytes[pos] == b'$' && pos + 1 < bytes.len() && bytes[pos + 1] == b'{' {
            pos += 2;
            while pos < bytes.len() && bytes[pos] != b'}' {
                pos += 1;
            }
            pos += 1;
            continue;
        }
        if bytes[pos] == b'{' {
            return Some(pos);
        }
        pos += 1;
    }
    None
}

#[test]
fn test_split_at_structural_brace_behavior() {
    // Normal brace, no interpolation
    assert_eq!(split_at_structural_brace("guard g_0 {"), Some(("guard g_0 ", "")));

    // Structural brace after template interpolation
    assert_eq!(split_at_structural_brace("guard g_${i} {"), Some(("guard g_${i} ", "")));

    // Multiple interpolations
    assert_eq!(
        split_at_structural_brace("guard g_${a}_${b} { when s_${a};"),
        Some(("guard g_${a}_${b} ", " when s_${a};"))
    );

    // No structural brace
    assert_eq!(split_at_structural_brace("guard g_${i}"), None);

    // Unclosed interpolation (should consume to end or recover gracefully)
    assert_eq!(split_at_structural_brace("guard g_${i"), None);
}

#[test]
fn test_find_structural_brace_behavior() {
    // Normal brace, no interpolation
    assert_eq!(find_structural_brace("property p_0 {"), Some(13));

    // Structural brace after template interpolation (index 16)
    assert_eq!(find_structural_brace("property p_${i} {"), Some(16));

    // Multiple interpolations (index 21)
    assert_eq!(find_structural_brace("property p_${a}_${b} { assert always (x);"), Some(21));

    // No structural brace
    assert_eq!(find_structural_brace("property p_${i}"), None);
}

// --- High-Level Integration Tests for Parsers/Macro Integration ---

#[test]
fn test_parser_guard_template_braces() {
    let input = r#"
module test_mod {
    signal s_0: in bool;
    signal s_1: in bool;

    for i in 0..2 {
        guard g_${i} {
            when s_${i} for 2 cycles;
        }
    }
}"#;

    let expanded = expand_macros(input);
    let program = parse_mirr(&expanded).unwrap();

    assert_eq!(program.module.guards.len(), 2);
    assert_eq!(program.module.guards[0].name, "g_0");
    assert_eq!(program.module.guards[1].name, "g_1");
}

#[test]
fn test_parser_reflex_template_braces() {
    let input = r#"
module test_mod {
    signal in_0: in bool;
    signal in_1: in bool;
    signal out_0: out bool;
    signal out_1: out bool;
    
    guard g_0 { when in_0 for 1 cycles; }
    guard g_1 { when in_1 for 1 cycles; }

    for i in 0..2 {
        reflex r_${i} on g_${i} {
            out_${i} = true;
        }
    }
}"#;

    let expanded = expand_macros(input);
    let program = parse_mirr(&expanded).unwrap();

    // Verify reflex blocks correctly parsed and named
    assert_eq!(program.module.reflexes.len(), 2);

    let reflexes = &program.module.reflexes;
    assert!(
        reflexes
            .iter()
            .any(|r| r.name == "r_0"
                && r.guard_names == vec!["g_0".to_string(), "always".to_string()])
    );
    assert!(
        reflexes
            .iter()
            .any(|r| r.name == "r_1"
                && r.guard_names == vec!["g_1".to_string(), "always".to_string()])
    );
}

#[test]
fn test_parser_property_template_braces() {
    let input = r#"
module test_mod {
    signal req_0: in bool;
    signal ack_0: in bool;
    signal req_1: in bool;
    signal ack_1: in bool;

    for i in 0..2 {
        property p_${i} {
            always (req_${i} -> ack_${i});
        }
    }
}"#;

    let expanded = expand_macros(input);
    let program = parse_mirr(&expanded).unwrap();

    assert_eq!(program.module.properties.len(), 2);

    let prop_0 = &program.module.properties[0];
    assert_eq!(prop_0.name, "p_0");

    let prop_1 = &program.module.properties[1];
    assert_eq!(prop_1.name, "p_1");
}

#[test]
fn test_parser_malformed_guard_braces_error() {
    // Guard missing structural open brace
    let input = r#"
module test_mod {
    signal s_0: in bool;
    for i in 0..1 {
        guard g_${i}
            when s_${i} for 2 cycles;
    }
}"#;

    let expanded = expand_macros(input);
    let result = parse_mirr(&expanded);
    assert!(result.is_err());
}
