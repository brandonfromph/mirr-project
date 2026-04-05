# Q1 Agent Report - MCP Server and Copilot Connection Audit
Date: 2026-04-05
Scope: Determine whether repo-local MCP server is connected to Copilot, what is required to connect and test it, and which workflow markdown files should be updated.

## Required Files Read

| File | Status | Evidence |
|---|---|---|
| .vscode/mcp.json | READ | .vscode/mcp.json:2-5 |
| .github/copilot-instructions.md | READ | .github/copilot-instructions.md:9, .github/copilot-instructions.md:33, .github/copilot-instructions.md:55-61 |
| AGENTS.md | NOT FOUND IN WORKSPACE | `bat --style=plain --paging=never -n AGENTS.md` returned os error 2; `rg --files -g '*AGENTS.md'` returned no matches (exit code 1) |
| mcp_server/package.json | READ | mcp_server/package.json:5, mcp_server/package.json:8-10, mcp_server/package.json:14 |
| mcp_server/src/server.ts | READ | mcp_server/src/server.ts:26-29, mcp_server/src/server.ts:377-379, mcp_server/src/server.ts:462-465, mcp_server/src/server.ts:679-682, mcp_server/src/server.ts:753-754, mcp_server/src/server.ts:787-790, mcp_server/src/server.ts:1325-1327 |

## Required Commands Executed Sequentially

1. rg -n "mcp|copilot|stdio|transport" .vscode/ .github/ --type json --type md
- First attempt failed due PowerShell pipe parsing because pattern was not single-quoted.
- Rerun (successful): `rg -n 'mcp|copilot|stdio|transport' .vscode/ .github/ --type json --type md`
- Relevant hit for this question: .vscode/mcp.json:5 (`https://api.githubcopilot.com/mcp/`).

2. rg -n "startStdioServer|stdio-direct" mcp_server/src/server.ts
- Successful output:
  - mcp_server/src/server.ts:682 -> function startStdioServer()
  - mcp_server/src/server.ts:861 -> stdio-direct debug helper comment
  - mcp_server/src/server.ts:1326 -> "MCP server operating in stdio-direct mode"
  - mcp_server/src/server.ts:1327 -> startStdioServer();

## Explicit Verdicts (YES/NO)

- Is Copilot configured to use an MCP endpoint in this workspace? YES.
  - Evidence: .vscode/mcp.json:2-5 defines server `github` with URL `https://api.githubcopilot.com/mcp/`.

- Is the repo-local MCP server (mcp_server/src/server.ts) connected to Copilot right now? NO.
  - Evidence: .vscode/mcp.json:2-5 only defines `github` remote endpoint and does not define a local stdio command entry for this repo's mcp_server.
  - Evidence: mcp_server/package.json:8-10 provides local start/dev/test scripts, but these are not referenced by .vscode/mcp.json.

- Is the local MCP server transport itself suitable for Copilot MCP client wiring? YES (stdio-direct).
  - Evidence: mcp_server/src/server.ts:679-682 and mcp_server/src/server.ts:1325-1327.

## Connection Evidence Table

| Claim | Verdict | Evidence | Notes |
|---|---|---|---|
| VS Code MCP config points to GitHub Copilot MCP | YES | .vscode/mcp.json:3-5 | Active MCP endpoint is GitHub-hosted |
| Local server is stdio-direct only | YES | mcp_server/src/server.ts:679-682, mcp_server/src/server.ts:1325-1327 | No daemon startup path in active entrypoint |
| Local server exposes MCP tool discovery shape | YES | mcp_server/src/server.ts:462-465, mcp_server/src/server.ts:753-754 | ListTools is mapped to mcp_schema |
| Local server can accept API key in stdio envelope | YES | mcp_server/src/server.ts:787-790 and mcp_server/src/server.ts:26-29 | `msg.apiKey` is mapped to headers |
| Local server is currently registered in workspace MCP config | NO | .vscode/mcp.json:2-5 | Only `github` server is present |
| Local server has direct Copilot endpoint wiring in code | NO | `rg -n 'githubcopilot|copilot\.com|github' mcp_server/src/server.ts` returned no matches | server.ts has no Copilot URL coupling |

## Maturity Comparison (Connection Readiness)

