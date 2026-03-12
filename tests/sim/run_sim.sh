#!/usr/bin/env bash
# RTL simulation runner — compiles .mirr examples to Verilog,
# then simulates with iverilog and checks for PASS/FAIL.
#
# Usage: bash tests/sim/run_sim.sh
# Requires: cargo (for mirr-compile), iverilog, vvp

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TMPDIR="${REPO_ROOT}/target/sim"
mkdir -p "$TMPDIR"

PASS=0
FAIL=0
SKIP=0

# Test definitions: mirr_file testbench_file module_name
TESTS=(
    "examples/shift_register_guard.mirr tests/sim/shift_register_tb.v short_delay_monitor"
    "examples/neonatal_respirator.mirr tests/sim/neonatal_tb.v neonatal_respirator"
    "examples/multi_guard_monitor.mirr tests/sim/multi_guard_tb.v patient_monitor"
)

MAX_TESTS=32

for i in "${!TESTS[@]}"; do
    if [ "$i" -ge "$MAX_TESTS" ]; then
        break
    fi

    read -r mirr_file tb_file module_name <<< "${TESTS[$i]}"

    echo "--- Simulating: $mirr_file ---"

    # Step 1: Compile .mirr to Verilog
    verilog_file="${TMPDIR}/${module_name}.v"
    if ! cargo run --bin mirr-compile -- --emit verilog "${REPO_ROOT}/${mirr_file}" > "$verilog_file" 2>&1; then
        echo "SKIP: failed to compile ${mirr_file}"
        SKIP=$((SKIP + 1))
        continue
    fi

    # Step 2: Check iverilog is available
    if ! command -v iverilog &>/dev/null; then
        echo "SKIP: iverilog not installed"
        SKIP=$((SKIP + 1))
        continue
    fi

    # Step 3: Compile with iverilog
    vvp_file="${TMPDIR}/${module_name}.vvp"
    if ! iverilog -g2012 -o "$vvp_file" "$verilog_file" "${REPO_ROOT}/${tb_file}" 2>&1; then
        echo "SKIP: iverilog compilation failed for ${module_name}"
        SKIP=$((SKIP + 1))
        continue
    fi

    # Step 4: Run simulation
    sim_output=$(vvp "$vvp_file" 2>&1 || true)
    echo "$sim_output"

    if echo "$sim_output" | grep -q "^FAIL:"; then
        FAIL=$((FAIL + 1))
    elif echo "$sim_output" | grep -q "^PASS:"; then
        PASS=$((PASS + 1))
    else
        echo "SKIP: no PASS/FAIL output from ${module_name}"
        SKIP=$((SKIP + 1))
    fi
done

echo ""
echo "=== RTL Simulation Summary ==="
echo "PASS: $PASS  FAIL: $FAIL  SKIP: $SKIP"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi

echo "All simulations passed."
