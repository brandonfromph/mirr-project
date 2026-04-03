#!/usr/bin/env bash
# Proposal 096 critical gate wrapper; delegates to the canonical Rust closeout command.

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$script_dir"

cargo_target_dir="$script_dir/target/proposal-096-run"
export CARGO_TARGET_DIR="$cargo_target_dir"
cargo run --bin mirr-general -- ci --format json
