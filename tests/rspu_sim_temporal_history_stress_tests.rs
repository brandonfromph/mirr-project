use nasa_rust_project::emit::rspu_isa::{RspuInstruction, RspuProgram};
use nasa_rust_project::emit::rspu_sim::RspuSimulator;

fn make_program(instructions: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        instructions,
        registers_used: 256,
        guards_used: 64,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    }
}

#[test]
fn qa_stress_history_wraparound() {
    let mut sim = RspuSimulator::new();
    const MAX_HISTORY: u64 = 64; // from mod.rs

    // Cycle 0 to 64: Store the cycle number in R128.
    for i in 0..=MAX_HISTORY {
        let prog = make_program(vec![
            RspuInstruction::LoadImm { dst: 128, value: i, width: 8 },
            RspuInstruction::Halt,
        ]);
        sim.run(&prog, 1).unwrap();
    }

    // Now at Cycle 65. The history for Cycle 0 should be gone (overwritten).
    // The history for Cycle 64 should be at delay 1.
    // The history for Cycle 1 should be at delay 64.

    let prog_check = make_program(vec![
        RspuInstruction::Prev { dst: 130, signal: 128, delay: 1 }, // Should be 64
        RspuInstruction::Prev { dst: 131, signal: 128, delay: 64 }, // Should be 1
        RspuInstruction::Halt,
    ]);
    sim.run(&prog_check, 1).unwrap();

    assert_eq!(sim.registers.read(130).value, 64, "Delay 1 should be the most recent cycle");
    assert_eq!(
        sim.registers.read(131).value,
        1,
        "Delay 64 should be the oldest preserved cycle (Cycle 1)"
    );
}

#[test]
fn qa_stress_prev_feedback_loop() {
    let mut sim = RspuSimulator::new();

    // Cycle 1: R128 = 1
    sim.run(
        &make_program(vec![
            RspuInstruction::LoadImm { dst: 128, value: 1, width: 8 },
            RspuInstruction::Halt,
        ]),
        1,
    )
    .unwrap();

    // Cycles 2-10: R128 = Prev(R128, 1) + 1
    // This tests if Prev can be used to build a counter/accumulator.
    for _ in 2..=10 {
        let prog = make_program(vec![
            RspuInstruction::Prev { dst: 129, signal: 128, delay: 1 },
            RspuInstruction::AluImm {
                op: nasa_rust_project::emit::rspu_isa::AluOp::Add,
                dst: 128,
                a: 129,
                imm: 1,
            },
            RspuInstruction::Halt,
        ]);
        sim.run(&prog, 1).unwrap();
    }

    assert_eq!(
        sim.registers.read(128).value,
        10,
        "Temporal feedback loop should correctly accumulate to 10"
    );
}

#[test]
fn qa_stress_boundary_delay_max() {
    let mut sim = RspuSimulator::new();
    const MAX_HISTORY: u32 = 64;

    // Fill history
    for i in 1..=MAX_HISTORY {
        sim.run(
            &make_program(vec![
                RspuInstruction::LoadImm { dst: 128, value: i as u64, width: 8 },
                RspuInstruction::Halt,
            ]),
            1,
        )
        .unwrap();
    }

    // Exact boundary check
    let prog = make_program(vec![
        RspuInstruction::Prev { dst: 129, signal: 128, delay: MAX_HISTORY },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&prog, 1);
    assert!(result.is_ok(), "Delay exactly equal to history depth must succeed");
    assert_eq!(sim.registers.read(129).value, 1, "Should retrieve the very first cycle stored");

    // Off-by-one check
    let prog_fail = make_program(vec![
        RspuInstruction::Prev { dst: 130, signal: 128, delay: MAX_HISTORY + 1 },
        RspuInstruction::Halt,
    ]);
    assert!(
        sim.run(&prog_fail, 1).is_err(),
        "Delay exceeding history depth must fail deterministically"
    );
}
