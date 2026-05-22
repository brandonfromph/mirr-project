//! CLI entry point for the MIRR compiler toolchain.
//!
//! Provides `mirr-parse` functionality: reads `.mirr` source files and emits
//! structured JSON, DOT graphs, or pretty-printed IR to stdout.

#![forbid(unsafe_code)]
#![deny(warnings)]

use std::{env, fs, process};

use nasa_rust_project::{parse_mirr, BootstrapOpts, BootstrapRunner, TemporalGuardCompiler};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Check for help flag
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        process::exit(0);
    }

    // Parse command line arguments
    let mut compile_temporal = false;
    let mut emit_json = false;
    let mut emit_dot = false;
    let mut emit_verilog = false;
    let mut selfhost_compile = false;
    let mut selfhost_json = false;
    let mut selfhost_verilog = false;
    let mut input_path = None;

    for arg in args {
        match arg.as_str() {
            "--compile" | "-c" => compile_temporal = true,
            "--json" | "-j" => emit_json = true,
            "--dot" | "-d" => emit_dot = true,
            "--verilog" => emit_verilog = true,
            "--selfhost-compile" => selfhost_compile = true,
            "--selfhost-compile-json" => {
                selfhost_compile = true;
                selfhost_json = true;
            }
            "--selfhost-compile-verilog" => {
                selfhost_compile = true;
                selfhost_verilog = true;
            }
            path if path.starts_with('-') => {
                eprintln!("Unknown option: {}", path);
                print_help();
                process::exit(1);
            }
            path => {
                if input_path.is_some() {
                    eprintln!("Multiple input files specified");
                    print_help();
                    process::exit(1);
                }
                input_path = Some(path.to_string());
            }
        }
    }

    let input_path = match input_path {
        Some(path) => path,
        None => {
            eprintln!("Usage: nasa-rust-project [OPTIONS] <file.mirr>");
            print_help();
            process::exit(1);
        }
    };

    // --selfhost-compile: run the bootstrap runner and exit.
    if selfhost_compile {
        let runner = BootstrapRunner::new(BootstrapOpts {
            run_mirr_stages: false,
            fixture_root: None,
            emit_netlist_json: selfhost_json,
            emit_netlist_verilog: selfhost_verilog,
            fail_fast: false,
            run_lexer_driver: false,
        });
        let result = runner.run(&input_path);
        result.print_report();
        eprintln!("{}", result.summary_line());
        if result.ok {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }

    // Try to read as UTF-8 first; if that fails we'll attempt to
    // interpret the bytes as UTF-16 little-endian (the form PowerShell
    // tends to write) and convert it.
    let mut source = match fs::read_to_string(&input_path) {
        Ok(text) => text,
        Err(first_err) => {
            // maybe the file is UTF-16? try a manual decode
            match fs::read(&input_path) {
                Ok(bytes) => {
                    // if length is odd we can't decode, give up
                    if bytes.len() % 2 == 0 {
                        let words: Vec<u16> = bytes
                            .chunks(2)
                            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                            .collect();
                        if let Ok(decoded) = String::from_utf16(&words) {
                            decoded
                        } else {
                            eprintln!("Failed to read '{}': {}", input_path, first_err);
                            process::exit(1);
                        }
                    } else {
                        eprintln!("Failed to read '{}': {}", input_path, first_err);
                        process::exit(1);
                    }
                }
                Err(_e2) => {
                    eprintln!("Failed to read '{}': {}", input_path, first_err);
                    process::exit(1);
                }
            }
        }
    };

    // Remove leading UTF-8 BOM (U+FEFF) if present. The UTF-16 code path above
    // already handles the UTF-16 BOM during decoding, so only U+FEFF needs
    // stripping here. U+FFFE is a noncharacter, not a BOM — removing it was
    // incorrect (MED-03 fix). Use strip_prefix to avoid O(n) String::remove(0).
    if let Some(stripped) = source.strip_prefix('\u{FEFF}') {
        source = stripped.to_string();
    }

    // MEGA-10: Pre-process with ergonomic macro processor before parsing
    let processed_source = nasa_rust_project::compiler::macro_proc::expand_macros(&source);

    // Parse the MIRR file
    let program = match parse_mirr(&processed_source) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("Parse error: {}", error);
            process::exit(1);
        }
    };

    // If no temporal compilation requested, just print the parsed AST
    if !compile_temporal {
        println!("{:#?}", program);
        return;
    }

    let config = nasa_rust_project::pipeline::PipelineConfig {
        temporal: true,
        base_dir: Some(
            std::path::PathBuf::from(&input_path)
                .parent()
                .unwrap_or(std::path::Path::new(""))
                .to_path_buf(),
        ),
        ..Default::default()
    };

    let pipeline_result =
        match nasa_rust_project::pipeline::run_pipeline_on_program(program, &config) {
            Ok(result) => result,
            Err(errors) => {
                for err in errors.errors {
                    eprintln!("Pipeline error: {}", err);
                }
                process::exit(1);
            }
        };

    let netlist = match pipeline_result.temporal_netlist {
        Some(netlist) => netlist,
        None => {
            eprintln!("Temporal compilation was skipped");
            process::exit(1);
        }
    };

    // Output results
    // support multiple output formats; if none requested show summary
    let compiler = TemporalGuardCompiler::new();
    if emit_json {
        match compiler.emit_netlist_json(&netlist) {
            Ok(json) => println!("{}", json),
            Err(error) => {
                eprintln!("JSON emission error: {}", error);
                process::exit(1);
            }
        }
    }
    if emit_dot {
        match compiler.emit_netlist_dot(&netlist) {
            Ok(dot) => println!("{}", dot),
            Err(error) => {
                eprintln!("DOT emission error: {}", error);
                process::exit(1);
            }
        }
    }
    if emit_verilog {
        match compiler.emit_netlist_verilog(&netlist) {
            Ok(v) => println!("{}", v),
            Err(error) => {
                eprintln!("Verilog emission error: {}", error);
                process::exit(1);
            }
        }
    }
    if !emit_json && !emit_dot && !emit_verilog {
        // Default output: summary and detailed information
        println!("Temporal Guard Compilation Results:");
        println!("{}", netlist.summary());

        println!("\nDetailed Guard Information:");
        for (i, guard) in netlist.guards.iter().enumerate() {
            println!("Guard {}: {:?}", i + 1, guard);
        }

        println!("\nGenerated Signals:");
        for signal in &netlist.signals {
            println!("  - {} ({:?})", signal.name, signal.kind);
        }
    }
}

