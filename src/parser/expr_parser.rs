//! Expression parser for MIRR guard conditions and reflex RHS.
//!
//! Recursive-descent parser producing `Expr` AST nodes from token streams.
//! Supports comparisons, arithmetic, boolean operators, and parenthesized groups.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::types::BinaryOp;
use crate::ast::types::{LiteralValue, UnaryOp};
use crate::error::MirrError;
use crate::lexer::tokenize_expr;
use crate::lexer::tokenizer::Token;

/// Binding power (left, right) for binary operators.
/// Higher number = tighter binding.
fn infix_binding_power(op: &BinaryOp) -> (u8, u8) {
    match op {
        BinaryOp::Or => (2, 3),
        BinaryOp::And => (4, 5),
        BinaryOp::BitwiseOr => (6, 7),
        BinaryOp::BitwiseAnd => (8, 9),
        BinaryOp::Xor => (10, 11),
        BinaryOp::Eq | BinaryOp::Ne => (12, 13),
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => (14, 15),
        BinaryOp::Shl | BinaryOp::Shr => (16, 17),
        BinaryOp::Add | BinaryOp::Sub => (18, 19),
        BinaryOp::Mul => (20, 21),
    }
}

const MAX_EXPR_DEPTH: usize = 128;

/// Map a token to a binary operator, if applicable.
#[inline]
fn token_to_binop(tok: &Token) -> Option<BinaryOp> {
    match tok {
        Token::AmpAmp => Some(BinaryOp::And),
        Token::Amp => Some(BinaryOp::BitwiseAnd),
        Token::PipePipe => Some(BinaryOp::Or),
        Token::Pipe => Some(BinaryOp::BitwiseOr),
        Token::Caret => Some(BinaryOp::Xor),
        Token::EqEq => Some(BinaryOp::Eq),
        Token::BangEq => Some(BinaryOp::Ne),
        Token::Lt => Some(BinaryOp::Lt),
        Token::Le => Some(BinaryOp::Le),
        Token::Gt => Some(BinaryOp::Gt),
        Token::Ge => Some(BinaryOp::Ge),
        Token::LtLt => Some(BinaryOp::Shl),
        Token::GtGt => Some(BinaryOp::Shr),
        Token::Plus => Some(BinaryOp::Add),
        Token::Minus => Some(BinaryOp::Sub),
        Token::Star => Some(BinaryOp::Mul),
        _ => None,
    }
}

