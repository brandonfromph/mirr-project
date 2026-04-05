export const KB_ROOT = ".kb-data" as const;
export const MAX_KB_KEY_SIZE = 128 as const;
export const MAX_OUTPUT_BYTES = 65_536 as const;
export const MAX_WAVE_LINES = 128 as const;
export const MAX_LSP_SOURCE_BYTES = 1_048_576 as const;

const PROPOSAL_ID_RE = /^[0-9]{3,}$/;

export type MrtDispatchTool =
  | "mrt_audit"
  | "mrt_brain_get"
  | "mrt_general_ci"
  | "mrt_general_ci_compile"
  | "mrt_general_ci_fast"
  | "mrt_wave_dry_run"
  | "mrt_wave_apply"
  | "mrt_lsp_diagnostics";

function requireBrainKey(key: string): string {
  if (key.length === 0) {
    throw new Error("missing_key");
  }
  if (key.length > MAX_KB_KEY_SIZE) {
    throw new Error("kb_key_too_long");
  }
  return key;
}

function requireProposalId(proposalId: string): string {
  if (proposalId.length === 0) {
    throw new Error("missing_proposal_id");
  }
  if (!PROPOSAL_ID_RE.test(proposalId)) {
    throw new Error("invalid_proposal_id");
  }
  return proposalId;
}

function requireProposalFile(proposalFile: string): string {
  if (proposalFile.length === 0) {
    throw new Error("missing_proposal_file");
  }

  const normalized = proposalFile.replace(/\\/g, "/").replace(/\/+/g, "/").replace(/^\.\//, "");

  if (!normalized.startsWith("proposals/")) {
    throw new Error("invalid_proposal_file");
  }
  if (!normalized.endsWith(".md")) {
    throw new Error("invalid_proposal_file");
  }
  if (normalized.includes("../")) {
    throw new Error("invalid_proposal_file");
  }

  return normalized;
}

function normalizeMaxLines(maxLines: number): number {
  if (!Number.isFinite(maxLines)) {
    throw new Error("invalid_max_lines");
  }

  const truncated = Math.trunc(maxLines);
  if (truncated < 1) {
    return 1;
  }
  if (truncated > MAX_WAVE_LINES) {
    return MAX_WAVE_LINES;
  }
  return truncated;
}

function frameJsonRpcMessage(payload: Record<string, unknown>): string {
  const body = JSON.stringify(payload);
  const length = Buffer.byteLength(body, "utf8");
  return `Content-Length: ${length}\r\n\r\n${body}`;
}

export function brainGetArgs(key: string): string[] {
  const validatedKey = requireBrainKey(key);

  return ["--kb-root", KB_ROOT, "--format", "json", "get", "--key", validatedKey];
}

export function generalCiCompileArgs(): string[] {
  return ["ci", "--profile", "compile", "--format", "json"];
}

export function generalCiFastArgs(): string[] {
  return ["ci", "--profile", "fast", "--format", "json"];
}

export function clipOutput(raw: string): { text: string; truncated: boolean } {
  if (Buffer.byteLength(raw, "utf8") <= MAX_OUTPUT_BYTES) {
    return { text: raw, truncated: false };
  }

  let text = raw;
  while (Buffer.byteLength(text, "utf8") > MAX_OUTPUT_BYTES && text.length > 0) {
    text = text.slice(0, text.length - 1);
  }

  return { text, truncated: true };
}

function waveArgs(
  proposalId: string,
  proposalFile: string,
  maxLines: number,
  dryRun: boolean
): string[] {
  const id = requireProposalId(proposalId);
  const file = requireProposalFile(proposalFile);
  const lines = normalizeMaxLines(maxLines).toString();

  const args = [
    "--proposal-id",
    id,
    "--proposal-file",
    file,
    "--max-lines",
    lines,
  ];

  if (dryRun) {
    args.push("--dry-run");
  }

  return args;
}

export function waveDryRunArgs(
  proposalId: string,
  proposalFile: string,
  maxLines: number
): string[] {
  return waveArgs(proposalId, proposalFile, maxLines, true);
}

export function waveApplyArgs(
  proposalId: string,
  proposalFile: string,
  maxLines: number
): string[] {
  return waveArgs(proposalId, proposalFile, maxLines, false);
}

export function lspDiagnosticsInvocation(source: string): {
  args: string[];
  stdinData: string;
} {
  if (source.length === 0) {
    throw new Error("missing_source");
  }
  if (Buffer.byteLength(source, "utf8") > MAX_LSP_SOURCE_BYTES) {
    throw new Error("lsp_source_too_large");
  }

  const initialize = frameJsonRpcMessage({
    jsonrpc: "2.0",
    id: 1,
    method: "initialize",
    params: {
      processId: null,
      rootUri: null,
      capabilities: {},
      workspaceFolders: null,
    },
  });

  const didOpen = frameJsonRpcMessage({
    jsonrpc: "2.0",
    method: "textDocument/didOpen",
    params: {
      textDocument: {
        uri: "file:///mrt-input.mirr",
        languageId: "mirr",
        version: 1,
        text: source,
      },
    },
  });

  const shutdown = frameJsonRpcMessage({
    jsonrpc: "2.0",
    id: 2,
    method: "shutdown",
    params: null,
  });

  const exit = frameJsonRpcMessage({
    jsonrpc: "2.0",
    method: "exit",
  });

  return {
    args: [],
    stdinData: `${initialize}${didOpen}${shutdown}${exit}`,
  };
}