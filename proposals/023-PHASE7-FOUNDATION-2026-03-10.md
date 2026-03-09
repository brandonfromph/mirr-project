# PHASE7-FOUNDATION — Documentation Overhaul, Living Document, and OSS-CAD-Suite Toolchain

**Proposal #:** 023
**Campaign IDs:** DOCS-003, LIVINGDOC-001, TOOLCHAIN-001
**Date:** 2026-03-10
**Status:** SIGNED — EXECUTED
**Scope:** Architecture (10+ edited files, 25+ new files)
**Depends on:** SYNTH-001 (021, executed), PAPER-001 (022, executed)
**DAC Submission Tag:** `dac2027-submission` at `b20c3c0` (frozen, never modified)

---

## Philosophy Gate

- **Generative power of three** -- Toolchain modules do not add new behavioral constructs. Signal/Guard/Reflex remain the only three.
- **NASA Power-of-10** -- Every new `src/toolchain/*.rs` file carries `#![forbid(unsafe_code)]`. All iterations bounded by explicit constants.
- **Hardware-synthesizable** -- Toolchain modules read `PipelineResult` but never modify it. No new AST nodes.
- **Properties don't affect hardware** -- Formal verification reads SVA bind files; it does not alter RTL emission.
- **Zero-Debt Invariant** -- No new debt introduced. Existing ad-hoc path normalization in `synth_yosys_tests.rs` is centralized (debt reduction).

---

## Campaign Overview

This is the first Phase 7 proposal. It establishes the infrastructure for all future campaigns via three coordinated sub-campaigns:

| Sub-Campaign | ID | Scope | Purpose |
|---|---|---|---|
| A | DOCS-003 | 7 edits, 3 new files | Fix factual errors in roadmap, update stale docs, create glossary + contributing guide + canonical INDEX |
| B | LIVINGDOC-001 | 1 edit, 16 new files | Create modular IEEE living documentation paper; mandate living-doc updates in SKILL.md |
| C | TOOLCHAIN-001 | 5 edits, 9 new files | Build `src/toolchain/` subsystem: sby formal verification, Verilator lint/sim, nextpnr PnR, icetime, EQY |

**Combined totals:** 13 edited files, 28 new files, ~25 new tests, ~2,500 new lines.

---

## Pre-Flight Verification

```
Tag:    dac2027-submission at b20c3c0 (frozen)
Branch: main
Tests:  961 pass, 0 fail
Clippy: 0 warnings
Yosys:  11/11 examples synthesize clean
```

---

# Campaign A: DOCS-003 -- Full Documentation Overhaul + Roadmap

## A — Motivation

The roadmap banner at `docs/roadmap.md:82` claims "Phases 0 through 8 are complete," but Phase 5 is "semi-Completed," Phase 7 (generic) is "Not Started," and Phase 8 is "Not Started." Test counts say 954 everywhere; actual count is 961. The language spec (`mirr_spec.md`) describes Phase 1 only -- no signed types, no `property`, no `def`/`reflect`. No canonical `docs/INDEX.md` exists. No glossary. No contributing guide. `type-system.md` says E601-E607 but actual range is E601-E609.

## A — Current State Assessment

| Document | Status | Issue |
|---|---|---|
| `docs/roadmap.md` | Active | Banner wrong; test count 954 stale; Phase 7 ordering confusing |
| `docs/mirr_spec.md` | Frozen | Phase 1 era; no iN types, property, def/reflect, prev() |
| `docs/type-system.md` | Active | Says E601-E607, actual E601-E609 |
| `docs/logic_simplification.md` | Frozen | Says 58 tests; pipeline diagram missing property/pattern passes |
| `docs/benchmarks.md` | Active | Pipeline description incomplete |
| `docs/home.md` | Active | Missing glossary/contributing links; version may be stale |
| `docs/_config.yml` | Active | Excludes INDEX.md from Jekyll build |
| `docs/INDEX.md` | Missing | Does not exist at docs root |
| `docs/glossary.md` | Missing | Does not exist |
| `docs/contributing.md` | Missing | Does not exist |

## A — File-by-File Changes

### A1. `docs/roadmap.md`

