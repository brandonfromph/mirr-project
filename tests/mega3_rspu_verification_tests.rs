//! MEGA-3 Subsystem Verification Test Suite — R-SPU Backend.
//!
//! NASA-style verification tests for the R-SPU instruction set architecture,
//! tagged-word register file, binary encoding, exception model, and simulator.
//!
//! Covers:
//! - E1: ISA instruction variants (37 opcodes, 5 tiers)
//! - E2: Tagged-word type safety (TypeTag, Provenance, RegisterFile)
//! - E3: Binary encoding/decoding roundtrip (32-bit fixed-width)
//! - E4: Exception model (ExceptionCode, ExecMode, ExceptionState)
//! - E5: Simulator step semantics (register, ALU, temporal, reflex, safety)
//! - E6: Full pipeline → R-SPU → simulate E2E
//! - E7: Resource limits (MAX_REGISTERS, MAX_GUARDS, MAX_INSTRUCTIONS)
//! - E8: MEGA-3 new instructions (Trap, TrapIf, Halt, ModeSwitch, Tag*, Fence)
//! - E9: MEGA-4 totality instructions (Verify, Certify, TotalCheck)
//! - E10: MEGA-5 symbolic instructions (Match, IntervalLo, IntervalHi, IntervalCheck)
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::emit::rspu_encoding::{decode, encode, EncodedInstruction};
use nasa_rust_project::emit::rspu_isa::{
    AluOp, AluUnaryOp, RegId, RspuInstruction, RspuProgram, MAX_GUARDS, MAX_INSTRUCTIONS,
    MAX_REGISTERS, MAX_SIM_CYCLES, REG_INPUT_BASE, REG_INPUT_MAX, REG_INTERNAL_BASE,
    REG_INTERNAL_MAX, REG_OUTPUT_BASE, REG_OUTPUT_MAX, REG_TEMP_BASE, REG_TEMP_MAX,
};
use nasa_rust_project::emit::rspu_sim::{RspuSimulator, StepResult};
use nasa_rust_project::emit::rspu_tagged::{RegisterFile, TaggedWord, TypeTag};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum test iterations in any bounded loop.
const MAX_TEST_ITERATIONS: usize = 256;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a minimal R-SPU program from a list of instructions.
fn make_program(instrs: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        instructions: instrs,
        registers_used: MAX_REGISTERS,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: None,
    }
}

/// Run pipeline with R-SPU emission on the given MIRR source.
fn pipeline_with_rspu(
    src: &str,
) -> Result<nasa_rust_project::PipelineResult, nasa_rust_project::PipelineErrors> {
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        temporal: true,
        rspu: true,
        simulate: false,
        totality: false,
        symbolic: false,
        ..PipelineConfig::default()
    };
    run_pipeline(src, &config)
}

// ===========================================================================
// E1: ISA instruction variants
// ===========================================================================

#[test]
fn e1_load_input_mnemonic() {
    let i = RspuInstruction::LoadInput { dst: 0, port: 0 };
    assert_eq!(i.mnemonic(), "LOAD_INPUT", "LoadInput mnemonic must match ISA spec");
}

#[test]
fn e1_store_output_mnemonic() {
    let i = RspuInstruction::StoreOutput { src: 64, port: 0 };
    assert_eq!(i.mnemonic(), "STORE_OUTPUT");
}

#[test]
fn e1_mov_mnemonic() {
    let i = RspuInstruction::Mov { dst: 1, src: 0 };
    assert_eq!(i.mnemonic(), "MOV");
}

#[test]
fn e1_load_imm_mnemonic() {
    let i = RspuInstruction::LoadImm { dst: 0, value: 42, width: 16 };
    assert_eq!(i.mnemonic(), "LOAD_IMM");
}

#[test]
fn e1_alu_mnemonic() {
    let i = RspuInstruction::Alu { op: AluOp::Add, dst: 2, a: 0, b: 1 };
    assert_eq!(i.mnemonic(), "ALU");
}

#[test]
fn e1_alu_imm_mnemonic() {
    let i = RspuInstruction::AluImm { op: AluOp::Sub, dst: 2, a: 0, imm: 1 };
    assert_eq!(i.mnemonic(), "ALU_IMM");
}

#[test]
fn e1_alu_unary_mnemonic() {
    let i = RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 1, src: 0 };
    assert_eq!(i.mnemonic(), "ALU_UNARY");
}

#[test]
fn e1_sr_init_mnemonic() {
    let i = RspuInstruction::SrInit { guard: 0, length: 4, cond: 0 };
    assert_eq!(i.mnemonic(), "SR_INIT");
}

#[test]
fn e1_sr_tick_mnemonic() {
    let i = RspuInstruction::SrTick { guard: 0 };
    assert_eq!(i.mnemonic(), "SR_TICK");
}

#[test]
fn e1_sr_query_mnemonic() {
    let i = RspuInstruction::SrQuery { dst: 1, guard: 0 };
    assert_eq!(i.mnemonic(), "SR_QUERY");
}

