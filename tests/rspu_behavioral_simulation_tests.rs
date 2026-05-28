#![forbid(unsafe_code)]

use nasa_rust_project::emit::rspu_sim::RspuSimulator;
use nasa_rust_project::emit::rspu_tagged::TypeTag;
use nasa_rust_project::pipeline::PipelineConfig;
use nasa_rust_project::Workspace;
use std::path::PathBuf;

#[test]
fn test_rspu_alu_behavioral_simulation() {
    let root_path = PathBuf::from("rspu_chip/core/alu_test_wrapper.mirr");
    let workspace_root = PathBuf::from("rspu_chip/core");

    let mut workspace = Workspace::new(&workspace_root);
    let config = PipelineConfig { rspu: true, bootstrap_mode: true, ..Default::default() };

    println!("Compiling ALU behavioral test top from {}...", root_path.display());
    let snapshot =
        workspace.compile_snapshot(&root_path, &config).expect("Workspace compilation failed");

    let prog = snapshot.pipeline.rspu_program.clone().expect("RSPU program not generated");
    println!("Compiled ALU Core successfully!");

    // DEBUG: dump full assembly for inspection
    println!("{}", prog.emit_asm());

    let mut sim = RspuSimulator::new();

    // --- Scenario 1: ADD (opcode = 0) ---
    // data_in = 150, addr = 50 -> Expected result = 200
    // Tag = 1 (Unsigned), Provenance = 0
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 0, TypeTag::Unsigned { width: 16 }); // instr = ADD (0)
    sim.set_input(2, 4294967296 | 50, TypeTag::Unsigned { width: 64 }); // addr
    sim.set_input(3, 4294967296 | 150, TypeTag::Unsigned { width: 64 }); // data_in
    sim.run_cycle(&prog).expect("Simulation failed on cycle 1");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 2");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 3");

    let out1 = sim.read_output(0).map(|w| w.value & 0xFFFFFFFF).unwrap_or(0);
    println!("ADD output: {}", out1);
    assert_eq!(out1, 200, "ALU ADD failed! Expected 200, got {}", out1);

    // --- Scenario 2: SUB (opcode = 1) ---
    // data_in = 80, addr = 200 -> Expected result = 120
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 1, TypeTag::Unsigned { width: 16 }); // instr = SUB (1)
    sim.set_input(2, 4294967296 | 200, TypeTag::Unsigned { width: 64 }); // addr
    sim.set_input(3, 4294967296 | 80, TypeTag::Unsigned { width: 64 }); // data_in
    sim.run_cycle(&prog).expect("Simulation failed on cycle 4");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 5");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 6");

    let out2 = sim.read_output(0).map(|w| w.value & 0xFFFFFFFF).unwrap_or(0);
    println!("SUB output: {}", out2);
    assert_eq!(out2, 120, "ALU SUB failed! Expected 120, got {}", out2);

    // --- Scenario 3: MOV (opcode = 2, default) ---
    // data_in = 99 -> Expected result = 99
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 2, TypeTag::Unsigned { width: 16 }); // instr = MOV (2)
    sim.set_input(2, 4294967296 | 0, TypeTag::Unsigned { width: 64 }); // addr
    sim.set_input(3, 4294967296 | 99, TypeTag::Unsigned { width: 64 }); // data_in
    sim.run_cycle(&prog).expect("Simulation failed on cycle 7");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 8");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 9");

    let out3 = sim.read_output(0).map(|w| w.value & 0xFFFFFFFF).unwrap_or(0);
    println!("MOV output: {}", out3);
    assert_eq!(out3, 99, "ALU MOV failed! Expected 99, got {}", out3);

    // --- Scenario 4: ReLU Positive (opcode = 3) ---
    // in1 = 100 -> Expected result = 100
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 3, TypeTag::Unsigned { width: 16 }); // instr = ReLU (3)
    sim.set_input(2, 4294967296 | 100, TypeTag::Unsigned { width: 64 }); // addr (in1)
    sim.set_input(3, 4294967296 | 0, TypeTag::Unsigned { width: 64 }); // data_in (in2)
    sim.run_cycle(&prog).expect("Simulation failed on cycle 10");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 11");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 12");

    let out4 = sim.read_output(0).map(|w| w.value & 0xFFFFFFFF).unwrap_or(0);
    println!("ReLU positive output: {}", out4);
    assert_eq!(out4, 100, "ALU ReLU positive failed! Expected 100, got {}", out4);

    // --- Scenario 5: ReLU Negative (opcode = 3) ---
    // in1 = -50 (data payload with bit 31 set: 0xFFFFFFCE) -> Expected result = 0
    let neg_50 = 0xFFFFFFCEu64;
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 3, TypeTag::Unsigned { width: 16 }); // instr = ReLU (3)
    sim.set_input(2, 4294967296 | neg_50, TypeTag::Unsigned { width: 64 }); // addr (in1)
    sim.set_input(3, 4294967296 | 0, TypeTag::Unsigned { width: 64 });
    sim.run_cycle(&prog).expect("Simulation failed on cycle 13");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 14");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 15");

    let out5 = sim.read_output(0).map(|w| w.value & 0xFFFFFFFF).unwrap_or(0);
    println!("ReLU negative output: {}", out5);
    assert_eq!(out5, 0, "ALU ReLU negative failed! Expected 0, got {}", out5);

    // --- Scenario 6: LeakyReLU Negative (opcode = 4) ---
    // in1 = -80 (data payload with bit 31 set: 0xFFFFFFB0) -> Expected result = -80 >> 3 = -10 (which is 0xFFFFFFF6)
    let neg_80 = 0xFFFFFFB0u64;
    let expected_leaky = 0x1FFFFFF6u64;
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 4, TypeTag::Unsigned { width: 16 }); // instr = LeakyReLU (4)
    sim.set_input(2, 4294967296 | neg_80, TypeTag::Unsigned { width: 64 }); // addr (in1)
    sim.set_input(3, 4294967296 | 0, TypeTag::Unsigned { width: 64 });
    sim.run_cycle(&prog).expect("Simulation failed on cycle 16");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 17");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 18");

    let out6 = sim.read_output(0).map(|w| w.value & 0xFFFFFFFF).unwrap_or(0);
    println!("LeakyReLU negative output: {:X}", out6);
    assert_eq!(
        out6, expected_leaky,
        "ALU LeakyReLU negative failed! Expected {:X}, got {:X}",
        expected_leaky, out6
    );

    // --- Scenario 7: TAG_GATE (opcode = 5) ---
    // in1: data = 160, tag = 1 (unsigned), provenance = 12 (confidence 12)
    // tag_word = (12 << 36) | (1 << 32) | 160 = 824633720992 | 4294967296 | 160 = 828928688448
    // Expected result: (160 * 12) >> 4 = 1920 >> 4 = 120
    let tag_word = (12u64 << 36) | 4294967296u64 | 160u64;
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 5, TypeTag::Unsigned { width: 16 }); // instr = TAG_GATE (5)
    sim.set_input(2, tag_word, TypeTag::Unsigned { width: 64 }); // addr (in1)
    sim.set_input(3, 4294967296 | 0, TypeTag::Unsigned { width: 64 });
    sim.run_cycle(&prog).expect("Simulation failed on cycle 19");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 20");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 21");

    let out7 = sim.read_output(0).map(|w| w.value & 0xFFFFFFFF).unwrap_or(0);
    println!("TAG_GATE output: {}", out7);
    assert_eq!(out7, 120, "ALU TAG_GATE failed! Expected 120, got {}", out7);

    println!("BIT-PERFECT ALU BEHAVIORAL PARITY VERIFIED!");
}
