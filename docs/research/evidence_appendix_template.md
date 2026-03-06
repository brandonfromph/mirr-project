# Evidence Appendix Template (Preprint / DAC Submission)

> **Status:** Draft  
> **Version:** 0.1  
> **Use:** Copy this into your paper appendix and fill from `artifacts/research/*`.

---

## A. Reproducibility Metadata

- Repository URL: `...`
- Commit hash: `...`
- OS / CPU / RAM: `...`
- Rust version: `...`
- Python version: `...`

### Reproduction commands

```bash
cargo build --release
cargo test --test self_hosting_parity_tests
cargo test --test self_hosting_ir_schema_tests
py -3 scripts/research/run_experiments.py --skip-build   # Windows
# or
python3 scripts/research/run_experiments.py --skip-build # POSIX
```

---

## B. Experiment E1 — Temporal Strategy Sweep

**Question:** Does the implementation choose strategy according to threshold rule (`N<=16` shift-register, else counter)?

**Input set:** delays = `...`

**Output artifact:** `artifacts/research/strategy_sweep.csv`

### Table B1 — Strategy correctness

| Metric | Value |
|---|---:|
| Total cases | ... |
| Matched expected strategy | ... |
| Accuracy | ... |

### Figure B1 — Delay vs strategy (optional)

Plot from CSV: x-axis delay cycles, y-axis chosen strategy.

---

## C. Experiment E2 — Determinism

**Question:** Does repeated compilation of same input produce byte-identical JSON output?

**Input:** `examples/neonatal_respirator.mirr`, repeated `R` runs.

**Output artifact:** `artifacts/research/determinism_runs.csv`

### Table C1 — Determinism summary

| Metric | Value |
|---|---:|
| Runs | ... |
| Unique hashes | ... |
| Mismatch count | ... |

---

## D. Experiment E3 — Throughput Baseline

**Question:** What is current compile-time baseline for implemented pipeline?

**Output artifact:** `artifacts/research/throughput_baseline.csv`

### Table D1 — Runtime summary per fixture

| Fixture | Mean (ms) | Median (ms) | p95 (ms) | Stddev (ms) |
|---|---:|---:|---:|---:|
| ... | ... | ... | ... | ... |

---

## E. Experiment E4 — Bootstrap Failure Modes

**Question:** Does staged bootstrap fail safely and at expected stage for faulty inputs?

**Output artifact:** `artifacts/research/bootstrap_failure_modes.csv`

### Table E1 — Failure-mode behavior

| Case | Expected success | Observed success | First failed stage |
|---|---:|---:|---|
| canonical_example | True | ... | ... |
| malformed_parse_error | False | ... | ... |
| missing_file_read_error | False | ... | ... |

---

## F. Claims Boundary Statement

Use this exact structure:

1. **Implemented + measured in this work:** compiler parsing/validation, temporal lowering, deterministic IR artifacts, stage-1 self-host verification.
2. **Supported by external literature (not reproduced here):** Cement2 timing closure, SmaRTLy area reduction, FIRWINE formal proofs.
3. **Future work:** MIRR interpreter stage-2, RTL backend, FPGA DPR validation, clinical deployment.

Reference: `docs/research/claims_evidence_matrix.md`

---

## G. Threats to Validity

1. No FPGA/silicon measurements in this artifact package.
2. Throughput measured on host machine (environment-specific).
3. Fixture corpus still limited; expanded suite planned in Stream 4.

---

## H. Artifact Checklist (for submission)

- [ ] Include `artifacts/research/summary.md`
- [ ] Include all CSV files
- [ ] Include command transcript/logs
- [ ] Include commit hash + environment block
- [ ] Include claims boundary paragraph in main paper
