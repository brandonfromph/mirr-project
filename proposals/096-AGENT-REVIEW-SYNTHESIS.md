# PROPOSAL 096 AGENT REVIEW SYNTHESIS

**Date**: 2026-04-02  
**Orchestrator Mode**: Multi-Agent Review Swarm  
**Agents Deployed**: 4 Researchers, 2 Code Reviewers, 2 Architects, 1 Implementer  

---

## EXECUTIVE SUMMARY

Proposal 096 has **executable code and working baselines**, but **flawed governance design** that will cause enforcement failures if merged as-is. The design promises durable contracts but lacks human accountability, hides dependencies, and contains aspirational components treating placeholders as finished integration.

**Verdict**: 🟡 **OPERATIONAL CODE | UNSOUND DESIGN** — Fix governance before Phase 1 merge.

---

## CRITICAL FINDINGS (Must Fix Before Merge)

### 1. UNSOUND: Signoff Evidence Table is Contradictory

**Finding**: Phase 0 marked COMPLETE in signoff table, but "Baseline artifacts" gate marked PENDING.

**Why it matters**: 
- If baselines are pending, Phase 0 is incomplete
- If Phase 0 is complete, baselines must exist
- The proposal creates ambiguity: Can Wave 1 start, or not?

**Evidence**:
- Implementer captured baselines → they now exist in `proposals/evidence/096/`
- But proposal text still lists them as PENDING
- This contradiction will confuse reviewers and block merge

**Fix Required**:
```markdown
Change signoff table:
- Phase 0: COMPLETE → VERIFIED (baselines now in proposals/evidence/096/)
- Baseline artifacts: PENDING → COMPLETE (9 artifacts, 236 KB captured)

Add to Phase 0 output: "Baseline capture verified 2026-04-02"
```

---

### 2. CRITICAL: LRA Adapter Still Shells Out to Binary

**Finding**: 
- Section B promises: "Make LRA a direct library consumer of the compiler crate"
- Debt Audit (D1) identifies: "LRA compile path shells out to the compiler binary" 
- LRA Adapter Exit Criteria promise: "Allowed only during defined transition waves"
- **Actual code**: `crates/lra-cli/src/main.rs:197` still calls `Command::new("cargo").args(["run", "--bin", "mirr-compile", ...])`

**Why it matters**: 
- Zero-Debt policy is violated — wrapper remains in the codebase
- LRA is coupled to binary naming/versioning across upgrades
- If the wrapper is truly "temporary," the proposal must define when it's removed; currently it's indefinite

**Evidence**: 
- Code review confirms wrapper is still present
- No library import of compiler crate exists in LRA's Cargo.toml
- LRA tests pass, but they test the wrapper, not direct library consumption

**Fix Required** (CHOOSE ONE):

**Option A** (Recommended): Make refactoring a deliverable
```markdown
File Manifest Edit:
- Add to Section B under "Edited": 
  crates/lra-cli/src/main.rs | LRA integration boundary refactor (direct library call, not cargo shell-out)

Add explicit step in execution plan:
Step 2.B: Refactor lra-cli compile handler to consume compiler library directly
Acceptance gate: cargo test -p lra-cli passes; no execSync to "cargo run --bin" in main.rs

Removal timeline: Must be completed by end of this wave (2026-04-XX)
```

**Option B** (If not ready): Defer with deadline
```markdown
Mark as: "Deferred to Wave 2 with explicit removal deadline 2026-05-15"
Add blocker: "If Wave 2 closes before refactor, proposal fails"
```

---

### 3. CRITICAL: mirr-general --ci Is Incomplete

**Finding**: 
- Proposal claims final gate is `cargo run --bin mirr-general -- ci`
- Hardcoded steps in `src/bin/mirr-general.rs` include: fmt, clippy, build, test, consumer smoke tests
- **Missing steps from proposal's Final Integrated Gate**:
  - `python scripts/validate_proposals.py --strict`
  - `python scripts/repo_metrics.py --json`
  - `cargo doc --no-deps` (with RUSTDOCFLAGS="-D warnings")

