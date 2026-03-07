//! mirr-simulate — CLI binary for the MAPE-K simulation harness.
//!
//! Runs a time-stepped MAPE-K feedback loop simulation and outputs
//! results as a structured summary with optional JSON audit trail.
//!
//! Usage:
//!   mirr-simulate --config sim_config.json --ticks 10000 [--audit audit.json]
//!   mirr-simulate --neonatal [--ticks N] [--audit audit.json]

#![forbid(unsafe_code)]

use std::process;

use nasa_rust_project::mape_k::{
    self,
    AdaptationAction, SimConfig, SensorConfig, TemporalProperty,
    SignalPredicate, ActionEntry, MapeKSimulator,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut config_path: Option<String> = None;
    let mut ticks: u64 = 10_000;
    let mut audit_path: Option<String> = None;
    let mut neonatal_mode = false;
    let mut show_help = false;
    let mut stats_mode = false;

    // Simple arg parsing (no external crate dependency).
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--config" => {
                i += 1;
                if i < args.len() {
                    config_path = Some(args[i].clone());
                }
            }
            "--ticks" => {
                i += 1;
                if i < args.len() {
                    ticks = args[i].parse().unwrap_or(10_000);
                }
            }
            "--audit" => {
                i += 1;
                if i < args.len() {
                    audit_path = Some(args[i].clone());
                }
            }
            "--neonatal" => neonatal_mode = true,
            "--stats" => stats_mode = true,
            "--help" | "-h" => show_help = true,
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                process::exit(1);
            }
        }
        i += 1;
    }

    if show_help {
        print_help();
        return;
    }

    let config = if neonatal_mode {
        neonatal_respirator_config()
    } else if let Some(ref path) = config_path {
        load_config(path)
    } else {
        eprintln!("Error: specify --config <path> or --neonatal");
        eprintln!("Run with --help for usage.");
        process::exit(1);
    };

    // Run simulation.
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(ticks);

    // Print summary.
    print!("{}", result.summary());

    if stats_mode {
        println!("  Final signal state:");
        let mut sorted = result.final_signal_state.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, val) in &sorted {
            println!("    {name} = {val}");
        }
    }

    // Write audit trail if requested.
    if let Some(ref path) = audit_path {
        match serde_json::to_string_pretty(&result.adaptation_log) {
            Ok(json) => {
                if let Err(e) = std::fs::write(path, json) {
                    eprintln!("Error writing audit file: {e}");
                    process::exit(1);
                }
                println!("Audit log written to {path}");
            }
            Err(e) => {
                eprintln!("Error serializing audit log: {e}");
                process::exit(1);
            }
        }
    }
}

/// Built-in neonatal respirator scenario (Kwon et al. 2021 inspired).
///
/// Simulates a pressure sensor with noise that experiences a fault
/// (sensor dropout) at tick 500, triggering the MAPE-K loop to
/// detect the sustained pressure drop and activate the emergency clamp.
fn neonatal_respirator_config() -> SimConfig {
    SimConfig {
        sensors: vec![
            SensorConfig {
                name: "airway_pressure".to_string(),
                base_value: 120,
                noise_amplitude: 5,
                fault_at_tick: Some(500),
                fault_value: 10, // Sensor degrades to dangerously low value.
                fault_end_tick: None,
                seed: 42,
            },
        ],
        properties: vec![
            // Safety property: airway pressure must always be above 50.
            TemporalProperty::Always(
                SignalPredicate::GreaterThan("airway_pressure".to_string(), 50),
            ),
            // Sustained low: pressure below 50 for 10 consecutive ticks
            // is a critical condition requiring emergency action.
            TemporalProperty::Persists(
                SignalPredicate::LessThan("airway_pressure".to_string(), 50),
                10,
            ),
        ],
        action_table: vec![
            // On sustained low pressure (property 1): emergency stop.
            ActionEntry {
                trigger_property_idx: 1,
                action: AdaptationAction::EmergencyStop,
                priority: 100,
            },
            // On any pressure violation (property 0): set clamp signal.
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::SetSignal {
                    name: "airway_pressure".to_string(),
                    value: 1,
                },
                priority: 50,
            },
        ],
        window_size: 64,
        knowledge_capacity: mape_k::knowledge::MAX_LOG_ENTRIES,
    }
}

fn load_config(path: &str) -> SimConfig {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading config file '{path}': {e}");
            process::exit(1);
        }
    };
    match serde_json::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing config JSON: {e}");
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("mirr-simulate — MAPE-K simulation harness for MIRR/R-SPU");
    println!();
    println!("Usage:");
    println!("  mirr-simulate --neonatal [--ticks N] [--audit FILE] [--stats]");
    println!("  mirr-simulate --config FILE --ticks N [--audit FILE] [--stats]");
    println!();
    println!("Options:");
    println!("  --neonatal     Run built-in neonatal respirator scenario");
    println!("  --config FILE  Load simulation config from JSON file");
    println!("  --ticks N      Number of simulation ticks (default: 10000)");
    println!("  --audit FILE   Write JSON audit trail to FILE");
    println!("  --stats        Print final signal state");
    println!("  --help, -h     Show this help");
}
