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

  var source = document.getElementById('mirr-source').value;
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

function handleRspuSim(source, cycles) {
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
      document.getElementById('mirr-source').value = EXAMPLES[key];
      updateHighlight();
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
    var source = document.getElementById('mirr-source').value;
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
    var source = document.getElementById('mirr-source').value;
    var cycles = parseInt(document.getElementById('rspu-cycles').value, 10) || 10;
    var output = document.getElementById('rspu-sim-output');
    var data = handleRspuSim(source, cycles);
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
    var data = handleMapekSim(document.getElementById('mirr-source').value, ticks);
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

// Keyboard shortcut: Ctrl+Enter or Cmd+Enter to compile
document.getElementById('mirr-source')
  .addEventListener('keydown', function(e) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      compile();
    }
  });

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

// Register Service Worker for offline support + protocol
if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('sw.js').catch(function() {});
}

// ── MIRR syntax highlighting ────────────────────────────────────────

function escapeHtml(s) {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

function highlightMirr(code) {
  var MAX_LINES = 500;
  var signalKw = {signal:1,input:1,output:1,wire:1,reg:1,assign:1};
  var guardKw = {guard:1,when:1,cycles:1,for:1};
  var reflexKw = {reflex:1,on:1};
  var generalKw = {module:1,always:1,temporal:1,require:1,ensure:1,if:1,else:1,
    let:1,fn:1,struct:1,enum:1,match:1,return:1,property:1,pattern:1,prev:1,use:1};
  var dirs = {in:1,out:1,internal:1};
  var types = {u1:1,u2:1,u3:1,u4:1,u5:1,u6:1,u7:1,u8:1,u9:1,u10:1,u11:1,u12:1,
    u13:1,u14:1,u15:1,u16:1,u32:1,u64:1,i8:1,i16:1,i32:1,i64:1,bool:1,bit:1,clock:1,reset:1};
  var bools = {true:1,false:1};
  var lines = code.split('\n');
  var result = [];
  for (var i = 0; i < Math.min(lines.length, MAX_LINES); i++) {
    var line = lines[i];
    var trimmed = line.trimStart ? line.trimStart() : line.replace(/^\s+/, '');
    if (trimmed.indexOf('//') === 0) {
      var leading = line.substring(0, line.length - trimmed.length);
      result.push(escapeHtml(leading) + '<span class="mirr-cmt">' + escapeHtml(trimmed) + '</span>');
      continue;
    }
    var out = '';
    var j = 0;
    while (j < line.length) {
      var ch = line[j];
      if (ch === '/' && j + 1 < line.length && line[j + 1] === '/') {
        out += '<span class="mirr-cmt">' + escapeHtml(line.substring(j)) + '</span>';
        break;
      }
      if (/[a-zA-Z_]/.test(ch)) {
        var ident = '';
        while (j < line.length && /[a-zA-Z0-9_]/.test(line[j])) { ident += line[j++]; }
        var cls = signalKw[ident] ? 'mirr-signal' :
                  guardKw[ident] ? 'mirr-guard' :
                  reflexKw[ident] ? 'mirr-reflex' :
                  generalKw[ident] ? 'mirr-kw' :
                  dirs[ident] ? 'mirr-dir' :
                  types[ident] ? 'mirr-type' :
                  bools[ident] ? 'mirr-bool' : 'mirr-name';
        out += '<span class="' + cls + '">' + escapeHtml(ident) + '</span>';
        continue;
      }
      if (/[0-9]/.test(ch)) {
        var num = '';
        while (j < line.length && /[0-9_xbo]/.test(line[j])) { num += line[j++]; }
        out += '<span class="mirr-num">' + escapeHtml(num) + '</span>';
        continue;
      }
      if (ch === '@') {
        var ann = '@'; j++;
        while (j < line.length && /[a-zA-Z0-9_]/.test(line[j])) { ann += line[j++]; }
        out += '<span class="mirr-ann">' + escapeHtml(ann) + '</span>';
        continue;
      }
      if (ch === '#') {
        var tag = '#'; j++;
        while (j < line.length && /[a-zA-Z0-9_]/.test(line[j])) { tag += line[j++]; }
        out += '<span class="mirr-tag">' + escapeHtml(tag) + '</span>';
        continue;
      }
      if (/[+\-*=<>!&|^~%]/.test(ch)) {
        out += '<span class="mirr-op">' + escapeHtml(ch) + '</span>';
        j++; continue;
      }
      out += escapeHtml(ch);
      j++;
    }
    result.push(out);
  }
  return result.join('\n');
}

function updateHighlight() {
  var textarea = document.getElementById('mirr-source');
  var overlay = document.getElementById('highlight-overlay');
  if (overlay && textarea) {
    overlay.innerHTML = highlightMirr(textarea.value) + '\n';
    overlay.scrollTop = textarea.scrollTop;
    overlay.scrollLeft = textarea.scrollLeft;
  }
}

(function initHighlight() {
  var textarea = document.getElementById('mirr-source');
  var overlay = document.getElementById('highlight-overlay');
  if (textarea && overlay) {
    textarea.addEventListener('input', updateHighlight);
    textarea.addEventListener('scroll', function() {
      overlay.scrollTop = textarea.scrollTop;
      overlay.scrollLeft = textarea.scrollLeft;
    });
    updateHighlight();
    // Activate transparent text only after overlay is working
    var container = textarea.closest('.editor-container');
    if (container) container.classList.add('highlight-active');
  }
})();

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
    svg.setAttribute('width', svgWidth);
    svg.setAttribute('height', svgHeight);
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
    const container = document.getElementById(containerId);
    if (!container) return;
    container.innerHTML = '';

    let graph;
    try {
        graph = typeof graphJson === 'string' ? JSON.parse(graphJson) : graphJson;
    } catch (e) {
        container.innerHTML = '<pre class="viz-error">Invalid graph data</pre>';
        return;
    }

    const nodes = graph.nodes || [];
    const edges = graph.edges || [];
    const positions = layoutGraph(nodes, edges);

    const svgWidth = 900;
    const svgHeight = Math.max(400, nodes.length * 60 + 40);

    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('width', svgWidth);
    svg.setAttribute('height', svgHeight);
    svg.setAttribute('class', 'circuit-graph-svg');

    // Arrowhead marker definition
    const defs = document.createElementNS('http://www.w3.org/2000/svg', 'defs');
    const marker = document.createElementNS('http://www.w3.org/2000/svg', 'marker');
    marker.setAttribute('id', 'arrowhead');
    marker.setAttribute('markerWidth', '10');
    marker.setAttribute('markerHeight', '7');
    marker.setAttribute('refX', '10');
    marker.setAttribute('refY', '3.5');
    marker.setAttribute('orient', 'auto');
    const polygon = document.createElementNS('http://www.w3.org/2000/svg', 'polygon');
    polygon.setAttribute('points', '0 0, 10 3.5, 0 7');
    polygon.setAttribute('fill', '#666');
    marker.appendChild(polygon);
    defs.appendChild(marker);
    svg.appendChild(defs);

    // Draw edges first (behind nodes)
    edges.forEach(function(edge) {
        var fromPos = positions[edge.from];
        var toPos = positions[edge.to];
        if (fromPos && toPos) {
            drawEdge(svg, fromPos, toPos, edge.label);
        }
    });

    // Draw nodes
    nodes.forEach(function(node) {
        var pos = positions[node.id];
        if (pos) {
            drawNode(svg, node, pos.x, pos.y);
        }
    });

    container.appendChild(svg);
}

