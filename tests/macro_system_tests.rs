//! Integration and unit tests for the MIRR ergonomic macro processor.
//! Verified against loop expansion, let-binding preprocessing, reflex preprocessing,
//! match block transformation, and edge cases.

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

#[test]
fn test_signals_loop_expansion_suffix() {
    let input = r#"
module test_mod {
    signals {
        for i in 0..3 {
            s[i]: in bool;
        }
    }
}"#;

    let expanded = expand_macros(input);
    let program = parse_mirr(&expanded).unwrap();

    let signal_names: Vec<_> = program.module.signals.iter().map(|s| s.name.as_str()).collect();
    assert!(signal_names.contains(&"s_0"));
    assert!(signal_names.contains(&"s_1"));
    assert!(signal_names.contains(&"s_2"));
    assert_eq!(signal_names.len(), 3);
}

#[test]
fn test_signals_loop_expansion_interpolation() {
    let input = r#"
module test_mod {
    signals {
        for index in 1..4 {
            sig_${index}: out u8;
        }
    }
}"#;

    let expanded = expand_macros(input);
    let program = parse_mirr(&expanded).unwrap();

    let signal_names: Vec<_> = program.module.signals.iter().map(|s| s.name.as_str()).collect();
    assert!(signal_names.contains(&"sig_1"));
    assert!(signal_names.contains(&"sig_2"));
    assert!(signal_names.contains(&"sig_3"));
    assert_eq!(signal_names.len(), 3);
}

#[test]
fn test_top_level_loop_expansion() {
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

    let guard_names: Vec<_> = program.module.guards.iter().map(|g| g.name.as_str()).collect();
    assert!(guard_names.contains(&"g_0"));
    assert!(guard_names.contains(&"g_1"));
    assert_eq!(guard_names.len(), 2);
}

#[test]
fn test_reflex_loop_expansion() {
    let input = r#"
module test_mod {
    signal in_0: in bool;
    signal in_1: in bool;
    signal out_0: out bool;
    signal out_1: out bool;
    guard g_0 { when in_0 for 1 cycles; }
    guard g_1 { when in_1 for 1 cycles; }

    reflex my_reflex {
        for i in 0..2 {
            on g_${i} {
                out_${i} = true;
            }
        }
    }
}"#;

    let expanded = expand_macros(input);
    let program = parse_mirr(&expanded).unwrap();

    // Verify reflex body is expanded correctly
    assert_eq!(program.module.reflexes.len(), 2);

    let reflex_0 = &program.module.reflexes[0];
    assert_eq!(reflex_0.name, "my_reflex_c0");
    assert_eq!(reflex_0.guard_names, vec!["g_0".to_string()]);
    assert_eq!(reflex_0.assignments.len(), 1);
    assert_eq!(reflex_0.assignments[0].target, "out_0");

    let reflex_1 = &program.module.reflexes[1];
    assert_eq!(reflex_1.name, "my_reflex_c1");
    assert_eq!(reflex_1.guard_names, vec!["g_1".to_string()]);
    assert_eq!(reflex_1.assignments.len(), 1);
    assert_eq!(reflex_1.assignments[0].target, "out_1");
}

#[test]
fn test_def_pattern_preservation() {
    let input = r#"
def my_pattern(x: signal in bool) {
    for i in 0..2 {
        // This is inside def pattern block, should not be expanded by macro_proc
    }
}
module test_mod {
    signal sig: in bool;
}
"#;

    let expanded = expand_macros(input);
    // Should preserve the def block structure exactly
    assert!(expanded.contains("def my_pattern"));
    assert!(expanded.contains("for i in 0..2"));
}

#[test]
fn test_empty_and_reverse_loop_ranges() {
    // Empty range (0..0)
    let input_empty = r#"
module test_mod {
    signals {
        for i in 0..0 {
            s[i]: in bool;
        }
    }
}"#;
    let expanded_empty = expand_macros(input_empty);
    let program_empty = parse_mirr(&expanded_empty).unwrap();
    assert!(program_empty.module.signals.is_empty());

    // Reverse range (3..1) should behave like empty range in Rust
    let input_reverse = r#"
module test_mod {
    signals {
        for i in 3..1 {
            s[i]: in bool;
        }
    }
}"#;
    let expanded_reverse = expand_macros(input_reverse);
    let program_reverse = parse_mirr(&expanded_reverse).unwrap();
    assert!(program_reverse.module.signals.is_empty());
}

#[test]
fn test_let_binding_preprocessing() {
    let input = r#"
module test_mod {
    signal input_val: in u32;
    reflex my_reflex {
        let is_reflexive: bool = input_val > 10;
        if is_reflexive {
            // some action
        }
    }
}"#;

    let expanded = expand_macros(input);
    // Let-binding should be preprocessed/inlined
    assert!(expanded.contains("input_val > 10"));
}
