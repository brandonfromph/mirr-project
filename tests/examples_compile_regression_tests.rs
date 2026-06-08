#![forbid(unsafe_code)]

use std::fs;

use mirrc::pipeline::{run_pipeline, PipelineConfig};

const REGRESSION_EXAMPLES: &[&str] = &[
    "examples/aerospace_flight_ctrl.mirr",
    "examples/array_register_file.mirr",
    "examples/automotive_brake.mirr",
    "examples/autonomous_vehicle.mirr",
    "examples/comm_watchdog.mirr",
    "examples/flight_controller.mirr",
    "examples/industrial_safety_plc.mirr",
    "examples/medical_ventilator.mirr",
    "examples/power_supply_monitor.mirr",
    "examples/sensor_fusion_advanced.mirr",
    "examples/struct_packet.mirr",
    "examples/thermal_management.mirr",
    "examples/tmr_voting_system.mirr",
    "examples/watchdog_timer.mirr",
];

#[test]
fn compile_regression_examples_through_pipeline() {
    const MAX_EXAMPLES: usize = 32;
    let mut failures = Vec::new();

    for (idx, example_path) in REGRESSION_EXAMPLES.iter().enumerate() {
        if idx >= MAX_EXAMPLES {
            break;
        }

        let source = fs::read_to_string(example_path)
            .unwrap_or_else(|err| panic!("failed to read {example_path}: {err}"));

        match run_pipeline(&source, &PipelineConfig::default()) {
            Ok(result) => {
                if let Err(err) =
                    mirrc::validation::validate_module(&result.program.module)
                {
                    failures.push(format!("{example_path} (validation): {err}"));
                }
            }
            Err(err) => failures.push(format!("{example_path}: {err}")),
        }
    }

    assert!(
        failures.is_empty(),
        "{} regression example(s) failed to compile:\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}
