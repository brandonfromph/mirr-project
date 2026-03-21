#![forbid(unsafe_code)]
// ---------------------------------------------------------------------------
// Stress / edge-case tests
// ---------------------------------------------------------------------------

use nasa_rust_project::parse_mirr;
use nasa_rust_project::MirrProgram;

fn assert_parse_ok(source: &str) -> MirrProgram {
    parse_mirr(source).expect("expected parse to succeed")
}

#[test]
fn stress_many_signals() {
    let mut src = String::from("module big {\n");
    for i in 0..200 {
        src.push_str(&format!("    signal s{}: in bool;\n", i));
    }
    src.push('}');
    let p = assert_parse_ok(&src);
    assert_eq!(p.module.signals.len(), 200);
}

#[test]
fn stress_many_guards() {
    let mut src = String::from("module big {\n    signal s: in bool;\n");
    for i in 0..100 {
        src.push_str(&format!(
            "    guard g{} {{\n        when s\n        for {} cycles;\n    }}\n",
            i, i
        ));
    }
    src.push('}');
    let p = assert_parse_ok(&src);
    assert_eq!(p.module.guards.len(), 100);
}

#[test]
fn stress_many_reflexes() {
    let mut src = String::from(
        "module big {\n    signal s: out bool;\n    guard g {\n        when s\n        for 1 cycles;\n    }\n",
    );
    for i in 0..80 {
        src.push_str(&format!(
            "    reflex r{} {{\n        on g {{\n            s = true;\n        }}\n    }}\n",
            i
        ));
    }
    src.push('}');
    let p = assert_parse_ok(&src);
    assert_eq!(p.module.reflexes.len(), 80);
}

#[test]
fn stress_full_module() {
    let mut src = String::from("module stress {\n");
    for i in 0..50 {
        src.push_str(&format!("    signal s{}: in bool;\n", i));
    }
    for i in 0..30 {
        src.push_str(&format!(
            "    guard g{} {{\n        when s0\n        for {} cycles;\n    }}\n",
            i,
            i + 1
        ));
    }
    // Need writable signals for reflex assignments.
    src.push_str("    signal out0: out bool;\n");
    for i in 0..20 {
        src.push_str(&format!(
            "    reflex r{} {{\n        on g0 {{\n            out0 = true;\n        }}\n    }}\n",
            i
        ));
    }
    src.push('}');
    let p = assert_parse_ok(&src);
    assert_eq!(p.module.signals.len(), 51); // 50 inputs + 1 output
    assert_eq!(p.module.guards.len(), 30);
    assert_eq!(p.module.reflexes.len(), 20);
}

#[test]
fn stress_large_cycle_count() {
    let source = r#"
module t {
    signal s: in bool;
    guard g {
        when s
        for 18446744073709551615 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards[0].cycles, u64::MAX);
}

#[test]
fn stress_deep_expression_nesting_error() {
    // Test that the parser handles deep nesting by returning an error instead of crashing.
    let mut expr = String::from("s");
    for _ in 0..200 {
        expr = format!("(!({}))", expr);
    }
    let source = format!(
        "module deep {{\n    signal s: in bool;\n    signal out: out bool;\n    reflex r {{\n        on g {{\n            out = {};\n        }}\n    }}\n    guard g {{\n        when s\n        for 1 cycles;\n    }}\n}}",
        expr
    );
    let result = parse_mirr(&source);
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(
        error_msg.contains("Unbalanced parentheses"),
        "expected 'Unbalanced parentheses' error, got: {}",
        error_msg
    );
}

#[test]
fn stress_extreme_large_module() {
    // Generate a module with a large number of components.
    let mut src = String::from("module extreme {\n");
    let count = 2000;
    for i in 0..count {
        src.push_str(&format!("    signal s{}: in bool;\n", i));
        src.push_str(&format!("    signal o{}: out bool;\n", i));
        src.push_str(&format!(
            "    guard g{} {{\n        when s{}\n        for 1 cycles;\n    }}\n",
            i, i
        ));
        src.push_str(&format!(
            "    reflex r{} {{\n        on g{} {{\n            o{} = s{};\n        }}\n    }}\n",
            i, i, i, i
        ));
    }
    src.push('}');
    let p = assert_parse_ok(&src);
    assert_eq!(p.module.signals.len(), count * 2);
    assert_eq!(p.module.guards.len(), count);
    assert_eq!(p.module.reflexes.len(), count);
}
