#!/usr/bin/env node
// Node script to generate a secure API token and optionally append a hashed entry to mcp_server/config.json
// Usage:
//   node mcp_server/scripts/generate_api_key.js --id my-key-id --role committer [--append]
//   node mcp_server/scripts/generate_api_key.js --help

const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

function usage() {
  console.log("Usage: node mcp_server/scripts/generate_api_key.js --id <id> --role <role> [--append] [--config <path>]");
  console.log("");
  console.log("Options:");
  console.log("  --id       Identifier for the API key (required)");
  console.log("  --role     Role for the key (admin|committer|builder|other) (required)");
  console.log("  --append   Append the generated key (hashed) to mcp_server/config.json (creates backup)");
  console.log("  --config   Path to config.json (default: mcp_server/config.json)");
  console.log("");
  process.exit(1);
}

function parseArgs() {
  const args = process.argv.slice(2);
  const out = {};
  for (let i = 0; i < args.length; i++) {
    const a = args[i];
    if (a === "--id") out.id = args[++i];
    else if (a === "--role") out.role = args[++i];
    else if (a === "--append") out.append = true;
    else if (a === "--config") out.config = args[++i];
    else if (a === "--help" || a === "-h") usage();
    else {
      console.error("Unknown arg:", a);
      usage();
    }
  }
  return out;
}

function sha256hex(s) {
  return crypto.createHash("sha256").update(s).digest("hex");
}

(async function main() {
  const argv = parseArgs();
  if (!argv.id || !argv.role) usage();

  const token = crypto.randomBytes(32).toString("hex");
  const tokenHash = sha256hex(token);
  const entry = { id: String(argv.id), role: String(argv.role), tokenHash };

  console.log(JSON.stringify({ success: true, token, tokenHash, entry }, null, 2));

  if (argv.append) {
    const cfgPath = path.resolve(process.cwd(), argv.config || "mcp_server/config.json");
    if (!fs.existsSync(cfgPath)) {
      console.error("Config file not found at", cfgPath);
      process.exit(2);
    }
    // backup
    try {
      const bak = cfgPath + "." + new Date().toISOString().replace(/[:.]/g, "-") + ".bak";
      fs.copyFileSync(cfgPath, bak);
      const raw = fs.readFileSync(cfgPath, "utf8");
      const cfg = JSON.parse(raw || "{}");
      if (!Array.isArray(cfg.api_keys)) cfg.api_keys = [];
      // Append hashed entry (store tokenHash instead of raw token)
      cfg.api_keys.push({ id: entry.id, role: entry.role, token: tokenHash, hashed: true });
      if (!Array.isArray(cfg.revoked_keys)) cfg.revoked_keys = [];
      fs.writeFileSync(cfgPath, JSON.stringify(cfg, null, 2), "utf8");
      console.error("Appended hashed key to", cfgPath, "backup saved to", bak);
    } catch (e) {
      console.error("Failed to append to config.json:", e && e.message ? e.message : String(e));
      process.exit(3);
    }
  } else {
    console.error("Token generated (not appended). Re-run with --append to add a hashed entry to config.json.");
  }
})();