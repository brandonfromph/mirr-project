# Phase 3 — Task 1 Implementation Actions (RTM population & owner assignment)

Objective
- Complete Task 1 by confirming owners, assigning verifiers, scheduling the RTM review, and creating tracked work items.

Quick summary of current state
- RTM populated with PH3-REQ-001..PH3-REQ-010 in docs/requirements_rtm.md with provisional owners (roles).
- Hazard analysis initial draft in docs/phase3_hazard_analysis.md.

Concrete next actions (do these one-by-one and mark done)

1) Confirm owners and verifier names
- Replace provisional role names in docs/requirements_rtm.md with real assignees (GitHub usernames or full names).
- For each PH3-REQ, add "Verifier" column entry with at least one reviewer.
- Suggested format: "Owner: <name> (@gh-user) — Verifier: <name> (@gh-user)"
- Deliverable: updated docs/requirements_rtm.md

2) Create tracking issues for each requirement
- Create a GitHub issue per PH3-REQ with:
  - Title: "PH3-REQ-XXX: <short description>"
  - Body: Requirement text, acceptance criteria, RTM link, suggested estimate, owner, verifier, HA references.
- Link issues to RTM by including the issue number in the RTM "Notes" column.
- Deliverable: issues created and linked.

3) Schedule RTM safety review meeting
- Create a calendar invite (include Product, Architect, Compiler Lead, Runtime Lead, Verification Engineer, Test Lead, Release Manager).
- Attach these docs: docs/phase3_plan.md, docs/requirements_rtm.md, docs/phase3_hazard_analysis.md, docs/phase3_tasks.md.
- Goal: review & confirm owners, acceptance criteria, and sign-off thresholds.
- Deliverable: meeting invite and agenda.

4) Produce sign-off checklist for meeting
- Agenda items:
  - Quick overview of Phase 3 plan and RTM
  - Walk PH3-REQ 1..10: owner, acceptance criteria, verification method
  - Map HA entries to requirements
  - Capture action items and open issues
  - Agree on estimates and dates
- Deliverable: meeting notes and updated RTM.

5) Update RTM verification statuses as work starts
- When an issue/PR is opened to satisfy a requirement, set its Verification Status = "In progress" and add PR/CI links in Notes.
- When CI evidence exists, update to "Verified" and reference artifacts (CI run id, artifact path).

Commands & templates (examples — run locally / in CI manually)

- Create branch for RTM edits:
  git checkout -b chore/PH3-RTM-populate-owners

- Issue template (body):
  PH3-REQ: PH3-REQ-XXX
  Short description: ...
  Acceptance criteria: ...
  Owner: ...
  Verifier: ...
  RTM row: docs/requirements_rtm.md#L...
  Tests/fixtures: ...
  HA refs: HA-XXX
  Est. work-days: X

- Meeting agenda template file: docs/meetings/phase3_rtm_review_agenda.md

Risks & mitigations
- Risk: Owners unavailable → Mitigation: assign interim owners and capture in meeting.
- Risk: RTM too coarse → Mitigation: break requirements into smaller PH3-REQ-0XXa/b as needed.

Deliverables this step creates in repo
- docs/phase3_task1_actions.md (this file)
- Updated docs/requirements_rtm.md (after owner confirmation)
- docs/meetings/phase3_rtm_review_agenda.md (create before scheduling)

Acceptance criteria for Task 1
- All PH3-REQ rows have an assigned Owner and Verifier.
- Issues created for each PH3-REQ and linked in RTM.
- RTM review meeting scheduled with stakeholders and agenda attached.