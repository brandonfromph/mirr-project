# mirr_core — MIRR Standard Library

Core library primitives written in MIRR for the self-hosting compiler pipeline.

## Modules

| File | Description |
|------|-------------|
| `diagnostics.mirr` | Diagnostic message formatting and error reporting |
| `fixed_map.mirr` | Fixed-capacity key-value map (bounded, no heap allocation) |
| `str.mirr` | String utilities for MIRR source processing |
| `token_buffer.mirr` | Bounded token ring buffer for the lexer pipeline |

## Design constraints

All modules follow NASA Power-of-10 rules:
- No dynamic allocation
- Bounded iteration
- Fixed-capacity data structures