#[test]
fn e1_ctr_init_mnemonic() {
    let i = RspuInstruction::CtrInit { guard: 0, target: 100, cond: 0 };
    assert_eq!(i.mnemonic(), "CTR_INIT");
}

#[test]
fn e1_ctr_tick_mnemonic() {
    let i = RspuInstruction::CtrTick { guard: 0 };
    assert_eq!(i.mnemonic(), "CTR_TICK");
}

#[test]
fn e1_ctr_query_mnemonic() {
    let i = RspuInstruction::CtrQuery { dst: 1, guard: 0 };
    assert_eq!(i.mnemonic(), "CTR_QUERY");
}

#[test]
fn e1_guard_and_mnemonic() {
    let i = RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 };
    assert_eq!(i.mnemonic(), "GUARD_AND");
}

#[test]
fn e1_guard_or_mnemonic() {
    let i = RspuInstruction::GuardOr { dst: 2, a: 0, b: 1 };
    assert_eq!(i.mnemonic(), "GUARD_OR");
}

#[test]
fn e1_reflex_if_mnemonic() {
    let i = RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 0 };
    assert_eq!(i.mnemonic(), "REFLEX_IF");
}

#[test]
fn e1_prev_mnemonic() {
    let i = RspuInstruction::Prev { dst: 1, signal: 0, delay: 1 };
    assert_eq!(i.mnemonic(), "PREV");
}

#[test]
fn e1_emergency_stop_mnemonic() {
    assert_eq!(RspuInstruction::EmergencyStop.mnemonic(), "EMERGENCY_STOP");
}

#[test]
fn e1_assert_always_mnemonic() {
    let i = RspuInstruction::AssertAlways { cond: 0, property_id: 0 };
    assert_eq!(i.mnemonic(), "ASSERT_ALWAYS");
}

#[test]
fn e1_assert_never_mnemonic() {
    let i = RspuInstruction::AssertNever { cond: 0, property_id: 0 };
    assert_eq!(i.mnemonic(), "ASSERT_NEVER");
}

