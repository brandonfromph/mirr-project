# Q7 Agent Report - MRT Arsenal CLI Polish Audit
Date: 2026-04-05
Agent: Q7-agent
Workspace: C:\Users\elvie\mirrc

## Scope
- src/bin/mirr-audit.rs
- src/bin/mirr-brain.rs
- src/bin/mirr-wave.rs
- src/bin/mirr-general.rs
- src/bin/mirr-lsp.rs

## 1) Overall polish verdict
VERDICT: NO.
MRT is not yet polished across all five CLI binaries for the requested criteria.
Only mirr-brain is consistently strong across json format, help and version support, bounded output, and NASA P10 guardrails.

Major blockers:
1. Error code strategy is inconsistent or missing across binaries.
2. Version flag is missing in mirr-general and mirr-lsp.
3. Explicit output bounds are missing or partial in mirr-audit, mirr-wave, and mirr-lsp.
4. NASA P10 safety headers and bounded-loop discipline are inconsistent.

## Runtime command evidence
| Command | Observed result |
|---|---|
| cargo.exe run --bin mirr-audit -- --help | Help text printed with format and version flags |
| cargo.exe run --bin mirr-audit -- --version | Printed mirr-audit 0.3.0 |
| cargo.exe run --bin mirr-audit -- --glob src/bin/mirr-lsp.rs --format json | Printed JSON array output |
| cargo.exe run --bin mirr-brain -- --help | Help text printed with subcommands and global format flag |
| cargo.exe run --bin mirr-brain -- --version | Printed mirr-brain 0.3.0 |
| cargo.exe run --bin mirr-brain -- --format json get --key __q7_missing_key__ | Printed JSON object with status ERROR and message Unknown key |
| cargo.exe run --bin mirr-wave -- --help | Help text printed with version flag and max-lines option |
| cargo.exe run --bin mirr-wave -- --version | Printed mirr-wave 0.3.0 |
| cargo.exe run --bin mirr-wave -- --format json | Failed with unexpected argument --format |
| cargo.exe run --bin mirr-general -- --help | Printed custom quick usage text |
| cargo.exe run --bin mirr-general -- --version | Failed with unrecognized subcommand --version |
| cargo.exe run --bin mirr-general -- inspect --format json | Printed JSON inspect payload |
| cargo.exe run --bin mirr-lsp -- --version | Process entered server mode and timed out after 10 seconds; no version response |

## 2) Evidence table with exact paths and lines
| Topic | Evidence |
|---|---|
| mirr-audit help and version wiring | src/bin/mirr-audit.rs:35 and src/bin/mirr-audit.rs:36 |
| mirr-audit json format flag and branch | src/bin/mirr-audit.rs:42, src/bin/mirr-audit.rs:240, src/bin/mirr-audit.rs:262 |
| mirr-audit error code style is mixed | src/bin/mirr-audit.rs:133 (E801), src/bin/mirr-audit.rs:166 (SEC-01), src/bin/mirr-audit.rs:183 (D1-D7) |
| mirr-audit unbounded scans and output paths | src/bin/mirr-audit.rs:97, src/bin/mirr-audit.rs:114, src/bin/mirr-audit.rs:192, src/bin/mirr-audit.rs:201 |
| mirr-audit missing forbid unsafe crate header | src/bin/mirr-audit.rs:26 is first code item after comments; rg match for #![forbid(unsafe_code)] returned none |
| mirr-brain help and version wiring | src/bin/mirr-brain.rs:41 and src/bin/mirr-brain.rs:42 |
| mirr-brain json format support | src/bin/mirr-brain.rs:51 and src/bin/mirr-brain.rs:197 |
| mirr-brain bounded output controls | src/bin/mirr-brain.rs:35, src/bin/mirr-brain.rs:36, src/bin/mirr-brain.rs:91, src/bin/mirr-brain.rs:178 |
| mirr-brain strict safety headers | src/bin/mirr-brain.rs:10 and src/bin/mirr-brain.rs:11 |
| mirr-brain errors are text without stable E-codes | src/bin/mirr-brain.rs:166 and src/bin/mirr-brain.rs:201 |
| mirr-wave help and version wiring | src/bin/mirr-wave.rs:20 and src/bin/mirr-wave.rs:21 |
| mirr-wave has no format selector flag | src/bin/mirr-wave.rs:23-40 args list has no format field; runtime probe rejected --format json |
| mirr-wave json output exists but is always printed from log stash path | src/bin/mirr-wave.rs:270 and src/bin/mirr-wave.rs:277 |
| mirr-wave boundedness is partial only | src/bin/mirr-wave.rs:37 and src/bin/mirr-wave.rs:180 bound edit chunk size, but src/bin/mirr-wave.rs:102 and src/bin/mirr-wave.rs:141 iterate unbounded by explicit MAX constants |
| mirr-wave missing forbid unsafe crate header | src/bin/mirr-wave.rs:13 is first code item after comments; rg match for #![forbid(unsafe_code)] returned none |
| mirr-general json format support | src/bin/mirr-general.rs:450, src/bin/mirr-general.rs:454, src/bin/mirr-general.rs:457 |
| mirr-general help text support | src/bin/mirr-general.rs:520, src/bin/mirr-general.rs:522, src/bin/mirr-general.rs:554 |
| mirr-general no CLI version route | src/bin/mirr-general.rs:615 and runtime probe returned unrecognized subcommand --version |
| mirr-general boundedness controls | src/bin/mirr-general.rs:527, src/bin/mirr-general.rs:645, src/bin/mirr-general.rs:796, src/bin/mirr-general.rs:833, src/bin/mirr-general.rs:862 |
| mirr-general safety header and warning policy gap | src/bin/mirr-general.rs:1 has forbid unsafe; rg match for #![deny(warnings)] returned none |
| mirr-lsp has no parser for help version or format | src/bin/mirr-lsp.rs:15, src/bin/mirr-lsp.rs:16, src/bin/mirr-lsp.rs:17, src/bin/mirr-lsp.rs:21, src/bin/mirr-lsp.rs:22 |
| mirr-lsp safety header but no explicit bounds in wrapper | src/bin/mirr-lsp.rs:11 and rg match for MAX_ returned none |

