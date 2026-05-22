#![forbid(unsafe_code)]
//! Parity Gate: ISA-to-RTL Cross-Verification
//! Verifies that synthesized RSPU bytecode exactly matches the
//! logical behavior of the MIRR source across 16 parallel cores.

use nasa_rust_project::emit::rspu_sim::RspuSimulator;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_rspu_16_core_parity_gate() {
    // 1. ISA GOLDEN TRUTH
    // We expect each core to perform: (CoreID * 10) + Iteration
    // For 10 iterations.
    let num_cores = 16;
    let iterations = 10;
    let mut golden_results = vec![0u64; num_cores];
    for (core_id, val) in golden_results.iter_mut().enumerate() {
        let mut acc = core_id as u64 * 10;
        for i in 0..iterations {
            acc += i as u64;
        }
        *val = acc;
    }

    // 2. MIRR SOURCE (RTL)
    // We'll generate a MIRR module that simulates these 16 cores.
    let mut mirr = String::from("module rspu_parity {\n");
    for i in 0..num_cores {
        mirr.push_str(&format!("    signal core_{}_out: out u17;\n", i));
    }

    mirr.push_str("    reflex main {\n");
    mirr.push_str("        on always {\n");
    for core_id in 0..num_cores {
        let mut acc_expr = format!("{}", core_id * 10);
        for i in 0..iterations {
            acc_expr = format!("({} + {})", acc_expr, i);
        }
        mirr.push_str(&format!("            core_{}_out = {};\n", core_id, acc_expr));
    }
    mirr.push_str("        }\n");
    mirr.push_str("    }\n");
    mirr.push_str("}\n");

    // 3. COMPILE TO RSPU
    let cfg = PipelineConfig {
        rspu: true,
        ..Default::default()
    };
    let res = run_pipeline(&mirr, &cfg).expect("Compilation failed");
    let prog = res.rspu_program.expect("RSPU program not generated");

    println!("--- Synthesized Assembly ---");
    println!("{}", prog.emit_asm());

    // 4. RTL SIMULATION
    let mut sim = RspuSimulator::new();
    // Run for enough cycles to complete the reflex
    let sim_result = sim.run(&prog, 1000).expect("Simulation failed");
    println!("Cycles: {}, Halted: {}", sim_result.cycles, sim_result.halted);

    // 5. PARITY CHECK
    println!("--- Parity Gate Results ---");
    for (i, &expected) in golden_results.iter().enumerate() {
        let actual = sim_result.outputs.get(&(i as u16)).map(|v| v.value).unwrap_or(0);
        println!("Core {}: Expected {}, Actual {}", i, expected, actual);
        assert_eq!(actual, expected, "Core {} parity mismatch!", i);
    }
    println!("BIT-PERFECT PARITY ACHIEVED");
}
