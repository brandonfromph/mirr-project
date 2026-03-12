#![forbid(unsafe_code)]
// ---------------------------------------------------------------------------
// Self-Hosting IR Schema Tests
// ---------------------------------------------------------------------------
// Verifies that the Rust compiler pipeline produces JSON output matching
// the canonical IR contract defined in docs/self_hosting_ir_contract.md.
//
// Test coverage:
//   - AST JSON matches golden fixture files in tests/fixtures/ast/
//   - Netlist JSON matches golden fixture files in tests/fixtures/netlist/
//   - ir_version field is present and correct
//   - Round-trip: parse JSON back into structs and compare equality
// ---------------------------------------------------------------------------

use nasa_rust_project::{parse_mirr, MirrAstJson, TemporalGuardCompiler, TemporalNetlistJson};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Load a golden fixture file relative to the workspace root.
fn load_fixture(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read fixture '{}': {}", path, e))
}

/// Normalize JSON for comparison: parse then re-serialize with serde_json to
/// strip extra whitespace and normalize key ordering from serde's default output.
fn normalize_json(raw: &str) -> serde_json::Value {
    serde_json::from_str(raw)
        .unwrap_or_else(|e| panic!("Failed to parse JSON: {}. Input was:\n{}", e, raw))
}

// ---------------------------------------------------------------------------
// AST IR schema tests
// ---------------------------------------------------------------------------

