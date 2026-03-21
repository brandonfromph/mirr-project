# MIRR Runtime Interface (MRI) Specification

**Version:** 1.0.0
**Date:** 2026-03-20
**Campaign:** MEGA-15 TARGET-PORTABLE

## Overview

The MIRR Runtime Interface (MRI) defines the abstract execution interface that
all MIRR targets must implement. Any platform implementing MRI can run MIRR
programs with guaranteed behavioral equivalence.

MRI guarantees:
- **Bounded execution:** Every `mri_tick()` completes within MAX_TICKS total calls
- **Deterministic I/O:** Inputs are read before outputs are written each tick
- **Sub-Turing:** No target can introduce unbounded computation

## Interface

### `mri_init`

```c
mri_module_t* mri_init(const uint8_t* module_binary, size_t len);
```

Initialize a MIRR module from a compiled binary. Returns NULL on failure.

**Parameters:**
- `module_binary`: Pointer to the compiled MIRR module bytes
- `len`: Length of the binary in bytes

**Returns:** Module handle, or NULL if the binary is invalid

### `mri_set_input`

```c
void mri_set_input(mri_module_t* mod, uint16_t port, uint64_t value);
```

Set the value of an input signal.

**Parameters:**
- `mod`: Module handle
- `port`: Input port index (0-based)
- `value`: Signal value (unsigned, up to 64 bits)

### `mri_tick`

```c
mri_status_t mri_tick(mri_module_t* mod);
```

Advance the module by one clock cycle. This is the core execution function.

**Execution order:**
1. Read all input signal values
2. Evaluate all guard conditions
3. Fire all active reflexes (assignments)
4. Update all output signal values
5. Increment tick counter

**Returns:**
- `MRI_OK`: Tick completed successfully
- `MRI_ERROR_TIMEOUT`: MAX_TICKS exceeded

### `mri_get_output`

```c
uint64_t mri_get_output(mri_module_t* mod, uint16_t port);
```

Read the value of an output signal.

**Parameters:**
- `mod`: Module handle
- `port`: Output port index (0-based)

**Returns:** Signal value (unsigned, up to 64 bits)

### `mri_check_property`

```c
bool mri_check_property(mri_module_t* mod, uint32_t property_id);
```

Evaluate an LTL property over the module's execution history.

**Parameters:**
- `mod`: Module handle
- `property_id`: Property index (0-based)

**Returns:** true if the property holds, false otherwise

### `mri_destroy`

```c
void mri_destroy(mri_module_t* mod);
```

Clean up module state and free all resources.

**Parameters:**
- `mod`: Module handle (may be NULL)

### `mri_run`

```c
mri_status_t mri_run(mri_module_t* mod, size_t max_ticks);
```

Run the module for `max_ticks` clock cycles and check all properties.

**Parameters:**
- `mod`: Module handle
- `max_ticks`: Maximum number of ticks to execute

**Returns:**
- `MRI_OK`: All ticks completed, all properties passed
- `MRI_ERROR_TIMEOUT`: MAX_TICKS exceeded
- `MRI_ERROR_PROPERTY_FAIL`: A property was violated

## Status Codes

```c
typedef enum {
    MRI_OK = 0,
    MRI_ERROR_INVALID_MODULE = -1,
    MRI_ERROR_OUT_OF_MEMORY = -2,
    MRI_ERROR_TIMEOUT = -3,
    MRI_ERROR_PROPERTY_FAIL = -4,
} mri_status_t;
```

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `MRI_MAX_TICKS` | 1,000,000 | Maximum ticks per `mri_run()` call |
| `MRI_MAX_INPUTS` | 256 | Maximum input ports per module |
| `MRI_MAX_OUTPUTS` | 256 | Maximum output ports per module |
| `MRI_MAX_SIGNALS` | 256 | Maximum total signals per module |
| `MRI_MAX_PROPERTIES` | 32 | Maximum LTL properties per module |

## Target Implementations

| Target | File | Status |
|--------|------|--------|
| R-SPU ISA Simulator | `src/emit/rspu_sim/` | Reference implementation |
| RISC-V | `src/emit/riscv.rs` | This campaign |
| ARM Thumb-2 | `src/emit/arm.rs` | This campaign |
| WASM | `crates/mirr-wasm/` | This campaign |

## Cross-Target Guarantee

Given the same MIRR module and the same input sequence, all MRI-compliant
targets produce the identical output sequence. This is guaranteed by:

1. **Sub-Turing proof (MEGA-14):** Every MIRR module is a finite-state
   transducer with identical behavior on any execution substrate
2. **R-SPU golden model:** The R-SPU ISA simulator is the reference
   implementation that all other targets must match
3. **Bounded execution:** No target can introduce unbounded computation
   because MRI enforces MAX_TICKS