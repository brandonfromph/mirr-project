//! Result types returned by each totality analysis.

#![forbid(unsafe_code)]

/// Aggregate result from all five totality analyses.
#[derive(Debug, Clone)]
pub struct TotalityResult {
    /// Resource bound analysis result.
    pub resource_bound: ResourceBound,
    /// Output completeness analysis result.
    pub output_completeness: OutputCompletenessResult,
    /// Guard coverage analysis result.
    pub guard_coverage: GuardCoverageResult,
    /// Temporal bound analysis result.
    pub temporal_bound: TemporalBoundResult,
    /// Dependency acyclicity analysis result.
    pub acyclicity: AcyclicityResult,
    /// Summary of all declared properties.
    pub property_summary: Vec<PropertySummary>,
    /// True if all five analyses pass.
    pub is_total: bool,
}

/// Hardware resource usage and whether each fits within hardware limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceBound {
    /// Number of registers needed (input + output + internal + temps).
    pub registers: u32,
    /// Estimated instruction count (signals + guards + reflexes + properties).
    pub instructions_estimate: u32,
    /// Number of guard hardware units needed.
    pub guards: u32,
    /// Maximum cycle count (from guard analysis).
    pub max_cycles: u64,
    /// True if all resources fit within hardware limits.
    pub pass: bool,
}

/// Whether every output signal has at least one driving reflex.
#[derive(Debug, Clone)]
pub struct OutputCompletenessResult {
    /// Output signals with no driving reflex (partial function).
    pub undriven_outputs: Vec<String>,
    /// True if every output is driven.
    pub pass: bool,
}

/// Whether each output's guard disjunction is satisfiable.
#[derive(Debug, Clone)]
pub struct GuardCoverageResult {
    /// Number of outputs with at least one coverable guard.
    pub covered_outputs: u32,
    /// Number of outputs checked.
    pub total_outputs: u32,
    /// True if all outputs have at least one coverable guard.
    pub pass: bool,
}

/// Worst-case temporal latency.
#[derive(Debug, Clone)]
pub struct TemporalBoundResult {
    /// Maximum guard cycle count across all guards.
    pub max_guard_cycles: u64,
    /// Maximum prev delay chain length.
    pub max_prev_delay: u64,
    /// Total worst-case latency: max_guard_cycles + max_prev_delay.
    pub worst_case_latency: u64,
    /// True always (latency ≤ u64::MAX is definitionally bounded for MIRR).
    pub pass: bool,
}

/// Whether the signal dependency graph is acyclic.
#[derive(Debug, Clone)]
pub struct AcyclicityResult {
    /// True if no combinational cycles found.
    pub pass: bool,
    /// If a cycle exists, one signal name on the cycle.
    pub cycle_witness: Option<String>,
}

/// Summary of a declared property.
#[derive(Debug, Clone)]
pub struct PropertySummary {
    /// Property name.
    pub name: String,
    /// Property formula kind (Always, Never, etc.).
    pub kind: String,
}
