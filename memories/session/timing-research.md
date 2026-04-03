# Session: Timing Semantics Research for MEGA-10/11

## Task
Reframe composite ops proposal from "add execution coverage" to "define cycle/timing semantics".

## Key Findings

### 1. Current Simulator Cycle Model
- **Architecture**: Tick-based interpreter (src/mirr_executor/)
- **Execution Model**: R-SPU ISA with deterministic single-cycle execution
- **Key Guarantee**: EVERY R-SPU instruction executes in exactly 1 cycle
  - From src/emit/rspu_isa.rs: "Every variant executes in a single cycle (deterministic timing model)"
- **Timing Properties**:
  - Single-issue, in-order processor
  - No pipelining stalls
  - No branch mispredictions
  - No variable-latency operations
  - WCET = instruction count (fully predictable)

### 2. Existing Cycle Guarantees
- **Register tier** (LOAD_INPUT, STORE_OUTPUT, MOV, LOAD_IMM): 1 cycle
- **ALU tier** (ALU, ALU_IMM, ALU_UNARY): 1 cycle
- **Temporal tier** (SR_INIT, SR_TICK, CTR_INIT, CTR_TICK, etc.): 1 cycle
- **Reflex tier** (REFLEX_IF, PREV): 1 cycle
- **Safety/Exception/Control tiers**: 1 cycle each

### 3. Composite Operations Status
- **ArrayIndex**: Currently unsupported (error E720)
- **FieldAccess**: Currently unsupported (error E720)
- **ArrayLiteral, StructLiteral**: Unsupported
- **Location**: src/emit/rspu.rs:391-397
- **Issue**: "R-SPU does not support composite type expressions"

### 4. Composite Type Architecture (from MEGA-10 proposal)
- **Width Calculation**: Recursive
  - Arrays: element.width() * length
  - Structs: Sum of all field widths
- **Constraint Rules** (Wave 3):
  - ArrayIndex: 	arget_width == array_element_width
  - FieldAccess: 	arget_width == struct_definition.find(field).width()
  - ArrayLiteral: 	arget_width == sum(all_elements.width())
- **Verilog Mapping**:
  - [u8; 4] ? logic [7:0] signal_name [0:3]
  - struct P {x: u8;} ? packed struct typedef
  - ixed<16, 8> ? logic [15:0] with documentation

### 5. Temporal Semantics Documents
- Phase 2 document covers guard lowering (shift registers vs counters)
- Shift register threshold: 16 cycles (N = 16 ? SR, N > 16 ? counter)
- Temporal guards have defined cycle costs
- BUT: No explicit timing contract for expression evaluation

### 6. Testing Infrastructure
- No timing/cycle-specific tests found
- Temporal tests exist but focus on lowering, not cycle accounting
- No regression test suite for timing guarantees

## Gap Analysis
1. **No formal timing contract** - latency bounds undefined
2. **No cycle accounting for composite ops** - ArrayIndex/FieldAccess cost unknown
3. **No timing test infrastructure** - no way to prevent regressions
4. **No width/cycle coupling** - wide ops might need multiple cycles?

## Architecture Implications
- MIRR is sub-Turing, safety-critical, requires deterministic timing
- R-SPU enforces bounded execution (NASA P10)
- Timing must be predictable for autonomic control (MAPE-K)
- Width does not imply variable cycle cost (maintain 1-cycle execution)

## Next Steps
1. Define formal timing contract with latency bounds
2. Assign cycle costs to composite operations (recommend 1 cycle, bounded latency)
3. Create timing test infrastructure
4. Add invariant checks to catch timing regressions
