#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-}"
expected_target_dir="${2:-}"
expected_wrapper="${3:-sccache}"

if [[ -z "$repo_root" || -z "$expected_target_dir" ]]; then
  echo "preflight error: usage preflight-gate.sh <repo_root> <expected_target_dir> [expected_wrapper]" >&2
  exit 1
fi

config_file="$repo_root/.cargo/config.toml"
if [[ ! -f "$config_file" ]]; then
  echo "preflight error: missing $config_file" >&2
  exit 1
fi

if [[ "${CARGO_TARGET_DIR:-}" != "$expected_target_dir" ]]; then
  echo "preflight error: CARGO_TARGET_DIR drift" >&2
  echo "  expected: $expected_target_dir" >&2
  echo "  actual:   ${CARGO_TARGET_DIR:-<unset>}" >&2
  exit 1
fi

if ! grep -Fq "rustc-wrapper = \"$expected_wrapper\"" "$config_file"; then
  echo "preflight error: rustc-wrapper drift in .cargo/config.toml" >&2
  echo "  expected rustc-wrapper = \"$expected_wrapper\"" >&2
  exit 1
fi

if ! command -v sccache >/dev/null 2>&1; then
  echo "preflight error: sccache is not available on PATH" >&2
  exit 1
fi

resolved_sccache="$(command -v sccache)"
if [[ "$expected_wrapper" != "sccache" ]]; then
  resolved_norm="${resolved_sccache//\\//}"
  expected_norm="${expected_wrapper//\\//}"
  if [[ "$resolved_norm" != "$expected_norm" ]]; then
    echo "preflight error: sccache PATH resolution drift" >&2
    echo "  expected: $expected_wrapper" >&2
    echo "  actual:   $resolved_sccache" >&2
    exit 1
  fi
fi

echo "preflight ok: wrapper/path/target-dir aligned"
