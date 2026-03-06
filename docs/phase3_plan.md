# Phase 3 Plan — Ten Tasks (NASA good-practices aligned)

Overview
- Goal: Complete Phase 3 by delivering a production-quality, analyzable, and verifiable runtime + compiler artifacts complying with NASA Power-of-10 and project roadmap constraints.
- Structure: 10 tasks. Each task lists objective, deliverables, step-by-step actions, exit criteria, owners/estimations, and NASA practice mappings.

---

Task 1 — Requirements Refinement & Safety Analysis
- Objective: Tighten Phase 3 requirements; produce traceable safety and requirements artefacts.
- Deliverables:
  - Updated requirements doc (traceable IDs)
  - Hazard and safety analysis (HA) with mitigations
  - Acceptance criteria matrix
- Steps:
  1. Gather Phase 2 outputs and stakeholder feedback; list all functional and non-functional requirements; assign IDs.
  2. Produce a Requirements Traceability Matrix mapping requirements → modules/tests.
  3. Conduct Preliminary Hazard Analysis: identify failure modes, severity, likelihood, mitigations.
  4. Define explicit acceptance criteria for each requirement (pass/fail metrics).
  5. Review with stakeholders; revise until sign-off.
- Exit criteria: Sign-offed requirements doc + HA with mitigations and RTM.
- NASA practices: Small scope per function, explicit acceptance, check return values.

Task 2 — Architecture & Interface Design (Deterministic, Bounded)
- Objective: Finalize architecture ensuring deterministic execution, fixed resource bounds, and small, verifiable modules.
- Deliverables:
  - Updated architecture diagrams
  - Module interface specifications (pre/postconditions)
  - Resource budgets per module
- Steps:
  1. Decompose system into modules sized to fit single-sheet-per-function guideline.
  2. Define APIs and strict data contracts for each module with parameter validation.
  3. Specify worst-case bounds: loop counts, stack/utilization, memory budgets.
  4. Produce call graphs and timing/resource budgets for critical paths.
  5. Peer review architecture; capture action items.
- Exit criteria: Architecture doc with resource budgets and reviewed interfaces.
- NASA practices: Fixed bounds, minimal control flow, scope-limited data.

Task 3 — Temporal Guard Compiler & Backend Integration
- Objective: Integrate and stabilize temporal guard compilation path with deterministic lowerings.
- Deliverables:
  - Updated temporal/compiler pipeline
  - Regression tests for temporal guards
  - Performance baseline comparisons
- Steps:
  1. Audit current temporal compiler stages; list gaps vs spec.
  2. Implement missing lowering passes with explicit checks and assertions.
  3. Add deterministic scheduling constraints and fixed iteration limits.
  4. Create regression test suite covering guard patterns and edge-cases.
  5. Run performance and determinism experiments; record baselines.
- Exit criteria: All temporal tests pass and performance within target tolerances.
- NASA practices: Assertions density, no dynamic allocation post-init, fixed-loop bounds.

Task 4 — Self-hosting & Porting Validation
- Objective: Ensure the compiler/runtime can self-host or produce verified outputs; complete porting steps.
- Deliverables:
  - Self-hosting build pipeline
  - Porting checklist completed (compiler_mirr -> rust)
  - Parity test results
- Steps:
  1. List remaining porting tasks (see compiler_mirr/PORTING_STEPS.md).
  2. Implement each porting step with regression tests; keep changes small and reviewable.
  3. Build self-hosting artifacts; run parity tests against reference outputs.
  4. Fix mismatches iteratively; keep detailed defect log.
  5. Produce a reproducible build script and CI entries.
- Exit criteria: Self-hosting build completes and parity tests meet acceptance.
- NASA practices: Small incremental changes, check return values, high assertion density.

Task 5 — Determinism & Throughput Benchmarking
- Objective: Demonstrate required determinism and throughput; quantify variability and CPU/memory budgets.
- Deliverables:
  - Determinism test harness
  - Throughput benchmark suite and results
  - Runbook for repeating experiments
- Steps:
  1. Define determinism metrics and measurement methodology.
  2. Extend experiments in artifacts/research (reuse existing scripts).
  3. Run sweep across inputs/flags; collect and analyze variability.
  4. Identify non-deterministic sources; mitigate (e.g., RNG seeds, unordered maps).
  5. Produce report with recommendations and thresholds.
- Exit criteria: Determinism within defined thresholds; baseline throughput documented.
- NASA practices: Repeatable experiments, instrumentation, fixed budgets.

Task 6 — Formal/Static Verification & Assertion Expansion
- Objective: Increase static verification and runtime assertions to meet safety density targets.
- Deliverables:
  - Enhanced assertion set across modules
  - Static checks integrated into CI (linters, clippy, fmt, custom checks)
  - Verification report summarizing coverage
