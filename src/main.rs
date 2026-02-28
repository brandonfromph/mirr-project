#![forbid(unsafe_code)]
#![deny(warnings)]

use std::{env, fs, process};

use nasa_rust_project::parse_mirr;

fn main() {
    let mut args = env::args().skip(1);

    let input_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Usage: nasa-rust-project <file.mirr>");
            process::exit(1);
        }
    };

    let source = match fs::read_to_string(&input_path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("Failed to read '{input_path}': {error}");
            process::exit(1);
        }
    };

    match parse_mirr(&source) {
        Ok(program) => {
            println!("{:#?}", program);
        }
        Err(error) => {
            eprintln!("Parse error: {error}");
            process::exit(1);
        }
    }
}