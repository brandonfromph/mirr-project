# MIRR Project Documentation Index

> **Maintainer:** MIRR Core Team  
> **Last updated:** 2026-03-07
> **Rule:** Every document in `docs/` MUST be listed here with its status.  
> New PRs that add or modify docs must update this index.

---

## Document Status Legend

| Tag | Meaning |
|-----|---------|
| 🟢 **Frozen** | Locked for current milestone; changes require version bump + ADR |
| 🔵 **Active** | Current and maintained; edits follow normal PR process |
| 🟡 **Draft** | Work-in-progress; not yet authoritative |
| 🔴 **Deprecated** | Superseded or stale; retained for historical reference only |

---

## 1. Language & Compiler Specifications

| Document | Path | Status | Description |
|----------|------|--------|-------------|
| MIRR Language Spec | [`mirr_spec.md`](mirr_spec.md) | 🟢 Frozen | Minimal core language (signal, guard, reflex) |
| MIRR-CORE Subset Spec | [`self_hosting_core_spec.md`](self_hosting_core_spec.md) | 🟢 Frozen | Self-hostable language subset (v1) |
| IR Contract | [`self_hosting_ir_contract.md`](self_hosting_ir_contract.md) | 🟢 Frozen | AST + HIR + Netlist JSON contracts (v1.0) |
| Temporal Guard Compiler | [`phase2_temporal_guard_compiler.md`](phase2_temporal_guard_compiler.md) | 🟢 Frozen | Cement2-inspired temporal lowering design |
| Interpreter Runtime Spec | [`interpreter/runtime_spec.md`](interpreter/runtime_spec.md) | 🟡 Draft | Runtime semantics and architecture for MIRR-CORE interpreter |
| Tutorial: MIRR from Scratch | [`tutorial.md`](tutorial.md) | 🔵 Active | 10-lesson beginner guide — no prior hardware experience required |

## 2. Schemas (Machine-Readable)

| Schema | Path | Status | Validates |
|--------|------|--------|-----------|
| AST Schema | [`schemas/mirr_ast.schema.json`](schemas/mirr_ast.schema.json) | 🟢 Frozen | Parsed module JSON structure |
| Temporal Netlist Schema | [`schemas/mirr_temporal_netlist.schema.json`](schemas/mirr_temporal_netlist.schema.json) | 🟢 Frozen | Lowered guard netlist JSON |

## 3. Milestones & Roadmap

| Document | Path | Status | Description |
|----------|------|--------|-------------|
| Project Roadmap | [`roadmap.md`](roadmap.md) | 🔵 Active | Phase 0–10 R-SPU roadmap |
| Self-Hosting Milestone v1 | [`self_hosting_milestone.md`](self_hosting_milestone.md) | 🟢 Frozen | Stage-1 self-hosting achievement record |
| Post-Milestone Plan | [`post_milestone_plan.md`](post_milestone_plan.md) | 🟡 Draft | Next-steps breakdown (interpreter → native) |

## 3.1 Research Packaging (Submission Support)

| Document | Path | Status | Description |
|----------|------|--------|-------------|
| Claims-Evidence Matrix | [`research/claims_evidence_matrix.md`](research/claims_evidence_matrix.md) | 🟡 Draft | Maps manuscript claims to reproducible in-repo evidence |
| Evidence Appendix Template | [`research/evidence_appendix_template.md`](research/evidence_appendix_template.md) | 🟡 Draft | Paper appendix scaffold for reproducibility and artifacts |

## 4. Architecture Decision Records (ADRs)

| ADR | Path | Status | Decision |
|-----|------|--------|----------|
| ADR-001 | [`decisions/ADR-001-doc-governance.md`](decisions/ADR-001-doc-governance.md) | 🔵 Active | Documentation governance and PR policy |
| ADR-002 | [`decisions/ADR-002-interpreter-architecture.md`](decisions/ADR-002-interpreter-architecture.md) | 🟡 Proposed | MIRR-CORE interpreter architecture (tree-walking runtime) |

> New ADRs use the template at [`templates/adr_template.md`](templates/adr_template.md).

## 5. Testing & Benchmarks

