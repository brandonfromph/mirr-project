# Changelog

All notable changes to the MIRR compiler are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-03-12

### Added

- **Extended type system:** signed integers (i1–i64), linear signal ownership, refinement types, effect types, clock domains, phantom tags.
- **S-expression IR:** homoiconic intermediate representation with reader macros, evaluator, and pretty-printer (`--emit sexpr`).
- **R-SPU ISA v2:** tagged architecture, binary encoding, cycle-accurate simulator, exception handling (E700–E714).
- **Width inference overhaul:** SCC solver, formal verification with Rocq proofs (FIRWINE).
- **Living Research Artifact (LRA):** interactive paper with in-browser compiler, Service Worker offline support, JSON-RPC 2.0 protocol.
- **LRA-1.0 standard:** specification at `template/spec/LRA-1.0.md` with Bronze/Silver/Gold compliance tiers.
- **`lra-cli` crate:** `init`, `validate`, `serve`, `badge`, `build` subcommands for LRA project management.
- **Markdown pipeline:** `lra build` compiles Markdown with YAML frontmatter into LRA-compliant HTML.
- **4 new emit targets:** testbench, fpga_scaffold, build_script, dsp (total: 10 targets).
- **MAPE-K feedback loop:** Monitor–Analyze–Plan–Execute–Knowledge autonomic computing module.
- **Multi-error reporting:** error accumulation with bounded diagnostic buffer.
- **DSP inference:** multiply operations mapped to FPGA DSP slices.
- **Synthesis-ready SystemVerilog:** Yosys-compatible output with `(* keep *)` attributes.
- **rustc-quality diagnostics:** source spans, caret display, multi-line context, error code references.
- **LRA template repository:** fork-ready template for creating new Living Research Artifacts.
- **Cross-paper query:** `lra-client.js` library for iframe-based JSON-RPC 2.0 communication between papers.
- Error codes E5xx (width), E6xx (type), E7xx (R-SPU), E8xx (S-expression).
- Documentation: S-expression guide, MAPE-K guide, FPGA targets guide.

### Changed

- Type checker enabled by default (`PipelineConfig::typecheck` now `true`).
- Pipeline supports extended type checking via `PipelineConfig::extended_typecheck`.
- Expression parser uses bounded recursion with `MAX_EXPR_DEPTH` (128).
- S-expression parser uses bounded recursion with `MAX_SEXPR_DEPTH` (64).
- Crate version bumped from `0.2.0` to `0.3.0`.

### Removed

- `MirrError::LexicalError` dead code variant cleanup completed.
- 12 unused error code variants removed from `typeck::extended`.
- `print_sexpr_compact` function (replaced by S-expression pretty-printer).

## [0.2.0] - 2026-03-08

### Added

- Property directive keywords: `cover` and `assume` in property blocks (default remains `assert`).
- Property formula variants: `NeverImplies`, `EventuallyWithin`, `AlwaysFollowedBy`.
- FIRRTL emit target (`--emit firrtl`).
- SVA standalone emit target (`--emit sva`).
- Pattern system: `def`/`reflect` blocks with `${param}` substitution for reusable guard/reflex/property templates.
- Error code prefixes `[E1xx]`-`[E4xx]` on all compiler error messages.
- `schema_version` field in JSON netlist output (value: `"0.2.0"`).
- Beginner tutorial (`docs/tutorial.md`) — 10-lesson guide for absolute beginners.
- Migration guide (`docs/migration-guide.md`) — upgrade notes for API and JSON consumers.
- VS Code syntax highlighting extension (`vscode-mirr/`).

### Changed

- Error messages now prefixed with structured error codes (e.g. `[E100] Parse error: ...`).
- JSON netlist always includes a `properties` array (previously absent when empty).
- `PropertyJson` now includes a `directive` field (`"assert"`, `"cover"`, or `"assume"`).
- `PropertyJson` `kind` field has three new values: `never_implies`, `eventually_within`, `always_followed_by`.
- SVA output uses directive-dependent keyword (`cover property`/`assume property` instead of always `assert property`).
- DOT property nodes use directive-dependent fill colors (blue=assert, yellow=cover, green=assume).
- Crate version bumped from `0.1.0` to `0.2.0`.

### Removed

- `MirrError::LexicalError` variant (was dead code).
- `MirrError::TemporalCausalityViolation` variant (was dead code).

## [0.1.0] - 2026-02-01

### Added

- Core MIRR language: `signal`, `guard`, `reflex` constructs.
- Verilog RTL emit target (`--emit verilog`).
- DOT graph emit target (`--emit dot`).
- JSON AST emit target (`--emit json`).
- Temporal guard compiler with counter-based lowering.
- Width inference with SCC-based propagation.
- Logic simplification pass.
- Pipeline architecture: parse, validate, simplify, width, temporal.
- Golden fixture parity tests for neonatal respirator and seizure monitor.
- Self-hosting IR contract (`ir_version: "1.0"`).
- 11 example `.mirr` programs.
- NASA Power-of-10 compliance: `#![forbid(unsafe_code)]`, bounded algorithms, no recursion.
