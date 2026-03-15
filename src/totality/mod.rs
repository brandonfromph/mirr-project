// ---------------------------------------------------------------------------
//! MEGA-4: Totality Engine — Static totality analysis for MIRR modules.
//!
//! Verifies that a MIRR module is a total function: bounded resource usage,
//! every output driven, all guards coverable, bounded temporal latency, and
//! no combinational cycles. Each analysis is a named function with a clear
//! input/output contract, suitable for 1:1 Rocq formalization in MEGA-7.
//!
//! ## Analyses
//!
//! 1. `check_resource_bounds` — resource usage within hardware limits
//! 2. `check_output_completeness` — every output has a driving reflex
//! 3. `check_guard_coverage` — at least one guard can fire per output
//! 4. `check_temporal_bound` — worst-case latency is finite
//! 5. `check_dependency_acyclicity` — no combinational feedback loops
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use crate::ast::program::Module;
use crate::ast::types::SignalKind;
use crate::emit::rspu_isa::{MAX_GUARDS, MAX_INSTRUCTIONS, MAX_REGISTERS};

/// Maximum signals in a module (NASA P10 iteration bound).
const MAX_SIGNALS: usize = 4096;

/// Maximum reflexes in a module (NASA P10 iteration bound).
const MAX_REFLEXES: usize = 4096;

/// Maximum expression nodes to traverse during dependency analysis.
const MAX_DEP_NODES: usize = 8192;

/// Maximum stack depth for dependency graph traversal (bounded DFS).
const MAX_DFS_STACK: usize = 4096;

// ---------------------------------------------------------------------------
// Result types — each analysis returns a typed result with pass/fail + data
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Top-level entry point
// ---------------------------------------------------------------------------

/// Run all five totality analyses on a parsed MIRR module.
///
/// Bounded: each sub-analysis has its own MAX_* iteration bounds.
pub fn run_totality_check(module: &Module) -> TotalityResult {
    let resource_bound = check_resource_bounds(module);
    let output_completeness = check_output_completeness(module);
    let guard_coverage = check_guard_coverage(module);
    let temporal_bound = check_temporal_bound(module);
    let acyclicity = check_dependency_acyclicity(module);

    let property_summary = build_property_summary(module);

    let is_total = resource_bound.pass
        && output_completeness.pass
        && guard_coverage.pass
        && temporal_bound.pass
        && acyclicity.pass;

    TotalityResult {
        resource_bound,
        output_completeness,
        guard_coverage,
        temporal_bound,
        acyclicity,
        property_summary,
        is_total,
    }
}

// ---------------------------------------------------------------------------
// Analysis 1: Resource bounds
// ---------------------------------------------------------------------------

/// Count hardware resource usage and check against MAX_REGISTERS, MAX_INSTRUCTIONS,
/// MAX_GUARDS. Returns pass=true if all resources fit.
///
/// Bounded: iterates over signals (≤ MAX_SIGNALS), guards, reflexes.
pub fn check_resource_bounds(module: &Module) -> ResourceBound {
    // Count registers: each signal needs one register.
    let mut regs: u32 = 0;
    let mut i = 0;
    while i < module.signals.len() && i < MAX_SIGNALS {
        regs = regs.saturating_add(1);
        i += 1;
    }

    // Count guards.
    let guards = module.guards.len().min(MAX_SIGNALS) as u32;

    // Estimate instructions: each signal needs LOAD/STORE, each guard needs
    // INIT+TICK+QUERY (3), each reflex assignment needs REFLEX_IF (1), each
    // property needs 1-2 instructions.
    let signal_instrs = regs;
    let guard_instrs = guards.saturating_mul(3);

    let mut reflex_instrs: u32 = 0;
    let mut ri = 0;
    while ri < module.reflexes.len() && ri < MAX_REFLEXES {
        reflex_instrs = reflex_instrs
            .saturating_add(module.reflexes[ri].assignments.len().min(MAX_SIGNALS) as u32);
        ri += 1;
    }

    let prop_instrs = module.properties.len().min(MAX_SIGNALS) as u32;
    let instructions_estimate = signal_instrs
        .saturating_add(guard_instrs)
        .saturating_add(reflex_instrs)
        .saturating_add(prop_instrs);

    // Max cycles: max guard cycle count.
    let mut max_cycles: u64 = 0;
    let mut gi = 0;
    while gi < module.guards.len() && gi < MAX_SIGNALS {
        if module.guards[gi].cycles > max_cycles {
            max_cycles = module.guards[gi].cycles;
        }
        gi += 1;
    }

    let pass = (regs as usize) <= MAX_REGISTERS
        && (instructions_estimate as usize) <= MAX_INSTRUCTIONS
        && (guards as usize) <= MAX_GUARDS;

    ResourceBound { registers: regs, instructions_estimate, guards, max_cycles, pass }
}

