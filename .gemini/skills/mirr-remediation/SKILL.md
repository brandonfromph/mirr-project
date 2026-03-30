---
name: mirr-remediation
description: Specialist workflows for repairing "Dark Age" (March 17-22, 2026) implementations in the NASA MIRR codebase. Use when audit finds disconnected modules, missing pipeline wiring, or hallucinated stubs.
---

# MIRR Remediation Skill

This skill provides expert procedural guidance for recovering from the "Dark Age" period (March 17-22, 2026), where features were partially implemented but left disconnected or faked.

## Core Workflows

### 1. The "Last Mile" Pipeline Wiring
Use this when a module exists in `src/` but is never called by `src/pipeline.rs` or `src/main.rs`.

1. **Audit Stage Outputs**: Identify the return type of the orphan module.
2. **Locate Insertion Point**: Find the appropriate Stage (1.x, 2.x, etc.) in `src/pipeline.rs`.
3. **Trace Data Flow**: Ensure the module receives necessary state (e.g., `MirrProgram`, `TypeMap`, `TemporalNetlist`).
4. **Patch `PipelineResult`**: Add a field to the result struct if the stage produces an artifact that subsequent stages need.

### 2. Stub Audit & Unmasking
Use this to differentiate between high-quality disconnected logic and hallucinated stubs.

| Indicator | Quality | Action |
|---|---|---|
| 10KB+ File Size | HIGH | Integrate via Pipeline Wiring. |
| Complex Regex/Logic | HIGH | Keep and repair. |
| `// TODO` in every fn | LOW | Delete and rewrite from proposal. |
| Hardcoded Strings | LOW | Delete and replace with Diagnostic API. |
| Placeholder Structs | MEDIUM | Stub out implementation if proposal exists. |

### 3. Symbol Integration
When fixing cross-file imports (MEGA-11), follow this symbol resolution sequence:

1. **Local Resolution**: Check the current module's `ModuleSymbols`.
2. **Alias Resolution**: Parse `alias.symbol` and look up in `ImportContext`.
3. **Conflict Detection**: Use `SymbolTable::check_symbol_conflicts()` to prevent E909/E910 errors.

## Error Code Mapping

Refer to `docs/error_codes.md` for the following ranges:
- **E13xx**: Import/Multi-file errors.
- **E9xx**: Symbol/Namespace resolution errors.
- **E17x-E19x**: MEGA-1 Extended Type errors.

## Verification Protocol

1. **Clean `cargo check`**: Absolute requirement.
2. **Symbolic Link Check**: Verify `src/import` and `src/symbols` are actually being utilized by setting a breakpoint or using `println!` in a debug run.
3. **Regression Gate**: Run `tests/semantic_validation_tests.rs` after any pipeline wiring change.
