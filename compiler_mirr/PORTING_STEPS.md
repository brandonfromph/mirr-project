# Porting plan — lexer (concise)

Summary
- We used placeholders to let the Rust reference progress while preserving original full implementations as `.bak` files.
- Backups let us revert or extract logic safely; placeholders unblock the bootstrap so we can port incrementally and re-run quickly.

Stepwise port for compiler_mirr/lexer.mirr
1. Prep
   - Keep compiler_mirr/lexer.mirr.bak untouched.
   - Work in small, compileable increments so the Rust reference can parse every intermediate file.

2. Port order (lexer)
   a. Top-level constants → Add as comments or minimal consts the parser accepts.
   b. Small helper functions (pure intrinsics) → port one or two simple helpers (is_whitespace_byte, is_digit_byte).
      - Run bootstrap after each addition.
   c. classify_ident_kind → port in length-grouped blocks (len==2, len==3, ...) one group at a time.
      - Test by running the Rust reference on a small sample file if possible.
   d. lex_single_char_op / lex_two_char_op → port one operator group at a time.
   e. parse_decimal → port and run unit checks.
   f. lex_source (main loop) → port in phases:
      - whitespace + line-comment skip
      - two-char ops branch
      - single-char ops branch
      - integer literal branch
      - ident/keyword branch (hook to classify_ident_kind)
      - unknown-byte diagnostic
   g. LexResult and token buffer pushes — ensure token_buffer API matches stdlib/mirr_core/token_buffer.mirr; adapt calls to available API.

3. Iteration and verification
   - After each small change: run
```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File build_selfhost.ps1
```
   - Inspect the bootstrap output; if parse fails, revert last change or reduce scope.
   - Use the Rust reference errors to target minimal fixes.

4. Safety and critical-system considerations
   - Placeholders are temporary; they reduce risk of large, destructive edits by saving originals.
   - For critical systems, do not accept placeholders as final: port full, reviewed implementations before relying on the self-hosted compiler.
   - Maintain `.bak` copies and run the project's test suite after porting each module.

5. Next actions I will take if you approve
   - Start porting lexer.mirr.bak: add constants as minimal safe consts, then port is_whitespace_byte + is_digit_byte and re-run bootstrap.
   - Continue with classify_ident_kind group-by-group.

Commands to run locally
- Run bootstrap:
  - powershell -NoProfile -ExecutionPolicy Bypass -File build_selfhost.ps1
- Check for self-hosted binary:
  - powershell Test-Path target\selfhosted_mirr_compiler.exe

Harness wiring required (implementation notes)
- Purpose: map MIRR lexer push requests (emit_push_*) into stdlib token_buffer operations on the host side.
- Token APIs:
  - token_make(kind: u32, start: usize, len: usize, int_value: u64) → Token (defined in stdlib/mirr_core/token_buffer.mirr)
  - token_buffer_push(&mut buf, tok) -> bool
- Signals emitted from compiler_mirr/lexer.mirr:
  - emit_push_integer
  - emit_push_ident
  - emit_push_eq_eq
  - emit_push_excl_eq
  - emit_push_le
  - emit_push_ge
  - emit_push_arrow
  - emit_push_dot_dot
  - emit_push_kw_when, emit_push_kw_bool, emit_push_tok_true, emit_push_kw_else, emit_push_kw_loop, emit_push_kw_enum
- Host wiring responsibilities:
  1. When a push signal is observed asserted for the current byte position:
     - Construct a Token using token_make with the appropriate TOKEN_* kind and position/length/int value.
     - Call token_buffer_push(&mut buf, tok). On false, record a diagnostic (DIAG_LEX_BUFFER_FULL).
  2. Maintain the lexer position and any parsed integer value state on the host/harness side; the MIRR lexer currently only signals token types and length-classes.
  3. Clear/ack semantics: the MIRR module clears emit_push_* signals on each input_tick; host should sample push signals after each tick and perform the corresponding push exactly once per asserted tick.
  4. EOF: host must append EOF token after lexing completes using token_make(TOKEN_EOF, src_len, 0, 0) and push it.
- Suggested integration points:
  - The bootstrap runner currently orchestrates Parse/Validate/TemporalLower; add a small "MIRR runtime harness" module (e.g., src/mirr_runtime.rs or extend src/bootstrap_runner.rs) responsible for:
    * Driving compiler_mirr modules (feeding input bytes → input_byte_* signals)
    * Sampling emit_* and emit_push_* outputs each tick
    * Calling token_make / token_buffer_push as described above
  - Keep mapping code small and well-documented so it can be replaced by a full MIRR interpreter later.
- Testing:
  - Add a unit test that runs the MIRR lexer module over a short source and verifies pushed tokens match the Rust lexer's token_buffer output for the same input.
  - Use existing tests/fixtures for parity checks.

If you want, I can implement the suggested harness changes in Rust next — toggle to Act mode and I'll modify the runner to sample emit_push_* signals and call token_buffer_push.