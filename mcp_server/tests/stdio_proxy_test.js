const { spawn } = require('child_process');
const assert = require('assert');
const path = require('path');
const fs = require('fs');

// helper to send a single JSON‑RPC request over stdio
function sendRequest(proc, req) {
  // allow passing apiKey property separately or inside req.apiKey
  req.id = req.id || Math.floor(Math.random() * 1e6);
  const expectedId = req.id;
  return new Promise((resolve) => {
    let buf = '';
    const onData = d => {
      buf += d.toString();
      let idx;
      while ((idx = buf.indexOf('\n')) >= 0) {
        const line = buf.slice(0, idx).trim();
        buf = buf.slice(idx + 1);
        if (!line) continue;
        try {
          const msg = JSON.parse(line);
          if (msg.id !== expectedId) {
            continue;
          }
          proc.stdout.off('data', onData);
          resolve(msg);
        } catch (e) {
          // ignore parse failures
        }
      }
    };
    proc.stdout.on('data', onData);
    proc.stdin.write(JSON.stringify(req) + '\n');
  });
}

async function startServer(args = [], extraEnv = {}) {
  const env = { ...process.env, ...extraEnv };
  const proc = spawn('node', ['start.js', ...args], {
    cwd: path.resolve(__dirname, '..'),
    env
  });
  proc.stderr.on('data', d => process.stderr.write('wrapper stderr: ' + d));
  proc.on('exit', (code, signal) => {
    console.log(`wrapper process exited code=${code} signal=${signal}`);
  });
  proc.on('error', err => {
    console.log('wrapper spawn error', err);
  });
  await new Promise(r => setTimeout(r, 200));
  return { proc };
}

async function stopServer(proc) {
  proc.kill();
  await new Promise(r => setTimeout(r, 50));
}

async function testConnReset() {
  console.log('testing simulated ECONNRESET');
  const { proc } = await startServer([], { MCP_TEST_FORCE_RESET: '1' });
  const resp = await sendRequest(proc, { method: 'mcp_schema', params: {} });
  assert.strictEqual(resp.error?.code, -32000, 'expected -32000 error');
  assert.ok(resp.error.message.includes('ECONNRESET'), 'message should mention ECONNRESET');
  await stopServer(proc);
}

