# Q11 Agent Report - MRT Arsenal Dependency and Usage Audit

Date: 2026-04-05
Agent: Q11-agent

Scope files:
- Cargo.toml
- Cargo.lock
- mcp_server/package.json
- mcp_server/src/server.ts
- mcp_server/src/mrt.ts
- src/bin/mirr-brain.rs
- src/bin/mirr-wave.rs
- src/bin/mirr-audit.rs

## 1) Explicit YES/NO Verdicts
- Complete visibility across the scoped dependency manifests and lock data: YES.
- Active usage health for the audited MRT surface: NO.
- Reason for NO: at least one declared dependency has no scoped usage evidence or is latent in the default runtime path.

## 2) Evidence Table (Exact Paths + Lines)
| Topic | Evidence |
|---|---|
| Rust dependency declarations | Cargo.toml:14-30; Cargo.toml:90 |
| Rust lock resolution for audited crate deps | Cargo.lock:98; Cargo.lock:177; Cargo.lock:224; Cargo.lock:310; Cargo.lock:464; Cargo.lock:585; Cargo.lock:612; Cargo.lock:930; Cargo.lock:1075; Cargo.lock:1229; Cargo.lock:1288; Cargo.lock:1331; Cargo.lock:1423; Cargo.lock:1453; Cargo.lock:1466; Cargo.lock:1554 |
| Root crate dependency linkage in lockfile | Cargo.lock:1025-1041 |
| mirr-brain runtime deps in use | src/bin/mirr-brain.rs:28; src/bin/mirr-brain.rs:30; src/bin/mirr-brain.rs:31; src/bin/mirr-brain.rs:102; src/bin/mirr-brain.rs:116; src/bin/mirr-brain.rs:197 |
| mirr-wave runtime deps in use | src/bin/mirr-wave.rs:13; src/bin/mirr-wave.rs:14; src/bin/mirr-wave.rs:15; src/bin/mirr-wave.rs:60; src/bin/mirr-wave.rs:65; src/bin/mirr-wave.rs:270; src/bin/mirr-wave.rs:281 |
| mirr-audit runtime deps in use | src/bin/mirr-audit.rs:26; src/bin/mirr-audit.rs:28; src/bin/mirr-audit.rs:29; src/bin/mirr-audit.rs:30; src/bin/mirr-audit.rs:63; src/bin/mirr-audit.rs:68; src/bin/mirr-audit.rs:97; src/bin/mirr-audit.rs:191; src/bin/mirr-audit.rs:239 |
| MCP package declarations | mcp_server/package.json:13-27 |
| MCP active server dependency use | mcp_server/src/server.ts:1; mcp_server/src/server.ts:2; mcp_server/src/server.ts:7; mcp_server/src/server.ts:20; mcp_server/src/server.ts:21; mcp_server/src/server.ts:316; mcp_server/src/server.ts:317; mcp_server/src/server.ts:321; mcp_server/src/server.ts:322; mcp_server/src/server.ts:1174; mcp_server/src/server.ts:1180 |
| MCP SDK alternate path dependency use | mcp_server/src/mrt.ts:1; mcp_server/src/mrt.ts:2; mcp_server/src/mrt.ts:6; mcp_server/src/mrt.ts:32; mcp_server/src/mrt.ts:182; mcp_server/src/mrt.ts:375 |

## 3) Full Dependency Table

