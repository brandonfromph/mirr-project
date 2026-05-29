#![forbid(unsafe_code)]
#![allow(clippy::identity_op, clippy::erasing_op)]

use nasa_rust_project::emit::rspu_sim::RspuSimulator;
use nasa_rust_project::emit::rspu_tagged::TypeTag;
use nasa_rust_project::pipeline::PipelineConfig;
use nasa_rust_project::Workspace;
use std::path::PathBuf;

#[test]
fn test_noc_router_priority_scheduling() {
    let root_path = PathBuf::from("rspu_chip/interconnect/noc_test_wrapper.mirr");
    let workspace_root = PathBuf::from("rspu_chip/interconnect");

    let mut workspace = Workspace::new(&workspace_root);
    let config = PipelineConfig { rspu: true, bootstrap_mode: true, ..Default::default() };

    println!("Compiling NoC behavioral test top from {}...", root_path.display());
    let snapshot =
        workspace.compile_snapshot(&root_path, &config).expect("Workspace compilation failed");

    let prog = snapshot.pipeline.rspu_program.clone().expect("RSPU program not generated");
    println!("Compiled NoC Router successfully!");
    println!("=== RSPU ALL INSTRUCTIONS ===");
    let asm = prog.emit_asm();
    for line in asm.lines() {
        println!("{}", line);
    }
    println!("=====================================");

    let mut sim = RspuSimulator::new();

    // Default all inputs
    sim.set_input(0, 1, TypeTag::Bool); // clk = 1
    sim.set_input(1, 1, TypeTag::Bool); // rst_n = 1
    for i in 0..16 {
        sim.set_input(2 + 2 * i, 0, TypeTag::Bool); // port_tx_valid_i = false
        sim.set_input(3 + 2 * i, 0, TypeTag::Unsigned { width: 64 }); // port_tx_data_i = 0
    }

    // --- Scenario 1: Standard routing (No contention) ---
    // Port 0 tx valid, data target port 5 (dest_id = 5), payload 12345
    // tx_data = (5 << 60) | 12345 = 576460752303435881
    let tx_data_0 = (5u64 << 60) | 12345u64;
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 1, TypeTag::Bool); // rst_n
    sim.set_input(2 + 2 * 0, 1, TypeTag::Bool); // tx_valid_0 = true
    sim.set_input(3 + 2 * 0, tx_data_0, TypeTag::Unsigned { width: 64 }); // tx_data_0

    sim.run_cycle(&prog).expect("Simulation failed on cycle 1");
    println!("--- Simulator outputs after cycle 1 ---");
    for i in 0..16 {
        let valid = sim.read_output(2 * i).map(|w| w.value != 0).unwrap_or(false);
        let data = sim.read_output(2 * i + 1).map(|w| w.value).unwrap_or(0);
        if valid || data != 0 {
            println!("  Port {}: valid={}, data={}", i, valid, data);
        }
    }

    sim.run_cycle(&prog).expect("Simulation failed on cycle 2");
    println!("--- Simulator outputs after cycle 2 ---");
    for i in 0..16 {
        let valid = sim.read_output(2 * i).map(|w| w.value != 0).unwrap_or(false);
        let data = sim.read_output(2 * i + 1).map(|w| w.value).unwrap_or(0);
        if valid || data != 0 {
            println!("  Port {}: valid={}, data={}", i, valid, data);
        }
    }

    sim.run_cycle(&prog).expect("Simulation failed on cycle 3");
    println!("--- Simulator outputs after cycle 3 ---");
    for i in 0..16 {
        let valid = sim.read_output(2 * i).map(|w| w.value != 0).unwrap_or(false);
        let data = sim.read_output(2 * i + 1).map(|w| w.value).unwrap_or(0);
        if valid || data != 0 {
            println!("  Port {}: valid={}, data={}", i, valid, data);
        }
    }

    // Check outputs
    let rx_valid_5 = sim.read_output(2 * 5).map(|w| w.value != 0).unwrap_or(false);
    let rx_data_5 = sim.read_output(2 * 5 + 1).map(|w| w.value).unwrap_or(0);

    assert!(rx_valid_5, "Port 5 should receive a valid packet");
    assert_eq!(
        rx_data_5, tx_data_0,
        "Port 5 data mismatch! Expected {}, got {}",
        tx_data_0, rx_data_5
    );

    // --- Scenario 2: High-priority reflexive preemption (Contention on destination 7) ---
    // Port 1 (Standard): dest_id = 7, priority bit = 0, payload = 11111
    // Port 2 (Reflexive): dest_id = 7, priority bit = 1, payload = 22222
    // Expected result: Port 7 should receive the Reflexive packet from Port 2.
    let tx_data_1 = (7u64 << 60) | 11111u64;
    let tx_data_2 = (7u64 << 60) | (1u64 << 59) | 22222u64;

    // Reset inputs
    for i in 0..16 {
        sim.set_input(2 + 2 * i, 0, TypeTag::Bool);
        sim.set_input(3 + 2 * i, 0, TypeTag::Unsigned { width: 64 });
    }

    sim.set_input(2 + 2 * 1, 1, TypeTag::Bool); // tx_valid_1 = true
    sim.set_input(3 + 2 * 1, tx_data_1, TypeTag::Unsigned { width: 64 }); // tx_data_1
    sim.set_input(2 + 2 * 2, 1, TypeTag::Bool); // tx_valid_2 = true
    sim.set_input(3 + 2 * 2, tx_data_2, TypeTag::Unsigned { width: 64 }); // tx_data_2

    sim.run_cycle(&prog).expect("Simulation failed on cycle 3");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 4");

    let rx_valid_7 = sim.read_output(2 * 7).map(|w| w.value != 0).unwrap_or(false);
    let rx_data_7 = sim.read_output(2 * 7 + 1).map(|w| w.value).unwrap_or(0);

    assert!(rx_valid_7, "Port 7 should receive a valid packet");
    assert_eq!(
        rx_data_7, tx_data_2,
        "Port 7 should receive the high-priority packet! Expected {}, got {}",
        tx_data_2, rx_data_7
    );

    // --- Scenario 3: Dedicated Reflexive Channels (Preemption via dedicated port 14) ---
    // Port 3 (Standard): dest_id = 8, priority bit = 0, payload = 33333
    // Port 14 (Dedicated Reflexive): dest_id = 8, priority bit = 0, payload = 44444
    // Expected result: Port 8 should receive the packet from Port 14 because 14 is a dedicated reflexive channel.
    let tx_data_3 = (8u64 << 60) | 33333u64;
    let tx_data_14 = (8u64 << 60) | 44444u64;

    // Reset inputs
    for i in 0..16 {
        sim.set_input(2 + 2 * i, 0, TypeTag::Bool);
        sim.set_input(3 + 2 * i, 0, TypeTag::Unsigned { width: 64 });
    }

    sim.set_input(2 + 2 * 3, 1, TypeTag::Bool); // tx_valid_3 = true
    sim.set_input(3 + 2 * 3, tx_data_3, TypeTag::Unsigned { width: 64 }); // tx_data_3
    sim.set_input(2 + 2 * 14, 1, TypeTag::Bool); // tx_valid_14 = true
    sim.set_input(3 + 2 * 14, tx_data_14, TypeTag::Unsigned { width: 64 }); // tx_data_14

    sim.run_cycle(&prog).expect("Simulation failed on cycle 5");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 6");

    let rx_valid_8 = sim.read_output(2 * 8).map(|w| w.value != 0).unwrap_or(false);
    let rx_data_8 = sim.read_output(2 * 8 + 1).map(|w| w.value).unwrap_or(0);

    assert!(rx_valid_8, "Port 8 should receive a valid packet");
    assert_eq!(
        rx_data_8, tx_data_14,
        "Port 8 should receive packet from dedicated port 14! Expected {}, got {}",
        tx_data_14, rx_data_8
    );

    println!("NOC DUAL-PRIORITY QUEUES & PREEMPTIVE ROUTING VERIFIED!");
}
