// paper.js — Interactive demo layer for MIRR Living Research Artifact
// No external dependencies. No npm. No CDN.
// GPL-3.0 — same license as the compiler.

// Must match MAX_SOURCE_BYTES in crates/mirr-wasm/src/lib.rs
const MAX_SOURCE_BYTES = 65_536;

// WASM exports — populated by dynamic import so syntax highlighting
// survives if the WASM shim is missing or fails to load.
var COMPILERS = {};
let wasmReady = false;

// Embedded examples — avoids fetch() dependency
const EXAMPLES = {
  tmr: `module tmr_sensor_fusion {
    signal sensor_a:     in u16;
    signal sensor_b:     in u16;
    signal sensor_c:     in u16;
    signal sensor_a_ok:  in bool;
    signal voted_value:  out u16;
    signal fault_flag:   out bool;

    guard a_healthy {
        when sensor_a_ok
        for 1 cycles;
    }

    guard a_sick {
        when !sensor_a_ok
        for 8 cycles;
    }

    reflex vote_a {
        on a_healthy {
            voted_value = sensor_a;
        }
    }

    reflex flag_fault {
        on a_sick {
            fault_flag = true;
        }
    }

    property no_spurious_fault {
        always (fault_flag -> !sensor_a_ok);
    }
}`,

  flight: `module flight_controller {
    signal altitude:     in u32;
    signal airspeed:     in u16;
    signal pitch_angle:  in u16;
    signal throttle_cut: out bool;
    signal terrain_warn: out bool;
    signal stabilise:    out bool;

    guard altitude_low {
        when altitude < 500
        for 10 cycles;
    }

    guard overspeed {
        when airspeed > 340
        for 5 cycles;
    }

    guard excessive_pitch {
        when pitch_angle > 30
        for 8 cycles;
    }

    reflex terrain_alert {
        on altitude_low {
            terrain_warn = true;
        }
    }

    reflex cut_throttle {
        on overspeed {
            throttle_cut = true;
        }
    }

    reflex auto_stabilise {
        on excessive_pitch {
            stabilise = true;
        }
    }

    property speed_bounded {
        always (airspeed < 400);
    }
}`,

  respirator: `module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for 1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}`
};

// ── CodeMirror MIRR mode + editor init ────────────────────────────────

var cmEditor = null;

function getSource() {
  if (cmEditor) return cmEditor.getValue();
  var ta = document.getElementById('mirr-source');
  return ta ? ta.value : '';
}

function setSource(text) {
  if (cmEditor) { cmEditor.setValue(text); return; }
  var ta = document.getElementById('mirr-source');
  if (ta) ta.value = text;
}