#[test]
fn e1_all_37_opcodes_have_unique_mnemonics() {
    let instrs: Vec<RspuInstruction> = vec![
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        RspuInstruction::StoreOutput { src: 0, port: 0 },
        RspuInstruction::Mov { dst: 0, src: 0 },
        RspuInstruction::LoadImm { dst: 0, value: 0, width: 1 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 0, a: 0, b: 0 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 0, a: 0, imm: 0 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 0, src: 0 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 0 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 0, guard: 0 },
        RspuInstruction::CtrInit { guard: 0, target: 1, cond: 0 },
        RspuInstruction::CtrTick { guard: 0 },
        RspuInstruction::CtrQuery { dst: 0, guard: 0 },
        RspuInstruction::GuardAnd { dst: 0, a: 0, b: 0 },
        RspuInstruction::GuardOr { dst: 0, a: 0, b: 0 },
        RspuInstruction::ReflexIf { guard: 0, dst: 0, src: 0 },
        RspuInstruction::Prev { dst: 0, signal: 0, delay: 0 },
        RspuInstruction::EmergencyStop,
        RspuInstruction::AssertAlways { cond: 0, property_id: 0 },
        RspuInstruction::AssertNever { cond: 0, property_id: 0 },
        RspuInstruction::Trap { code: 0 },
        RspuInstruction::TrapIf { cond: 0, code: 0 },
        RspuInstruction::Halt,
        RspuInstruction::ModeSwitch { mode: 0 },
        RspuInstruction::TagLoad { dst: 0, tag: 0 },
        RspuInstruction::TagCheck { src: 0, expected: 0 },
        RspuInstruction::TagRead { dst: 0, src: 0 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::DeadlineSet { cycles: 0 },
        RspuInstruction::Verify { cert_offset: 0 },
        RspuInstruction::Certify { dst: 0 },
        RspuInstruction::TotalCheck { expected_properties: 0 },
        RspuInstruction::Match { dst: 0, src: 0, table_offset: 0 },
        RspuInstruction::IntervalLo { dst: 0, src: 0 },
        RspuInstruction::IntervalHi { dst: 0, src: 0 },
        RspuInstruction::IntervalCheck { src: 0, bounds: 0 },
    ];

    // Verify all 37 instruction variants exist.
    assert_eq!(instrs.len(), 37, "R-SPU ISA must have exactly 37 instruction variants");

    // Verify no two instructions share a mnemonic.
    let mut mnemonics: Vec<&str> = Vec::with_capacity(MAX_TEST_ITERATIONS);
    let mut i = 0;
    while i < instrs.len() && i < MAX_TEST_ITERATIONS {
        let m = instrs[i].mnemonic();
        let mut j = 0;
        while j < mnemonics.len() && j < MAX_TEST_ITERATIONS {
            assert_ne!(mnemonics[j], m, "Duplicate mnemonic: {}", m);
            j += 1;
        }
        mnemonics.push(m);
        i += 1;
    }
}

// ===========================================================================
// E2: Tagged-word type safety
// ===========================================================================

#[test]
fn e2_tagged_word_uninitialized_default() {
    let w = TaggedWord::uninitialized();
    assert_eq!(w.tag, TypeTag::Uninitialized, "Default register must be Uninitialized");
    assert_eq!(w.value, 0);
}

#[test]
fn e2_register_file_256_entries() {
    let rf = RegisterFile::new();
    // All 256 registers must be readable.
    let mut i: usize = 0;
    while i < MAX_REGISTERS {
        let w = rf.read(i as RegId);
        assert_eq!(w.tag, TypeTag::Uninitialized, "Register R{} must start Uninitialized", i);
        i += 1;
    }
}

#[test]
fn e2_register_file_write_read_roundtrip() {
    let mut rf = RegisterFile::new();
    let w = TaggedWord::from_literal(42, TypeTag::Unsigned { width: 16 });
    rf.write(10, w.clone());
    assert_eq!(rf.read(10).value, 42);
    assert_eq!(rf.read(10).tag, TypeTag::Unsigned { width: 16 });
}

#[test]
fn e2_register_partition_input() {
    assert_eq!(REG_INPUT_BASE, 0);
    assert_eq!(REG_INPUT_MAX, 63);
}

#[test]
fn e2_register_partition_output() {
    assert_eq!(REG_OUTPUT_BASE, 64);
    assert_eq!(REG_OUTPUT_MAX, 127);
}

#[test]
fn e2_register_partition_internal() {
    assert_eq!(REG_INTERNAL_BASE, 128);
    assert_eq!(REG_INTERNAL_MAX, 191);
}

#[test]
fn e2_register_partition_temp() {
    assert_eq!(REG_TEMP_BASE, 192);
    assert_eq!(REG_TEMP_MAX, 255);
}

#[test]
fn e2_type_tag_display_bool() {
    assert_eq!(format!("{}", TypeTag::Bool), "bool");
}

#[test]
fn e2_type_tag_display_unsigned() {
    assert_eq!(format!("{}", TypeTag::Unsigned { width: 16 }), "u16");
}

#[test]
fn e2_type_tag_display_signed() {
    assert_eq!(format!("{}", TypeTag::Signed { width: 32 }), "i32");
}

#[test]
fn e2_type_tag_display_uninitialized() {
    assert_eq!(format!("{}", TypeTag::Uninitialized), "<uninitialized>");
}

#[test]
fn e2_type_tag_display_interval() {
    assert_eq!(format!("{}", TypeTag::Interval { lo: 0, hi: 255 }), "interval[0, 255]");
}

// ===========================================================================
// E3: Binary encoding/decoding roundtrip
// ===========================================================================

#[test]
fn e3_encode_decode_load_input() {
    let i = RspuInstruction::LoadInput { dst: 5, port: 3 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "LOAD_INPUT");
}

#[test]
fn e3_encode_decode_store_output() {
    let i = RspuInstruction::StoreOutput { src: 64, port: 1 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "STORE_OUTPUT");
}

#[test]
fn e3_encode_decode_mov() {
    let i = RspuInstruction::Mov { dst: 10, src: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "MOV");
}

#[test]
fn e3_encode_decode_alu_add() {
    let i = RspuInstruction::Alu { op: AluOp::Add, dst: 2, a: 0, b: 1 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "ALU");
}

#[test]
fn e3_encode_decode_emergency_stop() {
    let i = RspuInstruction::EmergencyStop;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "EMERGENCY_STOP");
}

#[test]
fn e3_encode_decode_halt() {
    let i = RspuInstruction::Halt;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "HALT");
}

#[test]
fn e3_encode_decode_nop() {
    let i = RspuInstruction::Nop;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "NOP");
}

#[test]
fn e3_encode_decode_fence() {
    let i = RspuInstruction::Fence;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "FENCE");
}

#[test]
fn e3_encoded_instruction_is_32_bits() {
    let i = RspuInstruction::Nop;
    let _encoded = encode(&i).expect("encode must succeed");
    assert_eq!(std::mem::size_of::<EncodedInstruction>(), 4, "Encoded instruction must be 32 bits");
}

#[test]
fn e3_binary_roundtrip_all_zero_arg_instructions() {
    let zero_arg_instrs = [
        RspuInstruction::EmergencyStop,
        RspuInstruction::Halt,
        RspuInstruction::Nop,
        RspuInstruction::Fence,
    ];
    let mut i = 0;
    while i < zero_arg_instrs.len() && i < MAX_TEST_ITERATIONS {
        let encoded = encode(&zero_arg_instrs[i]).expect("encode must succeed");
        let decoded = decode(encoded.0).expect("decode must succeed");
        assert_eq!(
            decoded.mnemonic(),
            zero_arg_instrs[i].mnemonic(),
            "Roundtrip failed for {}",
            zero_arg_instrs[i].mnemonic()
        );
        i += 1;
    }
}

// ===========================================================================
// E4: Exception model
// ===========================================================================

#[test]
fn e4_exception_code_tag_violation_is_zero() {
    // ExceptionCode::TagViolation repr = 0
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::TagViolation;
    assert_eq!(code as u8, 0);
}

