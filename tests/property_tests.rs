//! Phase 7a: Safety property tests.
//!
//! Tests property parsing, validation, SVA emission, JSON emission,
//! DOT emission, and pipeline integration.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::property::{PropertyDecl, PropertyFormula};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue};
use nasa_rust_project::emit;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::validate_module;

// =========================================================================
// Helper: parse then return first property
// =========================================================================

fn parse_first_property(source: &str) -> PropertyDecl {
    let program = parse_mirr(source).expect("should parse");
    program.module.properties.into_iter().next().expect("should have a property")
}

fn parse_err(source: &str) -> String {
    parse_mirr(source).expect_err("should fail").to_string()
}

fn validate_err(source: &str) -> String {
    let program = parse_mirr(source).expect("should parse");
    validate_module(&program.module).expect_err("should fail validation").to_string()
}

/// Minimal module source with signals and a guard for property tests.
fn wrap_property(property_body: &str) -> String {
    format!(
        r#"
module test_mod {{
    signal x: in bool;
    signal y: out bool;
    signal z: in u16;

    guard g {{
        when x
        for 2 cycles;
    }}

    reflex r {{
        on g {{
            y = true;
        }}
    }}

    {property_body}
}}
"#
    )
}

// =========================================================================
// Parser tests
// =========================================================================

#[test]
fn parse_always_property_simple() {
    let src = wrap_property("property p1 {\n    always (x);\n}");
    let prop = parse_first_property(&src);
    assert_eq!(prop.name, "p1");
    assert!(matches!(prop.formula, PropertyFormula::Always(Expr::Signal(ref s)) if s == "x"));
}

#[test]
fn parse_never_property_simple() {
    let src = wrap_property("property p1 {\n    never (y);\n}");
    let prop = parse_first_property(&src);
    assert_eq!(prop.name, "p1");
    assert!(matches!(prop.formula, PropertyFormula::Never(Expr::Signal(ref s)) if s == "y"));
}

#[test]
fn parse_always_implies_property() {
    let src = wrap_property("property p1 {\n    always (x -> y);\n}");
    let prop = parse_first_property(&src);
    assert_eq!(prop.name, "p1");
    match &prop.formula {
        PropertyFormula::AlwaysImplies { antecedent, consequent } => {
            assert!(matches!(antecedent, Expr::Signal(ref s) if s == "x"));
            assert!(matches!(consequent, Expr::Signal(ref s) if s == "y"));
        }
        other => panic!("Expected AlwaysImplies, got: {:?}", other),
    }
}

#[test]
fn parse_always_complex_implies() {
    let src = wrap_property("property p1 {\n    always (z < 50 -> y);\n}");
    let prop = parse_first_property(&src);
    match &prop.formula {
        PropertyFormula::AlwaysImplies { antecedent, consequent } => {
            assert!(matches!(antecedent, Expr::Binary { op: BinaryOp::Lt, .. }));
            assert!(matches!(consequent, Expr::Signal(ref s) if s == "y"));
        }
        other => panic!("Expected AlwaysImplies, got: {:?}", other),
    }
}

#[test]
fn parse_always_complex_both_sides() {
    let src = wrap_property("property p1 {\n    always (z < 50 -> x && y);\n}");
    let prop = parse_first_property(&src);
    match &prop.formula {
        PropertyFormula::AlwaysImplies { antecedent, consequent } => {
            assert!(matches!(antecedent, Expr::Binary { op: BinaryOp::Lt, .. }));
            assert!(matches!(consequent, Expr::Binary { op: BinaryOp::And, .. }));
        }
        other => panic!("Expected AlwaysImplies, got: {:?}", other),
    }
}

#[test]
fn parse_property_missing_keyword_error() {
    let src = wrap_property("property p1 {\n    sometimes (x);\n}");
    let msg = parse_err(&src);
    assert!(
        msg.contains("must start with 'always'") || msg.contains("must start with"),
        "Unexpected error: {msg}"
    );
}

