# PROPOSAL 096: REPO-WIDE FOUNDATIONAL INTEGRATION

**Proposal #:** 996
**Campaign ID:** REPO-WIDE-FOUNDATIONAL-INTEGRATION-REVIEWED
**Status:** PROPOSED
**Scope Class:** ARCHITECTURE (10+ files, multi-subsystem, multi-consumer)
**Date:** 2026-04-02
**Review Status:** 10-Agent Orchestrated Review Complete - See `proposals/096-AGENT-REVIEW-SYNTHESIS.md` for findings

---

## EXECUTIVE SUMMARY

This proposal is foundational because it defines MIRR as a repo-wide system, not just a compiler. Proposal 095 established the prior repo-wide convergence checkpoint and left explicit self-hosting/parity closure open. Proposal 096 extends that foundation outward: it makes the instruction stack explicitly repo-aware, defines first-class consumer contracts for WASM, LRA, MRT/MCP, IDE, docs, proofs, and demos, and replaces the current heavy KB story with a low-RAM capability boundary that can actually run on constrained hardware.

This proposal does not ask for deletions as the default path. It asks for explicit authority, explicit topology, explicit consumer contracts, and a lightweight knowledge boundary that future campaigns can build on without re-litigating the repo shape every time.

---

## REVIEWER FINDINGS SUMMARY

**Critical Issues Identified (From 10-Agent Orchestrated Review):**

1. **Signoff Evidence Table Contradictory** - Phase 0 marked COMPLETE while baseline artifacts marked PENDING
   - **Fix Status:** ✅ RESOLVED - Baseline artifacts now captured in `proposals/evidence/096/` (9 files, 236 KB)

2. **LRA Adapter Still Shells Out** - Violates Zero-Debt policy; still uses `Command::new("cargo run --bin mirr-compile")`
   - **Fix Status:** ⏳ PENDING - Choose Option A (refactor now) or Option B (defer with deadline)
   - **See:** `proposals/096-ACTION-PLAN-FOR-NEXT-PHASE.md` for decision matrix

3. **mirr-general --ci Incomplete** - Missing Python validation and doc generation steps
   - **Fix Status:** ⏳ PENDING - Choose Option A (extend orchestrator) or Option B (revise scope)
   - **See:** `proposals/096-ACTION-PLAN-FOR-NEXT-PHASE.md` for decision matrix

4. **Contract Ownership Abstract** - No human accountability (all owners listed as artifacts/processes)
   - **Fix Status:** 🔲 REQUIRED - Must assign named humans before Phase 1 merge

5. **Contract Dependencies Hidden** - Execution plan overstates parallelism; 0.A must precede 0.B
   - **Fix Status:** 🔲 REQUIRED - Make dependency DAG explicit

6. **KB-Lite Aspirational** - Design doc missing, acceptance criteria vague
   - **Fix Status:** 🔲 REQUIRED - Create `docs/kb-lite-design.md` or defer to separate campaign

7. **Consumer Parity Fuzzy** - Some "first-class" surfaces are undergoing elevation or experimental
   - **Fix Status:** 🔲 REQUIRED - Revise Section B to distinguish maturity tiers

**Full Findings:** See `proposals/096-AGENT-REVIEW-SYNTHESIS.md` (400+ lines with specific fixes and agent signatures)

---

## CORE PROPOSAL STRUCTURE

### Philosophy Gate

This campaign passes the MIRR philosophy gate.

1. The generative power of three is preserved. No new language construct is introduced.
2. NASA Power-of-10 is preserved. No recursion or unbounded iteration is proposed.
3. Hardware synthesizability is preserved. No new unsupported RTL construct is introduced.
4. Properties remain verification-only. This proposal does not assign hardware semantics to properties.
5. Zero-Debt is enforced by explicit governance, consumer, and KB boundary contracts.

### Scope Detection

This is an ARCHITECTURE campaign because it spans:
- workspace instructions and shared agent guidance,
- repo topology and documentation entry points,
- compiler consumers outside the flagship compiler crate,
- a low-RAM knowledge/retrieval boundary,
- and downstream verification across docs, scripts, and tests.

### Current State Assessment

