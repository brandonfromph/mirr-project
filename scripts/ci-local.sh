#!/usr/bin/env bash
# ci-local.sh — Run the same checks as GitHub Actions CI, locally.
# Usage: bash scripts/ci-local.sh
# Run this before every push to catch failures early.
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

pass() { echo -e "${GREEN}PASS${NC}: $1"; }
fail() { echo -e "${RED}FAIL${NC}: $1"; exit 1; }

echo "===== 1/5  cargo fmt --check ====="
cargo fmt -- --check || fail "cargo fmt"
pass "cargo fmt"

echo ""
echo "===== 2/5  cargo clippy ====="
cargo clippy --all-targets --all-features -- -D warnings || fail "cargo clippy"
pass "cargo clippy"

echo ""
echo "===== 3/5  cargo test ====="
cargo test --all || fail "cargo test"
pass "cargo test"

echo ""
echo "===== 4/5  cargo doc ====="
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps || fail "cargo doc"
pass "cargo doc"

echo ""
echo "===== 5/5  Compile examples ====="
failed=0
for f in examples/*.mirr; do
  name=$(basename "$f")
  case "$name" in
    malformed_input.mirr|validation_errors.mirr)
      echo "  SKIP (error case): $name"
      continue
      ;;
    flight_controller_signed.mirr)
      echo "  SKIP (signed guard lowering not yet supported): $name"
      continue
      ;;
  esac
  if cargo run --bin mirr-compile -- --emit verilog "$f" > /dev/null 2>&1; then
    echo "  OK: $name"
  else
    echo "  FAIL: $name"
    failed=1
  fi
done
if [ "$failed" -eq 1 ]; then
  fail "example compilation"
fi
pass "example compilation"

echo ""
echo -e "${GREEN}All CI checks passed. Safe to push.${NC}"
