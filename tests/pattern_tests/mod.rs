#![forbid(unsafe_code)]
//! Phase 7b: Homoiconic pattern system tests.
//!
//! Tests pattern definition parsing, pattern call parsing, substitution,
//! expansion, name prefixing, origin tagging, depth limits, internal signal
//! scoping, error messages, and emission integration.
//!
//! Minimum 48 tests. All error messages pinned with exact strings from spec.

use nasa_rust_project::ast::pattern::{PatternArg, PatternParamKind};
use nasa_rust_project::ast::types::{SignalKind, SignalType};
use nasa_rust_project::emit;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::validate_module;

// =========================================================================
// Helpers
// =========================================================================

/// Parse and return the program, or panic with the error.
fn parse_ok(source: &str) -> nasa_rust_project::MirrProgram {
    parse_mirr(source).unwrap_or_else(|e| panic!("Parse failed: {e}"))
}

/// Parse and return the error message string.
fn parse_err(source: &str) -> String {
    parse_mirr(source).expect_err("Expected parse error").to_string()
}

/// Run full pipeline and return the error message string.
fn pipeline_err(source: &str) -> String {
    match run_pipeline(source, &PipelineConfig::default()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("Expected pipeline error"),
    }
}

/// Run full pipeline and return the PipelineResult.
fn pipeline_ok(source: &str) -> nasa_rust_project::pipeline::PipelineResult {
    run_pipeline(source, &PipelineConfig::default())
        .unwrap_or_else(|e| panic!("Pipeline failed: {e}"))
}

/// A minimal module footer to make tests self-contained.
const MOD_FOOTER: &str = r#"
module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;

/// Minimal pattern definition for monitor_sensor.
/// Uses the parser-compatible multiline format.
fn monitor_sensor_source() -> &'static str {
    r#"
def monitor_sensor(
    sensor: signal in u16,
    low:    u16,
    high:   u16,
    cycles: u32,
    alarm:  signal out bool
) {
    reflect {
        signal ${sensor}_debounced: internal bool;

        guard ${sensor}_too_low {
            when ${sensor} < ${low}
            for  ${cycles} cycles;
        }

        guard ${sensor}_too_high {
            when ${sensor} > ${high}
            for  ${cycles} cycles;
        }

        reflex ${sensor}_response_low {
            on ${sensor}_too_low {
                ${alarm} = true;
            }
        }

        reflex ${sensor}_response_high {
            on ${sensor}_too_high {
                ${sensor}_debounced = true;
            }
        }

        property ${sensor}_alarm_correct {
            always (${sensor} < ${low} -> ${alarm});
        }
    }
}
"#
}

/// Ventilator module using monitor_sensor.
fn ventilator_source() -> String {
    format!(
        r#"
{monitor}
module ventilator {{
    signal airway_pressure: in  u16;
    signal heart_rate:      in  u16;
    signal pressure_alarm:  out bool;
    signal heartrate_alarm: out bool;

    monitor_sensor(airway_pressure, 50, 200, 1000, pressure_alarm);
    monitor_sensor(heart_rate, 40, 180, 500, heartrate_alarm);
}}
"#,
        monitor = monitor_sensor_source()
    )
}

// =========================================================================
// Category 1: Pattern Definition Parsing (10 tests)
// =========================================================================

mod sub1;
mod sub2;
