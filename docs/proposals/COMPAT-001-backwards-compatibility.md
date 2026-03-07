# Proposal: Backwards Compatibility Campaign

**Campaign ID:** COMPAT-001
**Author:** Claude (AI pair-programmer)
**Status:** PROPOSED
**Date:** 2026-03-08

---

## Problem Statement

Since Phase 6, the MIRR compiler has added several features that introduce breaking changes across four surfaces:

1. **Rust API** — New enum variants (`PropertyFormula`, `PropertyDirective`) break exhaustive `match` in downstream Rust code
2. **JSON netlist schema** — New `properties` array (always present), new `directive` field, 3 new `kind` values
3. **MIRR language syntax** — 5 new keywords (`cover`, `assume`, `eventually`, `within`, `followed_by`)
4. **Error message format** — All errors now have `[E1xx]`–`[E4xx]` prefix codes

None of these changes have a CHANGELOG, a version bump, a migration guide, or schema versioning. Any external consumer — Rust crate user, JSON netlist parser, or MIRR syntax highlighter — will break silently.

---

## Scope

Add versioning, a CHANGELOG, a migration guide, and schema version fields so that existing and future consumers can detect and adapt to changes. **No code behavior changes.** This is documentation and metadata only, with two small code additions (schema version fields).

---

## Deliverables

### 1. `CHANGELOG.md` — Full change history (CREATE)

Standard Keep a Changelog format. Retroactive entries for all phases:

```
## [Unreleased]

### Added
- PropertyDirective: cover, assume keywords for property blocks
- PropertyFormula: NeverImplies, EventuallyWithin, AlwaysFollowedBy variants
- FIRRTL emit target (--emit firrtl)
- SVA standalone emit target (--emit sva)
- Pattern system: def/reflect with ${param} substitution
- Error code prefixes [E1xx]–[E4xx]
- VS Code Copilot skills (.github/skills/)
- 56 property directive tests, 12 FIRRTL tests

### Changed
- Error messages now prefixed with error codes (e.g. "[E100] Parse error: ...")
- JSON netlist always includes `properties` array (previously absent)
- PropertyJson now includes `directive` field
- PropertyJson `kind` field has 3 new values: never_implies, eventually_within, always_followed_by
- SVA output uses directive-dependent keyword (cover/assume instead of always assert)
- DOT property nodes use directive-dependent colors

### Removed
- MirrError::LexicalError variant (was dead code)
- MirrError::TemporalCausalityViolation variant (was dead code)
```

### 2. `docs/migration-guide.md` — Consumer migration guide (CREATE)

Targeted at three audiences:

#### For Rust API consumers
- `PropertyFormula` now has 6 variants — add wildcard arms or handle new variants
- `PropertyDecl` has a new `directive` field — use `PropertyDirective::default()` (Assert) for construction
- `Module` has 3 new fields (`properties`, `pattern_calls`, `pattern_origins`) — all have `#[serde(default)]`
- New public types: `PropertyDirective`, `PatternDef`, `PatternOrigin`, `PatternParam`, etc.
- New public functions: `expand_patterns`, `emit_firrtl`, `emit_sva_only`

#### For JSON netlist consumers
- The `properties` array is now always present (may be empty `[]`)
- Each property object now has a `directive` field: `"assert"`, `"cover"`, or `"assume"`
- The `kind` field has 3 new possible values
- The `pattern_origins` array appears conditionally (only if patterns were expanded)
- **Action:** If you validate JSON strictly, allow unknown fields. If you match on `kind`, add the 3 new values.

#### For MIRR syntax tool authors
- 5 new keywords are valid inside `property` blocks: `cover`, `assume`, `eventually`, `within`, `followed_by`
- These do **not** affect `signal`, `guard`, or `reflex` parsing
- `def` and `reflect` are new top-level keywords for the pattern system
- **Action:** Update lexer/grammar if you parse MIRR syntax.

#### For error message parsers
- All error messages now start with `[E1xx]`–`[E4xx]` prefix
- **Action:** If you regex-match error messages, update patterns to expect the prefix.

### 3. Add `schema_version` to JSON netlist output (CODE CHANGE)

Add a top-level `"schema_version": "0.2.0"` field to `JsonNetlist` struct:

```rust
pub struct JsonNetlist {
    pub schema_version: String,  // NEW — "0.2.0"
    pub module_name: String,
    // ... existing fields
}
```

This lets JSON consumers detect which schema they're reading and adapt accordingly. The version follows semver:
- `0.1.0` — original schema (pre-properties, pre-patterns)
- `0.2.0` — adds properties array, directive field, pattern_origins

### 4. Version bump in `Cargo.toml` (METADATA CHANGE)

Bump from `0.1.0` to `0.2.0` to signal the API-breaking changes:

```toml
[package]
version = "0.2.0"
```

### 5. Update `docs/INDEX.md` (EDIT)

Add entries for CHANGELOG.md and migration-guide.md.

### 6. Update `README.md` (EDIT)

- Update test badge from 632 to 711
- Add link to CHANGELOG.md
- Add link to migration guide

---

## What this campaign does NOT do

- Does not add backwards compatibility shims or deprecated aliases
- Does not revert any features
- Does not add a formal JSON Schema (.json-schema) file (future campaign)
- Does not add semver automation or release tooling

---

## Constraints

| Constraint | Rule |
|---|---|
| No behavior changes | Compiler output remains identical |
| No new tests | This is documentation + 1 metadata field |
| Semver compliance | 0.1.0 → 0.2.0 (minor bump for pre-1.0 breaking changes) |
| CHANGELOG format | Keep a Changelog v1.1.0 |
| Schema version field | Always present, never null |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| CHANGELOG gets stale | Medium | Medium | Each future proposal must include a CHANGELOG entry |
| schema_version goes un-bumped | Medium | High | CI could lint for version consistency (future) |
| Migration guide misses a breaking change | Low | Medium | Audit was comprehensive; guide covers all 4 surfaces |

---

## Breaking Change Audit Summary

| Surface | Change | Serde default? | Backwards compatible? |
|---|---|---|---|
| `PropertyFormula` enum | 3 new variants | N/A (enum) | NO — exhaustive match breaks |
| `PropertyDirective` enum | New type (3 variants) | N/A | NO — new type in API |
| `PropertyDecl.directive` | New field | `#[serde(default)]` = Assert | JSON deser: YES. Rust struct literal: NO |
| `Module.properties` | New field | `#[serde(default)]` = `[]` | JSON deser: YES. Rust struct literal: NO |
| `Module.pattern_calls` | New field | `#[serde(default)]` = `[]` | JSON deser: YES. Rust struct literal: NO |
| `Module.pattern_origins` | New field | `#[serde(default)]` = `[]` | JSON deser: YES. Rust struct literal: NO |
| `MirrProgram.patterns` | New field | `#[serde(default)]` = `[]` | JSON deser: YES. Rust struct literal: NO |
| `JsonNetlist.properties` | Always present array | N/A | JSON consumers must tolerate new key |
| `PropertyJson.directive` | New field | N/A | JSON consumers must tolerate new key |
| `PropertyJson.kind` | 3 new values | N/A | Strict validators break |
| Error messages | `[Exxx]` prefix | N/A | Regex parsers break |
| MIRR keywords | 5 new | N/A | External parsers break |
| CLI flags | `--emit firrtl`, `--emit sva` | N/A | Purely additive — no break |

---

## Estimated File Changes

| File | Action | Lines |
|---|---|---|
| `CHANGELOG.md` | CREATE | ~80 |
| `docs/migration-guide.md` | CREATE | ~150 |
| `src/emit/json_netlist.rs` | EDIT | +3 (schema_version field + init) |
| `Cargo.toml` | EDIT | +1 (version bump) |
| `docs/INDEX.md` | EDIT | +4 |
| `README.md` | EDIT | +4 |

---

## Execution Order

| Step | Deliverable | Depends on |
|---|---|---|
| 1 | Create `CHANGELOG.md` | — |
| 2 | Create `docs/migration-guide.md` | — |
| 3 | Add `schema_version` to `JsonNetlist` | — |
| 4 | Bump version in `Cargo.toml` | Step 3 |
| 5 | Update `docs/INDEX.md` | Steps 1–2 |
| 6 | Update `README.md` | Steps 1–2 |
| 7 | Run `cargo test --all` + clippy | Steps 3–4 |
