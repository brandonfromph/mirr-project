# MIRR Agent Instructions

Use [docs/repo-topology.md](docs/repo-topology.md) as the canonical subsystem and maturity map. Treat [docs/roadmap.md](docs/roadmap.md) as planning context, not current state.

## Scope

MIRR is an integrated safety-critical platform:
- Core compiler: src/
- Consumers: crates/mirr-wasm, crates/mirr-arsenal-wasm, crates/lra-cli, crates/mirr-mcp-control-plane, vscode-mirr, demos, paper
- Assurance: tests/, proofs/, fuzz/, scripts/

## Non-Negotiables

- Preserve #![forbid(unsafe_code)] and warning-free builds.
- Keep implementations bounded and deterministic.
- Zero-debt policy: no dead code, placeholders, or duplicate wrappers.
- Do not remove working logic to force gates green.

## Architecture Boundaries

Compiler pipeline:
- Parse: src/parser/
- Validate and expand: src/validation/, src/expand/
- Type and width: src/typeck/, src/width/
- Temporal: src/temporal/
- Emit: src/emit/
- S-expression IR: src/sexpr/

When touching multiple surfaces, check impact on:
1. src/ and tests/
2. crates/mirr-wasm and crates/mirr-arsenal-wasm
3. crates/mirr-mcp-control-plane, mirr-general, scripts
4. crates/lra-cli
5. vscode-mirr
6. proofs and fuzz

## Build and Test

Default gate sequence:
- cargo fmt --all -- --check
- cargo check --all-targets
- cargo clippy --all-targets -- -D warnings
- cargo nextest run --workspace --no-fail-fast

Test strategy:
- Prefer cargo nextest for workspace and package validation.
- Use cargo test only for narrow, targeted debugging when nextest is not practical.
- Treat the local MIRR MCP bridge / stdio host as an E2E surface for control-plane changes.
- Include local MCP smoke tests in the verification plan for route, schema, and resolver work.

Full wave orchestration:
- cargo run --bin mirr-general -- ci --format json

Campaign workflow:
- Follow .github/skills/propose-campaign/SKILL.md

## Workspace Gotchas

- In PowerShell, prefer cargo.exe if alias/wrapper behavior is inconsistent.
- For nested orchestration on Windows, set CARGO_TARGET_DIR=target/ci-wave.
- Verify claims against source and tests, not stale status artifacts.
- The Rust MCP bridge is the mirror stdio host binary (`cargo run -p mirror --bin mirr-mcp-stdio-host`); verify `.vscode/mcp.json` points at it before relying on agent tool access.

## References

- [README.md](README.md)
- [CLAUDE.md](CLAUDE.md)
- [docs/repo-topology.md](docs/repo-topology.md)
- [docs/testing-guide.md](docs/testing-guide.md)
- [docs/error_codes.md](docs/error_codes.md)
- [docs/type-system.md](docs/type-system.md)
- [docs/contributing.md](docs/contributing.md)
- [crates/mirr-mcp-control-plane/Cargo.toml](crates/mirr-mcp-control-plane/Cargo.toml)
- [crates/mirr-mcp-control-plane/src/bin/mirr-mcp-stdio-host.rs](crates/mirr-mcp-control-plane/src/bin/mirr-mcp-stdio-host.rs)
- [vscode-mirr/README.md](vscode-mirr/README.md)
- [docs/web-rules.md](docs/web-rules.md)
