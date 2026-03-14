#![forbid(unsafe_code)]
//! Temporal Guard Lowering Tests
//!
//! Requirement coverage: P2-REQ-001 through P2-REQ-006, P2-REQ-013 through P2-REQ-015
//! Ref: MIRR-PHASE2-001 §6 (traceability table)

use nasa_rust_project::ast::{
    types::{BinaryOp, LiteralValue, UnaryOp},
    Expr,
};
use nasa_rust_project::{
    parse_mirr,
    temporal::low_level_ir::{
        CompiledGuard, ConditionKind, GeneratedSignalKind, TemporalNetlistJson,
    },
    MirrError, TemporalGuardCompiler,
};

// ---------------------------------------------------------------------------
// P2-REQ-001: Short guard (N≤16) lowers to ShiftRegisterGuard
// ---------------------------------------------------------------------------
#[test]
fn test_shift_register_compilation() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard short_delay_guard {
        when input_signal
        for 4 cycles;
    }

    reflex test_reflex {
        on short_delay_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    assert_eq!(netlist.guards.len(), 1);
    match &netlist.guards[0] {
        CompiledGuard::ShiftRegister(sr) => {
            assert_eq!(sr.name, "short_delay_guard");
            assert_eq!(sr.delay_cycles, 4);
            assert_eq!(sr.stages.len(), 4);
            assert_eq!(sr.input_signal, "input_signal");
        }
        other => panic!("Expected ShiftRegister, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// P2-REQ-002: Long guard (N>16) lowers to CounterGuard
// ---------------------------------------------------------------------------
#[test]
fn test_counter_compilation() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard long_delay_guard {
        when input_signal
        for 100 cycles;
    }

    reflex test_reflex {
        on long_delay_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    assert_eq!(netlist.guards.len(), 1);
    match &netlist.guards[0] {
        CompiledGuard::Counter(c) => {
            assert_eq!(c.name, "long_delay_guard");
            assert_eq!(c.target_count, 100);
            assert_eq!(c.counter_width(), 8); // ceil(log2(100))+1 = 8
            assert_eq!(c.input_signal, "input_signal");
        }
        other => panic!("Expected Counter, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// P2-REQ-003: Mixed guards produce both types in one netlist
// ---------------------------------------------------------------------------
#[test]
fn test_mixed_guard_compilation() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard short_guard {
        when input_signal
        for 8 cycles;
    }

    guard long_guard {
        when input_signal
        for 200 cycles;
    }

    reflex test_reflex {
        on short_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    assert_eq!(netlist.guards.len(), 2);

    let sr_count =
        netlist.guards.iter().filter(|g| matches!(g, CompiledGuard::ShiftRegister(_))).count();
    let ctr_count =
        netlist.guards.iter().filter(|g| matches!(g, CompiledGuard::Counter(_))).count();

    assert_eq!(sr_count, 1, "expected one shift-register guard");
    assert_eq!(ctr_count, 1, "expected one counter guard");
}

// ---------------------------------------------------------------------------
// P2-REQ-004: Guard names are preserved in the netlist
// ---------------------------------------------------------------------------
#[test]
fn test_guard_names_preserved() {
    let src = r#"
module test_module {
    signal sensor_input: in bool;
    signal alarm_output: out bool;

    guard critical_system_guard {
        when sensor_input
        for 16 cycles;
    }

    guard safety_interlock_guard {
        when sensor_input
        for 32 cycles;
    }

    reflex test_reflex {
        on critical_system_guard {
            alarm_output = sensor_input;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    let names: Vec<&str> = netlist
        .guards
        .iter()
        .map(|g| match g {
            CompiledGuard::ShiftRegister(sr) => sr.name.as_str(),
            CompiledGuard::Counter(c) => c.name.as_str(),
            CompiledGuard::Complex(cx) => cx.name.as_str(),
            CompiledGuard::DynamicCounter(dc) => dc.name.as_str(),
        })
        .collect();

    assert!(names.contains(&"critical_system_guard"));
    assert!(names.contains(&"safety_interlock_guard"));
}

// ---------------------------------------------------------------------------
// P2-REQ-005: Zero-delay guard lowers to an empty ShiftRegisterGuard
// ---------------------------------------------------------------------------
#[test]
fn test_zero_delay_guard() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard zero_delay_guard {
        when input_signal
        for 0 cycles;
    }

    reflex test_reflex {
        on zero_delay_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    assert_eq!(netlist.guards.len(), 1);
    match &netlist.guards[0] {
        CompiledGuard::ShiftRegister(sr) => {
            assert_eq!(sr.delay_cycles, 0);
            assert_eq!(sr.stages.len(), 0, "zero delay should produce no stages");
        }
        other => panic!("Expected ShiftRegister for zero delay, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// P2-REQ-013: Unsupported condition form → explicit TemporalCompilationError
//             (no silent synthetic signal name)
// ---------------------------------------------------------------------------
#[test]
fn test_unsupported_condition_is_rejected() {
    // AND/OR conditions are now handled via ComplexGuard decomposition, but
    // a binary XOR of two signals is still not reducible by the lowering pass.
    // The compiler MUST return an explicit error rather than silently
    // producing incorrect hardware.
    let src = r#"
module test_module {
    signal sig_a: in bool;
    signal sig_b: in bool;
    signal result: out bool;

    guard bad_guard {
        when sig_a
        for 4 cycles;
    }

    reflex r {
        on bad_guard {
            result = sig_a;
        }
    }
}
"#;
    // The parse itself succeeds — the error must come from the compiler.
    let program = parse_mirr(src).expect("parse should succeed");

    // Manually inject an unsupported condition to bypass the parser's valid
    // subset (the parser only produces supported forms today).
    let mut module = program.module;
    // Replace the guard condition with `sig_a XOR sig_b` — unsupported by the lowering pass.
    if let Some(guard) = module.guards.first_mut() {
        guard.condition = Expr::Binary {
            op: BinaryOp::Xor,
            left: Box::new(Expr::Signal("sig_a".to_string())),
            right: Box::new(Expr::Signal("sig_b".to_string())),
        };
    }

    let result = TemporalGuardCompiler::new().compile_temporal_guards(&module);
    assert!(result.is_err(), "Expected a TemporalCompilationError for unsupported AND condition");
    match result.unwrap_err() {
        MirrError::TemporalCompilationError { message, .. } => {
            assert!(
                message.contains("bad_guard"),
                "Error message must identify the offending guard; got: {message}"
            );
            assert!(
                message.contains("cannot be lowered"),
                "Error message must explain why; got: {message}"
            );
        }
        other => panic!("Expected TemporalCompilationError, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Additional Phase 2 test: compound boolean conditions now lower to ComplexGuard
// ---------------------------------------------------------------------------
#[test]
fn test_complex_guard_lowering() {
    let src = r#"
module test_module {
    signal a: in bool;
    signal b: in bool;
    signal out: out bool;

    guard combo {
        when a && b
        for 5 cycles;
    }

    reflex r {
        on combo {
            out = a;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    assert_eq!(netlist.guards.len(), 1);
    match &netlist.guards[0] {
        CompiledGuard::Complex(cx) => {
            assert_eq!(cx.name, "combo");
            assert_eq!(cx.sub_guards.len(), 2);
            assert_eq!(cx.output_signal, "combo_out");
            let combo_text = format!("{:?}", cx.combination_logic);
            assert!(combo_text.contains("combo_sub"));
        }
        other => panic!("expected ComplexGuard, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// P2-REQ-014: Negated-signal condition lowers successfully to NegatedSignal
// ---------------------------------------------------------------------------
#[test]
fn test_negated_signal_condition_lowering() {
    // `ConditionKind::try_from_expr` must accept `!<signal>` and return
    // NegatedSignal — this is a supported 1-bit hardware inversion.
    let not_expr = Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Signal("pressure_ok".to_string())),
    };
    let ck = ConditionKind::try_from_expr(&not_expr).expect("Not(Signal) must lower successfully");

    match ck {
        ConditionKind::NegatedSignal(ref s) => {
            assert_eq!(s, "pressure_ok");
        }
        other => panic!("Expected NegatedSignal, got {other:?}"),
    }
    assert_eq!(ck.primary_signal(), "pressure_ok");
}

// ---------------------------------------------------------------------------
// P2-REQ-015: All six comparison operators lower successfully (Step 2.2)
// ---------------------------------------------------------------------------
#[test]
fn test_comparison_condition_lowering() {
    // `when sensor_val == 1` — equality
    let eq_expr = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("sensor_val".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
    };
    let ck =
        ConditionKind::try_from_expr(&eq_expr).expect("Signal == Literal must lower successfully");

    match &ck {
        ConditionKind::Comparison { signal, op, value } => {
            assert_eq!(signal, "sensor_val");
            assert_eq!(*op, BinaryOp::Eq);
            assert_eq!(*value, LiteralValue::Integer(1));
        }
        other => panic!("Expected Comparison, got {other:?}"),
    }
    assert_eq!(ck.primary_signal(), "sensor_val");

    // All six comparison operators must be accepted (Step 2.2 extension).
    // `<`, `<=`, `>`, `>=` lower to magnitude comparator circuits.
    for op in [BinaryOp::Eq, BinaryOp::Ne, BinaryOp::Lt, BinaryOp::Le, BinaryOp::Gt, BinaryOp::Ge] {
        let expr = Expr::Binary {
            op,
            left: Box::new(Expr::Signal("pressure".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(50))),
        };
        assert!(
            ConditionKind::try_from_expr(&expr).is_ok(),
            "operator {op:?} must lower to a magnitude comparator"
        );
    }

    // Logical operators (AND, OR) are still rejected — not single-condition forms.
    let and_expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("a".to_string())),
        right: Box::new(Expr::Signal("b".to_string())),
    };
    assert!(
        ConditionKind::try_from_expr(&and_expr).is_err(),
        "AND of two signals is not a supported single condition"
    );
}

// ---------------------------------------------------------------------------
// P2-REQ-016: condition_kind is stored in ShiftRegisterGuard IR
// ---------------------------------------------------------------------------
#[test]
fn test_condition_kind_stored_in_shift_register_ir() {
    let src = r#"
module test_module {
    signal enable: in bool;
    signal output_signal: out bool;

    guard sr_guard {
        when enable
        for 8 cycles;
    }

    reflex r {
        on sr_guard {
            output_signal = enable;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    match &netlist.guards[0] {
        CompiledGuard::ShiftRegister(sr) => {
            assert_eq!(
                sr.condition_kind,
                nasa_rust_project::temporal::low_level_ir::ConditionKind::SimpleSignal(
                    "enable".to_string()
                )
            );
            assert_eq!(sr.condition_kind.describe(), "when enable (high)");
        }
        other => panic!("Expected ShiftRegister, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// P2-REQ-017: condition_kind is stored in CounterGuard IR
// ---------------------------------------------------------------------------
#[test]
fn test_condition_kind_stored_in_counter_ir() {
    let src = r#"
module test_module {
    signal airway_pressure: in u16;
    signal clamp_valve: out bool;

    guard pressure_drop {
        when airway_pressure < 50
        for 1000 cycles;
    }

    reflex emergency {
        on pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    match &netlist.guards[0] {
        CompiledGuard::Counter(c) => {
            assert_eq!(c.input_signal, "airway_pressure");
            assert_eq!(c.target_count, 1000);
            // condition_kind must record the full comparison semantics
            match &c.condition_kind {
                nasa_rust_project::temporal::low_level_ir::ConditionKind::Comparison {
                    signal,
                    op,
                    value,
                } => {
                    assert_eq!(signal, "airway_pressure");
                    assert_eq!(*op, BinaryOp::Lt);
                    assert_eq!(*value, LiteralValue::Integer(50));
                }
                other => panic!("Expected Comparison condition_kind, got {other:?}"),
            }
            assert_eq!(c.condition_kind.describe(), "when airway_pressure < 50");
        }
        other => panic!("Expected Counter, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// P2-REQ-018: Full-pipeline compilation of neonatal_respirator.mirr succeeds
// ---------------------------------------------------------------------------
#[test]
fn test_neonatal_respirator_compiles() {
    // The canonical MIRR example — `airway_pressure < 50 for 1000 cycles`.
    // After Step 2.2, `<` is a supported magnitude comparator form.
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
        .expect("neonatal_respirator.mirr must compile successfully after Step 2.2");

    // 1000 cycles → Counter strategy
    assert_eq!(netlist.guards.len(), 1);
    match &netlist.guards[0] {
        CompiledGuard::Counter(c) => {
            assert_eq!(c.name, "sustained_pressure_drop");
            assert_eq!(c.target_count, 1000);
            assert_eq!(c.input_signal, "airway_pressure");
            // 11-bit counter: ceil(log2(1000))+1 = 10+1 = 11
            assert_eq!(c.counter_width(), 11);
        }
        other => panic!("Expected Counter for 1000-cycle guard, got {other:?}"),
    }
    // Resource stats must be populated
    assert_eq!(netlist.statistics.counters_used, 1);
    assert_eq!(netlist.statistics.max_delay_cycles, 1000);
}

// ---------------------------------------------------------------------------
// P2-REQ-006: Generated signal count matches expected hardware count
// ---------------------------------------------------------------------------
#[test]
fn test_generated_signal_counts() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard four_cycle_guard {
        when input_signal
        for 4 cycles;
    }

    reflex test_reflex {
        on four_cycle_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let netlist = TemporalGuardCompiler::new()
        .compile_temporal_guards(&program.module)
        .expect("compile failed");

    let sr_signals = netlist
        .signals
        .iter()
        .filter(|s| s.kind == GeneratedSignalKind::ShiftRegisterStage)
        .count();

    // 4-cycle guard → 4 shift-register stages
    assert_eq!(sr_signals, 4, "expected 4 SR stage signals");
    // At least 4 stages + 1 output signal
    assert!(netlist.signals.len() >= 5);
}

// ---------------------------------------------------------------------------
// Task 7 — Netlist Parity Gate
//
// Verifies that the Rust TemporalGuardCompiler produces output that matches
// the golden fixture at tests/fixtures/netlist/neonatal_respirator.json.
// This fixture is the canonical IR contract (Task 2) and the reference that
// compiler_mirr/temporal_lowering.mirr must produce identical output to when
// executed via the bootstrap runner (Task 8).
// ---------------------------------------------------------------------------
#[test]
fn test_netlist_fixture_parity_neonatal_respirator() {
    // ---- 1. Compile the canonical example with the Rust pipeline ----
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
        .expect("temporal compile failed");

    let json_envelope = TemporalNetlistJson::from_netlist(&netlist);
    let actual_json_str =
        serde_json::to_string_pretty(&json_envelope).expect("JSON serialization failed");
    let actual: serde_json::Value =
        serde_json::from_str(&actual_json_str).expect("re-parse failed");

    // ---- 2. Load the golden fixture ----
    let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/netlist/neonatal_respirator.json");
    let fixture_str = std::fs::read_to_string(&fixture_path).expect("fixture file missing");
    let expected: serde_json::Value =
        serde_json::from_str(&fixture_str).expect("fixture JSON invalid");

    // ---- 3. IR version ----
    assert_eq!(actual["ir_version"], expected["ir_version"], "ir_version mismatch");

    // ---- 4. Guard count and strategy ----
    let actual_guards = actual["guards"].as_array().expect("guards not array");
    let expected_guards = expected["guards"].as_array().expect("guards not array");
    assert_eq!(
        actual_guards.len(),
        expected_guards.len(),
        "guard count mismatch: actual {} vs fixture {}",
        actual_guards.len(),
        expected_guards.len()
    );

    // The single guard must be a Counter strategy.
    let actual_guard_obj = &actual_guards[0];
    let expected_guard_obj = &expected_guards[0];

    assert!(
        actual_guard_obj.get("Counter").is_some(),
        "actual guard[0] must be Counter variant; got: {actual_guard_obj}"
    );
    assert!(
        expected_guard_obj.get("Counter").is_some(),
        "fixture guard[0] must be Counter variant"
    );

    let actual_ctr = &actual_guard_obj["Counter"];
    let expected_ctr = &expected_guard_obj["Counter"];

    assert_eq!(actual_ctr["name"], expected_ctr["name"], "guard name mismatch");
    assert_eq!(actual_ctr["input_signal"], expected_ctr["input_signal"], "input_signal mismatch");
    assert_eq!(
        actual_ctr["output_signal"], expected_ctr["output_signal"],
        "output_signal mismatch"
    );
    assert_eq!(
        actual_ctr["counter_signal"], expected_ctr["counter_signal"],
        "counter_signal mismatch"
    );
    assert_eq!(
        actual_ctr["comparator_signal"], expected_ctr["comparator_signal"],
        "comparator_signal mismatch"
    );
    assert_eq!(actual_ctr["target_count"], expected_ctr["target_count"], "target_count mismatch");

    // condition_kind must be Comparison { signal, op, value }
    let actual_ck = &actual_ctr["condition_kind"];
    let expected_ck = &expected_ctr["condition_kind"];
    assert_eq!(
        actual_ck["Comparison"]["signal"], expected_ck["Comparison"]["signal"],
        "condition signal mismatch"
    );
    assert_eq!(
        actual_ck["Comparison"]["op"], expected_ck["Comparison"]["op"],
        "condition op mismatch"
    );
    assert_eq!(
        actual_ck["Comparison"]["value"], expected_ck["Comparison"]["value"],
        "condition value mismatch"
    );

    // ---- 5. Signal count and kinds ----
    let actual_signals = actual["signals"].as_array().expect("signals not array");
    let expected_signals = expected["signals"].as_array().expect("signals not array");
    assert_eq!(
        actual_signals.len(),
        expected_signals.len(),
        "signal count mismatch: actual {} vs fixture {}",
        actual_signals.len(),
        expected_signals.len()
    );

    // Verify each signal by name+kind+ty (order must match fixture)
    for (i, (a, e)) in actual_signals.iter().zip(expected_signals.iter()).enumerate() {
        assert_eq!(a["name"], e["name"], "signals[{i}] name mismatch");
        assert_eq!(a["ty"], e["ty"], "signals[{i}] ty mismatch");
        assert_eq!(a["kind"], e["kind"], "signals[{i}] kind mismatch");
    }

    // ---- 6. Statistics ----
    let actual_stats = &actual["statistics"];
    let expected_stats = &expected["statistics"];
    assert_eq!(
        actual_stats["shift_registers_used"], expected_stats["shift_registers_used"],
        "shift_registers_used mismatch"
    );
    assert_eq!(
        actual_stats["counters_used"], expected_stats["counters_used"],
        "counters_used mismatch"
    );
    assert_eq!(
        actual_stats["logic_gates_used"], expected_stats["logic_gates_used"],
        "logic_gates_used mismatch"
    );
    assert_eq!(
        actual_stats["max_delay_cycles"], expected_stats["max_delay_cycles"],
        "max_delay_cycles mismatch"
    );
    assert_eq!(
        actual_stats["total_signals"], expected_stats["total_signals"],
        "total_signals mismatch"
    );
}