function layoutGraph(nodes, edges) {
    var positions = {};
    var columns = { 'Input': 100, 'Output': 800, 'Guard': 350, 'Reflex': 600, 'Internal': 500 };
    var counts = { 'Input': 0, 'Output': 0, 'Guard': 0, 'Reflex': 0, 'Internal': 0 };

    nodes.forEach(function(node) {
        var col = columns[node.type] || 450;
        var row = counts[node.type] || 0;
        positions[node.id] = { x: col, y: 40 + row * 60 };
        counts[node.type] = (counts[node.type] || 0) + 1;
    });

    return positions;
}

function drawNode(svg, node, x, y) {
    var rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
    rect.setAttribute('x', x - 50);
    rect.setAttribute('y', y - 15);
    rect.setAttribute('width', 100);
    rect.setAttribute('height', 30);
    rect.setAttribute('rx', 8);
    rect.setAttribute('class', 'circuit-node-' + (node.type || 'Internal'));
    rect.setAttribute('stroke-width', '2');
    svg.appendChild(rect);

    var text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('x', x);
    text.setAttribute('y', y);
    text.setAttribute('class', 'circuit-node-label');
    text.textContent = node.label || node.id;
    svg.appendChild(text);
}

function drawEdge(svg, fromPos, toPos, label) {
    var path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    var mx = (fromPos.x + toPos.x) / 2;
    var d = 'M ' + (fromPos.x + 50) + ' ' + fromPos.y +
            ' C ' + mx + ' ' + fromPos.y + ', ' + mx + ' ' + toPos.y +
            ', ' + (toPos.x - 50) + ' ' + toPos.y;
    path.setAttribute('d', d);
    path.setAttribute('class', 'circuit-edge');
    svg.appendChild(path);

    if (label) {
        var text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        text.setAttribute('x', mx);
        text.setAttribute('y', (fromPos.y + toPos.y) / 2 - 8);
        text.setAttribute('class', 'circuit-edge-label');
        text.textContent = label;
        svg.appendChild(text);
    }
}

