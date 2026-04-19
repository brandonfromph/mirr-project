# MRT Mega Campaign Roadmap

> Classification: Internal architecture planning
> Author: Project architecture synthesis
> Created: 2026-04-07
> Status: Strategic planning - pre-execution
> Placement: Repository root, aligned with private campaign planning artifacts

---

## SIGN/VETO Gate

- SIGN: Accept MRT as the cargo-equivalent control plane for MIRR and execute this campaign in order.
- VETO: Reject if policy, schema, or decision ownership remains split after the authority ramp.

---

## Overview

This document defines the architecture-scope campaign that moves MRT from a hybrid
TypeScript plus Rust bridge into a Rust-owned control plane with deterministic
contract behavior, fail-closed security, and replayable evidence.

The public roadmap states what to ship. This document states why sequence,
ownership, and gate contracts matter for a safety-critical control surface.

MRT is treated here as cargo for MIRR: one lifecycle interface, one policy
authority, one envelope model, one evidence chain.

### Planning Loop

1. Architecture Decision Agent sets immutable ownership and boundary decisions.
2. Planner Agent translates those decisions into bounded waves with hard gates.
3. Each wave runs fail-first tests, minimal deltas, verification, and evidence capture.
4. Advancement is fail-closed: unsigned ADRs, missing evidence, or drift block progress.

---

## The Unified Thesis

MRT is not a second compiler. MRT is MIRR's operational control plane.

It orchestrates existing MIRR tooling through one canonical lifecycle contract:
check, test, emit, gate, and evidence. Rust is the long-term authority for
contract, policy, and decision logic. TypeScript can exist only as a temporary
transport adapter and cannot own behavior.

### Foundation Table

| Foundation | Manifestation in MRT |
|---|---|
| Fail-safe defaults (Saltzer-Schroeder) | Unknown method, auth failure, schema mismatch, and ambiguity reject deterministically. |
| Single source of truth | Contract, policy matrix, envelope schema, and catalog are defined in one canonical owner. |
| Deterministic systems design | Same input and pinned environment produce identical envelope and evidence outputs. |
| Bounded execution | Explicit limits for retries, concurrency, timeout, payload size, and output bytes. |
| Anti-corruption boundary | Adapter layers can project transport but cannot redefine policy or decisions. |
| Strangler migration | Ownership moves incrementally from hybrid to Rust without operator breakage. |
| Evidence-based governance | Gate decisions are made from replayable artifacts, not status narrative. |
| Zero-debt closure | Temporary compatibility logic is removed after cutover proof, not retained. |
| Least privilege | Role and capability matrix are explicit, default-deny, and route-complete. |
| Contract-first operation | check/test/emit/gate/evidence verbs are stable and versioned before optimization. |
| Replayability | Every accepted run can be reconstructed from captured receipt and digests. |
| Safety-critical enforcement | CI gates block merges on parity, policy, envelope, and evidence regressions. |

---

## Campaign Dependency Graph

