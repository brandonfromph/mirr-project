# Living Research Artifact Specification v1.0

**Status:** Stable
**Date:** 2026-03-12
**License:** GPL-3.0-or-later

---

## 1. Purpose

A Living Research Artifact (LRA) is a self-contained, browser-readable
research paper that bundles executable evidence alongside its claims.
Unlike traditional PDFs, an LRA lets reviewers and readers verify claims
by executing the author's tool directly in the browser — no installation,
no server, no account.

The format is designed to be:

- **Verifiable**: Claims link to executable evidence.
- **Permanent**: GPL-3.0-or-later ensures the artifact can never be paywalled.
- **Self-contained**: No external dependencies, CDN links, or API calls.
- **Archivable**: A single directory that works offline.

## 2. Definitions

- **Artifact**: The complete package — HTML, CSS, JS, and optional WASM
  files — that constitutes one Living Research Artifact.
- **Claim**: A numbered, verifiable assertion made by the paper.
  Claims are the atomic units of contribution.
- **Evidence**: An interactive section that demonstrates or verifies
  a claim. Evidence may be a live tool demo, a data table, or a
  benchmark result.
- **Tool**: A program (typically compiled to WebAssembly) that runs
  in the browser to produce evidence for claims.

## 3. Required Structure

### 3.1 Required Files

Every conforming LRA MUST contain:

| File | Purpose |
|------|---------|
| `index.html` | The paper itself |
| `paper.css` | Stylesheet (may be inlined) |
| `LICENSE` | GPL-3.0-or-later license text |
| `CITATION.cff` | Machine-readable citation metadata |

### 3.2 Required HTML Sections

The `index.html` file MUST contain the following sections, identified
by their `id` attribute:

| Section ID | Required Content |
|-----------|-----------------|
| `abstract` | Paper abstract |
| `claims` | Numbered list of verifiable claims |
| `references` | Academic citations |
| `citation` | BibTeX or equivalent citation block |

### 3.2.1 Claims Markup

Each claim MUST be an `<li>` element with:
- `id="claim-N"` — unique identifier
- `data-lra-claim="N"` — machine-readable claim number

Evidence links SHOULD include `data-lra-evidence="section-id"` pointing
to the `<section>` that verifies the claim.

### 3.2.2 Structured Metadata

