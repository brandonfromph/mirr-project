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
fn qa_sr_total_satisfaction() {
    let mut sim = RspuSimulator::new();

    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 128, value: 1, width: 1 },
        RspuInstruction::SrInit { guard: 0, length: 2, cond: 128 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 129, guard: 0 },
        RspuInstruction::Halt,
    ]);

    // Run for 3 cycles (Cycle 0: init & tick -> next is 1. Cycle 1: init & tick -> next is 3. Cycle 2: Query reads current = 3)
    sim.run_cycle(&prog).unwrap();
    sim.run_cycle(&prog).unwrap();
    sim.run_cycle(&prog).unwrap();

    let res1 = sim.registers.read(129).value;
    println!("Res 1: {}", res1);
    assert_eq!(res1, 1);
}