\`\`\`
MEGA-0 (BASELINE-LOCK) -> MEGA-1 (DECISION-LOCK) -> MEGA-2 (CONTRACT-CORE)
                                                \-> MEGA-3 (POLICY-CORE)
MEGA-2 -----------------------------------------> MEGA-4 (EVIDENCE-CORE)

MEGA-3 + MEGA-4 -> MEGA-5 (RUST-AUTHORITY-RAMP) -> MEGA-6 (PARITY-CUTOVER)
                                            -> MEGA-7 (RUST-DEFAULT)
                                            -> MEGA-8 (BRIDGE-RETIREMENT)
\`\`\`

Execution order:
1. MEGA-0 -> MEGA-1 -> MEGA-2
2. MEGA-3 and MEGA-4 in parallel
3. MEGA-5 -> MEGA-6 -> MEGA-7 -> MEGA-8

Critical path:
1. MEGA-0 -> MEGA-1 -> MEGA-2 -> MEGA-3 -> MEGA-5 -> MEGA-6 -> MEGA-7 -> MEGA-8

Parallel track:
1. MEGA-4 runs in parallel with MEGA-3 and must complete before MEGA-5 exit.

---

## Required ADR Set

All ADRs below are mandatory before MEGA-3 merge:

| ADR ID | Decision | Required outcome |
|---|---|---|
| ADR-106-001 | Final control-plane owner | Rust-owned |
| ADR-106-002 | Canonical crate location | crates/mrt-mcp |
| ADR-106-003 | Contract source of truth | Rust canonical module |
| ADR-106-004 | Transport strategy | Dual parity (stdio + stream) |
| ADR-106-005 | Adapter lifecycle | Temporary TS bridge with retirement gate |
| ADR-106-006 | Auth and key model | Hash-only, fail-closed, no committed defaults |
| ADR-106-007 | Error envelope contract | Canonical envelope across transports |
| ADR-106-008 | Quota and concurrency model | Layered controls (global + per-token) |
| ADR-106-009 | CI gate posture | Blocking MRT hard gate |
| ADR-106-010 | Cutover and retirement criteria | Parity-evidence cutover |

ADR hard rule:
1. No ADR remains draft past MEGA-1 close.
2. Any merge into MEGA-3+ scope is blocked if ADR-106-001 through ADR-106-004 are unsigned.

---

## MEGA-0: BASELINE-LOCK (Phase M0) - Freeze Current Hybrid Behavior

### Vision

Before ownership changes, baseline current behavior and make it replayable.
Without a replayable baseline, later parity claims are non-falsifiable.

### Scope

- Freeze method catalog and route matrix.
- Capture stdio and stream baseline envelopes.
- Capture security baseline for auth defaults, path containment, and schema coverage.
- Create replay bundle in proposals/evidence/106.

### Key Deliverables

1. Baseline contract snapshot.
2. Baseline transport parity report.
3. Baseline security checklist closure.
4. Replay transcript from local and CI.

### Theoretical Foundations

- Scientific reproducibility: baseline must be replayable to support valid comparison.
- Regression methodology: baseline capture precedes change attribution.

### Dependencies

- None.

### Verification Criteria

- [ ] Baseline artifacts replay unchanged in local and CI.
- [ ] Contract and envelope snapshots are versioned and immutable.
- [ ] Security baseline is complete for all exposed routes.

---

## MEGA-1: DECISION-LOCK (Phase M1) - Lock Ownership and Governance

### Vision

Eliminate ambiguous ownership before implementation to prevent split-brain behavior.

### Scope

- Sign ADR-106-001 through ADR-106-010.
- Publish ownership matrix for contract, policy, transport, and evidence.
- Define no-merge conditions for unsigned architecture decisions.

### Key Deliverables

1. Signed ADR packet.
2. Ownership matrix and boundary declaration.
3. Merge policy updates for architecture gate enforcement.

### Theoretical Foundations

- Governance by explicit decision records.
- Safety-critical process control through hard gate contracts.

### Dependencies

- MEGA-0 baseline evidence.

### Verification Criteria

- [ ] All mandatory ADRs signed.
- [ ] No unresolved ownership overlap remains.
- [ ] CI policy blocks unsigned architecture-impacting merges.

---

## MEGA-2: CONTRACT-CORE (Phase M2) - Canonical Lifecycle Contract

### Vision

Define a single lifecycle contract for check, test, emit, gate, and evidence.

### Scope

- Canonicalize command verbs and schema contracts.
- Canonicalize success and error envelope shape.
- Generate transport projections from canonical contract.
- Eliminate transport-specific contract drift.

### Key Deliverables

1. Contract schema v1.
2. Canonical envelope specification.
3. Verb-to-route matrix with conformance tests.
4. Projection conformance report.

### Theoretical Foundations

- Contract-first architecture for stable interfaces.
- Single source of truth to remove duplicated behavior definitions.

### Dependencies

- MEGA-1 decision lock.

### Verification Criteria

- [ ] check/test/emit/gate/evidence verbs are canonical and versioned.
- [ ] Envelope schema is identical across all transport surfaces.
- [ ] Projection tests prove no transport-specific contract variants.

---

## MEGA-3: POLICY-CORE (Phase M3) - Canonical Policy and Envelope Authority

### Vision

Move auth, schema validation, role policy, and quota controls behind one choke point.

### Scope

- Enforce hash-only auth and fail-closed policy behavior.
- Centralize schema validation for all exposed route classes.
- Implement layered quota controls and deterministic rejection behavior.
- Bind policy outcome to canonical envelope model.

### Key Deliverables

1. Policy matrix (auth, role, schema, quota).
2. Negative-path security test suite.
3. Deterministic policy rejection envelope tests.
4. Route-complete schema coverage report.

### Theoretical Foundations

- Least privilege and default deny.
- Deterministic fault handling in safety-critical control systems.

### Dependencies

- MEGA-2 contract core.

### Verification Criteria

- [ ] No route bypasses canonical policy enforcement.
- [ ] Unknown method and auth failure behavior is deterministic.
- [ ] Quota and timeout enforcement are bounded and observable.

---

## MEGA-4: EVIDENCE-CORE (Phase M4) - Mandatory Replayable Evidence

### Vision

Every command must produce audit-grade evidence that supports deterministic replay.

### Scope

- Define evidence receipt model for all lifecycle commands.
- Capture input hash, command graph hash, policy version, envelope, and artifact digests.
- Build replay validator and completeness checker.
- Block success status if evidence is incomplete.

### Key Deliverables

1. Evidence specification.
2. Evidence writer implementation.
3. Replay verifier.
4. Completeness gate report.

### Theoretical Foundations

- Forensic traceability and reproducibility.
- Evidence-based operations for high-assurance systems.

### Dependencies

- MEGA-2 contract core.

### Verification Criteria

- [ ] Every lifecycle command emits complete evidence.
- [ ] Replay validator reconstructs outcomes without hidden state.
- [ ] Missing evidence hard-fails the gate.

---

## MEGA-5: RUST-AUTHORITY-RAMP (Phase M5) - Move Canonical Ownership to Rust

### Vision

Shift contract and policy authority from hybrid surfaces to Rust canonical owner.

### Scope

- Stand up canonical owner in crates/mrt-mcp.
- Route TS bridge through Rust decisions.
- Enforce adapter as projection-only boundary.
- Add drift detector for authority violations.

### Key Deliverables

1. Rust authority module for contract plus policy.
2. Adapter projection conformance suite.
3. Drift detector report.
4. Ownership trace report.

### Theoretical Foundations

- Anti-corruption boundary pattern.
- Strangler migration with authority preservation.

### Dependencies

- MEGA-3 and MEGA-4 complete.

### Verification Criteria

- [ ] Adapter no longer defines policy/schema/decision behavior.
- [ ] Rust authority serves canonical decisions for all required methods.
- [ ] Drift detector shows zero authority divergence.

---

## MEGA-6: PARITY-CUTOVER (Phase M6) - Dual-Run Parity and Cutover Readiness

### Vision

Prove operational parity under dual-run before default cutover.

### Scope

- Run Rust and bridge paths in shadow mode.
- Compare success and failure envelope parity.
- Compare policy outcomes and evidence completeness.
- Capture drift and block on non-zero deltas.

### Key Deliverables

1. Dual-run parity bundle.
2. Failure-path parity bundle.
3. Drift closure report.
4. Cutover readiness packet.

### Theoretical Foundations

- Differential testing and behavioral equivalence.
- Controlled migration through measurable parity constraints.

### Dependencies

- MEGA-5 complete.

### Verification Criteria

- [ ] Required route matrix shows zero parity delta.
- [ ] Failure-path envelopes match canonical spec.
- [ ] Evidence parity holds across dual-run paths.

---

## MEGA-7: RUST-DEFAULT (Phase M7) - Default Control Plane Cutover

### Vision

Make Rust default for all local and CI paths while preserving controlled fallback.

### Scope

- Switch default runtime path to Rust authority.
- Restrict TS bridge to explicit compatibility fallback.
- Enforce blocking CI gates on Rust-owned path.
- Publish rollback and recovery playbook.

### Key Deliverables

1. Default path switch report.
2. CI gate evidence for Rust default.
3. Fallback policy declaration.
4. Rollback playbook.

### Theoretical Foundations

- Progressive delivery with safe rollback.
- Deterministic operations under controlled failover.

### Dependencies

- MEGA-6 cutover readiness complete.

### Verification Criteria

- [ ] Rust path is default for local and CI.
- [ ] Fallback is explicit and non-authoritative.
- [ ] Two consecutive hard-gate runs succeed under Rust default.

---

## MEGA-8: BRIDGE-RETIREMENT (Phase M8) - Zero-Debt Closure

### Vision

Retire TS-owned behavior and close split-brain risk permanently.

### Scope

- Remove duplicate TS policy/contract logic.
- Keep optional launcher only if non-authoritative.
- Complete debt audit for wrapper, alias, dead code, and duplication bans.
- Update docs to reflect final ownership state.

### Key Deliverables

1. Retirement certificate.
2. D1-D7 debt audit closure table.
3. Post-cutover soak evidence.
4. Documentation consistency review.

### Theoretical Foundations

- Zero-debt invariant for safety-critical systems.
- Ownership closure to eliminate residual ambiguity.

### Dependencies

- MEGA-7 Rust default complete.

### Verification Criteria

- [ ] TS owns zero policy/schema/decision logic.
- [ ] No split-brain route remains.
- [ ] Debt audit finds no unresolved violations in campaign scope.

---

## Gate Contracts

| Gate | Blocks advancement when | Required evidence |
|---|---|---|
| G0 Baseline Repro Gate | Baseline behavior is not replayable | Baseline snapshot plus replay transcript |
| G1 ADR Gate | Mandatory ADR set unsigned or contradictory | Signed ADR packet and ownership matrix |
| G2 Contract Gate | Lifecycle verbs diverge by transport | Contract conformance matrix and parity suite |
| G3 Policy Gate | Any route bypasses auth/schema/quota policy | Negative-path suite and policy coverage report |
| G4 Evidence Gate | Any lifecycle path lacks complete receipt | Completeness report and replay verifier output |
| G5 Authority Gate | Adapter defines behavior | Authority trace report and conformance diff |
| G6 Cutover Gate | Success or failure parity differs | Dual-run parity bundle |
| G7 Retirement Gate | TS remains authoritative in any core path | Retirement certificate and debt audit closure |

TS bridge retirement criteria (hard):
1. Zero TS-owned policy logic.
2. Zero TS-owned schema authority.
3. Zero TS-owned decision branching.
4. Rust remains sole authority in default and failure modes.
5. Replay evidence remains valid with bridge disabled.

---

## Primary Six-Wave Migration Spine (Hybrid -> Rust-Owned MCP)

| Wave | Objective | Maps to MEGA phases | Exit condition |
|---|---|---|---|
| Wave 1 | Baseline plus decisions | MEGA-0 + MEGA-1 | Repro baseline and signed ADRs |
| Wave 2 | Canonical contract plus policy core | MEGA-2 + MEGA-3 | Contract and policy conformance complete |
| Wave 3 | Evidence plus authority ramp | MEGA-4 + MEGA-5 | Complete receipts and Rust canonical authority live |
| Wave 4 | Dual-run parity | MEGA-6 | Zero-delta parity on required route matrix |
| Wave 5 | Rust default cutover | MEGA-7 | Rust default green for two hard-gate runs |
| Wave 6 | Bridge retirement and closure | MEGA-8 | Zero-debt closure and retirement certificate |

---

## Definition of Done

### Proposal 106 truly complete

All conditions must be true:

| Domain | Complete condition | Evidence required |
|---|---|---|
| Ownership | Rust is sole control-plane authority | Signed ADR-106-001 plus ownership trace audit |
| Contract | One contract source for all lifecycle verbs | Contract source trace report |
| Security | Hash-only auth, fail-closed policy, route-complete schema gate | Security matrix closure |
| Runtime | Deterministic and bounded execution controls everywhere | Determinism and boundedness report |
| Parity | Stdio and stream parity for success and failure paths | Transport parity artifact |
| CI | Blocking MRT hard gate with local replay parity | CI logs plus replay transcript |
| Debt closure | No duplicate TS plus Rust policy logic | D1-D7 debt audit closure table |
| Documentation | Docs describe only behavior enforced by code | Documentation consistency review |

### Hardening still in progress

If any item below is true, Proposal 106 is not complete:

| Indicator | Why this means incomplete |
|---|---|
| TS and Rust both define policy rules | Split-brain authority remains |
| Unknown-method or auth failures differ by transport | Contract drift remains |
| CI can pass without MRT hard gate | Enforcement is incomplete |
| Any permissive bootstrap or committed default token remains | Security posture incomplete |
| Any exposed route bypasses schema validation | Input safety incomplete |
| Evidence or parity artifacts are not replayable | Validation incomplete |
| Documentation promises controls not enforced by code | Operational risk unresolved |

---

## Timeline Sketch

| Window | Campaign focus |
|---|---|
| Week 1 | MEGA-0 baseline freeze plus MEGA-1 decision lock |
| Week 2 | MEGA-2 contract core plus MEGA-3 policy core |
| Week 3 | MEGA-4 evidence core plus MEGA-5 Rust authority ramp |
| Week 4 | MEGA-6 parity cutover and MEGA-7 Rust default |
| Week 5 | MEGA-8 retirement and zero-debt closure |