An LRA SHOULD include a `<script type="application/ld+json">` block
using the [Schema.org ScholarlyArticle](https://schema.org/ScholarlyArticle)
type. This enables machine discovery of the paper's title, authors,
date, and license.

An LRA SHOULD include the following `<meta>` tags:

```html
<meta name="lra:abstract" content="...">
<meta name="lra:keywords" content="comma, separated, keywords">
```

### 3.3 Required Metadata

The `<head>` element MUST contain:

```html
<html lang="...">           <!-- Language attribute -->
<meta charset="UTF-8">      <!-- Character encoding -->
<meta name="viewport" ...>  <!-- Responsive viewport -->
<meta name="description" ...>  <!-- Paper description -->
<title>...</title>           <!-- Paper title -->
```

The `<head>` element SHOULD contain:

```html
<meta property="og:title" ...>        <!-- Open Graph title -->
<meta property="og:description" ...>  <!-- Open Graph description -->
<meta property="og:type" content="article">
```

### 3.4 Required License

Every conforming LRA MUST include these indicators:

1. A `LICENSE` file containing the GPL-3.0-or-later full text
2. A visible link to the license in the paper header or footer
3. The `license` field in `CITATION.cff` set to `GPL-3.0-or-later`

## 4. Optional Structure

### 4.1 Interactive Elements

An LRA MAY contain:

- `paper.js` — JavaScript for interactive demos
- `wasm/` — Directory containing WebAssembly modules
- One or more `<section class="demo">` blocks with interactive controls

### 4.2 Noscript Fallback

An LRA with JavaScript SHOULD include a `<noscript>` element
explaining what interactive features are unavailable without JS.

### 4.3 Accessibility

An LRA SHOULD include:

- `aria-label` attributes on interactive controls
- `aria-live="polite"` on output regions
- Keyboard shortcuts (documented in the UI)
- Focus-visible styles for keyboard navigation

## 5. Compliance Tiers

### 5.1 Bronze — Static

A Bronze LRA meets all Section 3 requirements.
It is a static HTML paper with claims, references, and GPL-3.0-or-later license.
No JavaScript required.

**Verification**: Bronze compliance can be checked by confirming:
- `<section id="abstract">` exists
- `<section id="claims">` exists with an `<ol>` containing `<li>` elements
- `<section id="references">` exists
- `<section id="citation">` exists
- A `LICENSE` file contains "GNU General Public License"
- A `CITATION.cff` file exists

### 5.2 Silver — Interactive

A Silver LRA meets Bronze requirements AND:
- Contains at least one `<section class="demo">` with interactive controls
- Includes a `<noscript>` fallback
- All body text is readable without JavaScript

**Verification**: Silver compliance adds:
- At least one `<section>` with `class="demo"`
- A `<noscript>` element exists
- `paper.js` exists (or inline `<script>`)

**Note on Protocol Requirements**: The Service Worker methods defined in
Section 11 (`lra.meta`, `lra.claims`, `lra.cite`, `lra.run_tool`) and the
`lra-client.js` library are now available (see Section 11.3). These are
NORMATIVE for Gold-tier papers but NOT required for Silver compliance.
Silver requires only an interactive demo, not protocol endpoints.

### 5.3 Gold — Executable

A Gold LRA meets Silver requirements AND:
- Loads a WebAssembly module that executes in the browser
- At least one claim is verifiable by running the tool in-browser
- Claims link to their evidence sections with anchor references
- Zero external dependencies (no CDN, no API calls, no fetch to remote servers)

**Verification**: Gold compliance adds:
- A `.wasm` file exists in the artifact
- At least one `<a href="#demo-...">` in the claims section
- No `fetch()` calls to external hosts in `paper.js`

## 6. Design Constraints

### 6.1 No External Dependencies

A conforming LRA MUST NOT load resources from external servers at runtime.
This includes:

- CDN-hosted CSS or JavaScript libraries
- Fonts loaded from external services
- API calls to remote servers
- Analytics or tracking scripts

**Rationale**: External dependencies break offline capability, introduce
privacy concerns, and create single points of failure. The artifact must
work from a local filesystem or GitHub Pages with no network beyond the
initial page load.

### 6.2 No Build Steps Required for Reading

A conforming LRA MUST be readable by opening `index.html` in a browser.
No build tool, package manager, or server is required to READ the paper.

A WASM module MAY require a build step to PRODUCE, but the pre-built
artifact must be serveable as static files.

### 6.3 GPL-3.0-or-later Copyleft

The GPL-3.0-or-later license is a deliberate design choice, not a default.

- It ensures the template format propagates openness: forks must
  share their modifications under the same terms.
- It prevents publishers from taking an LRA behind a paywall
  without violating the license already granted to the public.
- It applies to the template CODE (HTML structure, CSS, JavaScript),
  not to the CONTENT (the research text, figures, and data that
  authors write). Authors retain copyright on their content.

## 7. Non-Requirements

The following are explicitly NOT required:

- No specific CSS framework or design system
- No specific WASM toolchain (wasm-pack, Emscripten, etc.)
- No specific programming language for the tool
- No minimum paper length or section count beyond Section 3
- No specific citation style (APA, IEEE, etc.)
- No specific hosting provider (GitHub Pages is recommended, not required)

## 8. Versioning

This specification uses semantic versioning:

- **Patch** (1.0.x): Typo fixes, clarifications that don't change requirements
- **Minor** (1.x.0): New optional features, new compliance tier requirements
- **Major** (x.0.0): Breaking changes to required structure

A conforming artifact SHOULD declare its spec version in the footer
or metadata (e.g., `Standard: LRA-1.0`).

## 9. Conformance Badges

Authors MAY include a badge in their README or paper indicating
compliance tier:

```
Bronze: https://img.shields.io/badge/LRA-1.0--bronze-cd7f32
Silver: https://img.shields.io/badge/LRA-1.0--silver-c0c0c0
Gold:   https://img.shields.io/badge/LRA-1.0--gold-ffd700
```

## 10. Reference Implementation

The reference implementation is maintained at the template repository
URL declared in the template's `CITATION.cff` file. Forkers should update
this section to point to their own artifact.

## 11. Protocol (Informative — Future)

LRA-1.0 defines the document format. Future versions will define the
communication protocol that enables cross-paper queries and automated
verification.

### 11.1 Service Worker

Implementors SHOULD include a Service Worker (`sw.js`) that:

1. Caches the artifact for offline reading
2. Responds to `lra.ping` messages with `{ status: "ok", version: "1.0" }`

### 11.2 Capability Meta Tags

Implementors SHOULD include the following `<meta>` tags:

```html
<meta name="lra:version" content="1.0">
<meta name="lra:capability" content="your-tool-name">
<meta name="lra:license" content="GPL-3.0-or-later">
```

These tags declare the paper's identity and capabilities to the network.

### 11.3 LRA Protocol v1.0

The LRA Protocol uses JSON-RPC 2.0 envelopes over Service Worker
`postMessage` (same-origin) or `window.postMessage` via iframe
(cross-origin).

#### 11.3.1 Envelope Format

**Request:**
```json
{ "jsonrpc": "2.0", "method": "lra.meta", "id": 1 }
```

**Success response:**
```json
{ "jsonrpc": "2.0", "id": 1, "result": { ... } }
```

**Error response:**
```json
{ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" } }
```

#### 11.3.2 Methods

| Method | Params | Result |
|--------|--------|--------|
| `lra.ping` | none | `{ status, version, capability? }` |
| `lra.meta` | none | `{ title, authors, date, license, version, abstract, keywords, claims_count }` |
| `lra.run_tool` | `{ input, format? }` | `{ ok }` or `{ err }` |
| `lra.claims` | none | `[{ id, text, evidence_href }]` |
| `lra.cite` | `{ format: "bibtex"\|"apa"\|"ris" }` | `{ citation }` |

#### 11.3.3 Error Codes

| Code | Meaning |
|------|---------|
| `-32601` | Method not found |
| `-32602` | Invalid params |
| `-32603` | Tool execution error |
| `-32000` | Tool not loaded (WASM not ready or no active page) |

#### 11.3.4 Transport

**Same-origin:** Client sends `postMessage` to the controlling Service
Worker. The SW dispatches the method and replies via `event.source.postMessage`.

**Cross-origin:** Client creates a hidden `<iframe>` pointing to the target
paper. The iframe's `paper.js` receives the message via `window.onmessage`,
forwards it to its Service Worker, and relays the response back to the
parent via `event.source.postMessage`.

The reference client library `lra-client.js` implements create-iframe,
send-message, and timeout handling.

#### 11.3.5 run_tool Relay

The Service Worker cannot portably import WASM modules (module Service
Workers are not supported in all browsers). For `lra.run_tool`, the SW
relays the request to an open page tab via `self.clients.matchAll()` →
`client.postMessage()`. The page executes the tool and replies. The SW
forwards the result to the original requester.

If no page tab is open, the SW returns error code `-32000`.

#### 11.3.6 Compliance

The protocol methods are NORMATIVE for Gold-tier LRA papers that include
a WASM tool. Bronze and Silver papers MAY implement `lra.ping` and
`lra.meta` without implementing `lra.run_tool`.

The `lra-client.js` library is provided in the template for cross-paper
queries. Papers SHOULD include it as a static asset.

### 11.4 Headless Operation (Informative — Phase 6)

An LRA Service Worker SHOULD be capable of responding to protocol queries
without an open browser tab. The following methods SHOULD work headlessly:

- `lra.ping` — REQUIRED headless
- `lra.meta` — REQUIRED headless
- `lra.claims` — REQUIRED headless
- `lra.cite` — REQUIRED headless
- `lra.depends` — REQUIRED headless
- `lra.run_tool` — MAY require an open tab (if tool uses WASM or DOM)

#### 11.4.1 Rate Limiting

Headless LRA nodes SHOULD implement rate limiting to prevent abuse:
- Default: 60 requests per minute per client
- Rate limit exceeded returns error code `-32000` with message
  `"Rate limit exceeded"`
- The rate counter resets every 60 seconds

#### 11.4.2 Enhanced Ping

Headless nodes SHOULD return an enhanced `lra.ping` response:

```json
{
  "status": "ok",
  "version": "1.0",
  "capability": "tool-name",
  "uptime_ms": 3600000,
  "headless_methods": ["lra.ping", "lra.meta", "lra.claims", "lra.cite", "lra.depends"],
  "tool_requires_tab": true
}
```

#### 11.4.3 Graceful Degradation

When `lra.run_tool` is called without an open tab, the SW SHOULD return:

```json
{
  "error": {
    "code": -32000,
    "message": "Tool requires active browser tab",
    "data": {
      "headless": true,
      "retry": true,
      "methods_available": ["lra.ping", "lra.meta", "lra.claims", "lra.cite", "lra.depends"]
    }
  }
}
```

This tells the caller the paper node is alive and can respond to metadata
queries, but the tool requires a page tab for WASM execution.

### 11.5 Live Peer Review (Informative — Phase 7)

An LRA paper MAY support automated verification of its claims by other
LRA papers or CLI tools. This section specifies the protocol methods
that enable machine-to-machine peer review.

#### 11.5.1 `lra.verify_claim`

Sends a test input to a paper's tool to verify a specific claim.

**Parameters:**

```json
{
  "claim_id": "claim-1",
  "input": "module test { signal in s : u8; }"
}
```

**Response (claim has executable evidence):**

The request is relayed to the paper's tool (same as `lra.run_tool`).
The tool result is returned as the response.

**Response (claim has no executable evidence):**

```json
{
  "claim_id": "claim-1",
  "status": "no_executable_evidence",
  "claim_text": "Width inference is sound: no assignment silently truncates a value."
}
```

**Errors:**

- `-32602` if `claim_id` or `input` is missing
- `-32602` if `claim_id` is not found in `LRA_CLAIMS`

#### 11.5.2 `lra.challenge`

Records a failed verification attempt against a paper's claim.

**Parameters:**

```json
{
  "claim_id": "claim-1",
  "input": "module test { ... }",
  "expected": "expected output",
  "actual": "actual output",
  "verifier_hash": "sha256:abc123..."
}
```

**Response:**

```json
{
  "status": "challenge_recorded",
  "claim_id": "claim-1"
}
```

The challenge is stored in a bounded in-memory log (max 100 entries).

**Errors:**

- `-32602` if `claim_id` is missing

#### 11.5.3 `lra.verification_log`

Returns the append-only verification log.

**Parameters:** None.

**Response:** An array of verification entries:

```json
[
  {
    "claim_id": "claim-1",
    "input_hash": "sha256:e3b0c44298fc1c14...",
    "status": "verified",
    "timestamp": 1710489600000
  }
]
```

The `input_hash` is computed via Web Crypto `crypto.subtle.digest`
(SHA-256) to ensure unambiguous matching with CLI-signed receipts. The
log is bounded to 1000 entries.

#### 11.5.4 Verification Receipt Format

CLI tools (`lra verify`) produce structural verification receipts that
include:

- Content integrity: SHA-256 hash comparison against registry
- Claim extraction: list of claims with evidence classification
- Structural checks: LRA version tag, SW reference, capability tag

Browser-side verification (via `lra.verify_claim`) produces in-memory
log entries with SHA-256 input fingerprints.

#### 11.5.5 Node Identity

LRA nodes MAY have an Ed25519 keypair for signing verification receipts.
Key generation is performed by the CLI (`lra keygen`), which produces:

- `lra-identity.pub` — hex-encoded Ed25519 public key (32 bytes)
- `lra-identity.key` — hex-encoded Ed25519 secret key (64 bytes)

The keypair is stored on disk and managed by the CLI. Service Workers
do not store or use keypairs directly. Signed receipts are produced
offline by the CLI using the `ed25519-dalek` library.

### 11.6 Self-Healing Knowledge Graph (Informative — Phase 8)

An LRA network MAY form a self-healing knowledge graph where papers
track their dependency versions, receive update notifications, and
produce signed verification receipts. This section specifies the
protocol methods and CLi commands that enable this behavior.

#### 11.6.1 `lra.dep_versions`

Returns the full dependency list with version annotations.

**Parameters:** None.

**Response:**

```json
[
  { "hash": "sha256:abc123...", "min_version": "0.3.0" }
]
```

The existing `lra.depends` method continues to return flat hash strings
for backward compatibility. `lra.dep_versions` returns the full objects.

#### 11.6.2 `lra.notify`

Notifies a paper that one of its dependencies has been updated.
Fire-and-forget: the target logs the notification but does not
automatically re-verify.

**Parameters:**

```json
{
  "source_hash": "sha256:abc123...",
  "new_version": "0.4.0",
  "old_version": "0.3.0"
}
```

**Response:**

```json
{
  "status": "notification_received",
  "is_dependency": true
}
```

The notification is stored in a bounded in-memory log (max 100 entries).
The `is_dependency` field indicates whether the source hash matches a
declared dependency.

#### 11.6.3 `lra.notifications`

Returns the notification log.

**Parameters:** None.

**Response:** An array of notification entries:

```json
[
  {
    "source_hash": "sha256:abc123...",
    "new_version": "0.4.0",
    "old_version": "0.3.0",
    "is_dependency": true,
    "timestamp": 1710489600000
  }
]
```

#### 11.6.4 Signed Verification Receipts

CLI tools produce JSON verification receipts via `lra verify --receipt`:

```json
{
  "target_url": "https://example.github.io/paper/",
  "target_hash": "sha256:...",
  "registry_hash": "sha256:...",
  "integrity": "match",
  "claims_found": 4,
  "structural_checks": {
    "lra_version": true,
    "sw_reference": true,
    "claims_markup": true,
    "capability_tag": true
  },
  "timestamp": "2026-03-15T12:00:00Z",
  "verifier_version": "0.1.0"
}
```

Receipts are signed with `lra sign --key lra-identity.key --receipt receipt.json`,
which adds `signature` (hex-encoded Ed25519) and `signer_pubkey` fields.

#### 11.6.5 Semver Version Comparison

The CLI uses minimal semver parsing (major.minor.patch) for version-aware
dependency tracking. No range syntax or pre-release tags. Maximum 3 numeric
components, maximum 64 characters per version string.

#### 11.6.6 Network Health Status

The CLI command `lra status` queries every paper in the registry and reports:

- HTTP reachability
- Structural marker completeness (lra:version, sw.js, data-lra-claim, lra:capability)
- Content integrity (live SHA-256 vs registry hash)
- Version and verification summary

Each paper is fetched exactly once. Both marker checks and SHA-256 computation
use the same response body.

### 11.7 Peer-to-Peer Research Protocol (Informative — Phase 9)

Phase 9 adds cryptographic identity, computed reputation, peer discovery,
network crawling, and receipt verification.

#### 11.7.1 `lra.identity`

Returns the node's public identity, if configured.

**Request:** `{ "jsonrpc": "2.0", "method": "lra.identity", "id": 1 }`

**Response (configured):**
```json
{ "pubkey": "ed25519:<hex>", "name": "...", "url": "..." }
```

**Response (anonymous):**
```json
{ "pubkey": null, "status": "anonymous" }
```

Identity is opt-in. Nodes without a configured Ed25519 key respond as anonymous.

#### 11.7.2 `lra.reputation`

Returns a computed reputation score derived from the in-memory verification log.
The score is never stored — it is always recomputed on query.

**Request:** `{ "jsonrpc": "2.0", "method": "lra.reputation", "id": 1 }`

**Response:**
```json
{
  "total_verifications": 42,
  "verified": 40,
  "failed": 2,
  "challenges": 1,
  "score": 95,
  "uptime_ms": 86400000
}
```

- `score` is `round((verified / total) * 100)`, or `null` if no verifications exist.
- `uptime_ms` is milliseconds since SW install.

#### 11.7.3 `lra.peers`

Returns the node's list of known peer URLs.

**Request:** `{ "jsonrpc": "2.0", "method": "lra.peers", "id": 1 }`

**Response:**
```json
["https://example.github.io/paper-a/", "https://example.github.io/paper-b/"]
```

Peers are configured by the paper author in `sw.js` or discovered via crawl.
Bounded to `MAX_PEERS` (default: 50).

#### 11.7.4 Network Crawl

The CLI command `lra crawl <seed-url>` discovers the LRA network by:

1. Fetching the seed URL
2. Extracting `lra:capability`, `lra:version`, `data-lra-claim` markers
3. Following `lra:depends` links to discover upstream papers
4. Enriching results from the local registry

Crawl is bounded to `MAX_CRAWL_NODES` (default: 100) to prevent unbounded exploration.

#### 11.7.5 Receipt Verification

The CLI command `lra verify-receipt <path>` verifies a signed verification receipt:

1. Reads the `.signed.json` file
2. Extracts `signature` and `signer_pubkey` fields
3. Reconstructs the original unsigned content
4. Verifies the Ed25519 signature against the public key

If `--pubkey <path>` is provided, the receipt's signer is additionally validated
against the trusted public key file.
