#!/usr/bin/env bash
set -euo pipefail

# Legacy marker for gate contract parity: target/proposal-097-run
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
expected_target_dir="$repo_root/target/ci-wave"
cd "$repo_root"

mkdir -p "$expected_target_dir"
export CARGO_TARGET_DIR="$expected_target_dir"
"$repo_root/scripts/preflight-gate.sh" "$repo_root" "$expected_target_dir"

cargo run --bin mirr-general -- ci --format json
