import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  Tool,
} from "@modelcontextprotocol/sdk/types.js";
import { spawnSync } from "child_process";
import path from "path";
import {
  MAX_OUTPUT_BYTES,
  MAX_WAVE_LINES,
  brainGetArgs,
  clipOutput,
  generalCiCompileArgs,
  generalCiFastArgs,
  lspDiagnosticsInvocation,
  waveApplyArgs,
  waveDryRunArgs,
  type MrtDispatchTool,
} from "./mrt_kb_lite.js";

/**
 * MRT (MIRR Runtime Tooling) MCP Server
 * Bridges the Presidential Arsenal with the Gemini CLI.
 *
 * DESIGN: Pure Rust/WASM core engine wrapper.
 */

const WORKSPACE_ROOT = path.resolve(__dirname, "..", "..");
const MRT_EXEC_TIMEOUT_MS = 120000;

const server = new Server(
  {
    name: "mrt-arsenal",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

const MRT_TOOLS: Tool[] = [
  {
    name: "mrt_audit",
    description: "Run the Presidential Auditor to detect technical debt and refinement gaps.",
    inputSchema: {
      type: "object",
      properties: {
        mode: { type: "string", enum: ["workspace", "refinement", "proposal"], default: "workspace" },
        glob: { type: "string", default: "src/**/*.rs" },
      },
    },
  },
  {
    name: "mrt_brain_get",
    description: "Retrieve architectural invariants or telemetry from the Knowledge Core.",
    inputSchema: {
      type: "object",
      properties: {
        key: { type: "string" },
      },
      required: ["key"],
    },
  },
  {
    name: "mrt_general_ci",
    description: "Execute the NASA-grade CI gate.",
    inputSchema: {
      type: "object",
      properties: {},
    },
  },
  {
    name: "mrt_general_ci_compile",
    description: "Execute the NASA-grade CI compile profile.",
    inputSchema: {
      type: "object",
      properties: {},
    },
  },
  {
    name: "mrt_general_ci_fast",
    description: "Execute the NASA-grade CI fast profile.",
    inputSchema: {
      type: "object",
      properties: {},
    },
  },
  {
    name: "mrt_wave_dry_run",
    description: "Preview Wave dispatch changes for a proposal without mutating files.",
    inputSchema: {
      type: "object",
      properties: {
        proposal_id: { type: "string" },
        proposalId: { type: "string" },
        proposal_file: { type: "string" },
        proposalFile: { type: "string" },
        max_lines: { type: "number", default: MAX_WAVE_LINES, minimum: 1, maximum: MAX_WAVE_LINES },
        maxLines: { type: "number", default: MAX_WAVE_LINES, minimum: 1, maximum: MAX_WAVE_LINES },
      },
    },
  },
  {
    name: "mrt_wave_apply",
    description: "Apply Wave dispatch changes for a proposal (admin role required).",
    inputSchema: {
      type: "object",
      properties: {
        proposal_id: { type: "string" },
        proposalId: { type: "string" },
        proposal_file: { type: "string" },
        proposalFile: { type: "string" },
        max_lines: { type: "number", default: MAX_WAVE_LINES, minimum: 1, maximum: MAX_WAVE_LINES },
        maxLines: { type: "number", default: MAX_WAVE_LINES, minimum: 1, maximum: MAX_WAVE_LINES },
      },
    },
  },
  {
    name: "mrt_lsp_diagnostics",
    description: "Run MIRR LSP diagnostics using source text via JSON-RPC framing.",
    inputSchema: {
      type: "object",
      properties: {
        source: { type: "string" },
        source_text: { type: "string" },
        sourceText: { type: "string" },
        text: { type: "string" },
      },
    },
  },
];

type MrtToolName =
  | "mirr-audit"
  | "mirr-brain"
  | "mirr-general"
  | "mirr-wave"
  | "mirr-lsp";

type MrtRole = "reader" | "builder" | "committer" | "admin";

const TOOL_ROLE_ALLOWLIST: Record<MrtDispatchTool, readonly MrtRole[]> = {
  mrt_audit: ["builder", "committer", "admin"],
  mrt_brain_get: ["committer", "admin"],
  mrt_general_ci: ["builder", "admin"],
  mrt_general_ci_compile: ["builder", "admin"],
  mrt_general_ci_fast: ["builder", "admin"],
  mrt_wave_dry_run: ["builder", "committer", "admin"],
  mrt_wave_apply: ["admin"],
  mrt_lsp_diagnostics: ["builder", "committer", "admin"],
};

type MrtRequest = {
  schema_version: "1";
  tool: MrtToolName;
  args: string[];
};

async function callMrtInterface(
  tool: MrtToolName,
  args: string[],
  options: { stdinData?: string } = {}
): Promise<string> {
  const request: MrtRequest = {
    schema_version: "1",
    tool,
    args,
  };

  const binByTool: Record<MrtToolName, string> = {
    "mirr-audit": "mirr-audit",
    "mirr-brain": "mirr-brain",
    "mirr-general": "mirr-general",
    "mirr-wave": "mirr-wave",
    "mirr-lsp": "mirr-lsp",
  };

  const runResult = spawnSync(
    "cargo",
    ["run", "--bin", binByTool[tool], "--", ...args],
    {
      cwd: WORKSPACE_ROOT,
      encoding: "utf8",
      shell: false,
      windowsHide: true,
      input: options.stdinData,
      maxBuffer: MAX_OUTPUT_BYTES,
      timeout: MRT_EXEC_TIMEOUT_MS,
    }
  );

  const stdoutRaw = typeof runResult.stdout === "string" ? runResult.stdout : "";
  const stderrRaw = typeof runResult.stderr === "string" ? runResult.stderr : "";
  const stdout = clipOutput(stdoutRaw);
  const stderr = clipOutput(stderrRaw);

  if (runResult.error) {
    throw runResult.error;
  }

  if (runResult.status !== 0) {
    const stderrText = stderr.text.trim();
    const stdoutText = stdout.text.trim();
    const message =
      stderrText.length > 0
        ? stderrText
        : stdoutText.length > 0
          ? stdoutText
          : `cargo run --bin ${binByTool[tool]} failed`;
    throw new Error(message);
  }

  const output = stdout.text.trim();
  let result: unknown = output;
  if (output.length > 0) {
    try {
      result = JSON.parse(output);
    } catch {
      result = { stdout: output };
    }
  }

  const stderrText = stderr.text.trim();

  return JSON.stringify({
    schema_version: request.schema_version,
    tool,
    request,
    output_limit_bytes: MAX_OUTPUT_BYTES,
    stdout_truncated: stdout.truncated,
    stderr_truncated: stderr.truncated,
    stderr: stderrText.length > 0 ? stderrText : undefined,
    result,
  });
}

function getArgValue(args: unknown, keys: readonly string[]): unknown {
  if (!args || typeof args !== "object") {
    return undefined;
  }

  const argMap = args as Record<string, unknown>;
  for (const key of keys) {
    if (Object.prototype.hasOwnProperty.call(argMap, key)) {
      return argMap[key];
    }
  }

  return undefined;
}

function getStringArg(args: unknown, keys: readonly string[], fallback = ""): string {
  const value = getArgValue(args, keys);
  return typeof value === "string" ? value : fallback;
}

function getNumberArg(args: unknown, keys: readonly string[], fallback: number): number {
  const value = getArgValue(args, keys);
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === "string") {
    const parsed = Number(value);
    if (Number.isFinite(parsed)) {
      return parsed;
    }
  }

  return fallback;
}

function resolveRole(args: unknown): MrtRole {
  const role = getStringArg(args, ["role", "auth_role", "authRole"]);
  switch (role) {
    case "reader":
    case "builder":
    case "committer":
    case "admin":
      return role;
    default:
      throw new Error("MRT_EXEC_ERROR: missing or invalid role");
  }
}

function ensureRoleAllowed(tool: MrtDispatchTool, role: MrtRole): void {
  if (!TOOL_ROLE_ALLOWLIST[tool].includes(role)) {
    throw new Error(`MRT_EXEC_ERROR: unauthorized role '${role}' for tool '${tool}'`);
  }
}

server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: MRT_TOOLS,
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    switch (name) {
      case "mrt_audit": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_audit", role);
        const mode = getStringArg(args, ["mode"], "workspace");
        const glob = getStringArg(args, ["glob"], "src/**/*.rs");
        const output = await callMrtInterface("mirr-audit", ["--mode", mode, "--glob", glob, "--format", "json"]);
        return { content: [{ type: "text", text: output }] };
      }
      case "mrt_brain_get": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_brain_get", role);
        const output = await callMrtInterface("mirr-brain", brainGetArgs(getStringArg(args, ["key"])));
        return { content: [{ type: "text", text: output }] };
      }
      case "mrt_general_ci": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_general_ci", role);
        const output = await callMrtInterface("mirr-general", ["ci", "--format", "json"]);
        return { content: [{ type: "text", text: output }] };
      }
      case "mrt_general_ci_compile": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_general_ci_compile", role);
        const output = await callMrtInterface("mirr-general", generalCiCompileArgs());
        return { content: [{ type: "text", text: output }] };
      }
      case "mrt_general_ci_fast": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_general_ci_fast", role);
        const output = await callMrtInterface("mirr-general", generalCiFastArgs());
        return { content: [{ type: "text", text: output }] };
      }
      case "mrt_wave_dry_run": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_wave_dry_run", role);
        const proposalId = getStringArg(args, ["proposal_id", "proposalId"]);
        const proposalFile = getStringArg(args, ["proposal_file", "proposalFile"]);
        const maxLines = getNumberArg(args, ["max_lines", "maxLines"], MAX_WAVE_LINES);
        const output = await callMrtInterface("mirr-wave", waveDryRunArgs(proposalId, proposalFile, maxLines));
        return { content: [{ type: "text", text: output }] };
      }
      case "mrt_wave_apply": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_wave_apply", role);
        const proposalId = getStringArg(args, ["proposal_id", "proposalId"]);
        const proposalFile = getStringArg(args, ["proposal_file", "proposalFile"]);
        const maxLines = getNumberArg(args, ["max_lines", "maxLines"], MAX_WAVE_LINES);
        const output = await callMrtInterface("mirr-wave", waveApplyArgs(proposalId, proposalFile, maxLines));
        return { content: [{ type: "text", text: output }] };
      }
      case "mrt_lsp_diagnostics": {
        const role = resolveRole(args);
        ensureRoleAllowed("mrt_lsp_diagnostics", role);
        const source = getStringArg(args, ["source", "source_text", "sourceText", "text"]);
        const invocation = lspDiagnosticsInvocation(source);
        const output = await callMrtInterface("mirr-lsp", invocation.args, { stdinData: invocation.stdinData });
        return { content: [{ type: "text", text: output }] };
      }
      default:
        throw new Error(`MRT_EXEC_ERROR: unknown tool '${name}'`);
    }
  } catch (error: any) {
    const message = error && typeof error.message === "string" ? error.message : String(error);
    return {
      isError: true,
      content: [{ type: "text", text: JSON.stringify({ schema_version: "1", code: "MRT_EXEC_ERROR", message }) }],
    };
  }
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("MRT Arsenal MCP Server running on stdio");
}

main().catch((error) => {
  console.error("Fatal error in main():", error);
  process.exit(1);
});
