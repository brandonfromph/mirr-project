//! Reader macro registry for domain-specific notation extensions.
//!
//! Reader macros transform during S-expression parsing (read time),
//! before evaluation. Provides built-in macros for hardware-domain
//! notations: clock frequencies, temporal delays, range constraints.

#![forbid(unsafe_code)]

use crate::error::MirrError;
use crate::sexpr::parser::sexpr_err;
use crate::sexpr::types::SExpr;
use crate::sexpr::MAX_READER_MACROS;

/// Function type for reader macros: takes the argument string, returns S-expression.
type ReaderMacroFn = fn(&str) -> Result<SExpr, MirrError>;

/// Registry of reader macros.
///
/// Bounded by `MAX_READER_MACROS` entries.
pub struct ReaderMacroRegistry {
    macros: Vec<(String, ReaderMacroFn)>,
}

impl ReaderMacroRegistry {
    /// Create a new registry with built-in macros registered.
    pub fn new() -> Self {
        let mut reg = Self { macros: Vec::new() };
        // Register built-in macros (ignore errors — these are known to fit).
        let _ = reg.register("freq", reader_freq);
        let _ = reg.register("delay", reader_delay);
        let _ = reg.register("range", reader_range);
        reg
    }

    /// Register a new reader macro.
    ///
    /// Returns error if the registry is full.
    pub fn register(&mut self, name: &str, f: ReaderMacroFn) -> Result<(), MirrError> {
        if self.macros.len() >= MAX_READER_MACROS {
            return Err(sexpr_err("[E815] Too many reader macros"));
        }
        self.macros.push((name.to_string(), f));
        Ok(())
    }

    /// Expand a reader macro invocation.
    ///
    /// `name` is the macro name (e.g., "freq"), `args` is the argument string
    /// (e.g., "100MHz").
    pub fn expand(&self, name: &str, args: &str) -> Result<SExpr, MirrError> {
        for (macro_name, func) in &self.macros {
            if macro_name == name {
                return func(args);
            }
        }
        Err(sexpr_err(format!("[E815] Unknown reader macro: #{name}")))
    }

    /// Number of registered macros.
    pub fn len(&self) -> usize {
        self.macros.len()
    }

    /// True if no macros are registered.
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }
}

impl Default for ReaderMacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Built-in reader macros
// =========================================================================

/// Parse a frequency string into `(frequency <hertz>)`.
///
/// Supports Hz, KHz/kHz, MHz, GHz suffixes.
fn reader_freq(args: &str) -> Result<SExpr, MirrError> {
    let args = args.trim();
    let (num_str, multiplier) = if let Some(s) = args.strip_suffix("GHz") {
        (s, 1_000_000_000u64)
    } else if let Some(s) = args.strip_suffix("MHz") {
        (s, 1_000_000u64)
    } else if let Some(s) = args.strip_suffix("KHz").or_else(|| args.strip_suffix("kHz")) {
        (s, 1_000u64)
    } else if let Some(s) = args.strip_suffix("Hz") {
        (s, 1u64)
    } else {
        return Err(sexpr_err(format!(
            "[E808] Invalid frequency: '{args}'. Expected suffix: Hz, KHz, MHz, GHz"
        )));
    };

    let num: u64 = num_str
        .trim()
        .parse()
        .map_err(|_| sexpr_err(format!("[E808] Invalid frequency number: '{num_str}'")))?;

    Ok(SExpr::list(vec![SExpr::sym("frequency"), SExpr::int(num * multiplier)]))
}

/// Parse a delay annotation into `(temporal-delay <cycles>)`.
fn reader_delay(args: &str) -> Result<SExpr, MirrError> {
    let cycles: u64 = args
        .trim()
        .parse()
        .map_err(|_| sexpr_err(format!("[E808] Invalid delay value: '{args}'")))?;
    Ok(SExpr::list(vec![SExpr::sym("temporal-delay"), SExpr::int(cycles)]))
}

/// Parse a range annotation into `(refinement-range <lo> <hi>)`.
///
/// Format: `lo..hi` (inclusive on both ends).
fn reader_range(args: &str) -> Result<SExpr, MirrError> {
    let parts: Vec<&str> = args.trim().split("..").collect();
    if parts.len() != 2 {
        return Err(sexpr_err(format!("[E808] Invalid range: '{args}'. Expected format: lo..hi")));
    }
    let lo: u64 = parts[0]
        .trim()
        .parse()
        .map_err(|_| sexpr_err(format!("[E808] Invalid range lower bound: '{}'", parts[0])))?;
    let hi: u64 = parts[1]
        .trim()
        .parse()
        .map_err(|_| sexpr_err(format!("[E808] Invalid range upper bound: '{}'", parts[1])))?;
    Ok(SExpr::list(vec![SExpr::sym("refinement-range"), SExpr::int(lo), SExpr::int(hi)]))
}
