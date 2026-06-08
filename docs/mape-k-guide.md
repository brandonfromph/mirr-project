---
title: MAPE-K Simulation Guide
nav_order: 9
---

# MAPE-K Simulation Guide

MIRR includes a deterministic MAPE-K autonomic simulator that validates
whether hardware monitors can observe and respond to runtime anomalies.
Unlike purely static verification, the simulator exercises temporal
properties under fault injection, closing the loop between design-time
specification and runtime adaptation.

---

## Table of Contents

1. [What is MAPE-K?](#what-is-mape-k)
2. [Architecture](#architecture)
3. [The five components](#the-five-components)
4. [Bounded LTL properties](#bounded-ltl-properties)
5. [Signal predicates](#signal-predicates)
6. [Adaptation actions](#adaptation-actions)
7. [The neonatal monitoring scenario](#the-neonatal-monitoring-scenario)
8. [API entry points](#api-entry-points)
9. [Bounded iteration guarantees](#bounded-iteration-guarantees)
10. [Knowledge base and audit trail](#knowledge-base-and-audit-trail)

---

## What is MAPE-K?

MAPE-K is the canonical framework for self-adaptive systems, first
described by Kephart and Chess (2003). The acronym stands for:

- **M**onitor -- observe the system via sensors
- **A**nalyze -- evaluate temporal properties over observed data
- **P**lan -- select the best corrective action
- **E**xecute -- apply the selected action to the system
- **K**nowledge -- shared audit log of all decisions

These five phases execute in strict sequence on every simulation tick,
forming a feedback loop. The knowledge base is shared across all phases,
enabling post-run analysis and auditability.

The implementation lives in `src/mape_k/` (10 files, plus the `bridge/` logic, approximately 8,000 lines).

---

## Architecture

On each tick, the simulator executes the four MAPE phases in order,
with the knowledge base recording every adaptation decision:

```
+----------+   +----------+   +----------+   +----------+
| Monitor  | > | Analyze  | > |   Plan   | > | Execute  |
| (sample  |   | (LTL     |   | (select  |   | (apply   |
|  sensors)|   |  check)  |   |  action) |   |  action) |
+----------+   +----------+   +----------+   +----------+
      ^                                            |
      +------------ Knowledge Base <---------------+
```

Given system state at tick t, the state transition is:

```
state(t+1) = Execute(Plan(Analyze(Monitor(state(t), sensors), K(t)), K(t)), K(t))
```

where Monitor samples sensor values into rolling windows, Analyze
evaluates bounded LTL properties over those windows, Plan selects the
highest-priority applicable action, and Execute applies the chosen
adaptation to the signal state.

---

## The five components

### Monitor (`src/mape_k/monitor.rs`)

The Monitor samples all registered sensor channels each tick and stores
values in fixed-capacity ring buffers -- one per signal. The ring
buffer overwrites the oldest value when full, maintaining a sliding
window of recent history.

Key types:

- `Monitor` -- holds per-signal rolling windows and the tick counter.
- `RingBuffer` -- fixed-capacity ring buffer of `u64` values.

The window size is configurable but clamped to `MAX_WINDOW_SIZE` (1,024).
A maximum of `MAX_SENSORS` (64) channels can be tracked.

### Analyzer (`src/mape_k/analyzer.rs`)

The Analyzer evaluates a set of bounded temporal properties against the
Monitor's rolling windows each tick. It returns a `PropertyResult` for
each registered property indicating whether it is satisfied or violated,
along with the evidence tick.

Key types:

- `Analyzer` -- holds the property set and evaluates them.
- `PropertyResult` -- satisfied/violated status with evidence tick offset.

Maximum properties: `MAX_PROPERTIES` (256).

### Planner (`src/mape_k/planner.rs`)

The Planner holds a fixed-size action table and selects the
highest-priority matching action for the current set of property
results. Actions are never synthesized on the fly -- they come from a
pre-verified library.

Each action entry specifies:
- Which property index triggers it
- Whether it fires on violation or on satisfaction
- A priority (0-255; higher wins ties)

Key types:

- `Planner` -- holds the action table, performs selection.
- `ActionEntry` -- maps a property result to an action.
- `PlanResult` -- the selected action (if any) with context.

Maximum entries: `MAX_ACTION_ENTRIES` (256).

### Executor (`src/mape_k/executor.rs`)

The Executor applies the planner-selected action to the signal
environment. It validates action targets against the declared signal
set and captures before/after state snapshots for the knowledge base.

Three action types:

| Action | Effect |
|--------|--------|
| `SetSignal { name, value }` | Force a specific signal to a value |
| `SwitchMode { mode_name }` | Switch to a named operating mode (recorded, not directly applied) |
| `EmergencyStop` | Zero all signals immediately (safety clamp) |

Key types:

- `Executor` -- applies actions, tracks emergency state.
- `ExecutionRecord` -- before/after snapshots and success status.

### Knowledge Base (`src/mape_k/knowledge.rs`)

The Knowledge Base maintains a bounded FIFO ring buffer of adaptation
records for audit logging. Each record stores the tick, triggering
property index, action taken, pre/post state snapshots, and success
flag.

When the buffer reaches capacity, the oldest entry is evicted. A
monotonic counter tracks lifetime insertions (including evicted records)
for completeness audits.

Key types:

- `KnowledgeBase` -- bounded audit log.
- `AdaptationRecord` -- one decision record with full context.

Maximum entries: `MAX_LOG_ENTRIES` (4,096). The knowledge base exports
to JSON for DO-254 evidence trails.

---

## Bounded LTL properties

The Analyzer evaluates six bounded temporal operators over rolling
signal windows. All algorithms are iterative with O(window_size)
complexity per property per tick.

### Always -- G(P)

P must hold at every tick in the current window. The analyzer scans
the entire window and returns a violation on the first tick where P
is false.

```rust
TemporalProperty::Always(SignalPredicate::GreaterThan("pressure".into(), 50))
```

This checks: "pressure must be greater than 50 at every tick in the
window."

### EventuallyWithin -- F<=N(P)

P must become true at least once within the last N ticks. The analyzer
inspects the last N entries and returns satisfied on the first tick
where P is true.

```rust
TemporalProperty::EventuallyWithin(
    SignalPredicate::IsTrue("alarm_ack".into()),
    100,
)
```

This checks: "the alarm acknowledgment signal must become nonzero at
least once within the last 100 ticks."

### Persists -- P for N

P must hold for at least N consecutive ticks anywhere in the window.
The analyzer tracks consecutive satisfying ticks and fires when the
count reaches N.

```rust
TemporalProperty::Persists(
    SignalPredicate::LessThan("pressure".into(), 50),
    10,
)
```

This checks: "pressure has been below 50 for at least 10 consecutive
ticks somewhere in the window." This corresponds directly to the MIRR
`guard ... for N cycles` construct.

### AlwaysImplies -- G(P -> Q)

Whenever P holds, Q must also hold in the exact same cycle. The analyzer
evaluates both predicates on each tick in the window and returns a
violation if P is true but Q is false.

```rust
TemporalProperty::AlwaysImplies(
    SignalPredicate::IsTrue("alarm_active".into()),
    SignalPredicate::IsTrue("buzzer_on".into()),
)
```

### NeverImplies -- never (P -> Q)

Ensures that P implies Q is *never* universally true. There must exist at least
one tick in the window where P is true and Q is false. Violation occurs if
no such counter-example exists.

### AlwaysFollowedBy -- always (P followed_by N Q)

Whenever P holds, Q must hold exactly N cycles later. The analyzer
checks each tick where P is true, looks ahead N ticks within the window,
and returns a violation if Q is false or the window ends before N ticks.

```rust
TemporalProperty::AlwaysFollowedBy(
    SignalPredicate::IsTrue("button_press".into()),
    5,
    SignalPredicate::IsTrue("led_on".into()),
)
```

---

## Signal predicates

Atomic propositions are evaluated against a single `u64` signal value
each tick:

| Predicate | Syntax | True when |
|-----------|--------|-----------|
| `IsTrue(signal)` | `value != 0` | Signal is nonzero (boolean true) |
| `LessThan(signal, k)` | `value < k` | Signal is below threshold |
| `GreaterThan(signal, k)` | `value > k` | Signal is above threshold |
| `InRange(signal, lo, hi)` | `lo <= value <= hi` | Signal is within inclusive range |

---

## Adaptation actions

The planner selects from three action types:

### SetSignal

Force a signal to a specific value. The executor validates that the
target signal name exists before applying.

```rust
AdaptationAction::SetSignal { name: "clamp_valve".into(), value: 1 }
```

### SwitchMode

Switch to a named operating mode. In the R-SPU vision, this selects a
pre-synthesized bitstream. In simulation, the mode switch is recorded
but signal state is not directly modified.

```rust
AdaptationAction::SwitchMode { mode_name: "high_precision".into() }
```

### EmergencyStop

Immediate safety clamp -- zeros all signals to safe defaults. Maps to
the R-SPU "Immediate Layer (Static)" reflex. Once triggered, the
emergency flag remains active until explicitly cleared.

```rust
AdaptationAction::EmergencyStop
```

### Trigger conditions

Each action entry specifies whether it fires on property violation or
property satisfaction:

- `TriggerCondition::OnViolation` -- fire when the property is violated
  (safety invariant broken). This is the default.
- `TriggerCondition::OnSatisfaction` -- fire when the property is
  satisfied (a dangerous condition has been detected and confirmed).

---

## The neonatal monitoring scenario

The MIRR simulator includes a built-in neonatal airway-pressure scenario
that exercises the full MAPE-K loop.

### Scenario configuration

- **Sensor:** One channel (`airway_pressure`), base value 120,
  noise amplitude 5, fault injection at tick 500 (value drops to 10).
- **Properties:**
  1. `Always(pressure > 50)` -- invariant: pressure must never drop
     below 50.
  2. `Persists(pressure < 50, 10)` -- sustained anomaly: low pressure
     for 10 consecutive ticks.
- **Actions:**
  1. `SetSignal("clamp_valve", 1)` -- priority 50, fires on invariant
     violation.
  2. `EmergencyStop` -- priority 100, fires on sustained low pressure.

### Expected behavior

1. **Ticks 0-499:** Normal operation. Pressure fluctuates around 120
   (within noise range 115-125). No violations.
2. **Tick 500:** Fault injected. Pressure drops to 10. The Always
   property is immediately violated. The `SetSignal` action fires
   (lower priority).
3. **Tick 510:** The low-pressure condition has persisted for 10
   consecutive ticks. The Persists property fires. `EmergencyStop`
   (higher priority) is selected, zeroing all signals.
4. **Simulation halts** after emergency stop.

### Sensor model

Each sensor channel uses a deterministic Linear Congruential Generator
(LCG) PRNG. Identical seeds produce identical output across runs,
ensuring full reproducibility. Fault injection is configured by
specifying a start tick, fault value, and optional end tick.

```rust
SensorConfig {
    name: "airway_pressure".into(),
    base_value: 120,
    noise_amplitude: 5,
    fault_at_tick: Some(500),
    fault_value: 10,
    fault_end_tick: None,
    seed: 42,
}
```

---

## API entry points

### Running a simulation

```rust
use mirr::mape_k::*;

let config = SimConfig {
    sensors: vec![
        SensorConfig {
            name: "airway_pressure".into(),
            base_value: 120,
            noise_amplitude: 5,
            fault_at_tick: Some(500),
            fault_value: 10,
            fault_end_tick: None,
            seed: 42,
        },
    ],
    properties: vec![
        TemporalProperty::Always(
            SignalPredicate::GreaterThan("airway_pressure".into(), 50),
        ),
        TemporalProperty::Persists(
            SignalPredicate::LessThan("airway_pressure".into(), 50),
            10,
        ),
    ],
    action_table: vec![
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal {
                name: "clamp_valve".into(),
                value: 1,
            },
            priority: 50,
            trigger_on: TriggerCondition::OnViolation,
        },
        ActionEntry {
            trigger_property_idx: 1,
            action: AdaptationAction::EmergencyStop,
            priority: 100,
            trigger_on: TriggerCondition::OnSatisfaction,
        },
    ],
    window_size: 64,
    knowledge_capacity: 1024,
};

let mut sim = MapeKSimulator::new(config);
let result = sim.run(10_000);
println!("{}", result.summary());
```

### Reading the result

The `SimResult` struct contains:

| Field | Type | Description |
|-------|------|-------------|
| `total_ticks` | `u64` | Number of ticks simulated |
| `total_violations` | `u64` | Total property violations detected |
| `total_adaptations` | `u64` | Total adaptation actions executed |
| `emergency_triggered` | `bool` | Whether emergency stop fired |
| `emergency_tick` | `Option<u64>` | Tick at which emergency occurred |
| `final_signal_state` | `Vec<(String, u64)>` | Signal values at end of simulation |
| `adaptation_log` | `Vec<AdaptationRecord>` | Full audit trail from knowledge base |

### Exporting the audit trail

```rust
let json = sim.knowledge.to_json()?;
std::fs::write("audit_trail.json", json)?;
```

---

## Bounded iteration guarantees

Every loop and buffer in `src/mape_k/` is bounded by a named constant,
per NASA Power-of-10 compliance:

| Constant | Value | Controls |
|----------|-------|----------|
| `MAX_TICKS` | 10,000,000 | Maximum simulation ticks per run |
| `MAX_WINDOW_SIZE` | 1,024 | Rolling window capacity per signal |
| `MAX_SENSORS` | 64 | Maximum sensor channels |
| `MAX_PROPERTIES` | 256 | Maximum temporal properties |
| `MAX_ACTION_ENTRIES` | 256 | Maximum action table entries |
| `MAX_LOG_ENTRIES` | 4,096 | Knowledge base capacity |

The tick count passed to `run()` is clamped to `MAX_TICKS`. Window
sizes are clamped to `MAX_WINDOW_SIZE`. Excess properties and action
entries beyond their limits are silently truncated.

No unbounded iteration exists in the simulator. The main loop runs
for at most `MAX_TICKS` iterations. Each inner phase (monitor, analyze,
plan, execute) iterates over at most `MAX_SENSORS`, `MAX_PROPERTIES`,
or `MAX_ACTION_ENTRIES` items respectively.

---

## Knowledge base and audit trail

Every adaptation decision is recorded as an `AdaptationRecord`:

| Field | Type | Description |
|-------|------|-------------|
| `tick` | `u64` | When the adaptation occurred |
| `trigger_property_idx` | `usize` | Which property triggered it |
| `trigger_description` | `String` | Human-readable trigger label |
| `action` | `AdaptationAction` | What was done |
| `success` | `bool` | Whether it succeeded |
| `pre_state` | `Vec<(String, u64)>` | Signal state before |
| `post_state` | `Vec<(String, u64)>` | Signal state after |

The knowledge base serializes to JSON for integration with DO-254
evidence trails and other certification workflows.

---

## See Also

- [Tutorial](tutorial) -- Getting started with MIRR
- [R-SPU Reference](rspu_isa_spec) -- Instruction set architecture
- [Contributing](contributing) -- Coding standards and campaign workflow