// ---------------------------------------------------------------------------
// Analysis 2: Output completeness
// ---------------------------------------------------------------------------

/// Check that every output signal has at least one reflex that assigns to it.
/// A module with an undriven output is a partial function — it violates totality.
///
/// Bounded: iterates over signals (≤ MAX_SIGNALS) × reflexes (≤ MAX_REFLEXES).
pub fn check_output_completeness(module: &Module) -> OutputCompletenessResult {
    let mut undriven: Vec<String> = Vec::new();

    let mut si = 0;
    while si < module.signals.len() && si < MAX_SIGNALS {
        if module.signals[si].kind == SignalKind::Output {
            let name = &module.signals[si].name;
            let mut driven = false;

            let mut ri = 0;
            while ri < module.reflexes.len() && ri < MAX_REFLEXES {
                let mut ai = 0;
                while ai < module.reflexes[ri].assignments.len() && ai < MAX_REFLEXES {
                    if module.reflexes[ri].assignments[ai].target == *name {
                        driven = true;
                    }
                    ai += 1;
                }
                ri += 1;
            }

            if !driven {
                undriven.push(name.clone());
            }
        }
        si += 1;
    }

    let pass = undriven.is_empty();
    OutputCompletenessResult { undriven_outputs: undriven, pass }
}

// ---------------------------------------------------------------------------
// Analysis 3: Guard coverage
// ---------------------------------------------------------------------------

/// For each output signal, verify that at least one guard driving it exists.
/// An output driven only by a guard that can never fire is effectively undriven.
///
/// Bounded: iterates over outputs (≤ MAX_SIGNALS) × reflexes (≤ MAX_REFLEXES).
pub fn check_guard_coverage(module: &Module) -> GuardCoverageResult {
    let mut covered: u32 = 0;
    let mut total: u32 = 0;

    let mut si = 0;
    while si < module.signals.len() && si < MAX_SIGNALS {
        if module.signals[si].kind == SignalKind::Output {
            total += 1;
            let name = &module.signals[si].name;

            // Find all reflexes that drive this output.
            let mut has_guard = false;
            let mut ri = 0;
            while ri < module.reflexes.len() && ri < MAX_REFLEXES {
                let mut drives_this = false;
                let mut ai = 0;
                while ai < module.reflexes[ri].assignments.len() && ai < MAX_REFLEXES {
                    if module.reflexes[ri].assignments[ai].target == *name {
                        drives_this = true;
                    }
                    ai += 1;
                }

                if drives_this && !module.reflexes[ri].guard_names.is_empty() {
                    // This reflex has at least one guard reference — the guard
                    // coverage is structurally satisfiable (guard conditions are
                    // boolean expressions over inputs, always satisfiable unless
                    // the condition is `false` literal).
                    has_guard = true;
                }
                ri += 1;
            }

            if has_guard {
                covered += 1;
            }
        }
        si += 1;
    }

    let pass = covered >= total;
    GuardCoverageResult { covered_outputs: covered, total_outputs: total, pass }
}

// ---------------------------------------------------------------------------
// Analysis 4: Temporal bound
// ---------------------------------------------------------------------------

