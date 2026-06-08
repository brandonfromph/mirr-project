#![forbid(unsafe_code)]
use mirrc::mirr_driver;
use mirrc::mirr_driver::collect_tokens_from_pushes;
use mirrc::mirr_executor;

#[test]
fn interpreter_parity_basic() {
    // These test cases use only integer literals and two-char operators, which
    // are the token classes that both the MIRR interpreter (running lexer.mirr)
    // and the direct emulator (drive_lexer_from_bytes) handle identically.
    //
    // Note: keyword recognition (e.g. "true", "module") is not yet fully
    // implemented in lexer.mirr — the program currently maps most identifiers
    // to emit_push_ident. Parity for keyword inputs will be verified separately
    // once lexer.mirr is extended to emit keyword-specific push signals.
    let cases: Vec<&[u8]> = vec![b"42 == 100" as &[u8], b"1 != 2", b"1 <= 200 >= 3", b"42"];

    for input in cases {
        let emu = mirr_driver::drive_lexer_from_bytes(input);
        let exec = mirr_executor::drive_lexer_with_interpreter(input);

        assert_eq!(
            emu,
            exec,
            "ObservedPush sequences differ for input: {:?}",
            String::from_utf8_lossy(input)
        );

        let toks_emu = collect_tokens_from_pushes(&emu);
        let toks_exec = collect_tokens_from_pushes(&exec);

        assert_eq!(
            toks_emu,
            toks_exec,
            "Mapped token sequences differ for input: {:?}",
            String::from_utf8_lossy(input)
        );
    }
}
