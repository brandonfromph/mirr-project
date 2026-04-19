#!/usr/bin/env bash
# Quick CI checks for local iteration
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
expected_target_dir="$repo_root/target/ci-wave"

cd "$repo_root"
mkdir -p "$expected_target_dir"
export CARGO_TARGET_DIR="$expected_target_dir"
"$repo_root/scripts/preflight-gate.sh" "$repo_root" "$expected_target_dir"

echo "Fast CI: fmt, check only (no heavy wasm/coq)"
cargo fmt --all -- --check
todo="" # keep full local CI separate
cargo check --all-targets

echo "Fast checks passed"