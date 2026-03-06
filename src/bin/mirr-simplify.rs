//! CLI tool for MIRR logic simplification

use std::env;
use std::fs;
use nasa_rust_project::ast::Expr;
use nasa_rust_project::simplify::simplify_expr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: mirr-simplify <expr-json-file>");
        std::process::exit(1);
    }
    let input_path = &args[1];
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let expr: Expr = serde_json::from_str(&input).expect("Invalid Expr JSON");
    let simplified = simplify_expr(expr);
    let output = serde_json::to_string_pretty(&simplified).expect("Failed to serialize output");
    println!("{}", output);
}
