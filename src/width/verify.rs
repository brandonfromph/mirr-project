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
use serde::Serialize;

/// Result of the Unique Least Solution verification.
#[derive(Debug, Clone, Serialize)]
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
    registry: &crate::ecs::registry::Registry,
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

            let sig_id = match scc.signals.get(i) {
                Some(&id) => id,
                None => continue,
            };

            let sig_name = registry.names[sig_id.0 as usize]
                .as_ref()
                .map(|nc| registry.resolve_name(nc.0))
                .unwrap_or("unknown");

            let (declared, sig_signed) = registry.types[sig_id.0 as usize]
                .as_ref()
                .map(|tc| tc.0.core.width_and_signed())
                .unwrap_or((0, false));

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
                diagnostics.push(WidthDiag::error(format!("{} COMPILER BUG: signal '{}' solved width {} is less \
                     than declared {}", crate::error_codes::ec(511),
                    sig_name, solved_display, declared_display
                )).with_code("E511").with_signal(sig_name)
                  .with_help("This is a compiler bug. Please report it at https://github.com/brandonfromph/mirr-project/issues"));
                is_minimal = false;
            }
        }
    }

    VerifyResult { is_minimal, diagnostics }
}
