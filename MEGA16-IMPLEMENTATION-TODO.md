# MEGA-16 Implementation TODO

## Wave 1: Foundation Fixes (Parallel)
- [x] Section D: Fix opcode mismatches in compiler_mirr/emitter.mirr
- [x] Section G: Update professional framing in README.md
- [x] Section K: Add E901-E910 error codes to docs/error_codes.md

## Wave 2: New Modules (Parallel)
- [x] Section B: Create crates/lra-cli/src/receipt.rs
- [x] Section C: Create crates/lra-cli/src/legacy.rs
- [x] Section F: Create src/bin/mirr-explain/main.rs
- [x] Section H: Create src/bin/mirr-diff/main.rs

## Wave 3: Integration & Testing
- [x] Section A: Refactor crates/lra-cli/src/main.rs
- [x] Section E: Create tests/bootstrap_parity_tests.rs

## Wave 4: Build System & CI
- [x] Section I: Add bootstrap target to Makefile
- [x] Section J: Add bootstrap parity CI job

## Files Modified
- compiler_mirr/emitter.mirr: Fixed 20 opcode values to match canonical ISA
- README.md: Replaced academic framing with professional compliance language
- docs/error_codes.md: Added E901-E910 certification error range
- crates/lra-cli/src/main.rs: Added compile/receipt commands, legacy module
- src/emit/mod.rs: Made expr_text() public
- Makefile: Added bootstrap/bootstrap-check targets
- .github/workflows/ci.yml: Added bootstrap-parity CI job

## Files Created
- crates/lra-cli/src/receipt.rs: Build certification receipt module
- crates/lra-cli/src/legacy.rs: Deprecated command wrapper
- src/bin/mirr-explain/main.rs: Compilation trace tool
- src/bin/mirr-diff/main.rs: Structural diff tool
- tests/bootstrap_parity_tests.rs: Bootstrap parity tests
