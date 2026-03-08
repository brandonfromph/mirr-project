//! Expression parser for MIRR guard conditions and reflex RHS.
//!
//! Recursive-descent parser producing `Expr` AST nodes from token streams.
//! Supports comparisons, arithmetic, boolean operators, and parenthesized groups.

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
        BinaryOp::Xor => (6, 7),
        BinaryOp::Eq | BinaryOp::Ne => (8, 9),
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => (10, 11),
        BinaryOp::Shl | BinaryOp::Shr => (12, 13),
        BinaryOp::Add | BinaryOp::Sub => (14, 15),
        BinaryOp::Mul => (16, 17),
    }
}

const MAX_EXPR_DEPTH: usize = 128;

/// Map a token to a binary operator, if applicable.
/// NASA-style optimization: use lookup table for O(1) matching.
#[inline]
fn token_to_binop(tok: &Token) -> Option<BinaryOp> {
    match tok {
        Token::AmpAmp => Some(BinaryOp::And),
        Token::PipePipe => Some(BinaryOp::Or),
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

    /// Get the current token without advancing.
    /// NASA-style optimization: bounds-checked access with early return.
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Parse a complete expression consuming all tokens.
    /// NASA-style optimization: validate input size before parsing.
    fn parse_full(&mut self) -> Result<Expr, MirrError> {
        if self.tokens.is_empty() {
            return Err(MirrError::new("Empty expression."));
        }

        // Early validation: check for balanced parentheses
        if !self.has_balanced_parens() {
            return Err(MirrError::new("Unbalanced parentheses in expression."));
        }

        let expr = self.parse_expr(0, 0)?;
        if !self.at_end() {
            return Err(MirrError::new(format!(
                "Unexpected token in expression: {:?}",
                self.peek()
            )));
        }
        Ok(expr)
    }

    /// Check if parentheses are balanced in the token stream.
    /// NASA-style optimization: single-pass validation before parsing.
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
    /// NASA-style optimization: inline critical functions and reduce allocations.
    fn parse_expr(&mut self, min_bp: u8, depth: usize) -> Result<Expr, MirrError> {
        if depth > MAX_EXPR_DEPTH {
            return Err(MirrError::new(format!(
                "Expression depth exceeds limit of {}.",
                MAX_EXPR_DEPTH
            )));
        }

        // Parse prefix / atom.
        let mut lhs = self.parse_prefix(depth)?;

        // Parse infix operators with sufficient binding power.
        // NASA-style: minimize function calls in hot loop
        while let Some(tok) = self.current_token() {
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
        }

        Ok(lhs)
    }

    /// Parse a prefix expression or atom.
    /// NASA-style optimization: reduce cloning and use direct matching.
    fn parse_prefix(&mut self, depth: usize) -> Result<Expr, MirrError> {
        if depth > MAX_EXPR_DEPTH {
            return Err(MirrError::new(format!(
                "Expression depth exceeds limit of {}.",
                MAX_EXPR_DEPTH
            )));
        }

        let tok = self.advance().ok_or_else(|| MirrError::new("Unexpected end of expression."))?;

        match tok {
            Token::True => Ok(Expr::Literal(LiteralValue::Bool(true))),
            Token::False => Ok(Expr::Literal(LiteralValue::Bool(false))),
            Token::Integer(n) => Ok(Expr::Literal(LiteralValue::Integer(*n))),
            Token::Ident(name) => Ok(Expr::Signal(name.to_string())),
            Token::Bang => {
                // Unary not: bind tighter than any binary operator.
                let operand = self.parse_prefix(depth + 1)?;
                Ok(Expr::Unary { op: UnaryOp::Not, operand: Box::new(operand) })
            }
            Token::Minus => {
                // Unary negate: bind tighter than any binary operator.
                let operand = self.parse_prefix(depth + 1)?;
                Ok(Expr::Unary { op: UnaryOp::Negate, operand: Box::new(operand) })
            }
            Token::LParen => {
                let inner = self.parse_expr(0, depth + 1)?;
                match self.advance() {
                    Some(Token::RParen) => Ok(inner),
                    _ => Err(MirrError::new("Expected closing ')' in expression.")),
                }
            }
            other => {
                Err(MirrError::new(format!("Unexpected token at start of expression: {:?}", other)))
            }
        }
    }
}

/// Parse a string into an expression AST.
pub fn parse_expression(input: &str) -> Result<Expr, MirrError> {
    let tokens = tokenize_expr(input)?;
    let mut parser = ExprParser::new(tokens);
    parser.parse_full()
}
