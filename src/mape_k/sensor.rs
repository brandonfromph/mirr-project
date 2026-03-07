//! Deterministic stochastic sensor model for MAPE-K simulation.
//!
//! Generates time-series sensor data with configurable noise and fault
//! injection. Uses a simple LCG PRNG seeded deterministically so that
//! identical seeds produce identical output across runs (reproducibility).
//!
//! No external crates. No heap allocation in the tick loop.
//! All loops bounded by configuration parameters.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// LCG PRNG — deterministic, no heap, no external crate
// ---------------------------------------------------------------------------

/// Minimal Linear Congruential Generator (Numerical Recipes constants).
/// Deterministic: same seed always produces the same sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lcg {
    state: u64,
}

impl Lcg {
    pub fn new(seed: u64) -> Self {
        // Avoid zero state which produces a degenerate sequence.
        Self { state: seed | 1 }
    }

    /// Advance the LCG and return the next pseudo-random u64.
    pub fn next_u64(&mut self) -> u64 {
        // Knuth LCG constants (period 2^64).
        self.state = self.state.wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }

    /// Return a value in [0, bound) using rejection-free modular reduction.
    /// Returns 0 if bound is 0.
    pub fn next_bounded(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            return 0;
        }
        self.next_u64() % bound
    }
}

// ---------------------------------------------------------------------------
// Sensor configuration
// ---------------------------------------------------------------------------

/// Configuration for a single sensor channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorConfig {
    /// Human-readable name (e.g., "airway_pressure").
    pub name: String,
    /// Baseline value around which noise is centered.
    pub base_value: u64,
    /// Maximum noise amplitude (+/- from base_value).
    /// Output is clamped to [0, u64::MAX].
    pub noise_amplitude: u64,
    /// Optional: inject a step-fault at this tick number.
    /// The sensor output snaps to `fault_value` from this tick onward
    /// until `fault_end_tick` (if set) or forever.
    pub fault_at_tick: Option<u64>,
    /// Value to output during fault condition.
    pub fault_value: u64,
    /// Optional: tick at which the fault clears and normal output resumes.
    pub fault_end_tick: Option<u64>,
    /// Deterministic PRNG seed for this sensor channel.
    pub seed: u64,
}

// ---------------------------------------------------------------------------
// Sensor model instance
// ---------------------------------------------------------------------------

/// A running sensor model that produces one sample per tick.
#[derive(Debug, Clone)]
pub struct SensorModel {
    config: SensorConfig,
    rng: Lcg,
    current_tick: u64,
}

impl SensorModel {
    /// Create a new sensor model from configuration.
    pub fn new(config: SensorConfig) -> Self {
        let rng = Lcg::new(config.seed);
        Self {
            config,
            rng,
            current_tick: 0,
        }
    }

    /// Return the sensor's name.
    pub fn name(&self) -> &str {
        &self.config.name
    }

    /// Return the current tick.
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Sample the sensor for the current tick, then advance.
    ///
    /// If a fault is active at the current tick, returns `fault_value`.
    /// Otherwise returns `base_value +/- noise` (clamped to u64 range).
    pub fn sample(&mut self) -> u64 {
        let tick = self.current_tick;
        self.current_tick = self.current_tick.wrapping_add(1);

        // Check fault window.
        if let Some(fault_start) = self.config.fault_at_tick {
            if tick >= fault_start {
                let fault_ended = self.config.fault_end_tick
                    .map(|end| tick >= end)
                    .unwrap_or(false);
                if !fault_ended {
                    return self.config.fault_value;
                }
            }
        }

        // Normal operation: base +/- noise.
        if self.config.noise_amplitude == 0 {
            return self.config.base_value;
        }

        let noise_range = self.config.noise_amplitude.saturating_mul(2).saturating_add(1);
        let noise_offset = self.rng.next_bounded(noise_range);
        // noise_offset is in [0, 2*amplitude]. Subtract amplitude to center.
        let base = self.config.base_value;
        let amp = self.config.noise_amplitude;

        if noise_offset >= amp {
            base.saturating_add(noise_offset.wrapping_sub(amp))
        } else {
            base.saturating_sub(amp.wrapping_sub(noise_offset))
        }
    }

    /// Reset the sensor to its initial state (tick 0, original seed).
    pub fn reset(&mut self) {
        self.rng = Lcg::new(self.config.seed);
        self.current_tick = 0;
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn pressure_config(seed: u64) -> SensorConfig {
        SensorConfig {
            name: "airway_pressure".to_string(),
            base_value: 120,
            noise_amplitude: 5,
            fault_at_tick: None,
            fault_value: 0,
            fault_end_tick: None,
            seed,
        }
    }

    #[test]
    fn deterministic_same_seed() {
        let mut s1 = SensorModel::new(pressure_config(42));
        let mut s2 = SensorModel::new(pressure_config(42));
        for _ in 0..100 {
            assert_eq!(s1.sample(), s2.sample());
        }
    }

    #[test]
    fn different_seed_different_output() {
        let mut s1 = SensorModel::new(pressure_config(42));
        let mut s2 = SensorModel::new(pressure_config(99));
        let mut any_different = false;
        for _ in 0..20 {
            if s1.sample() != s2.sample() {
                any_different = true;
                break;
            }
        }
        assert!(any_different, "different seeds should produce different sequences");
    }

    #[test]
    fn noise_stays_in_range() {
        let mut s = SensorModel::new(pressure_config(7));
        for _ in 0..1000 {
            let v = s.sample();
            assert!((115..=125).contains(&v), "value {v} outside expected range");
        }
    }

    #[test]
    fn fault_injection() {
        let cfg = SensorConfig {
            name: "test".to_string(),
            base_value: 100,
            noise_amplitude: 5,
            fault_at_tick: Some(10),
            fault_value: 0,
            fault_end_tick: None,
            seed: 1,
        };
        let mut s = SensorModel::new(cfg);
        // First 10 ticks: normal (around 100).
        for _ in 0..10 {
            let v = s.sample();
            assert!((95..=105).contains(&v), "pre-fault value {v} unexpected");
        }
        // From tick 10 onward: fault value = 0.
        for _ in 10..20 {
            assert_eq!(s.sample(), 0);
        }
    }

    #[test]
    fn fault_with_end_tick() {
        let cfg = SensorConfig {
            name: "test".to_string(),
            base_value: 100,
            noise_amplitude: 0,
            fault_at_tick: Some(5),
            fault_value: 999,
            fault_end_tick: Some(8),
            seed: 1,
        };
        let mut s = SensorModel::new(cfg);
        for _ in 0..5 { assert_eq!(s.sample(), 100); }
        for _ in 5..8 { assert_eq!(s.sample(), 999); }
        // After fault_end_tick: normal again.
        assert_eq!(s.sample(), 100); // tick 8
    }

    #[test]
    fn zero_noise_returns_base() {
        let cfg = SensorConfig {
            name: "steady".to_string(),
            base_value: 42,
            noise_amplitude: 0,
            fault_at_tick: None,
            fault_value: 0,
            fault_end_tick: None,
            seed: 123,
        };
        let mut s = SensorModel::new(cfg);
        for _ in 0..100 {
            assert_eq!(s.sample(), 42);
        }
    }

    #[test]
    fn reset_reproduces_sequence() {
        let mut s = SensorModel::new(pressure_config(77));
        let first_run: Vec<u64> = (0..50).map(|_| s.sample()).collect();
        s.reset();
        let second_run: Vec<u64> = (0..50).map(|_| s.sample()).collect();
        assert_eq!(first_run, second_run);
    }
}
