#!/usr/bin/env bash
# Quick CI checks for local iteration
set -euo pipefail

echo "Fast CI: fmt, check only (no heavy wasm/coq)"
cargo fmt --all -- --check
todo="" # keep full local CI separate
cargo check --all-targets --all-features

echo "Fast checks passed"