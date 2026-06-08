// sw.js — LRA Service Worker
// Phase 1: Offline cache
// Phase 4: JSON-RPC 2.0 protocol (lra.ping, lra.meta, lra.claims, lra.cite, lra.run_tool)
// Phase 5: Discovery + dependencies (lra.depends)
// Phase 6: Autonomous node (rate limiting, headless capability, graceful degradation)
// Phase 7: Live peer review (verify_claim, challenge, verification_log)
// Phase 8: Self-healing knowledge graph (dep_versions, notify, notifications)
// Phase 9: Peer-to-peer research protocol (identity, reputation, peers)
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

// Phase 6: Install timestamp for uptime tracking
var _installTime = Date.now();

// Phase 6: Rate limiting (NASA Power-of-10 — bounded counters)
var MAX_REQUESTS_PER_MINUTE = 60;
var _requestCount = 0;
var _requestWindowStart = Date.now();

// Phase 6: Pending relay cleanup (60s TTL — 2x client timeout)
var MAX_PENDING_TTL = 60000;
var MAX_PENDING_ENTRIES = 100;

// Phase 6: Headless capability list (methods that work without an open tab)
var HEADLESS_METHODS = ['lra.ping', 'lra.meta', 'lra.claims', 'lra.cite', 'lra.depends',
                        'lra.verification_log', 'lra.dep_versions', 'lra.notify',
                        'lra.notifications', 'lra.identity', 'lra.reputation', 'lra.peers'];

// Phase 7: Verification log (bounded, in-memory, append-only)
var MAX_VERIFICATION_LOG = 1000;
var _verificationLog = [];

// Phase 7: Challenge log (bounded, in-memory)
var MAX_CHALLENGE_LOG = 100;
var _challenges = [];

// Phase 7: Bounded claim search
var MAX_CLAIMS = 100;

// Phase 8: Notification log (bounded, in-memory)
var MAX_NOTIFICATIONS = 100;
var _notifications = [];

// Phase 9: Node identity (REPLACE with your Ed25519 pubkey or leave null for anonymous)
var LRA_IDENTITY = null;

// Phase 9: Known peers (REPLACE with URLs of other LRA papers you know)
var MAX_PEERS = 50;
var LRA_PEERS = [];

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
  claims_count: 3
};

var LRA_CLAIMS = [
  { id: 'claim-1', text: 'REPLACE: First verifiable claim', evidence_href: '#demo-1' },
  { id: 'claim-2', text: 'REPLACE: Second claim', evidence_href: null },
  { id: 'claim-3', text: 'REPLACE: Third claim', evidence_href: null }
];

var LRA_CITATION_BIBTEX = 'REPLACE: your bibtex';
var LRA_CITATION_APA = 'REPLACE: your APA citation';

var LRA_CITATION_RIS = 'TY  - COMP\nTI  - REPLACE: Your Paper Title\nAU  - REPLACE: Your Name\nPY  - REPLACE: Year\nUR  - REPLACE: URL\nER  - ';