## 3) Polish matrix per binary
| Binary | --format json output | Consistent error codes | Bounded output | --help text | --version flag | NASA P10 compliance |
|---|---|---|---|---|---|---|
| mirr-audit | YES | NO | NO | YES | YES | NO |
| mirr-brain | YES | NO | YES | YES | YES | YES |
| mirr-wave | NO | NO | PARTIAL | YES | YES | NO |
| mirr-general | YES | NO | YES | YES | NO | PARTIAL |
| mirr-lsp | NO | NO | NO | NO | NO | PARTIAL |

### Binary notes
- mirr-audit: Clap surface is good, but no crate-level forbid unsafe and no explicit MAX bounds for scan volume or output volume.
- mirr-brain: Most polished binary in this set; clear bounds and strict safety headers.
- mirr-wave: Uses clap help and version, but lacks selectable format contract and has only partial boundedness.
- mirr-general: Strong bounded orchestration and json support, but no user-facing version flag and no deny warnings header.
- mirr-lsp: Runtime server shim only; no CLI contract for help version or output format.

## 4) CLI surface diagram
~~~mermaid
flowchart TD
    U[Operator or Orchestrator]
    A[mirr-audit Clap CLI]
    B[mirr-brain Clap CLI]
    W[mirr-wave Clap CLI]
    G[mirr-general Manual router]
    L[mirr-lsp LSP server shim]

    U --> A
    U --> B
    U --> W
    U --> G
    U --> L

    A -->|optional stash_key| B
    W -->|stash logs and receipts| B
    G -->|cargo and CI orchestration| C[Toolchain and workspace tasks]
    L -->|stdin stdout JSON RPC| S[mirrc::lsp::server::run]

    A -. format json supported .- U
    B -. format json supported .- U
    G -. format json supported on inspect and ci .- U
    W -. no format selector flag .- U
    L -. no help version format parser .- U
~~~

## 5) Implementation-first hardening and polish sketch
1. Introduce a shared MRT CLI contract module for output envelope and error code taxonomy.
- Add a new shared type with fields status, error_code, message, limits, and payload.
- Use one stable code namespace for all CLI binaries, for example M1xx for audit, M2xx for brain, M3xx for wave, M4xx for general, M5xx for lsp wrapper errors.

2. Standardize help and version behavior in all binaries.
- Keep clap in mirr-audit, mirr-brain, and mirr-wave.
- Migrate mirr-general to clap derive parser while preserving existing subcommand behavior.
- Add clap parser to mirr-lsp wrapper with no-op server mode default and explicit --version and --help support.

3. Normalize format selection semantics.
- Enforce a shared output format enum text and json in every binary.
- Add format selector to mirr-wave so json is explicit and text mode can be human-readable.
- Keep mirr-lsp transport payloads machine readable; wrapper flags should still follow the shared contract.

4. Add explicit bounded output constants where missing.
- mirr-audit: MAX_AUDIT_FILES, MAX_AUDIT_FINDINGS, MAX_AUDIT_OUTPUT_BYTES.
- mirr-wave: MAX_EDITS_PER_WAVE, MAX_LOG_ERRORS, MAX_LOG_FILES_APPLIED, MAX_LOG_BYTES.
- mirr-lsp wrapper: MAX_REQUEST_BYTES and MAX_RESPONSE_BYTES for wrapper-level diagnostics and logging.

5. Enforce NASA P10 crate headers consistently.
- Add #![forbid(unsafe_code)] where missing.
- Add #![deny(warnings)] to all audited binaries for consistent safety gate behavior.

6. Add polish regression tests at CLI boundary.
- Per binary tests for --help, --version, --format json happy path and invalid format failure path.
- Add deterministic bound-hit tests to verify clipped output and explicit error code emission.

7. Rollout sequence to minimize risk.
- Step A: Introduce shared output and error code types.
- Step B: Migrate mirr-general and mirr-lsp to consistent parser surface.
- Step C: Add missing bounds in mirr-audit and mirr-wave.
- Step D: Add full CLI contract tests and then run full CI gate.

READY FOR ORCHESTRATOR
