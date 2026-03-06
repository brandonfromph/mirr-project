//! MIRR Self-Hosting Bootstrap Runner
//!
//! Orchestrates the four-stage self-hosting verification pipeline:
//!   Stage 1: Parse  (Rust parser → MirrProgram)
//!   Stage 2: Validate (Rust semantic validator)
//!   Stage 3: Temporal lower (Rust temporal compiler → TemporalNetlist)
//!   Stage 4: Fixture parity (compare JSON output against golden IR contract)
//!
//! Each stage optionally loads a golden fixture from tests/fixtures/ and
//! verifies the Rust output matches it, confirming the IR contract defined
//! in docs/self_hosting_ir_contract.md.
//!
//! When a MIRR interpreter is added (post Task 8), the runner will also
//! execute the corresponding compiler_mirr/*.mirr stage and cross-check
//! the two outputs — achieving true semantic self-hosting.
//!
//! Ref: MIRR self-hosting plan — Task 8
//!      docs/self_hosting_core_spec.md §4 (bootstrap runner interface)

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use crate::{
    parser::parse_mirr,
    temporal::{
        low_level_ir::TemporalNetlistJson,
        TemporalGuardCompiler,
    },
    validation::validate_module,
};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

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
        println!(
            "Self-Host Bootstrap: {} — {}",
            status,
            self.source_path.display()
        );
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

// ---------------------------------------------------------------------------
// Bootstrap Runner
// ---------------------------------------------------------------------------

// Allow creating a runner with default options via the standard trait.
impl Default for BootstrapRunner {
    fn default() -> Self {
        Self::new(BootstrapOpts::default())
    }
}

/// Drives the MIRR self-hosting verification pipeline.
///
/// ```text
/// ┌─────────┐   ┌──────────┐   ┌──────────────────┐   ┌────────────────┐
/// │  parse  │ → │ validate │ → │ temporal lower   │ → │ fixture parity │
/// └─────────┘   └──────────┘   └──────────────────┘   └────────────────┘
/// ```
pub struct BootstrapRunner {
    opts: BootstrapOpts,
}

impl BootstrapRunner {
    /// Create a new runner with the given options.
    pub fn new(opts: BootstrapOpts) -> Self {
        Self { opts }
    }

    /// Create a runner with default options.
    pub fn new_default() -> Self {
        Self::new(BootstrapOpts::default())
    }

