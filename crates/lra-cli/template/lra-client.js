// lra-client.js — Query other Living Research Artifacts
// Usage:
//   var paper = new LRAClient('https://example.github.io/paper/');
//   paper.meta().then(function(m) { console.log(m.title); });
//   paper.runTool({ input: '...', format: 'json' }).then(console.log);
//   paper.close();
//
// GPL-3.0 — see LICENSE for terms.

/* exported LRAClient */

// Architecture: LRAClient creates a hidden <iframe> pointing at the target
// LRA paper's URL. Communication uses JSON-RPC 2.0 over window.postMessage.
// The target paper's Service Worker handles protocol dispatch, relaying
// lra.run_tool calls to the page's tool (WASM module or JS function).

function LRAClient(url) {
  this._url = url;
  this._iframe = null;
  this._ready = false;
  this._pending = {};
  this._nextId = 1;
  this._onMessage = this._handleMessage.bind(this);
  window.addEventListener('message', this._onMessage);
}

// Lazy-load the iframe on first request. The iframe loads the remote paper,
// which registers its Service Worker, enabling cross-origin LRA queries.
LRAClient.prototype._ensureIframe = function() {
  if (this._iframe) return Promise.resolve();
  var self = this;
  return new Promise(function(resolve) {
    var f = document.createElement('iframe');
    f.style.display = 'none';
    f.src = self._url;
    f.addEventListener('load', function() {
      self._ready = true;
      resolve();
    });
    document.body.appendChild(f);
    self._iframe = f;
  });
};

// Send a JSON-RPC 2.0 request via postMessage to the iframe's window.
// Responses are matched by numeric ID. Timeout after 30s to prevent leaks.
LRAClient.prototype._send = function(method, params) {
  var self = this;
  return this._ensureIframe().then(function() {
    return new Promise(function(resolve, reject) {
      var id = self._nextId++;
      self._pending[id] = { resolve: resolve, reject: reject };
      self._iframe.contentWindow.postMessage(
        { jsonrpc: '2.0', method: method, params: params, id: id }, '*'
      );
      // Timeout after 30 seconds
      setTimeout(function() {
        if (self._pending[id]) {
          delete self._pending[id];
          reject(new Error('LRA query timeout: ' + method));
        }
      }, 30000);
    });
  });
};

// Receive JSON-RPC 2.0 responses from the iframe and resolve pending promises.
LRAClient.prototype._handleMessage = function(event) {
  var data = event.data;
  if (!data || !data.jsonrpc) return;
  var p = this._pending[data.id];
  if (!p) return;
  delete this._pending[data.id];
  if (data.error) p.reject(new Error(data.error.message));
  else p.resolve(data.result);
};

LRAClient.prototype.ping = function() {
  return this._send('lra.ping');
};

LRAClient.prototype.meta = function() {
  return this._send('lra.meta');
};

LRAClient.prototype.claims = function() {
  return this._send('lra.claims');
};

LRAClient.prototype.cite = function(format) {
  return this._send('lra.cite', { format: format || 'bibtex' });
};

LRAClient.prototype.runTool = function(params) {
  return this._send('lra.run_tool', params);
};

LRAClient.prototype.close = function() {
  window.removeEventListener('message', this._onMessage);
  if (this._iframe) {
    this._iframe.remove();
    this._iframe = null;
  }
};
