#!/usr/bin/env node
// Revoke an API key entry in mcp_server/config.json by id, token, or tokenHash.
// Usage:
//   node mcp_server/scripts/revoke_api_key.js --id my-key-id
//   node mcp_server/scripts/revoke_api_key.js --token <raw-token>
//   node mcp_server/scripts/revoke_api_key.js --tokenHash <hex>
//   node mcp_server/scripts/revoke_api_key.js --config path/to/config.json

const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

function usage() {
  console.error("Usage: node mcp_server/scripts/revoke_api_key.js --id <id> | --token <raw-token> | --tokenHash <hex> [--config <path>]");
  process.exit(1);
}

function sha256hex(s) {
  return crypto.createHash("sha256").update(s).digest("hex");
}

const args = process.argv.slice(2);
let id, token, tokenHash, cfgPath;
for (let i = 0; i < args.length; i++) {
  const a = args[i];
  if (a === "--id") id = args[++i];
  else if (a === "--token") token = args[++i];
  else if (a === "--tokenHash") tokenHash = args[++i];
  else if (a === "--config") cfgPath = args[++i];
  else { usage(); }
}
if (!id && !token && !tokenHash) usage();
cfgPath = path.resolve(process.cwd(), cfgPath || "mcp_server/config.json");
if (!fs.existsSync(cfgPath)) { console.error("Config not found:", cfgPath); process.exit(2); }

const raw = fs.readFileSync(cfgPath, "utf8");
const cfg = JSON.parse(raw || "{}");
if (!Array.isArray(cfg.api_keys)) cfg.api_keys = [];
if (!Array.isArray(cfg.revoked_keys)) cfg.revoked_keys = [];

const matches = [];
// compute tokenHash if raw token provided
if (token && !tokenHash) tokenHash = sha256hex(token);

for (let i = cfg.api_keys.length - 1; i >= 0; i--) {
  const e = cfg.api_keys[i];
  if (!e) continue;
  if (id && e.id === id) {
    matches.push(e);
    cfg.api_keys.splice(i, 1);
    continue;
  }
  if (token && !e.hashed && e.token === token) {
    matches.push(e);
    cfg.api_keys.splice(i, 1);
    continue;
  }
  if (tokenHash && ((e.hashed && e.token === tokenHash) || (!e.hashed && sha256hex(e.token) === tokenHash))) {
    matches.push(e);
    cfg.api_keys.splice(i, 1);
    continue;
  }
}

// append revoked metadata
const now = new Date().toISOString();
for (const m of matches) {
  cfg.revoked_keys.push({ id: m.id, role: m.role, revoked_at: now });
}

if (matches.length === 0) {
  console.error("No matching keys found.");
  process.exit(3);
}

// backup and write
const bak = cfgPath + "." + now.replace(/[:.]/g, "-") + ".bak";
fs.copyFileSync(cfgPath, bak);
fs.writeFileSync(cfgPath, JSON.stringify(cfg, null, 2), "utf8");
console.log("Revoked keys:", matches.map(m => ({ id: m.id, role: m.role })));
console.error("Backup saved to", bak);