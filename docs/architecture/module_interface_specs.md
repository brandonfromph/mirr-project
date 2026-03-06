# Architecture & Module Interface Specs — Phase 3 (Task 2)

Goal
- Finalize architecture decomposition and module interfaces to ensure deterministic, bounded, and verifiable implementation (follow NASA Power-of-10).

Scope
- Cover critical runtime and compiler modules: runtime, executor, temporal compiler, emitter, parser, bootstrap/porting glue.

Module decomposition (high-level)
- mirr_runtime — runtime lifecycle, initialization, preallocated pools, runtime monitors
- mirr_executor — hot-path execution, scheduling, guard invocation
- temporal/compiler — temporal lowering, scheduling policy, deterministic transforms
- temporal/emit — low-level emit, resource budgeting, timing annotations
- mirr_driver / bootstrap_runner — self-hosting entrypoints and orchestration
- parser / lexer — parsing front-end (bounded loops, fixed-size buffers)
- validation / semantic — checks and invariants

Module interface spec template
- Module: <name>
- Responsibility: one-sentence
- Public functions / APIs:
  - fn name(args) -> Result<T, E>
    - Inputs: types and bounds (explicit)
    - Outputs: type
    - Precondition: explicit
    - Postcondition: explicit
    - Error returns: enumerate and meaning
    - Assertions: list invariants checked (>=2 per critical function where applicable)
    - Fixed bounds: document loop upper bounds, buffer sizes, and stack usage
- Resource budget:
  - Worst-case stack (bytes)
  - Heap usage (preallocated pool size; post-init heap = 0)
  - CPU worst-case path (documentation)
- Verification hooks:
  - Tests to exercise invariants
  - Logging / counters for runtime budgets

Fixed-bounds & control-flow rules (enforce Power-of-10)
- No recursion.
- All loops must state an explicit integer upper bound in comments and interface docs.
- No dynamic allocation after init: allocate fixed pools at startup or use stack-local fixed arrays.
- Limit pointer indirection to single dereference.
- Functions must be short (single-sheet guideline) — break complex behavior into smaller functions.
- Each public API must validate parameters and return Result; callers must check returns.

Determinism requirements
- Deterministic iteration order: avoid HashMap iteration where order matters; use deterministic maps or sort keys.
- RNG usage: only via injected RNG seeded from inputs or test harness.
- Scheduling: temporal/compiler must produce a canonical ordering given same inputs and seed — record seed provenance in artifacts.

Resource budgeting process
1. For each module, estimate worst-case stack and heap.
2. Add instrumentation counters and runtime asserts that verify budgets during tests.
3. Capture budgets in module header comments and docs/architecture/resource_budgets.csv.

Interface review checklist (for PRs)
- [ ] API inputs/outputs documented with bounds
- [ ] Pre/Postconditions present
- [ ] At least two assertions in critical functions (where meaningful)
- [ ] No post-init heap allocation introduced
- [ ] Loop bounds documented and defensively enforced
- [ ] Caller checks return values of non-void functions
- [ ] Static checks (clippy, fmt) pass

Deliverables (this task)
- docs/architecture/module_interface_specs.md (this file)
- docs/architecture/resource_budgets.csv (create after estimates)
- Interface headers added to each module source (small docblock templates)
- Peer-reviewed interface PRs

Immediate next actions
1. Generate per-module spec files under docs/architecture/ using the template for:
   - src/mirr_runtime.rs
   - src/mirr_executor.rs
   - src/temporal/compiler.rs and src/temporal/emit.rs
2. Run a brief static analysis pass to list loops and potential dynamic allocations (tools: cargo clippy / grep).
3. Schedule architecture walkthrough with Architect + Compiler Lead + Runtime Lead.
4. Add resource_budgets.csv stub to docs/architecture/.