async function runTests() {
  delete process.env.MCP_TEST_FORCE_RESET;

  // ensure an initial admin key exists in config so we can manage keys
  const cfgPath = path.join(__dirname, '..', 'config.json');
  try {
    fs.writeFileSync(cfgPath, JSON.stringify({
      api_keys: [{ id: 'admin', role: 'admin', token: 'ADMIN' }]
    }, null, 2));
  } catch (e) {}

  console.log('running stdio-direct tests');
  const { proc } = await startServer();

  const schema = await sendRequest(proc, { method: 'mcp_schema', params: {} });
  assert.strictEqual(schema.jsonrpc, '2.0');
  assert.ok(schema.result && schema.result.methods);

  const health = await sendRequest(proc, { method: 'health', params: {} });
  assert.strictEqual(health.jsonrpc, '2.0');
  assert.strictEqual(health.result.ok, true);

  // introspect registered handlers in stdio mode; retry until admin endpoints appear
  let handlersList;
  for (let i = 0; i < 10; i++) {
    handlersList = await sendRequest(proc, { method: 'list_handlers', params: {} });
    if (handlersList.result && Array.isArray(handlersList.result.handlers) && handlersList.result.handlers.includes('generate_api_key')) {
      break;
    }
    console.log('waiting for admin handlers, current:', handlersList);
    await new Promise(r => setTimeout(r, 100));
  }
  console.log('registered handlers', handlersList);

  const resp = await sendRequest(proc, { method: 'no_such_method', params: {} });

  // test API key generation/listing/revocation using the ADMIN key
  const requestObj = { method: 'generate_api_key', params: { id: 'bob', role: 'committer' }, apiKey: 'ADMIN' };
  console.log('sending request', requestObj);
  const gen = await sendRequest(proc, requestObj);
  console.log('generate_api_key response', gen);
  assert.ok(gen.result && gen.result.token);
  const list = await sendRequest(proc, { method: 'list_api_keys', params: {}, apiKey: 'ADMIN' });
  assert.ok(list.result.keys.some(k => k.id === 'bob'));
  const revoke = await sendRequest(proc, { method: 'revoke_api_key', params: { id: 'bob' }, apiKey: 'ADMIN' });
  assert.strictEqual(revoke.result.ok, true);

  // subsequent generation with non-admin key should fail
  const badGen = await sendRequest(proc, { method: 'generate_api_key', params: { id: 'alice', role: 'committer' }, apiKey: 'bob-token' });
  assert.ok(badGen.error);

  // ---- RBAC and path whitelist checks ----
  // bob was a committer; generate another committer and a builder for tests
  const genBuilder = await sendRequest(proc, { method: 'generate_api_key', params: { id: 'charlie', role: 'builder' }, apiKey: 'ADMIN' });
  assert.ok(genBuilder.result && genBuilder.result.token);
  const builderToken = genBuilder.result.token;
  const genComm = await sendRequest(proc, { method: 'generate_api_key', params: { id: 'dave', role: 'committer' }, apiKey: 'ADMIN' });
  assert.ok(genComm.result && genComm.result.token);
  const committerToken = genComm.result.token;

  // builder should be able to run cargo but not write_file
  const cargoResp = await sendRequest(proc, { method: 'run_cargo', params: { subcommand: 'check' }, apiKey: builderToken });
  assert.ok(cargoResp.result && cargoResp.result.exitCode === 0);
  const noWrite = await sendRequest(proc, { method: 'write_file', params: { path: 'temp_builder.txt', content: 'x' }, apiKey: builderToken });
  assert.ok(noWrite.error && noWrite.error.code === 403, 'builder must not write_file');

  // committer may write but not run_cargo? committer is allowed both
  const commWrite = await sendRequest(proc, { method: 'write_file', params: { path: 'temp_committer.txt', content: 'y' }, apiKey: committerToken });
  assert.strictEqual(commWrite.result.ok, true);
  const commCargo = await sendRequest(proc, { method: 'run_cargo', params: { subcommand: 'check' }, apiKey: committerToken });
  assert.ok(commCargo.result && commCargo.result.exitCode === 0);

  // path whitelist: attempt to write outside allowed_paths
  const outside = await sendRequest(proc, { method: 'write_file', params: { path: '../outside.txt', content: 'bad' }, apiKey: committerToken });
  assert.ok(outside.error && outside.error.code === 403);

  // we now treat unknown methods as harmless null results for compatibility
  assert.strictEqual(resp.result, null);
  // server should have logged the missing method (check stderr if needed)

  // simulate a CLINE probe such as ctx.sample – should also return null
  const probe = await sendRequest(proc, { method: 'ctx.sample', params: {} });
  assert.strictEqual(probe.result, null);

  // another common probe: resources/templates/list
  const tpl = await sendRequest(proc, { method: 'resources/templates/list', params: {} });
  assert.deepStrictEqual(tpl.result, { templates: [] });

  // exercise search_files against workspace root
  const search = await sendRequest(proc, { method: 'search_files', params: { path: '.', pattern: '**/*.rs', ignore: ['target/**'] } });
  assert.ok(Array.isArray(search.result.matches));

  // schema validation: missing required field for read_text_file
  const badRead = await sendRequest(proc, { method: 'read_text_file', params: { head: 10 } });
  assert.strictEqual(badRead.error?.code, 400, 'expected schema error for missing path');
  assert.ok(badRead.error.message.includes('schema_validation'));

  // schema validation: extra property for write_file
  const badWrite = await sendRequest(proc, { method: 'write_file', params: { path: 'x', content: 'y', extra: 123 }, apiKey: 'ADMIN' });
  assert.strictEqual(badWrite.error?.code, 400, 'expected schema error for extra property');
  assert.ok(badWrite.error.message.includes('schema_validation'));

  // MRT runtime path must enforce deny-by-default role policy.
  const anonMrt = await sendRequest(proc, { method: 'mrt_audit', params: { mode: 'workspace' } });
  assert.strictEqual(anonMrt.error?.code, 401, 'mrt_audit without API key must be rejected');

  const adminOnly = await sendRequest(proc, { method: 'mrt_brain_get', params: { key: 'proposal-096' }, apiKey: builderToken });
  assert.strictEqual(adminOnly.error?.code, 403, 'builder must not call mrt_brain_get');

  const unknownTool = await sendRequest(proc, {
    method: 'mrt_execute',
    params: { tool: 'mrt_unknown_tool', args: [] },
    apiKey: 'ADMIN'
  });
  assert.strictEqual(unknownTool.error?.code, 400, 'unknown mrt_execute tool must return 400');
  assert.ok(unknownTool.error.message.includes('unknown_tool'));

  // Wave 2 strict MRT contract checks (source-level canaries).
  const mrtPath = path.join(__dirname, '..', 'src', 'mrt.ts');
  const mrtText = fs.readFileSync(mrtPath, 'utf8');
  assert.ok(mrtText.includes('TOOL_ROLE_ALLOWLIST'), 'mrt role allowlist must exist');
  assert.ok(mrtText.includes("MRT_EXEC_ERROR: unauthorized role"), 'unauthorized role message must be explicit');
  assert.ok(mrtText.includes("MRT_EXEC_ERROR: unknown tool"), 'unknown tool message must be explicit');

  // exercise directory_tree with depth 1
  const tree = await sendRequest(proc, { method: 'directory_tree', params: { path: '.', maxDepth: 1 } });
  assert.strictEqual(tree.result.root.toLowerCase().endsWith('nasa-rust-project'), true);

  // concurrency test: spawn several long_running requests in parallel; the
  // default limit is 2 per token, so at least one should fail with 429.
  const jobs = [];
  for (let i = 0; i < 4; i++) {
    jobs.push(sendRequest(proc, { method: 'long_running', params: {} }));
  }
  const results = await Promise.all(jobs);
  const statusCodes = results.map(r => r.error ? r.error.code : 200);
  assert.ok(statusCodes.includes(429) || statusCodes.includes(500), 'expected at least one concurrency error');

  const fn = path.join(__dirname, '..', '..', 'temp_test.txt');
  const write = await sendRequest(proc, { method: 'write_file', params: { path: 'temp_test.txt', content: 'xyz' }, apiKey: committerToken });
  assert.strictEqual(write.result.ok, true);
  const read = await sendRequest(proc, { method: 'read_text_file', params: { path: 'temp_test.txt' }, apiKey: committerToken });
  assert.strictEqual(read.result.content, 'xyz');
  fs.unlinkSync(fn);

  await stopServer(proc);
  const portFile = path.join(__dirname, '..', '.mcp_port');
  const pipeFile = path.join(__dirname, '..', '.mcp_pipe');
  if (fs.existsSync(portFile) || fs.existsSync(pipeFile)) {
    throw new Error('unexpected port/pipe file created');
  }

  // garbage input shouldn't crash
  const p = await startServer();
  p.proc.stdin.write('notjson\n');
  await new Promise(r => setTimeout(r, 100));
  await stopServer(p.proc);

  await testConnReset();

  console.log('all tests passed');
}

runTests().catch(err => {
  console.error('tests failed', err);
  process.exit(1);
});

