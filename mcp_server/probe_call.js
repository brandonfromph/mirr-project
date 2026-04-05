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
    while (true) {
      const hdrEnd = buf.indexOf('\r\n\r\n');
      if (hdrEnd === -1) break;
      const header = buf.slice(0, hdrEnd);
      const m = header.match(/Content-Length:\s*(\d+)/i);
      if (!m) break;
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

  // send a tools/call request for read_text_file
  const req = { jsonrpc: '2.0', id: 10, method: 'tools/call', params: { name: 'read_text_file', arguments: { path: 'mcp_server/README.md' } } };
  // small delay to let server initialize
  setTimeout(() => sendRpc(proc, req), 200);

  setTimeout(() => { proc.kill(); process.exit(0); }, 3000);
}

run().catch(err => { console.error(err); process.exit(1); });