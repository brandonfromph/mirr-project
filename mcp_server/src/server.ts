import express from "express";
import bodyParser from "body-parser";
import { promises as fs } from "fs";
import path from "path";
import { promisify } from "util";
import { execFile } from "child_process";
import glob from "glob";

const execFileAsync = promisify(execFile);
const globAsync = promisify(glob);


function getApiKeyFromReq(req: express.Request) {
  const h = (req.headers["x-mcp-api-key"] || req.headers["authorization"] || "").toString();
  if (!h) return null;
  if (h.startsWith("Bearer ")) return h.slice(7);
  return h;
}
function requireRole(req: express.Request, roles: string[]) {
  const token = getApiKeyFromReq(req);
  if (!token) {
    console.error('requireRole: no token found in headers', req.headers);
    return { ok: false, reason: "missing_api_key" };
  }
  const entry = verifyApiKey(token);
  if (!entry) return { ok: false, reason: "invalid_api_key" };
  if (!roles.includes(entry.role)) return { ok: false, reason: "insufficient_role", role: entry.role };
  return { ok: true, entry };
}

// Wave 2 contract: auth verdict is bound to MRT tool dispatch policy.
function requireMrtDispatchRole(req: express.Request, toolName: string) {
  const toolRoleAllowlist: Record<string, string[]> = {
    mrt_audit: ["builder", "committer", "admin"],
    mrt_brain_get: ["committer", "admin"],
    mrt_general_ci: ["builder", "admin"],
  };
  const allowed = toolRoleAllowlist[toolName];
  if (!allowed) {
    return { ok: false, reason: "unknown_tool" };
  }
  return requireRole(req, allowed);
}

type MrtDispatchTool = "mrt_audit" | "mrt_brain_get" | "mrt_general_ci";

function getBodyString(body: unknown, key: string, fallback = ""): string {
  if (!body || typeof body !== "object") {
    return fallback;
  }
  const value = (body as Record<string, unknown>)[key];
  return typeof value === "string" ? value : fallback;
}

function resolveMrtInvocation(toolName: MrtDispatchTool, body: unknown): { bin: string; args: string[] } {
  if (toolName === "mrt_audit") {
    const mode = getBodyString(body, "mode", "workspace");
    const globExpr = getBodyString(body, "glob", "src/**/*.rs");
    return {
      bin: "mirr-audit",
      args: ["--mode", mode, "--glob", globExpr, "--format", "json"],
    };
  }

  if (toolName === "mrt_brain_get") {
    const key = getBodyString(body, "key");
    if (key.length === 0) {
      throw new Error("missing_key");
    }
    return {
      bin: "mirr-brain",
      args: ["get", "--key", key, "--format", "json"],
    };
  }

  return {
    bin: "mirr-general",
    args: ["ci", "--format", "json"],
  };
}

async function handleMrtDispatch(toolName: MrtDispatchTool, req: express.Request, res: express.Response) {
  const rr = requireMrtDispatchRole(req, toolName);
  if (!rr.ok) {
    const code = rr.reason === "missing_api_key" ? 401 : rr.reason === "unknown_tool" ? 400 : 403;
    return res.status(code).json({ error: rr.reason, role: (rr as any).role ?? null });
  }

  try {
    const invocation = resolveMrtInvocation(toolName, req.body || {});
    const execResult = await withConcurrencyLimit(req, async () => {
      return await execFileAsync(
        "cargo",
        ["run", "--bin", invocation.bin, "--", ...invocation.args],
        {
          cwd: WORKSPACE_ROOT,
          maxBuffer: 20 * 1024 * 1024,
          timeout: DEFAULT_TIMEOUT,
        }
      );
    });

    const stdout = typeof (execResult as any).stdout === "string" ? (execResult as any).stdout.trim() : "";
    const stderr = typeof (execResult as any).stderr === "string" ? (execResult as any).stderr.trim() : "";
    return res.json({
      schema_version: "1",
      tool: toolName,
      args: invocation.args,
      exitCode: 0,
      stdout,
      stderr,
    });
  } catch (err: any) {
    if (err && err.message === "concurrency_limit_exceeded") {
      return res.status(429).json({ error: err.message });
    }
    if (err && err.message === "missing_key") {
      return res.status(400).json({ error: "missing_key" });
    }
    const stderr = err?.stderr?.toString?.() ?? "";
    const stdout = err?.stdout?.toString?.() ?? "";
    const details = stderr || stdout || String(err?.message || err);
    return res.status(400).json({ error: "mrt_exec_failed", details });
  }
}

