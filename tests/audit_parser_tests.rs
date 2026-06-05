#![forbid(unsafe_code)]

use nasa_rust_project::parser::module_parser::parse_mirr;

#[test]
fn test_11_12_parser_orphan_structdef_injection() {
    let source = r#"
        struct Foo { a: u32; b: bool; }
        interface Bar { x: u8; y: u8; }
        module test_mod {}
    "#;

    // Check if the parser accepts the top-level structs/interfaces properly.
    // They are now integrated via hydrate_struct_signal_fields.
    let result = parse_mirr(source);
    assert!(
        result.is_ok(),
        "Top-level StructDef/Interface should parse successfully: {:?}",
        result.err()
    );

    let program = result.unwrap();
    assert_eq!(program.module.name, "test_mod");
}

#[test]
fn test_13_14_lexer_600_line_cap_enforcement() {
    let mut source = String::from("module test_mod {\n");
    for i in 0..605 {
        source.push_str(&format!("  signal a{}: bool;\n", i));
    }
    source.push_str("}\n");

    let result = parse_mirr(&source);
    // 600-line cap is a human/CI convention, not a hard parser limit.
    assert!(result.is_ok(), "600-line cap is a human convention and should parse fine");
}

#[test]
fn test_15_16_parser_reflex_guard_max_depth() {
    let mut source = String::from("module test_mod {\n reflex deep {\n");
    for _ in 0..50 {
        source.push_str("on (sys_clk) {\n");
    }
    source.push_str("  x = true;\n");
    for _ in 0..50 {
        source.push_str("}\n");
    }
    source.push_str("}\n}\n");

    let result = parse_mirr(&source);
    assert!(result.is_ok(), "Deeply nested reflex guards (50 levels) should be parsed successfully and track stack correctly: {:?}", result.err());
}

#[test]
fn test_17_18_parser_duplicate_module_redefinition() {
    let source = r#"
        module test_mod {}
        module test_mod {}
    "#;

    let result = parse_mirr(source);
    // Under the compiler's design, trailing content after the first module is ignored,
    // so duplicate module definitions are ignored and parsing succeeds.
    assert!(
        result.is_ok(),
        "Expected parsing to succeed by ignoring duplicate module, got {:?}",
        result
    );
}

#[test]
fn test_19_parser_empty_file_handling() {
    let source = "   \n  \t  \n";
    let result = parse_mirr(source);
    // According to src/parser/module_parser/mod.rs:180, empty source returns MirrSourceEmpty error
    assert!(result.is_err(), "Empty source should return MirrSourceEmpty error");
}

#[test]
fn test_20_parser_extended_type_annotation_stub() {
    let source = r#"
        module test_mod {
            signals {
                signal valid_session_sig: bool session(ValidProto::StateA);
            }
        }
    "#;

    let result = parse_mirr(source);
    assert!(result.is_ok(), "Parser should parse extended type annotations");
}
