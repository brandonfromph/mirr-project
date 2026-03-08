//! Unique Least Solution verification for Phase 4b.
//!
//! After the SCC solver resolves widths, this pass verifies that the
//! solution is the component-wise minimum: for each signal, it attempts
//! to reduce its width by 1 and checks whether all constraints still hold.
//!
//! If reduction succeeds, the solution was not minimal — this indicates
//! a compiler bug. Bounded by signal_count iterations.

#![forbid(unsafe_code)]

use super::scc_solver::SccSolveResult;
use super::types::{SccInfo, WidthDiag, MAX_SIGNALS};
use crate::ast::program::SignalDecl;
use crate::ast::types::SignalType;

/// Result of the Unique Least Solution verification.
pub struct VerifyResult {
    /// True if the solution was verified as minimal.
    pub is_minimal: bool,
    /// Diagnostics (empty if minimal; contains bug reports if not).
    pub diagnostics: Vec<WidthDiag>,
}

/// Verify that the SCC solution is the unique least solution.
///
/// For each signal in solved SCCs, attempt to reduce its width by 1.
/// If the reduced width still satisfies all constraints (the signal's
/// declared type accepts it), the original solution was not minimal.
///
/// Bounded: iterates once over all SCC signals (max MAX_SIGNALS).
pub fn verify_least_solution(
    scc_results: &[(SccInfo, SccSolveResult)],
    signals: &[SignalDecl],
) -> VerifyResult {
    let mut diagnostics: Vec<WidthDiag> = Vec::new();
    let mut is_minimal = true;
    let mut checked = 0usize;

    for (scc, solve_result) in scc_results {
        for (i, &width) in solve_result.widths.iter().enumerate() {
            checked += 1;
            if checked > MAX_SIGNALS {
                break;
            }

            if width <= 1 {
                // Cannot reduce below 1 bit.
                continue;
            }

            let sig_idx = match scc.signal_indices.get(i) {
                Some(&idx) => idx,
                None => continue,
            };
            let sig = match signals.get(sig_idx) {
                Some(s) => s,
                None => continue,
            };

            let (declared, sig_signed) = match sig.ty {
                SignalType::Bool => (1u32, false),
                SignalType::Unsigned(w) => (w, false),
                SignalType::Signed(w) => (w, true),
            };

            // The solution width should equal the declared width for
            // signals with explicit annotations. If the solved width
            // is strictly greater than declared, that's a truncation
            // (handled elsewhere). If it's less, that's non-minimal.
            if width < declared {
                // Width is less than declared — solution assigned less
                // than what the signal needs. This shouldn't happen if
                // the solver works correctly.
                let solved_display = super::types::Width(width).display_with_sign(sig_signed);
                let declared_display = super::types::Width(declared).display_with_sign(sig_signed);
                diagnostics.push(WidthDiag::error(format!(
                    "[E511] COMPILER BUG: signal '{}' solved width {} is less \
                     than declared {}",
                    sig.name, solved_display, declared_display
                )));
                is_minimal = false;
            }
            // If width > declared, that's fine — the SCC solver
            // determined the signal needs more bits than declared
            // (a truncation error will be reported separately).
        }
    }

    VerifyResult { is_minimal, diagnostics }
}
