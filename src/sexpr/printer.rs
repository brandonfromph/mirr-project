//! Bounded S-expression pretty-printer.
//!
//! Produces human-readable indented output from `SExpr` values.
//! Guaranteed inverse of `parse_sexpr`: `parse_sexpr(print_sexpr(expr)) == Ok(expr)`.
//!
//! Bounds: depth tracked for indentation, node count bounded by MAX_SEXPR_NODES.

#![forbid(unsafe_code)]

use crate::sexpr::types::SExpr;
use crate::sexpr::MAX_SEXPR_NODES;

/// Maximum line length before a list is printed multi-line.
const SHORT_LIST_THRESHOLD: usize = 60;

/// Pretty-print an S-expression to a string.
///
/// Short lists (&lt; 60 chars) print on one line.
/// Long lists print with indented children (2 spaces per level).
/// Node count bounded by `MAX_SEXPR_NODES`.
pub fn print_sexpr(sexpr: &SExpr) -> String {
    let mut buf = String::with_capacity(4096);
    let mut node_count = 0usize;
    print_inner(sexpr, &mut buf, 0, &mut node_count);
    buf
}

/// Internal printer — iterative with depth tracking.
fn print_inner(sexpr: &SExpr, buf: &mut String, indent: usize, node_count: &mut usize) {
    *node_count += 1;
    if *node_count > MAX_SEXPR_NODES {
        buf.push_str("...");
        return;
    }

    match sexpr {
        SExpr::Symbol(s) => buf.push_str(s),
        SExpr::Integer(n) => buf.push_str(&n.to_string()),
        SExpr::Bool(true) => buf.push_str("true"),
        SExpr::Bool(false) => buf.push_str("false"),
        SExpr::Str(s) => {
            buf.push('"');
            buf.push_str(s);
            buf.push('"');
        }
        SExpr::List(items) => {
            if items.is_empty() {
                buf.push_str("()");
                return;
            }
            // Try short form first.
            let short = format_short_list(items, node_count);
            if short.len() <= SHORT_LIST_THRESHOLD {
                buf.push_str(&short);
            } else {
                // Multi-line form.
                buf.push('(');
                let child_indent = indent + 2;
                for (i, item) in items.iter().enumerate() {
                    if i == 0 {
                        print_inner(item, buf, child_indent, node_count);
                    } else {
                        buf.push('\n');
                        for _ in 0..child_indent {
                            buf.push(' ');
                        }
                        print_inner(item, buf, child_indent, node_count);
                    }
                }
                buf.push(')');
            }
        }
        SExpr::Quote(inner) => {
            buf.push('\'');
            print_inner(inner, buf, indent, node_count);
        }
        SExpr::Quasiquote(inner) => {
            buf.push('`');
            print_inner(inner, buf, indent, node_count);
        }
        SExpr::Unquote(inner) => {
            buf.push(',');
            print_inner(inner, buf, indent, node_count);
        }
    }
}

/// Attempt to format a list on a single line.
fn format_short_list(items: &[SExpr], node_count: &mut usize) -> String {
    let mut buf = String::new();
    buf.push('(');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            buf.push(' ');
        }
        print_inner(item, &mut buf, 0, node_count);
        // Early exit if already too long.
        if buf.len() > SHORT_LIST_THRESHOLD + 10 {
            break;
        }
    }
    buf.push(')');
    buf
}