### 3.1 Rust Dependencies (Cargo.toml + Cargo.lock)
| Dependency | Version (declared -> locked) | MRT-specific usage | Active or dead-weight status | Maturity | Evidence |
|---|---|---|---|---|---|
| anyhow | 1.0.102 -> 1.0.102 | Error propagation in all scoped MRT binaries via Result returns | Active | Mature-core | Cargo.toml:15; Cargo.lock:98; src/bin/mirr-brain.rs:102; src/bin/mirr-wave.rs:60; src/bin/mirr-audit.rs:63 |
| clap | 4.4.0 -> 4.5.60 | CLI argument parsing and subcommand surfaces in all scoped MRT binaries | Active | Mature-core | Cargo.toml:16; Cargo.lock:224; src/bin/mirr-brain.rs:28; src/bin/mirr-wave.rs:13; src/bin/mirr-audit.rs:26 |
| ed25519-dalek | 2.1.1 -> 2.2.0 | No usage found in scoped MRT binaries | Dead weight in scoped MRT surface | Mature-crypto | Cargo.toml:17; Cargo.lock:464; src/bin/mirr-brain.rs:1-206; src/bin/mirr-wave.rs:1-286; src/bin/mirr-audit.rs:1-272 |
| glob | 0.3.3 -> 0.3.3 | Recursive pattern scan in refinement, proposal, and workspace audit modes | Active | Mature-support | Cargo.toml:18; Cargo.lock:612; src/bin/mirr-audit.rs:28; src/bin/mirr-audit.rs:97; src/bin/mirr-audit.rs:113; src/bin/mirr-audit.rs:191 |
| rand | 0.8 -> 0.8.5 | No usage found in scoped MRT binaries | Dead weight in scoped MRT surface | Mature-support | Cargo.toml:19; Cargo.lock:1229; src/bin/mirr-brain.rs:1-206; src/bin/mirr-wave.rs:1-286; src/bin/mirr-audit.rs:1-272 |
| regex | 1.12.3 -> 1.12.3 | Policy scanner regexes for D2, D3, D5, D7, and security red lines | Active | Mature-core | Cargo.toml:20; Cargo.lock:1288; src/bin/mirr-audit.rs:29; src/bin/mirr-audit.rs:68-77 |
| rusqlite (bundled) | 0.32 -> 0.32.1 | SQLite-backed kb_entries store in mirr-brain with schema creation and query paths | Active | Mature-core | Cargo.toml:21; Cargo.lock:1331; Cargo.lock:930; src/bin/mirr-brain.rs:30; src/bin/mirr-brain.rs:102-110; src/bin/mirr-brain.rs:116; src/bin/mirr-brain.rs:165 |
| serde | 1.0 -> 1.0.228 | Serialize and deserialize for MRT response and log contracts | Active | Mature-core | Cargo.toml:22; Cargo.lock:1423; src/bin/mirr-brain.rs:31; src/bin/mirr-wave.rs:14; src/bin/mirr-audit.rs:30 |
| serde_json | 1.0 -> 1.0.149 | JSON serialization for output payloads and log stashing | Active | Mature-core | Cargo.toml:23; Cargo.lock:1453; src/bin/mirr-brain.rs:197; src/bin/mirr-wave.rs:270; src/bin/mirr-audit.rs:239 |
| sha2 | 0.10 -> 0.10.9 | SHA-256 snapshot integrity hashing in wave execution | Active | Mature-crypto | Cargo.toml:24; Cargo.lock:1466; src/bin/mirr-wave.rs:15; src/bin/mirr-wave.rs:281-284 |
| chrono | 0.4 -> 0.4.44 | RFC3339 wave timestamp generation | Active | Mature-support | Cargo.toml:25; Cargo.lock:177; src/bin/mirr-wave.rs:65 |
| num_cpus | 1.16 -> 1.17.0 | No usage found in scoped MRT binaries | Dead weight in scoped MRT surface | Mature-support | Cargo.toml:26; Cargo.lock:1075; src/bin/mirr-brain.rs:1-206; src/bin/mirr-wave.rs:1-286; src/bin/mirr-audit.rs:1-272 |
| criterion (dev) | 0.5 -> 0.5.1 | Benchmark-only dependency, not runtime in scoped MRT binaries | Tooling-only in this audit scope | Mature-tooling | Cargo.toml:29; Cargo.lock:310 |
| tempfile (dev) | 3 -> 3.26.0 | Test-only dependency, not runtime in scoped MRT binaries | Tooling-only in this audit scope | Mature-tooling | Cargo.toml:30; Cargo.lock:1554 |
| getrandom (wasm32 target) | 0.2 -> 0.2.17 | wasm32 entropy support; no usage in scoped native MRT binaries | Conditional platform dependency, inactive in native scope | Conditional-platform | Cargo.toml:90; Cargo.lock:585; Cargo.lock:598 |