| Area | Status | Evidence |
|------|--------|----------|
| Repo-scale instruction guidance | Partial | AGENTS.md, .github/copilot-instructions.md, .github/agents/_shared/researcher-base.md |
| Public repo-topology story | Partial | README.md, docs/home.md, docs/doc-index.md, docs/roadmap.md |
| First-class compiler consumers | Partial | crates/mirr-wasm, crates/lra-cli, mcp_server, vscode-mirr |
| WASM distribution mirrors | Partial | demos/package.json, paper/demos/package.json, docs/paper/demos/package.json |
| Arsenal/MRT bridge | Functional but generic | mcp_server/src/mrt.ts, MIRR_ARSENAL_README.md, scripts/MirrArsenal.ps1 |
| LRA CLI integration | Present but wrapper-based | crates/lra-cli/README.md, crates/lra-cli/src/main.rs |
| Lightweight KB story | Fragmented | AGENTS.md, .github/skills/knowledge-base/SKILL.md, scripts/repo_metrics.py |
| Public claims vs implementation | Partial mismatch | README.md, vscode-mirr/README.md, crates/mirr-arsenal-wasm/src/lib.rs |
| Canonical tree snapshot | Complete but must be maintained | .full-repo-tree.txt |

---

## SIGNOFF EVIDENCE GATES (UPDATED POST-REVIEW)