fn print_help() {
    println!("NASA Rust Project - MIRR Compiler");
    println!();
    println!("Usage: nasa-rust-project [OPTIONS] <file.mirr>");
    println!();
    println!("Options:");
    println!("  -c, --compile              Compile temporal guards to low-level netlist");
    println!("  -j, --json                 Emit netlist as JSON (requires --compile)");
    println!(
        "  -d, --dot                  Emit netlist as Graphviz DOT format (requires --compile)"
    );
    println!(
        "      --verilog              Emit netlist as simple Verilog module (requires --compile)"
    );
    println!("      --selfhost-compile     Run full self-hosting bootstrap pipeline");
    println!("      --selfhost-compile-json  Same as above, also emit netlist JSON");
    println!("      --selfhost-compile-verilog  Same as above, also emit Verilog");
    println!("  -h, --help                 Show this help message");
    println!();
    println!("Examples:");
    println!("  nasa-rust-project example.mirr                    # Parse and display AST");
    println!("  nasa-rust-project --compile example.mirr          # Compile temporal guards");
    println!("  nasa-rust-project --compile --json example.mirr   # Compile and emit JSON");
    println!("  nasa-rust-project --compile --dot example.mirr    # Compile and emit DOT");
    println!("  nasa-rust-project --selfhost-compile example.mirr # Self-hosting pipeline");
}
