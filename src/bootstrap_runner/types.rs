//! Public types for the MIRR self-hosting bootstrap runner.

#![forbid(unsafe_code)]

use std::path::PathBuf;

/// Options controlling what the bootstrap runner verifies.
#[derive(Debug, Clone, Default)]
pub struct BootstrapOpts {
    /// Root directory used to locate golden fixtures.
    /// Defaults to the directory that contains the source file.
    /// Override to point at `{repo}/tests/fixtures/`.
    pub fixture_root: Option<PathBuf>,

    /// Emit the compiled netlist as pretty-printed JSON to stdout on success.
    pub emit_netlist_json: bool,

    /// Emit the compiled netlist as a simple Verilog module string.
    pub emit_netlist_verilog: bool,

    /// Stop after the first stage failure instead of collecting all results.
    pub fail_fast: bool,

    /// If true, run the experimental MIRR lexer driver to collect tokens from
    /// the compiler_mirr lexer module (off by default).
    pub run_lexer_driver: bool,
}

/// Result of one pipeline stage.
#[derive(Debug, Clone)]
pub struct StageResult {
    /// Human-readable stage name (e.g. "Parse", "Validate").
    pub name: String,
    /// `true` if the stage completed without errors.
    pub ok: bool,
    /// Short description of what happened (pass detail or failure reason).
    pub message: String,
}

/// Aggregated result of the full bootstrap run.
#[derive(Debug)]
pub struct BootstrapResult {
    /// Path of the compiled source file.
    pub source_path: PathBuf,
    /// Per-stage results, in pipeline order.
    pub stages: Vec<StageResult>,
    /// `true` iff every stage passed.
    pub ok: bool,
    /// Pretty-printed netlist JSON (populated when `opts.emit_netlist_json`
    /// is set and the pipeline reached Stage 3 successfully).
    pub netlist_json: Option<String>,

    /// Verilog module string (populated when `opts.emit_netlist_verilog` is set).
    pub netlist_verilog: Option<String>,
}

impl BootstrapResult {
    /// Print a structured report to stdout.
    pub fn print_report(&self) {
        let status = if self.ok { "PASS" } else { "FAIL" };
        println!("Self-Host Bootstrap: {} — {}", status, self.source_path.display());
        for (i, stage) in self.stages.iter().enumerate() {
            let icon = if stage.ok { "✓" } else { "✗" };
            println!("  Stage {}: {} [{}] {}", i + 1, icon, stage.name, stage.message);
        }
        if let Some(ref json) = self.netlist_json {
            println!("\n--- Netlist JSON ---");
            println!("{json}");
        }
        if let Some(ref v) = self.netlist_verilog {
            println!("\n--- Netlist Verilog ---");
            println!("{v}");
        }
    }

    /// Returns a one-line summary suitable for CI log output.
    pub fn summary_line(&self) -> String {
        let status = if self.ok { "PASS" } else { "FAIL" };
        let pass_count = self.stages.iter().filter(|s| s.ok).count();
        format!(
            "[SELF-HOST {status}] {}/{} stages passed — {}",
            pass_count,
            self.stages.len(),
            self.source_path.display()
        )
    }
}
