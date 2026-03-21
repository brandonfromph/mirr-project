# MIRR Safety Standard Library

Pre-written, formally verified patterns for common safety-critical constructs.

## Patterns

| Pattern | File | Description |
|---------|------|-------------|
| TMR Voter | `tmr_voter.mirr` | Triple Modular Redundancy majority voter |
| Watchdog Timer | `watchdog.mirr` | Configurable watchdog timer with heartbeat |
| Sensor Validator | `sensor_valid.mirr` | Range-checking and fault detection |
| Signal Debouncer | `debouncer.mirr` | N-cycle stable signal detector |
| Heartbeat Monitor | `heartbeat.mirr` | Missing periodic signal detector |
| CRC-8 Checksum | `crc8.mirr` | Bounded CRC-8 for data integrity |
| Priority Encoder | `priority_enc.mirr` | Fixed-priority interrupt encoder |
| Majority Gate | `majority.mirr` | N-input majority gate (generalized TMR) |

## Design Principles

1. **Pure MIRR** — uses only Signal/Guard/Reflex
2. **Bounded** — every loop has a fixed upper bound
3. **Self-contained** — no external dependencies
4. **Hardware-synthesizable** — compiles to existing RTL primitives

## Usage

Import patterns using multi-file compilation:

```mirr
import "stdlib/safety/tmr_voter.mirr" as tmr;

module my_system {
    // ... use tmr_voter pattern
}