# MCP Phase 1 Implementation Tasks

This document mirrors the detailed breakdown in `mcp_roadmap.md` and serves as a
working checklist for the core safety and API work.

## Design

* Define RBAC model, token format, permission sets.
* Path whitelist schema and canonicalization rules.
* Per-token concurrency quota mechanism.
* JSON schema selection (AJV) and author core schemas.

## Implementation

* `auth` module: token parsing/verification/expiration.
* RBAC enforcement middleware.
* Filesystem helpers: canonicalization + whitelist check.
* Concurrency tracking per token.
* AJV validation integration at transport layer.

## Testing

* Unit tests for RBAC logic.
* Path-resolution corner cases.
* Concurrency/quota tests.
* Schema validation tests (fuzz/malformed payloads).
* Integration tests covering multiple tokens and permission profiles.

## Documentation

* Update `PRODUCTION_CHECKLIST.md` with Phase‑1 items.
* Add API spec snippets to `docs/mcp_api.md` or README.

## Notes

Tasks in this document are tracked also in GitHub issues and the main
`PRODUCTION_CHECKLIST.md` file.  Remove items from the checklist as they
reach completion and update this file accordingly.
