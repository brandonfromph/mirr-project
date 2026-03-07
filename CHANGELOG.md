# Changelog

All notable changes to the MIRR compiler are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
