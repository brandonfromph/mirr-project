//! Hygienic macro expander for S-expression based pattern expansion.
//!
//! Provides name-capture prevention during hardware template expansion
//! by suffixing internal names with unique hygiene marks.
//!
//! This is a PARALLEL path alongside the existing text-based expander
//! (`src/expand/mod.rs`). It does NOT replace the existing expander.

#![forbid(unsafe_code)]

use crate::error::MirrError;
use crate::sexpr::parser::sexpr_err;
use crate::sexpr::types::SExpr;
use crate::sexpr::MAX_MACRO_EXPAND_DEPTH;
use crate::sexpr::MAX_SEXPR_NODES;

/// Hygienic macro expander.
///
/// Each expansion gets a unique `expansion_id` to prevent name collisions.
/// Bounded by `MAX_MACRO_EXPAND_DEPTH`.
pub struct MacroExpander {
    expansion_counter: usize,
    max_depth: usize,
}

impl MacroExpander {
    /// Create a new macro expander with default limits.
    pub fn new() -> Self {
        Self { expansion_counter: 0, max_depth: MAX_MACRO_EXPAND_DEPTH }
    }

    /// Expand a template S-expression with hygiene marks.
    ///
    /// - `template`: the S-expression template body
    /// - `param_names`: parameter names that should NOT be renamed (pass-through)
    /// - `bindings`: parameter name -> actual argument S-expression
    /// - `depth`: current expansion depth
    ///
    /// Returns the expanded S-expression with internal names suffixed by `__hyg{id}`.
    pub fn expand_hygienic(
        &mut self,
        template: &SExpr,
        param_names: &[String],
        bindings: &[(String, SExpr)],
        depth: usize,
    ) -> Result<SExpr, MirrError> {
        if depth > self.max_depth {
            return Err(sexpr_err(format!(
                "{} Macro expansion depth exceeded",
                crate::error_codes::ec(814)
            )));
        }
        self.expansion_counter += 1;
        let expansion_id = self.expansion_counter;

        self.rename_internal(template, param_names, bindings, expansion_id, 0)
    }

    /// Recursively rename internal names in a template.
    ///
    /// - Parameters (in `param_names`) are bound to their actual arguments from `bindings`
    /// - All other string literals that look like identifiers get a hygiene suffix
    #[allow(clippy::only_used_in_recursion)]
    fn rename_internal(
        &self,
        expr: &SExpr,
        param_names: &[String],
        bindings: &[(String, SExpr)],
        expansion_id: usize,
        iters: usize,
    ) -> Result<SExpr, MirrError> {
        if iters > MAX_SEXPR_NODES {
            return Err(sexpr_err(format!(
                "{} Macro expansion exceeded node budget",
                crate::error_codes::ec(804)
            )));
        }

        match expr {
            SExpr::Str(s) => {
                // Check if this string is a parameter name.
                if let Some(binding) = bindings.iter().find(|(k, _)| k == s) {
                    return Ok(binding.1.clone());
                }
                // Check if it's a parameter name without binding (pass through).
                if param_names.contains(s) {
                    return Ok(expr.clone());
                }
                // Internal name: apply hygiene suffix.
                if is_identifier(s) {
                    Ok(SExpr::str_val(&format!("{s}__hyg{expansion_id}")))
                } else {
                    Ok(expr.clone())
                }
            }
            SExpr::Symbol(_) => {
                // Symbols are structural tags (signal, guard, etc.) — don't rename.
                Ok(expr.clone())
            }
            SExpr::List(items) => {
                let mut result = Vec::new();
                let mut count = iters;
                for item in items {
                    count += 1;
                    result.push(self.rename_internal(
                        item,
                        param_names,
                        bindings,
                        expansion_id,
                        count,
                    )?);
                }
                Ok(SExpr::list(result))
            }
            SExpr::Quote(inner) => {
                let renamed =
                    self.rename_internal(inner, param_names, bindings, expansion_id, iters + 1)?;
                Ok(SExpr::Quote(Box::new(renamed)))
            }
            SExpr::Quasiquote(inner) => {
                let renamed =
                    self.rename_internal(inner, param_names, bindings, expansion_id, iters + 1)?;
                Ok(SExpr::Quasiquote(Box::new(renamed)))
            }
            SExpr::Unquote(inner) => {
                let renamed =
                    self.rename_internal(inner, param_names, bindings, expansion_id, iters + 1)?;
                Ok(SExpr::Unquote(Box::new(renamed)))
            }
            // Atoms (Integer, Bool) pass through unchanged.
            _ => Ok(expr.clone()),
        }
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a string looks like a MIRR identifier (not a keyword or operator).
fn is_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let first = s.as_bytes()[0];
    (first.is_ascii_alphabetic() || first == b'_')
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
