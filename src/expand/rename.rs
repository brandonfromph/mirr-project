//! Name prefixing and renaming for pattern expansion.

#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::expr::Expr;
use crate::ast::macro_nodes::{ModuleMacroStmt, ReflexMacroStmt};
use crate::ast::property::PropertyFormula;
use crate::ast::MAX_EXPR_NODES;

pub fn collect_fragment_names(fragment: &crate::ast::pattern::ReflectBlock) -> HashSet<String> {
    let mut names = HashSet::with_capacity(32);
    for stmt in &fragment.statements {
        collect_stmt_names(stmt, &mut names);
    }
    names
}

fn collect_stmt_names(stmt: &ModuleMacroStmt, names: &mut HashSet<String>) {
    match stmt {
        ModuleMacroStmt::Signal(s) => {
            names.insert(s.name.clone());
        }
        ModuleMacroStmt::Guard(g) => {
            names.insert(g.name.clone());
        }
        ModuleMacroStmt::Reflex(r) => {
            names.insert(r.name.clone());
            // Nested statements in reflexes (assignments) don't declare new top-level names
            // but might contain let bindings.
            for rs in &r.statements {
                collect_reflex_stmt_names(rs, names);
            }
        }
        ModuleMacroStmt::Property(p) => {
            names.insert(p.name.clone());
        }
        ModuleMacroStmt::PatternCall(_) => {}
        ModuleMacroStmt::ForLoop { body, .. } => {
            for s in body {
                collect_stmt_names(s, names);
            }
        }
        ModuleMacroStmt::LetBinding { name, .. } => {
            names.insert(name.clone());
        }
    }
}

