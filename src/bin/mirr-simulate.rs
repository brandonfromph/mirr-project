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
use clap::Parser;
use serde_json::json;

use nasa_rust_project::mape_k::{
    self, ActionEntry, AdaptationAction, MapeKSimulator, SensorConfig, SignalPredicate, SimConfig,
    TemporalProperty, TriggerCondition,
};

#[derive(Parser, Debug)]
#[command(name = "mirr-simulate", version, about = "MAPE-K simulation harness for MIRR/R-SPU")]
struct Cli {
    /// Load simulation config from JSON file
    #[arg(long)]
    config: Option<String>,

    /// Number of simulation ticks
    #[arg(long, default_value_t = 10000)]
    ticks: u64,

    /// Write JSON audit trail to FILE
    #[arg(long)]
    audit: Option<String>,

    /// Run built-in neonatal respirator scenario
    #[arg(long)]
    neonatal: bool,

    /// Print final signal state
    #[arg(long)]
    stats: bool,

    /// Export CLI schema as JSON for tool integration
    #[arg(long, hide = true)]
    help_json: bool,
}

fn main() {
    let args = Cli::parse();

    if args.help_json {
        use clap::CommandFactory;
        fn get_cmd_manifest(cmd: &clap::Command) -> serde_json::Value {
            let mut args_list = Vec::new();
            for arg in cmd.get_arguments() {
                args_list.push(serde_json::json!({
                    "id": arg.get_id().as_str(),
                    "long": arg.get_long(),
                    "short": arg.get_short(),
                    "help": arg.get_help().map(|h| h.to_string()),
                    "required": arg.is_required_set(),
                }));
            }
            let mut subs = Vec::new();
            for sub in cmd.get_subcommands() {
                subs.push(get_cmd_manifest(sub));
            }
            serde_json::json!({
                "name": cmd.get_name(),
                "about": cmd.get_about().map(|a| a.to_string()),
                "version": cmd.get_version().map(|v| v.to_string()),
                "args": args_list,
                "subcommands": subs,
            })
        }
        let cmd = Cli::command();
        println!("{}", serde_json::to_string_pretty(&get_cmd_manifest(&cmd)).unwrap());
        process::exit(0);
    }

    let config = if args.neonatal {
        neonatal_respirator_config()
    } else if let Some(ref path) = args.config {
        load_config(path)
    } else {
        eprintln!("Error: specify --config <path> or --neonatal");
        eprintln!("Run with --help for usage.");
        process::exit(1);
    };

    // Run simulation.
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(args.ticks);

    // Print summary.
    print!("{}", result.summary());

    if args.stats {
        println!("  Final signal state:");
        let mut sorted = result.final_signal_state.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, val) in &sorted {
            println!("    {name} = {val}");
        }
    }

    // Write audit trail if requested.
    if let Some(ref path) = args.audit {
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
        sensors: vec![SensorConfig {
            name: "airway_pressure".to_string(),
            base_value: 120,
            noise_amplitude: 5,
            fault_at_tick: Some(500),
            fault_value: 10, // Sensor degrades to dangerously low value.
            fault_end_tick: None,
            seed: 42,
            is_observable: true,
        }],
        properties: vec![
            // Safety property: airway pressure must always be above 50.
            TemporalProperty::Always(SignalPredicate::GreaterThan(
                "airway_pressure".to_string(),
                50,
            )),
            // Sustained low: pressure below 50 for 10 consecutive ticks
            // is a critical condition requiring emergency action.
            TemporalProperty::Persists(
                SignalPredicate::LessThan("airway_pressure".to_string(), 50),
                10,
            ),
        ],
        action_table: vec![
            // On sustained low pressure (property 1 satisfied): emergency stop.
            // Persists(LessThan(50), 10) being *satisfied* means dangerously
            // low pressure has held for 10 consecutive ticks.
            ActionEntry {
                trigger_property_idx: 1,
                action: AdaptationAction::EmergencyStop,
                priority: 100,
                trigger_on: TriggerCondition::OnSatisfaction,
            },
            // On any pressure violation (property 0 violated): log warning.
            // Always(GreaterThan(50)) being *violated* means pressure dropped.
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::SetSignal {
                    name: "airway_pressure".to_string(),
                    value: 1,
                },
                priority: 50,
                trigger_on: TriggerCondition::OnViolation,
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

