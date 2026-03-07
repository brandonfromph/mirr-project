# nasa-rust-mcp-server

Local MCP server for the nasa-rust-project workspace. Provides safe, workspace-scoped tools for file operations, repository search, and running allowed `cargo` subcommands.

Quick start

1. Install dependencies:
   - npm install

2. Development:
   - npm run dev
   - The server operates exclusively in **stdio‑direct** mode.  It never
     opens a TCP port or named pipe; instead it reads line‑delimited JSON‑RPC
     requests from stdin and writes responses to stdout.  This removes all
     network configuration, eliminates `EADDRINUSE`/`ECONNRESET` issues, and
     mirrors how the built‑in VS‑Code MCP helpers work.
   - Unknown or future methods are not treated as fatal errors; the server
     logs the method name and replies with `result: null` so clients such as
     CLINE can probe for capabilities without seeing `-32601` errors.
   - Example CLINE configuration (stdio‑direct mode):
     ```json
     "local_custom": {
       "type": "stdio",
       "command": "node",
       "args": [
         "C:\\Users\\elvie\\nasa-rust-project\\mcp_server\\start.js",
         "--stdio-direct"
       ],
       "env": { "MCP_WORKSPACE_ROOT": "C:\\Users\\elvie\\nasa-rust-project" }
     }
     ```
   - To point the server at this workspace explicitly when running manually:
     - Windows (PowerShell/cmd): `set MCP_WORKSPACE_ROOT=c:\Users\elvie\nasa-rust-project`
     - Unix: `export MCP_WORKSPACE_ROOT=/path/to/nasa-rust-project`

3. Build for production:
   - npm run build
   - npm start

Exposed HTTP endpoints (POST except /health)
- POST /read_text_file
  - JSON: { "path": "relative/or/absolute/path/within/workspace", "head": number?, "tail": number? }
  - Returns: { path, content }

- POST /write_file
  - JSON: { "path": "relative/path", "content": "string" }
  - Writes file under workspace.

- POST /list_directory
  - JSON: { "path": "relative/dir" }
  - Returns entries with type and size.

- POST /directory_tree
  - JSON: { "path": "relative/dir", "excludePatterns": ["node_modules", "target"] }
  - Returns recursive tree JSON.

- POST /search_files
  - JSON: { "path": "relative/dir", "pattern": "**/*.rs", "ignore": ["target/**"] }
  - Returns matched file paths.

- POST /run_cargo
  - JSON: { "subcommand": "build" | "test" | "check", "args": ["--release"] }
  - Runs cargo in the workspace root. Only allowed subcommands to limit risk.

- GET /health
  - Returns server status and configured workspace root.

Security & notes
- The server restricts all file access to the configured workspace root (MCP_WORKSPACE_ROOT or inferred repo root).
- Only a small set of cargo subcommands are permitted. No network installs or arbitrary shell execution.
- Use environment variables to further restrict behavior if desired.


Authentication (API keys)
- A simple API-key based RBAC is supported. Generate a key with:
  node mcp_server/scripts/generate_api_key.js --id my-key --role committer
  To append the key to mcp_server/config.json (creates a backup), add --append:
  node mcp_server/scripts/generate_api_key.js --id my-key --role committer --append

- Example: call an endpoint with the X-MCP-API-KEY header
  curl -X POST http://localhost:8081/run_cargo \
    -H "Content-Type: application/json" \
    -H "X-MCP-API-KEY: <token-from-generator>" \
    -d '{"subcommand":"test","args":[]}'

Optional usage note
- The MCP server is an **optional** convenience layer. Agents or tools may choose to call the MCP HTTP endpoints or operate directly via the workspace CLI. The server does not enforce usage — it provides safer, workspace-scoped helpers (auth, whitelists, backups, audit) that other agents can opt into for convenience and safety.