| Location | Current | Fix |
|---|---|---|
| Line 82 (banner) | "Phases 0 through 8 are complete" + "954 tests" | "Phases 0-4, 6, 7a, 7b complete. Phase 5 partial." + "961 tests" |
| Line 208 (Phase 5) | "semi-Completed" | "Partial" with note: Phase 5a complete, 5b not started |
| Line 258 (Phase 6 tests) | "954 tests" | "961 tests" |
| Lines 302-315 (Phase 7) | Listed AFTER 7a/7b, confusing | Rename to "Phase 7c+ — Advanced" and move after 7b |
| Lines 413-503 (repo tree) | Missing newer modules | Add `src/toolchain/`, `src/diagnostic.rs`, `src/suggest.rs`; update test count |
| Lines 533-538 (See Also) | Missing links | Add Glossary and Contributing |

### A2. `docs/mirr_spec.md`

Add deprecation banner at top:
```markdown
> **Deprecated.** This spec describes the Phase 1 minimal core only.
> For the current language, see [Tutorial](tutorial) and [Type System](type-system).
```

### A3. `docs/type-system.md`

| Line | Current | Fix |
|---|---|---|
| 11 | "E601-E607" | "E601-E609" |
| 10 | "TYPE-001, TYPE-002, TYPE-003" | "TYPE-001 through TYPE-005" |

### A4. `docs/logic_simplification.md`

| Line | Current | Fix |
|---|---|---|
| 94 | "58 tests" | Remove hard-coded count, say "tests in `tests/simplify_tests.rs`" |
| 13-17 | Pipeline diagram | Add property and pattern passes |

### A5. `docs/benchmarks.md`

| Line | Current | Fix |
|---|---|---|
| 24 | Pipeline description missing typecheck/patterns | Add "typecheck + pattern expansion" to pipeline description |

### A6. `docs/home.md`

- Add Glossary and Contributing to documentation table
- Verify version string (update to v0.3.0 if stale)

### A7. `docs/_config.yml`

- Remove `INDEX.md` from exclude list

### A8. `docs/INDEX.md` -- New File

Full documentation index with status legend. Covers all 22 proposals, all active/frozen/deprecated docs, schemas. Replaces `_archive/INDEX.md` as canonical.

### A9. `docs/glossary.md` -- New File

~30 terms: Cement2, SmaRTLy, FIRWINE, MAPE-K, DPR, LTL, SCC, SVA, R-SPU, Guard, Reflex, Signal, Pattern, Property, AHL, AIG, NBTI, DO-254, FIRRTL, etc.

### A10. `docs/contributing.md` -- New File

Coding standards (forbid unsafe, NASA P10), campaign workflow, error code allocation table (E1xx-E7xx), testing requirements, documentation requirements.

## A — Execution Order

| Wave | Files |
|---|---|
| 1 | `docs/roadmap.md` (highest priority — factual errors) |
| 2 | `docs/mirr_spec.md`, `docs/type-system.md`, `docs/logic_simplification.md`, `docs/benchmarks.md` |
| 3 | `docs/glossary.md`, `docs/contributing.md` (new files) |
| 4 | `docs/INDEX.md` (depends on Waves 1-3 for final file list) |
| 5 | `docs/_config.yml`, `docs/home.md` (depends on Wave 4) |

## A — Verification

```bash
grep -n "Phases 0 through 8" docs/roadmap.md          # Expected: no output
grep -rn "954" docs/ --include="*.md"                   # Expected: no active refs
ls docs/INDEX.md docs/glossary.md docs/contributing.md  # Expected: all exist
head -12 docs/type-system.md | grep "E60"               # Expected: E601-E609
grep "INDEX.md" docs/_config.yml                        # Expected: no output
```

---

# Campaign B: LIVINGDOC-001 -- Modular IEEE Living Documentation Paper + SKILL.md Mandate

## B — Motivation

The frozen DAC paper at `paper/dac2027-mirr.tex` was compressed to 10 IEEE pages. The type system gets half a page. The R-SPU gets one page. Case studies are limited to two examples. The MAPE-K simulator gets one sentence. Every future campaign (FPGA targets, LSP, new Rocq theorems) produces results that have no home.

