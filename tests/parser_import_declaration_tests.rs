#![forbid(unsafe_code)]
//! Parser import declaration integration tests.
//!
//! Covers happy-path import parsing and bounded E801/E802/E803 failures.

use nasa_rust_project::parse_mirr;

const MINIMAL_MODULE: &str = r#"
module import_smoke {
    signal a: in bool;
    signal b: out bool;
}
"#;

fn parse_err(source: &str) -> String {
    parse_mirr(source).expect_err("should fail parse").to_string()
}

#[test]
fn parses_imports_before_module() {
    let source = format!(
        "import \"std/logic.mirr\" as logic;\nimport \"std/temporal.mirr\" as temporal;\n{MINIMAL_MODULE}"
    );

    let program = parse_mirr(&source).expect("imports should parse");
    assert_eq!(program.imports.len(), 2);
    assert_eq!(program.imports[0].path, "std/logic.mirr");
    assert_eq!(program.imports[0].alias, "logic");
    assert_eq!(program.imports[1].path, "std/temporal.mirr");
    assert_eq!(program.imports[1].alias, "temporal");
}

#[test]
fn import_without_semicolon_reports_e801() {
    let source = format!("import \"std/logic.mirr\" as logic\n{MINIMAL_MODULE}");
    let msg = parse_err(&source);
    assert!(msg.contains("[E801]"), "expected E801, got: {msg}");
    assert!(msg.contains("must end with ';'"), "expected semicolon guidance, got: {msg}");
}

#[test]
fn import_without_alias_reports_e801() {
    let source = format!("import \"std/logic.mirr\";\n{MINIMAL_MODULE}");
    let msg = parse_err(&source);
    assert!(msg.contains("[E801]"), "expected E801, got: {msg}");
    assert!(msg.contains("must specify an alias"), "expected alias guidance, got: {msg}");
}

#[test]
fn import_with_empty_path_reports_e803() {
    let source = format!("import \"\" as logic;\n{MINIMAL_MODULE}");
    let msg = parse_err(&source);
    assert!(msg.contains("[E803]"), "expected E803, got: {msg}");
    assert!(
        msg.contains("Import path cannot be empty"),
        "expected empty-path guidance, got: {msg}"
    );
}

#[test]
fn too_many_imports_reports_e802() {
    let mut source = String::new();
    for i in 0..17 {
        source.push_str(&format!("import \"pkg_{i}.mirr\" as p{i};\n"));
    }
    source.push_str(MINIMAL_MODULE);

    let msg = parse_err(&source);
    assert!(msg.contains("[E802]"), "expected E802, got: {msg}");
    assert!(msg.contains("max 16"), "expected import bound detail, got: {msg}");
}
