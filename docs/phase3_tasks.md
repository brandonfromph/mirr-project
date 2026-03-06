# Phase 3 — Task Implementation (10 tasks, owners, detailed steps)

This file expands the Phase 3 plan into concrete, actionable implementation steps for each task. Owners and estimates are assigned as placeholders; adjust during planning.

Task 1 — Requirements Refinement & Safety Analysis
- Owner: Product Lead
- Estimate: 5d
- Deliverables: requirements doc, RTM, HA entries, acceptance criteria matrix
- Steps:
  1.1 Collect artifacts: docs/phase3_plan.md, phase2 outputs, stakeholder notes.
  1.2 Enumerate each requirement; create PH3-REQ-### rows in docs/requirements_rtm.md.
  1.3 For each requirement: write measurable acceptance criteria (pass/fail) and verification method.
  1.4 Add cross-links to code modules and tests.
  1.5 Prepare safety review packet and schedule safety-review meeting.
  1.6 Update RTM after review; mark sign-off.
- Immediate next action: assign owners to outstanding RTM rows and schedule meeting.

Task 2 — Architecture & Interface Design
- Owner: Architect
- Estimate: 6d
- Deliverables: updated architecture diagrams, module interface specs, resource budgets
- Steps:
  2.1 Produce module decomposition with one responsibility per module (single-sheet rule).
  2.2 For each module, write interface spec: inputs, outputs, pre/postconditions, error returns.
  2.3 For each loop/control construct, document fixed upper bounds and rationale.
  2.4 Establish per-module resource budgets (stack, heap, CPU worst-case).
  2.5 Peer review interfaces; record action items and update docs.
- Immediate next action: draft module list and interfaces for critical runtime modules (src/mirr_runtime.rs, src/mirr_executor.rs).

Task 3 — Temporal Guard Compiler & Backend Integration
- Owner: Compiler Lead
- Estimate: 8d
- Deliverables: deterministic lowering passes, regression tests, baseline numbers
- Steps:
  3.1 Review temporal lowering code paths (src/temporal/*) and map to MIRR spec.
  3.2 Implement deterministic scheduling policy and fixed iteration guards.
  3.3 Add explicit assertions at pass boundaries validating invariants.
  3.4 Add unit/regression tests covering typical and edge guard patterns.
  3.5 Measure performance; record baseline in artifacts/research.
- Immediate next action: add failing regression test that encodes current nondeterminism to drive fixes.

Task 4 — Self-hosting & Porting Validation
- Owner: Porting Lead
- Estimate: 6d
- Deliverables: reproducible self-hosting build, parity reports
- Steps:
  4.1 Audit compiler_mirr/PORTING_STEPS.md and map open items.
  4.2 Implement remaining porting steps in small patches with tests.
  4.3 Create CI job that performs a self-hosting build and runs parity tests.
  4.4 Triage mismatches, fix iteratively, log defects.
  4.5 Finalize reproducible build script and document invocation.
- Immediate next action: create CI job draft and run self-hosting locally to gather failures.

Task 5 — Determinism & Throughput Benchmarking
- Owner: Research Lead
- Estimate: 5d
- Deliverables: determinism harness, throughput benchmarks, runbook
- Steps:
  5.1 Define determinism protocol: seed policy, repeat counts, statistical thresholds.
  5.2 Extend scripts/research/run_experiments.py to support repeatable runs and logging.
  5.3 Create inputs suite (small, medium, stress) under artifacts/research/tmp_inputs.
  5.4 Run sweeps and collect artifacts/research/determinism_runs.csv and throughput_baseline.csv.
  5.5 Produce report with detected nondeterministic sources and mitigation plan.
- Immediate next action: add deterministic-seed flag to experiment runner.

Task 6 — Formal/Static Verification & Assertion Expansion
- Owner: Verification Engineer
- Estimate: 7d
- Deliverables: assertion additions, CI static checks, verification report
- Steps:
  6.1 Identify critical functions (runtime, temporal compiler, emitter) and add >=2 assertions per function where meaningful.
  6.2 Add parameter validation on all public APIs; ensure callers check return values.
  6.3 Integrate static checks (rustfmt, clippy, custom grep for assertions) into CI and fail on warnings.
  6.4 Run a pass to detect heap allocation sites; flag any post-init allocations.
  6.5 Produce verification report and mark PH3-REQ verification statuses.
- Immediate next action: add CI step that runs clippy with deny-warnings.

Task 7 — Resource-Bounded Implementation & Memory Safety
- Owner: Runtime Lead
- Estimate: 5d
- Deliverables: allocation policy, fixed pools, runtime monitors
- Steps:
  7.1 Inventory allocations using instrumentation (bench + debug build).
  7.2 Refactor hot-path allocations into preallocated pools during init.
  7.3 Add runtime asserts that verify pool usage never exceeds capacity.
  7.4 Add tests simulating boundary conditions and failure modes.
  7.5 Update developer docs with allocation policy and examples.
- Immediate next action: run allocation profiler on core benchmark and produce list of dynamic allocations.

Task 8 — Testing Matrix Expansion & Golden Fixture Management
- Owner: Test Lead
- Estimate: 6d
- Deliverables: expanded tests, fixture-update runbook, CI automation
- Steps:
  8.1 Review current test coverage and identify gaps (temporal, resource bounds, failure modes).
  8.2 Add focused unit tests and small integration tests; each test maps to PH3-REQ entries.
  8.3 Create a guarded golden-update script and a runbook (docs/runbooks/golden_fixture_update.md).
  8.4 Add nightly CI job to run full matrix and record results.
  8.5 Implement parity triage checklist and automation for failing cases.
- Immediate next action: add a small temporal guard edge-case test to tests/ and link to RTM.

Task 9 — Documentation, Runbooks & Training
- Owner: Tech Writer
- Estimate: 4d
- Deliverables: developer guide, operator runbooks, training materials
- Steps:
  9.1 Draft developer guide with Power-of-10 checklist and code-review checklist.
  9.2 Write runbooks for build_selfhost, experiment replication, golden fixture update.
  9.3 Create quick-start tutorial to reproduce benchmarks and determinism tests.
  9.4 Schedule a 1hr walkthrough with engineering team and record notes.
  9.5 Finalize and version docs in repo.
- Immediate next action: create docs/runbooks/ directory and add golden_fixture_update.md stub.

Task 10 — Release Preparation & Post-Milestone Plan
- Owner: Release Manager
- Estimate: 3d
- Deliverables: release bundle, post-milestone roadmap, acceptance packet
- Steps:
  10.1 Freeze code and create release branch/tag.
  10.2 Run full CI: tests, static checks, benchmarks, verifications.
  10.3 Package binaries, fixtures, docs, and checksums; store in artifacts/releases/.
  10.4 Produce post-milestone maintenance plan: monitoring, triage, hotfix policy.
  10.5 Obtain stakeholder sign-offs and publish release with provenance.
- Immediate next action: draft release checklist and add to docs/release_checklist.md.

Change control / Traceability
- Update docs/requirements_rtm.md rows when changes occur.
- Link commits and PRs to PH3-REQ and HA IDs in branch names and PR descriptions.