// Allowed paths/commands (from config) helpers
function isPathAllowed(relPath: string) {
  const allowed = Array.isArray(CONFIG.allowed_paths) ? CONFIG.allowed_paths : ["."];
  const abs = path.resolve(WORKSPACE_ROOT, relPath || ".");
  for (const p of allowed) {
    const ap = path.resolve(WORKSPACE_ROOT, p);
    if (abs.toLowerCase().startsWith(ap.toLowerCase())) return true;
  }
  return false;
}
function isCommandAllowed(kind: "cargo" | "executable", value: string) {
  if (kind === "cargo") {
    const list = CONFIG.allowed_commands?.cargo ?? ["build", "test", "check"];
    return list.includes(value);
  } else {
    const exes = CONFIG.allowed_commands?.executables ?? [];
    // match by basename or relative path
    const name = path.basename(value);
    return exes.includes(name) || exes.includes(value);
  }
}

const app = express();
app.use(bodyParser.json({ limit: "5mb" }));

// ----- JSON schema validation setup (Phase 1) -----
import Ajv, { ValidateFunction } from "ajv";
const ajv = new Ajv({ allErrors: true, strict: false });

const schemas: Record<string, any> = {
  read_text_file: {
    type: "object",
    properties: {
      path: { type: "string" },
      head: { type: "number" },
      tail: { type: "number" }
    },
    required: ["path"],
    additionalProperties: false,
  },
  write_file: {
    type: "object",
    properties: {
      path: { type: "string" },
      content: { type: "string" },
      dry_run: { type: "boolean" },
      prechecks: { type: "array", items: { type: "string" } },
      commit: { type: "boolean" },
      commit_message: { type: "string" }
    },
    required: ["path", "content"],
    additionalProperties: false,
  },
};

const validators: Record<string, ValidateFunction> = {};
for (const name in schemas) {
  validators[name] = ajv.compile(schemas[name]);
}

function validateBody(name: string) {
  return (req: express.Request, res: express.Response, next: express.NextFunction) => {
    const validator = validators[name];
    if (!validator) return next();
    const ok = validator(req.body);
    if (!ok) {
      // convert AJV errors to a simpler message
      const msg = validator.errors?.map((e: any) => `${e.instancePath} ${e.message}`).join(", ");
      return res.status(400).json({ error: "schema_validation", details: msg });
    }
    next();
  };
}


// debug middleware: log every incoming request path/method to stderr.  helps
// diagnose unexpected connection resets during tests.
app.use((req: any, res: any, next: any) => {
  console.error('server received request', req.method, req.path);
  next();
});

// ---------- helper for stdio‑direct mode ----------
// we will capture registered routes so the same handlers can be invoked
// directly when communicating over stdio instead of HTTP.
type RouteHandler = (
  req: express.Request,
  res: express.Response,
  next?: express.NextFunction,
) => any;

const handlers: Record<string, (req: express.Request, res: express.Response) => Promise<void>> = {};

async function runHandlerChain(
  routeHandlers: RouteHandler[],
  req: express.Request,
  res: express.Response,
): Promise<void> {
  for (const handler of routeHandlers) {
    if (handler.length >= 3) {
      let calledNext = false;
      await new Promise<void>((resolve, reject) => {
        const next: express.NextFunction = (err?: any) => {
          calledNext = true;
          if (err) {
            reject(err);
            return;
          }
          resolve();
        };

        try {
          const maybe = handler(req, res, next);
          if (maybe && typeof maybe.then === "function") {
            maybe.then(() => {
              if (!calledNext) {
                resolve();
              }
            }).catch(reject);
            return;
          }
          if (!calledNext) {
            resolve();
          }
        } catch (err) {
          reject(err);
        }
      });

      // Middleware without next() ended the chain.
      if (!calledNext) {
        break;
      }
      continue;
    }

    const maybe = handler(req, res);
    if (maybe && typeof maybe.then === "function") {
      await maybe;
    }
    break;
  }
}