**Why it matters**: 
- If reviewers approve based on "use the Rust orchestrator," but the orchestrator is incomplete, Wave 5 (Verification) will fail
- Teams will fall back to manual PowerShell scripts, undermining the "Rust-native" claim

**Evidence** (Code Review):
```rust
// Current src/bin/mirr-general.rs ci subcommand orchestrates:
1. cargo fmt --check
2. cargo clippy --all-targets -- -D warnings
3. cargo build --all-targets
4. cargo test --all
5. cargo test -p lra-cli
6. npm --prefix mcp_server test
7. cargo check -p mirr-wasm
8. IDE language check (node -e)
9. Demo mirror check (node -e)
10. Proof make targets (Make compilation)
11. Fuzz harness check (cargo +nightly)
12. LRA adapter test

// MISSING:
13. python scripts/validate_proposals.py --strict
14. python scripts/repo_metrics.py --json
15. cargo doc --no-deps
```

**Fix Required** (CHOOSE ONE):

**Option A** (Recommended): Complete the orchestrator
```rust
// Add to mirr-general.rs ci subcommand:
Command::new("python")
    .args(["scripts/validate_proposals.py", "--strict"])
    .output()?;

Command::new("python")
    .args(["scripts/repo_metrics.py", "--json"])
    .output()?;

// For cargo doc, handle cross-platform:
#[cfg(windows)]
std::env::set_var("RUSTDOCFLAGS", "-D warnings");
Command::new("cargo")
    .args(["doc", "--no-deps"])
    .output()?;
#[cfg(not(windows))]
// Unix: RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

**Option B**: Revise proposal
```markdown
Revise Final Integrated Gate section:
"The final gate is `cargo run --bin mirr-general -- ci`, which orchestrates core Rust compilation, 
testing, and consumer acceptance. Python validation (repo_metrics, validate_proposals) is gated 
separately via direct script execution pending orchestrator extension."

Keep Python validation, doc generation as separate prerequisites.
```

---

### 4. CRITICAL: Contract Ownership is Abstract, Not Human-Accountable

**Finding**: 
- Contracts 0.A-0.E list owners as: "AGENTS + copilot instructions", "Compiler + consumer docs", "KB-lite design + tooling layer", etc.
- These are artifacts/processes, not humans
- **No named person can enforce a contract** if it's violated

**Why it matters**: 
- When a contract is violated (e.g., topology doc drifts), there's no clear human to make the enforcement decision
- Disputes escalate without resolution path
- Drift will be rediscovered per campaign instead of prevented

**Evidence** (Researcher-Alpha):
```markdown
Contract Ownership table found no:
- Individual names
- Team assignments  
- Escalation paths
- Rollback procedures

Example problems:
- "Repo Topology Authority owner: AGENTS + copilot instructions" — Not a person
- "Verifier: Docs readers, proposal reviewers" — Distributed, unclear
- No "Last synced" timestamp to detect staleness
```

**Fix Required**:
```markdown
Expand Contract Ownership table (Section 0):

| Contract | Owner (Name/Role) | Sync Frequency | Last Synced | Escalation |
|----------|-------------------|-----------------|-------------|-----------|
| 0.A Repo Topology Authority | [Maintainer] | Quarterly | 2026-04-02 | If drift > 2 weeks, raise blocker |
| 0.B Consumer Matrix | [LRA+WASM leads] | Quarterly | 2026-04-02 | If surface drifts, revert + notify |
| 0.C KB-Lite Boundary | [KB owner] | Monthly | TBD (pending design) | If performance regresses, hold merge |
| 0.D No-Surprise Compatibility | [Release owner] | Per wave | Per wave | If emitted text changes undeclared, block merge |
| 0.E No-Deletion Default | [Proposal reviewer] | Per campaign | Per campaign | Deletion without justification → reject PR |

