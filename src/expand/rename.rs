//! Name prefixing and renaming for pattern expansion.

#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::expr::Expr;
use crate::ast::property::PropertyFormula;
use crate::ast::MAX_EXPR_NODES;

use super::ExpandedFragment;

pub(super) fn collect_fragment_names(fragment: &ExpandedFragment) -> HashSet<String> {
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

/// Apply name prefixing to all names in the fragment.
///
/// Scheme: `{prefix}_{original_name}`
///
/// Also renames references: guard conditions (signal refs), reflex guard_names,
/// reflex assignment targets (only if they reference internal signals from
/// this fragment), and property formula signal refs.
pub(super) fn apply_name_prefixing(
    fragment: &mut ExpandedFragment,
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
            if let Some(new_name) = rename.get(&assignment.target) {
                assignment.target = new_name.clone();
            }
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
                if let Some(new_name) = rename.get(name.as_str()) {
                    *name = new_name.clone();
                }
            }
            Expr::Prev { signal, .. } => {
                if let Some(new_name) = rename.get(signal.as_str()) {
                    *signal = new_name.clone();
                }
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
pub(super) fn set_origin_tags(fragment: &mut ExpandedFragment, origin: &str) {
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
