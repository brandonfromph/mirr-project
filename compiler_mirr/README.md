# compiler_mirr — MIRR-in-MIRR compiler port status

Status:
- This directory contains the MIRR implementation of the compiler used for self-hosting.
- Many original MIRR source files were backed up to compiler_mirr/backups/ to allow incremental porting.
- Current active files are minimal, parseable primitives used to unblock the bootstrap and are being ported incrementally.

Porting protocol:
1. Make small, reversible edits and run the top-level bootstrap (build_selfhost.ps1) after each edit.
2. If a file is being replaced, keep the original in compiler_mirr/backups/.
3. Do not introduce unsupported MIRR top-level constructs (functions, consts) without porting them gradually into signal/guard/reflex patterns or using stdlib helpers.
4. Use compiler_mirr/PORTING_STEPS.md to track next steps.

Contact:
- See top-level README.md for overall project notes and how to run the bootstrap.