Add new section: "Governance Escalation Path"
- If contract violation detected: Raise issue with [Owner] 
- If Owner unresponsive (3 days): Escalate to [Lead]
- If unresolved by merge day: Hold merge on contract
```

---

### 5. HIGH: Contract Dependencies Are Hidden

**Finding**: 
- 0.B (Consumer Matrix) depends on 0.A (Repo Topology) being defined first
- 0.D (No-Surprise Compatibility) depends on 0.B being finalized
- Execution plan shows them as independent waves
- **Declares "Parallelizable? Yes" but has implicit serialization**

**Why it matters**: 
- Teams may implement Section B before Section A is approved
- Consumer contracts will reference undefined topology
- This creates circular dependencies and stale references

**Evidence** (Architect Review):
```
Contract dependency DAG:
0.A (Topology) → 0.B (Consumer Matrix) → 0.D (Compatibility)
       ↓                ↓
   Section A         Section B              Section D

Execution plan claims Section B "Depends on: Step 1" (Section A)
BUT also claims "Parallelizable? Yes, with exclusive file ownership"

This is contradictory. If B depends on A completion, it cannot be parallelized.
```

**Fix Required**:
```markdown
Add Section 0.F: "Contract Dependency DAG"

Dependency chain:
  Phase 0 (Evidence baseline) 
    ↓
  0.A (Repo Topology) — MUST complete before 0.B reads it
    ↓
  0.B (Consumer Matrix) — can read frozen draft of 0.A
    ↓
  0.D (Compatibility) — final revision after 0.B finalized
  
0.C (KB-Lite) — independent, can proceed in parallel
0.E (No-Deletion) — cross-cutting, applies to all

Revised Parallel Wave Plan:
Wave 1: Phase 0 (Evidence baseline) + 0.A (Repo Topology) [SERIAL]
  - Phase 0 must complete before 0.A starts
  - 0.A docs must reach "stable draft" before Section A PR merge
  
Wave 2: Section A (implementation of 0.A docs) [SERIAL after Wave 1]
  - Cannot start until Section A PR is merged and topology is frozen

Wave 2B: Section B (Consumer Matrix) [PARALLEL with Wave 2]
  - Reads frozen topology from merged 0.A docs
  - Can proceed in parallel with Section A implementation

Waves 3-4: Sections C & D [PARALLEL]
  - No dependencies; can proceed independently

Add gate: "Before any Section B PR is opened, Section A topology doc must be merged and [Owner] must approve."
```

---

### 6. CRITICAL: Evidence Gates Table Has Phase 0 Complete While KB-Lite is Aspirational

**Finding**: 
- Signoff table shows "KB boundary: COMPLETE"
- But KB-Lite design doc (`docs/kb-lite-design.md`) doesn't exist
- Acceptance criteria are vague ("no always-on daemon", "low-RAM by default")
- Python scripts are diagnostics, not KB retrieval system

**Why it matters**: 
- If Phase 0 signoff includes KB boundary definition, but the design doc is missing, the signoff is incomplete
- When Section C (KB Boundary) starts implementation, there's no authoritative design to build against

**Evidence** (Researcher-Gamma):
```
Proposal claims KB-Lite acceptance criteria:
1. Runtime posture: "No mandatory always-on heavy vector daemon" — TOO VAGUE
2. Memory posture: "stays under 512 MB" — NO MEASUREMENT RECORDED
3. Latency: "<= 5 seconds" — MEASURED (0.66s ✓) but not gated
4. Fallback: "repo-local scripts remain usable" — SCRIPTS RUN but NO TEST

Current state:
- repo_metrics.py: EXISTS, runs ok (0.66s), is diagnostic only
- validate_proposals.py: EXISTS, runs ok, is validation only
- Neither implements KB search, memory, proposal recall (KB SKILL.md promises 64 capabilities, scripts deliver 2%)
- docs/kb-lite-design.md: DOES NOT EXIST
```

**Fix Required**:
```markdown
Before Phase 0 signoff:

1. Create docs/kb-lite-design.md (REQUIRED):
   - Section 1: Current KB Stack Inventory (what exists now, who uses it)
   - Section 2: KB-Lite Capability Boundary (explicit: what moves, what stays, what changes)
   - Section 3: Migration Steps (phased, with owner and gate per phase)
   - Section 4: Consumer Impact Assessment (WASM, LRA, MCP, IDE, demos affected, how)
   - Section 5: Acceptance Test Suite (with measurable criteria and evidence path)
   - Section 6: Rollback Trigger (if KB-lite fails, what triggers rollback)

