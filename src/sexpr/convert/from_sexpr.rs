#![allow(dead_code)]
use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use crate::ecs::components::{
    ConditionComponent, CyclesComponent, KindComponent, ReflexComponent, TypeComponent,
};
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
    registry: &mut Registry,
    expr: &SExpr,
) -> Result<crate::ecs::EntityId, MirrError> {
    let items = match expr {
        SExpr::List(list) => list,
        _ => return Err(sexpr_err("Expected program list".to_string())),
    };
    expect_head(items, "program")?;
    if items.len() < 3 {
        return Err(sexpr_err("Expected program to have patterns and module".to_string()));
    }

    let _patterns_expr = &items[1]; // Parse patterns later if needed
    let module_expr = &items[2];

    parse_module(registry, module_expr)
}

fn parse_module(registry: &mut Registry, expr: &SExpr) -> Result<crate::ecs::EntityId, MirrError> {
    let items = match expr {
        SExpr::List(list) => list,
        _ => return Err(sexpr_err("Expected module list".to_string())),
    };
    expect_head(items, "module")?;
    if items.len() < 8 {
        return Err(sexpr_err("Module missing children".to_string()));
    }

    let name = items[1].as_str_val().unwrap_or("<unnamed>");
    let module_id = registry.create_entity(name, KindComponent::MODULE);

    parse_signals(registry, module_id, &items[2])?;
    parse_guards(registry, module_id, &items[3])?;
    parse_reflexes(registry, module_id, &items[4])?;
    // Properties, pattern calls, and pattern origins skipped for MVP roundtrip

    Ok(module_id)
}

fn parse_signals(
    registry: &mut Registry,
    module_id: crate::ecs::EntityId,
    expr: &SExpr,
) -> Result<(), MirrError> {
    let items = match expr {
        SExpr::List(list) => list,
        _ => return Err(sexpr_err("Expected signals list".to_string())),
    };
    expect_head(items, "signals")?;

    for sig_expr in &items[1..] {
        let sig_list = match sig_expr {
            SExpr::List(list) => list,
            _ => continue,
        };
        expect_head(sig_list, "signal")?;

        let name = sig_list[1].as_str_val().unwrap_or("").to_string();

        let kind_str = sig_list[2].as_symbol().unwrap_or("internal");
        let kind = match kind_str {
            "input" => SignalKind::Input,
            "output" => SignalKind::Output,
            "internal" => SignalKind::Internal,
            _ => SignalKind::Internal,
        };

        let ty_core = parse_signal_type(&sig_list[3])?;

        let sig_id = registry.create_signal(
            name,
            KindComponent(crate::ecs::components::EntityKind::SIGNAL(kind)),
            TypeComponent(ExtendedType::from_core(ty_core)),
        );
        registry.set_parent(sig_id, module_id);
    }

    Ok(())
}

fn parse_signal_type(expr: &SExpr) -> Result<SignalType, MirrError> {
    match expr {
        SExpr::Symbol(s) => match s.as_str() {
            "bool" => Ok(SignalType::Bool),
            _ => Err(sexpr_err(format!("Unknown signal type symbol: {}", s))),
        },
        SExpr::List(list) => {
            if list.is_empty() {
                return Err(sexpr_err("Empty type list".to_string()));
            }
            let sym = list[0].as_symbol().unwrap_or("");
            match sym {
                "unsigned" => {
                    let w = list[1].as_integer().unwrap_or(32) as u32;
                    Ok(SignalType::Unsigned(w))
                }
                "signed" => {
                    let w = list[1].as_integer().unwrap_or(32) as u32;
                    Ok(SignalType::Signed(w))
                }
                _ => Err(sexpr_err(format!("Unknown signal type list: {}", sym))),
            }
        }
        _ => Err(sexpr_err("Invalid type expression".to_string())),
    }
}

fn parse_guards(
    registry: &mut Registry,
    module_id: crate::ecs::EntityId,
    expr: &SExpr,
) -> Result<(), MirrError> {
    let items = match expr {
        SExpr::List(list) => list,
        _ => return Err(sexpr_err("Expected guards list".to_string())),
    };
    expect_head(items, "guards")?;

    for g_expr in &items[1..] {
        let g_list = match g_expr {
            SExpr::List(list) => list,
            _ => continue,
        };
        expect_head(g_list, "guard")?;

        let name = g_list[1].as_str_val().unwrap_or("").to_string();
        let cond_expr = parse_expr(&g_list[2])?;
        let cycles = g_list.get(3).and_then(|e| e.as_integer()).unwrap_or(1);

        let guard_id = registry.create_entity(&name, KindComponent::GUARD);
        let cond_id = registry.ingest_expr(&cond_expr)?;

        if registry.conditions.len() <= guard_id.0 as usize {
            registry.conditions.resize(guard_id.0 as usize + 1, None);
        }
        registry.conditions[guard_id.0 as usize] = Some(ConditionComponent(cond_id));

        if registry.cycles.len() <= guard_id.0 as usize {
            registry.cycles.resize(guard_id.0 as usize + 1, None);
        }
        registry.cycles[guard_id.0 as usize] = Some(CyclesComponent(cycles));
        registry.set_parent(guard_id, module_id);
    }

    Ok(())
}

