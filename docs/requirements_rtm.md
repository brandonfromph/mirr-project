# Requirements Traceability Matrix (RTM) — Phase 3 (Populated)

Purpose
- Trace Phase 3 requirements to design modules, tests, hazards, and acceptance criteria.
- Each requirement is atomic and measurable per NASA Power-of-10 guidance.

RTM (table)

| Req ID | Short Description | Category | Source / Rationale | Priority | Owner | Design Module(s) | Hazards (HA ID) | Acceptance Criteria | Verification Method | Test IDs / Fixtures | Verification Status | Notes |
|--------|-------------------|----------|--------------------:|:--------:|:-----:|------------------:|----------------:|--------------------|--------------------:|---------------------:|---------------------:|-------|
| PH3-REQ-001 | Deterministic temporal guard scheduling | Func | Phase3 plan Task 3 | High | elvie <elvie@example.com> (auto-assigned) | src/temporal/compiler.rs, src/temporal/emit.rs | HA-001 | Given same inputs and seed, 100 repeated runs produce identical guard ordering and outputs | Determinism harness (100 repeats) | tests/temporal_guard_determinism.rs; artifacts/research/tmp_inputs/* | Not verified | Seed policy required |
| PH3-REQ-002 | No dynamic allocation in runtime hot path post-init | NF (Safety) | Phase3 plan Task 7 | High | elvie <elvie@example.com> (auto-assigned) | src/mirr_runtime.rs, src/mirr_executor.rs | HA-002 | Allocation profiler reports zero heap allocations during steady-state hot-path tests | Allocation profiler + unit tests | tests/resource_bounds.rs; artifacts/research/* | Not verified | Preallocate pools at init |
| PH3-REQ-003 | Assertion density: >=2 assertions per critical function | NF | Phase3 plan Task 6 | Medium | elvie <elvie@example.com> (auto-assigned) | All critical modules | HA-003 | Static audit shows each critical function has >=2 assertions where applicable | Static code scan + peer review | clippy/static-checks; manual audit | Not verified | Define "critical" functions list |
| PH3-REQ-004 | Module-level fixed loop bounds documented | NF | Phase3 plan Task 2 | High | elvie <elvie@example.com> (auto-assigned) | All modules with loops (list) | HA-004 | Every loop has an explicit, documented upper bound and rationale | Design review + code inspection | docs/architecture/*; tests/loop_bounds.rs | Not verified | Document in module interfaces |
| PH3-REQ-005 | Self-hosting parity with compiler_mirr reference | Func | Phase3 plan Task 4 | High | elvie <elvie@example.com> (auto-assigned) | compiler_mirr porting targets | HA-005 | Parity test suite passes for reference inputs (no unexplained diffs) | Parity test runs in CI | tests/self_hosting_parity.rs; fixtures/golden.json | Not verified | Reproducible build script required |
| PH3-REQ-006 | Determinism and throughput baseline established | NF | Phase3 plan Task 5 | Medium | elvie <elvie@example.com> (auto-assigned) | Benchmark harness, scripts/research | HA-006 | Baseline CSVs recorded; variability <= defined thresholds | Benchmark runs + analysis report | artifacts/research/determinism_runs.csv; throughput_baseline.csv | Not verified | Define thresholds in RTM notes |
| PH3-REQ-007 | No post-init dynamic allocation policy enforced | NF | Phase3 plan Task 7 | High | elvie <elvie@example.com> (auto-assigned) | src/* runtime modules | HA-002 | CI/bench shows zero allocations beyond init in steady-state tests | Allocation audit + CI gating | tests/resource_bounds.rs | Not verified | Enforcement via CI checks |
| PH3-REQ-008 | Golden fixtures managed with safe update runbook | NF | Phase3 plan Task 8 | Medium | elvie <elvie@example.com> (auto-assigned) | tests/fixtures/, docs/runbooks | HA-008 | Golden updates only via guarded script and recorded approvals | Runbook + guarded golden-update script | docs/runbooks/golden_fixture_update.md; tests/fixtures/* | Not verified | Add approval workflow |
| PH3-REQ-009 | Developer & operator runbooks available and verified | NF | Phase3 plan Task 9 | Low | elvie <elvie@example.com> (auto-assigned) | docs/runbooks/, docs/* | HA-009 | Runbooks validated by at least one trained reviewer and checklist | Training session + reviewer sign-off | docs/runbooks/* | Not verified | Record reviewer name/date |
| PH3-REQ-010 | Release artifacts packaged with provenance and checksums | NF | Phase3 plan Task 10 | High | elvie <elvie@example.com> (auto-assigned) | artifacts/releases/ | HA-010 | Release bundle contains binaries, fixtures, docs, and signed checksums | Release process run and sign-off | artifacts/releases/* | Not verified | Link to release tag/branch |

How to update
- Add new PH3-REQ-### rows as needed.
- When implementing, update "Verification Status" to "In progress" or "Verified" with date and evidence (CI run id, artifact path).
- Link PRs and commits to PH3-REQ IDs in branch names and descriptions (e.g., feat/PH3-REQ-002-fix-pools).

Next immediate actions for Task 1
1. Confirm owners above or replace with assigned personnel.
2. Schedule RTM review meeting and capture verifier names.
3. Begin filling "Verification Status" as tests/CI are added.