2. Revise Acceptance Criteria to be measurable:
   a) Runtime: Checklist "Heavy daemon processes that must NOT run: [@vpxa/kb-server, elasticsearch, ...]"
      - Test: lsof | grep -E "(elasticsearch|kb-server)" returns empty
   b) Memory: Record via /usr/bin/time -v and store in proposals/evidence/096/memory-baseline.txt
      - Test: grep RssMax proposals/evidence/096/memory-baseline.txt | awk '{print $2}' < 512000 (KB)
   c) Latency: Already measured (0.66s); keep, gate on it
   d) Fallback: Script availability test (already passing)

3. Add to signoff: "KB-Lite design doc approved by [KB Owner] before Phase 0 closes"
```

---

### 7. HIGH: Consumer Parity Definition is Fuzzy (Some "First-Class" are Aspirational)

**Finding**: 
- Proposal elevates 8 surfaces to "first-class consumer" status
- Actual state assessment reveals several are aspirational or incomplete:
  - **LRA**: Compile command shells out (wrapper, marked deprecated in code)
  - **Arsenal WASM**: 26-line stub with placeholder methods (`get_law()` returns hardcoded strings)
  - **IDE**: Described as "incomplete user-facing story" in Section B4
  - **Demos**: Classified as "generated/public artifact" mirrors (not independent consumers)

**Why it matters**: 
- If these are proclaimed first-class but remain loosely coupled, future campaigns will treat them as aspirational
- Consumers relying on these surfaces will be surprised by continued drift and incompleteness

**Evidence** (Researcher-Beta):
```
Component maturity assessment:

✅ SETTLED FIRST-CLASS:
  - WASM: Direct pipeline call, proper error wrapping, cross-platform
  - Proofs: Make targets, AST validation
  - Fuzz: Cargo-fuzz integration
  - Scripts: Repo metrics, proposal validation

⚠️ UNDERGOING ELEVATION:
  - LRA: Compile command still shells out; marked legacy::warn_deprecated()
  - MRT/MCP: Wrapper-only design (all calls via execSync to cargo binaries)
  - IDE: Language support present; LSP optional/incomplete
  
🔴 EXPERIMENTAL/PLACEHOLDER:
  - Arsenal WASM: Stub methods (get_law() hardcoded, validate_wave_hash() string equality only)
       No actual Arsenal integration; ir_version hardcoded to '0.3.0'
       Marked experimental in comments only, not in Cargo.toml
```

**Fix Required**:
```markdown
Revise Section B to distinguish maturity tiers:

### B1-B6: SETTLED FIRST-CLASS CONSUMERS
(WASM, Proofs, Fuzz, Scripts, Demos as mirrors)

State: Direct integration, no wrappers, API stable
Acceptance: Smoke tests pass, parity maintained per wave

### B7-B9: UNDERGOING ELEVATION
(LRA, MRT/MCP, IDE)

State: Partial wrappers in transition
Timeline: Each must reach settled state by end of Wave 2
Acceptance: Explicit tests for wrapper-free direct calls; wrapper code must be removed

### B10: EXPERIMENTAL SURFACES
(Arsenal WASM)

State: Placeholder (26 lines, hardcoded values, no real Arsenal integration)
Maturity: Pre-alpha — API unstable, methods subject to change
Acceptance: Marked as experimental in Cargo.toml [package] with version < 0.1.0
            Deprecation notice in README

Transition to first-class: Requires real compiler integration (not stub methods)
Timeline: TBD (deferred to separate Arsenal integration campaign)

Add to Consumer Acceptance Bundle:
- "Undergoing elevation" surfaces MUST have explicit tests for direct calls (no wrappers by Wave 2 close)
- "Experimental" surfaces MUST be marked as such in package metadata
```

---

### 8. HIGH: Windows+PowerShell Consumer Acceptance Bundle May Not Be Gateable

**Finding**: 
- Acceptance bundle includes commands that assume Unix/Bash
- Example: `Push-Location proofs\rspu; make; Pop-Location` requires Unix make toolchain
- No clear answer on Windows CI: "Can this gate pass without waivers?"

**Why it matters**: 
- When Windows CI runs Wave 5 verification, proof targets will fail
- Either gate is skipped with waiver (violating contract), or developers forced to WSL

**Evidence** (Code + Architect Review):
```
Fragile commands in acceptance bundle:
1. make targets: ✗ Windows (requires Unix shell, gcc, make)
2. node -e proof checks: ⚠️ Windows (quoting issues, path separators)
3. npm test: ✓ Windows (Node cross-platform)
4. cargo tests: ✓ Windows (Rust cross-platform)