| Document | Path | Status | Description |
|----------|------|--------|-------------|
| Fixture Matrix | [`testing/fixture_matrix.md`](testing/fixture_matrix.md) | 🟡 Draft | Test fixture taxonomy and coverage map |
| Benchmark Protocol | [`benchmarks/benchmark_protocol.md`](benchmarks/benchmark_protocol.md) | 🟡 Draft | Performance measurement methodology |
| Error Code Reference | [`error_codes.md`](error_codes.md) | 🔵 Active | Complete catalogue of all MIRR compiler error codes (E1xx–E4xx) |
| Migration Guide | [`migration-guide.md`](migration-guide.md) | 🔵 Active | Upgrade notes for API and JSON consumers (0.1.0 to 0.2.0) |

## 6. Runbooks (Operational)

| Runbook | Path | Status | Scope |
|---------|------|--------|-------|
| Parity Triage | [`runbooks/parity_triage.md`](runbooks/parity_triage.md) | 🟡 Draft | How to diagnose Rust-vs-MIRR pipeline differences |
| Golden Fixture Update | [`runbooks/golden_fixture_update.md`](runbooks/golden_fixture_update.md) | 🟡 Draft | Procedure for updating authoritative test fixtures |

## 7. Templates

| Template | Path | Purpose |
|----------|------|---------|
| ADR Template | [`templates/adr_template.md`](templates/adr_template.md) | Architecture Decision Record |
| Design Spec Template | [`templates/design_spec_template.md`](templates/design_spec_template.md) | Feature/component design document |
| Test Plan Template | [`templates/test_plan_template.md`](templates/test_plan_template.md) | Test strategy for a feature or phase |

## 8. Other

| Document | Path | Status | Description |
|----------|------|--------|-------------|
| Demo Viewer | [`demo_viewer.html`](demo_viewer.html) | 🔵 Active | Browser-based AST/netlist viewer |
| MIRR Logo Guide | [`branding/mirr_logo.md`](branding/mirr_logo.md) | 🟡 Draft | Canonical three-snake ouroboros logo and `.mirr` icon usage guidance |
| MCP Server | [`mcp_server/README.md`](../mcp_server/README.md) | 🔵 Active | stdio‑direct protocol server for agents (TypeScript project) |
| Logic Simplification | [`logic_simplification.md`](logic_simplification.md) | 🟢 Frozen | Logic simplification design notes |
| MCP API | [`mcp_api.md`](mcp_api.md) | 🟡 Draft | MCP server API specification |
| MCP Phase 1 Tasks | [`mcp_phase1_tasks.md`](mcp_phase1_tasks.md) | 🟢 Frozen | MCP Phase 1 task list |
| MCP Roadmap | [`mcp_roadmap.md`](mcp_roadmap.md) | 🟡 Draft | MCP server feature roadmap |
| NASA Coding Guidelines | [`nasa_coding_guidelines.md`](nasa_coding_guidelines.md) | 🟢 Frozen | NASA/JPL Power-of-10 coding standards reference |
| Phase 3 Hazard Analysis | [`phase3_hazard_analysis.md`](phase3_hazard_analysis.md) | 🟢 Frozen | Phase 3 hazard analysis notes |
| Phase 3 Plan | [`phase3_plan.md`](phase3_plan.md) | 🟢 Frozen | Phase 3 implementation plan |
| Phase 3 Task Actions | [`phase3_task1_actions.md`](phase3_task1_actions.md) | 🟢 Frozen | Phase 3 task action items |
| Phase 3 Tasks | [`phase3_tasks.md`](phase3_tasks.md) | 🟢 Frozen | Phase 3 task list |
| Requirements RTM | [`requirements_rtm.md`](requirements_rtm.md) | 🟡 Draft | Requirements traceability matrix |
| CHANGELOG | [`../CHANGELOG.md`](../CHANGELOG.md) | 🔵 Active | Versioned change history (Keep a Changelog format) |

## 9. Research Scripts

| Script | Path | Purpose |
|--------|------|---------|
| Experiment Runner | [`../scripts/research/run_experiments.py`](../scripts/research/run_experiments.py) | Generates reproducible CSV/Markdown evidence artifacts |

---

## Governance Rules

1. **Every new doc** must be added to this index before merge.
2. **Frozen docs** require an ADR + version bump to modify.
3. **Draft docs** must move to Active or be archived within 30 days.
4. **Quarterly pruning:** review all docs; archive stale material; update statuses.
5. **PR checklist:** if your change touches behavior, schema, CLI, or safety rules, the corresponding doc and this index must be updated in the same PR.