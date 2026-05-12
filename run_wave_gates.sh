#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$script_dir"

export CARGO_TARGET_DIR="$repo_root/target/proposal-097-run"
cd "$repo_root"

cargo run --bin mirr-general -- ci --format json
