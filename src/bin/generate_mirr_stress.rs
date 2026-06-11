//! CLI for generating MIRR stress-test code.
//!
//! This binary mirrors the earlier Python prototype but is written in Rust for
//! type safety, easy distribution, and reuse within the Cargo build/test
//! infrastructure. The generated output is printed to STDOUT so callers can
//! redirect it to a file or pipe it into the compiler.

#![forbid(unsafe_code)]

use clap::Parser;
use std::fmt::Write as FmtWrite;

fn generate_mux_forest(size: usize) -> anyhow::Result<String> {
    // create a module with `size` boolean control signals and a reflex that
    // nests `if` statements to mimic a deep mux hierarchy.  The controls are
    // logically related by simple dependence in the reflex body, which should
    // be enough to exercise SmaRTLy's SAT-based redundancy elimination.
    let mut s = String::new();
    writeln!(&mut s, "module mux_forest {{")?;
    writeln!(&mut s, "    signal in_sig: in bool;")?;
    writeln!(&mut s, "    signal out_sig: out bool;")?;
    for i in 0..size {
        writeln!(&mut s, "    signal s{}: internal bool;", i)?;
    }
    writeln!(&mut s)?;
    writeln!(&mut s, "    reflex r {{")?;
    writeln!(&mut s, "        on always {{")?;
    // create a simple chain of assignments so that signals depend on their
    // predecessor and the input; this gives a linear mux-like structure.
    if size > 0 {
        writeln!(&mut s, "            s0 = in_sig;")?;
        for i in 1..size {
            writeln!(&mut s, "            s{} = s{} || in_sig;", i, i - 1)?;
        }
        writeln!(&mut s, "            out_sig = s{};", size - 1)?;
    } else {
        writeln!(&mut s, "            out_sig = in_sig;")?;
    }
    writeln!(&mut s, "        }}")?;
    writeln!(&mut s, "    }}")?;
    writeln!(&mut s, "}}\n")?;
    Ok(s)
}

fn generate_temporal_chain(size: usize) -> anyhow::Result<String> {
    // build a sequence of guards that delay the input by one cycle each;
    // the final guard triggers a reflex that copies the input to output.
    let mut s = String::new();
    writeln!(&mut s, "module temporal_chain {{")?;
    writeln!(&mut s, "    signal in_sig: in bool;")?;
    writeln!(&mut s, "    signal out_sig: out bool;")?;
    writeln!(&mut s)?;

    for i in 0..size {
        writeln!(&mut s, "    guard g{} {{", i)?;
        if i == 0 {
            writeln!(&mut s, "        when in_sig")?;
        } else {
            writeln!(&mut s, "        when g{}", i - 1)?;
        }
        writeln!(&mut s, "        for 1 cycles;")?;
        writeln!(&mut s, "    }}")?;
        writeln!(&mut s)?;
    }

    writeln!(&mut s, "    reflex r {{")?;
    if size > 0 {
        writeln!(&mut s, "        on g{} {{", size - 1)?;
    } else {
        writeln!(&mut s, "        on always {{")?;
    }
    writeln!(&mut s, "            out_sig = in_sig;")?;
    writeln!(&mut s, "        }}")?;
    writeln!(&mut s, "    }}")?;
    writeln!(&mut s, "}}\n")?;
    Ok(s)
}

fn generate_width_chain(size: usize) -> anyhow::Result<String> {
    // create a chain of arithmetic assignments that form a simple SCC
    // between the first and last signal.  All signals are u32 for simplicity
    // but the sequence of additions/multiplications should trigger
    // bit-width inference in FIRWINE.
    let mut s = String::new();
    writeln!(&mut s, "module width_chain {{")?;
    writeln!(&mut s, "    signal in_sig: in u32;")?;
    writeln!(&mut s, "    signal out_sig: out u32;")?;
    for i in 0..size {
        writeln!(&mut s, "    signal s{}: internal u32;", i)?;
    }
    writeln!(&mut s)?;

    writeln!(&mut s, "    reflex r {{")?;
    writeln!(&mut s, "        on always {{")?;
    if size > 0 {
        writeln!(&mut s, "            s0 = in_sig + 1;")?;
        for i in 1..size {
            writeln!(&mut s, "            s{} = s{} + {};", i, i - 1, i)?;
        }
        // close the loop by reassigning s0 from the last element
        writeln!(&mut s, "            s0 = s{} + 1;", size - 1)?;
        writeln!(&mut s, "            out_sig = s0;")?;
    } else {
        writeln!(&mut s, "            out_sig = in_sig;")?;
    }
    writeln!(&mut s, "        }}")?;
    writeln!(&mut s, "    }}")?;
    writeln!(&mut s, "}}\n")?;
    Ok(s)
}

#[derive(Parser, Debug)]
#[command(about = "Generate MIRR stress test code")]
pub struct Args {
    #[arg(long, help = "Which template to emit")]
    pub typ: String,

    #[arg(long, default_value_t = 100, help = "Rough size parameter for the template")]
    pub size: usize,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let code = match args.typ.as_str() {
        "mux_forest" => generate_mux_forest(args.size),
        "temporal_chain" => generate_temporal_chain(args.size),
        "width_chain" => generate_width_chain(args.size),
        _ => return Err(anyhow::anyhow!("invalid type: {}", args.typ)),
    }?;

    print!("{}", code);
    Ok(())
}
