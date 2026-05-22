//! Sensor extraction: convert signal declarations into `SensorConfig` entries.

#![forbid(unsafe_code)]

use crate::ast::types::{SignalKind, SignalType};
use crate::mape_k::error::MapeKError;
use crate::mape_k::sensor::SensorConfig;
use crate::pipeline::PipelineResult;

use super::MAX_BRIDGE_SIGNALS;

/// Default noise amplitude for heuristic sensor generation.
pub(super) const DEFAULT_NOISE_AMPLITUDE: u64 = 2;

/// Default PRNG seed base (each sensor gets `SEED_BASE + index`).
const SEED_BASE: u64 = 1000;

/// Walk the program's signal declarations and produce a `SensorConfig`
/// for each one. All signals become sensors; `is_observable` is true
/// only for Input signals (outputs and internals are design-driven).
///
/// Heuristic defaults:
/// - `Bool`: base_value = 1, noise = 0 (deterministic toggle)
/// - `Unsigned(w)`: base_value = midpoint of [0, 2^w - 1], noise = 2
/// - `Signed(w)`: base_value = 0, noise = 2
pub(super) fn extract_sensors(
    result: &PipelineResult,
    errors: &mut Vec<MapeKError>,
) -> Vec<SensorConfig> {
    let signals = &result.program.module.signals;

    let signal_count = signals.len().min(MAX_BRIDGE_SIGNALS.saturating_add(1));
    if signal_count > MAX_BRIDGE_SIGNALS {
        errors.push(MapeKError::BridgeConfigError(format!(
            "too many signals: {} > {}",
            signal_count, MAX_BRIDGE_SIGNALS
        )));
        return Vec::new();
    }

    let mut sensors = Vec::with_capacity(signal_count);
    let mut idx: usize = 0;

    for sig in signals.iter().take(MAX_BRIDGE_SIGNALS) {
        let (base_value, noise_amplitude) = heuristic_sensor_defaults(&sig.ty.core);

        sensors.push(SensorConfig {
            name: sig.name.clone(),
            base_value,
            noise_amplitude,
            fault_at_tick: None,
            fault_value: 0,
            fault_end_tick: None,
            seed: SEED_BASE.wrapping_add(idx as u64),
            is_observable: sig.kind == SignalKind::Input,
        });

        idx = idx.saturating_add(1);
        if idx >= MAX_BRIDGE_SIGNALS {
            break;
        }
    }

    sensors
}

/// Compute heuristic `(base_value, noise_amplitude)` for a given signal type.
pub(super) fn heuristic_sensor_defaults(ty: &SignalType) -> (u64, u64) {
    match ty {
        SignalType::Bool => (1, 0),
        SignalType::Unsigned(width) => {
            let max_val = max_unsigned_value(*width);
            let midpoint = max_val / 2;
            (midpoint, DEFAULT_NOISE_AMPLITUDE.min(midpoint))
        }
        SignalType::Signed(width) => {
            let half = max_unsigned_value(width.saturating_sub(1));
            (0, DEFAULT_NOISE_AMPLITUDE.min(half))
        }
        SignalType::Array { .. }
        | SignalType::Struct { .. }
        | SignalType::FixedPoint { .. }
        | SignalType::Bundle(_)
        | SignalType::Fifo { .. } => (0, 0),
    }
}

/// Maximum unsigned value for a given bit-width, clamped to avoid overflow.
pub(super) fn max_unsigned_value(width: u32) -> u64 {
    if width == 0 {
        return 0;
    }
    if width >= 64 {
        return u64::MAX;
    }
    (1u64 << width).wrapping_sub(1)
}