// override app.post/get since those are the only verbs we use in this server
const originalPost = app.post.bind(app);
(app as any).post = (routePath: string, ...routeHandlers: RouteHandler[]) => {
  const method = routePath.replace(/^\//, "");
  handlers[method] = async (req: express.Request, res: express.Response) => {
    await runHandlerChain(routeHandlers, req, res);
  };
  return originalPost(routePath, ...routeHandlers);
};
const originalGet = app.get.bind(app);
(app as any).get = (routePath: string, ...routeHandlers: RouteHandler[]) => {
  const method = routePath.replace(/^\//, "");
  handlers[method] = async (req: express.Request, res: express.Response) => {
    await runHandlerChain(routeHandlers, req, res);
  };
  return originalGet(routePath, ...routeHandlers);
};




// Simple MCP schema description so clients (such as CLINE) can display
// human-readable tool information. This mirrors the built-in servers' metadata.
app.post("/mcp_schema", (_req, res) => {
  const schema = {
    name: "local_custom",
    methods: {
      read_text_file: {
        autoApprove: true,
        description: "Read the complete contents of a file from the file system as text. " +
          "Handles various encodings and provides detailed errors. Use head/tail parameters.",
        parameters: [
          { name: "path", required: true, type: "string" },
          { name: "head", required: false, type: "number" },
          { name: "tail", required: false, type: "number" },
        ],
      },
      write_file: {
        autoApprove: true,
        description: "Create or overwrite a file with new content. Operates within allowed dirs.",
        parameters: [
          { name: "path", required: true, type: "string" },
          { name: "content", required: true, type: "string" },
        ],
      },
      edit_file: {
        autoApprove: true,
        description: "Perform line-based edits to a text file and return a git-style diff.",
        parameters: [
          { name: "path", required: true, type: "string" },
          { name: "edits", required: true, type: "array" },
          { name: "dryRun", required: false, type: "boolean" },
        ],
      },
      create_directory: {
        autoApprove: true,
        description: "Ensure a directory exists, creating parent directories as needed.",
        parameters: [{ name: "path", required: true, type: "string" }],
      },
      list_directory: {
        autoApprove: true,
        description: "List files and directories at a path, marking types.",
        parameters: [{ name: "path", required: true, type: "string" }],
      },
      list_directory_with_sizes: {
        autoApprove: true,
        description: "Like list_directory but include sizes, with optional sorting.",
        parameters: [
          { name: "path", required: true, type: "string" },
          { name: "sortBy", required: false, type: "string" },
        ],
      },
      directory_tree: {
        autoApprove: true,
        description: "Return JSON tree of directories/files recursively.",
        parameters: [
          { name: "path", required: true, type: "string" },
          { name: "excludePatterns", required: false, type: "array" },
        ],
      },
      move_file: {
        autoApprove: true,
        description: "Move or rename a file or directory within allowed paths.",
        parameters: [
          { name: "source", required: true, type: "string" },
          { name: "destination", required: true, type: "string" },
        ],
      },
      search_files: {
        autoApprove: true,
        description: "Recursively search using glob patterns starting at path.",
        parameters: [
          { name: "path", required: true, type: "string" },
          { name: "pattern", required: false, type: "string" },
          { name: "excludePatterns", required: false, type: "array" },
        ],
      },
      get_file_info: {
        autoApprove: true,
        description: "Retrieve metadata about a file or directory.",
        parameters: [{ name: "path", required: true, type: "string" }],
      },
      list_allowed_directories: {
        autoApprove: true,
        description: "Return directories the server may access according to config.",
        parameters: [],
      },
      mrt_audit: {
        autoApprove: false,
        description: "Run mirr-audit with MRT role allowlist enforcement.",
        parameters: [
          { name: "mode", required: false, type: "string" },
          { name: "glob", required: false, type: "string" },
        ],
      },
      mrt_brain_get: {
        autoApprove: false,
        description: "Run mirr-brain get with MRT role allowlist enforcement.",
        parameters: [{ name: "key", required: true, type: "string" }],
      },
      mrt_general_ci: {
        autoApprove: false,
        description: "Run mirr-general ci with MRT role allowlist enforcement.",
        parameters: [],
      },
    },
  };
  res.json(schema);
});

 // Workspace root (default: repository root two levels up from this file)
const WORKSPACE_ROOT = (process.env.MCP_WORKSPACE_ROOT || path.resolve(__dirname, "..", "..")).toString().trim();
const WORKSPACE_ROOT_LOWER = WORKSPACE_ROOT.toLowerCase();

// Load runtime config and auth keys (config file located at mcp_server/config.json by default)
const fsSync = require('fs');
const CONFIG = (function () {
  try {
    const p = path.join(WORKSPACE_ROOT, "mcp_server", "config.json");
    if (fsSync.existsSync(p)) return JSON.parse(fsSync.readFileSync(p, "utf8"));
  } catch (e) {}
  return {};
})();

  // API key maps (raw tokens and hashed tokens)
const RAW_API_KEYS: Record<string, { id: string; role: string }> = {};
const HASHED_API_KEYS: Record<string, { id: string; role: string }> = {};

// persistent configuration helpers
function configPath() {
  return path.join(WORKSPACE_ROOT, "mcp_server", "config.json");
}

function writeConfig() {
  try {
    fsSync.writeFileSync(configPath(), JSON.stringify(CONFIG, null, 2));
  } catch (e) {
    console.error('failed to write config', e);
  }
}

function sha256hex(s: string) {
  const crypto = require("crypto");
  return crypto.createHash("sha256").update(s).digest("hex");
}

if (Array.isArray(CONFIG.api_keys)) {
  for (const k of CONFIG.api_keys) {
    if (!k || !k.id || !k.role) continue;
    // config may store hashed entries using { token: "<hex>", hashed: true }
    if (k.hashed) {
      if (k.token) HASHED_API_KEYS[k.token] = { id: k.id, role: k.role };
    } else {
      if (k.token) RAW_API_KEYS[k.token] = { id: k.id, role: k.role };
    }
  }
}

function verifyApiKey(token: string) {
  if (!token) return null;
  // direct match first
  const raw = RAW_API_KEYS[token];
  if (raw) return raw;
  // check hashed entries
  try {
    const h = sha256hex(token);
    const hh = HASHED_API_KEYS[h];
    if (hh) return hh;
  } catch (e) {}
  return null;
}

// Concurrency & timeout helpers
const CONCURRENCY: Record<string, number> = {};
const MAX_CONCURRENT_PER_KEY = Number(CONFIG.max_concurrent_per_key ?? 2);
const DEFAULT_TIMEOUT = Number(CONFIG.timeouts?.default_ms ?? 120000);

// helper used by the server in stdio‑direct mode.  It reads line‑delimited
// JSONRPC requests from stdin and dispatches to the same handlers that back
// the express routes (HTTP routes are defined but never bound in this mode).
  function startStdioServer() {
    process.stdin.setEncoding('utf8');
    function sendRpc(resp: any) {
      // Normalize/wrap into a JSON-RPC 2.0 envelope if caller provided a bare object.
      if (!resp || typeof resp !== 'object') return;
      if (!('jsonrpc' in resp)) resp.jsonrpc = '2.0';
      // Do not emit responses with missing/null id to stdout (these are notifications).
      if (resp.id === undefined || resp.id === null) {
        console.error('stdio: skipping sendRpc for notification or missing id:', JSON.stringify(resp));
        return;
      }
      try {
        const json = JSON.stringify(resp);
        // Only JSON-RPC messages are written to stdout; diagnostics remain on stderr.
        process.stdout.write(json + '\n');
      } catch (e) {
        console.error('stdio: failed to stringify response', e, resp);
      }
    }
    let buffer = '';

  process.stdin.on('data', chunk => {
    buffer += chunk;
    let idx;
    while ((idx = buffer.indexOf('\n')) >= 0) {
      const line = buffer.slice(0, idx).trim();
      buffer = buffer.slice(idx + 1);
      if (!line) continue;
      let msg: any;
      try { msg = JSON.parse(line); } catch {
        continue; // ignore invalid JSON
      }
      handleStdIoMessage(msg);
    }
  });

  async function handleStdIoMessage(msg: { id?: number; method?: string; params?: any; apiKey?: string }) {
    const base = { jsonrpc: '2.0', id: msg.id };

    // Test-only hook: simulate connection reset behavior expected by stdio proxy tests.
    if (process.env.MCP_TEST_FORCE_RESET === '1') {
      sendRpc({
        ...base,
        error: {
          code: -32000,
          message: 'ECONNRESET simulated by MCP_TEST_FORCE_RESET',
        },
      });
      return;
    }

    // Attempt to normalize incoming JSON-RPC method names so clients using
    // different naming conventions (camelCase, PascalCase, snake_case) can
    // invoke the server's handlers without hitting "method not found".
    let methodName: string | undefined = msg.method;
    if (!methodName) {
      console.error('stdio: missing method in message', msg);
      sendRpc({ ...base, result: null });
      return;
    }

    // If exact match exists, use it.
    if (!(methodName in handlers)) {
      // camelCase or PascalCase -> snake_case (e.g. readTextFile -> read_text_file)
      const camelToSnake = methodName.replace(/([A-Z])/g, '_$1').toLowerCase();
      if (camelToSnake in handlers) methodName = camelToSnake;
      else {
        // replace spaces/dashes with underscores
        const spaced = methodName.replace(/[\s\-]+/g, '_').toLowerCase();
        if (spaced in handlers) methodName = spaced;
        else {
          // common MCP-style mappings
          if (methodName === 'ListTools' || methodName === 'listTools' || methodName === 'List_Tools') {
            methodName = 'mcp_schema';
          } else if (methodName === 'ListResources' || methodName === 'listResources') {
            methodName = 'resources/templates/list';
          } else if ((methodName === 'CallTool' || methodName === 'callTool') && msg.params && typeof msg.params.name === 'string') {
            // Map CallTool to the underlying tool name provided in params.name
            // (this server exposes tools by their route name)
            methodName = msg.params.name;
          }
        }
      }
    }

    if (!methodName || !(methodName in handlers)) {
      // keep the previous compatibility behavior: return null result instead of JSON-RPC error
      console.error('stdio: unknown method', msg.method, '->', methodName, '- returning null result for compatibility');
      sendRpc({ ...base, result: null });
      return;
    }

    const req: express.Request = { headers: {}, body: msg.params || {}, query: msg.params || {} } as any;
    if (msg.apiKey) {
      // inject both header variants so getApiKeyFromReq will pick it up
      req.headers!["x-mcp-api-key"] = msg.apiKey;
      req.headers!["authorization"] = "Bearer " + msg.apiKey;
    }
    const res: express.Response = {
      statusCode: 200,
      status(code: number) { this.statusCode = code; return this; },
      json(obj: any) { send(msg.id, this.statusCode, obj); return this; },
      send(obj: any) { send(msg.id, this.statusCode, obj); return this; },
      setHeader(name: string, value: string) { return this; },
    } as any;

    try {
      const handler = handlers[methodName];
      if (handler) await handler(req, res);
    } catch (err) {
      send(msg.id, 500, { error: String(err) });
    }
  }

  function send(id: any, status: number, body: any) {
    const resp: any = { jsonrpc: '2.0', id };
    if (status >= 200 && status < 300) {
      resp.result = body;
    } else {
      resp.error = { code: status, message: typeof body === 'string' ? body : JSON.stringify(body) };
    }
    sendRpc(resp);
  }
}

function getToken(req: any) {
  return getApiKeyFromReq(req) || "__anon";
}

async function withConcurrencyLimit(req: any, fn: () => Promise<any>) {
  const token = getToken(req);
  CONCURRENCY[token] = (CONCURRENCY[token] || 0) + 1;
  if (CONCURRENCY[token] > MAX_CONCURRENT_PER_KEY) {
    CONCURRENCY[token]--;
    throw new Error("concurrency_limit_exceeded");
  }
  try {
    return await fn();
  } finally {
    CONCURRENCY[token]--;
  }
}

// Utility: ensure resolvedPath is inside allowed workspace root
function resolveSafe(relPath: string) {
  const candidate = path.resolve(WORKSPACE_ROOT, relPath || ".");
  if (!candidate.toLowerCase().startsWith(WORKSPACE_ROOT_LOWER)) {
    throw new Error("Path outside allowed workspace");
  }
  return candidate;
}

function linesSlice(content: string, head?: number, tail?: number) {
  const lines = content.split(/\r?\n/);
  if (typeof head === "number") return lines.slice(0, head).join("\n");
  if (typeof tail === "number") return lines.slice(Math.max(lines.length - tail, 0)).join("\n");
  return content;
}

// ---------------------------------------------------------------------------
// compatibility stubs for optional/probed methods
// ---------------------------------------------------------------------------
app.post("/ctx_sample", async (_req, res) => {
  // CLINE and other clients probe this method; return null so they stay happy
  res.json(null);
});

// debug helper - list registered handlers (useful in stdio-direct mode)
app.get("/list_handlers", (_req, res) => {
  res.json({ handlers: Object.keys(handlers) });
});

// admin API‑key management endpoints
// only admin role may create/revoke keys
app.post("/generate_api_key", (req, res) => {
  const rr = requireRole(req, ["admin"]);
  if (!rr.ok) {
    const code = rr.reason === "missing_api_key" ? 401 : 403;
    res.status(code).json({ error: rr.reason });
    return;
  }
  const { id, role } = req.body;
  if (!id || !role) {
    res.status(400).json({ error: "missing_id_or_role" });
    return;
  }
  const crypto = require("crypto");
  const token = crypto.randomBytes(16).toString("hex");
  CONFIG.api_keys = CONFIG.api_keys || [];
  CONFIG.api_keys.push({ id, role, token });
  RAW_API_KEYS[token] = { id, role };
  writeConfig();
  res.json({ token });
});

app.get("/list_api_keys", (req, res) => {
  const rr = requireRole(req, ["admin"]);
  if (!rr.ok) {
    const code = rr.reason === "missing_api_key" ? 401 : 403;
    res.status(code).json({ error: rr.reason });
    return;
  }
  const list = (CONFIG.api_keys || []).map((k: any) => ({ id: k.id, role: k.role }));
  res.json({ keys: list });
});

app.post("/revoke_api_key", (req, res) => {
  const rr = requireRole(req, ["admin"]);
  if (!rr.ok) {
    const code = rr.reason === "missing_api_key" ? 401 : 403;
    res.status(code).json({ error: rr.reason });
    return;
  }
  const { token, id } = req.body;
  if (!token && !id) {
    res.status(400).json({ error: "missing_token_or_id" });
    return;
  }
  CONFIG.api_keys = (CONFIG.api_keys || []).filter((k: any) => {
    if (token && k.token === token) return false;
    if (id && k.id === id) return false;
    return true;
  });
  if (token) delete RAW_API_KEYS[token];
  // if hashed there would be additional removal logic
  writeConfig();
  res.json({ ok: true });
});

// Tool: long_running (simulation endpoint for concurrency testing)
// NASA "10 power" philosophy: expect tasks to take 10x longer and throttle
// accordingly.  This handler purposely sleeps so we can hit the limit.
app.post("/long_running", async (req, res) => {
  try {
    await withConcurrencyLimit(req, async () => {
      // simulate work
      await new Promise(r => setTimeout(r, 50));
      res.json({ ok: true });
    });
  } catch (err: any) {
    if (err.message === "concurrency_limit_exceeded") {
      // 429 Too Many Requests is semantically appropriate
      res.status(429).json({ error: "concurrency_limit_exceeded" });
      return;
    }
    // unexpected error bubbling
    res.status(500).json({ error: String(err) });
  }
});

// MRT tool routes (active runtime path for stdio CallTool dispatch).
app.post("/mrt_audit", async (req, res) => {
  await handleMrtDispatch("mrt_audit", req, res);
});

app.post("/mrt_brain_get", async (req, res) => {
  await handleMrtDispatch("mrt_brain_get", req, res);
});

app.post("/mrt_general_ci", async (req, res) => {
  await handleMrtDispatch("mrt_general_ci", req, res);
});

app.post("/mrt_execute", async (req, res) => {
  const tool = getBodyString(req.body, "tool") as MrtDispatchTool;
  if (tool !== "mrt_audit" && tool !== "mrt_brain_get" && tool !== "mrt_general_ci") {
    return res.status(400).json({ error: "unknown_tool" });
  }
  return handleMrtDispatch(tool, req, res);
});

app.post("/resources/templates/list", async (_req, res) => {
  // placeholder implementation; real templates can be added later
  res.json({ templates: [] });
});

// Tool: read_text_file
app.post("/read_text_file", validateBody("read_text_file"), async (req, res) => {
  try {
    const { path: relPath, head, tail } = req.body;
    if (!isPathAllowed(relPath || ".")) return res.status(403).json({ error: "path_not_allowed" });
    const abs = resolveSafe(relPath || ".");
    const stat = await fs.stat(abs);
    if (!stat.isFile()) return res.status(400).json({ error: "Not a file" });
    const content = await fs.readFile(abs, "utf-8");
    return res.json({ path: abs, content: linesSlice(content, head, tail) });
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

  // Tool: write_file
app.post("/write_file", validateBody("write_file"), async (req, res) => {
  try {
    // RBAC: only users with 'committer' or 'admin' may perform writes
    const rr = requireRole(req, ["committer", "admin"]);
    if (!rr.ok) {
      const code = rr.reason === "missing_api_key" ? 401 : 403;
      return res.status(code).json({ error: rr.reason, role: (rr as any).role ?? null });
    }
    const { path: relPath, content, dry_run = false, prechecks = [], commit = false, commit_message } = req.body;
    if (typeof content !== "string") return res.status(400).json({ error: "Missing content" });
    if (!isPathAllowed(relPath)) return res.status(403).json({ error: "path_not_allowed" });
    const abs = resolveSafe(relPath);
    // ensure directories
    await fs.mkdir(path.dirname(abs), { recursive: true });
    const backupsDir = path.join(WORKSPACE_ROOT, ".mcp_backups");
    await fs.mkdir(backupsDir, { recursive: true });

    // create backup if file exists
    const exists = await fs.stat(abs).then(s => s.isFile()).catch(() => false);
    const now = new Date().toISOString().replace(/[:.]/g, "-");
    const safeName = (relPath || "unnamed").replace(/[\\/]/g, "_");
    const backupPath = path.join(backupsDir, `${now}-${safeName}.bak`);
    if (exists) await fs.copyFile(abs, backupPath);

    // dry-run preview
    if (dry_run) {
      const preview = content.length > 10000 ? content.slice(0, 10000) + "\n...[truncated]" : content;
      return res.json({ path: abs, dry_run: true, preview });
    }

    // write new content
    await fs.writeFile(abs, content, "utf-8");

    // pre-change checks: run chosen checks and restore backup on failure
    async function restoreOnFail(errMsg: string, details?: string) {
      if (exists) {
        await fs.copyFile(backupPath, abs).catch(() => {});
      } else {
        await fs.unlink(abs).catch(() => {});
      }
      return res.status(400).json({ error: errMsg, details });
    }

    if (Array.isArray(prechecks) && prechecks.length > 0) {
      if (prechecks.includes("rustfmt")) {
        try {
          await execFileAsync("cargo", ["fmt", "--", "-q"], { cwd: WORKSPACE_ROOT, timeout: DEFAULT_TIMEOUT });
        } catch (e: any) {
          return await restoreOnFail("rustfmt failed", e.stderr?.toString?.() ?? e.message);
        }
      }
      if (prechecks.includes("cargo_test")) {
        try {
          await execFileAsync("cargo", ["test"], { cwd: WORKSPACE_ROOT, maxBuffer: 20 * 1024 * 1024, timeout: DEFAULT_TIMEOUT });
        } catch (e: any) {
          return await restoreOnFail("cargo test failed", e.stderr?.toString?.() ?? e.message);
        }
      }
      if (prechecks.includes("nasa_lint")) {
        // Placeholder: run clippy as a strict lint; adapt if you have a custom linter
        try {
          await execFileAsync("cargo", ["clippy", "--", "-D", "warnings"], { cwd: WORKSPACE_ROOT, maxBuffer: 20 * 1024 * 1024, timeout: DEFAULT_TIMEOUT });
        } catch (e: any) {
          return await restoreOnFail("nasa_lint (clippy) failed", e.stderr?.toString?.() ?? e.message);
        }
      }
    }

    // audit log
    const logsDir = path.join(WORKSPACE_ROOT, ".mcp_logs");
    await fs.mkdir(logsDir, { recursive: true });
    const who = req.headers["x-mcp-user"] ?? "mcp";
    const logEntry = `${new Date().toISOString()} ${who} WRITE ${relPath} commit=${!!commit} message=${commit_message ?? ""}\n`;
    await fs.appendFile(path.join(logsDir, "edits.log"), logEntry, "utf-8");

    // optional git add/commit
    if (commit) {
      try {
        await execFileAsync("git", ["add", relPath], { cwd: WORKSPACE_ROOT });
        const cm = commit_message || `MCP write ${relPath} ${now}`;
        await execFileAsync("git", ["commit", "-m", cm], { cwd: WORKSPACE_ROOT });
      } catch (e: any) {
        // do not revert file on commit failure; surface git error
        return res.json({ ok: true, path: abs, wrote: true, gitError: e.stderr?.toString?.() ?? e.message });
      }
    }

    return res.json({ ok: true, path: abs, wrote: true });
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// Tool: list_directory
app.post("/list_directory", async (req, res) => {
  try {
    const { path: relPath } = req.body;
    if (!isPathAllowed(relPath || ".")) return res.status(403).json({ error: "path_not_allowed" });
    const abs = resolveSafe(relPath || ".");
    const entries = await fs.readdir(abs);
    const results = await Promise.all(
      entries.map(async (name) => {
        const p = path.join(abs, name);
        const s = await fs.stat(p);
        return { name, path: p, type: s.isDirectory() ? "dir" : "file", size: s.size };
      })
    );
    return res.json({ path: abs, entries: results });
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// Tool: directory_tree
async function treeFor(dir: string, exclude: string[] = []): Promise<any> {
  const name = path.basename(dir);
  const stat = await fs.stat(dir);
  if (!stat.isDirectory()) return { name, type: "file" };
  const childrenNames = await fs.readdir(dir);
  const children = [];
  for (const child of childrenNames) {
    const childPath = path.join(dir, child);
    const rel = path.relative(WORKSPACE_ROOT, childPath);
    if (exclude.some((pat) => rel.includes(pat))) continue;
    children.push(await treeFor(childPath, exclude));
  }
  return { name, type: "dir", children };
}
app.post("/directory_tree", async (req, res) => {
  try {
    const { path: relPath, excludePatterns } = req.body;
    if (!isPathAllowed(relPath || ".")) return res.status(403).json({ error: "path_not_allowed" });
    const abs = resolveSafe(relPath || ".");
    const tree = await treeFor(abs, Array.isArray(excludePatterns) ? excludePatterns : []);
    return res.json({ root: abs, path: abs, tree });
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// Tool: search_files (glob-style pattern relative to given path)
app.post("/search_files", async (req, res) => {
  try {
    const { path: relPath, pattern = "**/*", ignore = [] } = req.body;
    if (!isPathAllowed(relPath || ".")) return res.status(403).json({ error: "path_not_allowed" });
    const absBase = resolveSafe(relPath || ".");
    const opts = { cwd: absBase, nodir: true, ignore };
    const matches = await globAsync(pattern, opts as any);
    const fullPaths = matches.map((p) => path.resolve(absBase, p));
    return res.json({ base: absBase, pattern, matches: fullPaths });
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// Tool: run_cargo (allowed subcommands only)
const ALLOWED_CARGO = new Set(["build", "test", "check"]);
app.post("/run_cargo", async (req, res) => {
  try {
    // RBAC: require builder/committer/admin to run cargo commands via the MCP
    const rr = requireRole(req, ["builder", "committer", "admin"]);
    if (!rr.ok) {
      const code = rr.reason === "missing_api_key" ? 401 : 403;
      return res.status(code).json({ error: rr.reason, role: (rr as any).role ?? null });
    }
    const { subcommand, args = [] } = req.body;
    if (!isCommandAllowed("cargo", subcommand)) return res.status(403).json({ error: "cargo_subcommand_not_allowed" });
    if (!ALLOWED_CARGO.has(subcommand)) return res.status(400).json({ error: "Disallowed cargo subcommand" });
    // Execute cargo in workspace root with timeout and concurrency limits
    try {
      const result = await withConcurrencyLimit(req, async () => {
        return await execFileAsync("cargo", [subcommand, ...args], { cwd: WORKSPACE_ROOT, maxBuffer: 10 * 1024 * 1024, timeout: DEFAULT_TIMEOUT });
      });
      const { stdout, stderr } = result as any;
      return res.json({ exitCode: 0, stdout, stderr });
    } catch (err: any) {
      const e: any = err;
      // if concurrency_limit_exceeded, surface as 429
      if (e && e.message === "concurrency_limit_exceeded") {
        return res.status(429).json({ error: e.message });
      }
      return res.json({ exitCode: e.code ?? 1, stdout: e.stdout?.toString?.() ?? "", stderr: e.stderr?.toString?.() ?? String(e.message) });
    }
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

 // Health
app.get("/health", (_req, res) => res.json({ ok: true, workspaceRoot: WORKSPACE_ROOT }));

// EDA: read netlist or IR JSON file (returns parsed JSON)
app.post("/read_netlist", async (req, res) => {
  try {
    const { path: relPath } = req.body;
    if (!isPathAllowed(relPath || "")) return res.status(403).json({ error: "path_not_allowed" });
    const abs = resolveSafe(relPath || "");
    const stat = await fs.stat(abs);
    if (!stat.isFile()) return res.status(400).json({ error: "Not a file" });
    const content = await fs.readFile(abs, "utf-8");
    try {
      const parsed = JSON.parse(content);
      return res.json({ path: abs, json: parsed });
    } catch (e: any) {
      // return raw content if not JSON
      return res.json({ path: abs, raw: content });
    }
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// EDA: run a local simulator binary inside workspace (executable must be inside workspace)
// POST body: { "executable": "relative/path/to/exe", "args": ["--flag"] }
app.post("/run_simulator", async (req, res) => {
  try {
    // RBAC: only 'builder' or 'admin' may run local simulator binaries
    const rr = requireRole(req, ["builder", "admin"]);
    if (!rr.ok) {
      const code = rr.reason === "missing_api_key" ? 401 : 403;
      return res.status(code).json({ error: rr.reason, role: (rr as any).role ?? null });
    }
    const { executable, args = [] } = req.body;
    if (!executable || typeof executable !== "string") return res.status(400).json({ error: "Missing executable path" });
    if (!isCommandAllowed("executable", executable)) return res.status(403).json({ error: "executable_not_allowed" });
    const exePath = resolveSafe(executable);
    const stat = await fs.stat(exePath);
    if (!stat.isFile()) return res.status(400).json({ error: "Executable not found" });
    // Exec file directly (no shell)
    try {
      const result = await withConcurrencyLimit(req, async () => {
        return await execFileAsync(exePath, Array.isArray(args) ? args : [], { cwd: WORKSPACE_ROOT, maxBuffer: 20 * 1024 * 1024, timeout: DEFAULT_TIMEOUT });
      });
      const { stdout, stderr } = result as any;
      return res.json({ exitCode: 0, stdout, stderr });
    } catch (e: any) {
      if (e && e.message === "concurrency_limit_exceeded") {
        return res.status(429).json({ error: e.message });
      }
      return res.json({ exitCode: e.code ?? 1, stdout: e.stdout?.toString?.() ?? "", stderr: e.stderr?.toString?.() ?? String(e.message) });
    }
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// EDA: crude resource estimator from a JSON netlist
// POST body: { "path": "relative/path/to/netlist.json" }
app.post("/estimate_resources", async (req, res) => {
  try {
    const { path: relPath } = req.body;
    if (!isPathAllowed(relPath || "")) return res.status(403).json({ error: "path_not_allowed" });
    const abs = resolveSafe(relPath || "");
    const content = await fs.readFile(abs, "utf-8");
    const json = JSON.parse(content);
    // Heuristic counts (adjust per repository formats)
    const nodeCount = Array.isArray(json.nodes) ? json.nodes.length : (json.cells ? Object.keys(json.cells).length : 0);
    const signalCount = Array.isArray(json.signals) ? json.signals.length : (json.wires ? Object.keys(json.wires).length : 0);
    const lutEstimate = Math.max(1, Math.round(nodeCount * 1.8));
    const regEstimate = Math.max(0, Math.round(signalCount * 1.0));
    return res.json({ path: abs, nodeCount, signalCount, estimates: { luts: lutEstimate, regs: regEstimate } });
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// EDA: parity / golden-check — compare two files (paths inside workspace)
// POST body: { "expected": "path/to/golden.json", "actual": "path/to/output.json" }
app.post("/parity_check", async (req, res) => {
  try {
    const { expected, actual } = req.body;
    if (!expected || !actual) return res.status(400).json({ error: "Provide expected and actual file paths" });
    if (!isPathAllowed(expected) || !isPathAllowed(actual)) return res.status(403).json({ error: "path_not_allowed" });
    const aPath = resolveSafe(expected);
    const bPath = resolveSafe(actual);
    const a = await fs.readFile(aPath, "utf-8");
    const b = await fs.readFile(bPath, "utf-8");
    if (a === b) return res.json({ equal: true, expected: aPath, actual: bPath });
    // simple line-level diff info
    const aLines = a.split(/\r?\n/);
    const bLines = b.split(/\r?\n/);
    const max = Math.max(aLines.length, bLines.length);
    let firstDiff = -1;
    for (let i = 0; i < max; i++) {
      if ((aLines[i] ?? "") !== (bLines[i] ?? "")) { firstDiff = i; break; }
    }
    return res.json({ equal: false, expected: aPath, actual: bPath, firstDiffLine: firstDiff, expectedLine: aLines[firstDiff] ?? null, actualLine: bLines[firstDiff] ?? null });
  } catch (err: any) {
    return res.status(400).json({ error: String(err.message || err) });
  }
});

// startup path: always stdio‑direct
console.error('MCP server operating in stdio-direct mode');
startStdioServer();


// global error handler (in case connection flood triggers errors)
app.use((err: any, _req: any, res: any, _next: any) => {
  console.error('uncaught handler error', err);
  if (!res.headersSent) res.status(503).json({ error: 'server overloaded' });
});