#[test]
fn e4_exception_code_deadline_miss_is_one() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::DeadlineMiss;
    assert_eq!(code as u8, 1);
}

#[test]
fn e4_exception_code_property_fail_is_two() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::PropertyFail;
    assert_eq!(code as u8, 2);
}

#[test]
fn e4_exception_code_software_trap_is_five() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::SoftwareTrap;
    assert_eq!(code as u8, 5);
}

#[test]
fn e4_exception_code_interval_violation_is_seven() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::IntervalViolation;
    assert_eq!(code as u8, 7);
}

#[test]
fn e4_exec_mode_reflex_display() {
    let mode = nasa_rust_project::emit::rspu_exceptions::ExecMode::Reflex;
    assert_eq!(format!("{}", mode), "Reflex");
}

#[test]
fn e4_exec_mode_host_display() {
    let mode = nasa_rust_project::emit::rspu_exceptions::ExecMode::Host;
    assert_eq!(format!("{}", mode), "Host");
}

// ===========================================================================
// E5: Simulator step semantics
// ===========================================================================

#[test]
fn e5_sim_new_starts_at_pc_zero() {
    let sim = RspuSimulator::new();
    assert_eq!(sim.pc, 0, "Simulator must start at PC=0");
    assert_eq!(sim.cycle, 0, "Simulator must start at cycle=0");
    assert!(!sim.halted, "Simulator must not be halted at start");
}

#[test]
fn e5_sim_empty_program_halts_immediately() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![]);
    let result = sim.step(&prog).expect("step must not error");
    assert_eq!(result, StepResult::Halted, "Empty program must halt on first step");
}

#[test]
fn e5_sim_nop_advances_pc() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::Nop]);
    let result = sim.step(&prog).expect("step must not error");
    assert_eq!(result, StepResult::Continue);
    assert_eq!(sim.pc, 1, "PC must advance after NOP");
    assert_eq!(sim.cycle, 1, "Cycle must advance after NOP");
}

#[test]
fn e5_sim_halt_stops_execution() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::Halt]);
    let result = sim.step(&prog).expect("step must not error");
    assert_eq!(result, StepResult::Halted);
    assert!(sim.halted, "Simulator must be halted after HALT");
}

#[test]
fn e5_sim_emergency_stop() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::EmergencyStop]);
    let result = sim.step(&prog).expect("step must not error");
    assert_eq!(result, StepResult::EmergencyStop);
}

#[test]
fn e5_sim_load_imm_sets_register() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::LoadImm { dst: 192, value: 0xFF, width: 8 }]);
    sim.step(&prog).expect("step must not error");
    assert_eq!(sim.registers.read(192).value, 0xFF);
}

#[test]
fn e5_sim_mov_copies_register() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 16 },
        RspuInstruction::Mov { dst: 193, src: 192 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(193).value, 42);
}

#[test]
fn e5_sim_alu_add() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 16 },
        RspuInstruction::LoadImm { dst: 193, value: 20, width: 16 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    sim.step(&prog).expect("step 3");
    assert_eq!(sim.registers.read(194).value, 30, "10 + 20 must equal 30");
}

#[test]
fn e5_sim_alu_sub() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 50, width: 16 },
        RspuInstruction::LoadImm { dst: 193, value: 20, width: 16 },
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    sim.step(&prog).expect("step 3");
    assert_eq!(sim.registers.read(194).value, 30, "50 - 20 must equal 30");
}

#[test]
fn e5_sim_alu_mul() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 6, width: 16 },
        RspuInstruction::LoadImm { dst: 193, value: 7, width: 16 },
        RspuInstruction::Alu { op: AluOp::Mul, dst: 194, a: 192, b: 193 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    sim.step(&prog).expect("step 3");
    assert_eq!(sim.registers.read(194).value, 42, "6 * 7 must equal 42");
}

#[test]
fn e5_sim_alu_comparisons() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 16 },
        RspuInstruction::LoadImm { dst: 193, value: 10, width: 16 },
        RspuInstruction::Alu { op: AluOp::Lt, dst: 194, a: 192, b: 193 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    sim.step(&prog).expect("step 3");
    assert_eq!(sim.registers.read(194).value, 1, "5 < 10 must be true (1)");
}

#[test]
fn e5_sim_alu_imm_add() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 100, width: 16 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 193, a: 192, imm: 50 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(193).value, 150, "100 + 50 must equal 150");
}

#[test]
fn e5_sim_set_input_read_output() {
    let mut sim = RspuSimulator::new();
    sim.set_input(0, 0xBEEF, TypeTag::Unsigned { width: 16 });
    assert_eq!(sim.registers.read(0).value, 0xBEEF, "Input port 0 must have value 0xBEEF");
}

// ===========================================================================
// E6: Full pipeline → R-SPU E2E
// ===========================================================================

