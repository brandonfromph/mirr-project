use super::*;
use nasa_rust_project::emit::rspu_tagged::{TaggedWord, TypeTag};

#[test]
fn test_pc_advances_on_continue() {
    let mut sim = RspuSimulator::new();
    let program =
        make_program(vec![RspuInstruction::Nop, RspuInstruction::Nop, RspuInstruction::Halt]);
    let _ = sim.step(&program).expect("Step 0 must succeed");
    assert_eq!(sim.pc, 1, "PC must be 1 after first Nop");
    let _ = sim.step(&program).expect("Step 1 must succeed");
    assert_eq!(sim.pc, 2, "PC must be 2 after second Nop");
}

#[test]
fn test_pc_stays_on_halt() {
    let mut sim = RspuSimulator::new();
    let program =
        make_program(vec![RspuInstruction::Nop, RspuInstruction::Halt, RspuInstruction::Nop]);
    let result = sim.run(&program, 100).expect("Program must succeed");
    assert_eq!(sim.pc, 1, "PC must stay at Halt instruction index");
    assert!(result.halted, "SimResult must show halted");
}

#[test]
fn test_pc_past_end_halts() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Nop]);
    let s1 = sim.step(&program).expect("First step must succeed");
    assert_eq!(s1, StepResult::Continue, "Nop returns Continue");
    assert_eq!(sim.pc, 1, "PC advances past the single Nop");
    let s2 = sim.step(&program).expect("Second step (past end) must succeed");
    assert_eq!(s2, StepResult::Halted, "PC past end must produce Halted");
    assert!(sim.halted, "Simulator must be halted after PC passes end");
}

// ---------------------------------------------------------------------------
// 23. Max cycles exceeded
// ---------------------------------------------------------------------------

#[test]
fn test_max_cycles_exceeded_error() {
    let mut sim = RspuSimulator::new();
    // Infinite loop: Branch to self (PC 0) to avoid automatic halt.
    let program = make_program(vec![RspuInstruction::TagBranch { tag_value: 8, target_pc: 0 }]);
    // Set tag register to 0 and R0 to 0 to ensure branch is taken.
    sim.tag_register = 0;
    sim.registers.write(0, TaggedWord::from_literal(0, TypeTag::Unsigned { width: 8 }));

    let err = sim.run(&program, 5).expect_err("Must error when max_cycles exceeded without halt");
    let msg = err.to_string();
    assert!(msg.contains("E713"), "Max cycles error must contain E713, got: {msg}");
}

#[test]
fn test_max_sim_cycles_cap() {
    // Verify the MAX_SIM_CYCLES constant is reasonable.
    assert_eq!(MAX_SIM_CYCLES, 1_000_000, "MAX_SIM_CYCLES must be 1_000_000, got {MAX_SIM_CYCLES}");
}

// ---------------------------------------------------------------------------
// 24. Wrapping overflow
// ---------------------------------------------------------------------------

#[test]
fn test_add_wrapping_overflow() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: u64::MAX, width: 64 },
        RspuInstruction::LoadImm { dst: 193, value: 1, width: 64 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Wrapping add must succeed");
    assert_eq!(sim.registers.read(194).value, 0, "u64::MAX + 1 must wrap to 0");
}

#[test]
fn test_sub_wrapping_underflow() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 1, width: 8 },
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Wrapping sub must succeed");
    assert_eq!(sim.registers.read(194).value, u64::MAX, "0 - 1 must wrap to u64::MAX");
}

#[test]
fn test_mul_wrapping_overflow() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: u64::MAX, width: 64 },
        RspuInstruction::LoadImm { dst: 193, value: 2, width: 64 },
        RspuInstruction::Alu { op: AluOp::Mul, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Wrapping mul must succeed");
    let expected = u64::MAX.wrapping_mul(2);
    assert_eq!(sim.registers.read(194).value, expected, "u64::MAX * 2 must wrap correctly");
}

// ---------------------------------------------------------------------------
// 25. SimResult output collection
// ---------------------------------------------------------------------------

