// paper.js — Living Research Artifact interactive layer
// GPL-3.0 — see LICENSE for terms.
// No external dependencies. No npm. No CDN. No fetch().
//
// This file has two sections:
//   1. FRAMEWORK — protocol bridge, event wiring, service worker (do not modify)
//   2. YOUR TOOL — the regex tester demo (replace with your own tool)

// ═══════════════════════════════════════════════════════════════════════
// FRAMEWORK — Do not modify this section unless you know what you're doing.
// It handles: DOM wiring, keyboard shortcuts, LRA protocol, service worker.
// ═══════════════════════════════════════════════════════════════════════

var MAX_INPUT_BYTES = 65536;
var MAX_BENCHMARK_ITERATIONS = 1000;

// ── DOM references ──────────────────────────────────────────────────

var patternInput = document.getElementById('tool-pattern');
var textInput    = document.getElementById('tool-input');
var outputEl     = document.getElementById('tool-output');
var runBtn       = document.getElementById('run-btn');
var benchBtn     = document.getElementById('bench-btn');
var exampleSelect = document.getElementById('example-select');
var outputLabel  = document.getElementById('output-label');

// ── Event wiring ────────────────────────────────────────────────────

if (runBtn)   runBtn.addEventListener('click', function() { run(); });
if (benchBtn) benchBtn.addEventListener('click', function() { benchmark(); });

if (exampleSelect) {
  exampleSelect.addEventListener('change', function(e) {
    var key = e.target.value;
    if (key && EXAMPLES[key]) {
      if (patternInput) patternInput.value = EXAMPLES[key].pattern;
      if (textInput) textInput.value = EXAMPLES[key].input;
      run();
    }
  });
}

function handleKeyShortcut(e) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
    e.preventDefault();
    run();
  }
}

if (patternInput) patternInput.addEventListener('keydown', handleKeyShortcut);
if (textInput) textInput.addEventListener('keydown', handleKeyShortcut);

// ── runTool interface ───────────────────────────────────────────────
// Both the UI button and the LRA protocol bridge call this function.
// It delegates to your tool's run() function and returns a result string.

function runTool(params) {
  try {
    var input = (params && params.input) || '';
    var result = runToolImpl(params);
    return { ok: result };
  } catch (e) {
    return { error: e.message };
  }
}

// ── LRA Protocol bridge (Phase 4) ──────────────────────────────────
// Service Worker relays run_tool requests here because only the page
// has access to the tool (JS or WASM).

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.addEventListener('message', function(event) {
    var data = event.data || {};
    if (data.type !== 'lra.run_tool.relay') return;
    var result = runTool(data.params);
    if (navigator.serviceWorker.controller) {
      if (result.error) {
        navigator.serviceWorker.controller.postMessage({
          type: 'lra.run_tool.response',
          relay_id: data.relay_id,
          error: { code: -32603, message: result.error }
        });
      } else {
        navigator.serviceWorker.controller.postMessage({
          type: 'lra.run_tool.response',
          relay_id: data.relay_id,
          result: { ok: result.ok }
        });
      }
    }
  });
}

// ── Cross-origin query handler ──────────────────────────────────────
// When another paper embeds us in an iframe, it sends postMessage
// requests. We forward them to our Service Worker for dispatch.

window.addEventListener('message', function(event) {
  var data = event.data;
  if (!data || data.jsonrpc !== '2.0' || !data.method) return;
  if (!navigator.serviceWorker || !navigator.serviceWorker.controller) return;

  navigator.serviceWorker.controller.postMessage(data);

  function onReply(e) {
    if (e.data && e.data.id === data.id) {
      navigator.serviceWorker.removeEventListener('message', onReply);
      event.source.postMessage(e.data, '*');
    }
  }
  navigator.serviceWorker.addEventListener('message', onReply);
});

// ── Service Worker registration ─────────────────────────────────────

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('sw.js').catch(function() {});
}

// ── Boot ────────────────────────────────────────────────────────────

var verEl = document.getElementById('tool-version');
if (verEl) verEl.textContent = '1.0';

// ═══════════════════════════════════════════════════════════════════════
// YOUR TOOL — Replace everything below with your own tool.
// The framework above calls run(), benchmark(), and runToolImpl(params).
// You must define all three functions.
// ═══════════════════════════════════════════════════════════════════════

// ── Examples (swap with your own) ───────────────────────────────────

var EXAMPLES = {
  email: {
    pattern: '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$',
    input: [
      'user@example.com',
      'not-an-email',
      'jane.doe@university.edu',
      'foo@',
      'test@test.co.uk',
      '@missing-local.com',
      'valid+tag@gmail.com',
      'spaces in@address.org'
    ].join('\n')
  },
  url: {
    pattern: 'https?://[^\\s<>"]+',
    input: [
      'Visit https://example.com for details.',
      'No URL on this line.',
      'See http://docs.rs/regex/latest and https://crates.io/crates/regex',
      'Bare domain: example.com (no match)',
      'Secure: https://github.com/user/repo/blob/main/README.md'
    ].join('\n')
  },
  code: {
    pattern: 'function\\s+\\w+\\s*\\(',
    input: [
      'function hello() {',
      '  console.log("hi");',
      '}',
      'const arrow = () => 42;',
      'function processData(input) {',
      '  return input.trim();',
      '}'
    ].join('\n')
  }
};

// ── runToolImpl — called by the framework for protocol requests ─────
// REPLACE: This is the function the protocol bridge calls.
// It receives { input, pattern } and returns a result string.

