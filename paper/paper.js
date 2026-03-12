// paper.js — Interactive demo layer for MIRR Living Research Artifact
// No external dependencies. No npm. No CDN.
// GPL-3.0 — same license as the compiler.

import init, {
  compile_verilog,
  compile_firrtl,
  compile_rspu,
  compile_sexpr,
  compile_dot,
  infer_widths,
  mirr_version
} from './demos/mirr_wasm.js';

// Must match MAX_SOURCE_BYTES in crates/mirr-wasm/src/lib.rs
const MAX_SOURCE_BYTES = 65_536;

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

// Map format names to per-function WASM exports
const COMPILERS = {
  verilog: compile_verilog,
  firrtl:  compile_firrtl,
  rspu:    compile_rspu,
  sexpr:   compile_sexpr,
  json:    infer_widths,
  dot:     compile_dot
};

let wasmReady = false;

async function initWasm() {
  try {
    await init();
    wasmReady = true;
    document.getElementById('compiler-output').textContent =
      '// Compiler ready. Type MIRR source or load an example.';
    // Inject version
    const vResult = JSON.parse(mirr_version());
    if (vResult.ok) {
      document.querySelectorAll('.mirr-version')
        .forEach(el => el.textContent = vResult.ok);
    }
  } catch (err) {
    document.getElementById('compiler-output').textContent =
      'Failed to load compiler WASM: ' + err.message;
    document.getElementById('compiler-output').classList.add('error');
  }
}

function compile() {
  if (!wasmReady) return;

  const source = document.getElementById('mirr-source').value;
  const format = document.getElementById('emit-format').value;
  const output = document.getElementById('compiler-output');
  output.setAttribute('aria-busy', 'true');
  const label  = document.getElementById('output-label');

  if (source.length > MAX_SOURCE_BYTES) {
    output.textContent =
      `Source too large (${source.length} bytes). Limit is ${MAX_SOURCE_BYTES} bytes.`;
    output.classList.add('error');
    output.setAttribute('aria-busy', 'false');
    return;
  }

  label.textContent = '(' + format + ')';

  const compiler = COMPILERS[format];
  if (!compiler) return;

  const result = JSON.parse(compiler(source));

  if (result.ok !== undefined) {
    output.textContent = result.ok;
    output.classList.remove('error');
  } else if (result.err !== undefined) {
    output.textContent = result.err;
    output.classList.add('error');
  }
  output.setAttribute('aria-busy', 'false');
}

async function runBenchmarks() {
  if (!wasmReady) return;

  const btn = document.getElementById('bench-btn');
  const tbody = document.getElementById('benchmark-rows');
  btn.disabled = true;
  btn.textContent = 'Running...';
  tbody.innerHTML = '';

  const formats = ['verilog', 'firrtl', 'rspu', 'sexpr', 'json', 'dot'];
  const source = EXAMPLES.tmr;

  for (const fmt of formats) {
    const compiler = COMPILERS[fmt];
    let elapsed, lines, isError;
    try {
      const start = performance.now();
      const raw = compiler(source);
      elapsed = (performance.now() - start).toFixed(2);
      const result = JSON.parse(raw);
      lines = result.ok ? result.ok.split('\n').length : 0;
      isError = !!result.err;
    } catch (err) {
      elapsed = 'ERROR';
      lines = 0;
      isError = true;
    }

    const row = document.createElement('tr');
    row.innerHTML = `
      <td>${fmt}</td>
      <td>${elapsed}</td>
      <td>${lines}</td>
    `;
    if (isError) {
      row.classList.add('error');
    }
    tbody.appendChild(row);

    // Yield to browser between targets so UI stays responsive
    await new Promise(r => setTimeout(r, 0));
  }

  btn.disabled = false;
  btn.textContent = 'Run Benchmarks';
}

// Wire up controls
document.getElementById('compile-btn')
  .addEventListener('click', compile);

document.getElementById('example-select')
  .addEventListener('change', e => {
    const key = e.target.value;
    if (key && EXAMPLES[key]) {
      document.getElementById('mirr-source').value = EXAMPLES[key];
      compile();
    }
  });

document.getElementById('emit-format')
  .addEventListener('change', compile);

document.getElementById('bench-btn')
  .addEventListener('click', runBenchmarks);

// Keyboard shortcut: Ctrl+Enter or Cmd+Enter to compile
document.getElementById('mirr-source')
  .addEventListener('keydown', e => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      compile();
    }
  });

// Boot
initWasm();

// ── LRA Protocol bridge (Phase 4) ──────────────────────────────────
// Service Worker relays lra.run_tool requests here because only the
// page has access to the WASM module.

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.addEventListener('message', function(event) {
    var data = event.data || {};
    if (data.type !== 'lra.run_tool.relay') return;
    if (!wasmReady) {
      navigator.serviceWorker.controller.postMessage({
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
      navigator.serviceWorker.controller.postMessage({
        type: 'lra.run_tool.response',
        relay_id: data.relay_id,
        error: { code: -32602, message: 'Unknown format: ' + format }
      });
      return;
    }
    try {
      var raw = fn(input);
      var result = JSON.parse(raw);
      navigator.serviceWorker.controller.postMessage({
        type: 'lra.run_tool.response',
        relay_id: data.relay_id,
        result: result
      });
    } catch (e) {
      navigator.serviceWorker.controller.postMessage({
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
