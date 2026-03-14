//! Clock domain crossing detection and synchronizer insertion.
//!
//! Detects when signals cross between clock domains within a MIRR
//! design and inserts synchronizer chains to prevent metastability.
//! The existing `emit_synchronizer_chains()` in `emit/verilog.rs`
//! handles Verilog emission; this module handles detection and
//! netlist annotation.
//!
//! Clock domain annotation uses a naming convention: signals with
//! `_clkN` suffix belong to domain N. Default domain is `clk`.
//!
//! Bounded: MAX_CLOCK_DOMAINS domains, MAX_CROSSINGS crossings
//! (NASA Power-of-10 compliance).

#![forbid(unsafe_code)]

use crate::ast::program::Module;

/// Maximum number of clock domains (NASA P10: bounded resources).
pub const MAX_CLOCK_DOMAINS: usize = 16;

/// Maximum number of domain crossings to detect.
pub const MAX_CROSSINGS: usize = 128;

/// Default number of synchronizer stages (standard MTBF practice).
pub const DEFAULT_SYNC_STAGES: u32 = 2;

/// A clock domain within the design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClockDomain {
    /// Name of the clock domain (e.g., "clk", "clk2", "slow_clk").
    pub name: String,
    /// Optional frequency hint in Hz (for documentation/analysis).
    pub frequency_hint: Option<u64>,
}

/// A signal crossing between two clock domains.
#[derive(Debug, Clone)]
pub struct DomainCrossing {
    /// Name of the signal crossing domains.
    pub signal: String,
    /// Source clock domain.
    pub from_domain: String,
    /// Destination clock domain.
    pub to_domain: String,
    /// Number of synchronizer stages to insert.
    pub sync_stages: u32,
}

/// Result of clock domain analysis.
#[derive(Debug, Clone)]
pub struct ClockDomainResult {
    /// All detected clock domains.
    pub domains: Vec<ClockDomain>,
    /// All detected crossings.
    pub crossings: Vec<DomainCrossing>,
}

/// Detect clock domains and crossings in a module.
///
/// Uses the naming convention: signals containing `_clkN` belong to
/// domain `clkN`. Signals without a clock suffix belong to the default
/// domain `clk`.
pub fn detect_crossings(module: &Module) -> ClockDomainResult {
    let mut domains: Vec<ClockDomain> = Vec::new();
    let mut crossings: Vec<DomainCrossing> = Vec::new();

    // Default domain always exists.
    domains.push(ClockDomain { name: "clk".to_string(), frequency_hint: None });

    // Scan signal names for clock domain suffixes.
    let mut domain_count = 1usize;
    for signal in &module.signals {
        if domain_count >= MAX_CLOCK_DOMAINS {
            break;
        }
        if let Some(domain_name) = extract_clock_domain(&signal.name) {
            if !domains.iter().any(|d| d.name == domain_name) {
                domains.push(ClockDomain { name: domain_name, frequency_hint: None });
                domain_count += 1;
            }
        }
    }

    // If only one domain, no crossings possible.
    if domains.len() <= 1 {
        return ClockDomainResult { domains, crossings };
    }

    // Detect crossings: look for signals referenced in reflexes where
    // the signal's domain differs from the reflex's output domain.
    let mut crossing_count = 0usize;
    for reflex in &module.reflexes {
        for assignment in &reflex.assignments {
            if crossing_count >= MAX_CROSSINGS {
                break;
            }
            let target_domain = signal_domain(&assignment.target);
            let source_signals = collect_signal_refs(&assignment.value);

            for src_name in &source_signals {
                if crossing_count >= MAX_CROSSINGS {
                    break;
                }
                let src_domain = signal_domain(src_name);
                if src_domain != target_domain {
                    crossings.push(DomainCrossing {
                        signal: src_name.clone(),
                        from_domain: src_domain,
                        to_domain: target_domain.clone(),
                        sync_stages: DEFAULT_SYNC_STAGES,
                    });
                    crossing_count += 1;
                }
            }
        }
    }

    ClockDomainResult { domains, crossings }
}

/// Extract clock domain name from a signal name.
///
/// Looks for `_clkN` suffix. Returns None for default domain.
fn extract_clock_domain(name: &str) -> Option<String> {
    // Look for pattern: _clk followed by optional digits
    let bytes = name.as_bytes();
    let len = bytes.len();
    if len < 4 {
        return None;
    }

    // Scan for "_clk" substring (bounded by name length).
    let mut i = 0usize;
    let max_scan = len.saturating_sub(3);
    while i < max_scan {
        if bytes[i] == b'_'
            && i + 3 < len
            && bytes[i + 1] == b'c'
            && bytes[i + 2] == b'l'
            && bytes[i + 3] == b'k'
        {
            // Found "_clk" — extract the rest as domain name.
            return Some(name[i + 1..].to_string());
        }
        i += 1;
    }
    None
}

/// Determine which clock domain a signal belongs to.
fn signal_domain(name: &str) -> String {
    extract_clock_domain(name).unwrap_or_else(|| "clk".to_string())
}

/// Collect all signal name references from an expression.
///
/// Iterative traversal (NASA P10: no recursion).
fn collect_signal_refs(expr: &crate::ast::expr::Expr) -> Vec<String> {
    use crate::ast::expr::Expr;

    let mut signals = Vec::new();
    let mut stack: Vec<&Expr> = vec![expr];
    let mut iterations = 0usize;
    const MAX_ITERATIONS: usize = 512;

    while let Some(e) = stack.pop() {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            break;
        }
        match e {
            Expr::Signal(name) => signals.push(name.clone()),
            Expr::Prev { signal, .. } => signals.push(signal.clone()),
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            Expr::Literal(_) => {}
        }
    }
    signals
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::Module;

    fn empty_module() -> Module {
        Module {
            name: "test".to_string(),
            signals: Vec::new(),
            guards: Vec::new(),
            reflexes: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            properties: Vec::new(),
            span: None,
        }
    }

    #[test]
    fn default_domain_always_present() {
        let module = empty_module();
        let result = detect_crossings(&module);
        assert_eq!(result.domains.len(), 1);
        assert_eq!(result.domains[0].name, "clk");
    }

    #[test]
    fn no_crossings_in_single_domain() {
        let module = empty_module();
        let result = detect_crossings(&module);
        assert!(result.crossings.is_empty());
    }

    #[test]
    fn extract_domain_from_suffix() {
        assert_eq!(extract_clock_domain("data_clk2"), Some("clk2".to_string()));
        assert_eq!(extract_clock_domain("sensor_clk_fast"), Some("clk_fast".to_string()));
        assert_eq!(extract_clock_domain("normal_signal"), None);
        assert_eq!(extract_clock_domain("clk"), None); // No underscore prefix
    }

    #[test]
    fn signal_domain_default() {
        assert_eq!(signal_domain("sensor"), "clk");
        assert_eq!(signal_domain("data_clk2"), "clk2");
    }

    #[test]
    fn domain_count_bounded() {
        // Verify the constant is usable for typical hardware designs.
        let limit = MAX_CLOCK_DOMAINS;
        assert!(limit >= 2, "need at least 2 clock domains for CDC");
        assert!(limit <= 256, "too many clock domains for bounded analysis");
    }
}
