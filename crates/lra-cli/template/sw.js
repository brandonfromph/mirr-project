// sw.js — LRA Service Worker
// Phase 1: Offline cache
// Phase 4: JSON-RPC 2.0 protocol (lra.ping, lra.meta, lra.claims, lra.cite, lra.run_tool)
//
// Apache-2.0 — see LICENSE for terms.

// IMPORTANT: Bump this version string when you change any cached file
var CACHE_NAME = 'lra-v3';
var ASSETS = [
  'index.html',
  'paper.css',
  'paper.js',
  'lra-client.js',
  'lra-card.svg'
];

// ── LRA Protocol v1.0 — metadata constants ──────────────────────────
// REPLACE: Fill in your paper's metadata below.

var LRA_META = {
  title: 'REPLACE: Your Paper Title',
  authors: ['REPLACE: Your Name'],
  date: 'REPLACE: 2026-01',
  license: 'Apache-2.0',
  version: '1.0',
  abstract: 'REPLACE: Your abstract here.',
  keywords: ['REPLACE'],
  claims_count: 0
};

var LRA_CLAIMS = [];

var LRA_CITATION_BIBTEX = 'REPLACE: your bibtex';
var LRA_CITATION_APA = 'REPLACE: your APA citation';

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

  var reply = function(result) {
    event.source.postMessage({ jsonrpc: '2.0', id: id, result: result });
  };
  var error = function(code, message) {
    event.source.postMessage({ jsonrpc: '2.0', id: id, error: { code: code, message: message } });
  };

  switch (method) {
    case 'lra.ping':
      reply({ status: 'ok', version: '1.0' });
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
        error: { code: -32000, message: 'No active page — tool not loaded' }
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
