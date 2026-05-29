---
title: Consumer Contracts
status: Active
---

# Consumer Contracts

This document defines repo-wide consumer contracts used by architecture proposals.

| Consumer | Contract |
|---|---|
| crates/mirr-wasm | Public exported API remains backward-compatible unless explicitly versioned |
| crates/mirr-arsenal-wasm | Compile-contract output remains deterministic and schema-stable |
| crates/lra-cli | Compile path must call compiler library entrypoints, not shell-out wrappers |
| mcp_server | Tool routing is explicit, typed, allowlisted, and keeps MRT tool names stable; interface/bridge layer only and must not own core compiler logic |
| vscode-mirr | Package contract text must match actual capability |
| demos/proofs/fuzz/scripts | Must have explicit compatibility evidence in architecture waves |

## Ownership
- Compiler boundary: elvie (primary), compiler maintainers (backup), compiler reviewers (escalation)
- Consumer boundary: elvie (primary), consumer maintainers (backup), architecture reviewers (escalation)
- Proposal conformance: proposal reviewers (primary), repository governance maintainers (backup), campaign owner (escalation)

## Section C: Security Contract Obligations

### Default-Deny Policy
- All MCP tool routes are deny-by-default.
- A request is rejected unless the tool name is present in the explicit server allowlist.
- Unknown or unmapped tool names MUST return the canonical error envelope.

### Authentication Requirements
- Protected MCP operations require authenticated caller identity before tool dispatch.
- Unauthenticated requests to protected operations MUST be rejected before handler execution.
- Authentication failure responses MUST use the canonical error envelope.

### Validation Behavior
- Every request payload MUST be validated against its declared schema before execution.
- Validation failure MUST prevent tool execution.
- Validation errors MUST return the canonical error envelope with validation-specific details in details.

### Rate-Limit Behavior
- Rate limits are enforced per caller identity or transport principal.
- Requests exceeding configured limits MUST be rejected without invoking tool execution.
- Rate-limit rejections MUST return the canonical error envelope.

### Canonical Error Envelope Contract
- All non-success responses from MCP tool routing, authentication, validation, and rate-limit checks MUST use the same envelope fields.
- Required fields:

| Field | Type | Contract |
|---|---|---|
| ok | boolean | MUST be false on error responses |
| error.code | string | Stable machine-readable code |
| error.message | string | Human-readable error summary |
| error.details | object or null | Structured context for validation, auth, routing, or rate-limit failures |
| request_id | string | Correlation identifier for the request |
| timestamp | string | UTC timestamp in RFC 3339 format |

## Section D: LRA Tool Namespace and Contracts (Wave 4)

### Overview and Namespace Conventions

The LRA tool namespace exposes Linear Relation Analysis capabilities via the MCP protocol. All LRA tools:

- Follow the naming convention `lra_*` (prefix reserved for LRA dispatcher).
- Are subject to the default-deny authentication and authorization policy (Section C).
- Return responses in the canonical error envelope format on failure.
- Support per-caller rate-limiting (inherited from Wave 3).
- Are bounded by NASA Power-of-10 constraints: all string inputs have maximum lengths, all numeric inputs have ranges, and directory traversals are validated.

LRA tools are implemented as wrappers around the `lra-cli` command-line utility. Tool dispatch into the MCP protocol allows orchestrators and automation frameworks to:

- Scaffold and initialize Living Research Artifact projects.
- Validate papers against the LRA-1.0 specification (Bronze/Silver/Gold compliance tiers).
- Serve interactive papers with live reload during development.
- Sign verification receipts with Ed25519 keypairs.
- Verify deployed papers' claims and content integrity.
- Check compliance status and retrieve diagnostic information.

### Role-Based Access Control for LRA Tools

LRA tools follow a three-tier role hierarchy:

| Role | Write Capability | LRA Tool Access |
|---|---|---|
| **admin** | Full | All LRA tools (init, validate, serve, check, sign, verify) |
| **committer** | File write | lra_init, lra_validate, lra_serve, lra_check, lra_sign, lra_verify |
| **builder** | Read-only | lra_validate, lra_serve, lra_check, lra_verify |
| **anonymous** | None | Denied (401 auth_missing_api_key) |

**Access Grants:**

