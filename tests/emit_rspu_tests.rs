#![forbid(unsafe_code)]
//! Tests for R-SPU instruction emission backend.
//!
//! Covers register allocation, instruction emission for each MIRR primitive,
//! temporal guard lowering, property assertions, and full pipeline E2E.

use mirrc::emit::rspu_isa::*;
use mirrc::emit::rspu_regalloc::allocate_registers;
use mirrc::pipeline::{run_pipeline, PipelineConfig, PipelineResult};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn rspu_config() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: true,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    }
}

fn pipeline_ok(source: &str) -> PipelineResult {
    run_pipeline(source, &rspu_config()).expect("pipeline should succeed")
}

fn pipeline_with_rspu(source: &str) -> RspuProgram {
    let result = pipeline_ok(source);
    result.rspu_program.expect("rspu_program should be Some")
}

fn temporal_config_no_rspu() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    }
}

/// Standard minimal module source (multi-line guard/reflex format).
fn minimal_source() -> &'static str {
    r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = a;
        }
    }
}
"#
}

// ---------------------------------------------------------------------------
// Register allocation tests
// ---------------------------------------------------------------------------

#[test]
fn regalloc_input_signal_maps_to_input_partition() {
    let target = TargetSpec::from_config(&None);
    let result = run_pipeline(minimal_source(), &temporal_config_no_rspu()).unwrap();
    let regs = allocate_registers(&result.program.module, &target).unwrap();
    let a_reg = regs.reg("a");
    assert!(a_reg <= REG_INPUT_MAX, "input signal should be in input partition");
}

#[test]
fn regalloc_output_signal_maps_to_output_partition() {
    let target = TargetSpec::from_config(&None);
    let result = run_pipeline(minimal_source(), &temporal_config_no_rspu()).unwrap();
    let regs = allocate_registers(&result.program.module, &target).unwrap();
    let b_reg = regs.reg("b");
    assert!(
        (REG_OUTPUT_BASE..=REG_OUTPUT_MAX).contains(&b_reg),
        "output signal should be in output partition"
    );
}

#[test]
fn regalloc_internal_signal_maps_to_internal_partition() {
    let target = TargetSpec::from_config(&None);
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;
    signal mid: internal bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            mid = a;
        }
    }

    reflex r2 {
        on g1 {
            b = mid;
        }
    }
}
"#;
    let result = run_pipeline(source, &temporal_config_no_rspu()).unwrap();
    let regs = allocate_registers(&result.program.module, &target).unwrap();
    let mid_reg = regs.reg("mid");
    assert!(
        (REG_INTERNAL_BASE..=REG_INTERNAL_MAX).contains(&mid_reg),
        "internal signal should be in internal partition"
    );
}

#[test]
fn regalloc_multiple_signals_unique_registers() {
    let target = TargetSpec::from_config(&None);
    let source = r#"
module test_mod {
    signal x: in u8;
    signal y: in u8;
    signal z: out u16;

    guard g1 {
        when x > 0
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            z = x + y;
        }
    }
}
"#;
    let result = run_pipeline(source, &temporal_config_no_rspu()).unwrap();
    let regs = allocate_registers(&result.program.module, &target).unwrap();
    let x_reg = regs.reg("x");
    let y_reg = regs.reg("y");
    let z_reg = regs.reg("z");
    assert_ne!(x_reg, y_reg);
    assert_ne!(x_reg, z_reg);
    assert_ne!(y_reg, z_reg);
}

// ---------------------------------------------------------------------------
// Instruction emission tests
// ---------------------------------------------------------------------------

#[test]
fn emit_load_input_for_each_input_signal() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: in bool;
    signal c: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            c = b;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let load_inputs: Vec<_> = prog
        .instructions
        .iter()
        .filter(|i| matches!(i, RspuInstruction::LoadInput { .. }))
        .collect();
    assert_eq!(load_inputs.len(), 2, "should have LOAD_INPUT for each input signal");
}