#[test]
fn parse_property_missing_closing_brace_error() {
    let src = r#"
module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p1 {
        always (x);
"#;
    let msg = parse_err(src);
    assert!(msg.contains("not closed with '}'"), "Unexpected error: {msg}");
}

#[test]
fn parse_property_empty_formula_error() {
    let src = wrap_property("property p1 {\n}");
    let msg = parse_err(&src);
    assert!(
        msg.contains("must start with 'always'") || msg.contains("must start with"),
        "Unexpected error: {msg}"
    );
}

#[test]
fn parse_property_nested_parens() {
    let src = wrap_property("property p1 {\n    always ((x && y) || z < 100);\n}");
    let prop = parse_first_property(&src);
    assert!(matches!(prop.formula, PropertyFormula::Always(Expr::Binary { op: BinaryOp::Or, .. })));
}

#[test]
fn parse_never_with_comparison() {
    let src = wrap_property("property p1 {\n    never (z > 100);\n}");
    let prop = parse_first_property(&src);
    match &prop.formula {
        PropertyFormula::Never(Expr::Binary { op: BinaryOp::Gt, right, .. }) => {
            assert!(matches!(right.as_ref(), Expr::Literal(LiteralValue::Integer(100))));
        }
        other => panic!("Expected Never(Binary Gt), got: {:?}", other),
    }
}

// =========================================================================
// Validation tests
// =========================================================================

#[test]
fn property_duplicate_name_pinned_message() {
    let src =
        wrap_property("property p1 {\n    always (x);\n}\n\nproperty p1 {\n    never (y);\n}");
    let msg = validate_err(&src);
    assert_eq!(msg, "Semantic error: [E210] Duplicate property name: 'p1'.");
}

#[test]
fn property_undeclared_signal_always_pinned() {
    let src = wrap_property("property p1 {\n    always (ghost);\n}");
    let msg = validate_err(&src);
    assert_eq!(msg, "Semantic error: [E211] Property 'p1' references undeclared signal 'ghost'.");
}

#[test]
fn property_undeclared_signal_never_pinned() {
    let src = wrap_property("property p1 {\n    never (phantom);\n}");
    let msg = validate_err(&src);
    assert_eq!(msg, "Semantic error: [E211] Property 'p1' references undeclared signal 'phantom'.");
}

#[test]
fn property_undeclared_signal_implies_antecedent_pinned() {
    let src = wrap_property("property p1 {\n    always (ghost -> y);\n}");
    let msg = validate_err(&src);
    assert_eq!(msg, "Semantic error: [E211] Property 'p1' references undeclared signal 'ghost'.");
}

#[test]
fn property_undeclared_signal_implies_consequent_pinned() {
    let src = wrap_property("property p1 {\n    always (x -> phantom);\n}");
    let msg = validate_err(&src);
    assert_eq!(msg, "Semantic error: [E211] Property 'p1' references undeclared signal 'phantom'.");
}

#[test]
fn property_valid_always_passes() {
    let src = wrap_property("property p1 {\n    always (x);\n}");
    let program = parse_mirr(&src).expect("should parse");
    validate_module(&program.module).expect("should pass validation");
}

#[test]
fn property_valid_never_passes() {
    let src = wrap_property("property p1 {\n    never (y);\n}");
    let program = parse_mirr(&src).expect("should parse");
    validate_module(&program.module).expect("should pass validation");
}

#[test]
fn property_valid_implies_passes() {
    let src = wrap_property("property p1 {\n    always (x -> y);\n}");
    let program = parse_mirr(&src).expect("should parse");
    validate_module(&program.module).expect("should pass validation");
}

// =========================================================================
// SVA emission tests
// =========================================================================

fn sv_with_property(property_body: &str) -> String {
    let src = wrap_property(property_body);
    let result = run_pipeline(&src, &PipelineConfig::default());
    match result {
        Ok(r) => emit::verilog::emit_sv(&r),
        Err(e) => panic!("Pipeline failed: {e}"),
    }
}

#[test]
fn sva_always_emits_assert_property() {
    let sv = sv_with_property("property p1 {\n    always (x);\n}");
    assert!(sv.contains("assert property"), "Missing assert property in:\n{sv}");
}

#[test]
fn sva_never_emits_negated() {
    let sv = sv_with_property("property p1 {\n    never (x);\n}");
    assert!(sv.contains("!(x)"), "Missing negation in:\n{sv}");
}

#[test]
fn sva_implies_emits_implication_operator() {
    let sv = sv_with_property("property p1 {\n    always (x -> y);\n}");
    assert!(sv.contains("|->"), "Missing |-> in:\n{sv}");
}

#[test]
fn sva_disable_iff_when_rst_n_declared() {
    let src = r#"
module with_rst {
    signal rst_n: in bool;
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p1 {
        always (x -> y);
    }
}
"#;
    let result = run_pipeline(src, &PipelineConfig::default()).expect("pipeline ok");
    let sv = emit::verilog::emit_sv(&result);
    assert!(sv.contains("disable iff (!rst_n)"), "Missing disable iff in:\n{sv}");
}

#[test]
fn sva_no_disable_iff_without_rst_n() {
    let sv = sv_with_property("property p1 {\n    always (x);\n}");
    assert!(!sv.contains("disable iff"), "Unexpected disable iff in:\n{sv}");
}

#[test]
fn sva_posedge_clk_always_present() {
    let sv = sv_with_property("property p1 {\n    always (x);\n}");
    assert!(sv.contains("@(posedge clk)"), "Missing @(posedge clk) in:\n{sv}");
}

#[test]
fn sva_standalone_mode_no_module_wrapper() {
    let src = wrap_property("property p1 {\n    always (x);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let sva = emit::verilog::emit_sva_only(&result);
    assert!(sva.contains("assert property"), "Missing assert property in standalone SVA");
    assert!(!sva.contains("module test_mod"), "Standalone SVA should not contain module decl");
    assert!(!sva.contains("endmodule"), "Standalone SVA should not contain endmodule");
}

#[test]
fn sva_complex_expression_renders_correctly() {
    let sv = sv_with_property("property p1 {\n    always (z < 50 -> x);\n}");
    assert!(sv.contains("(z < 50)"), "Missing comparison expression in:\n{sv}");
    assert!(sv.contains("|->"), "Missing implication in:\n{sv}");
}

#[test]
fn sva_embedded_in_full_sv() {
    let sv = sv_with_property("property p1 {\n    always (x);\n}");
    let assert_pos = sv.find("assert property").expect("should have assert");
    let endmod_pos = sv.find("endmodule").expect("should have endmodule");
    assert!(assert_pos < endmod_pos, "SVA assertion should appear before endmodule");
}

#[test]
fn sva_empty_properties_no_section() {
    let src = r#"
module no_props {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = run_pipeline(src, &PipelineConfig::default()).expect("pipeline ok");
    let sv = emit::verilog::emit_sv(&result);
    assert!(!sv.contains("Safety Properties"), "Should have no properties section");
}

// =========================================================================
// JSON emission tests
// =========================================================================

#[test]
fn json_properties_key_present() {
    let src = wrap_property("property p1 {\n    always (x);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let json = emit::json_netlist::emit_json(&result).expect("json ok");
    assert!(json.contains("\"properties\""), "Missing properties key in JSON");
}

#[test]
fn json_always_kind_string() {
    let src = wrap_property("property p1 {\n    always (x);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let netlist = emit::json_netlist::build_netlist(&result);
    assert_eq!(netlist.properties.len(), 1);
    assert_eq!(netlist.properties[0].kind, "always");
}

#[test]
fn json_never_kind_string() {
    let src = wrap_property("property p1 {\n    never (x);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let netlist = emit::json_netlist::build_netlist(&result);
    assert_eq!(netlist.properties[0].kind, "never");
}

#[test]
fn json_always_implies_kind_string() {
    let src = wrap_property("property p1 {\n    always (x -> y);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let netlist = emit::json_netlist::build_netlist(&result);
    assert_eq!(netlist.properties[0].kind, "always_implies");
}

#[test]
fn json_formula_text_readable() {
    let src = wrap_property("property p1 {\n    always (z < 50 -> x);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let netlist = emit::json_netlist::build_netlist(&result);
    let text = &netlist.properties[0].formula_text;
    assert!(text.contains("->"), "formula_text should contain ->: {text}");
    assert!(text.contains("50"), "formula_text should contain 50: {text}");
}

#[test]
fn json_empty_properties_array() {
    let src = r#"
module no_props {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = run_pipeline(src, &PipelineConfig::default()).expect("pipeline ok");
    let netlist = emit::json_netlist::build_netlist(&result);
    assert!(netlist.properties.is_empty());
}

// =========================================================================
// Pipeline / integration tests
// =========================================================================

#[test]
fn pipeline_property_semantic_error_propagates() {
    let src = wrap_property("property p1 {\n    always (ghost);\n}");
    let err = run_pipeline(&src, &PipelineConfig::default());
    match err {
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains("undeclared signal 'ghost'"), "Unexpected: {msg}");
        }
        Ok(_) => panic!("Expected pipeline error for undeclared signal"),
    }
}

#[test]
fn pipeline_property_duplicate_error() {
    let src =
        wrap_property("property p1 {\n    always (x);\n}\n\nproperty p1 {\n    never (y);\n}");
    let err = run_pipeline(&src, &PipelineConfig::default());
    match err {
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains("Duplicate property name"), "Unexpected: {msg}");
        }
        Ok(_) => panic!("Expected pipeline error for duplicate property"),
    }
}

#[test]
fn pipeline_properties_survive_simplification() {
    let src = wrap_property("property p1 {\n    always (x);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    assert_eq!(result.program.module.properties.len(), 1);
    assert_eq!(result.program.module.properties[0].name, "p1");
}

#[test]
fn pipeline_neonatal_with_properties_e2e() {
    let src = r#"
module neonatal_respirator {
    signal airway_pressure: in u16;
    signal clamp_valve: out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for 1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }

    property pressure_response {
        always (airway_pressure < 50 -> clamp_valve);
    }

    property no_spurious_clamp {
        never (clamp_valve);
    }
}
"#;
    let result = run_pipeline(src, &PipelineConfig::default()).expect("pipeline ok");
    let sv = emit::verilog::emit_sv(&result);
    assert!(sv.contains("assert property"), "SV should contain SVA assertions");
    assert!(sv.contains("|->"), "SV should contain implication");
    assert_eq!(result.program.module.properties.len(), 2);
}

// =========================================================================
// DOT emission tests
// =========================================================================

#[test]
fn dot_module_property_nodes_present() {
    let src = wrap_property("property p1 {\n    always (x);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let dot = emit::dot::emit_module_dot(&result);
    assert!(dot.contains("prop_p1"), "DOT should contain property node");
    assert!(dot.contains("shape=note"), "Property node should be shape=note");
    assert!(dot.contains("lightblue"), "Property node should be lightblue");
}

#[test]
fn dot_property_edges_connect_to_signals() {
    let src = wrap_property("property p1 {\n    always (x -> y);\n}");
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("pipeline ok");
    let dot = emit::dot::emit_module_dot(&result);
    assert!(dot.contains("x -> prop_p1"), "Should have edge from x to prop_p1");
    assert!(dot.contains("y -> prop_p1"), "Should have edge from y to prop_p1");
}