A living document solves this: no page limit, DAC quality, single source of truth, modular structure, parameterized metrics, and a SKILL.md mandate ensuring it stays current.

**Why B3 (modular + mandate) over B1 (single file) or B2 (modular, no mandate):**
- B1 fails at scale -- a single `.tex` exceeds 2000 lines within 3 campaigns
- B2 has no enforcement -- without SKILL.md mandate, the living doc rots under time pressure
- B3 makes it a hard gate, same class as the Zero-Debt Invariant

## B — Directory Structure

```
paper/
  dac2027-mirr.tex                   # FROZEN. Never edited again.
  living-doc/
    main.tex                          # Master document with \input per chapter
    metrics.tex                       # All metrics as \newcommand macros
    campaign-log.tex                  # Append-only campaign history
    ch-introduction.tex               # Section I
    ch-language.tex                   # Section II (grammar, types, patterns, properties)
    ch-pipeline.tex                   # Section III (9 stages, error arch, temporal algo)
    ch-width.tex                      # Section IV (constraints, SCC, Rocq proofs)
    ch-typesys.tex                    # Section V (9 rules, inference figure)
    ch-rspu.tex                       # Section VI (ISA, registers, tick model)
    ch-eval.tex                       # Section VII (tests, benchmarks, synthesis)
    ch-related.tex                    # Section VIII
    ch-vision.tex                     # Section IX (claims mapping)
    ch-casestudies.tex                # Section X (neonatal, flight controller)
    ch-toolchain.tex                  # Section XI (stub for future campaigns)
    ch-conclusion.tex                 # Section XII
    ch-ai-disclosure.tex              # AI disclosure (IEEE policy)
```

## B — Key Files

### B1. `paper/living-doc/metrics.tex` — Parameterized Metrics

All project numbers as `\newcommand` macros. When a campaign changes a metric, it updates ONE line here and every chapter reference updates automatically.

```latex
\newcommand{\totalSourceLines}{18,157}
\newcommand{\totalTests}{961}
\newcommand{\testToSourceRatio}{0.97:1}
\newcommand{\rocqTheorems}{27}
\newcommand{\rocqMechanized}{14}
\newcommand{\rocqAxiomatized}{13}
\newcommand{\rocqProofLines}{816}
\newcommand{\rspuInstructions}{20}
\newcommand{\totalSourceFiles}{73}
\newcommand{\forbidUnsafeDirectives}{73}
\newcommand{\distinctErrorCodes}{147}
\newcommand{\typeRules}{9}
\newcommand{\pipelineStages}{9}
\newcommand{\emissionBackends}{7}
\newcommand{\examplePrograms}{14}
\newcommand{\synthesizableExamples}{11}
\newcommand{\propertyForms}{6}
% ... (~48 total \newcommand definitions)
```

### B2. `paper/living-doc/main.tex` — Master Document

Standard `article` class (not `IEEEtran`). `\input` per chapter. Bibliography. Table of contents. `\input{metrics}` in preamble. Uses `mathpartir`, `algorithmic`, `hyperref`, `longtable`.

### B3. `paper/living-doc/campaign-log.tex` — Append-Only History

```latex
\section{Campaign Log}\label{sec:campaign-log}
\begin{longtable}{lllp{5cm}p{4cm}}
\toprule
\# & Campaign ID & Date & Summary & Chapters Touched \\
\midrule
1 & LIVINGDOC-001 & 2026-03-10 & Initial creation from frozen paper & All \\
\bottomrule
\end{longtable}
```

### B4. Chapter files (`ch-*.tex`)

Each file contains the corresponding section from the frozen paper, with all hard-coded numbers replaced by `\newcommand` references. Initial content is identical to the frozen paper — but now free to expand.

**Key transformation rules:**
- Replace `954` with `\totalTests{}`
- Replace `18,157` with `\totalSourceLines{}`
- Replace `27 theorems` with `\rocqTheorems{} theorems`
- Remove `IEEEtran`-specific formatting
- Preserve all `\label{}` tags
- Each file starts with `\section{}`, no preamble