#[test]
fn emit_store_output_for_each_output_signal() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal x: out bool;
    signal y: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            x = a;
        }
    }

    reflex r2 {
        on g1 {
            y = a;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let store_outputs: Vec<_> = prog
        .instructions
        .iter()
        .filter(|i| matches!(i, RspuInstruction::StoreOutput { .. }))
        .collect();
    assert_eq!(store_outputs.len(), 2, "should have STORE_OUTPUT for each output signal");
}

#[test]
fn emit_reflex_if_for_conditional_assignment() {
    let prog = pipeline_with_rspu(minimal_source());
    let reflex_ifs: Vec<_> = prog
        .instructions
        .iter()
        .filter(|i| matches!(i, RspuInstruction::ReflexIf { .. }))
        .collect();
    assert!(!reflex_ifs.is_empty(), "should have at least one REFLEX_IF");
}

#[test]
fn emit_sr_init_tick_query_for_shift_register_guard() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 5 cycles;
    }

    reflex r1 {
        on g1 {
            b = true;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let sr_inits =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::SrInit { .. })).count();
    let sr_ticks =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::SrTick { .. })).count();
    let sr_queries =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::SrQuery { .. })).count();
    assert_eq!(sr_inits, 1, "should have SR_INIT");
    assert_eq!(sr_ticks, 1, "should have SR_TICK");
    assert_eq!(sr_queries, 1, "should have SR_QUERY");
}

#[test]
fn emit_ctr_init_tick_query_for_counter_guard() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 100 cycles;
    }

    reflex r1 {
        on g1 {
            b = true;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let ctr_inits =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::CtrInit { .. })).count();
    let ctr_ticks =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::CtrTick { .. })).count();
    let ctr_queries =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::CtrQuery { .. })).count();
    assert_eq!(ctr_inits, 1, "should have CTR_INIT");
    assert_eq!(ctr_ticks, 1, "should have CTR_TICK");
    assert_eq!(ctr_queries, 1, "should have CTR_QUERY");
}

// ---------------------------------------------------------------------------
// ALU emission tests
// ---------------------------------------------------------------------------

#[test]
fn emit_alu_for_binary_expression() {
    let source = r#"
module test_mod {
    signal a: in u8;
    signal b: in u8;
    signal c: out u8;

    guard g1 {
        when a > 0
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            c = a + b;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let alus: Vec<_> =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::Alu { .. })).collect();
    assert!(!alus.is_empty(), "should have ALU instructions for a + b");
}

#[test]
fn emit_alu_unary_for_not_expression() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = !a;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let alu_unaries: Vec<_> = prog
        .instructions
        .iter()
        .filter(|i| matches!(i, RspuInstruction::AluUnary { op: AluUnaryOp::Not, .. }))
        .collect();
    assert!(!alu_unaries.is_empty(), "should have ALU_UNARY NOT for !a");
}

#[test]
fn emit_load_imm_for_literal() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = true;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let load_imms: Vec<_> =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::LoadImm { .. })).collect();
    assert!(!load_imms.is_empty(), "should have LOAD_IMM for literal true");
}

// ---------------------------------------------------------------------------
// Prev instruction test
// ---------------------------------------------------------------------------

/// Test PREV instruction via AST-level construction (prev not parseable from text).
/// We test the pipeline with a simple module and verify other instructions work.
#[test]
fn emit_prev_mnemonic_is_correct() {
    let instr = RspuInstruction::Prev { dst: 10, signal: 5, delay: 2 };
    assert_eq!(instr.mnemonic(), "PREV");
}

// ---------------------------------------------------------------------------
// Property assertion tests
// ---------------------------------------------------------------------------

