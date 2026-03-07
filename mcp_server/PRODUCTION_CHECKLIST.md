# MCP Server Production Checklist

Priority 1 — Safety & access control (must-have)
- [x] Enforce RBAC on sensitive endpoints (write_file, run_cargo, run_simulator)
- [x] API key generator script (mcp_server/scripts/generate_api_key.js)
- [x] README auth usage documented (mcp_server/README.md)
- [x] Enforce allowed_paths / allowed_commands across endpoints
- [x] Basic concurrency limits and request timeouts
- [ ] Per-token concurrency quotas (separate from overall limits)
- [x] Port binding logic – **removed**; server now uses stdio-only (no `.mcp_port` file).
- [x] Pipe binding logic – N/A; named pipes are no longer supported.
- [x] StdIo proxy implemented so client can communicate without a network port
- [ ] Harden key management: hashed-token verification (server) + revoke/rotate CLI
- [ ] JSON schema validation for all endpoint payloads (AJV)
- [ ] Structured, tamper-evident audit log (JSONL + HMAC) + rotation/retention

Priority 2 — Runtime hardening & sandboxing
- [ ] Child-process kill-on-timeout & resource limits (CPU/mem)
- [ ] Per-key rate limits and throttling
- [ ] Optional Docker sandbox mode for running untrusted binaries
- [ ] Strict executable whitelist by path + optional binary hash verification

Priority 3 — Packaging & deployment
- [ ] Dockerfile + docker-compose / k8s manifest examples
- [ ] TLS support & bind-to-local-by-default
- [ ] Health/readiness/metrics endpoints (Prometheus)
- [ ] Graceful shutdown and signal handling
- [ ] Windows service / systemd service examples

Priority 4 — Tests & CI
- [x] Unit tests for endpoints (RBAC, path whitelists)
- [ ] Concurrency and quota enforcement tests
- [ ] Integration tests (spin server, run safe cargo/test flows)
- [ ] GitHub Actions workflow: lint, build, test, integration

Priority 5 — Secrets & security posture
- [ ] Move secrets/config into env/vault support; remove sensitive defaults from repo
- [ ] Store API keys hashed; add revoke list and admin CLI
- [ ] Rate limiting, IP allowlist, and protection for audit signing keys

Priority 6 — Observability & reliability
- [ ] Structured logging + tracing
- [ ] Metrics and dashboards; SLOs and alerting rules
- [ ] Backup/retention policy for .mcp_backups and logs

Immediate next implementation steps (I will do next in ACT mode)
1. Implement key revocation & hashed-token verification (done: server supports hashed keys) and add revoke CLI.
2. Add AJV JSON schema validation for endpoints and wire into server.
3. Replace edits.log with signed JSONL audit entries and add rotation.
4. Add unit tests for RBAC and allowed_paths and create GH Actions skeleton.

To proceed now, confirm and I will implement step 1 (revoke CLI + hashed-key storage improvements) and push tests for verification.

Additional next steps:
- Begin work on per-token concurrency tracking and associated unit tests.
- Draft the MIRR stress-test generator script under `scripts/` and add a placeholder integration test.
- Update checklist as Phase 1 design tasks are completed lower in the roadmap.