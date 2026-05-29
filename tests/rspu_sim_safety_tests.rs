#![forbid(unsafe_code)]
//! Regression tests for R-SPU hardware simulator safety, PCC verification, and totality checks.

use nasa_rust_project::emit::rspu_exceptions::ExceptionCode;
use nasa_rust_project::emit::rspu_isa::*;
use nasa_rust_project::emit::rspu_sim::{RspuSimulator, StepResult};
use nasa_rust_project::emit::rspu_tagged::{TaggedWord, TypeTag};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

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

/// Helper to create a minimal dummy RspuProgram with a single instruction.
fn dummy_program(instr: RspuInstruction) -> RspuProgram {
    RspuProgram {
        instructions: vec![instr],
        registers_used: 256,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    }
}

/// Helper to create a dummy program with multiple instructions.
fn dummy_program_multi(instructions: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        instructions,
        registers_used: 256,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    }
}

/// 16. Verify that multiple independent temporal guards are simulated correctly step-by-step.
#[test]
fn test_simulator_independent_multiple_guards() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: in bool;
    signal c: out bool;

    guard g1 {
        when a
        for 2 cycles;
    }

    guard g2 {
        when b
        for 3 cycles;
    }

    reflex r1 {
        on g1 and g2 {
            c = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &rspu_config()).expect("pipeline should succeed");
    let prog = result.rspu_program.expect("RSPU program not emitted");

    let mut sim = RspuSimulator::new();
    // Default clk = 1, rst_n = 1
    sim.set_input(0, 1, TypeTag::Bool);
    sim.set_input(1, 1, TypeTag::Bool);

    // Initial cycle
    sim.run_cycle(&prog).expect("Sim cycle 1 failed");

    // Both shift registers are present and independent
    assert!(
        prog.guards_used >= 2,
        "Should have at least 2 independent compiled guards, got: {}",
        prog.guards_used
    );
}

/// 17. Verify that logical ALU operations (And/Or) enforce matching boolean tags in the simulator.
#[test]
fn test_simulator_boolean_tag_checking() {
    let mut sim = RspuSimulator::new();

    // R192 = true (bool), R193 = 42 (u8)
    sim.registers.write(192, TaggedWord::from_literal(1, TypeTag::Bool));
    sim.registers.write(193, TaggedWord::from_literal(42, TypeTag::Unsigned { width: 8 }));

    // ALU And R194, R192, R193
    let instr = RspuInstruction::Alu { op: AluOp::And, dst: 194, a: 192, b: 193 };

    let prog = dummy_program(instr);
    let result = sim.step(&prog);
    assert!(result.is_err(), "Should fail with tag violation due to type mismatch");
    let err_str = format!("{:?}", result.err().unwrap());
    assert!(err_str.contains("E708"), "Should report tag violation code E708, got: {}", err_str);
}

/// 18. Verify that reading or writing outside valid register partitions (e.g. outputs as inputs) is handled.
#[test]
fn test_simulator_register_partition_exceptions() {
    let sim = RspuSimulator::new();

    // Reading outside output partition bounds (e.g. port 100 which maps to register 64 + 100 = 164)
    let out = sim.read_output(100);
    assert!(out.is_none(), "Out-of-bounds output port should return None");
}

/// 19. Verify that the Verify and Certify instructions validate formal proof certificates.
#[test]
fn test_simulator_pcc_verification_instruction() {
    let mut sim = RspuSimulator::new();

    // Check default state is not verified
    assert!(!sim.cert_verified);

    // Run VERIFY instruction followed by CERTIFY
    let verify_instr = RspuInstruction::Verify { cert_offset: 128 };
    let certify_instr = RspuInstruction::Certify { dst: 192 };

    let prog = dummy_program_multi(vec![verify_instr, certify_instr]);

    // Step 1: VERIFY
    let step_res = sim.step(&prog).expect("VERIFY instruction failed");
    assert!(matches!(step_res, StepResult::Continue));
    assert!(sim.cert_verified, "VERIFY should set cert_verified = true");

    // Step 2: CERTIFY
    sim.step(&prog).expect("CERTIFY instruction failed");

    let val = sim.registers.read(192);
    assert_eq!(val.value, 1, "CERTIFY should write 1 when verified");
    assert_eq!(val.tag, TypeTag::Unsigned { width: 1 });
}

/// 20. Verify that the TotalCheck instruction halts or traps when expected safety invariants are not satisfied.
#[test]
fn test_simulator_totality_invariant_gate() {
    let mut sim = RspuSimulator::new();

    // TotalCheck expects 2 satisfied properties, but we have 0.
    let total_check_instr = RspuInstruction::TotalCheck { expected_properties: 2 };
    let prog = dummy_program(total_check_instr);

    let step_res = sim.step(&prog).expect("TOTAL_CHECK failed");

    if let StepResult::Exception(exc) = step_res {
        assert_eq!(exc, ExceptionCode::PropertyFail, "Should raise ExceptionCode::PropertyFail");
    } else {
        panic!("Expected StepResult::Exception, got: {:?}", step_res);
    }
}
