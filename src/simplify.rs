//! Logic simplification for MIRR boolean expressions

use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};

/// Recursively simplify a boolean expression tree.
pub fn simplify_expr(expr: Expr) -> Expr {
    match expr {
        // Simplify subtrees first
        Expr::Unary { op, operand } => {
            let simp_operand = simplify_expr(*operand);
            match (op, &simp_operand) {
                // Double negation: !!X => X
                (UnaryOp::Not, Expr::Unary { op: UnaryOp::Not, operand }) =>
                    *operand.clone(),
                // Not of a literal: !true => false, !false => true
                (UnaryOp::Not, Expr::Literal(LiteralValue::Bool(b))) =>
                    Expr::Literal(LiteralValue::Bool(!b)),
                _ => Expr::Unary { op, operand: Box::new(simp_operand) },
            }
        }
        Expr::Binary { op, left, right } => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);
            match (op, &left, &right) {
                // AND rules
                (BinaryOp::And, Expr::Literal(LiteralValue::Bool(true)), x)
                    | (BinaryOp::And, x, Expr::Literal(LiteralValue::Bool(true))) => x.clone(),
                (BinaryOp::And, Expr::Literal(LiteralValue::Bool(false)), _)
                    | (BinaryOp::And, _, Expr::Literal(LiteralValue::Bool(false))) => Expr::Literal(LiteralValue::Bool(false)),
                // OR rules
                (BinaryOp::Or, Expr::Literal(LiteralValue::Bool(false)), x)
                    | (BinaryOp::Or, x, Expr::Literal(LiteralValue::Bool(false))) => x.clone(),
                (BinaryOp::Or, Expr::Literal(LiteralValue::Bool(true)), _)
                    | (BinaryOp::Or, _, Expr::Literal(LiteralValue::Bool(true))) => Expr::Literal(LiteralValue::Bool(true)),
                // XOR rules
                (BinaryOp::Xor, x, Expr::Literal(LiteralValue::Bool(false)))
                    | (BinaryOp::Xor, Expr::Literal(LiteralValue::Bool(false)), x) => x.clone(),
                (BinaryOp::Xor, Expr::Literal(LiteralValue::Bool(true)), x)
                    | (BinaryOp::Xor, x, Expr::Literal(LiteralValue::Bool(true))) => Expr::Unary {
                        op: UnaryOp::Not,
                        operand: Box::new(x.clone()),
                    },
                // Default: reconstruct
                _ => Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
            }
        }
        // Literals and signals are already simplified
        _ => expr,
    }
}
