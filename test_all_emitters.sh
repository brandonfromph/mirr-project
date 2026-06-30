#!/bin/bash

# Exit on error
set -e

echo "============================================="
echo "   MIRR Compiler Emitter Pipeline Tests"
echo "============================================="

# Ensure we are in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Must be run from the mirr-private root directory."
    exit 1
fi

# Create output directory
mkdir -p tests/emit_outputs

# Test 1: FIRRTL
echo "[1/4] Testing FIRRTL emission..."
cargo run --bin mirr -- compile examples/safety_property.mirr --emit firrtl --output tests/emit_outputs/safety_property.fir

# Test 2: RISC-V
echo "[2/4] Testing RISC-V assembly emission..."
cargo run --bin mirr -- compile examples/safety_property.mirr --emit riscv --output tests/emit_outputs/safety_property.s

# Test 3: ARM
echo "[3/4] Testing ARM assembly emission..."
cargo run --bin mirr -- compile examples/safety_property.mirr --emit arm --output tests/emit_outputs/safety_property.S

# Test 4: MAPE-K RTL (SystemVerilog)
echo "[4/4] Testing MAPE-K RTL emission..."
cargo run --bin mirr -- compile examples/safety_property.mirr --emit mape-k-rtl --output tests/emit_outputs/safety_property_mapek.sv

echo "============================================="
echo " All tests complete. Outputs saved in tests/emit_outputs/"
echo "============================================="
