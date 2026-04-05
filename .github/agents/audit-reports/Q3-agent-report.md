# Q3 Agent Report: MCP Startup Behavior for MRT Tools

Date: 2026-04-05
Scope: Determine whether MCP server startup is automatic vs manual for MRT tool calls, and whether a daemon supervisor/autostart script exists.

## Required Commands (Executed Sequentially)

1. `rg -n "stdio|daemon|autostart|spawn|start" mcp_server/src/server.ts`
- Result: matched stdio/direct startup lines and startup call.
- Key hits: `mcp_server/src/server.ts:377`, `mcp_server/src/server.ts:679`, `mcp_server/src/server.ts:682`, `mcp_server/src/server.ts:1325`, `mcp_server/src/server.ts:1326`, `mcp_server/src/server.ts:1327`.

2. `rg -n "mcp_server" scripts/ .vscode/ --type sh --type json`
- Result: no output, exit code 1 (no `mcp_server` launch wiring found in shell/json scripts under those paths).

## Explicit Verdict

Auto-start verdict (daemon/supervisor inside repo): **NO**.

- The repo does not contain a process supervisor or daemon auto-start mechanism for `mcp_server`.
- The server runtime is implemented as **stdio-direct**.
- Startup is either:
  - manual (`npm start` / `npm run dev`), or
  - host/client-managed process spawn (if external MCP client config launches `node mcp_server/start.js`).

So: not self-daemonized, not supervisor-managed, but stdio mode is supported for host-spawned lifecycle.

## Evidence Table

| Question | Evidence | Interpretation |
|---|---|---|
| Is server startup hardwired to stdio mode? | `mcp_server/src/server.ts:1325` comment says startup path always stdio-direct; `mcp_server/src/server.ts:1326-1327` logs and calls `startStdioServer()` | Runtime entrypoint is stdio-direct, not HTTP daemon listener startup |
| Does stdio handler exist and process JSON-RPC over stdin/stdout? | `mcp_server/src/server.ts:679-682` describes stdio-direct helper and starts line-delimited JSON-RPC; `mcp_server/src/server.ts:682` function `startStdioServer()` | Protocol transport is stdio streams |
| Is there a wrapper that enforces stdio-only behavior? | `mcp_server/start.js:3` (stdio-direct mode only), `mcp_server/start.js:45` (server will use stdio exclusively), `mcp_server/start.js:53` (`MCP_STDIO_DIRECT=1`), `mcp_server/start.js:54` (`require(distPath)`) | Wrapper launches same process in stdio-only mode |
| Any daemon/supervisor artifacts? | `mcp_server/start.js:24` says pidfile management no longer needed when never binding a port; `mcp_server/start.js:57` says legacy stdio proxy removed | No in-repo daemon/pidfile supervision path |
| What do package scripts provide? | `mcp_server/package.json:8` start => `node start.js`; `mcp_server/package.json:9` dev => `ts-node src/server.ts`; `mcp_server/package.json:10` test => stdio proxy test | Manual commands exist; no dedicated supervisor command |
| Any `.vscode` task that starts MCP server service? | `.vscode/tasks.json:660` and `.vscode/tasks.json:694` task labels are test-oriented; `.vscode/tasks.json:665` uses `mcp_server` as npm prefix for `test` | VS Code tasks reference MCP tests, not long-running service startup |
| Any `scripts/` launch orchestration? | `scripts/review_coverage_gate.py:9`, `scripts/review_coverage_gate.py:17`, `scripts/review_coverage_gate.py:39` only include `mcp_server` in path/risk sets | No launch supervisor in root scripts |
| Is there an MCP stdio implementation in MRT SDK server too? | `mcp_server/src/mrt.ts:2` imports `StdioServerTransport`; `mcp_server/src/mrt.ts:375-377` connects transport and logs running on stdio | MRT-oriented server code is also stdio-transport based |

## Startup Mode Comparison

| Mode | Present? | How it would start | Evidence |
|---|---|---|---|
| Manual start | YES | User/automation runs `npm --prefix mcp_server start` or `npm --prefix mcp_server run dev` | `mcp_server/package.json:8-9` |
| Process supervisor / daemon manager | NO | No PM2/systemd/nssm/pidfile supervisor flow in repo | `mcp_server/start.js:24`, `mcp_server/start.js:57`, plus no matching launch scripts in required command #2 |
| Stdio-direct (host-spawned MCP process) | YES | MCP host spawns Node process and communicates over stdin/stdout | `mcp_server/src/server.ts:1325-1327`, `mcp_server/start.js:3`, `mcp_server/start.js:53-54`, `mcp_server/src/mrt.ts:375-377` |

## Startup Topology / Sequence

```text
Copilot Agent (tool call)
  -> MCP Host/Client runtime (VS Code/Copilot MCP layer)
     -> spawn process command (if configured), e.g. node mcp_server/start.js
        -> start.js sets MCP_STDIO_DIRECT=1 and loads dist/server.js
           -> server.ts logs "MCP server operating in stdio-direct mode"
              -> startStdioServer() reads stdin JSON-RPC and writes stdout responses
                 -> MRT route handlers execute cargo-backed tools

No in-repo daemon supervisor layer is inserted in this chain.
```

## Concrete Test Plan to Validate Startup Behavior

1. Validate no supervisor wiring in tracked scripts/config
- Run required command #2 exactly.
- Expected: no `mcp_server` auto-launch matches in `scripts/*.sh` or `.vscode/*.json`.

2. Validate stdio-direct startup path
- Start server manually: `npm --prefix mcp_server start`.
- Send one JSON-RPC line on stdin (e.g., `{"jsonrpc":"2.0","id":1,"method":"health","params":{}}`).
- Expected: stdout JSON response; stderr includes stdio-direct startup log.

3. Validate host-spawned behavior (auto by client config, not by daemon)
- Configure MCP client entry to execute `node mcp_server/start.js` in stdio mode.
- Trigger an MRT tool call from client.
- Expected: process appears only when client session/tool call starts; request/response over stdio; no separate daemon bootstrapping script involved.

4. Validate absence of long-lived supervisor semantics
- Close MCP client session.
- Check process lifecycle: server process exits with client session termination (or explicit stop), rather than staying managed by a supervisor.

5. Regression check of MCP startup contract
- Run `npm --prefix mcp_server test`.
- Expected: stdio proxy tests pass, confirming current startup assumptions and transport behavior.

## Final Answer to Campaign Question

- Does MCP server daemon auto-start itself when a Copilot agent calls an MRT tool? **NO** (not by self-daemon/supervisor logic in this repo).
- Must it be manually started? **Manual start is supported and sufficient**.
- Is there process supervisor/autostart script? **No in-repo supervisor**.
- Is there stdio mode handling startup lifecycle? **YES**: stdio-direct is the designed runtime mode; host/client may spawn it automatically if configured.