## B — SKILL.md Changes

### B5. New Quality Checklist item (after line 356)

```markdown
- [ ] **Living document updated** -- `paper/living-doc/` chapters reflect any
      new features, metrics, or architectural changes; `metrics.tex` values
      updated if any counts changed; row added to `campaign-log.tex`
```

### B6. New Phase 4.5 — Living Document Gate (after line 608)

```markdown
### Phase 4.5 -- Living Document Gate

After the Zero-Debt Gate, update the living document at `paper/living-doc/`:

1. **Metrics update:** Update `metrics.tex` if any counts changed.
2. **Chapter update:** Update relevant `ch-*.tex` files. Create new `ch-<slug>.tex`
   for wholly new subsystems.
3. **Campaign log:** Append row to `campaign-log.tex`.
4. **Compile check:** `cd paper/living-doc && pdflatex main.tex` (deferred if pdflatex unavailable).

**Mandatory for Campaign and Architecture scope.** Patch-scope may skip if no user-visible changes,
but must still update `metrics.tex` if counts changed.
```

### B7. Updated Phase 5 Close Out (line 611)

Add: `4. Confirm living document gate passed (or state Patch-scope exemption)`

## B — Execution Order

| Wave | Files |
|---|---|
| 1 | Create `paper/living-doc/` directory |
| 2 | `metrics.tex`, `main.tex`, `campaign-log.tex`, SKILL.md changes (all independent) |
| 3 | All 13 `ch-*.tex` files (independent, can parallelize) |
| 4 | Verification |

## B — Verification

```bash
ls paper/living-doc/ch-*.tex | wc -l                     # Expected: 13
ls paper/living-doc/*.tex | wc -l                          # Expected: 16
grep -q 'Living document updated' .github/skills/propose-campaign/SKILL.md  # Expected: match
grep -q 'Living Document Gate' .github/skills/propose-campaign/SKILL.md     # Expected: match
grep -rn '18,157\|954\|0\.97:1' paper/living-doc/ch-*.tex  # Expected: 0 matches (all parameterized)
git diff paper/dac2027-mirr.tex                             # Expected: no output (frozen)
```

---

# Campaign C: TOOLCHAIN-001 -- Full OSS-CAD-Suite Foundation

## C — Motivation

The MIRR compiler emits SystemVerilog RTL and SVA assertions, but verification stops at the source file boundary. Engineers must manually configure sby, Verilator, nextpnr, icetime, and EQY. Each tool has its own config format, path conventions, and Windows quirks.

Campaign C closes this gap by integrating the full oss-cad-suite into `src/toolchain/`:

| Capability | CLI Flag | Tools Used |
|---|---|---|
| Formal verification | `--formal` | sby + Z3/Yices/Bitwuzla/Boolector |
| RTL linting | `--lint` | Verilator `--lint-only` |
| Cycle simulation | `--simulate` | Verilator compiled sim |
| Place and route | `--pnr` | nextpnr-ice40/ecp5/nexus |
| Static timing | `--timing` | icetime (iCE40 only) |
| Equivalence check | `--eqy` | EQY |

## C — Current State Assessment

| Area | Status | Evidence |
|---|---|---|
| Yosys synthesis | Complete | `tests/synth_yosys_tests.rs` — 6 tests |
| SVA emission | Complete | `emit_sv_synthesis()` + `emit_sva_bind_file()` |
| iCE40 scaffold | Complete | `fpga_scaffold.rs:202-228` |
| ECP5/Nexus scaffold | Not started | No `FpgaTarget` variants |
| sby formal | Not started | No `.sby` generator, no CLI flag |
| Verilator lint/sim | Not started | No invocation code |
| nextpnr invocation | Not started | Scripts emitted but not executed |
| icetime | Not started | No parser |
| EQY | Not started | No `.eqy` generator |
| Tool env detection | Partial | `yosys_available()` in test code only |
| Windows path normalization | Ad-hoc | `replace('\\', "/")` inline |

## C — New Source Files

### C1. `src/toolchain/mod.rs` — Tool Registry & Environment Detection

