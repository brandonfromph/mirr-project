# Runbook: Parity Triage

> **Status:** Draft  
> **Last updated:** 2026-03-01  
> **Applies to:** Any failure in `tests/self_hosting_parity_tests.rs` or future stage-2 parity tests

---

## 1. Purpose

This runbook describes how to diagnose and resolve discrepancies between the
**Rust reference pipeline** and the **MIRR-CORE pipeline** (currently bootstrap
runner; future interpreter). It ensures that parity failures are triaged
systematically rather than through ad hoc debugging.

## 2. When to Use

- A parity test fails in CI or local `cargo test`.
- A new fixture produces different output between pipelines.
- A schema validation test fails on pipeline output.

## 3. Triage Flowchart

```
Parity test fails
    │
    ▼
Is the failure in the Rust pipeline or the MIRR-CORE pipeline?
    │
    ├── Rust pipeline changed recently?
    │       │
    │       YES → Was the change intentional?
    │               │
    │               YES → Update golden fixture (see golden_fixture_update.md)
    │               │      + bump IR version if schema changed
    │               │
    │               NO  → Revert Rust change; it introduced a regression
    │
    ├── MIRR-CORE pipeline changed recently?
    │       │
    │       YES → The MIRR-CORE port has a bug.
    │              Debug by comparing stage-by-stage output (see §4).
    │       │
    │       NO  → Environment or dependency issue (see §5).
    │
    └── Neither changed?
            │
            → Non-determinism bug. Escalate immediately (see §6).
```

## 4. Stage-by-Stage Comparison

The bootstrap runner reports 5 stages. Isolate which stage diverges:

```bash
# Run self-host pipeline with JSON output
cargo run -- --selfhost-compile --selfhost-compile-json examples/neonatal_respirator.mirr > selfhost_output.json

# Run Rust reference pipeline (parse + temporal lower)
cargo run -- examples/neonatal_respirator.mirr --emit-json > rust_output.json

# Diff
diff selfhost_output.json rust_output.json
```

### Stage isolation

| Stage | What to check | Common failure modes |
|-------|--------------|---------------------|
| Read | File encoding, line endings, BOM | CR/LF mismatch on Windows |
| Parse | AST JSON structure | Operator precedence, missing expression variant |
| Validate | Diagnostic codes and messages | New validation rule not ported to MIRR-CORE |
| TemporalLower | Netlist JSON structure | Threshold miscalculation (shift vs counter boundary) |
| FixtureParity | Golden fixture match | Fixture stale after intentional Rust change |

## 5. Environment Issues

- **Rust version mismatch:** Check `rustc --version` matches CI.
- **serde_json formatting:** Ensure both pipelines use the same serialization
  settings (no pretty-print vs compact mismatch).
- **File system:** Windows path separators, file encoding, line endings.

## 6. Non-Determinism Escalation

If the same input produces different output on repeated runs with no code
changes, this is a **critical safety violation** (NASA determinism rule).

**Immediate actions:**
1. Capture both outputs and the exact command used.
2. File an issue tagged `P0-safety` and `non-determinism`.
3. Bisect to the commit that introduced non-determinism.
4. Block releases until resolved.

## 7. Resolution Checklist

- [ ] Root cause identified and documented in the issue/PR.
- [ ] Fix applied to the correct pipeline (Rust or MIRR-CORE).
- [ ] Golden fixtures updated if the Rust pipeline changed intentionally.
- [ ] All parity tests pass after fix.
- [ ] No regressions in other test suites.
- [ ] `docs/INDEX.md` updated if any docs changed.

---

*Runbook version: 1.0 — see `docs/INDEX.md` for governance rules.*