#[test]
fn ast_ir_version_field_is_correct() {
    let src = r#"
module test {
    signal s: in bool;
    guard g {
        when s
        for 1 cycles;
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let ast_json = MirrAstJson::from_program(&program);
    assert_eq!(ast_json.ir_version, "2.0");
}

#[test]
fn ast_serializes_to_valid_json() {
    let src = r#"
module test {
    signal s: in bool;
    guard g {
        when s
        for 1 cycles;
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let ast_json = MirrAstJson::from_program(&program);
    let json_str = serde_json::to_string_pretty(&ast_json).expect("serialization failed");
    // Must be valid JSON
    let _parsed: serde_json::Value = serde_json::from_str(&json_str).expect("invalid JSON output");
}

#[test]
fn ast_roundtrip_neonatal_respirator() {
    // Parse the canonical neonatal respirator example.
    let src = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;

    let program = parse_mirr(src).expect("parse failed");
    let produced = MirrAstJson::from_program(&program);

    // Load the golden fixture.
    let fixture_str = load_fixture("tests/fixtures/ast/neonatal_respirator.json");
    let expected: MirrAstJson =
        serde_json::from_str(&fixture_str).expect("golden fixture is invalid JSON");

    assert_eq!(
        produced, expected,
        "AST JSON does not match golden fixture tests/fixtures/ast/neonatal_respirator.json"
    );
}

#[test]
fn ast_json_contains_ir_version_key() {
    // The wire format must include the ir_version field at the root.
    let src = r#"
module m {
    signal x: in bool;
    guard g {
        when x
        for 4 cycles;
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let ast_json = MirrAstJson::from_program(&program);
    let json_str = serde_json::to_string(&ast_json).expect("serialization failed");
    assert!(json_str.contains("\"ir_version\""), "JSON output must contain ir_version key");
    assert!(json_str.contains("\"2.0\""), "ir_version value must be \"2.0\"");
}

#[test]
fn ast_golden_fixture_neonatal_signal_names() {
    let fixture_str = load_fixture("tests/fixtures/ast/neonatal_respirator.json");
    let value = normalize_json(&fixture_str);

    let signals = value["module"]["signals"].as_array().expect("signals must be array");
    let names: Vec<&str> =
        signals.iter().map(|s| s["name"].as_str().expect("signal name must be string")).collect();

    assert_eq!(names, ["respirator_enable", "airway_pressure", "clamp_valve"]);
}

#[test]
fn ast_golden_fixture_neonatal_guard_condition() {
    let fixture_str = load_fixture("tests/fixtures/ast/neonatal_respirator.json");
    let value = normalize_json(&fixture_str);

    let guard = &value["module"]["guards"][0];
    assert_eq!(guard["name"].as_str().unwrap(), "sustained_pressure_drop");
    assert_eq!(guard["cycles"].as_u64().unwrap(), 1000);

    let cond = &guard["condition"]["Binary"];
    assert_eq!(cond["op"].as_str().unwrap(), "Lt");
    assert_eq!(cond["left"]["Signal"].as_str().unwrap(), "airway_pressure");
    assert_eq!(cond["right"]["Literal"]["Integer"].as_u64().unwrap(), 50);
}

#[test]
fn ast_golden_fixture_seizure_monitor() {
    let fixture_str = load_fixture("tests/fixtures/ast/seizure_monitor.json");
    let value = normalize_json(&fixture_str);

    assert_eq!(value["module"]["name"].as_str().unwrap(), "seizure_monitor");
    let signals = value["module"]["signals"].as_array().unwrap();
    assert_eq!(signals.len(), 4);

    let guard = &value["module"]["guards"][0];
    assert_eq!(guard["name"].as_str().unwrap(), "seizure_pattern");
    assert_eq!(guard["cycles"].as_u64().unwrap(), 32);

    // Condition: eeg_spike && !artifact_noise
    let cond = &guard["condition"]["Binary"];
    assert_eq!(cond["op"].as_str().unwrap(), "And");
    assert_eq!(cond["left"]["Signal"].as_str().unwrap(), "eeg_spike");
    assert_eq!(cond["right"]["Unary"]["op"].as_str().unwrap(), "Not");
    assert_eq!(cond["right"]["Unary"]["operand"]["Signal"].as_str().unwrap(), "artifact_noise");
}

// ---------------------------------------------------------------------------
// Netlist IR schema tests
// ---------------------------------------------------------------------------

#[test]
fn netlist_ir_version_field_is_correct() {
    let src = r#"
module test {
    signal s: in bool;
    signal out: out bool;
    guard g {
        when s
        for 4 cycles;
    }
    reflex r {
        on g {
            out = s;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");
    let netlist_json = TemporalNetlistJson::from_netlist(&netlist);
    assert_eq!(netlist_json.ir_version, "2.0");
}

#[test]
fn netlist_serializes_to_valid_json() {
    let src = r#"
module test {
    signal s: in bool;
    signal out: out bool;
    guard g {
        when s
        for 4 cycles;
    }
    reflex r {
        on g {
            out = s;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");
    let netlist_json = TemporalNetlistJson::from_netlist(&netlist);
    let json_str = serde_json::to_string_pretty(&netlist_json).expect("serialization failed");
    let _parsed: serde_json::Value = serde_json::from_str(&json_str).expect("invalid JSON output");
}

#[test]
fn netlist_golden_fixture_neonatal_respirator() {
    let src = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");
    let produced = TemporalNetlistJson::from_netlist(&netlist);

    let fixture_str = load_fixture("tests/fixtures/netlist/neonatal_respirator.json");
    let expected: TemporalNetlistJson =
        serde_json::from_str(&fixture_str).expect("golden fixture is invalid JSON");

    assert_eq!(
        produced, expected,
        "Netlist JSON does not match golden fixture tests/fixtures/netlist/neonatal_respirator.json"
    );
}

#[test]
fn netlist_golden_fixture_counter_strategy_used() {
    // 1000 cycles → Counter strategy
    let fixture_str = load_fixture("tests/fixtures/netlist/neonatal_respirator.json");
    let value = normalize_json(&fixture_str);

    let guards = value["guards"].as_array().expect("guards must be array");
    assert_eq!(guards.len(), 1);
    assert!(guards[0].get("Counter").is_some(), "1000-cycle guard must use Counter strategy");

    let counter = &guards[0]["Counter"];
    assert_eq!(counter["name"].as_str().unwrap(), "sustained_pressure_drop");
    assert_eq!(counter["target_count"].as_u64().unwrap(), 1000);
    assert_eq!(counter["input_signal"].as_str().unwrap(), "airway_pressure");

    let cmp = &counter["condition_kind"]["Comparison"];
    assert_eq!(cmp["op"].as_str().unwrap(), "Lt");
    assert_eq!(cmp["value"]["Integer"].as_u64().unwrap(), 50);
}

#[test]
fn netlist_statistics_fields_present_in_json() {
    let src = r#"
module m {
    signal s: in bool;
    signal o: out bool;
    guard g {
        when s
        for 4 cycles;
    }
    reflex r {
        on g {
            o = s;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");
    let netlist_json = TemporalNetlistJson::from_netlist(&netlist);
    let json_str = serde_json::to_string(&netlist_json).expect("serialization failed");
    let value: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    let stats = &value["statistics"];
    assert!(stats.get("shift_registers_used").is_some());
    assert!(stats.get("counters_used").is_some());
    assert!(stats.get("logic_gates_used").is_some());
    assert!(stats.get("max_delay_cycles").is_some());
    assert!(stats.get("total_signals").is_some());
    assert!(stats.get("compilation_time_us").is_some());
}
