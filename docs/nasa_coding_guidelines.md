# NASA‑Style Coding Guidelines for MCP Server

The MCP server forms a critical safety boundary between AI/IDE clients and the
workspace.  Each line of code must therefore meet high standards of readability,
robustness and testability.  These guidelines capture the minimum expectations
for anything committed to `mcp_server/`.

## 1. Type Safety

- Prefer explicit TypeScript types over `any`.  Use `express.Request` and
  `express.Response` for web handlers.
- Avoid casting whenever possible; if you must cast, leave a `// FIXME` with a
  rationale.
- Run `tsc` with `--noImplicitAny` and treat compiler warnings as errors.

## 2. Documentation & Comments

- Every exported function, handler or complex logic block should include a
  JSDoc comment explaining its purpose, inputs, outputs, and failure modes.
- Use descriptive variable names; avoid abbreviations unless they are domain
  accepted (e.g. `req`, `res`, `rbac`).
- Code should read like a narrative; follow the NASA "annotated code" style
  that explains *why* not just *what*.

## 3. Testing

- Every public endpoint must have at least one positive and one negative test.
- Edge conditions (invalid input, permission denied, concurrency limits) are
  as important as the happy path.
- Use the existing `stdio_proxy_test.js` harness as the canonical example.
- Aim for >90% line coverage on any new modules.

## 4. Error Handling

- Propagate errors explicitly; do not swallow exceptions.  Prefer `try/catch`
  with logging before returning an HTTP error.
- Convert unexpected exceptions to a 500 response with a descriptive message
  (but never leak secrets).
- Use well-defined error codes in JSON bodies (e.g. `concurrency_limit_exceeded`).

## 5. Concurrency & Resource Limits

- Assume every request could be run simultaneously by multiple clients.
- Enforce per-token concurrency limits (see `withConcurrencyLimit`).
- Long‑running operations should provide a mechanism to cancel or timeout.

## 6. Coding Style & Formatting

- Follow the project ESLint/Prettier configuration (if added).  Consistent
  indentation and quote style reduces cognitive load.
- Limit line length to ~100 characters for readability in diff viewers.
- Avoid deeply nested blocks; extract helpers where logical.

## 7. Security

- Always validate and sanitize inputs.  Never trust client‑supplied paths or
  command arguments.
- Enforce the workspace root via `resolveSafe` and `isPathAllowed` on every
  filesystem operation.
- Log authentication attempts and failures; do not log API keys or secrets.

## 8. Review Process

- All changes must be reviewed by at least one other developer familiar with
  the MCP project.  Code should be accompanied by a summary of the reasoning
  behind design decisions.
- Major features (new endpoints, schema changes) require updates to
  `mcp_roadmap.md` and corresponding tests.

---
*These guidelines are a living document.  Add new rules as the project matures.*