- `lra_init` (write/scaffold): **admin**, **committer** — Only authenticated users with project creation authority may scaffold new LRA projects.
- `lra_validate` (read/analyze): **admin**, **committer**, **builder** — Compliance checking is read-only; all roles with build access may validate.
- `lra_serve` (write/server): **admin**, **committer**, **builder** — Dev server startup is write-capable but non-destructive; permitted for builders.
- `lra_check` (read/alias): **admin**, **committer**, **builder** — Alias for `lra_validate`; same access as validate.
- `lra_sign` (write/crypto): **admin**, **committer** — Cryptographic signing requires elevated trust; restricted to committers and admins.
- `lra_verify` (read/network): **admin**, **committer**, **builder** — Network verification is read-only; all roles with build access may verify.

**Access Denial:**

- Unauthenticated callers: 401 `auth_missing_api_key`
- Invalid API key: 403 `auth_invalid_api_key`
- Insufficient role for tool: 403 `auth_insufficient_role` (details include caller's current role)
- Unknown tool name: 400 `validation_unknown_method`

### LRA Tool Catalog

Six LRA tools are exposed via the MCP `mrt_dispatch` protocol. Each tool maps to a subcommand in `lra-cli`.

#### Tool Reference Table

| Tool Name | Category | Type | Required Roles | Primary Input | Success Response | Status Codes |
|---|---|---|---|---|---|---|
| `lra_init` | Project | Write | admin, committer | project_name (string, 1–255 chars, no path separators) | 200: STDOUT from `lra init <name>`; project structure created | 201, 400, 403, 429 |
| `lra_validate` | Compliance | Read | admin, committer, builder | path (string, 1–1024 chars, no `..`) | 200: STDOUT from `lra validate <path>`; tier (Bronze/Silver/Gold) | 200, 400, 403, 404, 429 |
| `lra_serve` | Dev Server | Write | admin, committer, builder | port (number, 1024–65535; optional, default 8080) | 200: STDOUT; server running at specified port | 200, 400, 403, 429 |
| `lra_check` | Compliance | Read | admin, committer, builder | path (string, 1–1024 chars) | 200: STDOUT from `lra validate`; alias for validate | 200, 400, 403, 404, 429 |
| `lra_sign` | Crypto | Write | admin, committer | receipt (string, 1–1024 chars); key (path, 1–1024 chars, default: lra-identity.key) | 200: STDOUT; signed receipt JSON | 200, 400, 403, 429 |
| `lra_verify` | Network | Read | admin, committer, builder | target (URL or hash, 1–1024 chars, no `..`) | 200: STDOUT; verification results (claims, tier, integrity) | 200, 400, 403, 429 |

#### Tool Details and Error Mappings

##### `lra_init` — Initialize LRA Project

**Category:** Write / Scaffold

**Roles:** admin, committer

**Input Schema:**

```json
{
  "project_name": "my-paper"
}
```

| Parameter | Type | Required | Bounds | Notes |
|---|---|---|---|---|
| `project_name` | string | Yes | 1–255 characters | Project directory name; cannot contain `/`, `\`, or `..` |

**Success Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_init",
  "exitCode": 0,
  "stdout": "Created LRA project: my-paper/\n  index.html     — your paper (edit this!)\n  paper.css      — styling\n  ...",
  "stderr": "",
  "output_limit_bytes": 65536
}
```

**Behavior:**

- Creates a directory with the project name.
- Scaffolds standard LRA-1.0 files: `index.html`, `paper.css`, `paper.js`, `sw.js`, `CITATION.cff`, `LICENSE`.
- Returns exit code 0 on success.
- Returns exit code 1 if directory already exists or write permission denied.

**Error Scenarios:**

| HTTP Status | error_code | error_message | details | Cause |
|---|---|---|---|---|
| 401 | `auth_missing_api_key` | API key is required. | — | No X-MCP-API-Key header provided |
| 403 | `auth_invalid_api_key` | API key is invalid. | — | API key does not match any known key |
| 403 | `auth_insufficient_role` | API key role is not allowed. | `{ "role": "builder" }` | Caller has role builder; requires committer or admin |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "missing_project_name" }` | project_name argument absent or empty |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "project_name_too_long" }` | project_name exceeds 255 characters |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "invalid_project_name" }` | project_name contains path separators or `..` |
| 400 | `validation_mrt_exec_failed` | MRT execution failed. | `"Directory already exists"` | Scaffolding failed (dir exists, permission denied, etc.) |
| 429 | `limit_concurrency_exceeded` | Concurrency limit exceeded. | — | Caller has exhausted per-identity concurrency quota |

##### `lra_validate` — Validate LRA Compliance

**Category:** Read / Analyze

**Roles:** admin, committer, builder

**Input Schema:**

```json
{
  "path": "my-paper/index.html"
}
```

| Parameter | Type | Required | Bounds | Notes |
|---|---|---|---|---|
| `path` | string | Yes | 1–1024 characters | Path to LRA index.html or project directory; normalized; no `..` |

**Success Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_validate",
  "exitCode": 0,
  "stdout": "LRA Validate — my-paper/index.html\n\nBronze (16 checks)\n  ✓ html lang attribute\n  ✓ meta charset\n  ...\nSilver (3 checks)\n  ✓ demo section\n  ...\nGold (4 checks)\n  ✗ WASM module present\n  ...\nTier: SILVER (23/23 pass)\n",
  "stderr": "",
  "output_limit_bytes": 65536
}
```