/// Parser state: a token stream with a position cursor.
struct ExprParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl ExprParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Parse a complete expression consuming all tokens.
    fn parse_full(&mut self) -> Result<Expr, MirrError> {
        if self.tokens.is_empty() {
            return Err(MirrError::parse_error(format!(
                "{} Empty expression.",
                crate::error_codes::ec(170)
            )));
        }

        // Early validation: check for balanced parentheses
        if !self.has_balanced_parens() {
            return Err(MirrError::parse_error(format!(
                "{} Unbalanced parentheses in expression.",
                crate::error_codes::ec(171)
            )));
        }

        let expr = self.parse_expr(0, 0)?;
        if !self.at_end() {
            return Err(MirrError::parse_error(format!(
                "{} Unexpected token in expression: {:?}",
                crate::error_codes::ec(176),
                self.peek()
            )));
        }
        Ok(expr)
    }

    /// Check if parentheses are balanced in the token stream.
    fn has_balanced_parens(&self) -> bool {
        let mut depth = 0;
        for token in &self.tokens {
            match token {
                Token::LParen => {
                    depth += 1;
                    if depth > MAX_EXPR_DEPTH {
                        return false; // Prevent stack overflow
                    }
                }
                Token::RParen => {
                    if depth == 0 {
                        return false; // Unmatched closing paren
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        depth == 0
    }

    /// Pratt parser: parse expression with given minimum binding power.
    fn parse_expr(&mut self, min_bp: u8, depth: usize) -> Result<Expr, MirrError> {
        if depth > MAX_EXPR_DEPTH {
            return Err(MirrError::parse_error(format!(
                "{} Expression depth exceeds limit of {}.",
                crate::error_codes::ec(172),
                MAX_EXPR_DEPTH
            )));
        }

        // Parse prefix / atom.
        let mut lhs = self.parse_prefix(depth)?;

        // Parse indexed and field access with higher precedence than binary ops.
        loop {
            if let Some(Token::LBracket) = self.peek() {
                self.advance();
                let index_expr = self.parse_expr(0, depth + 1)?;
                match self.advance() {
                    Some(Token::RBracket) => {
                        lhs =
                            Expr::ArrayIndex { array: Box::new(lhs), index: Box::new(index_expr) };
                        continue;
                    }
                    _ => {
                        return Err(MirrError::parse_error(format!(
                            "{} Expected closing ']' in array index.",
                            crate::error_codes::ec(178)
                        )));
                    }
                }
            }

            if let Some(Token::Dot) = self.peek() {
                self.advance();
                match self.advance() {
                    Some(Token::Ident(field)) => {
                        lhs = Expr::FieldAccess { object: Box::new(lhs), field: field.clone() };
                        continue;
                    }
                    _ => {
                        return Err(MirrError::parse_error(format!(
                            "{} Expected field name after '.'.",
                            crate::error_codes::ec(179)
                        )));
                    }
                }
            }

            // Parse infix operators with sufficient binding power.
            if let Some(tok) = self.peek() {
                let op = token_to_binop(tok);
                let Some(op) = op else {
                    break; // Not an infix operator; stop.
                };

                let (left_bp, right_bp) = infix_binding_power(&op);
                if left_bp < min_bp {
                    break;
                }

                // Consume the operator token.
                self.advance();

                let rhs = self.parse_expr(right_bp, depth + 1)?;
                lhs = Expr::Binary { op, left: Box::new(lhs), right: Box::new(rhs) };
                continue;
            }
            break;
        }

        Ok(lhs)
    }

    /// Parse a prefix expression or atom.
    fn parse_prefix(&mut self, depth: usize) -> Result<Expr, MirrError> {
        if depth > MAX_EXPR_DEPTH {
            return Err(MirrError::parse_error(format!(
                "{} Expression depth exceeds limit of {}.",
                crate::error_codes::ec(172),
                MAX_EXPR_DEPTH
            )));
        }

        let tok = self.advance().cloned().ok_or_else(|| {
            MirrError::parse_error(format!(
                "{} Unexpected end of expression.",
                crate::error_codes::ec(173)
            ))
        })?;

        match tok {
            Token::True => Ok(Expr::Literal(LiteralValue::Bool(true))),
            Token::False => Ok(Expr::Literal(LiteralValue::Bool(false))),
            Token::Integer(n) => Ok(Expr::Literal(LiteralValue::Integer(n))),
            Token::Ident(name) => {
                let mut full_name = name;
                while let Some(Token::ColonColon) = self.peek() {
                    self.advance();
                    if let Some(Token::Ident(next)) = self.advance() {
                        full_name.push_str("::");
                        full_name.push_str(next);
                    } else {
                        return Err(MirrError::parse_error(format!(
                            "{} Expected identifier after '::'.",
                            crate::error_codes::ec(181)
                        )));
                    }
                }

                if let Some(Token::LParen) = self.peek() {
                    self.advance(); // consume '('
                    let mut args: Vec<Expr> = Vec::with_capacity(2);

                    if let Some(Token::RParen) = self.peek() {
                        self.advance();
                    } else {
                        loop {
                            args.push(self.parse_expr(0, depth + 1)?);
                            match self.peek() {
                                Some(Token::Comma) => {
                                    self.advance();
                                }
                                Some(Token::RParen) => {
                                    self.advance();
                                    break;
                                }
                                _ => {
                                    return Err(MirrError::parse_error(format!(
                                        "{} Expected ',' or ')' in function argument list.",
                                        crate::error_codes::ec(182)
                                    )));
                                }
                            }
                        }
                    }

                    match full_name.as_str() {
                        "prev" => {
                            if args.len() != 2 {
                                return Err(MirrError::parse_error(format!(
                                    "{} prev() expects exactly 2 arguments.",
                                    crate::error_codes::ec(183)
                                )));
                            }

                            let signal = match &args[0] {
                                Expr::Signal(s) => s.clone(),
                                _ => {
                                    return Err(MirrError::parse_error(format!(
                                        "{} prev() first argument must be a signal identifier.",
                                        crate::error_codes::ec(184)
                                    )));
                                }
                            };

                            let delay = match &args[1] {
                                Expr::Literal(LiteralValue::Integer(v)) => *v,
                                _ => {
                                    return Err(MirrError::parse_error(format!(
                                        "{} prev() delay must be an integer literal.",
                                        crate::error_codes::ec(185)
                                    )));
                                }
                            };

                            Ok(Expr::Prev { signal, delay })
                        }
                        "types::extract_data" => {
                            if args.len() != 1 {
                                return Err(MirrError::parse_error(
                                    "extract_data expects 1 argument".to_string(),
                                ));
                            }
                            // Lower to ((arg) & 0xFFFFFFFF)
                            Ok(Expr::Binary {
                                op: BinaryOp::BitwiseAnd,
                                left: Box::new(args[0].clone()),
                                right: Box::new(Expr::Literal(LiteralValue::Integer(4294967295))),
                            })
                        }
                        "types::extract_tag" => {
                            if args.len() != 1 {
                                return Err(MirrError::parse_error(
                                    "extract_tag expects 1 argument".to_string(),
                                ));
                            }
                            // Lower to (((arg) >> 32) & 0xF)
                            let shr = Expr::Binary {
                                op: BinaryOp::Shr,
                                left: Box::new(args[0].clone()),
                                right: Box::new(Expr::Literal(LiteralValue::Integer(32))),
                            };
                            Ok(Expr::Binary {
                                op: BinaryOp::BitwiseAnd,
                                left: Box::new(shr),
                                right: Box::new(Expr::Literal(LiteralValue::Integer(15))),
                            })
                        }
                        "types::extract_provenance" => {
                            if args.len() != 1 {
                                return Err(MirrError::parse_error(
                                    "extract_provenance expects 1 argument".to_string(),
                                ));
                            }
                            // Lower to (((arg) >> 36) & 0xF)
                            let shr = Expr::Binary {
                                op: BinaryOp::Shr,
                                left: Box::new(args[0].clone()),
                                right: Box::new(Expr::Literal(LiteralValue::Integer(36))),
                            };
                            Ok(Expr::Binary {
                                op: BinaryOp::BitwiseAnd,
                                left: Box::new(shr),
                                right: Box::new(Expr::Literal(LiteralValue::Integer(15))),
                            })
                        }
                        "types::pack_word" => {
                            if args.len() != 3 {
                                return Err(MirrError::parse_error(
                                    "pack_word expects 3 arguments".to_string(),
                                ));
                            }
                            // ((((P) << 36) | ((T) << 32)) | (D))
                            let d = args[0].clone();
                            let t = args[1].clone();
                            let p = args[2].clone();

                            let p_shl = Expr::Binary {
                                op: BinaryOp::Shl,
                                left: Box::new(p),
                                right: Box::new(Expr::Literal(LiteralValue::Integer(36))),
                            };
                            let t_shl = Expr::Binary {
                                op: BinaryOp::Shl,
                                left: Box::new(t),
                                right: Box::new(Expr::Literal(LiteralValue::Integer(32))),
                            };
                            let combined_pt = Expr::Binary {
                                op: BinaryOp::BitwiseOr,
                                left: Box::new(p_shl),
                                right: Box::new(t_shl),
                            };
                            Ok(Expr::Binary {
                                op: BinaryOp::BitwiseOr,
                                left: Box::new(combined_pt),
                                right: Box::new(d),
                            })
                        }
                        _ => Err(MirrError::parse_error(format!(
                            "{} Unknown function call '{}'.",
                            crate::error_codes::ec(186),
                            full_name
                        ))),
                    }
                } else if let Some(Token::LBrace) = self.peek() {
                    self.advance(); // consume '{'
                    let mut fields: Vec<(String, Expr)> = Vec::new();
                    loop {
                        if let Some(Token::RBrace) = self.peek() {
                            self.advance();
                            break;
                        }

                        let field_name = match self.advance() {
                            Some(Token::Ident(n)) => n.clone(),
                            Some(t) => {
                                return Err(MirrError::parse_error(format!(
                                    "{} Unexpected token in struct literal field name: {t:?}",
                                    crate::error_codes::ec(181)
                                )));
                            }
                            None => {
                                return Err(MirrError::parse_error(format!(
                                    "{} Unexpected end of struct literal.",
                                    crate::error_codes::ec(181)
                                )));
                            }
                        };

                        match self.advance() {
                            Some(Token::Colon) => {}
                            Some(t) => {
                                return Err(MirrError::parse_error(format!("{} Expected ':' after field name in struct literal, found: {t:?}", crate::error_codes::ec(181))));
                            }
                            None => {
                                return Err(MirrError::parse_error(format!(
                                    "{} Unexpected end of struct literal after field name.",
                                    crate::error_codes::ec(181)
                                )));
                            }
                        }

                        let value = self.parse_expr(0, depth + 1)?;
                        fields.push((field_name, value));

                        match self.peek() {
                            Some(Token::Comma) => {
                                self.advance();
                                continue;
                            }
                            Some(Token::RBrace) => {
                                self.advance();
                                break;
                            }
                            Some(t) => {
                                return Err(MirrError::parse_error(format!(
                                    "{} Expected ',' or '}}' in struct literal, found: {t:?}",
                                    crate::error_codes::ec(181)
                                )));
                            }
                            None => {
                                return Err(MirrError::parse_error(format!(
                                    "{} Unexpected end of struct literal.",
                                    crate::error_codes::ec(181)
                                )));
                            }
                        }
                    }
                    Ok(Expr::StructLiteral { name: full_name, fields })
                } else {
                    Ok(Expr::Signal(full_name))
                }
            }
            Token::Bang => {
                // Unary not: bind tighter than any binary operator.
                let operand = self.parse_expr(100, depth + 1)?;
                Ok(Expr::Unary { op: UnaryOp::Not, operand: Box::new(operand) })
            }
            Token::Minus => {
                // Unary negate: bind tighter than any binary operator.
                let operand = self.parse_expr(100, depth + 1)?;
                Ok(Expr::Unary { op: UnaryOp::Negate, operand: Box::new(operand) })
            }
            Token::Pipe => {
                // Vector reduction OR: binds tightly.
                let operand = self.parse_expr(100, depth + 1)?;
                Ok(Expr::Unary { op: UnaryOp::ReductionOr, operand: Box::new(operand) })
            }
            Token::LParen => {
                let inner = self.parse_expr(0, depth + 1)?;
                match self.advance() {
                    Some(Token::RParen) => Ok(inner),
                    _ => Err(MirrError::parse_error(format!(
                        "{} Expected closing ')' in expression.",
                        crate::error_codes::ec(175)
                    ))),
                }
            }
            Token::LBracket => {
                // Array literal parser
                let mut elements = Vec::new();
                if let Some(Token::RBracket) = self.peek() {
                    self.advance();
                    return Ok(Expr::ArrayLiteral(elements));
                }
                loop {
                    let elem = self.parse_expr(0, depth + 1)?;
                    elements.push(elem);
                    match self.advance() {
                        Some(Token::Comma) => continue,
                        Some(Token::RBracket) => break,
                        _ => {
                            return Err(MirrError::parse_error(format!(
                                "{} Expected ',' or ']' in array literal.",
                                crate::error_codes::ec(177)
                            )))
                        }
                    }
                }
                Ok(Expr::ArrayLiteral(elements))
            }
            other => Err(MirrError::parse_error(format!(
                "{} Unexpected token at start of expression: {:?}",
                crate::error_codes::ec(174),
                other
            ))),
        }
    }
}

/// Parse a string into an expression AST.
pub fn parse_expression(input: &str) -> Result<Expr, MirrError> {
    let tokens = tokenize_expr(input)?;
    let mut parser = ExprParser::new(tokens);
    parser.parse_full()
}