| Capability | Current State | Maturity | Evidence |
|---|---|---|---|
| Copilot MCP endpoint configured | GitHub hosted endpoint configured | Strong | .vscode/mcp.json:2-5 |
| Repo-local MCP registration in VS Code config | Missing | Not connected | .vscode/mcp.json:2-5 |
| Local transport implementation | stdio-direct implemented | Strong | mcp_server/src/server.ts:679-682, mcp_server/src/server.ts:1325-1327 |
| Local discovery contract for tool list | mcp_schema route + ListTools mapping | Moderate | mcp_server/src/server.ts:462-465, mcp_server/src/server.ts:753-754 |
| Connection/testing workflow docs | Fragmented and partly stale | Weak | .github/copilot-instructions.md:55-61; MIRR_ARSENAL_README.md:32-38; mcp_server/README.md:40-66 vs mcp_server/src/server.ts:1325-1327 |

## Topology/Pipeline Diagram

```text
Current workspace topology

VS Code / Copilot client
  |
  | reads
  v
.vscode/mcp.json (servers.github -> https://api.githubcopilot.com/mcp/)
  |
  +--> GitHub Copilot MCP endpoint (connected)

Repo-local MCP path (implemented but not wired in mcp.json)

Copilot/Client (if configured)
  |
  | spawn stdio process
  v
node mcp_server/start.js -> dist/server.js
  |
  v
startStdioServer() [mcp_server/src/server.ts:682,1327]
  |
  +--> ListTools -> mcp_schema [mcp_server/src/server.ts:753-754,462]
  +--> CallTool -> route handlers (mrt_* and other tools)
```

## What Is Needed To Connect Local MCP Server To Copilot

1. Add a local server entry to workspace MCP config.
- Target file: .vscode/mcp.json (current content only has `github`: lines 2-5).
- Add a second server entry (stdio) pointing to local server startup command.

2. Ensure local server build artifacts exist before connection.
- Evidence: mcp_server/package.json:5 (`main: dist/server.js`), mcp_server/package.json:7 (`build: tsc`), mcp_server/package.json:8 (`start: node start.js`).
- Required prep: install deps and build mcp_server before using `start`.

3. Validate auth expectations for MRT tools.
- Evidence: mcp_server/src/server.ts:26-29 (reads x-mcp-api-key/authorization), mcp_server/src/server.ts:225-229 (role check in dispatch), mcp_server/src/server.ts:787-790 (stdio `msg.apiKey` injection).
- If Copilot client does not pass `apiKey` envelope field, MRT role-gated tools may return 401/403.

4. Run connection tests.
- Baseline runtime test: mcp_server/package.json:10 (`npm run build && node tests/stdio_proxy_test.js`).
- Startup assertion: mcp_server/src/server.ts:1326 should log stdio-direct startup.
- Discovery assertion: ListTools path should resolve through mcp_schema mapping (mcp_server/src/server.ts:753-754).
- Tool-call assertion: call at least one non-privileged tool and one MRT tool with valid role key to confirm RBAC path.

## Workflow Markdown Files That Should Be Updated

| File | Why update is needed | Evidence |
|---|---|---|
| .github/copilot-instructions.md | Has a workflow section but does not describe how Copilot connects to local MCP server, nor how to test that connection in this repo. | .github/copilot-instructions.md:33, .github/copilot-instructions.md:35, .github/copilot-instructions.md:55-61 |
| MIRR_ARSENAL_README.md | Defines the governance workflow loop but omits MCP bridge connection/testing step for agent runtime path. | MIRR_ARSENAL_README.md:32-38 |
| mcp_server/README.md | Quick-start and transport notes are partly stale relative to active runtime: file says stdio-only (lines 12-16) but also documents HTTP endpoint usage and localhost curl flow (lines 40-66, 81-84). Needs one canonical Copilot/MCP connection workflow. | mcp_server/README.md:12-16, mcp_server/README.md:40-66, mcp_server/README.md:81-84, mcp_server/src/server.ts:1325-1327 |
| AGENTS.md (missing) | Campaign required this file, but it is absent. Either restore AGENTS.md or update canonical replacement workflow doc to include MCP connection protocol and pointers. | `bat AGENTS.md` os error 2; `rg --files -g '*AGENTS.md'` no matches |

## Final Answer

- Copilot is connected to an MCP endpoint in this workspace: YES (GitHub-hosted endpoint via .vscode/mcp.json).
- The repo-local MCP server is connected to Copilot: NO (no local mcp_server registration in .vscode/mcp.json).
- To connect and test: add local stdio server entry in .vscode/mcp.json, build/start mcp_server, verify stdio startup and ListTools/CallTool flow, and run npm test in mcp_server.
