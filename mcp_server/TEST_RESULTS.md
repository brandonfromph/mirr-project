# MCP Integration Test Results (summary)

Date: 2026-03-02

## What I ran
- Generated builder API key: id=test-runner (appended to mcp_server/config.json).
- Invoked POST /run_cargo {subcommand: "test"} via MCP — all cargo tests passed (exitCode 0).
- Invoked POST /read_netlist on tests/fixtures/netlist/neonatal_respirator.json — returned parsed JSON.
- Built binary: `cargo build --bin mirr-simplify` — succeeded.
- Invoked POST /run_simulator against built binary (multiple path variants) — binary executed but panicked for the chosen input.
- Invoked POST /parity_check comparing tests/fixtures/ast/seizure_monitor.json vs artifacts/output.json — files differed (artifacts/output.json empty).

## Findings
- Cargo tests: PASS (all unit/integration tests passed).
- read_netlist: PASS (JSON parsed).
- mirr-simplify execution:
  - Binary built successfully.
  - When run with `tests/fixtures/ast/seizure_monitor.json` the binary panicked:
    - Error: "Invalid Expr JSON: unknown variant `ir_version` ..." — indicates the input file format expected by mirr-simplify (an Expr JSON) differs from the AST JSON provided.
  - Several run_simulator attempts were blocked by executable whitelist (`executable_not_allowed`) until the exact path/name matched allowed entries.
- parity_check: FAIL (artifacts/output.json is empty / not produced by simulator run).

## Next recommended steps (short)
1. Decide how to run simulator under MCP:
   - Add exact built path (e.g., `target\\debug\\mirr-simplify.exe`) to `mcp_server/config.json.allowed_commands.executables`, or
   - Allow running a wrapper script that invokes the binary.
2. Provide the correct input type to `mirr-simplify` (generate an Expr JSON expected by the binary) or adapt the binary invocation to accept AST JSON.
3. Re-run simulator to produce artifacts/output.json, then re-run parity_check.
4. Add AJV request schemas and signed JSONL audit logging (next hardening work).

## Short status checklist
- [x] Server running and /health verified
- [x] API key generation & append
- [x] cargo test via MCP: PASS
- [x] read_netlist: PASS
- [x] Built mirr-simplify binary
- [ ] run_simulator: produce correct output (blocked/mismatch)
- [ ] parity_check: pass (after simulator output)
- [ ] AJV payload validation
- [ ] Signed JSONL audit log