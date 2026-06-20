#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::ast::program::Module;
use crate::ecs::components::{
    ConditionComponent, CyclesComponent, EntityId, KindComponent, PatternCallComponent,
    PatternInstanceComponent, TypeComponent,
};
use crate::ecs::registry::{Registry, COMP_PATTERN_CALL};
use crate::error::MirrError;
use crate::parser::ecs_parser::parse_expression_ecs;

const MAX_EXPANSION_ITERATIONS: usize = 10_000;

/// Resolves a pattern's arguments into a substitution environment.
fn build_signal_env(
    def: &crate::ast::pattern::PatternDef,
    call: &crate::ast::pattern::PatternCall,
) -> Result<HashMap<String, String>, MirrError> {
    if def.params.len() != call.arguments.len() {
        return Err(MirrError::SemanticError {
            message: format!(
                "Pattern '{}' expects {} arguments, but got {}",
                def.name,
                def.params.len(),
                call.arguments.len()
            ),
            span: call.span,
        });
    }

    let mut env = HashMap::new();
    for (param, arg) in def.params.iter().zip(call.arguments.iter()) {
        let arg_str = match arg {
            crate::ast::pattern::PatternArg::SignalRef(s) => s.to_string(),
            crate::ast::pattern::PatternArg::PatternRef(p) => p.to_string(),
            crate::ast::pattern::PatternArg::ConstInt(l) => l.to_string(),
            crate::ast::pattern::PatternArg::ConstBool(b) => b.to_string(),
        };
        env.insert(param.name.clone(), arg_str);
    }
    Ok(env)
}

fn expr_to_string(expr: &crate::ast::expr::Expr) -> String {
    use crate::ast::expr::Expr;
    match expr {
        Expr::Literal(l) => match l {
            crate::ast::types::LiteralValue::Bool(b) => b.to_string(),
            crate::ast::types::LiteralValue::Integer(i) => i.to_string(),
        },
        Expr::Signal(s) => s.clone(),
        Expr::Unary { op, operand } => {
            let op_str = match op {
                crate::ast::types::UnaryOp::Not => "!",
                crate::ast::types::UnaryOp::Negate => "-",
                crate::ast::types::UnaryOp::ReductionOr => "|",
            };
            format!("{}{}", op_str, expr_to_string(operand))
        }
        Expr::Binary { op, left, right } => {
            let op_str = match op {
                crate::ast::types::BinaryOp::Add => "+",
                crate::ast::types::BinaryOp::Sub => "-",
                crate::ast::types::BinaryOp::Mul => "*",
                crate::ast::types::BinaryOp::And => "&&",
                crate::ast::types::BinaryOp::Or => "||",
                crate::ast::types::BinaryOp::BitwiseAnd => "&",
                crate::ast::types::BinaryOp::BitwiseOr => "|",
                crate::ast::types::BinaryOp::Xor => "^",
                crate::ast::types::BinaryOp::Shl => "<<",
                crate::ast::types::BinaryOp::Shr => ">>",
                crate::ast::types::BinaryOp::Eq => "==",
                crate::ast::types::BinaryOp::Ne => "!=",
                crate::ast::types::BinaryOp::Lt => "<",
                crate::ast::types::BinaryOp::Le => "<=",
                crate::ast::types::BinaryOp::Gt => ">",
                crate::ast::types::BinaryOp::Ge => ">=",
            };
            format!("({} {} {})", expr_to_string(left), op_str, expr_to_string(right))
        }
        Expr::Prev { signal, delay } => format!("prev({}, {})", signal, delay),
        Expr::ArrayIndex { array, index } => {
            format!("{}[{}]", expr_to_string(array), expr_to_string(index))
        }
        Expr::FieldAccess { object, field } => format!("{}.{}", expr_to_string(object), field),
        _ => panic!("Unsupported expr in macro expansion"),
    }
}

fn lookup_symbol(registry: &mut Registry, name: &str) -> Option<EntityId> {
    let interner_id = registry.interner.intern(name);
    for i in 0..registry.next_id as usize {
        if let Some(n) = &registry.names[i] {
            if n.0 == interner_id {
                return Some(EntityId(i as u32));
            }
        }
    }
    None
}