// REPLACE: Add sha256 hashes of papers this work depends on
var LRA_DEPENDS = [];

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

  // Phase 6: Rate limiting
  var now = Date.now();
  if (now - _requestWindowStart > 60000) {
    _requestCount = 0;
    _requestWindowStart = now;
  }
  _requestCount++;
  if (_requestCount > MAX_REQUESTS_PER_MINUTE) {
    if (id !== undefined && id !== null) {
      event.source.postMessage({
        jsonrpc: '2.0', id: id,
        error: { code: -32000, message: 'Rate limit exceeded (max ' + MAX_REQUESTS_PER_MINUTE + '/min)' }
      });
    }
    return;
  }

  // Guard reply/error helpers — do not send responses for notifications (no id)
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
      reply({
        status: 'ok',
        version: '1.0',
        uptime_ms: now - _installTime,
        headless_methods: HEADLESS_METHODS,
        tool_requires_tab: true
      });
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
    case 'lra.depends': {
      // Phase 8: Backward-compat — always return flat hash strings
      var dep_hashes = [];
      var dk = 0;
      while (dk < LRA_DEPENDS.length && dk < MAX_CLAIMS) {
        if (typeof LRA_DEPENDS[dk] === 'string') dep_hashes.push(LRA_DEPENDS[dk]);
        else if (LRA_DEPENDS[dk].hash) dep_hashes.push(LRA_DEPENDS[dk].hash);
        dk++;
      }
      reply(dep_hashes);
      break;
    }
    case 'lra.dep_versions':
      reply(LRA_DEPENDS);
      break;
    case 'lra.notify': {
      var n_source_hash = params && params.source_hash;
      if (!n_source_hash) {
        error(-32602, 'notify requires source_hash');
        break;
      }
      var is_dependency = false;
      var di = 0;
      while (di < LRA_DEPENDS.length && di < MAX_CLAIMS) {
        var dep_hash = typeof LRA_DEPENDS[di] === 'string'
          ? LRA_DEPENDS[di] : (LRA_DEPENDS[di].hash || '');
        if (dep_hash === n_source_hash) { is_dependency = true; break; }
        di++;
      }
      if (_notifications.length < MAX_NOTIFICATIONS) {
        _notifications.push({
          source_hash: n_source_hash,
          new_version: (params && params.new_version) || null,
          old_version: (params && params.old_version) || null,
          is_dependency: is_dependency,
          timestamp: Date.now()
        });
      }
      reply({ status: 'notification_received', is_dependency: is_dependency });
      break;
    }
    case 'lra.notifications':
      reply(_notifications);
      break;
    case 'lra.verify_claim': {
      var vc_claim_id = params && params.claim_id;
      var vc_input = params && params.input;
      if (!vc_claim_id || !vc_input) {
        error(-32602, 'verify_claim requires claim_id and input');
        break;
      }
      var vc_claim = null;
      var ci = 0;
      while (ci < LRA_CLAIMS.length && ci < MAX_CLAIMS) {
        if (LRA_CLAIMS[ci].id === vc_claim_id) { vc_claim = LRA_CLAIMS[ci]; break; }
        ci++;
      }
      if (!vc_claim) {
        error(-32602, 'Unknown claim: ' + vc_claim_id);
        break;
      }
      if (!vc_claim.evidence_href) {
        reply({ claim_id: vc_claim_id, status: 'no_executable_evidence', claim_text: vc_claim.text });
        break;
      }
      relayToPage(event, id, { input: vc_input, _verify_claim_id: vc_claim_id });
      break;
    }
    case 'lra.challenge': {
      var ch_claim_id = params && params.claim_id;
      if (!ch_claim_id) {
        error(-32602, 'challenge requires claim_id');
        break;
      }
      if (_challenges.length < MAX_CHALLENGE_LOG) {
        _challenges.push({
          claim_id: ch_claim_id,
          input: (params && params.input) || null,
          expected: (params && params.expected) || null,
          actual: (params && params.actual) || null,
          verifier_hash: (params && params.verifier_hash) || null,
          timestamp: Date.now()
        });
      }
      reply({ status: 'challenge_recorded', claim_id: ch_claim_id });
      break;
    }
    case 'lra.verification_log':
      reply(_verificationLog);
      break;
    case 'lra.identity':
      if (LRA_IDENTITY) {
        reply(LRA_IDENTITY);
      } else {
        reply({ pubkey: null, status: 'anonymous' });
      }
      break;
    case 'lra.reputation': {
      var total_v = _verificationLog.length;
      var verified_c = 0;
      var failed_c = 0;
      var ri = 0;
      while (ri < _verificationLog.length && ri < MAX_VERIFICATION_LOG) {
        if (_verificationLog[ri].status === 'verified') verified_c++;
        else failed_c++;
        ri++;
      }
      reply({
        total_verifications: total_v,
        verified: verified_c,
        failed: failed_c,
        challenges: _challenges.length,
        score: total_v > 0 ? Math.round((verified_c / total_v) * 100) : null,
        uptime_ms: Date.now() - _installTime
      });
      break;
    }
    case 'lra.peers':
      reply(LRA_PEERS);
      break;
    default:
      if (method && method.indexOf('lra.') === 0)
        error(-32601, 'Method not found: ' + method);
  }
}

function relayToPage(event, id, params) {
  var now = Date.now();
  self.clients.matchAll({ type: 'window' }).then(function(clients) {
    if (clients.length === 0) {
      // Phase 6: Graceful degradation — tell caller the node is alive but tool needs a tab
      event.source.postMessage({
        jsonrpc: '2.0', id: id,
        error: {
          code: -32000,
          message: 'Tool requires active browser tab',
          data: { headless: true, retry: true, methods_available: HEADLESS_METHODS }
        }
      });
      return;
    }
    var relay_id = 'relay-' + id;
    clients[0].postMessage({
      type: 'lra.run_tool.relay',
      relay_id: relay_id,
      params: params
    });
    _pending[relay_id] = {
      source: event.source, id: id, timestamp: now,
      verify_claim_id: (params && params._verify_claim_id) || null,
      verify_input: (params && params.input) || null
    };

    // Phase 6: Cleanup stale pending entries (bounded iteration)
    var pendingKeys = Object.keys(_pending);
    var k = 0;
    while (k < pendingKeys.length && k < MAX_PENDING_ENTRIES) {
      var entry = _pending[pendingKeys[k]];
      if (entry.timestamp && (now - entry.timestamp) > MAX_PENDING_TTL) {
        delete _pending[pendingKeys[k]];
      }
      k++;
    }
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
      // Phase 7: Log verification result if this was a verify_claim relay
      if (pending.verify_claim_id && _verificationLog.length < MAX_VERIFICATION_LOG) {
        var vInput = (pending.verify_input || '');
        crypto.subtle.digest('SHA-256', new TextEncoder().encode(vInput))
          .then(function(hashBuffer) {
            var hashArray = new Uint8Array(hashBuffer);
            var hashHex = '';
            var hi = 0;
            while (hi < hashArray.length && hi < 32) {
              hashHex += ('00' + hashArray[hi].toString(16)).slice(-2);
              hi++;
            }
            if (_verificationLog.length < MAX_VERIFICATION_LOG) {
              _verificationLog.push({
                claim_id: pending.verify_claim_id,
                input_hash: 'sha256:' + hashHex,
                status: data.error ? 'failed' : 'verified',
                timestamp: Date.now()
              });
            }
          });
      }
    }
    return;
  }

  // Handle protocol requests
  handleProtocol(event);
});
