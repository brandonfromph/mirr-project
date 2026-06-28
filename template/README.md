# Living Research Artifact Template

> Your paper runs in the browser. Every claim is verifiable.
> Every fork carries GPL-3.0-or-later. The format spreads.

[![LRA-1.0](https://img.shields.io/badge/standard-LRA--1.0-gold)](spec/LRA-1.0.md)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)

## 30-Second Demo

**Option A** — with the CLI:
```bash
cargo install lra-cli
lra init my-paper
cd my-paper && lra serve
# Open http://localhost:8080
```

**Option B** — without installing anything:
1. Click **"Use this template"** on GitHub (or clone/fork)
2. `cd your-paper && python3 -m http.server 8080`
3. Open [http://localhost:8080](http://localhost:8080)

Either way, you see a working regex tester demo — a placeholder for YOUR tool.

**No npm. No CDN. No build step.** The demo runs from static files on any web server.

## Prerequisites

- A text editor and a web browser
- **Optional:** [Python 3](https://python.org) (for `python3 -m http.server`)
- **Optional:** [Rust](https://rustup.rs) (for `cargo install lra-cli`, WASM builds)
- **Optional:** [wasm-pack](https://rustwasm.github.io/wasm-pack/) (for WASM compilation)

## 5-Minute Setup

### Step 1: Create Your Paper

**Option A** — GitHub template (recommended):
Click the green **"Use this template"** button at the top of this repo. GitHub creates
a fresh repository under your account with the full template contents.

**Option B** — CLI scaffold:
```bash
cargo install lra-cli
lra init my-paper
cd my-paper
```

**Option C** — Manual clone:
```bash
git clone https://github.com/brandonfromph/lra-template my-paper
cd my-paper
rm -rf .git && git init
```

### Step 2: Write Your Content

Edit `index.html` — find every `<!-- REPLACE: ... -->` marker and fill in your content.
Every research paper needs:

- **Abstract** — problem, approach, key results
- **Numbered Claims** — each linked to evidence with `[Evidence]` anchors
- **At least one Demo section** — the interactive proof
- **References** — standard academic citation format

The template enforces this structure. If you skip a section, reviewers will notice
the placeholder text. That is intentional.

### Step 3: Connect Your Tool

#### Option A: Pure JavaScript (No Build Step)

The built-in regex tester is pure JS. Replace it with any client-side tool.
The key function is `run()` in `paper.js`:

```javascript
// In paper.js — replace the body of run():
function run() {
  const source = document.getElementById('tool-input').value;
  const output = document.getElementById('tool-output');

  const result = myAnalysis(source);
  output.textContent = result;
  output.setAttribute('aria-busy', 'false');
}
```

This is the fastest path. If your tool runs in JavaScript, you are done.

#### Option B: WebAssembly (Full Power)

For compiled tools (Rust, C++, Go), build to WASM and drop the output into `wasm/`:

```bash
# Rust example with wasm-pack
wasm-pack build --target web --out-dir wasm
```

Then replace `paper.js` with a WASM-based version:

```javascript
import init, { analyze } from './wasm/my_tool.js';

let wasmReady = false;

async function initWasm() {
  await init();
  wasmReady = true;
}

function run() {
  if (!wasmReady) return;
  const source = document.getElementById('tool-input').value;
  const result = JSON.parse(analyze(source));
  const output = document.getElementById('tool-output');
  output.textContent = result.ok || result.err;
  output.setAttribute('aria-busy', 'false');
}

initWasm();
```

We recommend a simple JSON protocol for WASM tool returns:
```javascript
{ "ok": "output text here" }   // success
{ "err": "error message here" } // error
```

This convention lets the demo layer handle success/error display consistently.
It is not built into the template — you wire it up in your `run()` function.

#### Option C: External API (NOT Recommended)

Calling an external server violates two core LRA properties:

- **Zero dependencies** — the paper must work offline, from a local file
- **Permanence** — APIs shut down; static files survive

If your tool absolutely requires a server, document it as a limitation and provide
a fallback mode. Reviewers will test offline first.

### Step 4: Deploy

Validate your paper first:
```bash
lra validate             # Check LRA-1.0 compliance
lra serve                # Preview at http://localhost:8080
```

Push to `main` — GitHub Pages deploys automatically via the included workflow.

1. Go to repo **Settings > Pages**
2. Set Source to **"GitHub Actions"**
3. Push — the workflow validates LRA structure and deploys

Your paper is live at `https://your-username.github.io/your-repo/`.

For other hosts (Netlify, Vercel, any static server), just serve the files directly.
There is nothing to build.

## File Structure

| File | Purpose |
|------|---------|
| `index.html` | Your paper — the only file you must edit |
| `paper.css` | Styling: dark mode, print, responsive, accessible |
| `paper.js` | Interactive layer: tool loader, demos, keyboard shortcuts |
| `sw.js` | Service worker: offline cache + LRA protocol endpoints |
| `lra-client.js` | Cross-paper query library (for querying other LRAs) |
| `lra-card.svg` | Social sharing card (OpenGraph/Twitter) |
| `wasm/` | Your WASM build output (gitignored until you add one) |
| `spec/LRA-1.0.md` | The formal LRA specification |
| `CITATION.cff` | Citation metadata — edit with your details |
| `CONTRIBUTING.md` | How to contribute to the LRA standard |
| `LICENSE` | GPL-3.0-or-later full text |

## Compliance Tiers

| Tier | Requirements | Badge |
|------|-------------|-------|
| Bronze | Static paper with Claims + References + GPL-3.0-or-later | `LRA-1.0-bronze` |
| Silver | Bronze + at least one interactive demo | `LRA-1.0-silver` |
| Gold | Silver + WASM-powered tool execution in-browser | `LRA-1.0-gold` |

**This template starts at Silver.** The regex tester demo is a working interactive tool.
Add your own WASM module to reach Gold.

## Features

- **Dark mode** — automatic via `prefers-color-scheme`, no toggle needed
- **Print stylesheet** — hides interactive controls, produces clean PDF output
- **Keyboard shortcuts** — Ctrl+Enter (Cmd+Enter on Mac) to run the tool
- **Accessible** — `aria-live` on output, `aria-label` on controls, `focus-visible` outlines
- **Responsive** — split pane collapses to single column on mobile
- **Zero dependencies** — no npm, no CDN, no `fetch()` calls to external services
- **Offline-capable** — entire paper works from a local file server
- **Protocol-enabled** — JSON-RPC 2.0 over Service Worker for cross-paper queries

## What's Coming

The LRA format is at Phase 4. Recent deliverables:

| Phase | Capability | Status |
|-------|-----------|--------|
| **Phase 3** | `lra` CLI tool: `lra init`, `lra validate`, `lra serve` | Done |
| **Phase 4** | Papers as queryable APIs (JSON-RPC over Service Worker) | Done |
| **Phase 5** | Decentralized registry of LRA papers | Planned |

Service Worker protocol endpoints and `lra-client.js` are included in this template.
Papers built from this template can query each other's tools programmatically.

## Philosophy

Traditional academic publishing locks research behind paywalls, strips
reproducibility by separating the paper from the code, and relies on
peer reviewers who cannot execute the claims they evaluate.

A Living Research Artifact inverts all three failures:

| Traditional | LRA |
|------------|-----|
| Paper behind paywall | GPL-3.0-or-later: free forever, copyleft protects it |
| Code in a separate repo (if at all) | Paper IS the repo |
| Claims reviewed by reading | Claims verified by executing |
| Static PDF, frozen at submission | Interactive HTML, updated by commit |
| Reviewer installs nothing | Reviewer runs your tool in-browser |

The format is designed to spread. Every fork inherits GPL-3.0-or-later.
Every derivative work must remain open. The template is the vector;
the license is the mechanism.

## License

**GPL-3.0-or-later.** See [LICENSE](LICENSE).

This means:
- **You CAN** use this template for any research paper, commercial or academic
- **You CAN** modify the HTML, CSS, and JS to fit your needs
- **You MUST** keep GPL-3.0-or-later on derivative works of the template
- **You MUST** share your modifications to the template under the same terms

Your paper content (the research text you write, your figures, your data)
is yours. The Apache applies to the template code — the HTML structure, CSS, JS,
and build workflow — not to the intellectual content you put inside it.

That distinction matters: you own your research. The template ensures
the container stays open.