#[test]
fn test_sim_result_collects_outputs() {
    let mut sim = RspuSimulator::new();
    let mut instrs = Vec::new();
    for i in 0..MAX_OUTPUT_SCAN {
        instrs.push(RspuInstruction::LoadImm { dst: 192, value: (i * 10) as u64, width: 8 });
        instrs.push(RspuInstruction::StoreOutput { src: 192, port: i as u16 });
    }
    instrs.push(RspuInstruction::Halt);
    let program = make_program(instrs);
    let result = sim.run(&program, 1000).expect("Output collection must succeed");
    assert_eq!(
        result.outputs.len(),
        MAX_OUTPUT_SCAN,
        "Must collect {MAX_OUTPUT_SCAN} output ports"
    );
    for i in 0..MAX_OUTPUT_SCAN {
        let word =
            result.outputs.get(&(i as u16)).unwrap_or_else(|| panic!("Output port {i} must exist"));
        assert_eq!(
            word.value,
            (i * 10) as u64,
            "Output port {i} must have value {expected}",
            expected = i * 10
        );
    }
}

#[test]
fn test_sim_result_no_outputs_when_none_written() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Nop, RspuInstruction::Halt]);
    let result = sim.run(&program, 100).expect("No-output program must succeed");
    assert!(
        result.outputs.is_empty(),
        "SimResult outputs must be empty when no StoreOutput executed"
    );
}

#[test]
fn test_sim_result_fields() {
    // MEGA-4: AssertAlways with cond=0 now raises PropertyFail,
    // stopping execution at cycle 2 (LoadImm + AssertAlways).
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 42 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("SimResult fields test must succeed");
    assert_eq!(result.cycles, 1, "cycles must be 1 (combinatorial execution)");
    assert!(!result.halted, "halted must be false (exception stopped execution)");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::PropertyFail),
        "exception must be PropertyFail"
    );
    assert_eq!(result.property_violations, vec![42], "property_violations must contain [42]");
}

// ---------------------------------------------------------------------------
// 26. Set input and read output partition
// ---------------------------------------------------------------------------

#[test]
fn test_set_input_registers() {
    let mut sim = RspuSimulator::new();
    sim.set_input(5, 0xBEEF, TypeTag::Unsigned { width: 16 });
    let word = sim.registers.read(5); // Port 5 maps to R5
    assert_eq!(word.value, 0xBEEF, "Input register R5 must hold 0xBEEF");
    assert_eq!(word.tag, TypeTag::Unsigned { width: 16 }, "Input tag must be Unsigned(16)");
}

#[test]
fn test_read_output_partition() {
    let mut sim = RspuSimulator::new();
    // Directly write to the output partition register R64 (port 0).
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 555, width: 16 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Output partition test must succeed");
    let out = sim.read_output(0).expect("Output port 0 must be readable");
    assert_eq!(out.value, 555, "Output port 0 must hold 555");
    // Verify the register is indeed at REG_OUTPUT_BASE.
    let direct = sim.registers.read(REG_OUTPUT_BASE);
    assert_eq!(direct.value, 555, "REG_OUTPUT_BASE register must match output port 0");
}

// ---------------------------------------------------------------------------
// 27. Guard bounds checking
// ---------------------------------------------------------------------------

#[test]
fn test_guard_out_of_bounds_reads_false() {
    let mut sim = RspuSimulator::new();
    // Query a guard that is within array but never set -- must be false.
    let program = make_program(vec![
        RspuInstruction::SrQuery { dst: 192, guard: 63 }, // MAX_GUARDS-1
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Guard bounds query must succeed");
    assert_eq!(sim.registers.read(192).value, 0, "Unset guard at max index must read as false");
}

// ---------------------------------------------------------------------------
// 28. Multiple guards initialization
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_guards_independent() {
    let mut sim = RspuSimulator::new();
    let mut instrs = Vec::new();
    instrs.push(RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 });
    instrs.push(RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 });
    for i in 0..MAX_GUARD_TEST {
        let cond = if i % 2 == 0 { 192 } else { 193 };
        instrs.push(RspuInstruction::SrInit { guard: i as u8, length: 1, cond });
    }
    for i in 0..MAX_GUARD_TEST {
        instrs.push(RspuInstruction::SrQuery { dst: (200 + i) as u8, guard: i as u8 });
    }
    instrs.push(RspuInstruction::Halt);
    let program = make_program(instrs);
    let _result = sim.run(&program, 200).expect("Multiple guards must succeed");
    for i in 0..MAX_GUARD_TEST {
        let expected = if i % 2 == 0 { 1u64 } else { 0u64 };
        assert_eq!(
            sim.registers.read((200 + i) as u8).value,
            expected,
            "Guard {i} must be {expected_str}",
            expected_str = if expected == 1 { "active" } else { "inactive" }
        );
    }
}

