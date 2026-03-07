const { spawn } = require('child_process');
const path = require('path');

function sendRpc(proc, obj) {
  const json = JSON.stringify(obj);
  const header = 'Content-Length: ' + Buffer.byteLength(json, 'utf8') + '\r\n\r\n';
  proc.stdin.write(header + json);
}

async function run() {
  const proc = spawn(process.execPath, ['start.js'], {
    cwd: path.resolve(__dirname),
    env: { ...process.env, MCP_STDIO_DIRECT: '1' },
    stdio: ['pipe', 'pipe', 'inherit']
  });

  let buf = '';
  proc.stdout.on('data', d => {
    buf += d.toString();
    // try to extract complete JSON objects from Content-Length framing
    while (true) {
      const hdrEnd = buf.indexOf('\r\n\r\n');
      if (hdrEnd === -1) break;
      const header = buf.slice(0, hdrEnd);
      const m = header.match(/Content-Length:\s*(\d+)/i);
      if (!m) {
        // fallback: handle line-delimited JSON
        const nl = buf.indexOf('\n');
        if (nl === -1) break;
        const line = buf.slice(0, nl).trim();
        buf = buf.slice(nl + 1);
        if (!line) continue;
        try { console.log('RECV:', JSON.parse(line)); } catch (e) { console.log('RECV_LINE:', line); }
        continue;
      }
      const len = parseInt(m[1], 10);
      const bodyStart = hdrEnd + 4;
      if (buf.length < bodyStart + len) break;
      const body = buf.slice(bodyStart, bodyStart + len);
      buf = buf.slice(bodyStart + len);
      try {
        const msg = JSON.parse(body);
        console.log('RECV_FRAMED:', JSON.stringify(msg, null, 2));
      } catch (e) {
        console.log('RECV_INVALID_JSON', e.message);
      }
    }
  });

  proc.on('exit', (c) => {
    // do nothing
  });

  // send a tools/list request (MCP) to retrieve manifest
  const req = { jsonrpc: '2.0', id: 1, method: 'tools/list', params: {} };
  sendRpc(proc, req);

  // also request a call probe: read_text_file (will likely return error without api key)
  const probeCall = { jsonrpc: '2.0', id: 2, method: 'tools/call', params: { name: 'read_text_file', arguments: { path: 'mcp_server/README.md' } } };
  // delay slightly to let server initialise
  setTimeout(() => sendRpc(proc, probeCall), 200);

  // allow some time to receive responses then exit
  setTimeout(() => {
    proc.kill();
    process.exit(0);
  }, 2000);
}

run().catch(err => { console.error(err); process.exit(1); });