(function initCodeMirror() {
  if (typeof CodeMirror === 'undefined') return;

  // Define MIRR language mode
  CodeMirror.defineMode('mirr', function() {
    var signalKw = /^(signal|input|output|wire|reg|assign)\b/;
    var guardKw  = /^(guard|when|cycles|for)\b/;
    var reflexKw = /^(reflex|on)\b/;
    var generalKw = /^(module|always|temporal|require|ensure|if|else|let|fn|struct|enum|match|return|property|pattern|prev|use)\b/;
    var dirs     = /^(in|out|internal)\b/;
    var types    = /^(u[0-9]+|i[0-9]+|bool|bit|clock|reset)\b/;
    var bools    = /^(true|false)\b/;

    return {
      startState: function() { return {}; },
      token: function(stream) {
        // Comments
        if (stream.match('//')) { stream.skipToEnd(); return 'comment'; }
        // Whitespace
        if (stream.eatSpace()) return null;
        // Annotations
        if (stream.match(/@[a-zA-Z_]\w*/)) return 'meta';
        // Tags
        if (stream.match(/#[a-zA-Z_]\w*/)) return 'tag';
        // Numbers
        if (stream.match(/0x[0-9a-fA-F_]+/) || stream.match(/0b[01_]+/) || stream.match(/0o[0-7_]+/) || stream.match(/[0-9][0-9_]*/)) return 'number';
        // Identifiers and keywords
        if (stream.match(/[a-zA-Z_]\w*/)) {
          var w = stream.current();
          if (signalKw.test(w))  return 'keyword';
          if (guardKw.test(w))   return 'def';
          if (reflexKw.test(w))  return 'builtin';
          if (generalKw.test(w)) return 'keyword';
          if (dirs.test(w))      return 'qualifier';
          if (types.test(w))     return 'type';
          if (bools.test(w))     return 'atom';
          return 'variable';
        }
        // Operators
        if (stream.match(/[+\-*=<>!&|^~%]+/)) return 'operator';
        // Braces / parens
        if (stream.match(/[{}()\[\];:,]/)) return 'punctuation';
        // Advance one char if nothing matched
        stream.next();
        return null;
      }
    };
  });

  var textarea = document.getElementById('mirr-source');
  if (!textarea) return;

  cmEditor = CodeMirror.fromTextArea(textarea, {
    mode: 'mirr',
    theme: 'material-darker',
    lineNumbers: true,
    matchBrackets: false,
    indentUnit: 4,
    tabSize: 4,
    indentWithTabs: false,
    lineWrapping: true,
    viewportMargin: Infinity,
    extraKeys: {
      'Ctrl-Enter': function() { compile(); },
      'Cmd-Enter':  function() { compile(); }
    }
  });
  cmEditor.setSize(null, 350);
})();

var compile_pipeline_stages, proof_status, simulate_rspu, simulate_mapek, mirr_version, simulate_waveform, compile_graph_data;

async function initWasm() {
  try {
    var wasm = await import('./demos/mirr_wasm.js');
    await wasm.default();
    COMPILERS = {
      verilog: wasm.compile_verilog,
      firrtl:  wasm.compile_firrtl,
      rspu:    wasm.compile_rspu,
      sexpr:   wasm.compile_sexpr,
      json:    wasm.infer_widths,
      dot:     wasm.compile_dot
    };
    compile_pipeline_stages = wasm.compile_pipeline_stages;
    proof_status = wasm.proof_status;
    simulate_rspu = wasm.simulate_rspu;
    simulate_mapek = wasm.simulate_mapek;
    mirr_version = wasm.mirr_version;
    simulate_waveform = wasm.simulate_waveform;
    compile_graph_data = wasm.compile_graph_data;
    wasmReady = true;
    document.getElementById('compiler-output').textContent =
      '// Compiler ready. Type MIRR source or load an example.';
    var vResult = JSON.parse(mirr_version());
    if (vResult.ok) {
      document.querySelectorAll('.mirr-version')
        .forEach(function(el) { el.textContent = vResult.ok; });
    }
  } catch (err) {
    document.getElementById('compiler-output').textContent =
      'Compiler WASM not available — syntax highlighting still active.';
  }
}

function compile() {
  if (!wasmReady) return;

  var source = getSource();
  var format = document.getElementById('emit-format').value;
  var output = document.getElementById('compiler-output');
  output.setAttribute('aria-busy', 'true');
  var label  = document.getElementById('output-label');

  if (source.length > MAX_SOURCE_BYTES) {
    output.textContent =
      'Source too large (' + source.length + ' bytes). Limit is ' + MAX_SOURCE_BYTES + ' bytes.';
    output.classList.add('error');
    output.setAttribute('aria-busy', 'false');
    return;
  }

  label.textContent = '(' + format + ')';

  var compiler = COMPILERS[format];
  if (!compiler) {
    output.setAttribute('aria-busy', 'false');
    return;
  }

  try {
    var result = JSON.parse(compiler(source));

    if (result.ok !== undefined) {
      output.textContent = result.ok;
      output.classList.remove('error');
    } else if (result.err !== undefined) {
      output.textContent = result.err;
      output.classList.add('error');
    }
  } catch (e) {
    output.textContent = 'Compilation error: ' + (e.message || e);
    output.classList.add('error');
  } finally {
    output.setAttribute('aria-busy', 'false');
  }
}

async function runBenchmarks() {
  if (!wasmReady) return;

  var btn = document.getElementById('bench-btn');
  var tbody = document.getElementById('benchmark-rows');
  btn.disabled = true;
  btn.textContent = 'Running...';
  while (tbody.firstChild) tbody.removeChild(tbody.firstChild);

  var formats = ['verilog', 'firrtl', 'rspu', 'sexpr', 'json', 'dot'];
  var source = EXAMPLES.tmr;

  for (var fi = 0; fi < formats.length; fi++) {
    var fmt = formats[fi];
    var compiler = COMPILERS[fmt];
    var elapsed, lines, isError;
    try {
      var start = performance.now();
      var raw = compiler(source);
      elapsed = (performance.now() - start).toFixed(2);
      var result = JSON.parse(raw);
      lines = result.ok ? result.ok.split('\n').length : 0;
      isError = !!result.err;
    } catch (err) {
      elapsed = 'ERROR';
      lines = 0;
      isError = true;
    }

    var row = document.createElement('tr');
    var tdFmt = document.createElement('td');
    tdFmt.textContent = fmt;
    var tdTime = document.createElement('td');
    tdTime.textContent = elapsed;
    var tdLines = document.createElement('td');
    tdLines.textContent = lines;
    row.appendChild(tdFmt);
    row.appendChild(tdTime);
    row.appendChild(tdLines);
    if (isError) {
      row.classList.add('error');
    }
    tbody.appendChild(row);

    // Yield to browser between targets so UI stays responsive
    await new Promise(function(r) { setTimeout(r, 0); });
  }

  btn.disabled = false;
  btn.textContent = 'Run Benchmarks';
}

function handlePipelineViz(source) {
  var STAGE_NAMES = ['parse', 'validate', 'expand', 'typecheck', 'simplify', 'width_infer', 'temporal_lower', 'emit'];
  var STAGE_KEYS = ['parsed', 'validated', 'expanded', 'typechecked', 'simplified', 'width_inferred', 'temporal_lowered', 'emitted'];
  try {
    var result = compile_pipeline_stages(source);
    var raw = JSON.parse(result);
    if (raw.ok !== undefined) {
      var boolMap = raw.ok;
      var stages = [];
      for (var i = 0; i < STAGE_NAMES.length && i < 20; i++) {
        var passed = boolMap[STAGE_KEYS[i]];
        stages.push({ name: STAGE_NAMES[i], output: passed ? 'passed' : 'failed' });
      }
      return { ok: stages };
    }
    return raw;
  } catch (e) {
    return { error: e.message };
  }
}

function handleProofStatus() {
  try {
    var result = proof_status();
    return JSON.parse(result);
  } catch (e) {
    return { error: e.message };
  }
}

function handleRspuSim(source) {
  try {
    var result = simulate_rspu(source);
    return JSON.parse(result);
  } catch (e) {
    return { error: e.message };
  }
}

function handleMapekSim(source, ticks) {
  try {
    var result = simulate_mapek(source, ticks);
    return JSON.parse(result);
  } catch (e) {
    return { error: e.message };
  }
}

// Wire up controls
document.getElementById('compile-btn')
  .addEventListener('click', compile);

document.getElementById('example-select')
  .addEventListener('change', function(e) {
    var key = e.target.value;
    if (key && EXAMPLES[key]) {
      setSource(EXAMPLES[key]);
      compile();
    }
  });

document.getElementById('emit-format')
  .addEventListener('change', compile);

document.getElementById('bench-btn')
  .addEventListener('click', runBenchmarks);

// Pipeline Visualization button
document.getElementById('pipeline-viz-btn')
  .addEventListener('click', function() {
    if (!wasmReady) return;
    var source = getSource();
    var output = document.getElementById('pipeline-viz-output');
    var data = handlePipelineViz(source);
    output.textContent = '';
    if (data.error) {
      output.textContent = 'Error: ' + data.error;
      output.classList.add('error');
    } else if (data.ok) {
      var stages = data.ok;
      for (var i = 0; i < stages.length && i < 20; i++) {
        var stageEl = document.createElement('div');
        stageEl.className = 'pipeline-stage';
        var nameEl = document.createElement('strong');
        nameEl.textContent = stages[i].name || ('Stage ' + (i + 1));
        stageEl.appendChild(nameEl);
        if (stages[i].output) {
          var preEl = document.createElement('pre');
          preEl.textContent = stages[i].output.substring(0, 500);
          stageEl.appendChild(preEl);
        }
        output.appendChild(stageEl);
        if (i < stages.length - 1) {
          var arrow = document.createElement('div');
          arrow.className = 'pipeline-arrow';
          arrow.textContent = '\u2193';
          output.appendChild(arrow);
        }
      }
      output.classList.remove('error');
    } else {
      output.textContent = JSON.stringify(data, null, 2);
    }
  });

// Proof Dashboard button
document.getElementById('proof-dash-btn')
  .addEventListener('click', function() {
    if (!wasmReady) return;
    var output = document.getElementById('proof-dash-output');
    var data = handleProofStatus();
    output.textContent = '';
    if (data.error) {
      output.textContent = 'Error: ' + data.error;
      output.classList.add('error');
    } else if (data.ok) {
      var proofs = data.ok;
      var headerEl = document.createElement('div');
      headerEl.className = 'proof-dash-header';
      headerEl.textContent = 'Build-time snapshot \u2014 ' + proofs.length + ' proofs across ' + (data.proof_files || 12) + ' files (' + (data.mechanized || 52) + '/' + proofs.length + ' mechanized, ' + (data.admitted || 3) + ' admitted)';
      output.appendChild(headerEl);
      for (var i = 0; i < proofs.length && i < 100; i++) {
        var status = proofs[i].status || 'unknown';
        var itemEl = document.createElement('div');
        itemEl.className = 'proof-item';
        var badge = document.createElement('span');
        badge.className = status === 'Proven' ? 'proof-badge proven' : 'proof-badge admitted';
        badge.textContent = status;
        itemEl.appendChild(badge);
        itemEl.appendChild(document.createTextNode(' '));
        var nameEl = document.createElement('strong');
        nameEl.textContent = proofs[i].name || '';
        itemEl.appendChild(nameEl);
        if (proofs[i].file) {
          var fileEl = document.createElement('small');
          var kindLabel = proofs[i].kind ? proofs[i].kind + ' in ' : '';
          fileEl.textContent = ' (' + kindLabel + proofs[i].file + ')';
          itemEl.appendChild(fileEl);
        }
        output.appendChild(itemEl);
      }
      output.classList.remove('error');
    } else {
      output.textContent = JSON.stringify(data, null, 2);
    }
  });

// Proof Live Status button (theorems section)
var proofLiveBtn = document.getElementById('proof-live-btn');
if (proofLiveBtn) {
  proofLiveBtn.addEventListener('click', function() {
    var output = document.getElementById('proof-live-output');
    if (!output) return;
    if (!wasmReady) {
      output.textContent = 'WASM not loaded \u2014 proof data shown is from build-time snapshot.';
      return;
    }
    var data = handleProofStatus();
    if (data && data.ok) {
      var proven = 0;
      var admitted = 0;
      for (var i = 0; i < data.ok.length && i < 100; i++) {
        if (data.ok[i].status === 'Proven') proven++;
        else admitted++;
      }
      output.innerHTML = '<p>Build-time snapshot: ' + proven + '/' + (proven + admitted) + ' mechanized (' + (100 * proven / (proven + admitted)).toFixed(1) + '%). ' + admitted + ' Admitted remain.</p>';
    } else if (data && data.error) {
      output.textContent = 'Error: ' + data.error;
    }
  });
}

// R-SPU Simulation button
document.getElementById('rspu-sim-btn')
  .addEventListener('click', function() {
    if (!wasmReady) return;
    var source = getSource();
    var output = document.getElementById('rspu-sim-output');
    var data = handleRspuSim(source);
    if (data.error) {
      output.textContent = 'Error: ' + data.error;
      output.classList.add('error');
    } else if (data.ok) {
      output.textContent = typeof data.ok === 'string' ? data.ok : JSON.stringify(data.ok, null, 2);
      output.classList.remove('error');
    } else {
      output.textContent = JSON.stringify(data, null, 2);
      output.classList.remove('error');
    }
  });

// MAPE-K Simulation button
document.getElementById('mapek-sim-btn')
  .addEventListener('click', function() {
    if (!wasmReady) return;
    var ticks = parseInt(document.getElementById('mapek-ticks').value, 10) || 100;
    var output = document.getElementById('mapek-sim-output');
    var data = handleMapekSim(getSource(), ticks);
    if (data.error) {
      output.textContent = 'Error: ' + data.error;
      output.classList.add('error');
    } else if (data.ok) {
      output.textContent = typeof data.ok === 'string' ? data.ok : JSON.stringify(data.ok, null, 2);
      output.classList.remove('error');
    } else {
      output.textContent = JSON.stringify(data, null, 2);
      output.classList.remove('error');
    }
  });

// Keyboard shortcut fallback: Ctrl+Enter (when CodeMirror is not active)
if (!cmEditor) {
  document.getElementById('mirr-source')
    .addEventListener('keydown', function(e) {
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        e.preventDefault();
        compile();
      }
    });
}

// Boot WASM (non-blocking — page works without it)
initWasm();

// ── LRA Protocol bridge (Phase 4) ──────────────────────────────────
// Service Worker relays lra.run_tool requests here because only the
// page has access to the WASM module.

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.addEventListener('message', function(event) {
    var data = event.data || {};
    if (data.type !== 'lra.run_tool.relay') return;
    var sw = navigator.serviceWorker.controller;
    if (!sw) return;
    if (!wasmReady) {
      sw.postMessage({
        type: 'lra.run_tool.response',
        relay_id: data.relay_id,
        error: { code: -32000, message: 'WASM compiler not loaded yet' }
      });
      return;
    }
    var input = (data.params && data.params.input) || '';
    var format = (data.params && data.params.format) || 'verilog';
    var fn = COMPILERS[format];
    if (!fn) {
      sw.postMessage({
        type: 'lra.run_tool.response',
        relay_id: data.relay_id,
        error: { code: -32602, message: 'Unknown format: ' + format }
      });
      return;
    }
    try {
      var raw = fn(input);
      var result = JSON.parse(raw);
      sw.postMessage({
        type: 'lra.run_tool.response',
        relay_id: data.relay_id,
        result: result
      });
    } catch (e) {
      sw.postMessage({
        type: 'lra.run_tool.response',
        relay_id: data.relay_id,
        error: { code: -32603, message: e.message }
      });
    }
  });
}