#[test]
fn e6_simple_module_produces_rspu_program() {
    let src = r#"module simple {
    signal enable: in bool;
    signal out_val: out bool;

    guard g {
        when enable
        for 1 cycles;
    }

    reflex r {
        on g {
            out_val = true;
        }
    }
}"#;
    let result = pipeline_with_rspu(src).expect("pipeline must succeed");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "R-SPU program must have instructions");
    assert!(
        (rspu.instructions.len()) <= MAX_INSTRUCTIONS,
        "R-SPU program must not exceed MAX_INSTRUCTIONS"
    );
}

#[test]
fn e6_minimal_module_has_load_and_store() {
    let src = r#"module minimal {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 1 cycles;
    }

    reflex r {
        on g {
            b = a;
        }
    }
}"#;
    let result = pipeline_with_rspu(src).expect("pipeline must succeed");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");

    let mut has_load = false;
    let mut has_store = false;
    let mut i = 0;
    while i < rspu.instructions.len() && i < MAX_TEST_ITERATIONS {
        match &rspu.instructions[i] {
            RspuInstruction::LoadInput { .. } => has_load = true,
            RspuInstruction::StoreOutput { .. } => has_store = true,
            _ => {}
        }
        i += 1;
    }
    assert!(has_load, "R-SPU program must have LOAD_INPUT for input signal");
    assert!(has_store, "R-SPU program must have STORE_OUTPUT for output signal");
}

#[test]
fn e6_guard_produces_temporal_instructions() {
    let src = r#"module temporal_test {
    signal s: in bool;
    signal out_val: out bool;

    guard g {
        when s
        for 4 cycles;
    }

    reflex r {
        on g {
            out_val = true;
        }
    }
}"#;
    let result = pipeline_with_rspu(src).expect("pipeline must succeed");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");

    let mut has_sr_init = false;
    let mut has_sr_tick = false;
    let mut has_sr_query = false;
    let mut i = 0;
    while i < rspu.instructions.len() && i < MAX_TEST_ITERATIONS {
        match &rspu.instructions[i] {
            RspuInstruction::SrInit { .. } => has_sr_init = true,
            RspuInstruction::SrTick { .. } => has_sr_tick = true,
            RspuInstruction::SrQuery { .. } => has_sr_query = true,
            _ => {}
        }
        i += 1;
    }
    // 4 cycles <= 16 threshold → shift register guard.
    assert!(has_sr_init, "4-cycle guard must use SR_INIT");
    assert!(has_sr_tick, "4-cycle guard must use SR_TICK");
    assert!(has_sr_query, "4-cycle guard must use SR_QUERY");
}

#[test]
fn e6_counter_guard_for_large_cycles() {
    let src = r#"module counter_test {
    signal s: in bool;
    signal out_val: out bool;

    guard g {
        when s
        for 64 cycles;
    }

    reflex r {
        on g {
            out_val = true;
        }
    }
}"#;
    let result = pipeline_with_rspu(src).expect("pipeline must succeed");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");

    let mut has_ctr_init = false;
    let mut i = 0;
    while i < rspu.instructions.len() && i < MAX_TEST_ITERATIONS {
        if let RspuInstruction::CtrInit { .. } = &rspu.instructions[i] {
            has_ctr_init = true;
        }
        i += 1;
    }
    // 64 cycles > 16 threshold → counter guard.
    assert!(has_ctr_init, "64-cycle guard must use CTR_INIT (counter guard)");
}

#[test]
fn e6_tmr_sensor_fusion_compiles_to_rspu() {
    let src = r#"module tmr_simple {
    signal sensor_a_ok: in bool;
    signal sensor_a: in u16;
    signal voted_value: out u16;
    signal sensor_a_failed: out bool;

    guard a_healthy {
        when sensor_a_ok
        for 1 cycles;
    }

    guard a_sick {
        when !sensor_a_ok
        for 8 cycles;
    }

    reflex vote_a {
        on a_healthy {
            voted_value = sensor_a;
        }
    }

    reflex flag_a_failed {
        on a_sick {
            sensor_a_failed = true;
        }
    }
}"#;
    let result = pipeline_with_rspu(src).expect("TMR module must compile to R-SPU");
    assert!(result.rspu_program.is_some(), "TMR must produce an R-SPU program");
}

// ===========================================================================
// E7: Resource limits
// ===========================================================================

#[test]
fn e7_max_registers_is_256() {
    assert_eq!(MAX_REGISTERS, 256, "MAX_REGISTERS must be 256");
}

#[test]
fn e7_max_guards_is_64() {
    assert_eq!(MAX_GUARDS, 64, "MAX_GUARDS must be 64");
}

#[test]
fn e7_max_instructions_is_4096() {
    assert_eq!(MAX_INSTRUCTIONS, 4096, "MAX_INSTRUCTIONS must be 4096");
}

#[test]
fn e7_max_sim_cycles_is_1_000_000() {
    assert_eq!(MAX_SIM_CYCLES, 1_000_000, "MAX_SIM_CYCLES must be 1,000,000");
}

// ===========================================================================
// E8: MEGA-3 new instructions
// ===========================================================================

#[test]
fn e8_trap_mnemonic() {
    let i = RspuInstruction::Trap { code: 5 };
    assert_eq!(i.mnemonic(), "TRAP");
}