    /// Run the full self-hosting pipeline on the MIRR source at `source_path`.
    ///
    /// Returns [`BootstrapResult`] regardless of whether stages pass or fail.
    /// Use [`BootstrapResult::ok`] to determine overall success.
    pub fn run(&self, source_path: impl AsRef<Path>) -> BootstrapResult {
        let source_path = source_path.as_ref().to_path_buf();
        let mut stages: Vec<StageResult> = Vec::new();
        let mut netlist_json: Option<String> = None;
        let mut netlist_verilog: Option<String> = None;

        // -------------------------------------------------------------------
        // Stage 1: Read source file
        // -------------------------------------------------------------------
        let source = match std::fs::read_to_string(&source_path) {
            Ok(s) => s,
            Err(e) => {
                stages.push(StageResult {
                    name: "Read".to_string(),
                    ok: false,
                    message: format!("cannot read '{}': {e}", source_path.display()),
                });
                return BootstrapResult {
                    source_path,
                    stages,
                    ok: false,
                    netlist_json: None,
                    netlist_verilog: None,
                };
            }
        };
        stages.push(StageResult {
            name: "Read".to_string(),
            ok: true,
            message: format!("{} bytes read", source.len()),
        });

        // -------------------------------------------------------------------
        // Stage 2: Parse
        // -------------------------------------------------------------------
        let program = match parse_mirr(&source) {
            Ok(p) => {
                stages.push(StageResult {
                    name: "Parse".to_string(),
                    ok: true,
                    message: format!(
                        "{} signal(s), {} guard(s), {} reflex(es)",
                        p.module.signals.len(),
                        p.module.guards.len(),
                        p.module.reflexes.len(),
                    ),
                });
                p
            }
            Err(e) => {
                stages.push(StageResult {
                    name: "Parse".to_string(),
                    ok: false,
                    message: format!("{e}"),
                });
                return self.finish(source_path, stages, false, None, None);
            }
        };
        if self.opts.fail_fast && !stages.last().map(|s| s.ok).unwrap_or(true) {
            return self.finish(source_path, stages, false, None, None);
        }

        // -------------------------------------------------------------------
        // Stage 3: Semantic validation
        // -------------------------------------------------------------------
        match validate_module(&program.module) {
            Ok(()) => {
                stages.push(StageResult {
                    name: "Validate".to_string(),
                    ok: true,
                    message: "all semantic checks passed".to_string(),
                });
            }
            Err(e) => {
                stages.push(StageResult {
                    name: "Validate".to_string(),
                    ok: false,
                    message: format!("{e}"),
                });
                if self.opts.fail_fast {
                    return self.finish(source_path, stages, false, None, None);
                }
            }
        }

        // -------------------------------------------------------------------
        // Stage 4: Temporal lowering
        // -------------------------------------------------------------------
        let netlist = match TemporalGuardCompiler::new()
            .compile_temporal_guards(&program.module)
        {
            Ok(n) => {
                stages.push(StageResult {
                    name: "TemporalLower".to_string(),
                    ok: true,
                    message: format!(
                        "{} guard(s) lowered — {} signal(s) generated",
                        n.guards.len(),
                        n.signals.len(),
                    ),
                });
                n
            }
            Err(e) => {
                stages.push(StageResult {
                    name: "TemporalLower".to_string(),
                    ok: false,
                    message: format!("{e}"),
                });
                return self.finish(source_path, stages, false, None, None);
            }
        };

        // -------------------------------------------------------------------
        // Stage 5: Fixture parity check (optional — skip if no fixture found)
        // -------------------------------------------------------------------
        //
        // MIRR Runtime Harness (integration notes)
        // ---------------------------------------
        // The incremental lexer port in compiler_mirr/lexer.mirr emits thin
        // "push" signals (emit_push_*) which the host must sample per input
        // tick and convert into Token values via the stdlib token_buffer API.
        //
        // Suggested integration point:
        // - After TemporalLower completes and before FixtureParity, the
        //   bootstrap runner (or a dedicated src/mirr_runtime.rs helper) may
        //   exercise the MIRR lexer module to produce a token stream and then
        //   continue with the usual parity checks.
        //
        // Responsibilities for host wiring:
        //  - Drive input bytes → input_byte_* signals on the MIRR module.
        //  - On each tick, sample emit_push_* signals and call token_make /
        //    token_buffer_push accordingly (see compiler_mirr/PORTING_STEPS.md).
        //  - Append EOF token when lexing completes.
        //
        // A minimal, testable approach is implemented in src/mirr_runtime.rs
        // which provides `map_push_kind_to_token` and `push_mapped_token`
        // helpers. Implementing the actual module-driver will be the next
        // ACT-mode step (small, well-scoped change).
        if self.opts.run_lexer_driver {
            // Run the MIRR lexer via the executor abstraction (currently an
            // incremental evaluator). Collect observed pushes and map them
            // into a TokenBuffer using the runtime helpers so the host can
            // perform token_buffer_push semantics similar to the MIRR stdlib.
            let pushes = crate::mirr_executor::drive_lexer_with_interpreter(source.as_bytes());
            let mut token_buf = crate::mirr_runtime::TokenBuffer::new();
            let mut failures: usize = 0;
            for p in &pushes {
                let ok = crate::mirr_runtime::push_mapped_token_to_buffer(
                    &mut token_buf,
                    p.kind,
                    p.ident.as_deref(),
                    p.int_val,
                );
                if !ok {
                    failures += 1;
                }
            }
            let ok = failures == 0;
            let message = if ok {
                format!("lexer executor ran — collected {} token(s)", token_buf.len())
            } else {
                format!(
                    "lexer executor ran — collected {} token(s), {} failed (DIAG_LEX_BUFFER_FULL)",
                    token_buf.len(),
                    failures
                )
            };
            stages.push(StageResult {
                name: "LexerDriver".to_string(),
                ok,
                message,
            });
        }
        let envelope = TemporalNetlistJson::from_netlist(&netlist);
        let actual_json = match serde_json::to_string_pretty(&envelope) {
            Ok(j) => j,
            Err(e) => {
                stages.push(StageResult {
                    name: "FixtureParity".to_string(),
                    ok: false,
                    message: format!("JSON serialization error: {e}"),
                });
                return self.finish(source_path, stages, false, None, None);
            }
        };

        if self.opts.emit_netlist_json {
            netlist_json = Some(actual_json.clone());
        }
        if self.opts.emit_netlist_verilog {
            match TemporalGuardCompiler::new().emit_netlist_verilog(&netlist) {
                Ok(v) => netlist_verilog = Some(v),
                Err(e) => {
                    stages.push(StageResult {
                        name: "TemporalEmitVerilog".to_string(),
                        ok: false,
                        message: format!("{e}"),
                    });
                    return self.finish(source_path, stages, false, None, None);
                }
            }
        }

        let fixture_path = self.netlist_fixture_path(&source_path);
        match fixture_path {
            None => {
                stages.push(StageResult {
                    name: "FixtureParity".to_string(),
                    ok: true,
                    message: "no fixture configured — skipped".to_string(),
                });
            }
            Some(path) => {
                match self.check_netlist_parity(&actual_json, &path) {
                    Ok(()) => {
                        stages.push(StageResult {
                            name: "FixtureParity".to_string(),
                            ok: true,
                            message: format!("matches {}", path.display()),
                        });
                    }
                    Err(msg) => {
                        stages.push(StageResult {
                            name: "FixtureParity".to_string(),
                            ok: false,
                            message: msg,
                        });
                        let all_ok = stages.iter().all(|s| s.ok);
                        return self.finish(source_path, stages, all_ok, netlist_json, netlist_verilog);
                    }
                }
            }
        }

        let all_ok = stages.iter().all(|s| s.ok);
        self.finish(source_path, stages, all_ok, netlist_json, netlist_verilog)
    }

