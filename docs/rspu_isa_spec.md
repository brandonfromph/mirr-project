---
title: R-SPU ISA v2 Specification
nav_order: 7
---

# R-SPU ISA v2 — Formal Specification

> **Status:** Active (MEGA-3 delivered)
> **ISA Version:** 2.0
> **Last updated:** 2026-03-10

---

## 1. Overview

The **R-SPU (Reflexive Signal Processing Unit)** is a safety-critical, deterministic,
single-issue, in-order processor designed for real-time autonomic control. It serves
as the hardware execution target for the MIRR compiler, mapping directly from the
three MIRR language primitives — Signal, Guard, and Reflex — into register
operations, temporal hardware units, and conditional datapath moves.

Key design principles:

- **Register-only architecture.** The R-SPU has no external memory load/store
  instructions. All data resides in the 256-entry register file, partitioned into
  four fixed regions (input ports, output ports, internal signals, temporaries).
  This eliminates cache misses, TLB faults, and memory-ordering hazards.

- **Deterministic single-cycle execution.** Every instruction executes in exactly
  one clock cycle. There are no pipeline stalls, branch mispredictions, or
  variable-latency operations. Worst-case execution time (WCET) equals the
  instruction count.

- **NASA Power-of-10 compliance.** The ISA enforces bounded resource usage at
  every level: bounded register file, bounded instruction count, bounded guard
  units, bounded exception depth. No recursion. No unbounded loops. All resource
  limits are compile-time constants with `MAX_*` prefixes.

- **Safety-first design.** The ISA includes a dedicated safety tier with
  `EMERGENCY_STOP`, `ASSERT_ALWAYS`, and `ASSERT_NEVER` instructions for MAPE-K
  autonomic control. Exception handling is bounded and deterministic. An unhandled
  exception in reflex mode triggers an immediate emergency stop.

- **Dual-mode execution.** The R-SPU supports two execution modes — reflex mode
  (deterministic, cycle-accurate, interrupt-free) and host mode (interrupt-driven,
  variable latency) — with explicit `MODE_SWITCH` transitions.

The R-SPU ISA v2 extends the original v1 instruction set with exception handling
(TRAP, TRAP_IF, HALT), mode switching (MODE_SWITCH), tagged memory operations
(TAG_LOAD, TAG_CHECK, TAG_READ), pipeline control (NOP, FENCE), and temporal
deadlines (DEADLINE_SET).

---

## 2. Register File Architecture

The R-SPU register file contains **256 eight-bit-indexed registers** (`RegId = u8`).
Registers are statically partitioned into four non-overlapping regions by signal kind.

### 2.1 Register Partitions

| Partition  | Range      | Count | Purpose                           | Maps to                |
|------------|------------|------:|-----------------------------------|------------------------|
| Input      | R0–R63     |    64 | Hardware input ports              | `SignalKind::Input`    |
| Output     | R64–R127   |    64 | Hardware output ports             | `SignalKind::Output`   |
| Internal   | R128–R191  |    64 | Module-internal signals           | `SignalKind::Internal` |
| Temporary  | R192–R255  |    64 | Expression intermediates          | Compiler-allocated     |

Partition constants (defined in `src/emit/rspu_isa.rs`):

```
REG_INPUT_BASE    =   0    REG_INPUT_MAX    =  63
REG_OUTPUT_BASE   =  64    REG_OUTPUT_MAX   = 127
REG_INTERNAL_BASE = 128    REG_INTERNAL_MAX = 191
REG_TEMP_BASE     = 192    REG_TEMP_MAX     = 255
```

The compiler's linear-scan register allocator (`src/emit/rspu_regalloc.rs`) maps
each MIRR signal to a register in the appropriate partition in a single O(n) pass.
Temporary registers are allocated on demand during expression emission. If any
partition overflows, the compiler emits error E701.

### 2.2 Tagged-Word Format

Each register holds a tagged word with three fields:

| Field       | Type         | Width   | Description                                |
|-------------|--------------|---------|--------------------------------------------|
| `value`     | `u64`        | 64 bits | The data payload                           |
| `tag`       | `TypeTag`    | 4 bits  | Runtime type discriminant                  |
| `provenance`| `Provenance` | 4 bits  | Data origin for taint tracking             |

**TypeTag values:**

