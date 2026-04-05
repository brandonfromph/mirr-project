const { spawn } = require('child_process');
const assert = require('assert');
const path = require('path');
const fs = require('fs');

// helper to send a single JSON-RPC request over stdio
function sendRequest(proc, req, timeoutMs = 0) {
  // allow passing apiKey property separately or inside req.apiKey
  req.id = req.id || Math.floor(Math.random() * 1e6);
  const expectedId = req.id;
  return new Promise((resolve) => {
    let buf = '';
    let timeoutHandle = null;

    const cleanup = () => {
      proc.stdout.off('data', onData);
      if (timeoutHandle !== null) {
        clearTimeout(timeoutHandle);
      }
    };

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
          cleanup();
          resolve(msg);
        } catch (e) {
          // ignore parse failures
        }
      }
    };
    proc.stdout.on('data', onData);
    if (timeoutMs > 0) {
      timeoutHandle = setTimeout(() => {
        cleanup();
        resolve(null);
      }, timeoutMs);
    }
    proc.stdin.write(JSON.stringify(req) + '\n');
  });
}

const READY_MAX_ATTEMPTS = 20;
const READY_DELAY_MS = 50;
const READY_REQUEST_TIMEOUT_MS = 200;

function sleepMs(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function waitForServerReady(proc, requiredHandler = null) {
  for (let attempt = 0; attempt < READY_MAX_ATTEMPTS; attempt++) {
    const handlers = await sendRequest(
      proc,
      { method: 'list_handlers', params: {} },
      READY_REQUEST_TIMEOUT_MS
    );
    if (
      handlers &&
      handlers.result &&
      Array.isArray(handlers.result.handlers) &&
      (!requiredHandler || handlers.result.handlers.includes(requiredHandler))
    ) {
      return handlers;
    }

    const schema = await sendRequest(
      proc,
      { method: 'mcp_schema', params: {} },
      READY_REQUEST_TIMEOUT_MS
    );
    if (
      schema &&
      schema.jsonrpc === '2.0' &&
      (schema.result || schema.error) &&
      !requiredHandler
    ) {
      return schema;
    }

    if (attempt + 1 < READY_MAX_ATTEMPTS) {
      await sleepMs(READY_DELAY_MS);
    }
  }

  throw new Error('server readiness check exceeded bounded attempts');
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
  return { proc };
}

async function stopServer(proc) {
  proc.kill();
  await new Promise(r => setTimeout(r, 50));
}

async function testConnReset() {
  console.log('testing simulated ECONNRESET');
  const { proc } = await startServer([], { MCP_TEST_FORCE_RESET: '1' });
  await waitForServerReady(proc);
  const resp = await sendRequest(proc, { method: 'mcp_schema', params: {} });
  assert.strictEqual(resp.error?.code, -32000, 'expected -32000 error');
  assert.ok(resp.error.message.includes('ECONNRESET'), 'message should mention ECONNRESET');
  await stopServer(proc);
}

async function runTests() {
  delete process.env.MCP_TEST_FORCE_RESET;

  // ensure an initial admin key exists in config so we can manage keys
  const cfgPath = path.join(__dirname, '..', 'config.json');
  const cfgBackup = fs.existsSync(cfgPath) ? fs.readFileSync(cfgPath, 'utf8') : null;

  try {
    try {
      fs.writeFileSync(cfgPath, JSON.stringify({
        api_keys: [{ id: 'admin', role: 'admin', token: 'ADMIN' }]
      }, null, 2));
    } catch (e) {}

    console.log('running stdio-direct tests');
    const { proc } = await startServer();
    const handlersList = await waitForServerReady(proc, 'generate_api_key');

  const schema = await sendRequest(proc, { method: 'mcp_schema', params: {} });
  assert.strictEqual(schema.jsonrpc, '2.0');
  assert.ok(schema.result && schema.result.methods);
  assert.ok(schema.result.methods.mrt_wave_dry_run, 'schema must advertise mrt_wave_dry_run');
  assert.ok(schema.result.methods.mrt_wave_apply, 'schema must advertise mrt_wave_apply');
  assert.ok(schema.result.methods.mrt_lsp_diagnostics, 'schema must advertise mrt_lsp_diagnostics');
  assert.ok(schema.result.methods.mrt_general_ci_compile, 'schema must advertise mrt_general_ci_compile');
  assert.ok(schema.result.methods.mrt_general_ci_fast, 'schema must advertise mrt_general_ci_fast');

  const health = await sendRequest(proc, { method: 'health', params: {} });
  assert.strictEqual(health.jsonrpc, '2.0');
  assert.strictEqual(health.result.ok, true);

  console.log('registered handlers', handlersList);
  assert.ok(handlersList.result && Array.isArray(handlersList.result.handlers));
  assert.ok(handlersList.result.handlers.includes('generate_api_key'));
  assert.ok(handlersList.result.handlers.includes('mrt_wave_dry_run'));
  assert.ok(handlersList.result.handlers.includes('mrt_wave_apply'));
  assert.ok(handlersList.result.handlers.includes('mrt_lsp_diagnostics'));
  assert.ok(handlersList.result.handlers.includes('mrt_general_ci_compile'));
  assert.ok(handlersList.result.handlers.includes('mrt_general_ci_fast'));

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

  // strict mode is default: unknown methods must fail closed.
  assert.strictEqual(resp.error?.code, 404, 'unknown methods must return 404 in strict default mode');
  assert.ok(
    resp.error.message.includes('MCP unknown method rejected'),
    'unknown-method response must mention explicit rejection'
  );

  // simulate a CLINE probe such as ctx.sample - strict mode should reject it.
  const probe = await sendRequest(proc, { method: 'ctx.sample', params: {} });
  assert.strictEqual(probe.error?.code, 404, 'ctx.sample must be rejected in strict default mode');

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

  const builderWaveApply = await sendRequest(proc, {
    method: 'mrt_wave_apply',
    params: {},
    apiKey: builderToken
  });
  assert.strictEqual(builderWaveApply.error?.code, 403, 'builder must not call mrt_wave_apply');

  const adminLspMissingSource = await sendRequest(proc, {
    method: 'mrt_lsp_diagnostics',
    params: {},
    apiKey: 'ADMIN'
  });
  assert.strictEqual(adminLspMissingSource.error?.code, 400, 'admin mrt_lsp_diagnostics without source must return 400');
  assert.ok(adminLspMissingSource.error.message.includes('missing_source'));

  const committerWaveDryRun = await sendRequest(proc, {
    method: 'mrt_wave_dry_run',
    params: {},
    apiKey: committerToken
  });
  assert.notStrictEqual(committerWaveDryRun.error?.code, 401, 'committer mrt_wave_dry_run must pass auth layer');
  assert.notStrictEqual(committerWaveDryRun.error?.code, 403, 'committer mrt_wave_dry_run must pass auth layer');

  const committerGeneralCiCompile = await sendRequest(proc, {
    method: 'mrt_general_ci_compile',
    params: {},
    apiKey: committerToken
  });
  assert.strictEqual(committerGeneralCiCompile.error?.code, 403, 'committer must not call mrt_general_ci_compile');

  const builderGeneralCiCompile = await sendRequest(proc, {
    method: 'mrt_general_ci_compile',
    params: {},
    apiKey: builderToken
  });
  assert.notStrictEqual(builderGeneralCiCompile.error?.code, 401, 'builder mrt_general_ci_compile must pass auth layer');
  assert.notStrictEqual(builderGeneralCiCompile.error?.code, 403, 'builder mrt_general_ci_compile must pass role gate');

  const builderGeneralCiFast = await sendRequest(proc, {
    method: 'mrt_general_ci_fast',
    params: {},
    apiKey: builderToken
  });
  assert.notStrictEqual(builderGeneralCiFast.error?.code, 401, 'builder mrt_general_ci_fast must pass auth layer');
  assert.notStrictEqual(builderGeneralCiFast.error?.code, 403, 'builder mrt_general_ci_fast must pass role gate');

  const adminBrainGet = await sendRequest(proc, {
    method: 'mrt_brain_get',
    params: { key: 'proposal-096' },
    apiKey: 'ADMIN'
  });
  assert.ok(adminBrainGet.result, 'admin mrt_brain_get should succeed with result');
  assert.strictEqual(adminBrainGet.result.exitCode, 0);
  assert.strictEqual(adminBrainGet.result.tool, 'mrt_brain_get');
  assert.strictEqual(adminBrainGet.result.output_limit_bytes, 65536);
  assert.ok(Object.prototype.hasOwnProperty.call(adminBrainGet.result, 'stdout_truncated'));
  assert.strictEqual(typeof adminBrainGet.result.stdout_truncated, 'boolean');
  assert.ok(Object.prototype.hasOwnProperty.call(adminBrainGet.result, 'stderr_truncated'));
  assert.strictEqual(typeof adminBrainGet.result.stderr_truncated, 'boolean');
  const adminBrainPayload = JSON.parse(adminBrainGet.result.stdout);
  assert.strictEqual(adminBrainPayload.backend, 'kb-data');
  assert.strictEqual(adminBrainPayload.result_limit, 16);
  assert.strictEqual(adminBrainPayload.entry_size_limit, 4096);
  assert.ok(Object.prototype.hasOwnProperty.call(adminBrainPayload, 'graph_db_present'));
  assert.strictEqual(typeof adminBrainPayload.graph_db_present, 'boolean');

  const unknownTool = await sendRequest(proc, {
    method: 'mrt_execute',
    params: { tool: 'mrt_unknown_tool', args: [] },
    apiKey: 'ADMIN'
  });
  assert.strictEqual(unknownTool.error?.code, 410, 'mrt_execute compatibility wrapper must be disabled by default');
  assert.ok(unknownTool.error.message.includes('mrt_execute_compat_disabled'));

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

  const compat = await startServer([], { MRT_COMPAT_UNKNOWN_METHODS: '1' });
  await waitForServerReady(compat.proc);
  const compatUnknown = await sendRequest(compat.proc, {
    method: 'no_such_method_compat',
    params: {}
  });
  assert.strictEqual(compatUnknown.result, null, 'compat mode unknown method must return null result');
  await stopServer(compat.proc);

  // garbage input shouldn't crash
  const p = await startServer();
  await waitForServerReady(p.proc);
  p.proc.stdin.write('notjson\n');
  await new Promise(r => setTimeout(r, 100));
  await stopServer(p.proc);

    await testConnReset();

    console.log('all tests passed');
  } finally {
    try {
      if (cfgBackup === null) {
        if (fs.existsSync(cfgPath)) {
          fs.unlinkSync(cfgPath);
        }
      } else {
        fs.writeFileSync(cfgPath, cfgBackup);
      }
    } catch (e) {
      console.error('failed to restore config.json after tests', e);
    }
  }
}

runTests().catch(err => {
  console.error('tests failed', err);
  process.exit(1);
});