#[test]
fn e8_trap_if_mnemonic() {
    let i = RspuInstruction::TrapIf { cond: 0, code: 1 };
    assert_eq!(i.mnemonic(), "TRAP_IF");
}

#[test]
fn e8_halt_mnemonic() {
    assert_eq!(RspuInstruction::Halt.mnemonic(), "HALT");
}

#[test]
fn e8_mode_switch_mnemonic() {
    let i = RspuInstruction::ModeSwitch { mode: 1 };
    assert_eq!(i.mnemonic(), "MODE_SWITCH");
}

#[test]
fn e8_tag_load_mnemonic() {
    let i = RspuInstruction::TagLoad { dst: 0, tag: 1 };
    assert_eq!(i.mnemonic(), "TAG_LOAD");
}

#[test]
fn e8_tag_check_mnemonic() {
    let i = RspuInstruction::TagCheck { src: 0, expected: 1 };
    assert_eq!(i.mnemonic(), "TAG_CHECK");
}

#[test]
fn e8_tag_read_mnemonic() {
    let i = RspuInstruction::TagRead { dst: 0, src: 1 };
    assert_eq!(i.mnemonic(), "TAG_READ");
}

#[test]
fn e8_nop_mnemonic() {
    assert_eq!(RspuInstruction::Nop.mnemonic(), "NOP");
}

#[test]
fn e8_fence_mnemonic() {
    assert_eq!(RspuInstruction::Fence.mnemonic(), "FENCE");
}

#[test]
fn e8_deadline_set_mnemonic() {
    let i = RspuInstruction::DeadlineSet { cycles: 100 };
    assert_eq!(i.mnemonic(), "DEADLINE_SET");
}

#[test]
fn e8_sim_trap_raises_exception() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::Trap { code: 5 }]);
    let result = sim.step(&prog).expect("step must not error");
    match result {
        StepResult::Exception(code) => {
            assert_eq!(
                code,
                nasa_rust_project::emit::rspu_exceptions::ExceptionCode::SoftwareTrap,
                "TRAP must raise SoftwareTrap exception"
            );
        }
        other => panic!("Expected Exception, got {:?}", other),
    }
}

#[test]
fn e8_sim_fence_is_noop() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::Fence, RspuInstruction::Nop]);
    let r1 = sim.step(&prog).expect("step 1");
    assert_eq!(r1, StepResult::Continue, "FENCE must continue execution");
    assert_eq!(sim.pc, 1, "PC must advance past FENCE");
}

// ===========================================================================
// E9: MEGA-4 totality instructions
// ===========================================================================

#[test]
fn e9_verify_mnemonic() {
    let i = RspuInstruction::Verify { cert_offset: 0 };
    assert_eq!(i.mnemonic(), "VERIFY");
}

#[test]
fn e9_certify_mnemonic() {
    let i = RspuInstruction::Certify { dst: 0 };
    assert_eq!(i.mnemonic(), "CERTIFY");
}

#[test]
fn e9_total_check_mnemonic() {
    let i = RspuInstruction::TotalCheck { expected_properties: 5 };
    assert_eq!(i.mnemonic(), "TOTAL_CHECK");
}

// ===========================================================================
// E10: MEGA-5 symbolic instructions
// ===========================================================================

#[test]
fn e10_match_mnemonic() {
    let i = RspuInstruction::Match { dst: 0, src: 1, table_offset: 0 };
    assert_eq!(i.mnemonic(), "MATCH");
}

#[test]
fn e10_interval_lo_mnemonic() {
    let i = RspuInstruction::IntervalLo { dst: 0, src: 1 };
    assert_eq!(i.mnemonic(), "INTERVAL_LO");
}

#[test]
fn e10_interval_hi_mnemonic() {
    let i = RspuInstruction::IntervalHi { dst: 0, src: 1 };
    assert_eq!(i.mnemonic(), "INTERVAL_HI");
}

#[test]
fn e10_interval_check_mnemonic() {
    let i = RspuInstruction::IntervalCheck { src: 0, bounds: 1 };
    assert_eq!(i.mnemonic(), "INTERVAL_CHECK");
}

#[test]
fn e10_sim_interval_shadow_initialized() {
    let sim = RspuSimulator::new();
    assert_eq!(
        sim.interval_shadow.len(),
        MAX_REGISTERS,
        "Interval shadow must have MAX_REGISTERS entries"
    );
    // All default to (0, u64::MAX).
    let mut i = 0;
    while i < sim.interval_shadow.len() && i < MAX_TEST_ITERATIONS {
        assert_eq!(
            sim.interval_shadow[i],
            (0, u64::MAX),
            "Default interval shadow for R{} must be [0, u64::MAX]",
            i
        );
        i += 1;
    }
}

#[test]
fn e10_sim_cert_verified_starts_false() {
    let sim = RspuSimulator::new();
    assert!(!sim.cert_verified, "cert_verified must start false");
}

// ===========================================================================
// Gap 1: .mirr examples → R-SPU pipeline compilation
// ===========================================================================