/// Injects a flattened AST Module into the ECS Registry.
fn inject_module(
    registry: &mut Registry,
    module: &Module,
    parent_module: EntityId,
    caller: EntityId,
) -> Result<(), MirrError> {
    // Inject Signals
    for sig in &module.signals {
        let kind = crate::ecs::components::EntityKind::SIGNAL(sig.kind);
        let entity = registry.create_entity(&sig.name, KindComponent(kind));
        registry.set_type(entity, TypeComponent::signal(sig.ty.clone()));
        registry.set_module(entity, crate::ecs::components::ModuleComponent(parent_module));
        registry.set_pattern_instance(
            entity,
            PatternInstanceComponent { pattern_name: module.name.clone(), caller },
        );
    }

    // Inject Guards
    for guard in &module.guards {
        let entity = registry.create_entity(&guard.name, KindComponent::GUARD);
        let cond_str = expr_to_string(&guard.condition);
        let cond_ent = parse_expression_ecs(registry, &cond_str)?;
        registry.set_condition(entity, ConditionComponent(cond_ent));
        registry.set_cycle(entity, CyclesComponent(guard.cycles));
        registry.set_module(entity, crate::ecs::components::ModuleComponent(parent_module));
        registry.set_pattern_instance(
            entity,
            PatternInstanceComponent { pattern_name: module.name.clone(), caller },
        );
    }

    // Inject Reflexes
    for reflex in &module.reflexes {
        let entity = registry.create_entity(&reflex.name, KindComponent::REFLEX);
        let mut guard_ents = Vec::new();
        for g in &reflex.guard_names {
            if let Some(gid) = lookup_symbol(registry, g) {
                guard_ents.push(gid);
            } else if let Some(gid) = lookup_symbol(registry, &format!("{}::{}", module.name, g)) {
                guard_ents.push(gid);
            }
        }

        let mut assign_ents = Vec::new();
        for assign in &reflex.assignments {
            let assign_ent = registry
                .create_entity(&format!("{}_assign", reflex.name), KindComponent::ASSIGNMENT);
            let target_str = &assign.target;
            let target_id = lookup_symbol(registry, target_str).unwrap_or(EntityId(0));
            let val_ent = parse_expression_ecs(registry, &expr_to_string(&assign.value))?;
            registry.set_assignment(
                assign_ent,
                crate::ecs::components::AssignmentComponent { target: target_id, value: val_ent, target_index: None },
            );
            assign_ents.push(assign_ent);
        }

        registry.set_reflex(
            entity,
            crate::ecs::components::ReflexComponent {
                guards: guard_ents,
                assignments: assign_ents,
                origin: reflex.origin.clone(),
            },
        );
        registry.set_module(entity, crate::ecs::components::ModuleComponent(parent_module));
        registry.set_pattern_instance(
            entity,
            PatternInstanceComponent { pattern_name: module.name.clone(), caller },
        );
    }

    // Inject pattern calls
    for call in &module.pattern_calls {
        let call_id = registry.next_id();
        let entity = registry.create_entity(
            &format!("{}_call_{}", call.pattern_name, call_id.0),
            KindComponent::PATTERN_CALL,
        );
        registry.set_module(entity, crate::ecs::components::ModuleComponent(parent_module));
        registry.set_pattern_call(entity, PatternCallComponent(call.clone()));
    }

    Ok(())
}

/// Main ECS entry point for Pattern Expansion.
pub fn expand_patterns(registry: &mut Registry) -> Result<(), MirrError> {
    let mut work_queue: Vec<EntityId> =
        registry.entities_with_components(COMP_PATTERN_CALL).collect();
    let mut iterations = 0;

    while let Some(call_entity) = work_queue.pop() {
        iterations += 1;
        if iterations > MAX_EXPANSION_ITERATIONS {
            return Err(MirrError::SemanticError {
                message: format!(
                    "Pattern expansion exceeded iteration limit of {}.",
                    MAX_EXPANSION_ITERATIONS
                ),
                span: None,
            });
        }

        // Must pop component so we can borrow registry
        let call_comp = registry.pattern_calls[call_entity.0 as usize]
            .clone()
            .expect("PatternCall component missing");

        let parent_module =
            registry.modules[call_entity.0 as usize].map(|m| m.0).unwrap_or(EntityId(0));

        // Locate PatternDef
        // Locate PatternDef
        let def_comp = match registry.get_entity_by_name(&call_comp.0.pattern_name) {
            Some(id) => {
                println!("DEBUG: Found pattern entity {} for '{}'", id.0, call_comp.0.pattern_name);
                if let Some(comp) = &registry.pattern_defs[id.0 as usize] {
                    comp.clone()
                } else {
                    return Err(MirrError::SemanticError {
                        message: format!("Entity '{}' is not a pattern", call_comp.0.pattern_name),
                        span: call_comp.0.span,
                    });
                }
            }
            None => {
                println!(
                    "DEBUG: Failed to find pattern '{}'. Available symbols in registry:",
                    call_comp.0.pattern_name
                );
                for (sym, ent) in registry.get_symbol_table() {
                    if sym.contains("noc_l1") {
                        println!("DEBUG:   sym: '{}' -> ent: {}", sym, ent.0);
                    }
                }
                return Err(MirrError::SemanticError {
                    message: format!("Pattern '{}' not found", call_comp.0.pattern_name),
                    span: call_comp.0.span,
                });
            }
        };

        if def_comp.0.is_extern {
            registry.extern_instantiations.push(call_entity);
            // DO NOT unset pattern call, so it remains for Verilog emission
            continue;
        }

        // Create environment mapping
        let signal_env = build_signal_env(&def_comp.0, &call_comp.0)?;

        // Expand using the robust ast_expand
        let mut temp_module = Module {
            name: def_comp.0.name.clone(),
            signals: vec![],
            guards: vec![],
            reflexes: vec![],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };

        let prefix = format!("{}_call_{}", def_comp.0.name.replace("::", "_"), call_entity.0);
        let mut param_names = std::collections::HashSet::new();
        for p in &def_comp.0.params {
            param_names.insert(p.name.clone());
        }

        let mut fragment = def_comp.0.body.clone();
        let names = crate::expand::rename::collect_fragment_names(&fragment);
        crate::expand::rename::apply_name_prefixing(&mut fragment, &prefix, &names, &param_names);
        crate::expand::rename::set_origin_tags(&mut fragment, &prefix);

        crate::expand::ast_expand::expand_statements_inplace(
            &mut temp_module,
            fragment.statements,
            HashMap::new(),
            signal_env,
            Some(prefix),
        )?;

        // Inject expanded primitives into Registry
        inject_module(registry, &temp_module, parent_module, call_entity)?;

        // Unset the pattern call so it's not processed again
        registry.unset_pattern_call(call_entity);

        // Add any newly spawned pattern calls to the queue
        for new_call_id in registry.entities_with_components(COMP_PATTERN_CALL) {
            if !work_queue.contains(&new_call_id)
                && !registry.extern_instantiations.contains(&new_call_id)
            {
                work_queue.push(new_call_id);
            }
        }
    }

    Ok(())
}
