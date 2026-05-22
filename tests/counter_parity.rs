#![forbid(unsafe_code)]
//! Counter Parity Test
//! Verifies that long-delay temporal guards (synthesized as Counters)
//! correctly track time and trigger actions after the specified delay.

use nasa_rust_project::emit::rspu_sim::RspuSimulator;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_rspu_counter_parity() {
    let delay_cycles = 25; // > 16, so it must be a counter
    let mirr = format!(
        r#"
module counter_parity {{
    signal trigger: in bool;
    signal result: out u64;

    guard timer_guard {{ when trigger for {} cycles; }}
    reflex timer {{
        on timer_guard {{
            result = 100;
        }}
    }}
}}
"#,
        delay_cycles
    );

    // 1. Compile
    let cfg = PipelineConfig {
        rspu: true,
        ..Default::default()
    };
    let res = run_pipeline(&mirr, &cfg).expect("Compilation failed");
    let prog = res.rspu_program.expect("RSPU program not generated");

    println!("--- Synthesized Assembly ---");
    println!("{}", prog.emit_asm());

    // 2. Simulate with Cycle-by-Cycle Inputs
    let mut sim = RspuSimulator::new();
    use nasa_rust_project::emit::rspu_tagged::TypeTag;

    // Cycle 0: Set trigger to true
    sim.set_input(0, 1, TypeTag::Bool); // trigger is P0
    sim.run_cycle(&prog).expect("Sim failed");

    for c in 1..=delay_cycles {
        // Set input for each cycle
        sim.set_input(0, 1, TypeTag::Bool);

        // Execute one clock tick
        sim.run_cycle(&prog).expect("Sim failed");

        let out = sim.read_output(0).map(|w| w.value).unwrap_or(0);
        if c < delay_cycles {
            assert_eq!(out, 0, "Output triggered too early at cycle {}", c);
        } else {
            assert_eq!(out, 100, "Output failed to trigger at cycle {}", c);
        }
    }

    println!("Counter Parity Verified: Triggered exactly at cycle {}!", delay_cycles);
}
