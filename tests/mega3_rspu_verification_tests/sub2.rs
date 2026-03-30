use super::*;

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
