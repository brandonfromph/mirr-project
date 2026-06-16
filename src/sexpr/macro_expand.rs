//! Hygienic macro expander for S-expression based pattern expansion.
//!
//! Provides name-capture prevention during hardware template expansion
//! by suffixing internal names with unique hygiene marks.
//!
//! **Phase 2**: The evaluation engine intercepts compile-time generative
//! directives (`for-generate`, `if-generate`, `let-bind`, `concat-sym`)
//! and unrolls them into flat, concrete hardware AST nodes.
//!
//! This is a PARALLEL path alongside the existing text-based expander
//! (`src/expand/mod.rs`). It does NOT replace the existing expander.

#![forbid(unsafe_code)]

use crate::error::MirrError;
use crate::sexpr::parser::sexpr_err;
use crate::sexpr::types::SExpr;
use crate::sexpr::MAX_LOOP_ITERATIONS;
use crate::sexpr::MAX_MACRO_EXPAND_DEPTH;
use crate::sexpr::MAX_SEXPR_DEPTH;
use crate::sexpr::MAX_SEXPR_NODES;
use crate::sexpr::MAX_TOTAL_GENERATED_NODES;

/// Hygienic macro expander with compile-time evaluation engine.
///
/// Each expansion gets a unique `expansion_id` to prevent name collisions.
/// Bounded by `MAX_MACRO_EXPAND_DEPTH`.
///
/// The evaluation engine recognizes these compile-time forms:
/// - `(for-generate "var" start end (body...))` — bounded loop unrolling
/// - `(if-generate cond then else)` — static conditional
/// - `(let-bind "name" value body)` — scoped binding
/// - `(concat-sym parts...)` — compile-time string concatenation
/// - Arithmetic: `(+ a b)`, `(- a b)`, `(* a b)`, `(< a b)`, etc.
pub struct MacroExpander {
    expansion_counter: usize,
    max_depth: usize,
    /// Compile-time environment: variable bindings from `for-generate` and `let-bind`.
    env: Vec<(String, SExpr)>,
}

