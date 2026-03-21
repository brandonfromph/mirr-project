---
title: HLS Optimization Guide
nav_order: 14
---

# HLS Optimization Guide

The MIRR compiler includes a High-Level Synthesis (HLS) pass that optimizes
hardware resource usage through scheduling, sharing, and binding.

## Overview

The HLS pass operates on the guard/reflex DAG and provides:
- **ASAP scheduling** — earliest cycle for each operation
- **ALAP scheduling** — latest cycle given target latency
- **Resource sharing** — time-multiplex compatible operations
- **Operation binding** — assign to physical resources
- **FIFO streaming** — bounded dataflow between operations

## Usage

```rust
use nasa_rust_project::hls::{OpDag, ResourceKind, HlsConfig};

let mut dag = OpDag::new();
let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8]).unwrap();
let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8]).unwrap();
dag.add_edge(a, b);

let config = HlsConfig { latency: 3, sharing: true, binding: true, fifo: true };
let result = nasa_rust_project::hls::run_hls_pass(&dag, &config).unwrap();
```

## Scheduling

### ASAP (As Soon As Possible)
Each operation is scheduled to the earliest cycle where all dependencies are met.

### ALAP (As Late As Possible)
Each operation is scheduled to the latest cycle that meets the target latency.

### Combined
The scheduler computes both ASAP and ALAP, then uses the slack for resource sharing.

## Resource Sharing

Compatible operations (same resource kind, non-overlapping time slots) share
the same physical resource. This reduces area at the cost of multiplexing logic.

## Operation Binding

Each operation is assigned to a physical resource ID. Operations sharing a
resource get the same binding ID.

## See Also

- [R-SPU ISA Spec](rspu_isa_spec) — Instruction set reference
- [FPGA Targets Guide](fpga-targets-guide) — FPGA board support