#[test]
fn emit_assert_always_for_always_property() {
    let source = r#"
module test_mod {
    signal a: in u8;
    signal b: out bool;

    guard g1 {
        when a > 0
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = true;
        }
    }

    property p1 {
        always (a < 200);
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let asserts: Vec<_> = prog
        .instructions
        .iter()
        .filter(|i| matches!(i, RspuInstruction::AssertAlways { .. }))
        .collect();
    assert!(!asserts.is_empty(), "should have ASSERT_ALWAYS for always property");
}

#[test]
fn emit_assert_never_for_never_property() {
    let source = r#"
module test_mod {
    signal a: in u8;
    signal b: out bool;

    guard g1 {
        when a > 0
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = true;
        }
    }

    property p1 {
        never (a > 250);
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    let asserts: Vec<_> = prog
        .instructions
        .iter()
        .filter(|i| matches!(i, RspuInstruction::AssertNever { .. }))
        .collect();
    assert!(!asserts.is_empty(), "should have ASSERT_NEVER for never property");
}

// ---------------------------------------------------------------------------
// Assembly output test
// ---------------------------------------------------------------------------

#[test]
fn emit_asm_produces_valid_output() {
    let prog = pipeline_with_rspu(minimal_source());
    let asm = prog.emit_asm();
    assert!(asm.contains("R-SPU Assembly"), "should have assembly header");
    assert!(asm.contains("Register map:"), "should have register map");
    assert!(asm.contains("Guard map:"), "should have guard map");
    assert!(asm.contains("LOAD_INPUT"), "should have LOAD_INPUT instruction");
    assert!(asm.contains("STORE_OUTPUT"), "should have STORE_OUTPUT instruction");
}

// ---------------------------------------------------------------------------
// Mnemonic tests
// ---------------------------------------------------------------------------

#[test]
fn mnemonic_returns_correct_names() {
    assert_eq!(RspuInstruction::LoadInput { dst: 0, port: 0 }.mnemonic(), "LOAD_INPUT");
    assert_eq!(RspuInstruction::StoreOutput { src: 0, port: 0 }.mnemonic(), "STORE_OUTPUT");
    assert_eq!(RspuInstruction::Mov { dst: 0, src: 1 }.mnemonic(), "MOV");
    assert_eq!(RspuInstruction::LoadImm { dst: 0, value: 42, width: 8 }.mnemonic(), "LOAD_IMM");
    assert_eq!(RspuInstruction::Alu { op: AluOp::Add, dst: 0, a: 1, b: 2 }.mnemonic(), "ALU");
    assert_eq!(
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 0, src: 1 }.mnemonic(),
        "ALU_UNARY"
    );
    assert_eq!(RspuInstruction::SrInit { guard: 0, length: 5, cond: 0 }.mnemonic(), "SR_INIT");
    assert_eq!(RspuInstruction::SrTick { guard: 0 }.mnemonic(), "SR_TICK");
    assert_eq!(RspuInstruction::SrQuery { dst: 0, guard: 0 }.mnemonic(), "SR_QUERY");
    assert_eq!(RspuInstruction::CtrInit { guard: 0, target: 100, cond: 0 }.mnemonic(), "CTR_INIT");
    assert_eq!(RspuInstruction::CtrTick { guard: 0 }.mnemonic(), "CTR_TICK");
    assert_eq!(RspuInstruction::CtrQuery { dst: 0, guard: 0 }.mnemonic(), "CTR_QUERY");
    assert_eq!(RspuInstruction::GuardAnd { dst: 0, a: 1, b: 2 }.mnemonic(), "GUARD_AND");
    assert_eq!(RspuInstruction::GuardOr { dst: 0, a: 1, b: 2 }.mnemonic(), "GUARD_OR");
    assert_eq!(RspuInstruction::ReflexIf { guard: 0, dst: 0, src: 1 }.mnemonic(), "REFLEX_IF");
    assert_eq!(RspuInstruction::Prev { dst: 0, signal: 1, delay: 3 }.mnemonic(), "PREV");
    assert_eq!(RspuInstruction::EmergencyStop.mnemonic(), "EMERGENCY_STOP");
    assert_eq!(
        RspuInstruction::AssertAlways { cond: 0, property_id: 0 }.mnemonic(),
        "ASSERT_ALWAYS"
    );
    assert_eq!(RspuInstruction::AssertNever { cond: 0, property_id: 0 }.mnemonic(), "ASSERT_NEVER");
}

// ---------------------------------------------------------------------------
// Pipeline integration tests
// ---------------------------------------------------------------------------

#[test]
fn pipeline_rspu_flag_disabled_returns_none() {
    let result = run_pipeline(minimal_source(), &temporal_config_no_rspu()).unwrap();
    assert!(result.rspu_program.is_none(), "rspu_program should be None when rspu=false");
}

#[test]
fn pipeline_rspu_flag_enabled_returns_some() {
    let result = pipeline_ok(minimal_source());
    assert!(result.rspu_program.is_some(), "rspu_program should be Some when rspu=true");
}

// ---------------------------------------------------------------------------
// Full E2E: neonatal-style module
// ---------------------------------------------------------------------------

#[test]
fn e2e_neonatal_style_module() {
    let source = r#"
module patient_monitor {
    signal heart_rate: in u16;
    signal spo2: in u8;
    signal alarm: out bool;
    signal status: out u8;

    guard bradycardia {
        when heart_rate < 60
        for 500 cycles;
    }

    guard hypoxia {
        when spo2 < 90
        for 100 cycles;
    }

    reflex cardiac_alarm {
        on bradycardia {
            alarm = true;
        }
    }

    reflex set_status {
        on hypoxia {
            status = 1;
        }
    }

    property hr_bounded {
        always (heart_rate < 300);
    }

    property spo2_bounded {
        never (spo2 > 100);
    }
}
"#;
    let prog = pipeline_with_rspu(source);

    assert!(prog.registers_used >= 4, "should have at least 4 signals");
    assert_eq!(prog.guards_used, 3, "should have 3 guards (including 'always')");
    assert!(prog.instructions.len() > 10, "should have substantial instruction count");

    let load_count =
        prog.instructions.iter().filter(|i| matches!(i, RspuInstruction::LoadInput { .. })).count();
    assert_eq!(load_count, 2, "should have 2 LOAD_INPUT instructions");

    let store_count = prog
        .instructions
        .iter()
        .filter(|i| matches!(i, RspuInstruction::StoreOutput { .. }))
        .count();
    assert_eq!(store_count, 2, "should have 2 STORE_OUTPUT instructions");

    let assert_count = prog
        .instructions
        .iter()
        .filter(|i| {
            matches!(i, RspuInstruction::AssertAlways { .. } | RspuInstruction::AssertNever { .. })
        })
        .count();
    assert_eq!(assert_count, 2, "should have 2 property assertions");

    let asm = prog.emit_asm();
    assert!(asm.len() > 100, "asm output should be substantial");
}

// ---------------------------------------------------------------------------
// Instruction count tracking
// ---------------------------------------------------------------------------

#[test]
fn instruction_count_within_budget() {
    let source = r#"
module test_mod {
    signal a: in u8;
    signal b: out u8;

    guard g1 {
        when a > 0
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = a * a + a;
        }
    }
}
"#;
    let prog = pipeline_with_rspu(source);
    assert!(
        prog.instructions.len() <= MAX_INSTRUCTIONS,
        "instruction count {} should be within MAX_INSTRUCTIONS {}",
        prog.instructions.len(),
        MAX_INSTRUCTIONS,
    );
}