fn parse_reflexes(
    registry: &mut Registry,
    module_id: crate::ecs::EntityId,
    expr: &SExpr,
) -> Result<(), MirrError> {
    let items = match expr {
        SExpr::List(list) => list,
        _ => return Err(sexpr_err("Expected reflexes list".to_string())),
    };
    expect_head(items, "reflexes")?;

    for r_expr in &items[1..] {
        let r_list = match r_expr {
            SExpr::List(list) => list,
            _ => continue,
        };
        expect_head(r_list, "reflex")?;

        let name = r_list[1].as_str_val().unwrap_or("").to_string();
        let reflex_id = registry.create_entity(&name, KindComponent::REFLEX);

        let on_list = match &r_list[2] {
            SExpr::List(l) => l,
            _ => return Err(sexpr_err("Expected on list".to_string())),
        };
        expect_head(on_list, "on")?;

        let mut guard_ids = Vec::new();
        for g_expr in &on_list[1..] {
            let g_name = g_expr.as_str_val().unwrap_or("").to_string();
            if let Some(id) = registry.get_entity_by_name(&g_name) {
                guard_ids.push(id);
            }
        }

        let mut assignment_ids = Vec::new();
        for a_expr in &r_list[3..] {
            let a_list = match a_expr {
                SExpr::List(list) => list,
                _ => continue,
            };
            expect_head(a_list, "assign")?;
            let (target_name, target_index) = match &a_list[1] {
                SExpr::List(idx_list) if idx_list.len() == 3 && idx_list[0].as_symbol() == Some("index") => {
                    let name = idx_list[1].as_str_val().unwrap_or("").to_string();
                    let idx = idx_list[2].as_integer().map(|i| i as usize);
                    (name, idx)
                }
                _ => (a_list[1].as_str_val().unwrap_or("").to_string(), None),
            };
            let target_id =
                registry.get_entity_by_name(&target_name).unwrap_or(crate::ecs::EntityId(0));

            let val_expr = parse_expr(&a_list[2])?;
            let val_id = registry.ingest_expr(&val_expr)?;

            let assign_id = registry.create_entity("assign", KindComponent::ASSIGNMENT);

            if registry.assignment_comps.len() <= assign_id.0 as usize {
                registry.assignment_comps.resize(assign_id.0 as usize + 1, None);
            }
            registry.assignment_comps[assign_id.0 as usize] =
                Some(crate::ecs::components::AssignmentComponent {
                    target: target_id,
                    value: val_id,
                    target_index,
                });
            registry.set_parent(assign_id, reflex_id);
            assignment_ids.push(assign_id);
        }

        if registry.reflex_comps.len() <= reflex_id.0 as usize {
            registry.reflex_comps.resize(reflex_id.0 as usize + 1, None);
        }
        registry.reflex_comps[reflex_id.0 as usize] =
            Some(ReflexComponent { guards: guard_ids, assignments: assignment_ids, origin: None });
        registry.set_parent(reflex_id, module_id);
    }

    Ok(())
}

fn parse_expr(expr: &SExpr) -> Result<Expr, MirrError> {
    match expr {
        SExpr::Bool(b) => Ok(Expr::Literal(LiteralValue::Bool(*b))),
        SExpr::Integer(i) => Ok(Expr::Literal(LiteralValue::Integer(*i))),
        SExpr::List(list) => {
            if list.is_empty() {
                return Err(sexpr_err("Empty expr list".to_string()));
            }
            let sym = list[0].as_symbol().unwrap_or("");
            match sym {
                "signal" => {
                    let name = list[1].as_str_val().unwrap_or("").to_string();
                    Ok(Expr::Signal(name))
                }
                ">" => {
                    let left = Box::new(parse_expr(&list[1])?);
                    let right = Box::new(parse_expr(&list[2])?);
                    Ok(Expr::Binary { op: BinaryOp::Gt, left, right })
                }
                // TODO: Add more operators as needed by tests
                _ => Err(sexpr_err(format!("Unknown expr symbol: {}", sym))),
            }
        }
        _ => Err(sexpr_err("Invalid expression".to_string())),
    }
}
