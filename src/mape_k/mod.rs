//! MAPE-K Simulation Harness — Phase 5 of the MIRR/R-SPU roadmap.
//!
//! Orchestrates the Monitor–Analyze–Plan–Execute–Knowledge feedback loop
//! for simulating safety-critical adaptive systems.
//!
//! # Architecture
//!
//! ```text
//! ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
//! │ Monitor  │ → │ Analyze  │ → │   Plan   │ → │ Execute  │
//! │ (sample  │   │ (LTL     │   │ (select  │   │ (apply   │
//! │  sensors)│   │  check)  │   │  action) │   │  action) │
//! └──────────┘   └──────────┘   └──────────┘   └──────────┘
//!       ↑                                            │
//!       └────────── Knowledge Base ←─────────────────┘
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! let config = SimConfig { ... };
//! let mut sim = MapeKSimulator::new(config);
//! let result = sim.run(10_000);  // run for 10,000 ticks
//! println!("{}", result.summary());
//! ```

#![forbid(unsafe_code)]

pub mod analyzer;
pub mod executor;
pub mod knowledge;
pub mod ltl;
pub mod monitor;
pub mod planner;
pub mod sensor;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Re-exports for public API
// ---------------------------------------------------------------------------

pub use analyzer::Analyzer;
pub use executor::{ExecutionRecord, Executor};
pub use knowledge::{AdaptationRecord, KnowledgeBase};
pub use ltl::{PropertyResult, SignalPredicate, TemporalProperty};
pub use monitor::{Monitor, RingBuffer};
pub use planner::{ActionEntry, AdaptationAction, PlanResult, Planner};
pub use sensor::{SensorConfig, SensorModel};

// ---------------------------------------------------------------------------
// Simulation configuration
// ---------------------------------------------------------------------------

/// Complete configuration for a MAPE-K simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    /// Sensor channel configurations.
    pub sensors: Vec<SensorConfig>,
    /// Temporal properties to check each tick.
    pub properties: Vec<TemporalProperty>,
    /// Action table entries mapping violations to actions.
    pub action_table: Vec<ActionEntry>,
    /// Rolling window size for the monitor.
    pub window_size: usize,
    /// Knowledge base capacity.
    pub knowledge_capacity: usize,
}

// ---------------------------------------------------------------------------
// Simulation result
// ---------------------------------------------------------------------------

/// Result of a completed MAPE-K simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimResult {
    /// Total ticks simulated.
    pub total_ticks: u64,
    /// Total property violations detected.
    pub total_violations: u64,
    /// Total adaptation actions executed.
    pub total_adaptations: u64,
    /// Whether an emergency stop was triggered.
    pub emergency_triggered: bool,
    /// Tick at which emergency was triggered (if any).
    pub emergency_tick: Option<u64>,
    /// Final signal state.
    pub final_signal_state: Vec<(String, u64)>,
    /// Adaptation log (from knowledge base).
    pub adaptation_log: Vec<AdaptationRecord>,
}

impl SimResult {
    /// Generate a human-readable summary of the simulation.
    pub fn summary(&self) -> String {
        let mut s = String::with_capacity(256);
        s.push_str(&format!("MAPE-K Simulation: {} ticks\n", self.total_ticks));
        s.push_str(&format!("  Violations detected: {}\n", self.total_violations));
        s.push_str(&format!("  Adaptations applied: {}\n", self.total_adaptations));
        if self.emergency_triggered {
            s.push_str(&format!(
                "  EMERGENCY STOP at tick {}\n",
                self.emergency_tick.unwrap_or(0)
            ));
        }
        s.push_str(&format!(
            "  Adaptation log entries: {}\n",
            self.adaptation_log.len()
        ));
        s
    }
}

// ---------------------------------------------------------------------------
// Simulator — the orchestrator
// ---------------------------------------------------------------------------

/// Maximum ticks per simulation run (bounded, NASA P10 compliance).
pub const MAX_TICKS: u64 = 10_000_000;

