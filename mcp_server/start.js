#!/usr/bin/env node
// Lightweight start wrapper for the local MCP server.  Operates in
// stdio-direct mode only and performs placeholder/installation checks.

'use strict';
const fs = require('fs');
const path = require('path');

// simple argument parsing only for workspace-root (others are irrelevant)
let argv = { 'workspace-root': process.env.MCP_WORKSPACE_ROOT || path.resolve(__dirname, '..', '..') };
for (let i = 2; i < process.argv.length; i++) {
  const arg = process.argv[i];
  if (arg === '--workspace-root' && i + 1 < process.argv.length) {
    argv['workspace-root'] = process.argv[++i];
  } else if (arg === '--help' || arg === '-h') {
    console.error('Usage: start.js [--workspace-root PATH]');
    process.exit(0);
  }
}

const WORKSPACE_ROOT = path.resolve(argv['workspace-root']);
const distPath = path.join(__dirname, 'dist', 'server.js');

// pidfile management is no longer needed when never binding a port

function checkPlaceholder() {
  const invoked = process.argv.join(' ');
  if (invoked.includes('${workspaceFolder}') || WORKSPACE_ROOT.includes('${workspaceFolder}')) {
    console.error('ERROR: Detected unresolved VS Code placeholder ${workspaceFolder} in invocation or environment.');
    console.error('Start the server from the project root or replace placeholders. Example:');
    console.error('  cd c:\\Users\\elvie\\nasa-rust-project\\mcp_server && npm run build && npm start');
    process.exit(2);
  }
}

function checkDist() {
  if (!fs.existsSync(distPath)) {
    console.error('ERROR: ' + distPath + ' not found.');
    console.error('Build the server first:');
    console.error('  cd mcp_server && npm run build');
    process.exit(2);
  }
}

// port/pipe utilities removed; server will use stdio exclusively

async function main() {
  console.error('wrapper env MCP_TEST_FORCE_RESET=', process.env.MCP_TEST_FORCE_RESET);
  checkPlaceholder();
  checkDist();

  // force the server to communicate over stdio only
  process.env.MCP_STDIO_DIRECT = '1';
  require(distPath);
}

// legacy stdio proxy removed; server runs directly in stdio-direct mode

main().catch(err => {
  console.error('startup failure:', err);
  process.exit(1);
});