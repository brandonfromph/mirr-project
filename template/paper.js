// paper.js — Living Research Artifact interactive layer
// Replace the WASM import below with your own module.
// See README.md for setup instructions.
//
// GPL-3.0 — see LICENSE for terms.
// No external dependencies. No npm. No CDN.

// ── Step 1: Import your WASM module ──────────────────────────────────
// Uncomment and replace with your actual WASM imports:
//
// import init, { your_function } from './wasm/your_tool.js';
//
// If you don't have a WASM module yet, the paper works as a static
// document — all text, tables, and references are readable without JS.

const MAX_INPUT_BYTES = 65_536;
let toolReady = false;

// ── Step 2: Initialize your tool ─────────────────────────────────────

async function initTool() {
  const output = document.getElementById('tool-output');
  try {
    // Uncomment when you have a WASM module:
    // await init();
    // toolReady = true;
    // output.textContent = '// Tool ready. Enter input or load an example.';

    // Placeholder until WASM is configured:
    output.textContent =
      '// No WASM module configured.\n' +
      '// See README.md to connect your tool.\n' +
      '//\n' +
      '// This paper works without WASM —\n' +
      '// all text and tables are static HTML.';
  } catch (err) {
    output.textContent = 'Failed to load WASM module: ' + err.message;
    output.classList.add('error');
  }
}

// ── Step 3: Define your run/compile function ─────────────────────────

function run() {
  if (!toolReady) return;

  const source = document.getElementById('tool-input').value;
  const output = document.getElementById('tool-output');
  output.setAttribute('aria-busy', 'true');

  if (source.length > MAX_INPUT_BYTES) {
    output.textContent =
      `Input too large (${source.length} bytes). Limit: ${MAX_INPUT_BYTES}.`;
    output.classList.add('error');
    output.setAttribute('aria-busy', 'false');
    return;
  }

  // Replace this with your actual tool invocation:
  // const result = JSON.parse(your_function(source));
  //
  // if (result.ok !== undefined) {
  //   output.textContent = result.ok;
  //   output.classList.remove('error');
  // } else if (result.err !== undefined) {
  //   output.textContent = result.err;
  //   output.classList.add('error');
  // }

  output.textContent = '// Tool not configured. See README.md.';
  output.setAttribute('aria-busy', 'false');
}

// ── Step 4: Embedded examples ────────────────────────────────────────
// Replace these with examples relevant to your paper.

const EXAMPLES = {
  example1: '// Replace with your first example input',
  example2: '// Replace with your second example input'
};

// ── Event wiring ─────────────────────────────────────────────────────

document.getElementById('run-btn')
  .addEventListener('click', run);

document.getElementById('example-select')
  .addEventListener('change', e => {
    const key = e.target.value;
    if (key && EXAMPLES[key]) {
      document.getElementById('tool-input').value = EXAMPLES[key];
      run();
    }
  });

// Keyboard shortcut: Ctrl+Enter or Cmd+Enter to run
document.getElementById('tool-input')
  .addEventListener('keydown', e => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      run();
    }
  });

// ── Boot ─────────────────────────────────────────────────────────────

initTool();