**Behavior:**

- Reads and parses the LRA HTML and project structure.
- Runs 23 compliance checks categorized into Bronze (16), Silver (3), and Gold (4).
- Reports pass/fail status for each check.
- Assigns a compliance tier (Bronze, Silver, Gold, or None).
- Returns exit code 0 on success (even if tier is None; validation process succeeded).

**Validation Tiers:**

- **Bronze:** Must pass all 16 Bronze checks (HTML structure, metadata, LICENSE, CITATION.cff, etc.).
- **Silver:** Must pass all Bronze + pass all 3 Silver checks (demo section, noscript fallback, paper.js).
- **Gold:** Must pass all Bronze + Silver + pass all 4 Gold checks (WASM module, evidence links, no external resources, aria-live).
- **None:** Fails to pass Bronze checks.

**Error Scenarios:**

| HTTP Status | error_code | error_message | details | Cause |
|---|---|---|---|---|
| 401 | `auth_missing_api_key` | API key is required. | — | No X-MCP-API-Key header |
| 403 | `auth_invalid_api_key` | API key is invalid. | — | API key invalid |
| 403 | `auth_insufficient_role` | API key role is not allowed. | `{ "role": "anonymous" }` | Caller refused role check |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "missing_path" }` | path argument absent |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "path_too_long" }` | path exceeds 1024 characters |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "invalid_path" }` | path contains `..` (traversal attempt) |
| 404 | `validation_mrt_exec_failed` | MRT execution failed. | `"Cannot read <path>: file not found"` | File does not exist or cannot be read |
| 400 | `validation_mrt_exec_failed` | MRT execution failed. | `"I/O error"` | File system error (permissions, etc.) |
| 429 | `limit_concurrency_exceeded` | Concurrency limit exceeded. | — | Caller concurrency quota exceeded |

##### `lra_serve` — Development Server

**Category:** Write / Server

**Roles:** admin, committer, builder

**Input Schema:**

```json
{
  "port": 8080
}
```

| Parameter | Type | Required | Bounds | Notes |
|---|---|---|---|---|
| `port` | number | No | 1024–65535 (default: 8080) | Port for dev server; must be in valid range |

**Success Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_serve",
  "exitCode": 0,
  "stdout": "LRA Dev Server\n  Serving at http://localhost:3000\n  Press Ctrl+C to stop\n  Watching for file changes...\n",
  "stderr": "",
  "output_limit_bytes": 65536
}
```

**Behavior:**

- Starts a local HTTP dev server.
- Watches for file changes and triggers live reload.
- Binds to the specified port (or default 8080).
- Returns exit code 0 on success.
- Returns exit code 1 if port is already in use or permission denied.

**Error Scenarios:**

