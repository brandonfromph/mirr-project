# Loop and Allocation Scan — Phase 3 Task 2

Summary
- Scan results: 151 loop occurrences and 120 allocation/format sites found across src/*.rs.
- Purpose: identify hot-path loops and post-init dynamic-allocation sites to enforce "no post-init dynamic allocation" and fixed-bounds rules.

High-risk files / findings
- src/mirr_executor.rs
  - Uses HashMap::new(), Vec::with_capacity(), HashMap<String, Value> created inside execution path.
  - Persistent and per-tick maps (persistent_env, env, guard_active, guard_counters) are constructed at tick-time → potential heap allocations during hot path.
  - Many loops over signals/reflexes (for g in &prog.module.guards, for r in &prog.module.reflexes, etc.) — must document and bound by module constants.
- src/mirr_runtime.rs / src/mirr_driver.rs
  - Several Vec::with_capacity(TOKEN_BUFFER_CAPACITY) uses — OK if capacity is constant and allocated at init; ensure buffer lives in preallocated pool rather than reallocated per tick.
- src/validation/semantic.rs and src/parser/*
  - Many Vec::with_capacity calls used during parsing/validation — acceptable if parsing is offline or initialization; document bounds.
- src/temporal/* (compiler, emit, low_level_ir)
  - format!/String::from/Box::new used for IR construction and diagnostics — acceptable during compile phases if bounded; ensure compile-time allocations are documented and limited.
- src/lexer/tokenizer.rs
  - Bounded loops with preallocated token buffers — aligned with Power-of-10 rules.

Immediate remediation recommendations
1. Hot-path preallocation
   - Move per-tick data structures into RuntimeHandle (preallocate Vec/HashMap pools during init) and reuse them each tick.
   - Replace HashMap usage in hot-path with fixed-size arrays or deterministic maps keyed by index where possible.
2. Replace dynamic allocations
   - For frequently-created small strings or formatted diagnostics in hot-path, avoid format! on hot path; defer or use preallocated buffers.
3. Enforce fixed bounds
   - Add explicit constants (MAX_SIGNALS, MAX_REFLEXES, MAX_GUARDS, TOKEN_BUFFER_CAPACITY) in module headers and assert loop bounds.
4. Add allocation-detection tests
   - Add an allocation audit test that runs representative hot-path scenarios under instrumentation and fails if any heap allocations occur post-init.
   - Integrate allocation audit into CI (nightly or gating for performance-critical PRs).
5. Add assertions and documentation
   - Document per-loop upper bounds in source comments and add runtime assertions in debug builds to detect bound violations.

Next concrete tasks
- [ ] Add module header docblocks in mirr_executor.rs and mirr_runtime.rs describing preallocated structures and budgets.
- [ ] Implement a small PoC refactor: move env/persistent_env/guard_active into a reusable preallocated struct within RuntimeHandle.
- [ ] Add allocation-audit test (e.g., using jemalloc/profiler or Rust allocation hooks) and CI step.
- [ ] Re-run scan after code changes and record results here.

References
- Files inspected: src/mirr_executor.rs, src/mirr_runtime.rs, src/validation/semantic.rs, src/parser/*, src/temporal/*, src/lexer/tokenizer.rs
- Related docs: docs/architecture/per_module_specs.md, docs/requirements_rtm.md (PH3-REQ-002 / PH3-REQ-007)