fn collect_reflex_stmt_names(stmt: &ReflexMacroStmt, names: &mut HashSet<String>) {
    match stmt {
        ReflexMacroStmt::Assignment(_) => {}
        ReflexMacroStmt::LetBinding { name, .. } => {
            names.insert(name.clone());
        }
        ReflexMacroStmt::OnBlock { body, .. } => {
            for s in body {
                collect_reflex_stmt_names(s, names);
            }
        }
        ReflexMacroStmt::ForLoop { body, .. } => {
            for s in body {
                collect_reflex_stmt_names(s, names);
            }
        }
        ReflexMacroStmt::IfElse { true_branch, false_branch, .. } => {
            for s in true_branch {
                collect_reflex_stmt_names(s, names);
            }
            for s in false_branch {
                collect_reflex_stmt_names(s, names);
            }
        }
        ReflexMacroStmt::Match { arms, .. } => {
            for arm in arms {
                for s in &arm.body {
                    collect_reflex_stmt_names(s, names);
                }
            }
        }
    }
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
pub fn apply_name_prefixing(
    fragment: &mut crate::ast::pattern::ReflectBlock,
    prefix: &str,
    original_names: &HashSet<String>,
    param_names: &HashSet<String>,
) {
    // Build rename map: original_name -> prefixed_name.
    let _ = param_names;
    let mut rename: HashMap<String, String> = HashMap::with_capacity(original_names.len());
    for name in original_names {
        rename.insert(name.clone(), format!("{prefix}_{name}"));
    }

    for stmt in &mut fragment.statements {
        rename_stmt(stmt, &rename);
    }
}

fn rename_stmt(stmt: &mut ModuleMacroStmt, rename: &HashMap<String, String>) {
    match stmt {
        ModuleMacroStmt::Signal(sig) => {
            if let Some(new_name) = rename.get(&sig.name) {
                sig.name = new_name.clone();
            }
        }
        ModuleMacroStmt::Guard(guard) => {
            if let Some(new_name) = rename.get(&guard.name) {
                guard.name = new_name.clone();
            }
            rename_expr_signals(&mut guard.condition, rename);
        }
        ModuleMacroStmt::Reflex(reflex) => {
            if let Some(new_name) = rename.get(&reflex.name) {
                reflex.name = new_name.clone();
            }
            for gname in &mut reflex.guard_names {
                if let Some(new_name) = rename.get(gname.as_str()) {
                    *gname = new_name.clone();
                }
            }
            for rs in &mut reflex.statements {
                rename_reflex_stmt(rs, rename);
            }
        }
        ModuleMacroStmt::Property(prop) => {
            if let Some(new_name) = rename.get(&prop.name) {
                prop.name = new_name.clone();
            }
            rename_property_signals(&mut prop.formula, rename);
        }
        ModuleMacroStmt::PatternCall(call) => {
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
        ModuleMacroStmt::ForLoop { body, .. } => {
            for s in body {
                rename_stmt(s, rename);
            }
        }
        ModuleMacroStmt::LetBinding { name, value, .. } => {
            if let Some(new_name) = rename.get(name.as_str()) {
                *name = new_name.clone();
            }
            rename_expr_signals(value, rename);
        }
    }
}

fn rename_reflex_stmt(stmt: &mut ReflexMacroStmt, rename: &HashMap<String, String>) {
    match stmt {
        ReflexMacroStmt::Assignment(assignment) => {
            rename_target(&mut assignment.target, rename);
            rename_expr_signals(&mut assignment.value, rename);
        }
        ReflexMacroStmt::LetBinding { name, value, .. } => {
            if let Some(new_name) = rename.get(name.as_str()) {
                *name = new_name.clone();
            }
            rename_expr_signals(value, rename);
        }
        ReflexMacroStmt::OnBlock { guard_names, body } => {
            for gname in guard_names {
                if let Some(new_name) = rename.get(gname.as_str()) {
                    *gname = new_name.clone();
                }
            }
            for s in body {
                rename_reflex_stmt(s, rename);
            }
        }
        ReflexMacroStmt::ForLoop { body, .. } => {
            for s in body {
                rename_reflex_stmt(s, rename);
            }
        }
        ReflexMacroStmt::IfElse { condition, true_branch, false_branch } => {
            rename_expr_signals(condition, rename);
            for s in true_branch {
                rename_reflex_stmt(s, rename);
            }
            for s in false_branch {
                rename_reflex_stmt(s, rename);
            }
        }
        ReflexMacroStmt::Match { expr, arms } => {
            rename_expr_signals(expr, rename);
            for arm in arms {
                for s in &mut arm.body {
                    rename_reflex_stmt(s, rename);
                }
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
        if let Expr::Signal(name) = node {
            apply_template_substitution(name, rename);
            let mut replacement = None;
            if name == "true" {
                replacement = Some(Expr::Literal(crate::ast::types::LiteralValue::Bool(true)));
            } else if name == "false" {
                replacement = Some(Expr::Literal(crate::ast::types::LiteralValue::Bool(false)));
            } else if let Ok(val) = name.parse::<u64>() {
                replacement = Some(Expr::Literal(crate::ast::types::LiteralValue::Integer(val)));
            }
            if let Some(new_expr) = replacement {
                *node = new_expr;
            }
        } else {
            match node {
                Expr::Prev { signal, .. } => {
                    apply_template_substitution(signal, rename);
                }
                Expr::Literal(_) => {}
                Expr::Unary { operand, .. } => stack.push(operand),
                Expr::Binary { left, right, .. } => {
                    stack.push(left);
                    stack.push(right);
                }
                Expr::ArrayIndex { ref array, ref index } => {
                    // BUG-4: Handle unrolled signals like data_in[i] -> data_in_0
                    // If index is a signal that's in the rename map, and [index] is also in the map,
                    // it means we're in a structural macro loop.
                    let mut collapsed_name = None;
                    if let Expr::Signal(ref a_name) = **array {
                        if let Expr::Signal(ref i_name) = **index {
                            let template_key = format!("[{}]", i_name);
                            if let Some(val_suffix) = rename.get(&template_key) {
                                collapsed_name = Some(format!("{}{}", a_name, val_suffix));
                            }
                        }
                    }

                    if let Some(new_name) = collapsed_name {
                        *node = Expr::Signal(new_name);
                    } else {
                        // Use a non-ref borrow for the stack push
                        if let Expr::ArrayIndex { array, index } = node {
                            stack.push(array);
                            stack.push(index);
                        }
                    }
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
                Expr::Signal(_) => unreachable!(),
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
pub fn set_origin_tags(fragment: &mut crate::ast::pattern::ReflectBlock, origin: &str) {
    for stmt in &mut fragment.statements {
        set_stmt_origin(stmt, origin);
    }
}

fn set_stmt_origin(stmt: &mut ModuleMacroStmt, origin: &str) {
    match stmt {
        ModuleMacroStmt::Signal(sig) => sig.origin = Some(origin.to_string()),
        ModuleMacroStmt::Guard(guard) => guard.origin = Some(origin.to_string()),
        ModuleMacroStmt::Reflex(_reflex) => {
            // UnexpandedReflex does not have an origin field.
            // But its components (Signal/Guard created from LetBinding) might need it.
            // Actually, origin tagging is usually applied to final flat components.
            // If we're tagging fragments, we should probably add origin to UnexpandedReflex too.
            // But for now, we'll let the expander carry it over.
        }
        ModuleMacroStmt::Property(prop) => prop.origin = Some(origin.to_string()),
        ModuleMacroStmt::PatternCall(_) => {}
        ModuleMacroStmt::ForLoop { body, .. } => {
            for s in body {
                set_stmt_origin(s, origin);
            }
        }
        ModuleMacroStmt::LetBinding { .. } => {}
    }
}

fn apply_template_substitution(target: &mut String, rename: &HashMap<String, String>) {
    // 1. Exact match (for parameters)
    if let Some(new_name) = rename.get(target) {
        *target = new_name.clone();
        return;
    }

    // 1b. Array base identifier match (e.g. replacing `tx_valid` in `tx_valid[1]`)
    if let Some(bracket_idx) = target.find('[') {
        let base = &target[..bracket_idx];
        if let Some(new_base) = rename.get(base) {
            if !base.starts_with("${") && !base.starts_with('[') {
                *target = format!("{}{}", new_base, &target[bracket_idx..]);
                // Fallthrough to allow ${} replacements in the index part if any
            }
        }
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

    for stmt in &mut fragment.statements {
        substitute_stmt(stmt, &rename);
    }
}

fn substitute_stmt(stmt: &mut ModuleMacroStmt, rename: &HashMap<String, String>) {
    match stmt {
        ModuleMacroStmt::Signal(sig) => {
            apply_template_substitution(&mut sig.name, rename);
        }
        ModuleMacroStmt::Guard(guard) => {
            apply_template_substitution(&mut guard.name, rename);
            if let Some(ref mut tc) = guard.template_cycles {
                apply_template_substitution(tc, rename);
            }
            rename_expr_signals(&mut guard.condition, rename);
        }
        ModuleMacroStmt::Reflex(reflex) => {
            apply_template_substitution(&mut reflex.name, rename);
            for gname in &mut reflex.guard_names {
                apply_template_substitution(gname, rename);
            }
            for rs in &mut reflex.statements {
                substitute_reflex_stmt(rs, rename);
            }
        }
        ModuleMacroStmt::Property(prop) => {
            apply_template_substitution(&mut prop.name, rename);
            rename_property_signals(&mut prop.formula, rename);
        }
        ModuleMacroStmt::PatternCall(call) => {
            apply_template_substitution(&mut call.pattern_name, rename);
            for arg in &mut call.arguments {
                match arg {
                    crate::ast::pattern::PatternArg::SignalRef(name)
                    | crate::ast::pattern::PatternArg::PatternRef(name) => {
                        apply_template_substitution(name, rename);
                    }
                    _ => {}
                }
            }
        }
        ModuleMacroStmt::ForLoop { body, .. } => {
            for s in body {
                substitute_stmt(s, rename);
            }
        }
        ModuleMacroStmt::LetBinding { name, value, .. } => {
            apply_template_substitution(name, rename);
            rename_expr_signals(value, rename);
        }
    }
}

fn substitute_reflex_stmt(stmt: &mut ReflexMacroStmt, rename: &HashMap<String, String>) {
    match stmt {
        ReflexMacroStmt::Assignment(assignment) => {
            apply_template_substitution(&mut assignment.target, rename);
            rename_expr_signals(&mut assignment.value, rename);
        }
        ReflexMacroStmt::LetBinding { name, value, .. } => {
            apply_template_substitution(name, rename);
            rename_expr_signals(value, rename);
        }
        ReflexMacroStmt::OnBlock { guard_names, body } => {
            for gname in guard_names {
                apply_template_substitution(gname, rename);
            }
            for s in body {
                substitute_reflex_stmt(s, rename);
            }
        }
        ReflexMacroStmt::ForLoop { body, .. } => {
            for s in body {
                substitute_reflex_stmt(s, rename);
            }
        }
        ReflexMacroStmt::IfElse { condition, true_branch, false_branch } => {
            rename_expr_signals(condition, rename);
            for s in true_branch {
                substitute_reflex_stmt(s, rename);
            }
            for s in false_branch {
                substitute_reflex_stmt(s, rename);
            }
        }
        ReflexMacroStmt::Match { expr, arms } => {
            rename_expr_signals(expr, rename);
            for arm in arms {
                for s in &mut arm.body {
                    substitute_reflex_stmt(s, rename);
                }
            }
        }
    }
}
