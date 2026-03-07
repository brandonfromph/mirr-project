//! Quick binary to exercise the MIRR lexer interpreter and show that the
//! lexer written in the MIRR language actually executes and produces tokens.

use nasa_rust_project::mirr_driver::collect_tokens_from_pushes;
use nasa_rust_project::mirr_executor;

fn main() {
    // simple test string; not literally "Hello world" but demonstrates that the
    // MIRR lexer can process text and emit recognizable tokens.
    let input = b"when foo == 42";

    let pushes = mirr_executor::drive_lexer_with_interpreter(input);
    println!("observed pushes from MIRR lexer interpreter: {:#?}", pushes);

    let tokens = collect_tokens_from_pushes(&pushes);
    println!("mapped token stream: {:#?}", tokens);
}