| Tag             | Encoding | Description                         |
|-----------------|----------|-------------------------------------|
| `Bool`          | 0x0      | Single-bit boolean                  |
| `Unsigned{w}`   | 0x1      | Unsigned integer of width `w` bits  |
| `Signed{w}`     | 0x2      | Signed (two's complement) of width `w` bits |
| `Uninitialized` | 0xF      | Register has not been written       |

**Provenance values:**

| Provenance   | Encoding | Description                              |
|--------------|----------|------------------------------------------|
| `Input(port)`| 0x0      | Value came from a hardware input port    |
| `Computed`   | 0x1      | Value was produced by an ALU operation   |
| `Literal`    | 0x2      | Value was loaded from an immediate       |
| `Unset`      | 0xF      | Register provenance has not been set     |

---

## 3. Instruction Encoding

All R-SPU instructions are encoded as **fixed 32-bit words**. The top 6 bits
(bits [31:26]) hold the opcode. The remaining 26 bits hold the payload, whose
interpretation depends on the instruction format.

### 3.1 Instruction Formats

The ISA defines four instruction formats:

**R-type (register-register):**

```
 31      26 25      18 17      10  9       2  1  0
+----------+----------+----------+----------+----+
|  opcode  |   dst    |   src1   |   src2   |func|
+----------+----------+----------+----------+----+
   6 bits    8 bits     8 bits     8 bits   2 bits
```

Used by: `MOV`, `ALU`, `ALU_UNARY`.

**I-type (register-immediate):**

```
 31      26 25      18 17      10  9            0
+----------+----------+----------+--------------+
|  opcode  |   dst    |   src    |    imm10     |
+----------+----------+----------+--------------+
   6 bits    8 bits     8 bits      10 bits
```

Used by: `LOAD_INPUT`, `STORE_OUTPUT`, `LOAD_IMM`, `ALU_IMM`.

**G-type (guard operations):**

```
 31      26 25      18 17      10  9       2  1  0
+----------+----------+----------+----------+----+
|  opcode  |  guard   |  src_dst |  guard2  |func|
+----------+----------+----------+----------+----+
   6 bits    8 bits     8 bits     8 bits   2 bits
```

Used by: `SR_INIT`, `SR_TICK`, `SR_QUERY`, `CTR_INIT`, `CTR_TICK`, `CTR_QUERY`,
`GUARD_AND`, `GUARD_OR`, `REFLEX_IF`.

**S-type (system/immediate-only):**

```
 31      26 25                                  0
+----------+------------------------------------+
|  opcode  |             imm26                  |
+----------+------------------------------------+
   6 bits               26 bits
```

Used by: `EMERGENCY_STOP`, `ASSERT_ALWAYS`, `ASSERT_NEVER`, `TRAP`, `HALT`,
`NOP`, `FENCE`, `DEADLINE_SET`.

---

## 4. Instruction Set

The R-SPU v2 ISA contains **30 instructions** organized into seven tiers. Each
instruction executes in exactly one cycle. Opcode numbers are decimal.

### 4.1 Register Tier (opcodes 0–3)

These instructions move data between ports, registers, and immediates.

| Opcode | Mnemonic       | Format | Fields                          | Description                                      |
|-------:|----------------|--------|---------------------------------|--------------------------------------------------|
|      0 | `LOAD_INPUT`   | I-type | `dst`, `port`                   | Load value from input port into register `dst`.  |
|      1 | `STORE_OUTPUT` | I-type | `src`, `port`                   | Store register `src` to output port.             |
|      2 | `MOV`          | R-type | `dst`, `src`                    | Copy register `src` to register `dst`.           |
|      3 | `LOAD_IMM`     | I-type | `dst`, `value`, `width`         | Load immediate `value` (with bit-width) into `dst`. |

### 4.2 ALU Tier (opcodes 4–6)

Arithmetic and logic operations on register operands.

| Opcode | Mnemonic    | Format | Fields                | Description                                |
|-------:|-------------|--------|-----------------------|--------------------------------------------|
|      4 | `ALU`       | R-type | `op`, `dst`, `a`, `b` | Binary ALU: `dst = a op b`.                |
|      5 | `ALU_IMM`   | I-type | `op`, `dst`, `a`, `imm` | Binary ALU with immediate: `dst = a op imm`. |
|      6 | `ALU_UNARY` | R-type | `op`, `dst`, `src`    | Unary ALU: `dst = op(src)`.                |

**AluOp encoding (14 binary operations):**

| Funct | Mnemonic | Operation              | Funct | Mnemonic | Operation              |
|------:|----------|------------------------|------:|----------|------------------------|
|     0 | `ADD`    | Addition               |     7 | `SHR`    | Logical right shift    |
|     1 | `SUB`    | Subtraction            |     8 | `EQ`     | Equality comparison    |
|     2 | `MUL`    | Multiplication         |     9 | `NE`     | Inequality comparison  |
|     3 | `AND`    | Bitwise/logical AND    |    10 | `LT`     | Less than              |
|     4 | `OR`     | Bitwise/logical OR     |    11 | `LE`     | Less than or equal     |
|     5 | `XOR`    | Bitwise XOR            |    12 | `GT`     | Greater than           |
|     6 | `SHL`    | Logical left shift     |    13 | `GE`     | Greater than or equal  |

**AluUnaryOp encoding (2 unary operations):**

| Funct | Mnemonic | Operation                       |
|------:|----------|---------------------------------|
|     0 | `NOT`    | Bitwise/logical NOT             |
|     1 | `NEG`    | Arithmetic negation (two's complement) |

### 4.3 Temporal Tier (opcodes 7–16)

Guard hardware units: shift registers and counters that implement MIRR's temporal
`for N cycles` semantics. Each guard unit is addressed by a `GuardId` (u8).

| Opcode | Mnemonic     | Format | Fields                        | Description                                        |
|-------:|--------------|--------|-------------------------------|----------------------------------------------------|
|      7 | `SR_INIT`    | G-type | `guard`, `length`, `cond`     | Initialize shift-register guard with pipeline length and condition register. |
|      8 | `SR_TICK`    | G-type | `guard`                       | Advance shift-register guard by one tick.          |
|      9 | `SR_QUERY`   | G-type | `dst`, `guard`                | Read shift-register guard result into register.    |
|     10 | `CTR_INIT`   | G-type | `guard`, `target`, `cond`     | Initialize counter guard with target count and condition register. |
|     11 | `CTR_TICK`   | G-type | `guard`                       | Advance counter guard by one tick.                 |
|     12 | `CTR_QUERY`  | G-type | `dst`, `guard`                | Read counter guard result into register.           |
|     13 | `GUARD_AND`  | G-type | `dst`, `a`, `b`               | Combine two guard outputs with AND: `dst = a & b`. |
|     14 | `GUARD_OR`   | G-type | `dst`, `a`, `b`               | Combine two guard outputs with OR: `dst = a | b`.  |
|     15 | `REFLEX_IF`  | G-type | `guard`, `dst`, `src`         | Conditional move: if guard active, `dst = src`.    |
|     16 | `PREV`       | G-type | `dst`, `signal`, `delay`      | Read previous-tick value: `dst = signal[t - delay]`. |

The three-instruction guard pattern (INIT, TICK, QUERY) corresponds directly to
MIRR's temporal compilation of `guard G { when COND; for N cycles; }`.

### 4.4 Safety Tier (opcodes 17–19)

MAPE-K autonomic safety actions and formal verification assertions.

| Opcode | Mnemonic         | Format | Fields                    | Description                                        |
|-------:|------------------|--------|---------------------------|----------------------------------------------------|
|     17 | `EMERGENCY_STOP` | S-type | *(none)*                  | Halt the R-SPU immediately. Non-recoverable safety action. |
|     18 | `ASSERT_ALWAYS`  | S-type | `cond`, `property_id`     | Assert condition is always true (verification only, no datapath). |
|     19 | `ASSERT_NEVER`   | S-type | `cond`, `property_id`     | Assert condition is never true (verification only, no datapath). |

`ASSERT_ALWAYS` and `ASSERT_NEVER` map to MIRR property declarations and generate
SystemVerilog Assertions (SVA) in the Verilog backend. They do not produce hardware
in the R-SPU datapath; they are consumed by formal verification tools.

### 4.5 Exception Tier (opcodes 20–22) — NEW in v2

Structured exception handling with bounded trap depth.

| Opcode | Mnemonic  | Format | Fields                    | Description                                          |
|-------:|-----------|--------|---------------------------|------------------------------------------------------|
|     20 | `TRAP`    | S-type | `exception_code`          | Raise an unconditional exception with the given code. |
|     21 | `TRAP_IF` | S-type | `cond`, `exception_code`  | Raise an exception if condition register is true.     |
|     22 | `HALT`    | S-type | `exit_code`               | Graceful program termination with exit code.          |

Exception codes are enumerated in section 6. The `HALT` instruction differs from
`EMERGENCY_STOP` in that it is a graceful termination (exit code 0 = success),
whereas `EMERGENCY_STOP` is an immediate safety-critical abort.

### 4.6 Control Tier (opcodes 23, 27–28) — NEW in v2

Mode switching and pipeline control.

| Opcode | Mnemonic      | Format | Fields             | Description                                                |
|-------:|---------------|--------|--------------------|------------------------------------------------------------|
|     23 | `MODE_SWITCH` | S-type | `target_mode`      | Switch between reflex mode and host mode (see section 7).  |
|     27 | `NOP`         | S-type | *(none)*           | No operation. Consumes one cycle. Used for alignment.      |
|     28 | `FENCE`       | S-type | *(none)*           | Memory/pipeline fence. Ensures all prior writes are visible before subsequent reads. |

### 4.7 Tagged Tier (opcodes 24–26) — NEW in v2

Runtime type tag inspection and manipulation for dynamic safety checks.

| Opcode | Mnemonic    | Format | Fields                     | Description                                             |
|-------:|-------------|--------|----------------------------|---------------------------------------------------------|
|     24 | `TAG_LOAD`  | R-type | `dst`, `tag_value`         | Load an explicit type tag into the tag field of register `dst`. |
|     25 | `TAG_CHECK` | R-type | `src`, `expected_tag`      | Trap if `src` register's tag does not match `expected_tag`. |
|     26 | `TAG_READ`  | R-type | `dst`, `src`               | Copy the tag field of `src` into the value field of `dst`. |

Tag operations enable runtime verification of type safety. `TAG_CHECK` raises
an exception (ExceptionCode::TagMismatch) if the actual tag does not match the
expected tag, providing a hardware-level defense against type confusion.

### 4.8 Temporal Extension (opcode 29) — NEW in v2

Hardware deadline enforcement for real-time constraints.

| Opcode | Mnemonic       | Format | Fields                      | Description                                             |
|-------:|----------------|--------|-----------------------------|---------------------------------------------------------|
|     29 | `DEADLINE_SET` | S-type | `guard`, `max_cycles`       | Set a hard deadline: if guard has not fired within `max_cycles`, raise DeadlineMiss exception. |

`DEADLINE_SET` supports MIRR's `eventually within N` property semantics at the
hardware level, converting a verification-only temporal assertion into a runtime
enforcement mechanism.

---

## 5. Execution Model

### 5.1 Pipeline

The R-SPU uses a **fetch-decode-execute** pipeline with single-issue, in-order
execution. There is no branch prediction, no speculative execution, and no
out-of-order retirement.

- **Fetch:** Read the 32-bit instruction word at the current program counter (PC).
- **Decode:** Extract the 6-bit opcode and route to the appropriate functional unit.
- **Execute:** Perform the operation and write results in the same cycle.

The program counter starts at address 0 and increments by 1 each cycle (there
are no branch or jump instructions; control flow is implicit via guards and
reflexes).

### 5.2 Cycle Semantics

Each clock cycle corresponds to one "tick" of the MIRR temporal model. Within
a single tick, the R-SPU executes the following deterministic sequence:

1. **Input capture:** `LOAD_INPUT` instructions read hardware port values into
   the input register partition (R0–R63).
2. **Guard evaluation:** Temporal guard instructions (`SR_INIT`/`TICK`/`QUERY`,
   `CTR_INIT`/`TICK`/`QUERY`, `GUARD_AND`/`OR`) update guard hardware units
   and query their activation state.
3. **Reflex execution:** `REFLEX_IF` instructions conditionally move values
   gated by guard activation. Expression preambles (`ALU`, `ALU_IMM`,
   `ALU_UNARY`, `LOAD_IMM`, `PREV`) compute intermediate values.
4. **Property assertion:** `ASSERT_ALWAYS` and `ASSERT_NEVER` check verification
   conditions (simulation/formal verification only).
5. **Output commit:** `STORE_OUTPUT` instructions write the output register
   partition (R64–R127) to hardware ports.

### 5.3 Program Termination

A program terminates when one of the following occurs:

- **`HALT` instruction:** Graceful termination with an exit code.
- **`EMERGENCY_STOP` instruction:** Immediate safety-critical abort.
- **Maximum cycle count reached:** The simulator enforces `MAX_SIM_CYCLES`
  to prevent runaway execution.
- **Unhandled exception in reflex mode:** Triggers `EMERGENCY_STOP` automatically.

---

## 6. Exception Model

### 6.1 Exception Codes

The R-SPU defines a fixed set of exception codes:

| Code | Name                  | Description                                          |
|-----:|-----------------------|------------------------------------------------------|
|    0 | `None`                | No exception (sentinel value).                       |
|    1 | `DivisionByZero`      | ALU division or modulo by zero.                      |
|    2 | `RegisterUninitialized` | Read from a register with `Uninitialized` tag.     |
|    3 | `TagMismatch`         | `TAG_CHECK` detected a type tag mismatch.            |
|    4 | `DeadlineMiss`        | `DEADLINE_SET` deadline expired without guard firing. |
|    5 | `AssertionViolation`  | `ASSERT_ALWAYS` or `ASSERT_NEVER` condition failed.  |
|    6 | `UserTrap`            | Software-initiated trap via `TRAP` or `TRAP_IF`.     |

### 6.2 Exception Handler Table

Exception handlers are registered in a bounded handler table:

- **Maximum handlers:** `MAX_TRAP_HANDLERS = 16`
- **Table structure:** Array of `(ExceptionCode, HandlerAddress)` pairs.
- **Lookup:** Linear scan (bounded by `MAX_TRAP_HANDLERS`).

When an exception occurs, the R-SPU searches the handler table for a matching
entry. If found, control transfers to the handler address. If not found, the
default action depends on the current execution mode.

### 6.3 Exception Depth

Exception nesting is bounded to prevent unbounded stack growth:

- **Maximum nesting depth:** `MAX_EXCEPTION_DEPTH = 8`
- If an exception occurs while already at maximum depth, the R-SPU executes
  an immediate `EMERGENCY_STOP` regardless of mode.

### 6.4 Exception Actions

Each exception handler may specify one of four actions:

| Action               | Description                                                |
|----------------------|------------------------------------------------------------|
| `Halt`               | Graceful program termination.                              |
| `EmergencyStop`      | Immediate safety-critical abort (non-recoverable).         |
| `IgnoreAndContinue`  | Suppress the exception and resume execution at PC+1.       |
| `TrapToHost`         | Transfer control to the host processor (exit reflex mode). |

### 6.5 Mode-Dependent Default Behavior

- **Reflex mode:** An unhandled exception triggers `EMERGENCY_STOP`. Safety
  is the overriding concern; the R-SPU must never continue in an undefined
  state during deterministic reflex execution.
- **Host mode:** An unhandled exception traps to the software handler on the
  host processor. The host may inspect the exception code, log diagnostics,
  and decide how to proceed.

---

## 7. Dual-Mode Execution

The R-SPU supports two execution modes to bridge the gap between deterministic
hardware control and flexible software supervision.

### 7.1 Reflex Mode

- **Deterministic, cycle-accurate execution.** Every instruction takes exactly
  one cycle. No interrupts, no preemption.
- **Guard and reflex semantics are fully active.** Shift registers tick,
  counters decrement, conditional moves fire.
- **All temporal guarantees hold.** WCET = instruction count.
- **Exceptions are fail-safe.** Unhandled exceptions trigger `EMERGENCY_STOP`.

This is the primary execution mode for safety-critical autonomic control.

### 7.2 Host Mode

- **Interrupt-driven, variable latency.** The host processor can interrupt
  the R-SPU between instructions.
- **Temporal guards are suspended.** Guard ticking does not occur; the host
  is responsible for managing time.
- **Used for diagnostics, configuration, and non-critical tasks.**
- **Exceptions trap to the host** for software-level handling.

### 7.3 Mode Transitions

The `MODE_SWITCH` instruction transitions between modes:

- **Reflex to Host:** Allowed at any point. The R-SPU completes the current
  instruction, then suspends temporal processing and enables host interrupts.
- **Host to Reflex:** Requires quiescence. The host must ensure there are no
  pending interrupts and no in-flight DMA transfers before issuing
  `MODE_SWITCH` back to reflex mode. Violating the quiescence requirement
  raises a `UserTrap` exception.

---

## 8. Resource Limits

All resource limits are compile-time constants enforced by the MIRR compiler
and the R-SPU hardware. These bounds ensure NASA Power-of-10 compliance:
every loop terminates, every buffer has a fixed maximum size, and every
resource can be statically verified at compile time.

| Constant                | Value       | Purpose                                                |
|-------------------------|------------:|--------------------------------------------------------|
| `MAX_REGISTERS`         |         256 | Total register file entries (4 partitions of 64).      |
| `MAX_GUARDS`            |          64 | Maximum temporal guard hardware units.                 |
| `MAX_INSTRUCTIONS`      |       4,096 | Maximum instructions in a single R-SPU program.        |
| `MAX_TRAP_HANDLERS`     |          16 | Maximum registered exception handlers.                 |
| `MAX_EXCEPTION_DEPTH`   |           8 | Maximum nested exception depth.                        |
| `MAX_SIM_CYCLES`        |   1,000,000 | Maximum simulation cycles (simulator-only bound).      |
| `MAX_TAGGED_WORD_BITS`  |         128 | Maximum bits per tagged register word (64 data + tags).|
| `MAX_EXPR_NODES`        |         512 | Maximum AST nodes per expression (compiler bound).     |

These constants are defined in `src/emit/rspu_isa.rs` (ISA-level) and
`src/ast/mod.rs` (compiler-level). Error E702 is emitted when the instruction
budget is exceeded, E703 when guard resources are exhausted, and E704/E705 when
expression or temporary register limits are reached.

---

## 9. Error Codes

The R-SPU subsystem uses error codes in the E7xx range. All errors are emitted
by the MIRR compiler during R-SPU code generation; the R-SPU hardware itself
does not produce software-visible error codes (it uses exception codes instead).

| Code | Message Pattern                                                          | Source                      |
|------|--------------------------------------------------------------------------|-----------------------------|
| E700 | *(R-SPU category fallback prefix)*                                       | `src/error.rs`              |
| E701 | `R-SPU register allocation failed: too many {kind} signals ({n} > {max}).` | `src/emit/rspu_regalloc.rs` |
| E702 | `R-SPU instruction budget exceeded: {n} instructions > {max}.`           | `src/emit/rspu.rs`          |
| E703 | `R-SPU guard resource exhausted: {n} guards > {max}.`                    | `src/emit/rspu.rs`          |
| E704 | `R-SPU expression exceeds maximum node count.`                           | `src/emit/rspu.rs`          |
| E705 | `R-SPU temporary registers exhausted.`                                   | `src/emit/rspu.rs`          |
| E706 | `R-SPU encoding field overflow: immediate/register exceeds bit-width.`   | `src/emit/rspu_encoding.rs` |
| E707 | `R-SPU unknown or reserved opcode in binary stream.`                     | `src/emit/rspu_encoding.rs` |
| E708 | `R-SPU tag violation: runtime type mismatch on tagged register.`         | `src/emit/rspu_tagged.rs`   |
| E709 | `R-SPU uninitialized register read.`                                     | `src/emit/rspu_tagged.rs`   |
| E710 | `R-SPU guard index out of bounds.`                                       | `src/emit/rspu_exceptions.rs` |
| E711 | `R-SPU exception depth overflow: nested traps exceed MAX_EXCEPTION_DEPTH.` | `src/emit/rspu_exceptions.rs` |
| E712 | `R-SPU simulation cycle budget exceeded (MAX_SIM_CYCLES).`               | `src/emit/rspu_sim.rs`      |
| E713 | `R-SPU trap handler table overflow: exceeds MAX_TRAP_HANDLERS.`          | `src/emit/rspu_exceptions.rs` |
| E714 | `R-SPU invalid execution mode for ModeSwitch.`                           | `src/emit/rspu_exceptions.rs` |
| E715 | `R-SPU deadline expired: DeadlineSet counter reached zero.`              | `src/emit/rspu_sim.rs`      |

All error codes E700–E715 are actively used in the current compiler and ISA simulator.
Codes E706–E715 were activated by MEGA-3 for the v2 instruction extensions: binary
encoding (E706–E707), tagged words (E708–E709), guard bounds (E710), exception model
(E711, E713–E714), simulation limits (E712), and temporal deadlines (E715).

---

## 10. See Also

- [Error Codes](error_codes.md) — Complete MIRR error code reference (E1xx–E8xx).
- [Roadmap](roadmap.md) — R-SPU compiler and EDA project roadmap.
- [Type System](type-system.md) — MIRR type rules behind E6xx errors and signal types.
- `src/emit/rspu_isa.rs` — Canonical Rust definition of the ISA types and constants.
- `src/emit/rspu.rs` — R-SPU code generation backend.
- `src/emit/rspu_regalloc.rs` — Bounded linear-scan register allocator.