```rust
pub mod sby;
pub mod verilator;
pub mod icetime;
pub mod eqy;

pub const MAX_TOOLS: usize = 32;
pub const MAX_VERSION_LEN: usize = 128;

pub enum Tool { Yosys, Sby, Verilator, IcarusVerilog, NextpnrIce40,
                NextpnrEcp5, NextpnrNexus, Icepack, Icetime, Eqy, ... }

pub struct ToolRegistry { pub tools: HashMap<Tool, ToolInfo> }

pub fn normalize_path_for_mingw(path: &Path) -> String { ... }
pub fn invoke_tool(registry, tool, args, working_dir) -> Result<Output, ToolchainError> { ... }
```

Centralizes: OSS_CAD_SUITE env var detection, PATH lookup, version extraction, DLL path setup, forward-slash normalization.

### C2. `src/toolchain/sby.rs` — SymbiYosys Formal Verification

```rust
pub const MAX_BMC_DEPTH: u32 = 200;
pub const DEFAULT_BMC_DEPTH: u32 = 20;
pub const MAX_ENGINES: usize = 4;

pub struct SbyConfig { bmc_depth, prove, engines, work_dir }
pub struct SbyResult { passed, property_results, elapsed_secs }

pub fn generate_sby_config(result, config, sv, bind) -> String { ... }
pub fn run_formal(result, config, registry, work_dir) -> Result<SbyResult> { ... }
```

Generated `.sby` uses the existing SVA bind file infrastructure — `emit_sva_bind_file()` output is already sby-compatible.

### C3. `src/toolchain/verilator.rs` — Lint & Compiled Simulation

```rust
pub const MAX_VERILATOR_WARNINGS: usize = 100;
pub const SUPPRESSED_WARNINGS: &[&str] = &["-Wno-UNUSEDSIGNAL", "-Wno-UNDRIVEN"];

pub fn run_lint(result, registry, work_dir) -> Result<LintResult> { ... }
pub fn run_simulation(result, registry, work_dir) -> Result<SimResult> { ... }
```

### C4. `src/toolchain/icetime.rs` — Static Timing Analysis

```rust
pub struct TimingResult { fmax_mhz, critical_path_ns, logic_levels }

pub fn run_timing(asc_path, device, registry) -> Result<TimingResult> { ... }
```

### C5. `src/toolchain/eqy.rs` — Equivalence Checking

```rust
pub struct EqyResult { equivalent, divergent_signal }

pub fn generate_eqy_config(module, gold_sv, gate_sv) -> String { ... }
pub fn run_equivalence(gold, gate, registry, work_dir) -> Result<EqyResult> { ... }
```

## C — Changes to Existing Files

### C6. `src/emit/fpga_target.rs` — Add LatticeEcp5, LatticeNexus

```rust
pub enum FpgaTarget {
    Generic, Xilinx7, XilinxUS, IntelCyclone, LatticeIce40,
    LatticeEcp5,   // NEW
    LatticeNexus,  // NEW
}
```

All 7 match methods extended. New methods: `icetime_device()`, `nextpnr_binary()`.

| Method | LatticeEcp5 | LatticeNexus |
|---|---|---|
| `constraint_extension()` | `"lpf"` | `"pdc"` |
| `build_tool()` | `"nextpnr-ecp5"` | `"nextpnr-nexus"` |
| `default_part()` | `"LFE5U-85F-6BG381C"` | `"LIFCL-40-9BG400C"` |

### C7. `src/emit/fpga_scaffold.rs` — ECP5 & Nexus Scaffolds

New functions: `emit_lpf()`, `emit_pdc()`, `emit_ecp5_sh()`, `emit_nexus_sh()`.

ECP5 build script: `yosys -p "synth_ecp5 ..." -> nextpnr-ecp5 -> ecppack`
Nexus build script: `yosys -p "synth_nexus ..." -> nextpnr-nexus -> prjoxide pack`

### C8. `src/bin/mirr-compile.rs` — New CLI Flags