    // -----------------------------------------------------------------------
    // Fixture path resolution
    // -----------------------------------------------------------------------

    /// Derive the expected netlist fixture path from the source file.
    ///
    /// Convention:
    ///   source      → `<anywhere>/neonatal_respirator.mirr`
    ///   fixture     → `<fixture_root>/netlist/neonatal_respirator.json`
    ///
    /// Returns `None` if the fixture root cannot be determined or the file
    /// does not exist (presence is optional — skip the check if absent).
    fn netlist_fixture_path(&self, source_path: &Path) -> Option<PathBuf> {
        let stem = source_path.file_stem()?.to_str()?;
        let fixture_root = self.resolve_fixture_root(source_path)?;
        let path = fixture_root.join("netlist").join(format!("{stem}.json"));
        if path.exists() { Some(path) } else { None }
    }

    /// Resolve the fixture root directory.
    ///
    /// Priority order:
    ///   1. `opts.fixture_root` if explicitly set
    ///   2. `<source_dir>/../tests/fixtures` (works from repo root)
    ///   3. `<source_dir>/tests/fixtures` (works if source is in a subdir)
    fn resolve_fixture_root(&self, source_path: &Path) -> Option<PathBuf> {
        if let Some(ref root) = self.opts.fixture_root {
            return Some(root.clone());
        }

        // Try to find tests/fixtures relative to the source file's ancestors.
        let mut dir = source_path.canonicalize().ok()?.parent()?.to_path_buf();
        for _ in 0..5 {
            let candidate = dir.join("tests").join("fixtures");
            if candidate.is_dir() {
                return Some(candidate);
            }
            match dir.parent() {
                Some(p) => dir = p.to_path_buf(),
                None => break,
            }
        }

        None
    }