| HTTP Status | error_code | error_message | details | Cause |
|---|---|---|---|---|
| 401 | `auth_missing_api_key` | API key is required. | — | No API key |
| 403 | `auth_invalid_api_key` | API key is invalid. | — | API key invalid |
| 403 | `auth_insufficient_role` | API key role is not allowed. | `{ "role": "anonymous" }` | Insufficient role |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "invalid_port" }` | Port outside 1024–65535 range |
| 400 | `validation_mrt_exec_failed` | MRT execution failed. | `"Address already in use"` | Port in use; bind failed |
| 429 | `limit_concurrency_exceeded` | Concurrency limit exceeded. | — | Concurrency quota exceeded |

##### `lra_check` — Check Compliance (Alias)

**Category:** Read / Alias

**Roles:** admin, committer, builder

**Input Schema:** Same as `lra_validate`

```json
{
  "path": "my-paper/index.html"
}
```

**Behavior:**

- Alias for `lra_validate` (compliance check).
- Returns identical output and error codes as `lra_validate`.
- Useful for semantic clarity when querying compliance status.

**Error Scenarios:** See `lra_validate` error table above.

##### `lra_sign` — Sign Verification Receipt

**Category:** Write / Crypto

**Roles:** admin, committer

**Input Schema:**

```json
{
  "receipt": "my-paper/verification-receipt.json",
  "key": "lra-identity.key"
}
```

| Parameter | Type | Required | Bounds | Notes |
|---|---|---|---|---|
| `receipt` | string | Yes | 1–1024 characters | Path to JSON receipt file (from `lra verify --receipt`) |
| `key` | string | No | 1–1024 characters (default: lra-identity.key) | Path to Ed25519 secret key file |

**Success Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_sign",
  "exitCode": 0,
  "stdout": "{\n  \"receipt\": { ... },\n  \"signature\": \"abc123...\",\n  \"public_key\": \"xyz789...\",\n  \"timestamp\": \"2026-04-06T14:22:11Z\"\n}\n",
  "stderr": "",
  "output_limit_bytes": 65536
}
```

**Behavior:**

- Reads a verification receipt (JSON).
- Reads the Ed25519 secret key.
- Signs the receipt with the key.
- Outputs the signed receipt with signature and public key.
- Returns exit code 0 on success.

**Error Scenarios:**

| HTTP Status | error_code | error_message | details | Cause |
|---|---|---|---|---|
| 401 | `auth_missing_api_key` | API key is required. | — | No API key |
| 403 | `auth_invalid_api_key` | API key is invalid. | — | API key invalid |
| 403 | `auth_insufficient_role` | API key role is not allowed. | `{ "role": "builder" }` | Insufficient role (requires committer or admin) |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "missing_receipt" }` | receipt argument absent |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "receipt_too_long" }` | receipt path exceeds 1024 characters |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "invalid_path" }` | receipt or key contains `..` |
| 404 | `validation_mrt_exec_failed` | MRT execution failed. | `"Receipt file not found"` | Receipt file does not exist |
| 404 | `validation_mrt_exec_failed` | MRT execution failed. | `"Key file not found"` | Key file does not exist |
| 400 | `validation_mrt_exec_failed` | MRT execution failed. | `"Invalid key format"` | Key file is not a valid Ed25519 secret key |
| 429 | `limit_concurrency_exceeded` | Concurrency limit exceeded. | — | Concurrency quota exceeded |

##### `lra_verify` — Verify Deployed Paper

**Category:** Read / Network

**Roles:** admin, committer, builder

**Input Schema:**

```json
{
  "path": "https://example.com/paper/"
}
```

| Parameter | Type | Required | Bounds | Notes |
|---|---|---|---|---|
| `path` (or `target`) | string | Yes | 1–1024 characters | URL of deployed paper or hash (registry lookup); no `..` |

**Success Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_verify",
  "exitCode": 0,
  "stdout": "LRA Verify — https://example.com/paper/\n\n  [PASS] Target URL: https://example.com/paper/\n  [PASS] Content hash matches registry\n  [PASS] LRA version tag detected\n  [PASS] Service Worker reference found\n  [PASS] 5 claims extracted\n  [PASS] All structural checks passed\n  [PASS] Compliance tier: SILVER\n\nVerification complete (all checks passed).\n",
  "stderr": "",
  "output_limit_bytes": 65536
}
```

**Behavior:**

- Fetches the HTML from the target URL (or looks up URL in registry by hash).
- Extracts and validates claims marked with `data-lra-claim` attributes.
- Checks structural markers (lra:version, sw.js reference, etc.).
- Computes SHA256 hash of content.
- Compares hash against registry (if available).
- Returns exit code 0 on success.
- Outputs detailed pass/fail status for each verification check.

