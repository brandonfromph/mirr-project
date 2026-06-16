//! Phase 4: Bit-width inference for MIRR expressions.
//!
//! Analyzes expression trees to assign minimum safe bit-widths to every node,
//! detects unsafe truncations, and emits clear diagnostics.
//!
//! Pipeline position: runs after Phase 3 simplification, before Verilog emit.

#![forbid(unsafe_code)]

pub mod constraint;
pub mod display;
#[allow(dead_code, deprecated)]
pub mod flatten;
pub mod graph;
pub mod scc;
pub mod scc_solver;
#[allow(dead_code, deprecated)]
pub mod solver;
pub mod types;
pub mod verify;

use serde::Serialize;
use types::{DiagSeverity, WidthDiag, WidthExpr, WidthStats};

// ---------------------------------------------------------------------------
// Public API: single expression
// ---------------------------------------------------------------------------

/// Result of width inference on a single MIRR expression or signal.
#[derive(Debug, Clone, Serialize)]
pub struct WidthInferenceResult {
    /// Width-annotated expression tree (None if flattening failed).
    pub expr: Option<WidthExpr>,
    /// Diagnostics emitted during inference.
    pub diagnostics: Vec<WidthDiag>,
    /// Statistics from this inference run.
    pub stats: WidthStats,
}

impl WidthInferenceResult {
    /// Returns true if any diagnostic is an error.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == DiagSeverity::Error)
    }
}

// Infer bit-widths for a single expression tree.
//
// `signals` provides the declared widths for Signal nodes.
//
// Pipeline: flatten -> generate constraints -> solve -> reconstruct.
// All steps are iterative with bounded loops.

// ---------------------------------------------------------------------------
// Public API: single-assignment truncation check (test + diagnostic use)
// ---------------------------------------------------------------------------

// Check a single assignment for unsafe truncation.
//
// Runs width inference on the RHS expression, then compares the inferred
// width against the target signal's declared width.
//
// The main pipeline uses [`infer_program_widths`] instead, which inlines this
// logic to accumulate per-node statistics. This entry point exists for
// integration tests and external tooling that need to check one assignment
// in isolation without constructing a full `MirrProgram`.
// ---------------------------------------------------------------------------
// Public API: full program
// ---------------------------------------------------------------------------

/// Result of width inference on an entire module.
#[derive(Debug, Clone, Serialize)]
pub struct ProgramWidthResult {
    /// Per-guard condition inference results (guard_name, result).
    pub guard_results: Vec<(String, WidthInferenceResult)>,
    /// Per-assignment inference results (reflex_name.target, result + truncation diags).
    pub assignment_results: Vec<(String, Vec<WidthDiag>)>,
    /// Aggregate statistics.
    pub stats: WidthStats,
}

impl ProgramWidthResult {
    /// Returns true if any diagnostic across the whole program is an error.
    pub fn has_errors(&self) -> bool {
        for (_, r) in &self.guard_results {
            if r.has_errors() {
                return true;
            }
        }
        for (_, diags) in &self.assignment_results {
            if diags.iter().any(|d| d.severity == DiagSeverity::Error) {
                return true;
            }
        }
        false
    }

    /// Collect all diagnostics across the entire program.
    pub fn all_diagnostics(&self) -> Vec<&WidthDiag> {
        let mut all: Vec<&WidthDiag> = Vec::new();
        for (_, r) in &self.guard_results {
            for d in &r.diagnostics {
                all.push(d);
            }
        }
        for (_, diags) in &self.assignment_results {
            for d in diags {
                all.push(d);
            }
        }
        all
    }
}

// Run width inference over an entire MIRR module.
//
// Infers widths for all guard conditions and all reflex assignment RHS,
// checking for truncations at every assignment site.
//
// Bounded: iterates over guards + reflexes (finite, from parsed program).
// ---------------------------------------------------------------------------
// Public API: Phase 4b — SCC-based width inference
// ---------------------------------------------------------------------------

/// Result of SCC-based width analysis on an entire module.
#[derive(Debug, Clone, Serialize)]
pub struct SccWidthResult {
    /// Phase 4a results (per-expression inference).
    pub phase4a: ProgramWidthResult,
    /// Detected non-trivial SCCs with classification.
    pub sccs: Vec<types::SccInfo>,
    /// Per-SCC solve results (parallel to `sccs`).
    pub scc_solves: Vec<(types::SccInfo, scc_solver::SccSolveResult)>,
    /// Least-solution verification result.
    pub verification: verify::VerifyResult,
    /// Aggregate statistics including SCC info.
    pub stats: WidthStats,
    /// All diagnostics from SCC analysis.
    pub scc_diagnostics: Vec<WidthDiag>,
    /// Signal names that belong to any detected SCC.
    /// Phase 4a truncation errors for these targets are suppressed
    /// because Phase 4b owns their width determination.
    pub scc_member_names: std::collections::HashSet<String>,
}

