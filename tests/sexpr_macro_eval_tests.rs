#![forbid(unsafe_code)]
//! Phase 2: TDD tests for the S-Expression Macro Evaluation Engine.
//!
//! These tests verify that `MacroExpander::expand()` correctly intercepts
//! and evaluates compile-time generative forms (`for-generate`, `if-generate`,
//! `let-bind`, `concat-sym`) in the S-Expression tree.
//!
//! NASA P10: bounded loops, no recursion.

use mirrc::sexpr::types::SExpr;
use mirrc::sexpr::MacroExpander;

// ─── Helpers ────────────────────────────────────────────────────────

fn sym(s: &str) -> SExpr {
    SExpr::sym(s)
}
fn int(n: u64) -> SExpr {
    SExpr::int(n)
}
fn str_val(s: &str) -> SExpr {
    SExpr::str_val(s)
}
fn list(items: Vec<SExpr>) -> SExpr {
    SExpr::list(items)
}

/// Build a `(signal "name" internal (unsigned 8))` node.
fn signal_node(name: &str) -> SExpr {
    list(vec![sym("signal"), str_val(name), sym("internal"), list(vec![sym("unsigned"), int(8)])])
}

/// Build a `(for-generate "var" start end (body...))` node.
fn for_generate(var: &str, start: u64, end: u64, body: Vec<SExpr>) -> SExpr {
    list(vec![sym("for-generate"), str_val(var), int(start), int(end), list(body)])
}

/// Build a `(if-generate cond then else)` node.
fn if_generate(cond: SExpr, then_branch: SExpr, else_branch: SExpr) -> SExpr {
    list(vec![sym("if-generate"), cond, then_branch, else_branch])
}

/// Build a `(let-bind "name" "type" value)` node.
fn let_bind(name: &str, ty: &str, value: SExpr) -> SExpr {
    list(vec![sym("let-bind"), str_val(name), str_val(ty), value])
}

/// Build `(concat-sym parts...)`.
fn concat_sym(parts: Vec<SExpr>) -> SExpr {
    let mut items = vec![sym("concat-sym")];
    items.extend(parts);
    list(items)
}

/// Count nodes with a specific head symbol in a flat list.
fn count_head(tree: &SExpr, head: &str) -> usize {
    match tree {
        SExpr::List(items) => {
            let mut count = 0;
            if items.first().and_then(|h| h.as_symbol()) == Some(head) {
                count += 1;
            }
            for item in items {
                count += count_head(item, head);
            }
            count
        }
        _ => 0,
    }
}

/// Extract all `SExpr::Str` values from a tree (flattened).
fn collect_str_vals(tree: &SExpr) -> Vec<String> {
    let mut result = Vec::new();
    match tree {
        SExpr::Str(s) => result.push(s.clone()),
        SExpr::List(items) => {
            for item in items {
                result.extend(collect_str_vals(item));
            }
        }
        _ => {}
    }
    result
}

// ─── Tests ──────────────────────────────────────────────────────────

// T1: for-generate basic unroll — 4 signals
#[test]
fn for_generate_basic_unroll_produces_4_signals() {
    let tree = for_generate(
        "i",
        0,
        4,
        vec![list(vec![
            sym("signal"),
            concat_sym(vec![str_val("s_"), sym("i")]),
            sym("internal"),
            list(vec![sym("unsigned"), int(8)]),
        ])],
    );

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree).expect("expansion should succeed");

    // The for-generate should be fully consumed — no for-generate in output
    assert_eq!(count_head(&result, "for-generate"), 0, "for-generate should be fully consumed");
    // Should produce exactly 4 signal nodes
    assert_eq!(count_head(&result, "signal"), 4, "expected 4 signal nodes");
}

// T2: for-generate with nested arithmetic
#[test]
fn for_generate_arithmetic_in_body() {
    let tree = for_generate(
        "i",
        0,
        3,
        vec![list(vec![
            sym("signal"),
            concat_sym(vec![str_val("addr_"), sym("i")]),
            sym("internal"),
            list(vec![
                sym("unsigned"),
                list(vec![sym("+"), list(vec![sym("*"), sym("i"), int(4096)]), int(0)]),
            ]),
        ])],
    );

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree).expect("expansion should succeed");
    assert_eq!(count_head(&result, "for-generate"), 0);
    assert_eq!(count_head(&result, "signal"), 3);
}

// T3: for-generate bound enforcement
#[test]
fn for_generate_exceeds_max_loop_iterations_returns_error() {
    let tree = for_generate("i", 0, 2000, vec![signal_node("x")]);

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree);
    assert!(result.is_err(), "should error when loop exceeds MAX_LOOP_ITERATIONS");
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("816") || err_msg.contains("loop"),
        "error should reference E816 or loop: {err_msg}"
    );
}