Fuzz waiver policy exists ("if nightly unavailable, record waiver")
BUT proof make targets have no waiver option → hard failure on Windows CI

mirr-general.rs uses Command::new("make") with current_dir → MORE cross-platform-friendly than PowerShell
BUT still requires make installed
```

**Fix Required**:
```markdown
Option A (Recommended): Move acceptance bundle into mirr-general orchestrator
  - mirr-general --ci already handles cross-platform better than PowerShell scripts
  - Add waiver support for environment-specific gates (make, nightly, etc.)
  - Windows CI: Can skip proof/fuzz with recorded waiver

Option B: Document Windows limitations
  - Proof targets require WSL or Unix shell on Windows
  - Add to INSTALLING.md: "Full acceptance bundle on Windows requires Git Bash or WSL"
  - CI gate: Linux/macOS run full bundle; Windows runs Rust+Node subset with waiver file

Recommended: Option A + add to mirr-general --ci:
```rust
#[cfg(target_os = "windows")]
fn skip_unix_only_targets() -> SkipWaiver {
    // Record that proof targets skipped on Windows
    // Still gate on Rust + Node components
}
```
```

---

## WORKING CORRECTLY (No Fixes Needed)

✅ **Baseline Capture**: All artifacts created, no corruption
✅ **Governance Gate Commands**: `mirr-general audit` and `mirr-audit --mode proposal` work
✅ **Consumer Smoke Gates**: WASM, LRA, MCP, IDE, demos, fuzz individually gateable
✅ **Python Tooling**: `repo_metrics.py` and `validate_proposals.py` operational
✅ **Code Quality**: No unsafe code violations, warning-free builds

---

## ACTIONABLE NEXT STEPS

### BEFORE WAVE 1 CAN START (Blocker Gates)

1. **[ ] Fix signoff table** — Update Phase 0 status from COMPLETE to VERIFIED
2. **[ ] Resolve LRA adapter** — Refactor to direct library call OR defer with explicit deadline
3. **[ ] Complete mirr-general --ci** — Add missing Python/doc steps OR revise proposal scope
4. **[ ] Assign contract owners** — Name humans for each contract (0.A-0.E)
5. **[ ] Document contract DAG** — Make dependencies explicit; serialize sections as needed
6. **[ ] Draft KB-Lite design** — Create `docs/kb-lite-design.md` with migration path
7. **[ ] Clarify consumer maturity** — Distinguish settled vs undergoing vs experimental

### AFTER FIXES (Can Start Wave 1)

8. Re-run architect review on revised proposal
9. Merge Phase 0 with verified baselines
10. Begin Section A (Instruction Docs) with explicit contract owner sign-offs
11. Gate Section B start on Section A topology doc approval

---

## SIGNATURES

**Researcher-Alpha**: Foundational claims validated; contract enforcement gaps critical  
**Researcher-Beta**: Consumer matrix incomplete; LRA/Arsenal require attention  
**Researcher-Gamma**: KB-Lite aspirational; design doc required before Phase 0 close  
**Researcher-Delta**: Execution plan has contradictions; baselines now captured  

**Code-Reviewer-Alpha**: Orchestration code has issues; mirr-general needs extension  
**Code-Reviewer-Beta**: Python tooling ready; LRA wrapper blocks Zero-Debt policy  

**Architect-Reviewer-Alpha**: Design unsound on contracts and ownership; needs fundamental revision  
**Architect-Reviewer-Beta**: Consumer parity fuzzy; parallel waves overstated; Windows gateable only with waivers  

**Implementer**: All code phases executable; baselines captured; ready to support implementation once proposal is revised  

---

**Date Compiled**: 2026-04-02 23:55 UTC  
**Recommendation**: Do not merge Phase 0 until the 7 critical findings are resolved.
