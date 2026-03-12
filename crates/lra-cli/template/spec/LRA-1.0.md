# Living Research Artifact Specification v1.0

**Status:** Stable
**Date:** 2026-03-12
**License:** GPL-3.0

---

## 1. Purpose

A Living Research Artifact (LRA) is a self-contained, browser-readable
research paper that bundles executable evidence alongside its claims.
Unlike traditional PDFs, an LRA lets reviewers and readers verify claims
by executing the author's tool directly in the browser — no installation,
no server, no account.

The format is designed to be:

- **Verifiable**: Claims link to executable evidence.
- **Permanent**: GPL-3.0 ensures the artifact can never be paywalled.
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
| `LICENSE` | GPL-3.0 license text |
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

1. A `LICENSE` file containing the GPL-3.0 full text
2. A visible link to the license in the paper header or footer
3. The `license` field in `CITATION.cff` set to `GPL-3.0`

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
It is a static HTML paper with claims, references, and GPL-3.0 license.
No JavaScript required.

**Verification**: Bronze compliance can be checked by confirming:
- `<section id="abstract">` exists
- `<section id="claims">` exists with an `<ol>` containing `<li>` elements
- `<section id="references">` exists
- `<section id="citation">` exists
- A `LICENSE` file contains "GNU GENERAL PUBLIC LICENSE"
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

### 6.3 GPL-3.0 Copyleft

The GPL-3.0 license is a deliberate design choice, not a default.

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
<meta name="lra:license" content="GPL-3.0">
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
