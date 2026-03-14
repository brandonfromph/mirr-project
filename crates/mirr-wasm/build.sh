#!/usr/bin/env bash
# Build MIRR WASM module for the interactive paper.
# Requires: wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)
set -euo pipefail
cd "$(dirname "$0")"
wasm-pack build --target web --out-dir ../../paper/demos
echo "WASM build complete. Output in paper/demos/"