| Flag | Type | Default | Description |
|---|---|---|---|
| `--formal` | bool | false | Run sby formal verification |
| `--formal-depth N` | u32 | 20 | BMC depth |
| `--formal-prove` | bool | false | Also run k-induction |
| `--formal-engine E` | String | z3 | Solver: z3/yices/bitwuzla/btor |
| `--lint` | bool | false | Verilator `--lint-only` |
| `--simulate` | bool | false | Verilator compiled simulation |
| `--pnr` | bool | false | Place and route (Lattice targets) |
| `--timing` | bool | false | icetime (iCE40 only) |
| `--eqy` | bool | false | Equivalence check |
| `--toolchain-path DIR` | Option | None | Override OSS-CAD-Suite root |

### C9. `src/lib.rs` — Register Module

Add `pub mod toolchain;`

### C10. `tests/synth_yosys_tests.rs` — Refactor

Migrate `yosys_available()` to use `ToolRegistry`. Replace inline `replace('\\', "/")` with `normalize_path_for_mingw()`.

## C — New Test Files

| File | Tests | Description |
|---|---|---|
| `tests/toolchain_sby_tests.rs` | 8 | Config generation, engine selection, end-to-end formal |
| `tests/toolchain_verilator_tests.rs` | 5 | Lint, simulation, warning handling |
| `tests/toolchain_nextpnr_tests.rs` | 8 | PnR end-to-end, icetime, ECP5/Nexus scaffolds |
| `tests/toolchain_eqy_tests.rs` | 4 | Config generation, equivalence verification |

**Total new tests: ~25.** All use conditional-skip pattern for graceful degradation when tools aren't installed.

## C — Execution Order

| Step | Files | Depends on | Build gate |
|---|---|---|---|
| 1 | `src/emit/fpga_target.rs` | -- | `cargo build` (exhaustive match) |
| 2 | `src/emit/fpga_scaffold.rs` | Step 1 | `cargo build` + existing scaffold tests |
| 3 | `src/toolchain/mod.rs`, `src/lib.rs` | -- | `cargo build` |
| 4 | `src/toolchain/sby.rs` | Step 3 | `cargo build` |
| 5 | `src/toolchain/verilator.rs` | Step 3 | `cargo build` |
| 6 | `src/toolchain/icetime.rs` | Step 3 | `cargo build` |
| 7 | `src/toolchain/eqy.rs` | Step 3 | `cargo build` |
| 8 | `src/bin/mirr-compile.rs` | Steps 3-7 | `cargo build` + `--help` verification |
| 9 | `tests/synth_yosys_tests.rs` refactor | Step 3 | `cargo test --test synth_yosys_tests` |
| 10 | `tests/toolchain_sby_tests.rs` | Steps 3-4 | `cargo test --test toolchain_sby_tests` |
| 11 | `tests/toolchain_verilator_tests.rs` | Steps 3, 5 | `cargo test --test toolchain_verilator_tests` |
| 12 | `tests/toolchain_nextpnr_tests.rs` | Steps 1-3, 6 | `cargo test --test toolchain_nextpnr_tests` |
| 13 | `tests/toolchain_eqy_tests.rs` | Steps 3, 7 | `cargo test --test toolchain_eqy_tests` |

---

# Cross-Campaign Risk Table

| # | Risk | Severity | Campaign | Mitigation |
|---|---|---|---|---|
| R1 | Windows MinGW path issues | High | C | `normalize_path_for_mingw()` centralized; unit tested |
| R2 | DLL dependency conflicts | Medium | C | `invoke_tool()` prepends oss-cad-suite `bin/` + `lib/` to child PATH |
| R3 | Tool version drift | Medium | C | `ToolRegistry` records versions; min-version documented, not hard-enforced |
| R4 | Exhaustive match breakage (FpgaTarget) | High | C | Desired -- Rust catches missing arms. Steps 1-2 done together. |
| R5 | Roadmap test count drifts again | Low | A | After C lands, count will be ~986. A uses current count at time of execution. |
| R6 | Living doc metrics stale on creation | Low | B | `metrics.tex` initialized with values from frozen paper; C updates when tests increase. |
| R7 | SKILL.md mandate adds overhead to patches | Low | B | Mandate explicitly exempts Patch-scope with no user-visible changes |
| R8 | mirr_spec.md deprecation breaks references | Low | A | Banner approach preserves file; only adds warning |
| R9 | Chapter files drift from frozen paper during migration | Medium | B | Verification step greps for hard-coded numbers that should be parameterized |
| R10 | sby working directory conflicts | Medium | C | Unique working dirs with timestamp suffix |
| R11 | icetime only supports iCE40 | Low | C | `icetime_device()` returns None for non-iCE40; CLI prints clear message |
| R12 | NASA P10 compliance for new modules | High | C | Every file starts with `#![forbid(unsafe_code)]`; all bounds documented |