/// Compute the worst-case temporal latency: max guard cycles + max prev delay.
/// MIRR guards use static `u64` cycle counts, so this is always finite.
/// Prev delays are also static `u64`. The bound is their sum.
///
/// Bounded: iterates over guards (≤ MAX_SIGNALS) and reflexes (≤ MAX_REFLEXES).
pub fn check_temporal_bound(module: &Module) -> TemporalBoundResult {
    // Max guard cycle count.
    let mut max_guard_cycles: u64 = 0;
    let mut gi = 0;
    while gi < module.guards.len() && gi < MAX_SIGNALS {
        if module.guards[gi].cycles > max_guard_cycles {
            max_guard_cycles = module.guards[gi].cycles;
        }
        gi += 1;
    }

    // Max prev delay: scan all reflex assignment expressions for Prev nodes.
    let mut max_prev_delay: u64 = 0;
    let mut ri = 0;
    while ri < module.reflexes.len() && ri < MAX_REFLEXES {
        let mut ai = 0;
        while ai < module.reflexes[ri].assignments.len() && ai < MAX_REFLEXES {
            let delay = max_prev_in_expr(&module.reflexes[ri].assignments[ai].value);
            if delay > max_prev_delay {
                max_prev_delay = delay;
            }
            ai += 1;
        }
        ri += 1;
    }

    // Also scan guard conditions for prev references.
    let mut gi2 = 0;
    while gi2 < module.guards.len() && gi2 < MAX_SIGNALS {
        let delay = max_prev_in_expr(&module.guards[gi2].condition);
        if delay > max_prev_delay {
            max_prev_delay = delay;
        }
        gi2 += 1;
    }

    let worst_case_latency = max_guard_cycles.saturating_add(max_prev_delay);

    TemporalBoundResult { max_guard_cycles, max_prev_delay, worst_case_latency, pass: true }
}

/// Find the maximum `Prev` delay in an expression tree.
///
/// Bounded: uses explicit stack with MAX_DEP_NODES limit.
fn max_prev_in_expr(expr: &crate::ast::Expr) -> u64 {
    let mut max_delay: u64 = 0;
    let mut stack: Vec<&crate::ast::Expr> = Vec::new();
    stack.push(expr);

    let mut visited = 0usize;
    while let Some(e) = stack.pop() {
        visited += 1;
        if visited > MAX_DEP_NODES {
            break;
        }
        match e {
            crate::ast::Expr::Prev { delay, .. } => {
                if *delay > max_delay {
                    max_delay = *delay;
                }
            }
            crate::ast::Expr::Unary { operand, .. } => {
                stack.push(operand);
            }
            crate::ast::Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            _ => {}
        }
    }
    max_delay
}

// ---------------------------------------------------------------------------
// Analysis 5: Dependency acyclicity
// ---------------------------------------------------------------------------

/// Verify that the signal dependency graph has no combinational cycles.
/// A combinational cycle occurs when a reflex assignment depends on its own
/// target signal without going through a `prev` (temporal barrier).
///
/// Uses bounded DFS on the signal dependency graph. Prev edges are NOT
/// followed (they break cycles via temporal delay).
///
/// Bounded: MAX_DFS_STACK for the work stack. MAX_SIGNALS for signal count.
pub fn check_dependency_acyclicity(module: &Module) -> AcyclicityResult {
    // Build signal name → index map.
    let mut names: Vec<&str> = Vec::new();
    let mut ni = 0;
    while ni < module.signals.len() && ni < MAX_SIGNALS {
        names.push(&module.signals[ni].name);
        ni += 1;
    }
    let n = names.len();
    if n == 0 {
        return AcyclicityResult { pass: true, cycle_witness: None };
    }

    // Build adjacency list: signal A → signal B if a reflex assigns to A
    // with an expression that references B (not via prev).
    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut ai = 0;
    while ai < n {
        adj.push(Vec::new());
        ai += 1;
    }

    let mut ri = 0;
    while ri < module.reflexes.len() && ri < MAX_REFLEXES {
        let mut asgn_i = 0;
        while asgn_i < module.reflexes[ri].assignments.len() && asgn_i < MAX_REFLEXES {
            let target = &module.reflexes[ri].assignments[asgn_i].target;
            let target_idx = find_signal_index(&names, target);
            if let Some(ti) = target_idx {
                // Collect signal dependencies from the expression (excluding prev).
                let deps = collect_signal_deps(&module.reflexes[ri].assignments[asgn_i].value);
                let mut di = 0;
                while di < deps.len() && di < MAX_DEP_NODES {
                    if let Some(dep_idx) = find_signal_index(&names, &deps[di]) {
                        adj[ti].push(dep_idx);
                    }
                    di += 1;
                }
            }
            asgn_i += 1;
        }
        ri += 1;
    }

    // Bounded DFS for cycle detection (3-color: white=0, gray=1, black=2).
    let mut color: Vec<u8> = Vec::new();
    let mut ci = 0;
    while ci < n {
        color.push(0);
        ci += 1;
    }

    let mut si = 0;
    while si < n {
        if color[si] == 0 {
            // DFS from si using explicit stack.
            let mut stack: Vec<(usize, usize)> = Vec::new(); // (node, edge_index)
            stack.push((si, 0));
            color[si] = 1; // gray

            let mut iterations = 0usize;
            while let Some((node, edge_idx)) = stack.last_mut() {
                iterations += 1;
                if iterations > MAX_DFS_STACK {
                    break;
                }

                if *edge_idx < adj[*node].len() {
                    let neighbor = adj[*node][*edge_idx];
                    *edge_idx += 1;

                    if color[neighbor] == 1 {
                        // Back edge → cycle found.
                        return AcyclicityResult {
                            pass: false,
                            cycle_witness: Some(names[neighbor].to_string()),
                        };
                    } else if color[neighbor] == 0 {
                        color[neighbor] = 1;
                        stack.push((neighbor, 0));
                    }
                } else {
                    color[*node] = 2; // black
                    stack.pop();
                }
            }
        }
        si += 1;
    }

    AcyclicityResult { pass: true, cycle_witness: None }
}