### 3.2 Node and TypeScript Dependencies (mcp_server/package.json)
| Dependency | Version (declared) | MRT-specific usage | Active or dead-weight status | Maturity | Evidence |
|---|---|---|---|---|---|
| @modelcontextprotocol/sdk | ^1.29.0 | MCP SDK server and stdio transport implementation in src/mrt.ts | Latent secondary runtime path (not the default server.ts path) | Mature-ecosystem | mcp_server/package.json:14; mcp_server/src/mrt.ts:1; mcp_server/src/mrt.ts:2; mcp_server/src/mrt.ts:32; mcp_server/src/mrt.ts:375 |
| ajv | ^8.12.0 | Request schema compilation and validation middleware in server.ts | Active | Mature-core | mcp_server/package.json:15; mcp_server/src/server.ts:321; mcp_server/src/server.ts:322; mcp_server/src/server.ts:352 |
| ajv-formats | ^2.1.1 | No import or registration in scoped MCP sources | Dead weight in scoped MCP surface | Mature-add-on | mcp_server/package.json:16; mcp_server/src/server.ts:1-1420; mcp_server/src/mrt.ts:1-381 |
| body-parser | ^1.20.1 | JSON request body middleware for Express handlers | Active | Mature-support | mcp_server/package.json:17; mcp_server/src/server.ts:2; mcp_server/src/server.ts:317 |
| express | ^4.18.2 | Route surface, request and response types, and middleware execution chain | Active | Mature-core | mcp_server/package.json:18; mcp_server/src/server.ts:1; mcp_server/src/server.ts:24; mcp_server/src/server.ts:316 |
| glob | ^8.1.0 | File pattern search support in search_files route via promisified glob | Active | Mature-support | mcp_server/package.json:19; mcp_server/src/server.ts:7; mcp_server/src/server.ts:21; mcp_server/src/server.ts:1180 |
| @types/express (dev) | ^4.17.17 | Compile-time typing for express.Request and express.Response usage | Active build-time dependency | Mature-tooling | mcp_server/package.json:22; mcp_server/src/server.ts:24; mcp_server/src/server.ts:213; mcp_server/src/server.ts:438 |
| @types/glob (dev) | ^8.0.0 | Compile-time typing for glob import and promisify call signatures | Active build-time dependency | Mature-tooling | mcp_server/package.json:23; mcp_server/src/server.ts:7; mcp_server/src/server.ts:21 |
| @types/node (dev) | ^20.4.2 | Compile-time typing for fs, path, child_process, process, and Buffer APIs | Active build-time dependency | Mature-tooling | mcp_server/package.json:24; mcp_server/src/server.ts:3-6; mcp_server/src/mrt.ts:7-8; mcp_server/src/mrt.ts:381 |
| ts-node (dev) | ^10.9.1 | Development startup path for TypeScript server | Active dev-path dependency | Mature-tooling | mcp_server/package.json:9; mcp_server/package.json:25 |
| typescript (dev) | ^5.1.6 | Transpilation from TypeScript source to dist output | Active build-path dependency | Mature-tooling | mcp_server/package.json:7; mcp_server/package.json:26 |

## 4) Dependency Topology Diagram (Mermaid)
```mermaid
graph TD
  subgraph R["Rust MRT surface from Cargo.toml"]
    CT[Cargo.toml]
    MB[src/bin/mirr-brain.rs]
    MW[src/bin/mirr-wave.rs]
    MA[src/bin/mirr-audit.rs]
    CT -->|anyhow clap rusqlite serde serde_json| MB
    CT -->|anyhow clap chrono serde serde_json sha2| MW
    CT -->|anyhow clap glob regex serde serde_json| MA
    CT -.scoped dead weight.-> RD[ed25519-dalek rand num_cpus criterion tempfile getrandom native]
  end

  subgraph N["MCP server surface from mcp_server/package.json"]
    PJ[mcp_server/package.json]
    ST[mcp_server/src/server.ts]
    MT[mcp_server/src/mrt.ts]
    PJ -->|express body-parser ajv glob| ST
    PJ -->|@modelcontextprotocol/sdk| MT
    PJ -.unused in scoped files.-> AJVF[ajv-formats]
    PJ -->|dev toolchain| DEV[@types/express @types/glob @types/node ts-node typescript]
  end

  ST -->|cargo run --bin mirr-*| MB
  ST -->|cargo run --bin mirr-*| MW
  ST -->|cargo run --bin mirr-*| MA
  MT -->|alternate SDK stdio path| MB
  MT -->|alternate SDK stdio path| MW
  MT -->|alternate SDK stdio path| MA
```

## 5) Concrete Implementation-First Cleanup Plan
1. Remove or wire ajv-formats immediately.
   - Preferred: remove line 16 from mcp_server/package.json because scoped code has no addFormats registration in server.ts.
   - Alternate: import ajv-formats and register it with the existing Ajv instance in mcp_server/src/server.ts right after line 322.
2. Choose one canonical MCP runtime path and enforce it in code and scripts.
   - If server.ts is canonical: keep server.ts and remove or archive mrt.ts and @modelcontextprotocol/sdk from runtime dependencies.
   - If mrt.ts is canonical: point entry/build/test scripts to mrt.ts path and retire duplicate stdio-direct contract in server.ts.
3. Reduce scoped Rust dead-weight by moving non-scoped dependencies to owners.
   - Move ed25519-dalek and rand to the crate or binary that actually uses them, not root Cargo.toml for MRT bins.
   - Move num_cpus to the scheduler owner crate if MRT scheduler is the only owner.
4. Keep runtime and non-runtime dependency boundaries explicit.
   - Retain criterion and tempfile only for benches and tests; do not rely on them in runtime bins.
   - Keep getrandom under wasm32 target dependency only, with no native path reliance.
5. Add dependency health gates for this audit scope.
   - Add a CI check that fails on unused direct dependencies in mcp_server/package.json and root Cargo.toml for scoped MRT binaries.
   - Add a short evidence command block in future audit reports to keep declared to active mapping reproducible.

## 6) Final Answer
- Complete visibility and full dependency inventory delivered: YES.
- Active usage health for the audited MRT surface: NO.

READY FOR ORCHESTRATOR: .github/agents/audit-reports/Q11-agent-report.md