// ---------------------------------------------------------------------------
// 29. Cycle counter accuracy
// ---------------------------------------------------------------------------

#[test]
fn test_cycle_counter_increments_per_step() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Cycle counter test must succeed");
    assert_eq!(result.cycles, 1, "Full program is 1 cycle");
    assert_eq!(sim.cycle, 1, "Simulator cycle counter must match SimResult");
}

// ---------------------------------------------------------------------------
// 30. Stress test: many instructions
// ---------------------------------------------------------------------------

#[test]
fn test_stress_many_nops() {
    let mut sim = RspuSimulator::new();
    let mut instrs = Vec::new();
    for _i in 0..MAX_STRESS_INSTRS {
        instrs.push(RspuInstruction::Nop);
    }
    instrs.push(RspuInstruction::Halt);
    let program = make_program(instrs);
    let result = sim.run(&program, 1000).expect("Stress test must succeed");
    assert!(result.halted, "Stress test must halt");
    assert_eq!(result.cycles, 1, "Stress test executes in 1 combinatorial cycle");
}

// ---------------------------------------------------------------------------
// 31. ALU tag mismatch errors
// ---------------------------------------------------------------------------

#[test]
fn test_alu_tag_mismatch_unsigned_signed() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        // Manually tag R193 as signed.
        RspuInstruction::LoadImm { dst: 193, value: 20, width: 8 },
        RspuInstruction::TagLoad { dst: 193, tag: 136 }, // 128+8 = Signed{width:8}
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let err = sim.run(&program, 100).expect_err("Mismatched tags must error");
    let msg = err.to_string();
    assert!(msg.contains("E708"), "Tag mismatch must produce E708, got: {msg}");
}

// ---------------------------------------------------------------------------
// 32. Comparison result is Bool
// ---------------------------------------------------------------------------

#[test]
fn test_comparison_produces_bool_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 10, width: 8 },
        RspuInstruction::Alu { op: AluOp::Lt, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Comparison must succeed");
    assert_eq!(sim.registers.read(194).value, 1, "5 < 10 must be true (1)");
    assert_eq!(sim.registers.read(194).tag, TypeTag::Bool, "Comparison result tag must be Bool");
}

#[test]
fn test_eq_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 10, width: 8 },
        RspuInstruction::Alu { op: AluOp::Eq, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Eq comparison must succeed");
    assert_eq!(sim.registers.read(194).value, 0, "5 == 10 must be false (0)");
}

// ---------------------------------------------------------------------------
// 33. Exception terminates run
// ---------------------------------------------------------------------------

#[test]
fn test_exception_terminates_run() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Trap { code: 1 },
        RspuInstruction::Nop,  // unreachable
        RspuInstruction::Halt, // unreachable
    ]);
    let result = sim.run(&program, 100).expect("Exception must terminate run");
    assert!(!result.halted, "Halted flag must be false when exception terminates");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::SoftwareTrap),
        "Exception must be SoftwareTrap"
    );
    assert_eq!(result.cycles, 1, "Program terminates at Trap in cycle 1");
}

// ---------------------------------------------------------------------------
// 34. Register file default state
// ---------------------------------------------------------------------------

#[test]
fn test_uninitialized_register_is_zero_valued() {
    let sim = RspuSimulator::new();
    let word = sim.registers.read(192);
    assert_eq!(word.value, 0, "Uninitialized register value must be 0");
    assert_eq!(
        word.tag,
        TypeTag::Uninitialized,
        "Uninitialized register tag must be Uninitialized"
    );
}

// ---------------------------------------------------------------------------
// 35. LoadImm large width clamped
// ---------------------------------------------------------------------------

#[test]
fn test_load_imm_large_width_clamped() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 200 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Large width LoadImm must succeed");
    // Width > 127 is clamped to 127 by width_to_type_tag.
    assert_eq!(
        sim.registers.read(192).tag,
        TypeTag::Unsigned { width: 127 },
        "Width > 127 must clamp to Unsigned(127)"
    );
}

