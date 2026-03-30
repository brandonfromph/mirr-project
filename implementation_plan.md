# Implementation Plan

## Overview

Extend the MIRR proposal governance infrastructure by enhancing `scripts/validate_proposals.py` with stricter validation rules, adding a CI job for automated proposal checking, updating contributor documentation, and creating a new `scripts/repo_metrics.py` script for repository metric tracking. This implementation follows the deliverables outlined in proposal 087 (REPO-AUDIT-GOVERNANCE).

## Context

The MIRR repository uses a proposal-based campaign workflow where all changes must be documented in `proposals/*.md` files. Currently, `scripts/validate_proposals.py` only checks filename format and the presence of D1..D7 rows in the Debt Audit section. Proposal 087 identifies gaps: no validation for status values, no uniqueness enforcement for Proposal # and Campaign ID, no check for non-empty "Action" values in the Debt Audit, and no CI integration. This implementation closes those gaps to enforce Zero-Debt and NASA Power-of-10 governance automatically.

## Types

No new Rust types are introduced. The implementation uses Python data structures (dicts, sets, lists) for validation logic in `scripts/validate_proposals.py` and `scripts/repo_metrics.py`.

### Python Data Structures (validate_proposals.py)

```python
VALID_STATUSES = {"PROPOSED", "SIGNED", "EXECUTED", "VETOED"}

# Regex for header fields
STATUS_RE = re.compile(r"^\*\*Status:\*\*\s*(.+)$", re.MULTILINE)
PROPOSAL_NUM_RE = re.compile(r"^\*\*Proposal #:\*\*\s*(\d+)$", re.MULTILINE)
CAMPAIGN_ID_RE = re.compile(r"^\*\*Campaign ID:\*\*\s*(.+)$", re.MULTILINE)
```

### Python Data Structures (repo_metrics.py)

```python
EXPECTED_METRICS = {
    "src_rust_files": None,  # populated from current scan
    "tests_rust_files": None,
    "proposals_count": None,
}
```

## Files

### New Files

| File | Purpose |
|------|---------|
| `scripts/repo_metrics.py` | Compute and verify repository metrics (src file count, tests file count, proposals count, unsafe/deprecated violation scans) |

### Modified Files

| File | Changes |
|------|---------|
| `scripts/validate_proposals.py` | Add validation for: status values, unique Proposal #, unique Campaign ID, non-empty Debt Audit "Action" values, Current State Assessment section presence |
| `.github/workflows/ci.yml` | Add `proposal-validation` job that runs `python scripts/validate_proposals.py --strict` on PRs touching `proposals/` or `.github/**` |
| `docs/contributing.md` | Add "Required Proposal Fields" section documenting mandatory header fields, Debt Audit table format, and Current State Assessment requirements |

### Files Not Modified

| File | Reason |
|------|--------|
| `CLAUDE.md` | Already references campaign workflow; no changes needed per proposal |
| `proposals/*.md` | Existing proposals are grandfathered; new validation applies to future proposals |

## Functions

### New Functions (scripts/validate_proposals.py)

| Function | Signature | File | Purpose |
|----------|-----------|------|---------|
| `validate_status` | `validate_status(text: str, filename: str) -> list[str]` | `scripts/validate_proposals.py` | Extract and validate the Status header field against VALID_STATUSES |
| `validate_unique_ids` | `validate_unique_ids(files: list[Path]) -> list[str]` | `scripts/validate_proposals.py` | Check that all Proposal # and Campaign ID values are unique across all proposal files |
| `validate_debt_audit_actions` | `validate_debt_audit_actions(text: str, filename: str) -> list[str]` | `scripts/validate_proposals.py` | Verify each D1..D7 row has a non-empty "Action" column |
| `validate_current_state` | `validate_current_state(text: str, filename: str) -> list[str]` | `scripts/validate_proposals.py` | Check that "Current State Assessment" section exists and has at least one table row |

### Modified Functions (scripts/validate_proposals.py)

| Function | Current File | Changes |
|----------|--------------|---------|
| `validate_file` | `scripts/validate_proposals.py` | Call new validation functions: `validate_status`, `validate_debt_audit_actions`, `validate_current_state` |
| `main` | `scripts/validate_proposals.py` | After per-file loop, call `validate_unique_ids` across all files; add `--check-all` flag support |

### New Functions (scripts/repo_metrics.py)

| Function | Signature | File | Purpose |
|----------|-----------|------|---------|
| `count_rust_files` | `count_rust_files(directory: Path) -> int` | `scripts/repo_metrics.py` | Count `.rs` files in a directory recursively |
| `count_proposals` | `count_proposals(proposals_dir: Path) -> int` | `scripts/repo_metrics.py` | Count `.md` files in proposals/ (excluding README) |
| `scan_violations` | `scan_violations(root: Path) -> dict[str, int]` | `scripts/repo_metrics.py` | Scan for `unsafe` keyword usage (excluding attributes/doc), `#[deprecated]`, `#[allow(dead_code)]` |
| `generate_metrics` | `generate_metrics(root: Path) -> dict` | `scripts/repo_metrics.py` | Orchestrate all metric computations and return a dict |
| `main` | `main() -> int` | `scripts/repo_metrics.py` | CLI entry point: compute metrics, optionally compare against a baseline file, print results |

## Classes

No new classes are needed. The implementation uses standalone functions following the existing pattern in `scripts/validate_proposals.py`.

## Dependencies

No new Python packages are required. The implementation uses only stdlib modules already imported (`re`, `pathlib.Path`, `argparse`). No Rust dependency changes.

## Testing

### Manual Verification

1. Run `python scripts/validate_proposals.py --strict` — should pass all existing proposals (or report warnings for legacy format)
2. Run `python scripts/repo_metrics.py` — should print current metrics
3. Create a test proposal with invalid status and verify the validator catches it

### CI Verification

After adding the CI job:
1. Create a test PR that modifies `proposals/` with an intentional error (e.g., missing Debt Audit row)
2. Verify the `proposal-validation` CI job fails with a clear error message
3. Verify normal PRs without proposal changes skip the validation job

### Existing Test Suite

No changes to Rust tests. The CI job runs alongside existing jobs (`test`, `coverage`, `supply-chain`, etc.) without interference.

## Implementation Order

| Step | Section | Files | Depends on |
|------|---------|-------|-----------|
| 1 | Extend validate_proposals.py | `scripts/validate_proposals.py` | — |
| 2 | Add repo_metrics.py | `scripts/repo_metrics.py` | — |
| 3 | Add CI job | `.github/workflows/ci.yml` | Step 1 |
| 4 | Update contributing docs | `docs/contributing.md` | Steps 1, 2 |

Steps 1 and 2 are independent and can be done in parallel. Step 3 depends on Step 1 (the CI job runs the extended validator). Step 4 documents the new tools and should be done last.

## Breakage Map

| Step | What breaks | Why | Fixed in |
|------|------------|-----|----------|
| 1 | Existing proposals with legacy format may fail strict validation | New checks for status, unique IDs, non-empty actions | Step 4 documents grandfathering; `--strict` flag allows warnings-only mode |
| 3 | CI may fail on PRs that touch proposals/ with legacy format | New required check | Document in contributing.md that legacy proposals are grandfathered |
| — | No Rust code changes | All changes are Python scripts and CI config | — |

## Verification

```bash
# Validate all proposals (warnings mode)
python scripts/validate_proposals.py

# Validate all proposals (strict mode, errors on issues)
python scripts/validate_proposals.py --strict

# Compute repository metrics
python scripts/repo_metrics.py

# Existing Rust checks (unchanged)
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all