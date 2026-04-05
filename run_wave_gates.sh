#!/usr/bin/env bash
set -euo pipefail

# Legacy marker for gate contract parity: target/proposal-097-run
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$repo_root"

cargo run --bin mirr-general -- ci --format json