impl MacroExpander {
    /// Create a new macro expander with default limits.
    pub fn new() -> Self {
        Self { expansion_counter: 0, max_depth: MAX_MACRO_EXPAND_DEPTH, env: Vec::new() }
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

    /// General-purpose macro expansion entry point for the entire program.
    pub fn expand(&mut self, expr: &SExpr) -> Result<SExpr, MirrError> {
        let mut node_count = 0;
        self.walk_ast(expr, 0, &mut node_count)
    }

    /// Recursively walk the AST, intercepting and evaluating generative forms.
    fn walk_ast(
        &mut self,
        expr: &SExpr,
        depth: usize,
        node_count: &mut usize,
    ) -> Result<SExpr, MirrError> {
        *node_count += 1;
        if *node_count > MAX_TOTAL_GENERATED_NODES {
            return Err(sexpr_err(format!(
                "{} Macro expansion exceeded max node limit",
                crate::error_codes::ec(814)
            )));
        }
        if depth > MAX_SEXPR_DEPTH {
            return Err(sexpr_err(format!(
                "{} S-Expression AST depth exceeded",
                crate::error_codes::ec(814)
            )));
        }

        match expr {
            SExpr::List(items) if !items.is_empty() => {
                // Check for generative head symbols
                if let Some(head) = items[0].as_symbol() {
                    match head {
                        "for-generate" => {
                            return self.eval_for_generate(items, depth, node_count);
                        }
                        "if-generate" => {
                            return self.eval_if_generate(items, depth, node_count);
                        }
                        "let-bind" => {
                            return self.eval_let_bind(items, depth, node_count);
                        }
                        "concat-sym" => {
                            return self.eval_concat_sym(items, node_count);
                        }
                        "+" | "-" | "*" | "/" | "<" | ">" | "==" | "!=" | "<=" | ">=" | "&&"
                        | "||" => {
                            if let Ok(evaled) = self.eval_arithmetic(head, items, depth, node_count)
                            {
                                return Ok(evaled);
                            }
                            // If compile-time arithmetic fails (e.g. operands are signals),
                            // fall through and treat it as a normal AST node.
                        }
                        _ => {}
                    }
                }

                // Normal list: recurse into children, splicing any generated lists
                let mut new_items = Vec::with_capacity(items.len());
                for item in items {
                    // Pre-check if the item is a generative macro that returns a spliced list
                    let is_spliceable = if let SExpr::List(child_items) = item {
                        if let Some(head) = child_items.first().and_then(|h| h.as_symbol()) {
                            head == "for-generate" || head == "if-generate" || head == "let-bind"
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    let expanded = self.walk_ast(item, depth + 1, node_count)?;

                    if is_spliceable {
                        if let SExpr::List(generated) = expanded {
                            new_items.extend(generated);
                        } else {
                            new_items.push(expanded);
                        }
                    } else {
                        new_items.push(expanded);
                    }
                }
                Ok(SExpr::List(new_items))
            }
            SExpr::List(_) => {
                // Empty list
                Ok(expr.clone())
            }
            SExpr::Symbol(name) => {
                // Check environment for symbol bindings (loop variables, let-binds)
                if let Some(val) = self.lookup(name) {
                    return Ok(val);
                }
                Ok(expr.clone())
            }
            SExpr::Quote(inner) => {
                let inner_expanded = self.walk_ast(inner, depth + 1, node_count)?;
                Ok(SExpr::Quote(Box::new(inner_expanded)))
            }
            SExpr::Quasiquote(inner) => {
                let inner_expanded = self.walk_ast(inner, depth + 1, node_count)?;
                Ok(SExpr::Quasiquote(Box::new(inner_expanded)))
            }
            SExpr::Unquote(inner) => {
                let inner_expanded = self.walk_ast(inner, depth + 1, node_count)?;
                Ok(SExpr::Unquote(Box::new(inner_expanded)))
            }
            // Atoms (Integer, Bool, String) pass through unchanged.
            _ => Ok(expr.clone()),
        }
    }

    // ── Generative Form Evaluators ─────────────────────────────────

    /// Evaluate `(for-generate "var" start end (body...))`.
    ///
    /// Unrolls the body `end - start` times, binding `var` to each iteration
    /// value. Returns a flat `SExpr::List` of all generated children, spliced.
    fn eval_for_generate(
        &mut self,
        items: &[SExpr],
        depth: usize,
        node_count: &mut usize,
    ) -> Result<SExpr, MirrError> {
        // (for-generate "var" start end (body...))
        if items.len() < 5 {
            return Err(sexpr_err(format!(
                "{} for-generate requires var, start, end, body",
                crate::error_codes::ec(806)
            )));
        }

        let var = items[1]
            .as_str_val()
            .ok_or_else(|| {
                sexpr_err(format!(
                    "{} for-generate var must be a string",
                    crate::error_codes::ec(806)
                ))
            })?
            .to_string();

        let start = self.eval_to_integer(&items[2], depth, node_count)?;
        let end = self.eval_to_integer(&items[3], depth, node_count)?;

        if end < start {
            return Err(sexpr_err(format!(
                "{} for-generate: end ({end}) < start ({start})",
                crate::error_codes::ec(816)
            )));
        }
        let iteration_count = (end - start) as usize;
        if iteration_count > MAX_LOOP_ITERATIONS {
            return Err(sexpr_err(format!(
                "{} for-generate loop exceeds MAX_LOOP_ITERATIONS ({iteration_count} > {MAX_LOOP_ITERATIONS})",
                crate::error_codes::ec(816)
            )));
        }

        let body = &items[4];
        let body_items = body.as_list().unwrap_or(&[]);

        let mut generated = Vec::new();
        let env_depth = self.env.len();

        for i in start..end {
            // Push binding
            self.env.push((var.clone(), SExpr::Integer(i)));

            // Walk each body item
            for body_item in body_items {
                let expanded = self.walk_ast(body_item, depth + 1, node_count)?;
                generated.push(expanded);
            }

            // Pop binding
            self.env.truncate(env_depth);
        }

        // Return a splice list — the generated nodes wrapped in a list.
        // If there's exactly one item, return it directly.
        if generated.len() == 1 {
            Ok(generated.into_iter().next().unwrap_or(SExpr::List(vec![])))
        } else {
            Ok(SExpr::List(generated))
        }
    }

    /// Evaluate `(if-generate cond then else)`.
    ///
    /// Statically evaluates the condition and returns only the chosen branch.
    fn eval_if_generate(
        &mut self,
        items: &[SExpr],
        depth: usize,
        node_count: &mut usize,
    ) -> Result<SExpr, MirrError> {
        if items.len() < 4 {
            return Err(sexpr_err(format!(
                "{} if-generate requires condition, then, else",
                crate::error_codes::ec(806)
            )));
        }

        let cond = self.walk_ast(&items[1], depth + 1, node_count)?;
        let is_true = self.is_truthy(&cond);

        if is_true {
            self.walk_ast(&items[2], depth + 1, node_count)
        } else {
            self.walk_ast(&items[3], depth + 1, node_count)
        }
    }

    /// Evaluate `(let-bind "var" "type" value)`.
    ///
    /// Binds `var` to `value` in the current block scope. Returns an empty list
    /// because let-bind statements produce no hardware nodes.
    fn eval_let_bind(
        &mut self,
        items: &[SExpr],
        depth: usize,
        node_count: &mut usize,
    ) -> Result<SExpr, MirrError> {
        if items.len() < 4 {
            return Err(sexpr_err(format!(
                "{} let-bind requires name, type, and value",
                crate::error_codes::ec(806)
            )));
        }

        let name = items[1]
            .as_str_val()
            .ok_or_else(|| {
                sexpr_err(format!("{} let-bind name must be a string", crate::error_codes::ec(806)))
            })?
            .to_string();

        // items[2] is the type, items[3] is the value
        let value = self.walk_ast(&items[3], depth + 1, node_count)?;

        self.env.push((name, value));

        // Let-bindings are pure macro definitions, they do not emit hardware AST nodes
        Ok(SExpr::List(Vec::new()))
    }

    /// Evaluate `(concat-sym parts...)`.
    ///
    /// Evaluates each part and concatenates them into a single `SExpr::Str`.
    fn eval_concat_sym(
        &mut self,
        items: &[SExpr],
        node_count: &mut usize,
    ) -> Result<SExpr, MirrError> {
        let mut result = String::new();
        for item in &items[1..] {
            // Resolve symbols from environment first
            let resolved = match item {
                SExpr::Symbol(name) => {
                    if let Some(val) = self.lookup(name) {
                        val
                    } else {
                        item.clone()
                    }
                }
                _ => item.clone(),
            };
            *node_count += 1;

            match &resolved {
                SExpr::Str(s) => result.push_str(s),
                SExpr::Integer(n) => result.push_str(&n.to_string()),
                SExpr::Bool(b) => result.push_str(&b.to_string()),
                SExpr::Symbol(s) => result.push_str(s),
                _ => {
                    // Try to evaluate complex expressions
                    let mut nc = 0;
                    if let Ok(evaled) = self.walk_ast(&resolved, 0, &mut nc) {
                        match &evaled {
                            SExpr::Str(s) => result.push_str(s),
                            SExpr::Integer(n) => result.push_str(&n.to_string()),
                            SExpr::Symbol(s) => result.push_str(s),
                            _ => result.push_str(&format!("{evaled}")),
                        }
                    }
                }
            }
        }
        Ok(SExpr::str_val(&result))
    }

    /// Evaluate compile-time arithmetic: `(+ a b)`, `(* a b)`, `(< a b)`, etc.
    fn eval_arithmetic(
        &mut self,
        op: &str,
        items: &[SExpr],
        depth: usize,
        node_count: &mut usize,
    ) -> Result<SExpr, MirrError> {
        if items.len() < 3 {
            return Err(sexpr_err(format!(
                "{} Arithmetic op '{op}' requires two operands",
                crate::error_codes::ec(806)
            )));
        }

        let left = self.eval_to_integer(&items[1], depth, node_count)?;
        let right = self.eval_to_integer(&items[2], depth, node_count)?;

        match op {
            "+" => Ok(SExpr::Integer(left.wrapping_add(right))),
            "-" => Ok(SExpr::Integer(left.wrapping_sub(right))),
            "*" => Ok(SExpr::Integer(left.wrapping_mul(right))),
            "/" => {
                if right == 0 {
                    return Err(sexpr_err(format!(
                        "{} Division by zero in compile-time arithmetic",
                        crate::error_codes::ec(816)
                    )));
                }
                Ok(SExpr::Integer(left / right))
            }
            "<" => Ok(SExpr::Bool(left < right)),
            ">" => Ok(SExpr::Bool(left > right)),
            "<=" => Ok(SExpr::Bool(left <= right)),
            ">=" => Ok(SExpr::Bool(left >= right)),
            "==" => Ok(SExpr::Bool(left == right)),
            "!=" => Ok(SExpr::Bool(left != right)),
            _ => Ok(SExpr::Integer(0)),
        }
    }

    // ── Helpers ─────────────────────────────────────────────────────

    /// Evaluate an expression to an integer value (resolving symbols and arithmetic).
    fn eval_to_integer(
        &mut self,
        expr: &SExpr,
        depth: usize,
        node_count: &mut usize,
    ) -> Result<u64, MirrError> {
        match expr {
            SExpr::Integer(n) => Ok(*n),
            SExpr::Symbol(name) => {
                if let Some(val) = self.lookup(name) {
                    match val {
                        SExpr::Integer(n) => Ok(n),
                        _ => Err(sexpr_err(format!(
                            "{} Symbol '{name}' does not evaluate to an integer",
                            crate::error_codes::ec(816)
                        ))),
                    }
                } else {
                    Err(sexpr_err(format!(
                        "{} Undefined symbol '{name}' in compile-time arithmetic",
                        crate::error_codes::ec(816)
                    )))
                }
            }
            SExpr::List(items) => {
                // Handle MIRR AST ("literal" "integer" <n>) directly
                if items.len() == 3
                    && items[0].as_str_val() == Some("literal")
                    && items[1].as_str_val() == Some("integer")
                {
                    if let Some(n) = items[2].as_integer() {
                        return Ok(n);
                    }
                }

                // Try to evaluate as arithmetic expression
                let evaled = self.walk_ast(expr, depth + 1, node_count)?;
                match evaled {
                    SExpr::Integer(n) => Ok(n),
                    SExpr::List(eval_items)
                        if eval_items.len() == 3
                            && eval_items[0].as_str_val() == Some("literal")
                            && eval_items[1].as_str_val() == Some("integer") =>
                    {
                        if let Some(n) = eval_items[2].as_integer() {
                            Ok(n)
                        } else {
                            Err(sexpr_err(format!(
                                "{} Expression does not evaluate to an integer",
                                crate::error_codes::ec(816)
                            )))
                        }
                    }
                    _ => Err(sexpr_err(format!(
                        "{} Expression does not evaluate to an integer",
                        crate::error_codes::ec(816)
                    ))),
                }
            }
            _ => Err(sexpr_err(format!(
                "{} Expected integer, got: {}",
                crate::error_codes::ec(816),
                expr
            ))),
        }
    }

    /// Check if an S-expression is truthy for `if-generate`.
    fn is_truthy(&self, expr: &SExpr) -> bool {
        match expr {
            SExpr::Bool(b) => *b,
            SExpr::Integer(n) => *n != 0,
            SExpr::List(items) => !items.is_empty(),
            SExpr::Str(s) => !s.is_empty(),
            _ => true,
        }
    }

    /// Look up a symbol in the compile-time environment (most-recent first).
    fn lookup(&self, name: &str) -> Option<SExpr> {
        self.env.iter().rev().find(|(k, _)| k == name).map(|(_, v)| v.clone())
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
