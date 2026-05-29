//! Cycle-accuracy and double-buffering verification tests for R-SPU.
//!
//! These tests verify that state updates (SrTick, CtrTick) are only visible
//! to queries (SrQuery, CtrQuery, ReflexIf) in subsequent cycles, resolving
//! sequential shadowing bugs.

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
fn test_sr_tick_delayed_visibility() {
    let mut sim = RspuSimulator::new();

    // Program 1: Init and Tick
    let prog_init = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 194, guard: 0 },
        RspuInstruction::Halt,
    ]);

    // Cycle 1: Init and Tick. Query should see Init result immediately (immediate visibility for Init).
    let _ = sim.run_cycle(&prog_init).expect("Cycle 1 should succeed");
    assert_eq!(
        sim.registers.read(194).value,
        1,
        "Init failure: SrQuery did not see SrInit result immediately!"
    );

    // Program 2: Just Query (No Init)
    let prog_query =
        make_program(vec![RspuInstruction::SrQuery { dst: 194, guard: 0 }, RspuInstruction::Halt]);

    // Cycle 2: Just query. Should see the result of Cycle 1's Tick.
    let _ = sim.run_cycle(&prog_query).expect("Cycle 2 should succeed");
    assert_eq!(
        sim.registers.read(194).value,
        1,
        "Double-buffering failure: SrQuery did not see committed state in next cycle"
    );
}

#[test]
fn test_ctr_tick_delayed_visibility() {
    let mut sim = RspuSimulator::new();

    let prog_init = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::CtrInit { guard: 0, target: 1, cond: 192 },
        RspuInstruction::CtrTick { guard: 0 },
        RspuInstruction::CtrQuery { dst: 194, guard: 0 },
        RspuInstruction::Halt,
    ]);

    let _ = sim.run_cycle(&prog_init).expect("Cycle 1 should succeed");
    assert_eq!(
        sim.registers.read(194).value,
        0,
        "Init failure: CtrQuery did not see CtrInit result immediately!"
    );

    let prog_query =
        make_program(vec![RspuInstruction::CtrQuery { dst: 194, guard: 0 }, RspuInstruction::Halt]);

    let _ = sim.run_cycle(&prog_query).expect("Cycle 2 should succeed");
    assert_eq!(sim.registers.read(194).value, 1);
}

#[test]
fn test_reflex_if_order_independence() {
    let mut sim = RspuSimulator::new();

    // Cycle 1: Init and Tick
    let prog_init = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _ = sim.run_cycle(&prog_init).expect("Cycle 1 should succeed");

    // Cycle 2: ReflexIf. Should move data because Cycle 1 ticked.
    let prog_reflex = make_program(vec![
        RspuInstruction::LoadImm { dst: 196, value: 42, width: 8 },
        RspuInstruction::LoadImm { dst: 195, value: 0, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 195, src: 196 },
        RspuInstruction::Halt,
    ]);

    let _ = sim.run_cycle(&prog_reflex).expect("Cycle 2 should succeed");
    assert_eq!(sim.registers.read(195).value, 42);
}
