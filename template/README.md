# Living Research Artifact Template

> A GPL-3.0 template for publishing interactive, verifiable research papers
> as single-page HTML applications with optional WebAssembly tools.

[![LRA-1.0](https://img.shields.io/badge/standard-LRA--1.0-gold)](spec/LRA-1.0.md)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)

## What is a Living Research Artifact?

A Living Research Artifact (LRA) is a research paper where:

1. **Claims are numbered and linked to evidence** — every assertion points to
   a demo or data table that verifies it.
2. **Tools run in the browser** — your compiler, simulator, or analysis tool
   executes as WebAssembly. Reviewers verify claims without installing anything.
3. **The paper is the repository** — code, proofs, data, and paper are one
   GPL-3.0 artifact. The paper can never be paywalled separately.

## Quick Start

1. Click **"Use this template"** on GitHub (or fork this repo)
2. Edit `index.html` — replace `<!-- REPLACE: ... -->` comments with your content
3. Edit `CITATION.cff` — add your name, title, and DOI
4. *(Optional)* Add your WASM module to `wasm/` and update `paper.js`
5. Push to GitHub — Pages deploys automatically via the included workflow

Your paper is now live at `https://your-username.github.io/your-repo/`.

## File Structure

| File | Purpose |
|------|---------|
| `index.html` | Your paper — edit this |
| `paper.css` | Styling (dark mode, print, responsive, accessible) |
| `paper.js` | Interactive layer — WASM loader, demos, keyboard shortcuts |
| `wasm/` | Your WASM build output goes here (gitignored) |
| `spec/LRA-1.0.md` | The formal LRA specification |
| `CITATION.cff` | Citation metadata — edit this |
| `CONTRIBUTING.md` | How to contribute to the LRA standard |
| `LICENSE` | GPL-3.0 full text |

## Compliance Tiers

The LRA specification defines three tiers:

| Tier | Badge | Requirements |
|------|-------|-------------|
| Bronze | `LRA-1.0-bronze` | Static HTML paper with Claims + References + GPL-3.0 |
| Silver | `LRA-1.0-silver` | Bronze + at least one interactive Demo section |
| Gold | `LRA-1.0-gold` | Silver + WASM-powered tool execution in-browser |

This template starts at **Silver** tier (has interactive demo scaffold).
Add your WASM module to reach **Gold**.

## Connecting Your WASM Module

The template uses a standard pattern for loading WASM modules:

1. Build your tool to WASM (using `wasm-pack`, `wasm-bindgen`, Emscripten, etc.)
2. Place the output files in `wasm/`
3. Edit `paper.js` — uncomment the import and replace function names:

```javascript
// Before (placeholder):
// import init, { your_function } from './wasm/your_tool.js';

// After (your tool):
import init, { compile, analyze } from './wasm/my_tool.js';
```

4. Update the `run()` function to call your tool
5. Update `EXAMPLES` with real examples from your paper

### JSON Protocol

We recommend a simple JSON protocol for WASM function returns:

```javascript
// Success:  {"ok": "output text here"}
// Error:    {"err": "error message here"}
```

This lets the demo layer handle success/error display consistently.

## Features

- **Dark mode** — automatic via `prefers-color-scheme`
- **Print stylesheet** — hides interactive controls, shows clean paper
- **Keyboard shortcuts** — Ctrl+Enter (Cmd+Enter on Mac) to run tool
- **Accessible** — `aria-live` on output, `aria-label` on controls, focus-visible
- **Responsive** — split pane collapses to single column on mobile
- **No dependencies** — zero npm packages, zero CDN links, zero fetch() calls
- **Offline-capable** — entire paper works from a local file (except WASM)

## Deploying

### GitHub Pages (recommended)

The included `.github/workflows/pages.yml` deploys automatically on push to `main`.

1. Go to your repo Settings > Pages
2. Set Source to "GitHub Actions"
3. Push to `main` — the workflow validates LRA structure and deploys

### Other Hosts

The paper is a static site. Deploy anywhere that serves HTML:
- Netlify: drag-and-drop the repo folder
- Vercel: connect the repo
- Any web server: just serve the files

## Philosophy

Traditional academic publishing locks research behind paywalls, strips
reproducibility by separating the paper from the code, and relies on
peer reviewers who cannot execute the claims they evaluate.

A Living Research Artifact inverts all three:

| Traditional | LRA |
|------------|-----|
| Paper behind paywall | GPL-3.0: free forever, copyleft protects it |
| Code in separate repo (if at all) | Paper IS the repo |
| Claims reviewed by reading | Claims verified by executing |
| Static PDF | Interactive HTML that runs your tool |

The GPL-3.0 license is deliberate: anyone who forks and modifies this
template must share their modifications under the same terms. The format
propagates openness.

## License

GPL-3.0. See [LICENSE](LICENSE).

This means:
- You CAN use this template for any research paper
- You CAN modify the HTML, CSS, and JS
- You MUST keep the GPL-3.0 license on derivative works
- You MUST share modifications under the same terms
- Your paper content (the text you write) is yours — the GPL applies to
  the template code (HTML structure, CSS, JS), not to your research content