---

# Cross-Campaign Constraint Table

| # | Constraint | Source | Enforced by |
|---|---|---|---|
| C1 | DAC paper never modified | Submission policy | `git diff paper/dac2027-mirr.tex` = empty |
| C2 | No unsafe code | NASA P10 | `#![forbid(unsafe_code)]` in every new file |
| C3 | Zero warnings | CI | `cargo clippy -- -D warnings` |
| C4 | Surface language unchanged | MIRR philosophy | No new AST nodes, parser rules, or keywords |
| C5 | Properties don't affect hardware | MIRR philosophy | Toolchain reads PipelineResult, never modifies |
| C6 | All metrics parameterized | Living doc design | Grep verification in Wave B4 |
| C7 | Campaign log append-only | Audit trail | Review during Phase 4.5 |
| C8 | Backward compatibility | Existing 961 tests | `cargo test --all` green before and after |
| C9 | Graceful tool degradation | Usability | Missing tools = clear message, not panic |

---

# Full Execution Wave Plan

| Wave | Campaign | Steps | Parallelizable? |
|---|---|---|---|
| W1 | A | roadmap.md fixes (highest priority factual errors) | -- |
| W2 | A | mirr_spec.md, type-system.md, logic_simplification.md, benchmarks.md | All independent |
| W3 | A | glossary.md, contributing.md (new files) | Both independent |
| W4 | A | INDEX.md (depends on W1-W3 for final file list) | Sequential |
| W5 | A | _config.yml, home.md | Sequential after W4 |
| W6 | B | Create `paper/living-doc/` dir + metrics.tex + main.tex + campaign-log.tex | All independent |
| W7 | B | All 13 ch-*.tex chapter files | All independent (13-way parallel) |
| W8 | B | SKILL.md changes (Quality Checklist + Phase 4.5 + Close Out) | Independent |
| W9 | C | fpga_target.rs + fpga_scaffold.rs (enum extension + scaffolds) | Together (exhaustive match) |
| W10 | C | src/toolchain/mod.rs + src/lib.rs (registry foundation) | Independent of W9 |
| W11 | C | sby.rs, verilator.rs, icetime.rs, eqy.rs (all independent) | 4-way parallel, depends on W10 |
| W12 | C | mirr-compile.rs CLI flags | Depends on W10-W11 |
| W13 | C | All 4 test files + synth_yosys_tests refactor | Depends on W10-W12 |
| W14 | ALL | Full verification: cargo test, cargo clippy, verification commands | Sequential after all |

---

# Full File Manifest

## Campaign A — Edited (7 files)

| File | Change |
|---|---|
| `docs/roadmap.md` | Fix banner, test counts, Phase 5 status, Phase 7 ordering, repo tree |
| `docs/mirr_spec.md` | Add deprecation banner |
| `docs/type-system.md` | E601-E609 range, campaign list update |
| `docs/logic_simplification.md` | Fix test count, update pipeline diagram |
| `docs/benchmarks.md` | Fix pipeline description |
| `docs/home.md` | Add glossary/contributing links, verify version |
| `docs/_config.yml` | Remove INDEX.md exclusion |

## Campaign A — New (3 files)

| File | Description |
|---|---|
| `docs/INDEX.md` | Canonical documentation index |
| `docs/glossary.md` | ~30 terms covering project terminology |
| `docs/contributing.md` | Coding standards, campaign workflow, error allocation |

## Campaign B — Edited (1 file)

| File | Change |
|---|---|
| `.github/skills/propose-campaign/SKILL.md` | Quality Checklist item + Phase 4.5 Gate + Close Out update |

