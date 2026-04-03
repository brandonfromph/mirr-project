# Untracked Artifacts Overview (2026-04-03)

This file explains the purpose of the untracked artifacts that were generated during Proposal 096/097 execution, review, and evidence capture.

## 1) Proposal 096 Inventory and Coverage Snapshots
Files:
- `.proposal096_*` (counts, file lists, sampled high-risk, unread-high-risk, hit index)

Purpose:
- Repository-wide inventory/census outputs used to quantify review coverage and risk-sampling.
- Traceability snapshots for what was scanned in large-scale review passes.

## 2) Wave and Delivery Summaries
Files:
- `DELIVERY_SUMMARY.md`
- `DETAILED_LINE_BY_LINE_SUMMARY.md`
- `QUICK_SUMMARY.md`
- `START_HERE_PROPOSAL_096.md`
- `README_PROPOSAL_096_REVIEW.md`
- `WAVE_*` reports
- `WORK_COMPLETION_CERTIFICATE_2026-04-02.md`

Purpose:
- Human-readable closeout and navigation docs created for campaign handoff and reviewer onboarding.

## 3) Gate and Validation Logs
Files:
- `cargo_check_output.txt`
- `check_output.txt`
- `ci-gate-results.json`
- `ci-gate.txt`
- `fuzz_check_output.txt`
- `git_status_out.txt`
- `metrics_output.txt`
- `parity_full_output.txt`
- `commit_msg.txt`

Purpose:
- Captured command outputs for reproducibility and post-hoc verification of gate outcomes.

## 4) Execution Wrappers and Phase Scripts
Files:
- `execute-wave-gates.ps1`
- `execute_all_gates.sh`
- `execute_p096.bat`
- `execute_p096.ps1`
- `phase1_baseline.ps1`
- `phase4_ci_steps.ps1`
- `phase6_regression.ps1`
- `run_critical_gates.sh`
- `run_wave_gates.ps1`
- `run_wave_gates.sh`
- `test_gates.ps1`

Purpose:
- Convenience wrappers and phase scripts used to run canonical and phase-specific gates.

## 5) Review Scratch Workspace
Files:
- `review_tmp/*` (diff captures, line-index extracts, decoded snippets)

Purpose:
- Temporary review material generated to inspect and annotate file-level changes.

## 6) Test and Fixture Additions (RWFI2 / Wave 6)
Files:
- `tests/fixtures/netlist/*.json` (new netlist fixtures)
- `tests/mega11_temporal_lowering_parity_tests.rs`
- `tests/test_accel_eda_reporting_tests.rs`
- `tests/test_accel_full_gate_tests.rs`
- `tests/test_accel_gate_contract_tests.rs`

Purpose:
- Added fixture corpus and gate-coverage tests introduced during Wave 6/7 contract acceleration work.

## 7) Utility Scripts and Repo Metrics Tools
Files:
- `scripts/repo_metrics.py`
- `scripts/review_coverage_gate.py`
- `scripts/run_upgraded_096_review.ps1`
- `scripts/validate_proposals.py`

Purpose:
- Support tools for repository metrics, review quality gates, and proposal consistency checks.

## 8) Auxiliary Repo-State Files
Files:
- `.full-repo-tree.txt`
- `.mirr_brain.json`
- `.task_probe.txt`
- `package.json`
- `fuzz/Cargo.lock`
- `temp_committer.txt`
- `memories/session/timing-research.md`

Purpose:
- Mixed metadata/state artifacts produced by tooling and local execution context.

## Notes
- These artifacts are primarily review/evidence/support outputs, not core compiler runtime sources.
- They were preserved intentionally for auditability and campaign traceability.