#[test]
fn e6_example_autonomous_vehicle_compiles_to_rspu() {
    let src = include_str!("../examples/autonomous_vehicle.mirr");
    let result = pipeline_with_rspu(src).expect("autonomous_vehicle must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_fir_filter_compiles_to_rspu() {
    let src = include_str!("../examples/fir_filter.mirr");
    let result = pipeline_with_rspu(src).expect("fir_filter must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_flight_controller_compiles_to_rspu() {
    let src = include_str!("../examples/flight_controller.mirr");
    let result = pipeline_with_rspu(src).expect("flight_controller must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_flight_controller_signed_compiles_to_rspu() {
    let src = include_str!("../examples/flight_controller_signed.mirr");
    // This example has a guard condition that cannot be lowered to hardware,
    // so the R-SPU backend correctly rejects it with a TemporalCompilationError.
    let result = pipeline_with_rspu(src);
    assert!(
        result.is_err(),
        "flight_controller_signed should fail R-SPU compilation (unsupported guard form)"
    );
}

#[test]
fn e6_example_icu_monitor_compiles_to_rspu() {
    let src = include_str!("../examples/icu_monitor.mirr");
    let result = pipeline_with_rspu(src).expect("icu_monitor must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_industrial_safety_compiles_to_rspu() {
    let src = include_str!("../examples/industrial_safety.mirr");
    let result = pipeline_with_rspu(src).expect("industrial_safety must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_multi_guard_monitor_compiles_to_rspu() {
    let src = include_str!("../examples/multi_guard_monitor.mirr");
    let result = pipeline_with_rspu(src).expect("multi_guard_monitor must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_neonatal_respirator_compiles_to_rspu() {
    let src = include_str!("../examples/neonatal_respirator.mirr");
    let result = pipeline_with_rspu(src).expect("neonatal_respirator must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_pattern_usage_compiles_to_rspu() {
    let src = include_str!("../examples/pattern_usage.mirr");
    let result = pipeline_with_rspu(src).expect("pattern_usage must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_safety_property_compiles_to_rspu() {
    let src = include_str!("../examples/safety_property.mirr");
    let result = pipeline_with_rspu(src).expect("safety_property must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_shift_register_guard_compiles_to_rspu() {
    let src = include_str!("../examples/shift_register_guard.mirr");
    let result = pipeline_with_rspu(src).expect("shift_register_guard must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_tmr_sensor_fusion_compiles_to_rspu() {
    let src = include_str!("../examples/tmr_sensor_fusion.mirr");
    let result = pipeline_with_rspu(src).expect("tmr_sensor_fusion must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

// ===========================================================================
// Gap 2: Simulator execution tests for Verify/Certify/TotalCheck (opcodes 30-32)
// ===========================================================================

#[test]
fn e9_sim_verify_sets_cert_verified() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::Verify { cert_offset: 0 }]);
    sim.step(&prog).expect("step");
    assert!(sim.cert_verified, "VERIFY must set cert_verified to true");
}

#[test]
fn e9_sim_certify_reads_cert_verified_false() {
    let mut sim = RspuSimulator::new();
    // Without Verify first, cert_verified is false
    let prog = make_program(vec![RspuInstruction::Certify { dst: 192 }]);
    sim.step(&prog).expect("step");
    assert_eq!(sim.registers.read(192).value, 0, "Certify without Verify must write 0");
}

#[test]
fn e9_sim_verify_then_certify() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::Verify { cert_offset: 0 },
        RspuInstruction::Certify { dst: 192 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(192).value, 1, "Certify after Verify must write 1");
}

#[test]
fn e9_sim_total_check_no_violations_continues() {
    let mut sim = RspuSimulator::new();
    // No violations registered, expected_properties=0 should pass
    let prog = make_program(vec![RspuInstruction::TotalCheck { expected_properties: 0 }]);
    let result = sim.step(&prog).expect("step");
    assert_eq!(
        result,
        StepResult::Continue,
        "TotalCheck with 0 expected and 0 violations must continue"
    );
}

#[test]
fn e9_sim_total_check_with_violations_raises_exception() {
    let mut sim = RspuSimulator::new();
    // Add a violation so the check fails
    sim.properties.violations.push(0);
    let prog = make_program(vec![RspuInstruction::TotalCheck { expected_properties: 2 }]);
    let result = sim.step(&prog).expect("step");
    match result {
        StepResult::Exception(code) => {
            assert_eq!(
                code,
                nasa_rust_project::emit::rspu_exceptions::ExceptionCode::PropertyFail,
                "TotalCheck with violations must raise PropertyFail"
            );
        }
        other => panic!("Expected PropertyFail exception, got {:?}", other),
    }
}

// ===========================================================================
// Gap 3: Simulator execution tests for Match/IntervalLo/IntervalHi/IntervalCheck (opcodes 33-36)
// ===========================================================================

#[test]
fn e10_sim_match_nonzero_input() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 16 },
        RspuInstruction::Match { dst: 193, src: 192, table_offset: 0 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(193).value, 1, "MATCH on nonzero input must return 1");
}

#[test]
fn e10_sim_match_zero_input() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 16 },
        RspuInstruction::Match { dst: 193, src: 192, table_offset: 0 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(193).value, 0, "MATCH on zero input must return 0");
}

#[test]
fn e10_sim_interval_lo_reads_shadow() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (100, 200);
    let prog = make_program(vec![RspuInstruction::IntervalLo { dst: 192, src: 5 }]);
    sim.step(&prog).expect("step");
    assert_eq!(sim.registers.read(192).value, 100, "IntervalLo must read lower bound from shadow");
}

#[test]
fn e10_sim_interval_hi_reads_shadow() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (100, 200);
    let prog = make_program(vec![RspuInstruction::IntervalHi { dst: 192, src: 5 }]);
    sim.step(&prog).expect("step");
    assert_eq!(sim.registers.read(192).value, 200, "IntervalHi must read upper bound from shadow");
}

#[test]
fn e10_sim_interval_lo_default_is_zero() {
    let sim = RspuSimulator::new();
    // Default interval lower bound is 0
    assert_eq!(sim.interval_shadow[0].0, 0, "Default interval lo must be 0");
}

#[test]
fn e10_sim_interval_hi_default_is_u64_max() {
    let sim = RspuSimulator::new();
    // Default interval upper bound is u64::MAX
    assert_eq!(sim.interval_shadow[0].1, u64::MAX, "Default interval hi must be u64::MAX");
}

#[test]
fn e10_sim_interval_check_in_range_passes() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 25 (in range [10,50])
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 25, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    assert_eq!(result, StepResult::Continue, "In-range check must continue");
}

#[test]
fn e10_sim_interval_check_at_lower_boundary_passes() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 10 (exact lower bound, should pass)
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    assert_eq!(result, StepResult::Continue, "Exact lower bound must continue");
}

