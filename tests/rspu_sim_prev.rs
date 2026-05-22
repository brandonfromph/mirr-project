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
fn test_sim_prev_basic() {
    let mut sim = RspuSimulator::new();

    // Cycle 1: Load 42 into R128 and Halt.
    let prog1 = make_program(vec![
        RspuInstruction::LoadImm { dst: 128, value: 42, width: 8 },
        RspuInstruction::Halt,
    ]);
    sim.run(&prog1, 1).expect("Cycle 1 should succeed");

    // Cycle 2: Use PREV to load Cycle 1's R128 into R129.
    let prog2 = make_program(vec![
        RspuInstruction::Prev { dst: 129, signal: 128, delay: 1 },
        RspuInstruction::Halt,
    ]);
    sim.run(&prog2, 1).expect("Cycle 2 should succeed");

    assert_eq!(
        sim.registers.read(129).value,
        42,
        "PREV(delay=1) should retrieve last cycle's value"
    );
}

#[test]
fn test_sim_prev_multi_cycle() {
    let mut sim = RspuSimulator::new();

    // Cycle 1: val = 10
    sim.run(
        &make_program(vec![
            RspuInstruction::LoadImm { dst: 128, value: 10, width: 8 },
            RspuInstruction::Halt,
        ]),
        1,
    )
    .unwrap();

    // Cycle 2: val = 20
    sim.run(
        &make_program(vec![
            RspuInstruction::LoadImm { dst: 128, value: 20, width: 8 },
            RspuInstruction::Halt,
        ]),
        1,
    )
    .unwrap();

    // Cycle 3: val = 30
    sim.run(
        &make_program(vec![
            RspuInstruction::LoadImm { dst: 128, value: 30, width: 8 },
            RspuInstruction::Halt,
        ]),
        1,
    )
    .unwrap();

    // Cycle 4: Query Prev
    let prog4 = make_program(vec![
        RspuInstruction::Prev { dst: 130, signal: 128, delay: 1 }, // Should be 30
        RspuInstruction::Prev { dst: 131, signal: 128, delay: 2 }, // Should be 20
        RspuInstruction::Prev { dst: 132, signal: 128, delay: 3 }, // Should be 10
        RspuInstruction::Halt,
    ]);
    sim.run(&prog4, 1).unwrap();

    assert_eq!(sim.registers.read(130).value, 30);
    assert_eq!(sim.registers.read(131).value, 20);
    assert_eq!(sim.registers.read(132).value, 10);
}

#[test]
fn test_sim_prev_delay_zero() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 128, value: 99, width: 8 },
        RspuInstruction::Prev { dst: 129, signal: 128, delay: 0 },
        RspuInstruction::Halt,
    ]);
    sim.run(&prog, 1).unwrap();
    assert_eq!(sim.registers.read(129).value, 99, "PREV(delay=0) should be identity in same cycle");
}

#[test]
fn test_sim_prev_insufficient_history() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::Prev { dst: 129, signal: 128, delay: 5 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&prog, 1);
    assert!(result.is_err(), "Should fail if delay > available cycles");
}
