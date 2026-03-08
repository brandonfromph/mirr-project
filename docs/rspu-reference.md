---
title: R-SPU Reference
nav_order: 5
---

# R-SPU Instruction Set Architecture Reference

> **Status:** Active
> **Module:** `src/emit/rspu_isa.rs`, `src/emit/rspu_regalloc.rs`, `src/emit/rspu.rs`
> **Campaign:** RSPU-001
> **Error codes:** E701–E703

The **Reflex Signal Processing Unit (R-SPU)** is a safety-critical instruction-level
target for MIRR. It maps 1:1 to MIRR's three primitives (signal, guard, reflex)
with bounded resource limits per NASA Power-of-10 rule P10.

---

## Register File

256 registers partitioned by signal kind:

| Partition    | Range       | Count | Maps to              |
|-------------|-------------|-------|----------------------|
| Input ports  | R0–R63      | 64    | `SignalKind::Input`  |
| Output ports | R64–R127    | 64    | `SignalKind::Output` |
| Internals    | R128–R191   | 64    | `SignalKind::Internal` |
| Temporaries  | R192–R255   | 64    | Expression intermediates |

The register allocator (`rspu_regalloc.rs`) performs a single bounded pass over
`module.signals`, assigning each signal to the appropriate partition. Expression
temporaries are allocated on-demand during emission.

**Error E701** is raised if any partition overflows.

---

## Resource Limits

{: .warning }
> All resource limits are hard compile-time caps. There is no dynamic
> allocation. Exceeding any limit produces an immediate compilation error
> (E701, E702, or E703).

| Resource      | Limit | Constant           |
|--------------|-------|--------------------|
| Registers     | 256   | `MAX_REGISTERS`    |
| Guard units   | 64    | `MAX_GUARDS`       |
| Instructions  | 4096  | `MAX_INSTRUCTIONS` |

All limits are compile-time constants. The emitter checks instruction count
after emission and raises **E702** on overflow.

---

## Instruction Set (20 instructions)

### Tier 1: Register

| Mnemonic       | Format                        | Description |
|---------------|-------------------------------|-------------|
| `LOAD_INPUT`   | `LOAD_INPUT Rd, Pn`           | Load input port *n* into register *d* |
| `STORE_OUTPUT`  | `STORE_OUTPUT Rs, Pn`         | Store register *s* to output port *n* |
| `MOV`          | `MOV Rd, Rs`                  | Copy register *s* to register *d* |
| `LOAD_IMM`     | `LOAD_IMM Rd, value (wN)`     | Load immediate value into *d* (N-bit) |

### Tier 2: ALU

| Mnemonic       | Format                        | Description |
|---------------|-------------------------------|-------------|
| `ALU`          | `ALU Rd, Ra, Rb, OP`          | Binary: `Rd = Ra OP Rb` |
| `ALU_IMM`      | `ALU_IMM Rd, Ra, imm, OP`     | Binary with immediate: `Rd = Ra OP imm` |
| `ALU_UNARY`    | `ALU_UNARY Rd, Rs, OP`        | Unary: `Rd = OP(Rs)` |

**Binary ALU operations (14):** ADD, SUB, MUL, AND, OR, XOR, SHL, SHR, EQ, NE, LT, LE, GT, GE

**Unary ALU operations (2):** NOT, NEG

### Tier 3: Temporal

| Mnemonic       | Format                        | Description |
|---------------|-------------------------------|-------------|
| `SR_INIT`      | `SR_INIT Gn, length, Rc`      | Initialize shift-register guard *n* with *length* stages, condition from *Rc* |
| `SR_TICK`      | `SR_TICK Gn`                  | Advance shift-register guard *n* by one tick |
| `SR_QUERY`     | `SR_QUERY Rd, Gn`             | Read shift-register guard *n* result into *Rd* |
| `CTR_INIT`     | `CTR_INIT Gn, target, Rc`     | Initialize counter guard *n* with *target* count, condition from *Rc* |
| `CTR_TICK`     | `CTR_TICK Gn`                 | Advance counter guard *n* by one tick |
| `CTR_QUERY`    | `CTR_QUERY Rd, Gn`            | Read counter guard *n* result into *Rd* |
| `GUARD_AND`    | `GUARD_AND Gd, Ga, Gb`        | Combine guards: `Gd = Ga AND Gb` |
| `GUARD_OR`     | `GUARD_OR Gd, Ga, Gb`         | Combine guards: `Gd = Ga OR Gb` |