// ── LRA Protocol: cross-origin query handler ───────────────────────
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

// Register Service Worker for offline support + protocol (HTTPS/localhost only)
if ('serviceWorker' in navigator && location.protocol !== 'file:') {
  navigator.serviceWorker.register('sw.js').catch(function() {});
}

// ── HTML utilities ──────────────────────────────────────────────────

function escapeHtml(s) {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

// ── Waveform Renderer (Campaign B3) ──

function renderWaveform(containerId, traceJson) {
    const container = document.getElementById(containerId);
    if (!container) return;
    container.innerHTML = '';

    let trace;
    try {
        trace = typeof traceJson === 'string' ? JSON.parse(traceJson) : traceJson;
    } catch (e) {
        container.innerHTML = '<pre class="viz-error">Invalid waveform data</pre>';
        return;
    }

    const signals = trace.signals || [];
    const totalCycles = Math.min(trace.total_cycles || 32, 1024);

    const LANE_HEIGHT = 40;
    const LANE_GAP = 10;
    const NAME_WIDTH = 120;
    const CYCLE_WIDTH = 30;

    const svgHeight = signals.length * (LANE_HEIGHT + LANE_GAP) + 40;
    const svgWidth = NAME_WIDTH + totalCycles * CYCLE_WIDTH + 20;

    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('viewBox', '0 0 ' + svgWidth + ' ' + svgHeight);
    svg.setAttribute('class', 'waveform-svg');

    // Cycle grid lines
    for (let c = 0; c <= totalCycles && c <= 1024; c++) {
        const x = NAME_WIDTH + c * CYCLE_WIDTH;
        const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
        line.setAttribute('x1', x);
        line.setAttribute('y1', 0);
        line.setAttribute('x2', x);
        line.setAttribute('y2', svgHeight);
        line.setAttribute('class', 'waveform-grid');
        svg.appendChild(line);

        if (c % 5 === 0) {
            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('x', x);
            text.setAttribute('y', svgHeight - 5);
            text.setAttribute('class', 'waveform-cycle-label');
            text.textContent = c.toString();
            svg.appendChild(text);
        }
    }

    // Render each signal
    signals.forEach(function(signal, idx) {
        const yBase = idx * (LANE_HEIGHT + LANE_GAP) + 20;
        renderSignalWave(svg, signal, yBase, LANE_HEIGHT, NAME_WIDTH, CYCLE_WIDTH);
    });

    container.appendChild(svg);
}

function renderSignalWave(svg, signal, yBase, laneHeight, nameWidth, cycleWidth) {
    // Signal name label
    const label = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    label.setAttribute('x', 5);
    label.setAttribute('y', yBase + laneHeight / 2);
    label.setAttribute('class', 'waveform-signal-label');
    label.textContent = signal.name || 'unknown';
    svg.appendChild(label);

    const values = signal.values || [];
    if (values.length === 0) return;

    const isBinary = signal.width === 1;
    const kind = signal.kind || 'input';
    const signalClass = 'waveform-signal-' + kind;

    // Build SVG path
    var pathD = '';
    for (var c = 0; c < values.length && c <= 1024; c++) {
        var x = nameWidth + c * cycleWidth;
        if (isBinary) {
            var high = yBase + 5;
            var low = yBase + laneHeight - 5;
            var y = values[c] ? high : low;
            if (c === 0) {
                pathD += 'M ' + x + ' ' + y;
            } else {
                var prevY = values[c - 1] ? high : low;
                if (prevY !== y) {
                    pathD += ' L ' + x + ' ' + prevY + ' L ' + x + ' ' + y;
                }
            }
            pathD += ' L ' + (x + cycleWidth) + ' ' + y;
        } else {
            // Multi-bit: draw hex value label in center of each cycle
            var valLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            valLabel.setAttribute('x', x + cycleWidth / 2);
            valLabel.setAttribute('y', yBase + laneHeight / 2);
            valLabel.setAttribute('class', 'waveform-value-label');
            valLabel.textContent = '0x' + (values[c] || 0).toString(16);
            svg.appendChild(valLabel);
        }
    }

    if (pathD) {
        var path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
        path.setAttribute('d', pathD);
        path.setAttribute('class', signalClass);
        svg.appendChild(path);
    }
}

// ── Circuit Graph Renderer (Campaign B4) ──

function renderCircuitGraph(containerId, graphJson) {
    var container = document.getElementById(containerId);
    if (!container) return;
    container.innerHTML = '';

    var graph;
    try {
        graph = typeof graphJson === 'string' ? JSON.parse(graphJson) : graphJson;
    } catch (e) {
        container.innerHTML = '<pre class="viz-error">Invalid graph data</pre>';
        return;
    }

    if (typeof d3 === 'undefined') {
        container.innerHTML = '<pre class="viz-error">D3.js not loaded</pre>';
        return;
    }

    var nodes = (graph.nodes || []).map(function(n) { return Object.assign({}, n); });
    var edges = (graph.edges || []).map(function(e) {
        return { source: e.from, target: e.to, label: e.label };
    });

    var width = 900;
    var height = Math.max(400, nodes.length * 60 + 40);

    var svg = d3.select(container).append('svg')
        .attr('viewBox', '0 0 ' + width + ' ' + height)
        .attr('class', 'circuit-graph-svg');

    // Zoom + pan
    var g = svg.append('g');
    svg.call(d3.zoom()
        .scaleExtent([0.3, 4])
        .on('zoom', function(event) { g.attr('transform', event.transform); })
    );

    // Arrowhead marker
    g.append('defs').append('marker')
        .attr('id', 'arrowhead')
        .attr('markerWidth', 10).attr('markerHeight', 7)
        .attr('refX', 18).attr('refY', 3.5)
        .attr('orient', 'auto')
      .append('polygon')
        .attr('points', '0 0, 10 3.5, 0 7')
        .attr('fill', '#666');

    // Force simulation
    var simulation = d3.forceSimulation(nodes)
        .force('link', d3.forceLink(edges).id(function(d) { return d.id; }).distance(160))
        .force('charge', d3.forceManyBody().strength(-400))
        .force('center', d3.forceCenter(width / 2, height / 2))
        .force('collide', d3.forceCollide(60))
        .force('x', d3.forceX(function(d) {
            var col = { Input: width * 0.1, Output: width * 0.9, Guard: width * 0.4, Reflex: width * 0.65 };
            return col[d.type] || width / 2;
        }).strength(0.3))
        .force('y', d3.forceY(height / 2).strength(0.05));

    // Edge lines
    var link = g.selectAll('.circuit-edge')
        .data(edges).enter().append('path')
        .attr('class', 'circuit-edge')
        .attr('marker-end', 'url(#arrowhead)');

    // Edge labels
    var linkLabel = g.selectAll('.circuit-edge-label')
        .data(edges.filter(function(e) { return e.label; }))
        .enter().append('text')
        .attr('class', 'circuit-edge-label')
        .text(function(d) { return d.label; });

    // Node groups
    var node = g.selectAll('.circuit-node-group')
        .data(nodes).enter().append('g')
        .attr('class', 'circuit-node-group')
        .call(d3.drag()
            .on('start', function(event, d) {
                if (!event.active) simulation.alphaTarget(0.3).restart();
                d.fx = d.x; d.fy = d.y;
            })
            .on('drag', function(event, d) { d.fx = event.x; d.fy = event.y; })
            .on('end', function(event, d) {
                if (!event.active) simulation.alphaTarget(0);
                d.fx = null; d.fy = null;
            })
        );

    node.append('rect')
        .attr('width', 100).attr('height', 30).attr('rx', 8)
        .attr('x', -50).attr('y', -15)
        .attr('class', function(d) { return 'circuit-node-' + (d.type || 'Internal'); })
        .attr('stroke-width', 2);

    node.append('text')
        .attr('class', 'circuit-node-label')
        .text(function(d) { return d.label || d.id; });

    // Tick: update positions each frame
    var MAX_TICKS = 300;
    var tickCount = 0;
    simulation.on('tick', function() {
        tickCount++;
        if (tickCount > MAX_TICKS) { simulation.stop(); return; }

        link.attr('d', function(d) {
            var mx = (d.source.x + d.target.x) / 2;
            return 'M ' + d.source.x + ' ' + d.source.y +
                   ' C ' + mx + ' ' + d.source.y + ', ' + mx + ' ' + d.target.y +
                   ', ' + d.target.x + ' ' + d.target.y;
        });

        linkLabel
            .attr('x', function(d) { return (d.source.x + d.target.x) / 2; })
            .attr('y', function(d) { return (d.source.y + d.target.y) / 2 - 8; });

        node.attr('transform', function(d) { return 'translate(' + d.x + ',' + d.y + ')'; });
    });
}

// ── Button Wiring (Campaign B7) ──

document.getElementById('btn-simulate-waveform')?.addEventListener('click', async function() {
    var source = getSource();
    var cyclesInput = document.getElementById('sim-cycles');
    var cycles = cyclesInput ? parseInt(cyclesInput.value || '32', 10) : 32;
    var container = document.getElementById('waveform-container');
    if (!container) return;

    try {
        if (typeof simulate_waveform === 'function') {
            var result = JSON.parse(simulate_waveform(source, Math.min(cycles, 1024)));
            if (result.ok) {
                renderWaveform('waveform-container', result.ok);
            } else {
                container.innerHTML = '<pre class="viz-error">' + escapeHtml(result.err) + '</pre>';
            }
        } else {
            container.innerHTML = '<pre class="viz-error">Waveform simulation not available (WASM not loaded)</pre>';
        }
    } catch (e) {
        container.innerHTML = '<pre class="viz-error">Simulation failed: ' + escapeHtml(e.message) + '</pre>';
    }
});

document.getElementById('btn-view-circuit')?.addEventListener('click', async function() {
    var source = getSource();
    var container = document.getElementById('graph-container');
    if (!container) return;

    try {
        if (typeof compile_graph_data === 'function') {
            var result = JSON.parse(compile_graph_data(source));
            if (result.ok) {
                renderCircuitGraph('graph-container', result.ok);
            } else {
                container.innerHTML = '<pre class="viz-error">' + escapeHtml(result.err) + '</pre>';
            }
        } else {
            container.innerHTML = '<pre class="viz-error">Graph visualization not available (WASM not loaded)</pre>';
        }
    } catch (e) {
        container.innerHTML = '<pre class="viz-error">Graph generation failed: ' + escapeHtml(e.message) + '</pre>';
    }
});

/* ── Full-text search (lunr.js) ── */
(function initSearch() {
    var MAX_SEARCH_NODES = 200;
    var MAX_SEARCH_RESULTS = 10;
    var SNIPPET_WINDOW = 100;
    var input = document.getElementById('site-search');
    var resultsBox = document.getElementById('search-results');
    var kbdHint = document.querySelector('.search-kbd');
    if (!input || !resultsBox) return;

    /* --- Build document store from all sections --- */
    var docs = [];
    var docMap = {};
    var sections = document.querySelectorAll('section[id]');
    for (var i = 0; i < sections.length && i < MAX_SEARCH_NODES; i++) {
        var sec = sections[i];
        var heading = sec.querySelector('h2, h3');
        var title = heading ? heading.textContent.trim() : sec.id;
        var text = sec.textContent.replace(/\s+/g, ' ').trim();
        var doc = { id: sec.id, title: title, text: text };
        docs.push(doc);
        docMap[sec.id] = doc;
    }

    /* --- Build lunr index (stemmed, tokenized, ranked) --- */
    var lunrIndex = null;
    if (typeof lunr !== 'undefined') {
        lunrIndex = lunr(function() {
            this.ref('id');
            this.field('title', { boost: 3 });
            this.field('text');
            for (var j = 0; j < docs.length; j++) {
                this.add(docs[j]);
            }
        });
    }

    var activeIndex = -1;
    var debounceTimer = null;

    /* --- Ctrl+K / Cmd+K / '/' keyboard shortcut --- */
    document.addEventListener('keydown', function(e) {
        var isSlash = e.key === '/' && !e.ctrlKey && !e.metaKey && !e.altKey;
        var isCtrlK = e.key === 'k' && (e.ctrlKey || e.metaKey);
        if (isSlash || isCtrlK) {
            var tag = document.activeElement ? document.activeElement.tagName : '';
            if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') {
                if (document.activeElement !== input) return;
            }
            e.preventDefault();
            input.focus();
            input.select();
        }
    });

    /* --- Input handler with debounce --- */
    input.addEventListener('input', function() {
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(doSearch, 150);
    });

    /* --- Keyboard navigation (arrows, Enter, Escape) --- */
    input.addEventListener('keydown', function(e) {
        var items = resultsBox.querySelectorAll('a.search-result-item');
        if (e.key === 'Escape') {
            input.value = '';
            resultsBox.hidden = true;
            activeIndex = -1;
            input.blur();
            return;
        }
        if (e.key === 'ArrowDown') {
            e.preventDefault();
            if (items.length === 0) return;
            activeIndex = (activeIndex + 1) % items.length;
            highlightItem(items);
            return;
        }
        if (e.key === 'ArrowUp') {
            e.preventDefault();
            if (items.length === 0) return;
            activeIndex = activeIndex <= 0 ? items.length - 1 : activeIndex - 1;
            highlightItem(items);
            return;
        }
        if (e.key === 'Enter') {
            e.preventDefault();
            if (activeIndex >= 0 && activeIndex < items.length) {
                items[activeIndex].click();
            } else if (items.length > 0) {
                items[0].click();
            }
            return;
        }
    });

    function highlightItem(items) {
        for (var i = 0; i < items.length; i++) {
            if (i === activeIndex) {
                items[i].classList.add('search-result-active');
                items[i].scrollIntoView({ block: 'nearest' });
            } else {
                items[i].classList.remove('search-result-active');
            }
        }
    }

    /* --- Hide on outside click --- */
    document.addEventListener('click', function(e) {
        if (!input.contains(e.target) && !resultsBox.contains(e.target)) {
            resultsBox.hidden = true;
            activeIndex = -1;
        }
    });

    /* --- Hide kbd hint on focus, show on blur --- */
    if (kbdHint) {
        input.addEventListener('focus', function() { kbdHint.hidden = true; });
        input.addEventListener('blur', function() {
            if (!input.value) kbdHint.hidden = false;
        });
    }

    /* --- Extract snippet around a query term in text --- */
    function extractSnippet(text, query) {
        var lower = text.toLowerCase();
        var terms = query.toLowerCase().split(/\s+/);
        var pos = -1;
        for (var t = 0; t < terms.length; t++) {
            pos = lower.indexOf(terms[t]);
            if (pos !== -1) break;
        }
        if (pos === -1) pos = 0;
        var start = Math.max(0, pos - SNIPPET_WINDOW / 2);
        var end = Math.min(text.length, pos + query.length + SNIPPET_WINDOW / 2);
        return (start > 0 ? '\u2026' : '') +
            text.substring(start, end) +
            (end < text.length ? '\u2026' : '');
    }

    /* --- Fallback substring search (when lunr CDN unavailable) --- */
    function substringSearch(query) {
        var matches = [];
        var q = query.toLowerCase();
        for (var i = 0; i < docs.length && matches.length < MAX_SEARCH_RESULTS; i++) {
            var d = docs[i];
            if (d.title.toLowerCase().indexOf(q) !== -1 || d.text.toLowerCase().indexOf(q) !== -1) {
                matches.push({ id: d.id, title: d.title, snippet: extractSnippet(d.text, query) });
            }
        }
        return matches;
    }

    /* --- Main search function --- */
    function doSearch() {
        var query = input.value.trim();
        if (query.length < 2) { resultsBox.hidden = true; activeIndex = -1; return; }

        var matches = [];

        if (lunrIndex) {
            /* lunr search: try exact first, then wildcard */
            var results = lunrIndex.search(query);
            if (results.length === 0 && query.indexOf('*') === -1) {
                results = lunrIndex.search(query + '*');
            }
            for (var i = 0; i < results.length && matches.length < MAX_SEARCH_RESULTS; i++) {
                var doc = docMap[results[i].ref];
                if (doc) {
                    matches.push({
                        id: doc.id,
                        title: doc.title,
                        snippet: extractSnippet(doc.text, query)
                    });
                }
            }
        } else {
            matches = substringSearch(query);
        }

        if (matches.length === 0) {
            resultsBox.innerHTML = '<div class="search-no-results">No results for \u201c' +
                escapeHtml(query) + '\u201d</div>';
        } else {
            var html = '';
            var safeQuery = escapeHtml(query);
            var reTerms = safeQuery.split(/\s+/).map(function(t) {
                return t.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
            }).join('|');
            var re = new RegExp('(' + reTerms + ')', 'gi');
            for (var j = 0; j < matches.length; j++) {
                var m = matches[j];
                var safeTitle = escapeHtml(m.title).replace(re, '<mark>$1</mark>');
                var safeSnippet = escapeHtml(m.snippet).replace(re, '<mark>$1</mark>');
                html += '<a class="search-result-item" href="#' + escapeHtml(m.id) + '" role="option">' +
                    '<span class="search-result-title">' + safeTitle + '</span>' +
                    '<span class="search-result-snippet">' + safeSnippet + '</span>' +
                    '</a>';
            }
            resultsBox.innerHTML = html;
        }
        resultsBox.hidden = false;
        activeIndex = -1;

        /* Clicking a result closes dropdown */
        var links = resultsBox.querySelectorAll('a');
        for (var k = 0; k < links.length; k++) {
            links[k].addEventListener('click', function() {
                resultsBox.hidden = true;
                input.value = '';
                activeIndex = -1;
                if (kbdHint) kbdHint.hidden = false;
            });
        }
    }
})();

