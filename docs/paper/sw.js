// sw.js — MIRR Paper Service Worker
// Phase 1: Offline cache (paper + WASM compiler)
// Phase 4: JSON-RPC 2.0 protocol (lra.ping, lra.meta, lra.claims, lra.cite, lra.run_tool)
//
// GPL-3.0 — see LICENSE for terms.

var CACHE_NAME = 'mirr-lra-v4';
var ASSETS = [
  'index.html',
  'paper.css',
  'paper.js',
  'lra-client.js',
  'lra-card.svg',
  './demos/mirr_wasm.js',
  './demos/mirr_wasm_bg.wasm'
];

// ── LRA Protocol v1.0 — MIRR metadata ──────────────────────────────

var LRA_META = {
  title: 'MIRR: A Safety-Critical HDL Compiler with Formal Width Inference',
  authors: ['Brandon'],
  date: '2026-03',
  license: 'GPL-3.0',
  version: '0.3.0',
  abstract: 'MIRR is an open-source Rust compiler for safety-critical hardware-software co-design. It compiles temporal guards, guarded reflexes, and LTL safety properties through a 9-stage deterministic pipeline into 9 emission backends. Width inference is backed by 1,077 lines of Rocq proofs.',
  keywords: ['HDL', 'compiler', 'safety-critical', 'formal verification',
             'width inference', 'SystemVerilog', 'FIRRTL', 'WASM',
             'NASA Power-of-10'],
  claims_count: 4,
  capability: 'mirr-compiler',
  formats: ['verilog', 'firrtl', 'rspu', 'sexpr', 'json', 'dot']
};

var LRA_CLAIMS = [
  { id: 'claim-1', text: 'MIRR compiles temporal specifications to correct SystemVerilog, FIRRTL, R-SPU assembly, S-expression IR, JSON netlist, and DOT graph.', evidence_href: '#demo-playground' },
  { id: 'claim-2', text: 'Width inference is sound: no assignment silently truncates a value.', evidence_href: null },
  { id: 'claim-3', text: 'All compiler algorithms are bounded: no unbounded recursion or iteration exists.', evidence_href: '#demo-benchmarks' },
  { id: 'claim-4', text: 'The compiler is safe: #![forbid(unsafe_code)] on every source file.', evidence_href: null }
];

var LRA_CITATION_BIBTEX = '@software{mirr2026,\n  title  = {MIRR: A Safety-Critical HDL Compiler},\n  author = {Brandon},\n  year   = {2026},\n  url    = {https://github.com/brandonfromph/mirr-project},\n  license = {GPL-3.0}\n}';

var LRA_CITATION_APA = 'Brandon. (2026). MIRR: A Safety-Critical HDL Compiler [Computer software]. https://github.com/brandonfromph/mirr-project';

var LRA_CITATION_RIS = 'TY  - COMP\nTI  - MIRR: A Safety-Critical HDL Compiler with Formal Width Inference\nAU  - Brandon\nPY  - 2026\nUR  - https://github.com/brandonfromph/mirr-project\nER  - ';

// ── Cache lifecycle ─────────────────────────────────────────────────

self.addEventListener('install', function(event) {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(function(cache) { return cache.addAll(ASSETS); })
      .then(function() { return self.skipWaiting(); })
  );
});

self.addEventListener('activate', function(event) {
  event.waitUntil(
    caches.keys().then(function(names) {
      return Promise.all(
        names.filter(function(name) { return name !== CACHE_NAME; })
          .map(function(name) { return caches.delete(name); })
      );
    }).then(function() { return self.clients.claim(); })
  );
});

self.addEventListener('fetch', function(event) {
  event.respondWith(
    caches.match(event.request)
      .then(function(cached) { return cached || fetch(event.request); })
  );
});

// ── LRA Protocol v1.0 — JSON-RPC 2.0 dispatch ──────────────────────

// Pending relay map for lra.run_tool responses from page
var _pending = {};

function handleProtocol(event) {
  var data = event.data || {};
  var method = data.method;
  var params = data.params;
  var id = data.id;
  if (!event.source) return;

  if (data.jsonrpc !== '2.0') return;

  // W12: Guard reply/error helpers — do not send responses for notifications (no id)
  var reply = function(result) {
    if (id === undefined || id === null) return;
    event.source.postMessage({ jsonrpc: '2.0', id: id, result: result });
  };
  var error = function(code, message) {
    if (id === undefined || id === null) return;
    event.source.postMessage({ jsonrpc: '2.0', id: id, error: { code: code, message: message } });
  };

  switch (method) {
    case 'lra.ping':
      reply({ status: 'ok', version: '1.0', capability: 'mirr-compiler' });
      break;
    case 'lra.meta':
      reply(LRA_META);
      break;
    case 'lra.claims':
      reply(LRA_CLAIMS);
      break;
    case 'lra.cite': {
      var fmt = (params && params.format) || 'bibtex';
      if (fmt === 'bibtex') reply({ citation: LRA_CITATION_BIBTEX });
      else if (fmt === 'apa') reply({ citation: LRA_CITATION_APA });
      else if (fmt === 'ris') reply({ citation: LRA_CITATION_RIS });
      else error(-32602, 'Unknown citation format: ' + fmt);
      break;
    }
    case 'lra.run_tool':
      relayToPage(event, id, params);
      break;
    default:
      if (method && method.indexOf('lra.') === 0)
        error(-32601, 'Method not found: ' + method);
  }
}

function relayToPage(event, id, params) {
  self.clients.matchAll({ type: 'window' }).then(function(clients) {
    if (clients.length === 0) {
      event.source.postMessage({
        jsonrpc: '2.0', id: id,
        error: { code: -32000, message: 'No active page — WASM compiler not loaded' }
      });
      return;
    }
    var relay_id = 'relay-' + id;
    clients[0].postMessage({
      type: 'lra.run_tool.relay',
      relay_id: relay_id,
      params: params
    });
    _pending[relay_id] = { source: event.source, id: id };
  });
}

self.addEventListener('message', function(event) {
  var data = event.data || {};

  // Handle relay responses from page
  if (data.type === 'lra.run_tool.response') {
    var pending = _pending[data.relay_id];
    if (pending) {
      delete _pending[data.relay_id];
      if (data.error) {
        pending.source.postMessage({
          jsonrpc: '2.0', id: pending.id,
          error: data.error
        });
      } else {
        pending.source.postMessage({
          jsonrpc: '2.0', id: pending.id,
          result: data.result
        });
      }
    }
    return;
  }

  // Handle protocol requests
  handleProtocol(event);
});