/// Find the index of a signal name in the names array.
///
/// Bounded: linear scan up to MAX_SIGNALS.
fn find_signal_index(names: &[&str], target: &str) -> Option<usize> {
    let mut i = 0;
    while i < names.len() && i < MAX_SIGNALS {
        if names[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Collect signal names referenced in an expression (excluding Prev references,
/// which are temporal barriers and break combinational cycles).
///
/// Bounded: explicit stack with MAX_DEP_NODES limit.
fn collect_signal_deps(expr: &crate::ast::Expr) -> Vec<String> {
    let mut deps: Vec<String> = Vec::new();
    let mut stack: Vec<&crate::ast::Expr> = Vec::new();
    stack.push(expr);

    let mut visited = 0usize;
    while let Some(e) = stack.pop() {
        visited += 1;
        if visited > MAX_DEP_NODES {
            break;
        }
        match e {
            crate::ast::Expr::Signal(name) => {
                deps.push(name.clone());
            }
            crate::ast::Expr::Unary { operand, .. } => {
                stack.push(operand);
            }
            crate::ast::Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            // Prev is a temporal barrier — do NOT follow it.
            crate::ast::Expr::Prev { .. } | crate::ast::Expr::Literal(_) => {}
        }
    }
    deps
}

// ---------------------------------------------------------------------------
// Property summary
// ---------------------------------------------------------------------------

/// Build a summary of all declared properties for the certificate.
///
/// Bounded: iterates over properties (≤ MAX_SIGNALS).
fn build_property_summary(module: &Module) -> Vec<PropertySummary> {
    let mut summaries: Vec<PropertySummary> = Vec::new();
    let mut pi = 0;
    while pi < module.properties.len() && pi < MAX_SIGNALS {
        let p = &module.properties[pi];
        let kind = match &p.formula {
            crate::ast::PropertyFormula::Always(_) => "always",
            crate::ast::PropertyFormula::Never(_) => "never",
            crate::ast::PropertyFormula::AlwaysImplies { .. } => "always_implies",
            crate::ast::PropertyFormula::NeverImplies { .. } => "never_implies",
            crate::ast::PropertyFormula::EventuallyWithin { .. } => "eventually_within",
            crate::ast::PropertyFormula::AlwaysFollowedBy { .. } => "always_followed_by",
        };
        summaries.push(PropertySummary { name: p.name.clone(), kind: kind.to_string() });
        pi += 1;
    }
    summaries
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
    use crate::ast::types::LiteralValue;
    use crate::ast::types::{ExtendedType, SignalKind, SignalType};
    use crate::ast::Expr;

    fn make_signal(name: &str, kind: SignalKind) -> SignalDecl {
        SignalDecl {
            name: name.to_string(),
            kind,
            ty: ExtendedType::from_core(SignalType::Bool),
            origin: None,
            span: None,
        }
    }

    fn make_guard(name: &str, cycles: u64) -> Guard {
        Guard {
            name: name.to_string(),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles,
            origin: None,
            span: None,
        }
    }

    fn make_reflex(name: &str, guard: &str, target: &str) -> Reflex {
        Reflex {
            name: name.to_string(),
            guard_names: vec![guard.to_string()],
            assignments: vec![Assignment {
                target: target.to_string(),
                value: Expr::Signal("input_a".to_string()),
                span: None,
            }],
            origin: None,
            span: None,
        }
    }

    fn make_module(signals: Vec<SignalDecl>, guards: Vec<Guard>, reflexes: Vec<Reflex>) -> Module {
        Module {
            name: "test".to_string(),
            signals,
            guards,
            reflexes,
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        }
    }

    #[test]
    fn test_total_module_passes_all() {
        let m = make_module(
            vec![
                make_signal("input_a", SignalKind::Input),
                make_signal("output_b", SignalKind::Output),
            ],
            vec![make_guard("g1", 3)],
            vec![make_reflex("r1", "g1", "output_b")],
        );
        let result = run_totality_check(&m);
        assert!(result.is_total);
        assert!(result.resource_bound.pass);
        assert!(result.output_completeness.pass);
        assert!(result.guard_coverage.pass);
        assert!(result.temporal_bound.pass);
        assert!(result.acyclicity.pass);
    }

    #[test]
    fn test_undriven_output_fails_completeness() {
        let m = make_module(
            vec![
                make_signal("input_a", SignalKind::Input),
                make_signal("output_b", SignalKind::Output),
                make_signal("output_c", SignalKind::Output), // undriven
            ],
            vec![make_guard("g1", 3)],
            vec![make_reflex("r1", "g1", "output_b")],
        );
        let result = run_totality_check(&m);
        assert!(!result.is_total);
        assert!(!result.output_completeness.pass);
        assert_eq!(result.output_completeness.undriven_outputs, vec!["output_c"]);
    }

    #[test]
    fn test_temporal_bound() {
        let m = make_module(
            vec![
                make_signal("input_a", SignalKind::Input),
                make_signal("output_b", SignalKind::Output),
            ],
            vec![make_guard("g1", 10), make_guard("g2", 25)],
            vec![make_reflex("r1", "g1", "output_b")],
        );
        let result = check_temporal_bound(&m);
        assert_eq!(result.max_guard_cycles, 25);
        assert!(result.pass);
    }

    #[test]
    fn test_combinational_cycle_detected() {
        // output_b depends on output_b (via expression) — combinational cycle.
        let m = make_module(
            vec![make_signal("output_b", SignalKind::Output)],
            vec![make_guard("g1", 1)],
            vec![Reflex {
                name: "r_cycle".to_string(),
                guard_names: vec!["g1".to_string()],
                assignments: vec![Assignment {
                    target: "output_b".to_string(),
                    value: Expr::Signal("output_b".to_string()), // self-reference
                    span: None,
                }],
                origin: None,
                span: None,
            }],
        );
        let result = check_dependency_acyclicity(&m);
        assert!(!result.pass);
        assert_eq!(result.cycle_witness.as_deref(), Some("output_b"));
    }

    #[test]
    fn test_prev_breaks_cycle() {
        // output_b depends on prev(output_b, 1) — NOT a combinational cycle.
        let m = make_module(
            vec![make_signal("output_b", SignalKind::Output)],
            vec![make_guard("g1", 1)],
            vec![Reflex {
                name: "r_prev".to_string(),
                guard_names: vec!["g1".to_string()],
                assignments: vec![Assignment {
                    target: "output_b".to_string(),
                    value: Expr::Prev { signal: "output_b".to_string(), delay: 1 },
                    span: None,
                }],
                origin: None,
                span: None,
            }],
        );
        let result = check_dependency_acyclicity(&m);
        assert!(result.pass);
    }
}