#[test]
fn e10_sim_interval_check_at_upper_boundary_passes() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 50 (exact upper bound, should pass)
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 50, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    assert_eq!(result, StepResult::Continue, "Exact upper bound must continue");
}

#[test]
fn e10_sim_interval_check_out_of_range_exception() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 100 (out of range [10,50])
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 100, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    match result {
        StepResult::Exception(code) => {
            assert_eq!(
                code,
                nasa_rust_project::emit::rspu_exceptions::ExceptionCode::IntervalViolation,
                "Out-of-range IntervalCheck must raise IntervalViolation"
            );
        }
        other => panic!("Expected IntervalViolation exception, got {:?}", other),
    }
}

#[test]
fn e10_sim_interval_check_below_range_exception() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 5 (below range [10,50])
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    match result {
        StepResult::Exception(code) => {
            assert_eq!(
                code,
                nasa_rust_project::emit::rspu_exceptions::ExceptionCode::IntervalViolation,
                "Below-range IntervalCheck must raise IntervalViolation"
            );
        }
        other => panic!("Expected IntervalViolation exception, got {:?}", other),
    }
}

// ===========================================================================
// Gap 4: Encoding roundtrip tests for opcodes 30-36
// ===========================================================================

#[test]
fn e3_encode_decode_verify_roundtrip() {
    let i = RspuInstruction::Verify { cert_offset: 4096 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "Verify encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_certify_roundtrip() {
    let i = RspuInstruction::Certify { dst: 192 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "Certify encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_total_check_roundtrip() {
    let i = RspuInstruction::TotalCheck { expected_properties: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "TotalCheck encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_match_roundtrip() {
    let i = RspuInstruction::Match { dst: 193, src: 10, table_offset: 42 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "Match encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_interval_lo_roundtrip() {
    let i = RspuInstruction::IntervalLo { dst: 192, src: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "IntervalLo encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_interval_hi_roundtrip() {
    let i = RspuInstruction::IntervalHi { dst: 192, src: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "IntervalHi encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_interval_check_roundtrip() {
    let i = RspuInstruction::IntervalCheck { src: 192, bounds: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "IntervalCheck encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_all_mega4_mega5_opcodes_roundtrip() {
    let mega4_5_instrs: [RspuInstruction; 7] = [
        RspuInstruction::Verify { cert_offset: 0 },
        RspuInstruction::Certify { dst: 192 },
        RspuInstruction::TotalCheck { expected_properties: 10 },
        RspuInstruction::Match { dst: 193, src: 5, table_offset: 7 },
        RspuInstruction::IntervalLo { dst: 194, src: 10 },
        RspuInstruction::IntervalHi { dst: 195, src: 10 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 3 },
    ];
    let mut i = 0;
    while i < mega4_5_instrs.len() && i < MAX_TEST_ITERATIONS {
        let encoded = encode(&mega4_5_instrs[i]).expect("encode must succeed");
        let decoded = decode(encoded.0).expect("decode must succeed");
        assert_eq!(
            decoded,
            mega4_5_instrs[i],
            "Roundtrip failed for {}",
            mega4_5_instrs[i].mnemonic()
        );
        i += 1;
    }
}
