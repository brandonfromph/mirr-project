#![forbid(unsafe_code)]
#![allow(clippy::identity_op, clippy::erasing_op)]

use mirrc::emit::rspu_sim::RspuSimulator;
use mirrc::emit::rspu_tagged::TypeTag;
use mirrc::pipeline::PipelineConfig;
use mirrc::Workspace;
use std::path::PathBuf;

#[test]
#[ignore = "Fundamentally incompatible with variable-scale architecture due to hardcoded 16-port MIRR wrapper"]
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

    let reg = snapshot.pipeline.ecs_registry.as_ref().expect("Registry required");
    let num_ports = reg
        .kinds
        .iter()
        .flatten()
        .filter(|k| {
            if let mirrc::ecs::EntityKind::SIGNAL(_) = k.0 {
                let idx = (k as *const _ as usize - reg.kinds.as_ptr() as usize)
                    / std::mem::size_of::<Option<mirrc::ecs::components::KindComponent>>();
                if let Some(name) = &reg.names[idx] {
                    let n_str = reg.resolve_name(name.0);
                    return n_str.starts_with("port_tx_valid_")
                        || n_str.starts_with("port_tx_valid[");
                }
            }
            false
        })
        .count() as u16;

    let num_ports = if num_ports > 0 { num_ports } else { 16 };
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
    for i in 0..num_ports {
        sim.set_input(2 + i, 0, TypeTag::Bool); // port_tx_valid[i] = false
        sim.set_input(2 + num_ports + i, 0, TypeTag::Unsigned { width: 64 }); // port_tx_data[i] = 0
    }
    sim.set_input(2 + 2 * num_ports, 0, TypeTag::Bool); // stream_tx_valid
    sim.set_input(2 + 2 * num_ports + 1, 0, TypeTag::Unsigned { width: 64 }); // stream_tx_data

    // --- Scenario 1: Standard routing (No contention) ---
    // Port 0 tx valid, data target port 5 (dest_id = 5), payload 12345
    // tx_data = (5 << 48) | 12345
    let p5 = 5 % num_ports;
    let tx_data_0 = ((p5 as u64) << 48) | 12345u64;
    sim.set_input(0, 1, TypeTag::Bool); // clk
    sim.set_input(1, 1, TypeTag::Bool); // rst_n
    sim.set_input(2 + 0, 1, TypeTag::Bool); // tx_valid_0 = true
    sim.set_input(2 + num_ports + 0, tx_data_0, TypeTag::Unsigned { width: 64 }); // tx_data_0

    sim.run_cycle(&prog).expect("Simulation failed on cycle 1");
    println!("--- Simulator outputs after cycle 1 ---");
    for i in 0..num_ports {
        let valid = sim.read_output(i).map(|w| w.value != 0).unwrap_or(false);
        let data = sim.read_output(num_ports + i).map(|w| w.value).unwrap_or(0);
        if valid || data != 0 {
            println!("  Port {}: valid={}, data={}", i, valid, data);
        }
    }

    sim.run_cycle(&prog).expect("Simulation failed on cycle 2");
    println!("--- Simulator outputs after cycle 2 ---");
    for i in 0..num_ports {
        let valid = sim.read_output(i).map(|w| w.value != 0).unwrap_or(false);
        let data = sim.read_output(num_ports + i).map(|w| w.value).unwrap_or(0);
        if valid || data != 0 {
            println!("  Port {}: valid={}, data={}", i, valid, data);
        }
    }

    sim.run_cycle(&prog).expect("Simulation failed on cycle 3");
    println!("--- Simulator outputs after cycle 3 ---");
    for i in 0..num_ports {
        let valid = sim.read_output(i).map(|w| w.value != 0).unwrap_or(false);
        let data = sim.read_output(num_ports + i).map(|w| w.value).unwrap_or(0);
        if valid || data != 0 {
            println!("  Port {}: valid={}, data={}", i, valid, data);
        }
    }

    // Check outputs
    let rx_valid_5 = sim.read_output(p5).map(|w| w.value != 0).unwrap_or(false);
    let rx_data_5 = sim.read_output(num_ports + p5).map(|w| w.value).unwrap_or(0);

    assert!(rx_valid_5, "Port {} should receive a valid packet", p5);
    let expected_payload_0 = 12345u64;
    assert_eq!(
        rx_data_5, expected_payload_0,
        "Port 5 received incorrect payload. Expected {}, got {}",
        expected_payload_0, rx_data_5
    );

    // --- Scenario 2: High-priority reflexive preemption (Contention on destination 7) ---
    // Port 1 (Standard): dest_id = 7, priority bit = 0, payload = 11111
    // Port 2 (Reflexive): dest_id = 7, priority bit = 1, payload = 22222
    // Expected result: Port 7 should receive the Reflexive packet from Port 2.
    let p7 = 7 % num_ports;
    let src1 = 1 % num_ports;
    let src2 = 2 % num_ports;
    let tx_data_1 = ((p7 as u64) << 48) | 11111u64;
    let tx_data_2 = ((p7 as u64) << 48) | (1u64 << 60) | 22222u64;

    // Reset inputs
    for i in 0..num_ports {
        sim.set_input(2 + i, 0, TypeTag::Bool);
        sim.set_input(2 + num_ports + i, 0, TypeTag::Unsigned { width: 64 });
    }
    sim.set_input(2 + 2 * num_ports, 0, TypeTag::Bool); // stream_tx_valid
    sim.set_input(2 + 2 * num_ports + 1, 0, TypeTag::Unsigned { width: 64 }); // stream_tx_data

    sim.set_input(2 + src1, 1, TypeTag::Bool); // tx_valid_1 = true
    sim.set_input(2 + num_ports + src1, tx_data_1, TypeTag::Unsigned { width: 64 }); // tx_data_1
    sim.set_input(2 + src2, 1, TypeTag::Bool); // tx_valid_2 = true
    sim.set_input(2 + num_ports + src2, tx_data_2, TypeTag::Unsigned { width: 64 }); // tx_data_2

    sim.run_cycle(&prog).expect("Simulation failed on cycle 3");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 4");

    let rx_valid_7 = sim.read_output(p7).map(|w| w.value != 0).unwrap_or(false);
    let rx_data_7 = sim.read_output(num_ports + p7).map(|w| w.value).unwrap_or(0);

    assert!(rx_valid_7, "Port {} should receive a valid packet", p7);
    let expected_payload_2 = 22222u64;
    assert_eq!(
        rx_data_7, expected_payload_2,
        "Port 7 should have received the high-priority packet from Port 2"
    );

    // --- Scenario 3: Dedicated Reflexive Channels (Preemption via dedicated port 14) ---
    // Port 3 (Standard): dest_id = 8, priority bit = 0, payload = 33333
    // Port 14 (Dedicated Reflexive): dest_id = 8, priority bit = 0, payload = 44444
    // Expected result: Port 8 should receive the packet from Port 14 because 14 is a dedicated reflexive channel.
    let p8 = 8 % num_ports;
    let p14 = 14 % num_ports;
    let src3 = 3 % num_ports;
    let tx_data_3 = ((p8 as u64) << 48) | 33333u64;
    let tx_data_14 = ((p8 as u64) << 48) | 44444u64;

    // Reset inputs
    for i in 0..num_ports {
        sim.set_input(2 + i, 0, TypeTag::Bool);
        sim.set_input(2 + num_ports + i, 0, TypeTag::Unsigned { width: 64 });
    }
    sim.set_input(2 + 2 * num_ports, 0, TypeTag::Bool); // stream_tx_valid
    sim.set_input(2 + 2 * num_ports + 1, 0, TypeTag::Unsigned { width: 64 }); // stream_tx_data

    sim.set_input(2 + src3, 1, TypeTag::Bool); // tx_valid_3 = true
    sim.set_input(2 + num_ports + src3, tx_data_3, TypeTag::Unsigned { width: 64 }); // tx_data_3
    sim.set_input(2 + p14, 1, TypeTag::Bool); // tx_valid_14 = true
    sim.set_input(2 + num_ports + p14, tx_data_14, TypeTag::Unsigned { width: 64 }); // tx_data_14

    sim.run_cycle(&prog).expect("Simulation failed on cycle 5");
    sim.run_cycle(&prog).expect("Simulation failed on cycle 6");

    let rx_valid_8 = sim.read_output(p8).map(|w| w.value != 0).unwrap_or(false);
    let rx_data_8 = sim.read_output(num_ports + p8).map(|w| w.value).unwrap_or(0);

    assert!(rx_valid_8, "Port {} should receive a valid packet", p8);
    let expected_payload_14 = 44444u64;
    assert_eq!(
        rx_data_8, expected_payload_14,
        "Port 8 should have received the packet from dedicated reflexive channel 14"
    );

    println!("NOC DUAL-PRIORITY QUEUES & PREEMPTIVE ROUTING VERIFIED!");
}