### Tier 4: Reflex (Execution)

| Mnemonic       | Format                        | Description |
|---------------|-------------------------------|-------------|
| `REFLEX_IF`    | `REFLEX_IF Gn, Rd, Rs`        | Conditional move: if guard *n* active, `Rd = Rs` |
| `PREV`         | `PREV Rd, Rs, delay`          | Previous-tick: `Rd = Rs` at `t - delay` |

### Tier 5: Safety (MAPE-K)

| Mnemonic         | Format                      | Description |
|-----------------|------------------------------|-------------|
| `EMERGENCY_STOP` | `EMERGENCY_STOP`             | Halt R-SPU immediately (non-recoverable) |
| `ASSERT_ALWAYS`  | `ASSERT_ALWAYS Rc, #id`      | Verify *Rc* is always true (property *id*) |
| `ASSERT_NEVER`   | `ASSERT_NEVER Rc, #id`       | Verify *Rc* is never true (property *id*) |

---

## Tick Execution Model

Each tick executes in a fixed sequence:

1. **Preamble** — `LOAD_INPUT` for every input signal
2. **Temporal guards** — `SR_INIT`/`CTR_INIT` + `TICK` + `QUERY` per guard
3. **Reflexes** — Expression evaluation + `REFLEX_IF` conditional moves
4. **Properties** — Expression evaluation + `ASSERT_ALWAYS`/`ASSERT_NEVER`
5. **Postamble** — `STORE_OUTPUT` for every output signal

All instruction execution is single-cycle and deterministic.

{: .note }
> The R-SPU has no pipeline stalls, no branch prediction, and no speculative
> execution. Timing is guaranteed by construction.

---

## Expression Evaluation

Expressions are compiled using an explicit work-stack (no recursion, per NASA P10).
Each sub-expression is evaluated bottom-up, storing intermediate results in
temporary registers (R192–R255).

The work-stack uses three item types:
- `Eval(expr)` — schedule an expression for evaluation
- `EmitUnary(op)` — pop one result, apply unary op, push result
- `EmitBinary(op)` — pop two results, apply binary op, push result

Bounded by `MAX_EXPR_NODES` (512 nodes per expression).

---

## Error Codes

| Code | Meaning |
|------|---------|
| E701 | Register partition overflow (too many signals for partition) |
| E702 | Instruction budget exceeded (> 4096 instructions) |
| E703 | Guard resource exhausted (> 64 temporal guard units) |

---

## Pipeline Integration

Enable R-SPU emission via `PipelineConfig`:

```rust
let config = PipelineConfig {
    rspu: true,
    // ... other fields
};
let result = run_pipeline(source, config)?;
let program = result.rspu_program.unwrap();
println!("{}", program.emit_asm());
```

The R-SPU backend requires temporal compilation to have run first
(`temporal_netlist` must be populated in `PipelineResult`).

---

## Assembly Output Format

`RspuProgram::emit_asm()` produces human-readable assembly:

```asm
; R-SPU Assembly — generated by MIRR compiler
; Registers used: 5
; Guards used:    1
; Instructions:   12
;
; Register map:
;   R0   = sensor
;   R1   = threshold
;   R64  = alarm
;
; Guard map:
;   G0   = sustained_fault

   0:  LOAD_INPUT  R0, P0
   1:  LOAD_INPUT  R1, P1
   2:  SR_INIT     G0, 3, R0
   3:  SR_TICK     G0
   4:  SR_QUERY    R0, G0
   5:  REFLEX_IF   G0, R64, R192
   6:  STORE_OUTPUT R64, P0
```

## See Also

- [Error Codes](error_codes) — Full error code catalogue (E7xx section)
- [Tutorial](tutorial) — Lesson 10: R-SPU emission
- [Type System](type-system) — Signed/unsigned types and width inference
- [Roadmap](roadmap) — Phase 8: R-SPU ISA
