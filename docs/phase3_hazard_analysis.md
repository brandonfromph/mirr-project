# Phase 3 Hazard Analysis (PHA) — Initial Draft

Purpose
- Identify hazards, failure modes, severity, likelihood, mitigations, and links to Phase 3 requirements (RTM).
- Follow NASA safety practices: small, traceable entries; explicit mitigations; verification artifacts.

Template for each hazard
- HA ID: HA-###
- Title: short title
- Description: concise failure mode
- Affected Requirements: list of PH3-REQ- IDs
- Severity (Cat): Cat I/II/III or High/Medium/Low
- Likelihood: Qualitative (Frequent, Probable, Occasional, Remote, Implausible)
- Root Causes: brief list
- Mitigations / Controls: design + test actions
- Verification Method: tests/analyses to show mitigation effective
- Owner: assigned engineer
- Status: Draft / In progress / Mitigated / Accepted

Initial hazard entries

HA-001
- Title: Non-deterministic temporal guard scheduling
- Description: Compiler or runtime produces differing guard ordering across otherwise identical runs.
- Affected Requirements: PH3-REQ-001
- Severity: High
- Likelihood: Probable (if unordered data structures or race-like scheduling exist)
- Root Causes: RNG usage without fixed seed; unordered collections; implicit scheduling nondeterminism
- Mitigations:
  - Enforce deterministic scheduling policy in temporal compiler
  - Seed all RNGs from traced inputs
  - Replace unordered maps in critical paths or use deterministic iteration
  - Add regression determinism harness in CI
- Verification Method: determinism test harness; 100 repeated runs with identical inputs/seeds
- Owner: TBD
- Status: Draft

HA-002
- Title: Runtime heap allocations in hot path exceed budget
- Description: Dynamic allocation in steady-state causes latency spikes and potential OOM.
- Affected Requirements: PH3-REQ-002
- Severity: High
- Likelihood: Occasional
- Root Causes: lazy allocations, caches, temporary buffers allocated on hot path
- Mitigations:
  - Allocate fixed pools at init time
  - Introduce allocation guard macros and audit CI checks
  - Add resource-bound tests and allocation detectors
- Verification Method: allocation profiler during benchmarks; tests asserting zero heap allocations post-init
- Owner: TBD
- Status: Draft

HA-003
- Title: Insufficient assertion density leads to undetected invalid states
- Description: Critical function failures not caught early due to missing assertions.
- Affected Requirements: PH3-REQ-003
- Severity: Medium
- Likelihood: Occasional
- Root Causes: legacy code, insufficient reviews, missing defensive checks
- Mitigations:
  - Add at least two assertions per critical function where meaningful
  - CI check to flag functions missing assertions (or manual checklist)
  - Peer-review guardrails and PR validation
- Verification Method: static review + CI reports; spot-check sampled functions
- Owner: TBD
- Status: Draft

Next steps / Work plan
1. Expand hazard list by reviewing docs/phase3_plan.md and code hotspots.
2. Assign owners and map each HA to RTM rows.
3. For each HA, produce concrete verification tests and add to test plan.
4. Review HA in safety meeting and update statuses.
5. Lock HA baseline and include in release sign-off.

References
- docs/phase3_plan.md
- docs/requirements_rtm.md