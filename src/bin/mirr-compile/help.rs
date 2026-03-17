//! Help text for the mirr-compile CLI.

#![forbid(unsafe_code)]

pub(super) fn print_help() {
    println!("mirr-compile — Unified MIRR compilation driver (Phase 6)");
    println!();
    println!("Usage:");
    println!("  mirr-compile <file.mirr> [OPTIONS]");
    println!();
    println!("Emission Options:");
    println!("  --emit FORMAT       Output format: dot, verilog, json, sva, firrtl, rspu,");
    println!("                      testbench, scaffold, build-script, sexpr, cert (default: dot)");
    println!("  --output FILE, -o   Write output to FILE (default: stdout)");
    println!("  --target FAMILY     FPGA target: generic, xilinx-7, xilinx-us, intel-cyclone,");
    println!("                      lattice-ice40, lattice-ecp5, lattice-nexus (default: generic)");
    println!("  --sync-stages N     Input synchronizer stages, 0 to disable (default: 2)");
    println!("  --dsp-threshold N   Min operand bits for DSP inference, 0 to disable (default: 9)");
    println!("  --testbench         Also emit a self-checking testbench (with --emit verilog)");
    println!("  --scaffold          Also emit constraint template and build script");
    println!("  --strip-sva         Omit SVA assertions from verilog output (for synthesis)");
    println!("  --sva-file FILE     Write SVA properties to a separate bind file");
    println!("  --dot-detail expr   Show full AST trees in DOT output");
    println!("  --stats             Print detailed pipeline statistics");
    println!();
    println!("Toolchain Options (requires oss-cad-suite in PATH):");
    println!("  --formal            Run SymbiYosys formal verification");
    println!("  --formal-depth N    BMC depth (default: 20, max: 200)");
    println!("  --formal-prove      Also run k-induction prove");
    println!("  --formal-engine E   Solver: z3, yices, bitwuzla, btor (default: z3)");
    println!("  --lint              Run Verilator lint-only");
    println!("  --simulate          Run Verilator compiled simulation");
    println!("  --pnr               Run nextpnr place and route (Lattice targets)");
    println!("  --timing            Run icetime static timing analysis (iCE40 only)");
    println!("  --eqy               Run EQY equivalence checking");
    println!("  --totality          Run MEGA-4 totality check and generate proof certificate");
    println!("  --symbolic          Run MEGA-5 symbolic interval analysis");
    println!("  --toolchain-path D  Override oss-cad-suite root directory");
    println!();
    println!("  --help, -h          Show this help");
    println!();
    println!("Examples:");
    println!("  mirr-compile program.mirr --emit verilog -o out.sv");
    println!("  mirr-compile program.mirr --emit verilog --target lattice-ecp5 --scaffold");
    println!("  mirr-compile program.mirr --emit verilog --strip-sva --formal");
    println!("  mirr-compile program.mirr --emit verilog --lint");
    println!("  mirr-compile program.mirr --emit json | jq .");
    println!("  mirr-compile program.mirr --emit dot | dot -Tpng -o graph.png");
    println!("  mirr-compile program.mirr --emit rspu");
    println!("  mirr-compile program.mirr --emit cert --totality -o program.mirrcert");
}
