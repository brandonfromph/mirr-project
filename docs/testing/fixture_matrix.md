# Test Fixture Taxonomy & Coverage Matrix

> **Status:** Draft  
> **Version:** 0.1  
> **Date:** 2026-03-01  
> **Author(s):** MIRR Core Team  
> **Related Milestone:** Post-Milestone Stream 4 (Additional Fixtures)

---

## 1. Purpose

This document catalogs every test fixture in the project, classifies it by
category, and maps it to the language/IR features it exercises. The coverage
matrix identifies blind spots so new fixtures can be prioritized.

## 2. Fixture Categories

| Category | Code | Description |
|----------|------|-------------|
| Normal | N | Valid program exercising standard features |
| Edge | E | Valid program at feature boundaries (e.g., threshold values) |
| Adversarial | A | Unusual but valid input designed to stress the parser/compiler |
| Error-Recovery | R | Invalid input; verifies error detection and messaging |

## 3. Current Fixtures

### 3.1 Example Programs (`examples/`)

| File | Category | Signals | Guards | Reflexes | Guard Strategy | Notes |
|------|----------|---------|--------|----------|---------------|-------|
| `neonatal_respirator.mirr` | N | 3 (2 in, 1 out) | 1 | 1 | Counter (1000 cycles) | Canonical example; all golden fixtures derived from this |

### 3.2 Golden Fixtures (`tests/fixtures/`)

| Fixture | Path | Derived from | IR Level | Schema validated? |
|---------|------|-------------|----------|------------------|
| Parsed AST | `tests/fixtures/parse/neonatal_respirator_parsed.json` | `neonatal_respirator.mirr` | Level 1 (AST) | ✅ |
| Temporal netlist | `tests/fixtures/netlist/neonatal_respirator.json` | `neonatal_respirator.mirr` | Level 3 (Netlist) | ✅ |

### 3.3 Inline Test Fixtures (in test files)

| Test file | Fixture type | Category | What it covers |
|-----------|-------------|----------|----------------|
| `tests/expr_tests.rs` | Inline expressions | N, E | 17 expression parser cases |
| `tests/module_tests.rs` | Inline modules | N, R | 23 module parser + error cases |
| `tests/validation_tests.rs` | Inline modules | N, R | 9 semantic validation cases |
| `tests/stress_tests.rs` | Inline programs | A | 5 stress/edge-case scenarios |
| `tests/self_hosting_parity_tests.rs` | File-based | N, R | 13 parity tests |
| `tests/self_hosting_ir_schema_tests.rs` | File-based | N | Schema conformance |
| `tests/temporal_lowering_tests.rs` | Inline + file | N | Guard compilation + netlist parity |
| `tests/temporal_emit_tests.rs` | Inline | N | Temporal IR emission |

## 4. Coverage Matrix

### 4.1 Language Features

| Feature | `neonatal_respirator` | Planned: shift-register | Planned: multi-guard | Planned: errors |
|---------|:--------------------:|:----------------------:|:-------------------:|:--------------:|
| `signal` (in bool) | ✅ | ✅ | ✅ | — |
| `signal` (in uN) | ✅ (u16) | ✅ (u8) | ✅ (u16, u32) | — |
| `signal` (out bool) | ✅ | ✅ | ✅ | — |
| `signal` (internal) | — | — | ✅ | — |
| `guard` (short delay, ≤16 cycles) | — | ✅ | ✅ | — |
| `guard` (long delay, >16 cycles) | ✅ (1000) | — | ✅ (500) | — |
| `guard` (simple signal condition) | — | ✅ | ✅ | — |
| `guard` (comparison condition) | ✅ (< 50) | — | ✅ | — |
| `guard` (compound condition &&/||) | — | — | ✅ | — |
| `guard` (negated condition !) | — | ✅ | — | — |
| `reflex` (single guard) | ✅ | ✅ | ✅ | — |
| `reflex` (multi-guard `and`) | — | — | ✅ | — |
| Multiple guards per module | — | — | ✅ | — |
| Multiple reflexes per module | — | — | ✅ | — |

### 4.2 Temporal Lowering Strategies

| Strategy | `neonatal_respirator` | Planned: shift-register | Planned: multi-guard |
|----------|:--------------------:|:----------------------:|:-------------------:|
| Counter (>16 cycles) | ✅ | — | ✅ |
| Shift register (≤16 cycles) | — | ✅ | ✅ |
| Mixed (both in one module) | — | — | ✅ |

### 4.3 Error Detection

