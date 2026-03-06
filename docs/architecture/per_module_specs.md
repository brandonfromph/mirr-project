# Per-module Interface Specs — Phase 3 (Task 2 implementation)

This file contains per-module interface specs for initial implementation work: mirr_runtime, mirr_executor, temporal/compiler, temporal/emit.

---

Module: mirr_runtime
- Responsibility: Manage runtime lifecycle, initialization, preallocated resource pools, and runtime monitors.
- Public functions (examples)
  - pub fn init(config: &RuntimeConfig) -> Result<RuntimeHandle, MirrError>
    - Inputs: RuntimeConfig (bounds: max_threads <= 1, pool_sizes explicit)
    - Outputs: RuntimeHandle
    - Precondition: config validated; called once at process start
    - Postcondition: All runtime pools allocated; no heap allocation allowed after init
    - Errors: MirrError::InvalidConfig, MirrError::AllocationFailure
    - Assertions: pool sizes > 0; no-null pointers; init completed flag set
    - Fixed bounds: pool capacities documented; loops for pool init bounded by pool size
- Resource budget (example placeholders)
  - Worst-case stack: 64 KiB
  - Heap post-init: 0 bytes (all pools preallocated)
  - Init-time heap: preallocated pools only
- Verification hooks: allocation audit tests, init-time assertions, unit tests for error returns

Module: mirr_executor
- Responsibility: Hot-path execution: evaluate guards, run reflexes, apply assignments deterministically.
- Public functions (examples)
  - pub fn tick(handle: &RuntimeHandle, inputs: &InputSignals) -> Result<ExecutionStats, MirrError>
    - Inputs: bounded InputSignals (max N signals)
    - Outputs: ExecutionStats (ticks consumed, allocations=0)
    - Precondition: runtime initialized
    - Postcondition: persistent state updated deterministically
    - Errors: MirrError::ExecutionError
    - Assertions: guard counters in-bounds, push buffer capacity respected
    - Fixed bounds: loops over signals/reflexes bounded by module-level constants
- Determinism rules: use deterministic iterations; avoid HashMap iteration when order matters
- Resource budget:
  - Worst-case stack: 32 KiB
  - Heap post-init: 0 bytes
- Verification hooks: temporal guard determinism tests, stress tests exercising fixed upper bounds

Module: temporal/compiler
- Responsibility: Lower MIRR temporal guard constructs to low-level IR deterministically and with fixed iteration limits.
- Public functions (examples)
  - pub fn compile_guards(guards: &[Guard], ctx: &mut CompileContext) -> Result<TemporalNetlist, MirrError>
    - Inputs: slice of Guards (length ≤ MAX_GUARDS)
    - Outputs: TemporalNetlist
    - Precondition: ctx validated; no recursion in lowering
    - Postcondition: netlist deterministic for given seed; invariants asserted
    - Errors: MirrError::TemporalCompilationError
    - Assertions: stage counts ≤ declared upper bound; signal_counter monotonic and bounded
    - Fixed bounds: MAX_GUARDS, MAX_STAGES per guard documented and enforced
- Determinism: seed provenance recorded; RNG only via injected seed
- Resource budget:
  - Worst-case stack: 64 KiB
  - Heap post-init: 0 bytes (temporary allocations during compile are allowed if bounded and documented; prefer stack/local buffers)
- Verification hooks: unit tests for lowering, invariants checked at pass boundaries

Module: temporal/emit
- Responsibility: Emit low-level IR and resource annotations; produce netlist diagnostics and DOT/HTML outputs.
- Public functions (examples)
  - pub fn emit_netlist(netlist: &TemporalNetlist, out: &mut impl Write) -> Result<(), MirrError>
    - Inputs: TemporalNetlist (bounded sizes)
    - Outputs: formatted netlist text
    - Precondition: netlist validated
    - Postcondition: no dynamic allocations in hot emit loops; emitted artefacts deterministic
    - Errors: MirrError::EmitError
    - Assertions: netlist sizes within budgets; counters match estimated resources
    - Fixed bounds: output buffer flush loops use bounded chunks
- Resource budget:
  - Worst-case stack: 32 KiB
  - Heap post-init: 0 bytes
- Verification hooks: emission parity tests, size/line-count assertions

---

Next concrete actions (implementation)
1. Add module header docblocks in the four source files with the spec summary and resource budget placeholders.
2. Create docs/architecture/resource_budgets.csv with exact numeric budgets once estimates are agreed.
3. Run a static scan to list all loop sites and dynamic allocation constructors (Vec::new(), HashMap::new(), Box::new(), etc.) and attach results to docs/architecture/loop_and_alloc_scan.md.
4. Schedule architecture walkthrough to finalize budgets and interface pre/postconditions.

Acceptable constraints & NASA practices enforced
- No recursion; fixed loop bounds; no post-init dynamic allocation; >=2 assertions per critical function where meaningful; caller checks return values.