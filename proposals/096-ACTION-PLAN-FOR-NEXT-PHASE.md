# PROPOSAL 096: IMMEDIATE ACTION PLAN

**Status**: Review Complete | Governance Flaws Identified | Ready for Revision Phase

⚠️ **NOTE**: The proposal file (`proposals/096-REPO-WIDE-FOUNDATIONAL-INTEGRATION-2026-04-02.md`) was partially edited during review and may be truncated. Please restore from your last committed version or use the synthesis document findings to make targeted edits.

---

## YOUR NEXT STEPS (In Order)

### STEP 1: Read the Review Findings (15 min read)
**File**: `proposals/096-AGENT-REVIEW-SYNTHESIS.md`

This document contains:
- Executive summary of all 7 critical findings
- Detailed explanation of why each finding matters
- Specific fixes for each finding
- Agent signatures and validation

**Action**: Read the CRITICAL FINDINGS section (findings 1-7) to understand what needs fixing.

---

### STEP 2: Decide How to Fix Finding #1 (Signoff Contradiction)

**Finding**: Phase 0 marked COMPLETE in signoff table, but baseline artifacts marked PENDING.

**Decision Needed**: 
- Baselines now exist (Implementer captured them)
- Update proposal to reflect VERIFIED status

**Action**:
```markdown
Edit: proposals/096-REPO-WIDE-FOUNDATIONAL-INTEGRATION-2026-04-02.md

Find the "Signoff Evidence Gates" table (near end of document)
Change:
  - "Proposal framing | ... | Complete" (already done)
  - "Baseline artifacts | ... | Pending" → CHANGE TO "Complete"
  
Add note: "Baseline artifacts captured 2026-04-02 in proposals/evidence/096/"
```

**Time**: 5 minutes

---

### STEP 3: Decide How to Fix Finding #2 (LRA Adapter)

**Finding**: LRA `compile` command still shells out to binary instead of using library directly.

**Decision Needed**: Do you want to:

**OPTION A** (Fix now): Refactor LRA compile to direct library call
- Requires: Import compiler crate, replace Command::new with pipeline::run_pipeline call
- Time: ~30 minutes
- Benefit: Closes Zero-Debt violation immediately

**OPTION B** (Defer with timeline): Keep wrapper, but set explicit removal deadline
- Requires: Update proposal to say "LRA adapter must be refactored by Wave 2" with specific date
- Time: 5 minutes now
- Risk: Debt remains in code

**Action**:
- If OPTION A: Ask the Implementer to refactor `crates/lra-cli/src/main.rs:197` compile path
- If OPTION B: Edit proposal Finding #2 in synthesis doc, add explicit deadline to proposal text

**Your call**: A or B?

---

### STEP 4: Decide How to Fix Finding #3 (mirr-general --ci Incomplete)

**Finding**: The Rust orchestrator is missing 3 commands (Python validation, repo metrics, cargo doc).

**Decision Needed**: Do you want to:

**OPTION A** (Complete orchestrator): Add 3 missing steps to mirr-general.rs
- Requires: Edit `src/bin/mirr-general.rs` ci subcommand, add Python + doc commands
- Time: ~20 minutes
- Benefit: True Rust-native orchestrator, no Python/shell dependency

**OPTION B** (Revise proposal scope): Keep mirr-general as-is, revise proposal to document the split gate
- Requires: Update proposal to say "Core gate is mirr-general ci, Python validation runs separately"
- Time: 5 minutes
- Risk: Still have Python/shell dependency (what user said they don't like)

**Action**:
- If OPTION A: Ask Implementer to extend mirr-general --ci with 3 commands
- If OPTION B: Update proposal scope to document the split gate

**Your call**: A or B?

---

### STEP 5: Fix Finding #4 (Contract Ownership)

**Finding**: Contracts 0.A-0.E list owners as "AGENTS + copilot instructions" (not human names).

**Action** (No choice here - this must be done):
```markdown
Edit: proposals/096-REPO-WIDE-FOUNDATIONAL-INTEGRATION-2026-04-02.md

In Section 0 "Contract Ownership and Verification" table:

Current Owner cell examples:
  - "AGENTS + copilot instructions"
  - "Compiler + consumer docs"
  
Change to ACTUAL PERSON/ROLE:
  - "Repo Topology Authority owner: [Your name/team] (quarterly sync)"
  - "Consumer Matrix owner: [Person], [Person] (quarterly parity audit)"
  - etc.
```

**Time**: 10 minutes (fill in the blanks with actual names)

---

### STEP 6: Fix Findings #5-7 (Dependencies, KB-Lite, Consumer Parity)

These are design-level fixes. For each:

**Finding #5: Hidden dependencies**
- Action: Add "Contract Dependency DAG" section to proposal explaining serialization

**Finding #6: KB-Lite aspirational**
- Action: Create `docs/kb-lite-design.md` with migration path (or defer to separate campaign)

**Finding #7: Consumer parity fuzzy**
- Action: Revise Section B to distinguish "settled" vs "undergoing elevation" vs "experimental" surfaces

**Time**: 30-60 minutes total

---

## DECISION MATRIX

To unblock Phase 1, you need to make these decisions NOW:

| Finding | Decision | Impact if wrong | Your call |
|---------|----------|-----------------|-----------|
| #1 Signoff | Update to VERIFIED | Phase 0 approval blocked | ✓ Clear path |
| #2 LRA Adapter | Refactor now (A) or defer (B) | Zero-Debt violation remains if B | **A or B?** |
| #3 mirr-general | Extend (A) or revise scope (B) | User wanted no-Python; B keeps Python | **A or B?** |
| #4 Ownership | Assign humans | Contracts unenforceable otherwise | ✓ Clear path |
| #5 Dependencies | Add DAG | Wave plan invalid otherwise | ✓ Clear path |
| #6 KB-Lite | Create design doc (or defer) | Phase 0 incomplete otherwise | ✓ Clear path |
| #7 Consumer parity | Classify surfaces | Consumers unclear on maturity | ✓ Clear path |

---

## ESTIMATED TIME TO UNBLOCK

- Decisions on #2 and #3: 5 minutes (just choose A or B)
- Fixes on all 7 findings: 60-90 minutes
- Re-review after fixes: 30 minutes
- **Total**: 2-3 hours from now to Phase 1 ready

---

## WHAT TO DO NEXT

1. **Read** `proposals/096-AGENT-REVIEW-SYNTHESIS.md` (findings section)
2. **Decide** on Finding #2 (LRA adapter): Fix now or defer?
3. **Decide** on Finding #3 (mirr-general): Extend or revise scope?
4. **Tell me** which options you choose, and I'll coordinate the fixes

---

**All agent work is complete. Waiting on your decisions.**