## Campaign B — New (16 files)

| File | Description |
|---|---|
| `paper/living-doc/main.tex` | Master document |
| `paper/living-doc/metrics.tex` | 48 `\newcommand` metric definitions |
| `paper/living-doc/campaign-log.tex` | Append-only campaign history |
| `paper/living-doc/ch-introduction.tex` | Section I |
| `paper/living-doc/ch-language.tex` | Section II |
| `paper/living-doc/ch-pipeline.tex` | Section III |
| `paper/living-doc/ch-width.tex` | Section IV |
| `paper/living-doc/ch-typesys.tex` | Section V |
| `paper/living-doc/ch-rspu.tex` | Section VI |
| `paper/living-doc/ch-eval.tex` | Section VII |
| `paper/living-doc/ch-related.tex` | Section VIII |
| `paper/living-doc/ch-vision.tex` | Section IX |
| `paper/living-doc/ch-casestudies.tex` | Section X |
| `paper/living-doc/ch-toolchain.tex` | Section XI (stub) |
| `paper/living-doc/ch-conclusion.tex` | Section XII |
| `paper/living-doc/ch-ai-disclosure.tex` | AI disclosure |

## Campaign C — Edited (5 files)

| File | Change |
|---|---|
| `src/emit/fpga_target.rs` | Add LatticeEcp5, LatticeNexus + new methods |
| `src/emit/fpga_scaffold.rs` | ECP5/Nexus constraint + build script generators |
| `src/bin/mirr-compile.rs` | 10 new CLI flags + help text + target validation |
| `src/lib.rs` | `pub mod toolchain;` |
| `tests/synth_yosys_tests.rs` | Refactor to use ToolRegistry |

## Campaign C — New (9 files)

| File | Description |
|---|---|
| `src/toolchain/mod.rs` | Tool registry, env detection, path normalization |
| `src/toolchain/sby.rs` | sby config generator + formal verification runner |
| `src/toolchain/verilator.rs` | Lint + compiled simulation |
| `src/toolchain/icetime.rs` | Static timing analysis parser |
| `src/toolchain/eqy.rs` | Equivalence checking config + runner |
| `tests/toolchain_sby_tests.rs` | 8 tests |
| `tests/toolchain_verilator_tests.rs` | 5 tests |
| `tests/toolchain_nextpnr_tests.rs` | 8 tests |
| `tests/toolchain_eqy_tests.rs` | 4 tests |

---

# Final Verification

```bash
# All tests pass (including ~25 new ones)
cargo test --all 2>&1 | tail -3

# Zero clippy warnings
cargo clippy --all-targets -- -D warnings

# No unsafe code in new files
grep -rn "unsafe" src/toolchain/ --include="*.rs" | grep -v "forbid"

# Frozen paper untouched
git diff paper/dac2027-mirr.tex

# New CLI flags visible
cargo run --bin mirr-compile -- --help 2>&1 | grep -E "formal|lint|simulate|pnr|timing|eqy"

# Living doc structure complete
ls paper/living-doc/*.tex | wc -l  # Expected: 16

# SKILL.md mandate present
grep -q 'Living Document Gate' .github/skills/propose-campaign/SKILL.md

# End-to-end formal verification (if oss-cad-suite in PATH)
cargo run --bin mirr-compile -- examples/tmr_sensor_fusion.mirr --emit verilog --formal
```

---

# Quality Checklist

- [x] Every source change pinned to specific file and location
- [x] Philosophy gate passed (no new constructs, properties verify-only, P10 compliance)
- [x] Zero-Debt Invariant maintained (ad-hoc path normalization centralized = debt reduction)
- [x] Risk table with entries for all three campaigns
- [x] Constraint table with enforcement mechanisms
- [x] Backward compatibility: all 961 existing tests unaffected
- [x] File manifest: 13 edited + 28 new = 41 files total
- [x] Wave plan: 14 waves with dependency chain
- [x] Verification commands are copy-pasteable
- [ ] **Living document updated** -- This is the campaign that creates it
- [x] Breakage map: FpgaTarget enum extension causes desired compile errors (caught in W9)
