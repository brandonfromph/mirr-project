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

The first LRA is the MIRR interactive paper:
https://brandonfromph.github.io/mirr-project/paper/

It serves as the reference implementation for Gold-tier compliance.