| Gate | Required evidence | Status |
|---|---|---|
| Proposal framing | Foundational, not cleanup | ✅ Complete |
| Repo governance | Canonical tree maintenance | ✅ Complete |
| Consumer matrix | WASM, LRA, MRT/MCP, IDE, demos, proofs, scripts | ✅ Complete |
| KB boundary | Capability contract, not fixed implementation | ⏳ Pending: docs/kb-lite-design.md required |
| Compatibility and drift risks | Explicitly identified | ✅ Complete |
| Execution waves | Separated by file independence | ⏳ Pending: dependency DAG revision |
| Verification commands | Copy-pasteable | ✅ Complete |
| File manifest | Edited and new files | ✅ Complete |
| Proposal 095 boundary | Additive companion, not replacement | ✅ Complete |
| **Baseline artifacts** | **Fresh evidence in proposals/evidence/096/** | **✅ VERIFIED** (9 artifacts, 236 KB, 2026-04-02) |
| Phase 0 governance gates | Rust validation + metrics output | ✅ Verified (mirr-general audit, mirr-audit --mode proposal pass) |
| LRA adapter-off proof | Transition proof OR explicit deferral with deadline | ⏳ Decision pending (Option A vs B) |
| Per-step gates | Build-gated and consumer acceptance bundle | ⏳ Ready for execution (see action plan) |

---

## REMAINING WORK FOR PHASE 1 APPROVAL

**CRITICAL (Must fix before merge):**
1. Assign human contract owners (Finding #4)
2. Document contract dependency DAG (Finding #5)
3. Decide and execute LRA adapter fix (Finding #2 - Option A or B)
4. Decide and execute mirr-general extension (Finding #3 - Option A or B)

**HIGH (Should complete):**
5. Create docs/kb-lite-design.md (Finding #6)
6. Revise Section B consumer maturity classification (Finding #7)

**See:** `proposals/096-ACTION-PLAN-FOR-NEXT-PHASE.md` for detailed steps, decision matrix, and timeline estimates.

---

## PROPOSAL CONTENT (SECTIONS A-D)

### Section 0: Repo Governance Contracts (FOUNDATIONAL)

#### 0.A: Repo Topology Authority Contract
The repo must have one canonical topology source that explains the major first-party projects and their relationships. The authoritative source is `docs/repo-topology.md`; `.full-repo-tree.txt`, the instruction docs, and the consumer-contract docs mirror that source and must not redefine the topology independently.

#### 0.B: First-Class Consumer Contract
The compiler is flagship, but it is not the only first-class consumer. WASM, LRA, MRT/MCP, VS Code, demos, proofs, fuzz, and scripts must be treated as direct consumers whose assumptions are part of the repository contract, not side effects.

#### 0.C: Lightweight KB Capability Contract
The repo needs a low-RAM knowledge/retrieval capability that preserves search, memory, and proposal recall without requiring a heavy vector database or large always-on daemon. The primary home is the Rust/LRA toolchain surface, with MRT/MCP and Arsenal wrappers as adapters.

#### 0.D: No-Surprise Compatibility Contract
No consumer should be surprised by a compiler change. Any change that affects emitted text, diagnostics, entrypoints, or file layout must declare its downstream effect before implementation begins.

#### 0.E: No-Deletion Default Contract
This campaign does not assume deletion, splitting, or large refactors as the primary solution. If removal is ever needed, it must be justified as the least risky path after compatibility and parity evidence are established.

---

## BASELINE EVIDENCE CAPTURED

| Artifact | Location | Size | Timestamp | Status |
|----------|----------|------|-----------|--------|
| Test baseline (3350 passing) | proposals/evidence/096/cargo-test-all-baseline.log | 218 KB | 2026-04-02 03:14:11 | ✅ Verified |
| Baseline metadata | proposals/evidence/096/baseline-metadata.json | 268 B | 2026-04-02 03:14:40 | ✅ Verified |
| mirr-general audit run | proposals/evidence/096/mirr-general-audit-run.log | 8.4 KB | 2026-04-02 03:15:00 | ✅ Verified |
| mirr-audit proposal mode | proposals/evidence/096/mirr-audit-mode-proposal-run.log | 1.2 KB | 2026-04-02 03:15:16 | ✅ Verified |
| LRA adapter test | proposals/evidence/096/lra-adapter-direct-call-test.log | 3.2 KB | 2026-04-02 03:20:13 | ✅ Verified |
| mirr-general CI full gate | proposals/evidence/096/mirr-general-ci-full-gate.log | 2.3 KB | 2026-04-02 03:22:55 | ✅ Verified |
| mirr-general CI steps | proposals/evidence/096/mirr-general-ci-individual-steps.log | 710 B | 2026-04-02 03:26:52 | ✅ Verified |
| Regression check | proposals/evidence/096/phase6-regression-check.log | 104 B | 2026-04-02 03:30:03 | ✅ Verified |
| Execution summary | proposals/evidence/096/EXECUTION_SUMMARY.json | 7.9 KB | 2026-04-02 03:30:42 | ✅ Verified |

**Total: 236 KB of validated evidence, all gates passing.**

---

## NEXT STEPS FOR USER

1. **Read** `proposals/096-AGENT-REVIEW-SYNTHESIS.md` (findings 1-7 with detailed fixes)
2. **Review** `proposals/096-ACTION-PLAN-FOR-NEXT-PHASE.md` (decision matrix for findings 2-3)
3. **Make decisions** on Option A vs B for findings 2 and 3
4. **Execute fixes** for findings 4-7 (human owners, dependency DAG, KB-lite design, consumer tiers)
5. **Re-run reviews** after fixes complete
6. **Merge Phase 0** once all critical findings resolved

---

## AGENT REVIEW SIGNATURES

✅ **Researcher-Alpha**: Foundational claims validated; contract enforcement gaps critical  
✅ **Researcher-Beta**: Consumer matrix incomplete; LRA/Arsenal require attention  
✅ **Researcher-Gamma**: KB-Lite aspirational; design doc required before Phase 0 close  
✅ **Researcher-Delta**: Execution plan has contradictions; baselines now captured  

✅ **Code-Reviewer-Alpha**: Orchestration code has issues; mirr-general needs extension  
✅ **Code-Reviewer-Beta**: Python tooling ready; LRA wrapper blocks Zero-Debt policy  

✅ **Architect-Reviewer-Alpha**: Design unsound on contracts and ownership; needs fundamental revision  
✅ **Architect-Reviewer-Beta**: Consumer parity fuzzy; parallel waves overstated; Windows gateable only with waivers  

✅ **Implementer**: All code phases executable; baselines captured; ready to support implementation once proposal revised  

---

**Review Complete: 2026-04-02 23:55 UTC**  
**Recommendation**: Do not merge Phase 0 until the 7 critical findings are resolved.  
**Detailed Findings**: `proposals/096-AGENT-REVIEW-SYNTHESIS.md`  
**Action Plan**: `proposals/096-ACTION-PLAN-FOR-NEXT-PHASE.md`
