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
fn qa_boundary_sr_length_zero() {
    let mut sim = RspuSimulator::new();
    // Initialize SR with length 0.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 128, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 0, cond: 128 },
        RspuInstruction::Halt,
    ]);
    sim.run(&prog, 1).unwrap();

    // Now try to query it. This calls the ! operator internally or via read_guard_bool.
    let prog_query =
        make_program(vec![RspuInstruction::SrQuery { dst: 129, guard: 0 }, RspuInstruction::Halt]);
    // This should not panic.
    let _ = sim.run(&prog_query, 1);
}

#[test]
fn qa_boundary_sr_length_64() {
    let mut sim = RspuSimulator::new();
    // Initialize SR with length 64.
    // Note: 1u64 << 64 is a panic/error in Rust if not handled.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 128, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 64, cond: 128 },
        RspuInstruction::Halt,
    ]);
    // This should not panic.
    let _ = sim.run(&prog, 1);
}