// ── Button Wiring (Campaign B7) ──

document.getElementById('btn-simulate-waveform')?.addEventListener('click', async function() {
    var source = document.getElementById('mirr-source').value;
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
    var source = document.getElementById('mirr-source').value;
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

/* ── Search ── */
(function initSearch() {
    var MAX_SEARCH_NODES = 2000;
    var MAX_SEARCH_RESULTS = 10;
    var input = document.getElementById('site-search');
    var resultsBox = document.getElementById('search-results');
    if (!input || !resultsBox) return;

    var index = [];
    var sections = document.querySelectorAll('section[id]');
    var nodeCount = 0;
    for (var i = 0; i < sections.length && nodeCount < MAX_SEARCH_NODES; i++) {
        var sec = sections[i];
        var heading = sec.querySelector('h2, h3');
        var title = heading ? heading.textContent.trim() : sec.id;
        var text = sec.textContent.substring(0, 500).replace(/\s+/g, ' ').trim();
        index.push({ id: sec.id, title: title, text: text });
        nodeCount++;
    }

    var debounceTimer = null;
    input.addEventListener('input', function() {
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(doSearch, 200);
    });

    input.addEventListener('keydown', function(e) {
        if (e.key === 'Escape') {
            input.value = '';
            resultsBox.hidden = true;
        }
    });

    document.addEventListener('click', function(e) {
        if (!input.contains(e.target) && !resultsBox.contains(e.target)) {
            resultsBox.hidden = true;
        }
    });

    function doSearch() {
        var query = input.value.trim().toLowerCase();
        if (query.length < 2) { resultsBox.hidden = true; return; }
        var matches = [];
        for (var i = 0; i < index.length && matches.length < MAX_SEARCH_RESULTS; i++) {
            var entry = index[i];
            if (entry.title.toLowerCase().indexOf(query) !== -1 ||
                entry.text.toLowerCase().indexOf(query) !== -1) {
                matches.push(entry);
            }
        }
        if (matches.length === 0) {
            resultsBox.innerHTML = '<div class="search-result-item search-no-results">No results</div>';
        } else {
            var html = '';
            for (var j = 0; j < matches.length; j++) {
                html += '<a class="search-result-item" href="#' + escapeHtml(matches[j].id) +
                    '" role="option">' + escapeHtml(matches[j].title) + '</a>';
            }
            resultsBox.innerHTML = html;
        }
        resultsBox.hidden = false;

        var links = resultsBox.querySelectorAll('a');
        for (var k = 0; k < links.length; k++) {
            links[k].addEventListener('click', function() {
                resultsBox.hidden = true;
                input.value = '';
            });
        }
    }
})();
