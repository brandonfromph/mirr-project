#![forbid(unsafe_code)]

use mirrc::lexer::tokenizer::{tokenize_expr, Token};

#[test]
fn tokenize_valid_expression_tokens() {
    let input = "! & | ^ < > = ( ) { } [ ] , : . ; && || << >> <= >= == != :: -> true false and or not + - * 0x1A 123";
    let tokens = tokenize_expr(input).unwrap();
    
    let expected = vec![
        Token::Bang, Token::Amp, Token::Pipe, Token::Caret, Token::Lt, Token::Gt, Token::Eq,
        Token::LParen, Token::RParen, Token::LBrace, Token::RBrace, Token::LBracket, Token::RBracket,
        Token::Comma, Token::Colon, Token::Dot, Token::Semicolon,
        Token::AmpAmp, Token::PipePipe, Token::LtLt, Token::GtGt, Token::Le, Token::Ge, Token::EqEq,
        Token::BangEq, Token::ColonColon, Token::MinusGt,
        Token::True, Token::False, Token::AmpAmp, Token::PipePipe, Token::Bang,
        Token::Plus, Token::Minus, Token::Star, Token::Integer(26), Token::Integer(123)
    ];
    
    assert_eq!(tokens, expected);
}

#[test]
fn tokenize_fails_on_bad_character() {
    let bad_input = "a + @";
    assert!(tokenize_expr(bad_input).is_err());
}

#[test]
fn tokenize_fails_on_bad_hex() {
    // 0xG is 0x and G. 0x fails to parse as hex.
    let bad_hex = "0xG";
    assert!(tokenize_expr(bad_hex).is_err());
}

#[test]
fn tokenize_fails_on_integer_overflow() {
    let bad_int = "99999999999999999999999999999999999";
    assert!(tokenize_expr(bad_int).is_err());
}

#[test]
fn tokenize_unclosed_template_interpolation() {
    let template = "${abc";
    let t_tokens = tokenize_expr(template).unwrap();
    assert_eq!(t_tokens[0], Token::Ident("${abc".to_string()));
}