- Steps:
  1. Identify critical functions and add at least two assertions per function where applicable.
  2. Introduce defensive parameter checks for all public APIs; ensure callers check returns.
  3. Add static analysis runs to CI and address all warnings.
  4. Run tools to detect dynamic allocations and ensure none occur post-init.
  5. Produce verification report listing assertions and remaining gaps.
- Exit criteria: Assertions added and CI passes static checks without warnings.
- NASA practices: Assertion density, parameter validation, compile-with-warnings.

Task 7 — Resource-Bounded Implementation & Memory Safety
- Objective: Enforce no dynamic allocation after initialization and formalize resource accounting.
- Deliverables:
  - Memory/resource allocation policy
  - Audit results and fixes
  - Resource monitors or runtime checks
- Steps:
  1. Inventory current allocation sites and identify any dynamic allocations post-init.
  2. Refactor code to allocate fixed pools at init time where needed.
  3. Implement runtime checks to assert resource usage stays within budgets.
  4. Add tests that exercise boundary conditions to ensure bounds hold.
  5. Document policy and update developer onboarding.
- Exit criteria: Zero dynamic allocations post-init; tests proving resource bounds.
- NASA practices: No post-init dynamic allocation, declare data at smallest scope.

Task 8 — Testing Matrix Expansion & Golden Fixture Management
- Objective: Expand test coverage, update golden fixtures, and ensure automated parity triage runbooks.
- Deliverables:
  - Expanded test matrix covering edge cases and failure modes
  - Updated golden fixtures and fixture update runbook
  - Automated test orchestration in CI
- Steps:
  1. Review existing tests (tests/*) and identify missing coverage areas (temporal, resources, failure modes).
  2. Add deterministic, small-unit tests for critical logic and cross-check with golden fixtures.
  3. Build automated fixture-update runbook and scripts; include safety checks to avoid accidental golden updates.
  4. Create stress tests that exercise fixed upper bounds and collect failure-mode data.
  5. Integrate nightly runs and reporting.
- Exit criteria: Coverage metrics met; golden fixtures stable and updateable via runbook.
- NASA practices: Test matrices, golden fixtures, reproducible updates.

Task 9 — Documentation, Runbooks & Training
- Objective: Produce complete docs for maintenance, operations, and safe release of Phase 3 artifacts.
- Deliverables:
  - Developer guide (coding rules, Power-of-10 checklist)
  - Operator runbooks (build, test, experiment replication)
  - Release checklist and post-mortem template
- Steps:
  1. Draft developer guide emphasizing Power-of-10 compliance and module-size limits.
  2. Produce runbooks: build_selfhost, run experiments, update fixtures, triage parity failures.
  3. Create quick-start docs for reproducing benchmarks and determinism tests.
  4. Hold a walkthrough/training session with team; capture feedback.
  5. Finalize docs and version them in repo.
- Exit criteria: Docs reviewed and accessible; at least one trained reviewer verifies runbook steps.
- NASA practices: Documented processes, checklists, reproducible runbooks.

Task 10 — Release Preparation & Post-Milestone Plan
- Objective: Prepare Phase 3 release artifacts and create a post-milestone maintenance plan.
- Deliverables:
  - Release bundle (artifacts, docs, checksums)
  - Post-milestone roadmap and maintenance schedule
  - Acceptance sign-off packet
- Steps:
  1. Freeze code; create release branch and tag.
  2. Run full CI: tests, static checks, benchmarks, verifications.
  3. Package artifacts (binaries, specimens, docs) and sign/checksum them.
  4. Produce a post-milestone plan: monitoring, bug triage policy, and hotfix process.
  5. Get stakeholder sign-offs and publish release with provenance.
- Exit criteria: Release artifacts published and sign-off recorded.
- NASA practices: Reproducible release, provenance, sign-off.

---

Appendix — Estimates and Owners (example)
- For each task, assign an owner and estimate in work-days during planning meeting. Example:
  - Task 1 — 5 days — Owner: Product lead
  - Task 2 — 6 days — Owner: Architect
  - Task 3 — 8 days — Owner: Compiler lead
  - Task 4 — 6 days — Owner: Porting lead
  - Task 5 — 5 days — Owner: Research lead
  - Task 6 — 7 days — Owner: Verification engineer
  - Task 7 — 5 days — Owner: Runtime lead
  - Task 8 — 6 days — Owner: Test lead
  - Task 9 — 4 days — Owner: Tech writer
  - Task 10 — 3 days — Owner: Release manager

Change control and traceability
- Record all changes against requirement IDs in RTM.
- Create changelog entries for any behavioral change and link to tests.

Maintenance of NASA Power-of-10 compliance
- Checklist to enforce during code reviews:
  - No recursion
  - Fixed loop bounds explicitly documented
  - No dynamic allocation after init
  - Functions small (single-sheet)
  - Assertions >= 2 per function where applicable
  - Parameter validation and caller checks
  - Preprocessor limited to includes/macros
  - Single dereference pointer use only