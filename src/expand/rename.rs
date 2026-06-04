//! Name prefixing and renaming for pattern expansion.

#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::expr::Expr;
use crate::ast::property::PropertyFormula;
use crate::ast::MAX_EXPR_NODES;

pub(super) fn collect_fragment_names(
    fragment: &crate::ast::pattern::ReflectBlock,
) -> HashSet<String> {
    let mut names = HashSet::with_capacity(32);
    for s in &fragment.signals {
        names.insert(s.name.clone());
    }
    for g in &fragment.guards {
        names.insert(g.name.clone());
    }
    for r in &fragment.reflexes {
        names.insert(r.name.clone());
    }
    for p in &fragment.properties {
        names.insert(p.name.clone());
    }
    names
}

fn rename_target(target: &mut String, rename: &HashMap<String, String>) {
    if let Some(bracket_pos) = target.find('[') {
        if target.ends_with(']') {
            let array_name = target[..bracket_pos].trim();
            let idx_str = target[bracket_pos + 1..target.len() - 1].trim();

            let new_array_name =
                rename.get(array_name).cloned().unwrap_or_else(|| array_name.to_string());
            let new_idx_str = rename.get(idx_str).cloned().unwrap_or_else(|| idx_str.to_string());

            *target = format!("{}[{}]", new_array_name, new_idx_str);
        }
    } else {
        if let Some(new_name) = rename.get(target) {
            *target = new_name.clone();
        }
    }
}

/// Apply name prefixing to all names in the fragment.
///
/// Scheme: `{prefix}_{original_name}`
///
/// Also renames references: guard conditions (signal refs), reflex guard_names,
/// reflex assignment targets (only if they reference internal signals from
/// this fragment), and property formula signal refs.
pub(super) fn apply_name_prefixing(
    fragment: &mut crate::ast::pattern::ReflectBlock,
    prefix: &str,
    original_names: &HashSet<String>,
    param_names: &HashSet<String>,
) {
    // Build rename map: original_name -> prefixed_name.
    // Only names declared inside the fragment are prefixed. Parameter names were
    // already resolved by text-level ${param} substitution and must NOT be
    // included here — their values are module-level signal names that belong to
    // the calling scope and must remain unchanged.
    let _ = param_names; // consumed by caller; not needed in rename map
    let mut rename: HashMap<String, String> = HashMap::with_capacity(original_names.len());
    for name in original_names {
        rename.insert(name.clone(), format!("{prefix}_{name}"));
    }

    // PRE-ADD all guard names to the rename map (needed for reflex references)
    for guard in &fragment.guards {
        rename.insert(guard.name.clone(), format!("{prefix}_{}", guard.name));
    }

    // Rename signal declarations.
    for sig in &mut fragment.signals {
        if let Some(new_name) = rename.get(&sig.name) {
            // Only rename if explicitly in the map (i.e., internal signals)
            sig.name = new_name.clone();
        }
        // Don't prefix signals that aren't in the rename map - they're likely
        // parameters that should have been substituted earlier
    }

    // Rename guard names.
    for guard in &mut fragment.guards {
        if let Some(new_name) = rename.get(&guard.name) {
            guard.name = new_name.clone();
        }
        rename_expr_signals(&mut guard.condition, &rename);
    }

    // Rename reflex names, guard_names references, and assignment references.
    for reflex in &mut fragment.reflexes {
        if let Some(new_name) = rename.get(&reflex.name) {
            reflex.name = new_name.clone();
        }
        // Rename guard references.
        for gname in &mut reflex.guard_names {
            if let Some(new_name) = rename.get(gname.as_str()) {
                *gname = new_name.clone();
            }
        }
        // Rename assignment targets and RHS expressions (only internal names).
        for assignment in &mut reflex.assignments {
            rename_target(&mut assignment.target, &rename);
            rename_expr_signals(&mut assignment.value, &rename);
        }
    }

    // Rename property names and formula signal references.
    for prop in &mut fragment.properties {
        if let Some(new_name) = rename.get(&prop.name) {
            prop.name = new_name.clone();
        }
        rename_property_signals(&mut prop.formula, &rename);
    }

    // Rename arguments in nested pattern calls.
    for call in &mut fragment.pattern_calls {
        for arg in &mut call.arguments {
            match arg {
                crate::ast::pattern::PatternArg::SignalRef(name)
                | crate::ast::pattern::PatternArg::PatternRef(name) => {
                    if let Some(new_name) = rename.get(name.as_str()) {
                        *name = new_name.clone();
                    }
                }
                _ => {}
            }
        }
    }
}

