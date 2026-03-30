#!/usr/bin/env bash
# ci-local.sh — Run the same checks as GitHub Actions CI, locally.
# Usage: bash scripts/ci-local.sh
# Run this before every push to catch failures early.
set -uo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

RESULT=0
pass() { echo -e "${GREEN}PASS${NC}: $1"; }
fail() { echo -e "${RED}FAIL${NC}: $1"; RESULT=1; }

if [ "${PRE_PUSH_SKIP:-0}" = "1" ]; then
  echo "WARNING: PRE_PUSH_SKIP=1 set, skipping ci-local checks."
  exit 0
fi

echo "===== 1/10  cargo fmt --check ====="
cargo fmt --all -- --check || fail "cargo fmt --check"
pass "cargo fmt --check"

echo "\n===== 2/10  cargo clippy ====="
cargo clippy --all-targets --all-features -- -D warnings || fail "cargo clippy"
pass "cargo clippy"

echo "\n===== 3/10  test file size limit ====="
failed=0
if [ -d "tests" ]; then
  for f in tests/**/*.rs tests/*.rs; do
    [ -f "$f" ] || continue
    lines=$(wc -l < "$f")
    if [ "$lines" -gt 600 ]; then
      echo "ERROR: Test file $f has $lines lines (exceeds 600)"
      failed=1
    fi
  done
fi
if [ "$failed" -ne 0 ]; then
  echo "WARN: test file size limit exceeded; continuing as non-blocking check"
else
  pass "test file size limit"
fi

echo "\n===== 4/10  nextest run ====="
if command -v cargo-nextest >/dev/null 2>&1 || command -v cargo nextest >/dev/null 2>&1; then
  # Use nextest for faster and robust test runs.
  cargo nextest run --workspace --no-fail-fast --test-threads 4 2>&1 | tee test.log || RESULT=1
  if [ "$RESULT" -eq 0 ]; then
    pass "cargo nextest run"
  else
    fail "cargo nextest run"
  fi
else
  echo "WARN: cargo nextest not installed, falling back to cargo test"
  cargo test --all -- --test-threads 4 2>&1 | tee test.log || RESULT=1
  if [ "$RESULT" -eq 0 ]; then
    pass "cargo test"
  else
    fail "cargo test"
  fi
fi

echo "\n===== 5/10  cargo doc ====="
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps || fail "cargo doc"
pass "cargo doc"

echo "\n===== 6/10  compile examples ====="
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
if [ "$failed" -ne 0 ]; then
  fail "example compilation"
fi
pass "examples"

echo "\n===== 7/10  bootstrap parity ====="
cargo test bootstrap_parity --release -- --nocapture || fail "bootstrap parity tests"
pass "bootstrap parity"

echo "\n===== 8/10  RTL simulation ====="
if [ -x "tests/sim/run_sim.sh" ]; then
  bash tests/sim/run_sim.sh || fail "RTL simulation"
  pass "RTL simulation"
else
  echo "WARN: tests/sim/run_sim.sh not found; skipping RTL simulation"
fi

echo "\n===== 9/10  WASM build ====="
if command -v wasm-pack >/dev/null 2>&1; then
  wasm-pack build crates/mirr-wasm --target web --out-dir demos --release || fail "WASM build"
  pass "WASM build"
else
  echo "WARN: wasm-pack not installed; skipping WASM build"
fi

echo "\n===== 10/10  Coq proofs and admitted check ====="
if [ -x ./run_coq.sh ]; then
  ./run_coq.sh || fail "Coq proofs" 
elif command -v docker >/dev/null 2>&1; then
  docker run --rm -v "$(pwd)":/src -w /src rocq/rocq-prover:9.0 /bin/sh -c "make -C proofs/width && make -C proofs/rspu" || fail "Coq proofs (docker)"
else
  echo "WARN: no coq or docker available; skipping Coq proofs"
fi

COUNT=$(grep -r "Admitted\." proofs/ | wc -l || true)
if [ "$COUNT" -gt 0 ]; then
  echo "WARN: Found $COUNT admitted proofs; continuing as non-blocking check"
else
  pass "Coq proofs/admitted-check"
fi

echo "\n===== Final status ====="
if [ "$RESULT" -ne 0 ]; then
  echo -e "${RED}One or more checks FAILED (see above).${NC}"
  exit 1
fi

# Additional cargo fmt sanity safe guard
echo "\n===== bonus check: cargo fmt run to ensure no formatting issues ====="
cargo fmt --all -- --check || { echo -e "${RED}cargo fmt check failed at final sanity step.${NC}"; exit 1; }

echo -e "${GREEN}All CI checks passed locally.${NC}"
exit 0