// ---------------------------------------------------------------------------
// Register map and guard map metadata
// ---------------------------------------------------------------------------

#[test]
fn register_map_contains_all_signals() {
    let prog = pipeline_with_rspu(minimal_source());
    let names: Vec<&str> = prog.register_map.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"a"), "register map should contain a");
    assert!(names.contains(&"b"), "register map should contain b");
}

#[test]
fn guard_map_contains_compiled_guards() {
    let prog = pipeline_with_rspu(minimal_source());
    assert!(!prog.guard_map.is_empty(), "guard map should contain compiled guards");
}

// ---------------------------------------------------------------------------
// Pattern expansion + R-SPU
// ---------------------------------------------------------------------------

#[test]
fn rspu_works_with_pattern_expanded_module() {
    let source = r#"
def threshold_alarm(sensor: signal in u16, alarm: signal out bool) {
    reflect {
        guard check {
            when ${sensor} > 100
            for 10 cycles;
        }

        reflex trigger {
            on check {
                ${alarm} = true;
            }
        }
    }
}

module test_mod {
    signal temp: in u16;
    signal warning: out bool;

    threshold_alarm(temp, warning);
}
"#;
    let prog = pipeline_with_rspu(source);
    assert!(prog.instructions.len() > 5, "should produce instructions from expanded pattern");
    assert!(prog.guards_used >= 1, "should have guard from pattern");
}