/// Rename signal references in an expression tree.
///
/// Uses an explicit iterative work stack — zero recursion (NASA P10 rule #1).
/// Bounded: at most 512 nodes.
pub(super) fn rename_expr_signals(expr: &mut Expr, rename: &HashMap<String, String>) {
    let mut stack: Vec<&mut Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        match node {
            Expr::Signal(name) => {
                apply_template_substitution(name, rename);
            }
            Expr::Prev { signal, .. } => {
                apply_template_substitution(signal, rename);
            }
            Expr::Literal(_) => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
            }
            Expr::FieldAccess { object, .. } => {
                stack.push(object);
            }
            Expr::ArrayLiteral(elems) => {
                for e in elems.iter_mut() {
                    stack.push(e);
                }
            }
            Expr::StructLiteral { fields, .. } => {
                for (_, v) in fields.iter_mut() {
                    stack.push(v);
                }
            }
            Expr::UnfoldIndex(name) => {
                if let Some(new_name) = rename.get(name.as_str()) {
                    *name = new_name.clone();
                }
            }
        }
    }
}

/// Rename signal references inside a property formula.
pub(super) fn rename_property_signals(
    formula: &mut PropertyFormula,
    rename: &HashMap<String, String>,
) {
    for expr in formula.exprs_mut() {
        rename_expr_signals(expr, rename);
    }
}

/// Set origin tags on all nodes in the fragment.
pub(super) fn set_origin_tags(fragment: &mut crate::ast::pattern::ReflectBlock, origin: &str) {
    for sig in &mut fragment.signals {
        sig.origin = Some(origin.to_string());
    }
    for guard in &mut fragment.guards {
        guard.origin = Some(origin.to_string());
    }
    for reflex in &mut fragment.reflexes {
        reflex.origin = Some(origin.to_string());
    }
    for prop in &mut fragment.properties {
        prop.origin = Some(origin.to_string());
    }
}

fn apply_template_substitution(target: &mut String, rename: &HashMap<String, String>) {
    // 1. Exact match (for parameters)
    if let Some(new_name) = rename.get(target) {
        *target = new_name.clone();
        return;
    }

    // 2. Substring substitution for ${var} and [var]
    // We sort keys by length descending to prevent partial match collisions.
    let mut keys: Vec<&String> =
        rename.keys().filter(|k| k.starts_with("${") || k.starts_with('[')).collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

    for key in keys {
        if target.contains(key) {
            *target = target.replace(key, &rename[key]);
        }
    }
}

pub(super) fn apply_parameter_substitution(
    fragment: &mut crate::ast::pattern::ReflectBlock,
    subs: &[(String, String)],
) {
    let mut rename: HashMap<String, String> = HashMap::with_capacity(subs.len() * 2);
    for (param, arg) in subs {
        rename.insert(format!("${{{}}}", param), arg.clone());
        rename.insert(format!("[{}]", param), format!("_{}", arg));
        rename.insert(param.clone(), arg.clone());
    }

    for sig in &mut fragment.signals {
        apply_template_substitution(&mut sig.name, &rename);
    }

    for guard in &mut fragment.guards {
        apply_template_substitution(&mut guard.name, &rename);
        if let Some(ref mut tc) = guard.template_cycles {
            apply_template_substitution(tc, &rename);
        }
        rename_expr_signals(&mut guard.condition, &rename);
    }

    for reflex in &mut fragment.reflexes {
        apply_template_substitution(&mut reflex.name, &rename);
        for gname in &mut reflex.guard_names {
            apply_template_substitution(gname, &rename);
        }
        for assignment in &mut reflex.assignments {
            // Target may contain array index [i]
            apply_template_substitution(&mut assignment.target, &rename);
            rename_expr_signals(&mut assignment.value, &rename);
        }
    }

    for prop in &mut fragment.properties {
        apply_template_substitution(&mut prop.name, &rename);
        rename_property_signals(&mut prop.formula, &rename);
    }

    for call in &mut fragment.pattern_calls {
        apply_template_substitution(&mut call.pattern_name, &rename);
        for arg in &mut call.arguments {
            match arg {
                crate::ast::pattern::PatternArg::SignalRef(name)
                | crate::ast::pattern::PatternArg::PatternRef(name) => {
                    apply_template_substitution(name, &rename);
                }
                _ => {}
            }
        }
    }
}
