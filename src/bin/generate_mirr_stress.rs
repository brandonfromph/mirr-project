//! CLI for generating MIRR stress-test code.
//!
//! This binary mirrors the earlier Python prototype but is written in Rust for
//! type safety, easy distribution, and reuse within the Cargo build/test
//! infrastructure. The generated output is printed to STDOUT so callers can
//! redirect it to a file or pipe it into the compiler.

#![forbid(unsafe_code)]

use clap::{Arg, Command};
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

fn main() -> anyhow::Result<()> {
    let matches = Command::new("generate_mirr_stress")
        .about("Generate MIRR stress test code")
        .arg(
            Arg::new("type")
                .long("type")
                .value_parser(["mux_forest", "temporal_chain", "width_chain"])
                .required(true)
                .help("Which template to emit"),
        )
        .arg(
            Arg::new("size")
                .long("size")
                .value_parser(clap::value_parser!(usize))
                .default_value("100")
                .help("Rough size parameter for the template"),
        )
        .get_matches();

    let size = *matches.get_one::<usize>("size").ok_or_else(|| anyhow::anyhow!("missing size"))?;
    let typ =
        matches.get_one::<String>("type").ok_or_else(|| anyhow::anyhow!("missing type"))?.as_str();

    let code = match typ {
        "mux_forest" => generate_mux_forest(size),
        "temporal_chain" => generate_temporal_chain(size),
        "width_chain" => generate_width_chain(size),
        _ => unreachable!(),
    }?;

    print!("{}", code);
    Ok(())
}