**Error Scenarios:**

| HTTP Status | error_code | error_message | details | Cause |
|---|---|---|---|---|
| 401 | `auth_missing_api_key` | API key is required. | — | No API key |
| 403 | `auth_invalid_api_key` | API key is invalid. | — | API key invalid |
| 403 | `auth_insufficient_role` | API key role is not allowed. | `{ "role": "anonymous" }` | Insufficient role |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "missing_target" }` | target argument absent |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "target_too_long" }` | target exceeds 1024 characters |
| 400 | `validation_failed` | Invalid MRT dispatch input. | `{ "reason": "invalid_target" }` | target contains `..` |
| 404 | `validation_mrt_exec_failed` | MRT execution failed. | `"HTTP 404: target not found"` | URL cannot be reached |
| 400 | `validation_mrt_exec_failed` | MRT execution failed. | `"Connection timeout (10s)"` | URL fetch timeout exceeded |
| 400 | `validation_mrt_exec_failed` | MRT execution failed. | `"Invalid JSON: registry parse failed"` | Registry file malformed |
| 429 | `limit_concurrency_exceeded` | Concurrency limit exceeded. | — | Concurrency quota exceeded |

### Rate Limiting for LRA Tools

LRA tools are subject to the same per-caller rate-limiting policy as other MRT tools (Wave 3 default):

- **Rate Limit:** Enforced per caller identity (API key holder).
- **Capacity:** Default 10 concurrent requests per caller.
- **Enforcement:** Before tool execution; no request to the subprocess if limit exceeded.
- **Response on Limit Exceeded:** HTTP 429 with error code `limit_concurrency_exceeded`.

```json
{
  "error_code": "limit_concurrency_exceeded",
  "message": "Concurrency limit exceeded.",
  "details": null
}
```

**Example:** Caller A has 10 concurrent LRA requests in flight. If caller A sends request #11, the server responds with HTTP 429 immediately without executing the tool.

### Example: Validating and Signing an LRA Project

Below is a complete walkthrough of a multi-step LRA workflow: scaffold, validate, and sign a verification receipt.

#### Step 1: Initialize a New LRA Project

**Request:**

```http
POST /mrt_dispatch HTTP/1.1
X-MCP-API-Key: sk-committer-abc123
Content-Type: application/json

{
  "tool": "lra_init",
  "args": {
    "project_name": "my-research-paper"
  }
}
```

**Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_init",
  "exitCode": 0,
  "stdout": "Created LRA project: my-research-paper/\n  index.html\n  paper.css\n  paper.js\n  sw.js\n  CITATION.cff\n  LICENSE\n",
  "stderr": ""
}
```

#### Step 2: Validate Compliance

**Request:**

```http
POST /mrt_dispatch HTTP/1.1
X-MCP-API-Key: sk-committer-abc123
Content-Type: application/json

{
  "tool": "lra_validate",
  "args": {
    "path": "my-research-paper/index.html"
  }
}
```

**Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_validate",
  "exitCode": 0,
  "stdout": "LRA Validate — my-research-paper/index.html\n\nBronze (16 checks)\n  ✓ html lang attribute\n  ✓ meta charset\n  ✓ meta viewport\n  ...\n\nSilver (3 checks)\n  ✓ demo section\n  ✓ noscript fallback\n  ✓ paper.js exists\n\nGold (4 checks)\n  ✗ WASM module present\n  ✓ evidence link\n  ✓ no external resources\n  ✓ aria-live region\n\nTier: SILVER (22/23 pass)\n",
  "stderr": ""
}
```

#### Step 3: Verify Deployed Paper (Network Check)

**Request:**

```http
POST /mrt_dispatch HTTP/1.1
X-MCP-API-Key: sk-committer-abc123
Content-Type: application/json

{
  "tool": "lra_verify",
  "args": {
    "path": "https://example.org/papers/my-research-paper/"
  }
}
```

**Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_verify",
  "exitCode": 0,
  "stdout": "LRA Verify — https://example.org/papers/my-research-paper/\n\n  [PASS] Target URL accessible\n  [PASS] Content hash: abc123def456...\n  [PASS] LRA version tag detected\n  [PASS] Service Worker reference found\n  [PASS] 3 claims extracted\n  [PASS] Compliance tier: SILVER\n\nVerification complete (all checks passed).\n",
  "stderr": ""
}
```

**Verification receipt generated and written to:**
```
my-research-paper/verification-receipt.json
```

#### Step 4: Sign Verification Receipt

**Request:**

```http
POST /mrt_dispatch HTTP/1.1
X-MCP-API-Key: sk-committer-abc123
Content-Type: application/json

{
  "tool": "lra_sign",
  "args": {
    "receipt": "my-research-paper/verification-receipt.json",
    "key": "~/.lra/identity.key"
  }
}
```

**Response (HTTP 200):**

```json
{
  "schema_version": "1",
  "tool": "lra_sign",
  "exitCode": 0,
  "stdout": "{\n  \"receipt\": {\n    \"target_url\": \"https://example.org/papers/my-research-paper/\",\n    \"claims_found\": 3,\n    \"compliance_tier\": \"SILVER\",\n    \"timestamp\": \"2026-04-06T15:30:00Z\"\n  },\n  \"signature\": \"ed25519_sig_abc123...\",\n  \"public_key\": \"ed25519_pk_xyz789...\",\n  \"signed_at\": \"2026-04-06T15:30:05Z\"\n}\n",
  "stderr": ""
}
```

### Example: Error Handling

#### Insufficient Role for `lra_sign`

**Request (with builder role):**

```http
POST /mrt_dispatch HTTP/1.1
X-MCP-API-Key: sk-builder-xy9
Content-Type: application/json

{
  "tool": "lra_sign",
  "args": {
    "receipt": "receipt.json",
    "key": "identity.key"
  }
}
```

**Response (HTTP 403):**

```json
{
  "error_code": "auth_insufficient_role",
  "message": "API key role is not allowed.",
  "details": {
    "role": "builder"
  }
}
```

#### Rate Limit Exceeded

**Request (11th concurrent request from same caller):**

```http
POST /mrt_dispatch HTTP/1.1
X-MCP-API-Key: sk-committer-abc123
Content-Type: application/json

{
  "tool": "lra_validate",
  "args": { "path": "..." }
}
```

**Response (HTTP 429):**

```json
{
  "error_code": "limit_concurrency_exceeded",
  "message": "Concurrency limit exceeded.",
  "details": null
}
```

#### Missing Required Argument

**Request (no project_name):**

```http
POST /mrt_dispatch HTTP/1.1
X-MCP-API-Key: sk-committer-abc123
Content-Type: application/json

{
  "tool": "lra_init",
  "args": {}
}
```

**Response (HTTP 400):**

```json
{
  "error_code": "validation_failed",
  "message": "Invalid MRT dispatch input.",
  "details": {
    "reason": "missing_project_name"
  }
}
```

### LRA Tool Argument Validation Reference

All LRA tool arguments are validated before tool execution. Validators are defined in [mcp_server/src/mrt_kb_lite.ts](../mcp_server/src/mrt_kb_lite.ts).

| Tool | Arg Name | Type | Max Length | Special Rules | Error Code |
|---|---|---|---|---|---|
| `lra_init` | project_name | string | 255 | No `/`, `\`, `..` | invalid_project_name |
| `lra_validate` | path | string | 1024 | No `..` | invalid_path |
| `lra_serve` | port | number | — | 1024–65535 | (handled by OS bind) |
| `lra_check` | path | string | 1024 | No `..` | invalid_path |
| `lra_sign` | receipt | string | 1024 | No `..` | invalid_path |
| `lra_sign` | key | string | 1024 | No `..` (default: lra-identity.key) | invalid_path |
| `lra_verify` | target | string | 1024 | No `..` (URL or hash) | invalid_target |

### Deprecation and Evolution

- **Current Status:** LRA tools are actively maintained and part of Wave 4 arsenal (2026-04-06).
- **Stability:** Tool names, argument keys, role requirements, and error codes are stable per this contract.
- **Future Evolution:** If LRA-1.0 specification evolves or new LRA tools are added, changes will be documented in a subsequent wave (e.g., Wave 5). Current tools will maintain backward compatibility.
- **Sunset Policy:** No lra_* tools are currently scheduled for deprecation. If removal or breaking changes are planned, advance notice of 1 full wave cycle (minimum 2 weeks) will be provided in project documentation.