impl SccWidthResult {
    /// Returns true if any non-suppressed diagnostic is an error.
    ///
    /// Phase 4a truncation errors for signals in SCCs are suppressed:
    /// Phase 4b owns the width of SCC member signals, so Phase 4a's
    /// acyclic truncation analysis does not apply to them.
    pub fn has_errors(&self) -> bool {
        // Guard errors always count (guards are never SCC members).
        for (_, r) in &self.phase4a.guard_results {
            if r.has_errors() {
                return true;
            }
        }
        // Assignment errors: suppress truncation errors for SCC member targets.
        for (label, diags) in &self.phase4a.assignment_results {
            let target = label.split('.').nth(1).unwrap_or(label.as_str());
            let is_scc_member = self.scc_member_names.contains(target);
            for d in diags {
                if d.severity == DiagSeverity::Error {
                    if is_scc_member && d.message.contains("truncates") {
                        continue;
                    }
                    return true;
                }
            }
        }
        // SCC-level diagnostics.
        if self.scc_diagnostics.iter().any(|d| d.severity == DiagSeverity::Error) {
            return true;
        }
        // Verification diagnostics.
        if self.verification.diagnostics.iter().any(|d| d.severity == DiagSeverity::Error) {
            return true;
        }
        false
    }
}

pub fn check_truncation(
    target_name: &str,
    target_width: u32,
    expr_width: crate::width::types::Width,
    target_signed: bool,
) -> Vec<crate::width::types::WidthDiag> {
    let mut diags = Vec::new();
    if expr_width.0 > target_width {
        let category = if target_signed { "signed" } else { "unsigned" };
        diags.push(
            crate::width::types::WidthDiag::error(format!(
                "{} assignment to '{}' truncates {} {} bits to {} bits",
                crate::error_codes::ec(505),
                target_name,
                category,
                expr_width.0,
                target_width
            ))
            .with_code("E505")
            .with_signal(target_name),
        );
    }
    diags
}
#[cfg(test)]
mod tests {
    use super::*;

    // Unused width types removed
    use std::collections::HashSet;

    #[test]
    fn test_width_result_has_errors() {
        let mut pwr = ProgramWidthResult {
            guard_results: vec![],
            assignment_results: vec![],
            stats: WidthStats {
                nodes_analyzed: 0,
                propagation_rounds: 0,
                diagnostics_count: 0,
                scc_count: 0,
                expansive_count: 0,
                nonexpansive_count: 0,
            },
        };
        assert!(!pwr.has_errors());

        let bad_diag = WidthDiag::error("bad");
        pwr.guard_results.push((
            "g".to_string(),
            WidthInferenceResult {
                expr: None,
                diagnostics: vec![bad_diag.clone()],
                stats: WidthStats {
                    nodes_analyzed: 0,
                    propagation_rounds: 0,
                    diagnostics_count: 0,
                    scc_count: 0,
                    expansive_count: 0,
                    nonexpansive_count: 0,
                },
            },
        ));
        assert!(pwr.has_errors());
        assert_eq!(pwr.all_diagnostics().len(), 1);

        let pwr2 = ProgramWidthResult {
            guard_results: vec![],
            assignment_results: vec![("a.b".to_string(), vec![bad_diag.clone()])],
            stats: WidthStats {
                nodes_analyzed: 0,
                propagation_rounds: 0,
                diagnostics_count: 0,
                scc_count: 0,
                expansive_count: 0,
                nonexpansive_count: 0,
            },
        };
        assert!(pwr2.has_errors());
        assert_eq!(pwr2.all_diagnostics().len(), 1);
    }

    #[test]
    fn test_scc_width_result_has_errors() {
        let mut res = SccWidthResult {
            phase4a: ProgramWidthResult {
                guard_results: vec![],
                assignment_results: vec![],
                stats: WidthStats {
                    nodes_analyzed: 0,
                    propagation_rounds: 0,
                    diagnostics_count: 0,
                    scc_count: 0,
                    expansive_count: 0,
                    nonexpansive_count: 0,
                },
            },
            sccs: vec![],
            scc_solves: vec![],
            verification: crate::width::verify::VerifyResult {
                is_minimal: true,
                diagnostics: vec![],
            },
            stats: WidthStats {
                nodes_analyzed: 0,
                propagation_rounds: 0,
                diagnostics_count: 0,
                scc_count: 0,
                expansive_count: 0,
                nonexpansive_count: 0,
            },
            scc_diagnostics: vec![],
            scc_member_names: HashSet::new(),
        };
        assert!(!res.has_errors());

        // Add scc_diagnostics error
        res.scc_diagnostics.push(WidthDiag::error("scc err"));
        assert!(res.has_errors());
        res.scc_diagnostics.clear();

        // Add verification error
        res.verification.diagnostics.push(WidthDiag::error("verify err"));
        assert!(res.has_errors());
        res.verification.diagnostics.clear();

        // Add suppressed truncation error
        res.scc_member_names.insert("a".to_string());
        res.phase4a
            .assignment_results
            .push(("x.a".to_string(), vec![WidthDiag::error("truncates")]));
        assert!(!res.has_errors()); // suppressed

        // Add unsuppressed assignment error
        res.phase4a.assignment_results.push(("x.b".to_string(), vec![WidthDiag::error("bad")]));
        assert!(res.has_errors()); // not suppressed
    }

    // Test removed because `infer_program_widths_with_scc` was deleted during ECS migration.
}