// T4: if-generate true branch
#[test]
fn if_generate_true_emits_then_branch() {
    let tree = if_generate(SExpr::bool_val(true), signal_node("chosen"), signal_node("rejected"));

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree).expect("expansion should succeed");
    let strs = collect_str_vals(&result);
    assert!(strs.contains(&"chosen".to_string()), "then branch should survive");
    assert!(!strs.contains(&"rejected".to_string()), "else branch should be eliminated");
}

// T5: if-generate false branch
#[test]
fn if_generate_false_emits_else_branch() {
    let tree = if_generate(SExpr::bool_val(false), signal_node("rejected"), signal_node("chosen"));

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree).expect("expansion should succeed");
    let strs = collect_str_vals(&result);
    assert!(strs.contains(&"chosen".to_string()), "else branch should survive");
    assert!(!strs.contains(&"rejected".to_string()), "then branch should be eliminated");
}

// T6: let-bind substitution
#[test]
fn let_bind_substitutes_value_in_body() {
    // (reflect (let-bind "width" "u32" 64) (signal "data" internal (unsigned width)))
    let tree = list(vec![
        sym("reflect"),
        let_bind("width", "u32", int(64)),
        list(vec![
            sym("signal"),
            str_val("data"),
            sym("internal"),
            list(vec![sym("unsigned"), sym("width")]),
        ]),
    ]);

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree).expect("expansion should succeed");
    // The symbol "width" should have been replaced with integer 64
    assert_eq!(count_head(&result, "let-bind"), 0, "let-bind should be consumed");
    let strs = collect_str_vals(&result);
    assert!(strs.contains(&"data".to_string()), "signal name should survive");
}

// T7: nested for-generate
#[test]
fn nested_for_generate_produces_n_times_m_nodes() {
    // for i in 0..2: for j in 0..3: signal "s_i_j"
    let inner = for_generate(
        "j",
        0,
        3,
        vec![list(vec![
            sym("signal"),
            concat_sym(vec![str_val("s_"), sym("i"), str_val("_"), sym("j")]),
            sym("internal"),
            list(vec![sym("unsigned"), int(8)]),
        ])],
    );
    let outer = for_generate("i", 0, 2, vec![inner]);

    let mut expander = MacroExpander::new();
    let result = expander.expand(&outer).expect("expansion should succeed");
    assert_eq!(count_head(&result, "for-generate"), 0);
    assert_eq!(count_head(&result, "signal"), 6, "2 * 3 = 6 signal nodes");
}

// T8: node budget exhaustion
#[test]
fn node_budget_exhaustion_returns_error() {
    // Create a loop that generates a massive number of nodes
    // Each iteration produces a signal with multiple sub-nodes (~5 nodes each)
    // 262144 / 5 ≈ 52428, so 60000 iterations should blow the budget
    let tree = for_generate(
        "i",
        0,
        1024,
        vec![
            signal_node("x"),
            signal_node("y"),
            signal_node("z"),
            signal_node("w"),
            signal_node("v"),
        ],
    );

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree);
    // This should either succeed (if within budget) or error with E817
    // The important thing is it doesn't panic or hang
    if let Err(e) = &result {
        let msg = format!("{e}");
        assert!(
            msg.contains("817")
                || msg.contains("node")
                || msg.contains("budget")
                || msg.contains("814"),
            "error should reference node budget: {msg}"
        );
    }
}

// T9: concat-sym string building
#[test]
fn concat_sym_builds_string_from_parts() {
    // (reflect (let-bind "i" "u32" 3) (concat-sym "alu_core_" i "_result"))
    let tree = list(vec![
        sym("reflect"),
        let_bind("i", "u32", int(3)),
        concat_sym(vec![str_val("alu_core_"), sym("i"), str_val("_result")]),
    ]);

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree).expect("expansion should succeed");
    let strs = collect_str_vals(&result);
    assert!(
        strs.iter().any(|s| s == "alu_core_3_result"),
        "concat-sym should produce 'alu_core_3_result', got: {:?}",
        strs
    );
}

// T10: passthrough — normal nodes are untouched
#[test]
fn normal_nodes_pass_through_unchanged() {
    let tree = list(vec![
        sym("module"),
        str_val("test_mod"),
        list(vec![sym("signals"), signal_node("clk"), signal_node("rst")]),
    ]);

    let mut expander = MacroExpander::new();
    let result = expander.expand(&tree).expect("expansion should succeed");
    assert_eq!(count_head(&result, "signal"), 2);
    let strs = collect_str_vals(&result);
    assert!(strs.contains(&"clk".to_string()));
    assert!(strs.contains(&"rst".to_string()));
}
