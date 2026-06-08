#![forbid(unsafe_code)]

//! Criterion benchmarks for the MIRR S-expression IR subsystem.
//!
//! Four groups — parse, print, convert, eval — exercise the full sexpr
//! pipeline at increasing scale. All generators use bounded iteration.

use criterion::{criterion_group, criterion_main, Criterion};
use mirrc::parse_mirr;
use mirrc::sexpr::{ast_to_sexpr, eval, parse_sexpr, print_sexpr, sexpr_to_ast, EvalState};
use std::hint::black_box;

// ---------------------------------------------------------------------------
// Bound constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

const MAX_BENCH_SEXPR_NODES: usize = 100;
const MAX_BENCH_SIGNALS: usize = 2;

// ---------------------------------------------------------------------------
// Input generators (bounded iteration, NASA-compliant)
// ---------------------------------------------------------------------------

/// Small S-expr: a single signal declaration.
fn small_sexpr_str() -> String {
    "(signal \"sensor\" input (unsigned 16))".to_string()
}

/// Medium S-expr: a module fragment with ~10 nodes.
fn medium_sexpr_str() -> String {
    let mut s = String::with_capacity(512);
    s.push_str("(module \"bench_medium\"\n");
    for i in 0..4_usize {
        s.push_str(&format!("  (signal \"s{i}\" input (unsigned 16))\n"));
    }
    for i in 0..4_usize {
        s.push_str(&format!("  (signal \"a{i}\" output (bool))\n"));
    }
    s.push_str("  (guard \"g0\" (> (ref \"s0\") 100) 3)\n)");
    s
}

/// Large S-expr: a module with 100 signal nodes.
fn large_sexpr_str() -> String {
    let mut s = String::with_capacity(4096);
    s.push_str("(module \"bench_large\"\n");
    for i in 0..MAX_BENCH_SEXPR_NODES {
        s.push_str(&format!("  (signal \"s{i}\" input (unsigned 16))\n"));
    }
    s.push(')');
    s
}

/// Small MIRR source: 2 signals, 1 guard, 1 reflex.
fn small_mirr() -> String {
    "module bench_small {\n    signal sensor: in u16;\n    signal alarm: out bool;\n\n\
     \x20   guard g {\n        when sensor > 100\n        for 3 cycles;\n    }\n\n\
     \x20   reflex r {\n        on g {\n            alarm = true;\n        }\n    }\n}\n"
        .to_string()
}

/// Medium MIRR source: 4 signals, 2 guards, 2 reflexes.
fn medium_mirr() -> String {
    let mut src = String::with_capacity(1024);
    src.push_str("module bench_medium {\n");
    for i in 0..MAX_BENCH_SIGNALS {
        src.push_str(&format!("    signal s{i}: in u16;\n"));
    }
    for i in 0..MAX_BENCH_SIGNALS {
        src.push_str(&format!("    signal a{i}: out bool;\n"));
    }
    for i in 0..MAX_BENCH_SIGNALS {
        let thresh = (i + 1) * 50;
        let cyc = (i + 1) * 2;
        src.push_str(&format!(
            "    guard g{i} {{\n        when s{i} > {thresh}\n        for {cyc} cycles;\n    }}\n"
        ));
    }
    for i in 0..MAX_BENCH_SIGNALS {
        src.push_str(&format!(
            "    reflex r{i} {{\n        on g{i} {{\n            a{i} = true;\n        }}\n    }}\n"
        ));
    }
    src.push_str("}\n");
    src
}

// ---------------------------------------------------------------------------
// Benchmark groups
// ---------------------------------------------------------------------------

fn bench_sexpr_parse(c: &mut Criterion) {
    let small = small_sexpr_str();
    let medium = medium_sexpr_str();
    let large = large_sexpr_str();
    let mut group = c.benchmark_group("sexpr_parse");
    group.bench_function("small", |b| b.iter(|| parse_sexpr(black_box(&small))));
    group.bench_function("medium", |b| b.iter(|| parse_sexpr(black_box(&medium))));
    group.bench_function("large", |b| b.iter(|| parse_sexpr(black_box(&large))));
    group.finish();
}

fn bench_sexpr_print(c: &mut Criterion) {
    let small = parse_sexpr(&small_sexpr_str()).expect("parse small");
    let medium = parse_sexpr(&medium_sexpr_str()).expect("parse medium");
    let large = parse_sexpr(&large_sexpr_str()).expect("parse large");
    let mut group = c.benchmark_group("sexpr_print");
    group.bench_function("small", |b| b.iter(|| print_sexpr(black_box(&small))));
    group.bench_function("medium", |b| b.iter(|| print_sexpr(black_box(&medium))));
    group.bench_function("large", |b| b.iter(|| print_sexpr(black_box(&large))));
    group.finish();
}

fn bench_sexpr_convert(c: &mut Criterion) {
    let small_prog = parse_mirr(&small_mirr()).expect("parse small mirr");
    let medium_prog = parse_mirr(&medium_mirr()).expect("parse medium mirr");
    let small_sx = ast_to_sexpr(&small_prog);
    let medium_sx = ast_to_sexpr(&medium_prog);
    let mut group = c.benchmark_group("sexpr_convert");
    group.bench_function("ast_to_sexpr_small", |b| b.iter(|| ast_to_sexpr(black_box(&small_prog))));
    group.bench_function("ast_to_sexpr_medium", |b| {
        b.iter(|| ast_to_sexpr(black_box(&medium_prog)))
    });
    group.bench_function("sexpr_to_ast_small", |b| b.iter(|| sexpr_to_ast(black_box(&small_sx))));
    group.bench_function("sexpr_to_ast_medium", |b| b.iter(|| sexpr_to_ast(black_box(&medium_sx))));
    group.finish();
}

fn bench_sexpr_eval(c: &mut Criterion) {
    let if_expr = parse_sexpr("(if true (cons 1 (cons 2 (list))) (cons 3 (list)))").unwrap();
    let list_expr = parse_sexpr("(car (cdr (cons 1 (cons 2 (cons 3 (list))))))").unwrap();
    let mut group = c.benchmark_group("sexpr_eval");
    group
        .bench_function("if_expr", |b| b.iter(|| eval(black_box(&if_expr), &mut EvalState::new())));
    group.bench_function("list_ops", |b| {
        b.iter(|| eval(black_box(&list_expr), &mut EvalState::new()))
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_sexpr_parse,
    bench_sexpr_print,
    bench_sexpr_convert,
    bench_sexpr_eval
);
criterion_main!(benches);