function runToolImpl(params) {
  var input = (params && params.input) || '';
  var pattern = (params && params.pattern) || '.*';
  var re = new RegExp(pattern, 'g');
  var lines = input.split('\n');
  var matches = 0;
  for (var i = 0; i < lines.length; i++) {
    re.lastIndex = 0;
    var m = lines[i].match(re);
    if (m) matches += m.length;
  }
  return matches + ' matches across ' + lines.length + ' lines';
}

// ── run() — called when the user clicks Run ─────────────────────────
// REPLACE: Swap the body with your own tool invocation.

function run() {
  if (!outputEl) return;
  outputEl.setAttribute('aria-busy', 'true');
  outputEl.classList.remove('error');

  var pattern = patternInput ? patternInput.value : '';
  var source  = textInput ? textInput.value : '';

  if (source.length > MAX_INPUT_BYTES) {
    outputEl.textContent = 'Error: Input too large (' + source.length + ' bytes). Limit: ' + MAX_INPUT_BYTES + '.';
    outputEl.classList.add('error');
    outputEl.setAttribute('aria-busy', 'false');
    return;
  }

  if (pattern.trim() === '') {
    outputEl.textContent = 'Enter a regex pattern above and click Run.';
    outputEl.setAttribute('aria-busy', 'false');
    return;
  }

  var re;
  try {
    re = new RegExp(pattern, 'g');
  } catch (err) {
    outputEl.textContent = 'Invalid regex: ' + err.message;
    outputEl.classList.add('error');
    outputEl.setAttribute('aria-busy', 'false');
    return;
  }

  var lines = source.split('\n');
  var matchCount = 0;
  var results = [];

  for (var i = 0; i < lines.length; i++) {
    var line = lines[i];
    re.lastIndex = 0;
    var m = line.match(re);
    if (m && m.length > 0) {
      matchCount += m.length;
      results.push('Line ' + (i + 1) + ' [' + m.length + ' match' + (m.length > 1 ? 'es' : '') + ']: ' + line);
      for (var j = 0; j < m.length; j++) {
        results.push('  -> "' + m[j] + '"');
      }
    } else {
      results.push('Line ' + (i + 1) + ' [no match]: ' + line);
    }
  }

  var summary =
    '--- Regex Tester Results ---\n' +
    'Pattern: /' + pattern + '/g\n' +
    'Lines tested: ' + lines.length + '\n' +
    'Total matches: ' + matchCount + '\n' +
    '----------------------------\n\n';

  outputEl.textContent = summary + results.join('\n');
  if (outputLabel) outputLabel.textContent = '(' + matchCount + ' match' + (matchCount !== 1 ? 'es' : '') + ')';
  outputEl.setAttribute('aria-busy', 'false');
}

// ── benchmark() — called when the user clicks Benchmark ─────────────
// REPLACE: Adapt this to benchmark your own tool's execution time.

function benchmark() {
  if (!outputEl) return;
  outputEl.setAttribute('aria-busy', 'true');
  outputEl.classList.remove('error');

  var pattern = patternInput ? patternInput.value : '';
  var source  = textInput ? textInput.value : '';

  if (pattern.trim() === '') {
    outputEl.textContent = 'Enter a regex pattern above before benchmarking.';
    outputEl.setAttribute('aria-busy', 'false');
    return;
  }

  if (source.length > MAX_INPUT_BYTES) {
    outputEl.textContent = 'Error: Input too large (' + source.length + ' bytes). Limit: ' + MAX_INPUT_BYTES + '.';
    outputEl.classList.add('error');
    outputEl.setAttribute('aria-busy', 'false');
    return;
  }

  var re;
  try {
    re = new RegExp(pattern, 'g');
  } catch (err) {
    outputEl.textContent = 'Invalid regex: ' + err.message;
    outputEl.classList.add('error');
    outputEl.setAttribute('aria-busy', 'false');
    return;
  }

  var lines = source.split('\n');

  // Warm-up pass
  for (var i = 0; i < lines.length; i++) {
    re.lastIndex = 0;
    lines[i].match(re);
  }

  // Timed iterations
  var start = performance.now();
  for (var iter = 0; iter < MAX_BENCHMARK_ITERATIONS; iter++) {
    for (var j = 0; j < lines.length; j++) {
      re.lastIndex = 0;
      lines[j].match(re);
    }
  }
  var elapsed = performance.now() - start;

  var perIter = elapsed / MAX_BENCHMARK_ITERATIONS;
  var perLine = elapsed / (MAX_BENCHMARK_ITERATIONS * lines.length);

  outputEl.textContent =
    '--- Benchmark Results ---\n' +
    'Pattern: /' + pattern + '/g\n' +
    'Iterations: ' + MAX_BENCHMARK_ITERATIONS + '\n' +
    'Lines per iteration: ' + lines.length + '\n' +
    'Total time: ' + elapsed.toFixed(2) + ' ms\n' +
    'Per iteration: ' + perIter.toFixed(4) + ' ms\n' +
    'Per line: ' + perLine.toFixed(4) + ' ms\n' +
    '-------------------------';

  if (outputLabel) outputLabel.textContent = '(benchmark)';
  outputEl.setAttribute('aria-busy', 'false');
}

// ── Boot message ────────────────────────────────────────────────────

if (outputEl) {
  outputEl.textContent =
    'Regex Tester ready.\n\n' +
    'Enter a pattern and test strings, then click Run (or Ctrl+Enter).\n' +
    'Or select an example from the dropdown above.';
}