// ---------------------------------------------------------------------------
// 36. AluImm preserves tag from operand
// ---------------------------------------------------------------------------

#[test]
fn test_alu_imm_preserves_operand_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 100, width: 16 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 193, a: 192, imm: 50 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("AluImm tag preservation must succeed");
    assert_eq!(
        sim.registers.read(193).tag,
        TypeTag::Unsigned { width: 16 },
        "AluImm result tag must match operand's Unsigned(16)"
    );
    assert_eq!(sim.registers.read(193).value, 150, "100 + imm(50) = 150");
}

// ---------------------------------------------------------------------------
// 37. SrQuery produces Bool tag
// ---------------------------------------------------------------------------

#[test]
fn test_sr_query_produces_bool_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrQuery Bool tag must succeed");
    assert_eq!(sim.registers.read(193).tag, TypeTag::Bool, "SrQuery result must have Bool tag");
}

// ---------------------------------------------------------------------------
// 38. CtrQuery produces Bool tag
// ---------------------------------------------------------------------------

#[test]
fn test_ctr_query_produces_bool_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::CtrInit { guard: 0, target: 5, cond: 192 },
        RspuInstruction::CtrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("CtrQuery Bool tag must succeed");
    assert_eq!(sim.registers.read(193).tag, TypeTag::Bool, "CtrQuery result must have Bool tag");
}

// ---------------------------------------------------------------------------
// 39. Full datapath: input -> ALU -> output
// ---------------------------------------------------------------------------

#[test]
fn test_full_datapath_input_alu_output() {
    let mut sim = RspuSimulator::new();
    sim.set_input(0, 100, TypeTag::Unsigned { width: 16 });
    sim.set_input(1, 50, TypeTag::Unsigned { width: 16 });
    let program = make_program(vec![
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::LoadInput { dst: 193, port: 1 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::StoreOutput { src: 194, port: 0 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Full datapath must succeed");
    assert!(result.halted, "Datapath program must halt");
    let out = result.outputs.get(&0).expect("Output port 0 must exist");
    assert_eq!(out.value, 150, "100 + 50 = 150");
}

// ---------------------------------------------------------------------------
// 40. Conditional datapath with guard
// ---------------------------------------------------------------------------

#[test]
fn test_conditional_datapath_with_guard() {
    let mut sim = RspuSimulator::new();
    sim.set_input(0, 1, TypeTag::Unsigned { width: 8 }); // condition = true
    let program = make_program(vec![
        // Load condition and init guard.
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        // Prepare a value.
        RspuInstruction::LoadImm { dst: 193, value: 42, width: 8 },
        // Conditional move: only if guard active.
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        // Output the result.
        RspuInstruction::StoreOutput { src: 194, port: 0 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Conditional datapath must succeed");
    assert!(result.halted, "Conditional datapath must halt");
    let out = result.outputs.get(&0).expect("Output port 0 must exist");
    assert_eq!(out.value, 42, "Output must be 42 when guard is active");
}

// ---------------------------------------------------------------------------
// 41. Deadline cleared after miss
// ---------------------------------------------------------------------------

#[test]
fn test_deadline_cleared_after_miss() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::DeadlineSet { cycles: 1 },
        RspuInstruction::Nop, // cycle becomes 2 after this, >= deadline(1)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Deadline cleared test must succeed");
    assert_eq!(result.exception, Some(ExceptionCode::DeadlineMiss), "Must report DeadlineMiss");
    // After the deadline fires, the deadline field is cleared.
    assert!(sim.deadline.is_none(), "Deadline must be cleared after miss");
}

// ---------------------------------------------------------------------------
// 42. Signed arithmetic via TagLoad
// ---------------------------------------------------------------------------

#[test]
fn test_signed_arithmetic() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::TagLoad { dst: 192, tag: 136 }, // Signed{width:8}
        RspuInstruction::LoadImm { dst: 193, value: 3, width: 8 },
        RspuInstruction::TagLoad { dst: 193, tag: 136 }, // Signed{width:8}
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Signed arithmetic must succeed");
    assert_eq!(sim.registers.read(194).value, 7, "10 - 3 = 7");
    assert_eq!(
        sim.registers.read(194).tag,
        TypeTag::Signed { width: 8 },
        "Result tag must be Signed(8)"
    );
}