/* ── Active Section Tracking (Jekyll-style nav highlighting) ── */
(function initScrollSpy() {
    var MAX_SECTIONS = 50;
    var navLinks = document.querySelectorAll('#site-header nav[aria-label] a');
    if (navLinks.length === 0) return;

    var sectionIds = [];
    for (var i = 0; i < navLinks.length && i < MAX_SECTIONS; i++) {
        var href = navLinks[i].getAttribute('href');
        if (href && href.charAt(0) === '#') {
            sectionIds.push(href.substring(1));
        }
    }

    var lastActive = '';
    var ticking = false;

    function updateActive() {
        ticking = false;
        var scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        var headerHeight = 80;
        var current = '';

        for (var i = 0; i < sectionIds.length; i++) {
            var el = document.getElementById(sectionIds[i]);
            if (el && el.offsetTop - headerHeight <= scrollTop + 10) {
                current = sectionIds[i];
            }
        }

        if (current !== lastActive) {
            lastActive = current;
            for (var j = 0; j < navLinks.length; j++) {
                var href = navLinks[j].getAttribute('href');
                if (href === '#' + current) {
                    navLinks[j].classList.add('nav-active');
                } else {
                    navLinks[j].classList.remove('nav-active');
                }
            }
        }
    }

    window.addEventListener('scroll', function() {
        if (!ticking) {
            ticking = true;
            requestAnimationFrame(updateActive);
        }
    }, { passive: true });

    updateActive();
})();