/// The MAPE-K Simulator: orchestrates the full feedback loop.
pub struct MapeKSimulator {
    sensors: Vec<SensorModel>,
    monitor: Monitor,
    analyzer: Analyzer,
    planner: Planner,
    executor: Executor,
    knowledge: KnowledgeBase,
    /// Mutable signal environment (signal name -> u64 value).
    signal_env: HashMap<String, u64>,
    /// Running counters.
    total_violations: u64,
    total_adaptations: u64,
    emergency_triggered: bool,
    emergency_tick: Option<u64>,
}

impl MapeKSimulator {
    /// Create a new simulator from configuration.
    pub fn new(config: SimConfig) -> Self {
        // Build sensor models.
        let sensors: Vec<SensorModel> = config.sensors.iter()
            .map(|c| SensorModel::new(c.clone()))
            .collect();

        // Collect signal names for the monitor.
        let signal_names: Vec<String> = config.sensors.iter()
            .map(|c| c.name.clone())
            .collect();
        let signal_name_refs: Vec<&str> = signal_names.iter()
            .map(|s| s.as_str())
            .collect();

        let monitor = Monitor::new(config.window_size, &signal_name_refs);
        let analyzer = Analyzer::new(config.properties);
        let planner = Planner::new(config.action_table);
        let executor = Executor::new(signal_names.clone());
        let knowledge = KnowledgeBase::new(config.knowledge_capacity);

        // Initialize signal environment.
        let mut signal_env = HashMap::with_capacity(signal_names.len());
        for name in &signal_names {
            signal_env.insert(name.clone(), 0);
        }

        Self {
            sensors,
            monitor,
            analyzer,
            planner,
            executor,
            knowledge,
            signal_env,
            total_violations: 0,
            total_adaptations: 0,
            emergency_triggered: false,
            emergency_tick: None,
        }
    }

    /// Run the simulation for the given number of ticks.
    /// Ticks are clamped to MAX_TICKS.
    pub fn run(&mut self, ticks: u64) -> SimResult {
        let max_ticks = ticks.min(MAX_TICKS);

        for _ in 0..max_ticks {
            self.tick();

            // If emergency stop was triggered, halt the simulation.
            if self.emergency_triggered {
                break;
            }
        }

        self.build_result()
    }

    /// Execute a single MAPE-K tick.
    fn tick(&mut self) {
        let current_tick = self.monitor.tick();

        // M — Monitor: sample all sensors.
        for sensor in &mut self.sensors {
            let value = sensor.sample();
            self.monitor.record_sample(sensor.name(), value);
            self.signal_env.insert(sensor.name().to_string(), value);
        }

        // A — Analyze: check all temporal properties.
        let violations = self.analyzer.violations(&self.monitor);

        if !violations.is_empty() {
            self.total_violations = self.total_violations
                .wrapping_add(violations.len() as u64);

            // P — Plan: select the best action for these violations.
            let plan = self.planner.select(&violations);

            // E — Execute: apply the selected action.
            if let Some(ref action) = plan.action {
                let exec_record = self.executor.apply(action, &mut self.signal_env);
                let trigger_idx = plan.trigger_property_idx.unwrap_or(0);

                // K — Knowledge: record the adaptation.
                let adaptation = AdaptationRecord::from_execution(
                    current_tick,
                    trigger_idx,
                    &action.label(),
                    &exec_record,
                );
                self.knowledge.record(adaptation);
                self.total_adaptations = self.total_adaptations.wrapping_add(1);

                // Track emergency stop.
                if self.executor.is_emergency_active() && !self.emergency_triggered {
                    self.emergency_triggered = true;
                    self.emergency_tick = Some(current_tick);
                }
            }
        }

        // Advance monitor tick.
        self.monitor.advance_tick();
    }

    fn build_result(&self) -> SimResult {
        let final_state: Vec<(String, u64)> = self.signal_env.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        SimResult {
            total_ticks: self.monitor.tick(),
            total_violations: self.total_violations,
            total_adaptations: self.total_adaptations,
            emergency_triggered: self.emergency_triggered,
            emergency_tick: self.emergency_tick,
            final_signal_state: final_state,
            adaptation_log: self.knowledge.records().to_vec(),
        }
    }
}