| Error class | Inline tests | Planned: file fixture |
|-------------|:-----------:|:---------------------:|
| Syntax error (malformed token) | ✅ (module_tests) | ✅ (malformed_input.mirr) |
| Syntax error (incomplete guard) | ✅ (module_tests) | — |
| Duplicate signal name | ✅ (validation_tests) | ✅ (validation_errors.mirr) |
| Undeclared signal reference | ✅ (validation_tests) | ✅ (validation_errors.mirr) |
| Undeclared guard reference | ✅ (validation_tests) | ✅ (validation_errors.mirr) |
| Type mismatch in expression | ✅ (validation_tests) | — |
| Missing file | ✅ (parity_tests) | — |
| Empty module | ✅ (stress_tests) | — |

## 5. Identified Gaps (Priority Order)

| Priority | Gap | Proposed fixture | Category |
|----------|-----|-----------------|----------|
| **P1** | No shift-register guard coverage in file fixtures | `examples/shift_register_guard.mirr` | E |
| **P1** | No multi-guard / multi-reflex coverage | `examples/multi_guard_monitor.mirr` | N |
| **P2** | No file-based error recovery fixtures | `examples/malformed_input.mirr` | R |
| **P2** | No file-based validation error fixtures | `examples/validation_errors.mirr` | R |
| **P3** | No internal signal usage in file fixtures | (covered by multi-guard) | N |
| **P3** | No compound guard condition (&&/||) in file fixtures | (covered by multi-guard) | N |
| **P3** | No negated guard condition (!) in file fixtures | (covered by shift-register) | E |

## 6. Planned Fixtures

### 6.1 `examples/shift_register_guard.mirr`

```mirr
module short_delay_monitor {
    signal sensor_active: in bool;
    signal alert_lamp:    out bool;

    guard brief_activation {
        when sensor_active
        for  8 cycles;
    }

    reflex activate_alert {
        on brief_activation {
            alert_lamp = true;
        }
    }
}
```

**Coverage:** shift-register strategy, simple signal condition, short delay.  
**Expected netlist:** 1 shift-register guard with 8 stages.

### 6.2 `examples/multi_guard_monitor.mirr`

```mirr
module patient_monitor {
    signal heart_rate:      in u16;
    signal blood_pressure:  in u16;
    signal alarm_active:    out bool;
    signal pump_override:   out bool;
    signal status_flag:     internal bool;

    guard bradycardia {
        when heart_rate < 60
        for  500 cycles;
    }

    guard hypotension {
        when blood_pressure < 90
        for  12 cycles;
    }

    reflex cardiac_alarm {
        on bradycardia {
            alarm_active = true;
        }
    }

    reflex emergency_override {
        on bradycardia and hypotension {
            pump_override = true;
        }
    }
}
```

**Coverage:** multi-guard, multi-reflex, mixed strategies (counter + shift-register), `and` combinator, internal signal.  
**Expected netlist:** 1 counter guard (500 cycles) + 1 shift-register guard (12 cycles).

### 6.3 `examples/malformed_input.mirr`

```
module broken_syntax {
    signal x: in bool
    guard missing_brace {
        when x
        for 10 cycles
}
```

**Coverage:** parser error recovery — missing semicolons, unmatched braces.  
**Expected behavior:** parse failure with actionable error message.

### 6.4 `examples/validation_errors.mirr`

```mirr
module duplicate_signals {
    signal sensor_a: in bool;
    signal sensor_a: in u16;
    signal output_x: out bool;

    guard undefined_ref {
        when nonexistent_signal
        for 5 cycles;
    }

    reflex bad_guard_ref {
        on no_such_guard {
            output_x = true;
        }
    }
}
```

**Coverage:** duplicate signal name, undeclared signal reference, undeclared guard reference.  
**Expected behavior:** validation failure with ≥3 diagnostics.

## 7. Golden Fixture Update Policy

See `docs/runbooks/golden_fixture_update.md` for the controlled update procedure.

## 8. Fixture Naming Convention

```
tests/fixtures/<ir_level>/<program_name>_<suffix>.json
```

| IR level | Directory | Suffix examples |
|----------|-----------|----------------|
| Tokens | `tokens/` | `_tokens.json` |
| AST | `ast/` | `_ast.json` |
| Parse | `parse/` | `_parsed.json` |
| Semantic | `semantic/` | `_validated.json` |
| Netlist | `netlist/` | `.json` (no suffix, legacy) or `_netlist.json` |

---

*Document version: 0.1 (Draft) — see `docs/INDEX.md` for governance rules.*