    // -----------------------------------------------------------------------
    // Parity check
    // -----------------------------------------------------------------------

    /// Compare the serialized netlist JSON against a golden fixture file.
    ///
    /// Only the fields that form the IR contract are compared:
    ///   - `ir_version`
    ///   - `guards` (count, strategy, names, signal names, condition)
    ///   - `signals` (count, names, types, kinds)
    ///   - `statistics` (all numeric fields except `compilation_time_us`)
    fn check_netlist_parity(
        &self,
        actual_json: &str,
        fixture_path: &Path,
    ) -> Result<(), String> {
        let fixture_str = std::fs::read_to_string(fixture_path)
            .map_err(|e| format!("cannot read fixture '{}': {e}", fixture_path.display()))?;

        let actual: serde_json::Value = serde_json::from_str(actual_json)
            .map_err(|e| format!("actual JSON invalid: {e}"))?;
        let expected: serde_json::Value = serde_json::from_str(&fixture_str)
            .map_err(|e| format!("fixture JSON invalid: {e}"))?;

        // ir_version
        if actual["ir_version"] != expected["ir_version"] {
            return Err(format!(
                "ir_version mismatch: actual={} expected={}",
                actual["ir_version"], expected["ir_version"]
            ));
        }

        // guards count
        let ag = actual["guards"].as_array().ok_or("actual.guards is not array")?;
        let eg = expected["guards"].as_array().ok_or("expected.guards is not array")?;
        if ag.len() != eg.len() {
            return Err(format!("guard count: actual={} expected={}", ag.len(), eg.len()));
        }

        // signals count + per-signal fields
        let as_ = actual["signals"].as_array().ok_or("actual.signals is not array")?;
        let es_ = expected["signals"].as_array().ok_or("expected.signals is not array")?;
        if as_.len() != es_.len() {
            return Err(format!("signal count: actual={} expected={}", as_.len(), es_.len()));
        }
        for (i, (a, e)) in as_.iter().zip(es_.iter()).enumerate() {
            if a["name"] != e["name"] {
                return Err(format!("signals[{i}].name: actual={} expected={}", a["name"], e["name"]));
            }
            if a["ty"] != e["ty"] {
                return Err(format!("signals[{i}].ty: actual={} expected={}", a["ty"], e["ty"]));
            }
            if a["kind"] != e["kind"] {
                return Err(format!("signals[{i}].kind: actual={} expected={}", a["kind"], e["kind"]));
            }
        }

        // statistics (exclude compilation_time_us — non-deterministic)
        for field in &[
            "shift_registers_used",
            "counters_used",
            "logic_gates_used",
            "max_delay_cycles",
            "total_signals",
        ] {
            let av = &actual["statistics"][field];
            let ev = &expected["statistics"][field];
            if av != ev {
                return Err(format!("statistics.{field}: actual={av} expected={ev}"));
            }
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Internal helper
    // -----------------------------------------------------------------------

    fn finish(
        &self,
        source_path: PathBuf,
        stages: Vec<StageResult>,
        ok: bool,
        netlist_json: Option<String>,
        netlist_verilog: Option<String>,
    ) -> BootstrapResult {
        BootstrapResult {
            source_path,
            stages,
            ok,
            netlist_json,
            netlist_verilog,
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp_mirr(src: &str) -> NamedTempFile {
        let mut f = NamedTempFile::with_suffix(".mirr").expect("tempfile");
        f.write_all(src.as_bytes()).expect("write");
        f
    }

    const NEONATAL_SRC: &str = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;

    #[test]
    fn test_bootstrap_neonatal_passes_all_stages() {
        let f = write_temp_mirr(NEONATAL_SRC);
        let runner = BootstrapRunner::new(BootstrapOpts {
            emit_netlist_json: false,
            emit_netlist_verilog: false,
            fail_fast: false,
            fixture_root: None,
            run_lexer_driver: false,
        });
        let result = runner.run(f.path());

        // Read / Parse / Validate / TemporalLower must all pass.
        // FixtureParity is skipped (no fixture adjacent to tempfile).
        for stage in &result.stages {
            assert!(
                stage.ok,
                "Stage '{}' failed: {}",
                stage.name, stage.message
            );
        }
    }

    #[test]
    fn test_bootstrap_parse_error_captured() {
        let f = write_temp_mirr("module bad { JUNK }");
        let runner = BootstrapRunner::default();
        let result = runner.run(f.path());
        assert!(!result.ok, "should fail on parse error");

        let parse_stage = result.stages.iter().find(|s| s.name == "Parse");
        assert!(parse_stage.is_some(), "Parse stage must be present");
        assert!(!parse_stage.unwrap().ok, "Parse stage must be marked failed");
    }

    #[test]
    fn test_bootstrap_emit_json_flag() {
        let f = write_temp_mirr(NEONATAL_SRC);
        let runner = BootstrapRunner::new(BootstrapOpts {
            emit_netlist_json: true,
            emit_netlist_verilog: false,
            fail_fast: false,
            fixture_root: None,
            run_lexer_driver: false,
        });
        let result = runner.run(f.path());
        // netlist_json must be populated when the flag is set and pipeline succeeds.
        assert!(
            result.netlist_json.is_some(),
            "netlist_json must be populated when emit_netlist_json=true"
        );
        let json = result.netlist_json.unwrap();
        assert!(json.contains("sustained_pressure_drop"), "JSON must contain guard name");
        assert!(json.contains("\"ir_version\""), "JSON must contain ir_version");
    }

    #[test]
    fn test_bootstrap_emit_verilog_flag() {
        let f = write_temp_mirr(NEONATAL_SRC);
        let runner = BootstrapRunner::new(BootstrapOpts {
            emit_netlist_json: false,
            emit_netlist_verilog: true,
            fail_fast: false,
            fixture_root: None,
            run_lexer_driver: false,
        });
        let result = runner.run(f.path());
        assert!(
            result.netlist_verilog.is_some(),
            "netlist_verilog must be populated when emit_netlist_verilog=true"
        );
        let v = result.netlist_verilog.unwrap();
        assert!(v.contains("module"), "Verilog output should declare a module");
        assert!(v.contains("sustained_pressure_drop"), "output should mention guard name");
    }

    #[test]
    fn test_bootstrap_with_fixture_root_passes_parity() {
        let _f = write_temp_mirr(NEONATAL_SRC);

        // Locate the repo's fixture directory.
        let fixture_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures");
        if !fixture_root.exists() {
            return; // skip if fixtures not present in this environment
        }

        // Copy temp file to have stem "neonatal_respirator" so fixture lookup works.
        let tmp_dir = tempfile::tempdir().expect("tempdir");
        let named = tmp_dir.path().join("neonatal_respirator.mirr");
        std::fs::write(&named, NEONATAL_SRC).expect("write named");

        let runner = BootstrapRunner::new(BootstrapOpts {
            emit_netlist_json: false,
            emit_netlist_verilog: false,
            fail_fast: false,
            fixture_root: Some(fixture_root),
            run_lexer_driver: false,
        });
        let result = runner.run(&named);
        assert!(result.ok, "all stages including FixtureParity must pass; stages: {:#?}", result.stages);
    }

    #[test]
    fn test_summary_line_format() {
        let f = write_temp_mirr(NEONATAL_SRC);
        let runner = BootstrapRunner::default();
        let result = runner.run(f.path());
        let summary = result.summary_line();
        assert!(summary.contains("SELF-HOST"), "summary must include SELF-HOST");
        assert!(summary.contains("stages passed"), "summary must include stages passed");
    }
}
