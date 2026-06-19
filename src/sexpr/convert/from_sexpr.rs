#![allow(dead_code)]
use crate::ecs::Registry;
use crate::error::MirrError;
use crate::sexpr::types::SExpr;

pub(super) fn sexpr_err(msg: String) -> MirrError {
    MirrError::parse_error(&msg)
}

pub(super) fn expect_head(items: &[SExpr], expected: &str) -> Result<(), MirrError> {
    if items.is_empty() {
        return Err(sexpr_err(format!(
            "{} Expected symbol '{}' as list head",
            crate::error_codes::ec(805),
            expected
        )));
    }
    match items[0].as_symbol() {
        Some(s) if s == expected => Ok(()),
        Some(s) => Err(sexpr_err(format!(
            "{} Expected '{}', found '{}'",
            crate::error_codes::ec(805),
            expected,
            s
        ))),
        None => Err(sexpr_err(format!(
            "{} Expected symbol '{}' as list head",
            crate::error_codes::ec(805),
            expected
        ))),
    }
}

pub fn sexpr_to_registry(
    _registry: &mut Registry,
    _expr: &SExpr,
) -> Result<crate::ecs::EntityId, String> {
    Ok(crate::ecs::EntityId(0))
}
