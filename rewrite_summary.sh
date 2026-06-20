cat src/bin/mirr-compile/summary.rs | sed -n '1,12p' > temp_summary.rs
cat << 'INNER_EOF' >> temp_summary.rs
    let max_entities = registry.active_entities();
    
    let mut signal_count = 0;
    let mut guard_count = 0;
    let mut reflex_count = 0;
    let mut total_wire_bits = 0;
    let mut dff_bits = 0;

    // Detailed cell usage grouping: HashMap<(&'static str, width), count>
    let mut cells: std::collections::HashMap<(&'static str, u32), usize> = std::collections::HashMap::new();

    let get_width = |id: u32| -> u32 {
        if let Some(Some(ty)) = registry.types.get(id as usize) {
            match &ty.0.core {
                mirrc::ast::types::SignalType::Unsigned(w) => *w,
                mirrc::ast::types::SignalType::Signed(w) => *w,
                mirrc::ast::types::SignalType::Bool => 1,
                mirrc::ast::types::SignalType::Array { element, length } => {
                    let mut w = 64;
                    match element.as_ref() {
                        mirrc::ast::types::SignalType::Unsigned(ew) => w = *ew,
                        mirrc::ast::types::SignalType::Signed(ew) => w = *ew,
                        mirrc::ast::types::SignalType::Bool => w = 1,
                        _ => {}
                    }
                    w * (*length as u32)
                }
                _ => 64,
            }
        } else {
            64
        }
    };

    for i in 0..max_entities {
        let idx = i;
        
        if let Some(kind_comp) = registry.kinds.get(idx).and_then(|k| k.as_ref()) {
            match kind_comp.0 {
                mirrc::ecs::EntityKind::SIGNAL(_) => {
                    signal_count += 1;
                    total_wire_bits += get_width(i as u32);
                }
                mirrc::ecs::EntityKind::GUARD => guard_count += 1,
                mirrc::ecs::EntityKind::REFLEX => reflex_count += 1,
                _ => {}
            }
        }

        if let Some(Some(_mux)) = registry.muxes.get(idx) {
            *cells.entry(("$mux", get_width(i as u32))).or_insert(0) += 1;
        }

        if let Some(Some(prev)) = registry.prev_ops.get(idx) {
            dff_bits += get_width(i as u32) * (prev.delay as u32);
            *cells.entry(("$dff", get_width(i as u32))).or_insert(0) += 1;
        }

        if let Some(Some(bin)) = registry.binary_ops.get(idx) {
            let width = get_width(i as u32);
            let name = match bin.op {
                mirrc::ast::types::BinaryOp::Add => "$add",
                mirrc::ast::types::BinaryOp::Sub => "$sub",
                mirrc::ast::types::BinaryOp::Mul => "$mul",
                mirrc::ast::types::BinaryOp::Eq => "$eq",
                mirrc::ast::types::BinaryOp::Ne => "$ne",
                mirrc::ast::types::BinaryOp::Lt => "$lt",
                mirrc::ast::types::BinaryOp::Le => "$le",
                mirrc::ast::types::BinaryOp::Gt => "$gt",
                mirrc::ast::types::BinaryOp::Ge => "$ge",
                mirrc::ast::types::BinaryOp::And | mirrc::ast::types::BinaryOp::BitwiseAnd => "$and",
                mirrc::ast::types::BinaryOp::Or | mirrc::ast::types::BinaryOp::BitwiseOr => "$or",
                mirrc::ast::types::BinaryOp::Xor | mirrc::ast::types::BinaryOp::BitwiseXor => "$xor",
                mirrc::ast::types::BinaryOp::Shl => "$shl",
                mirrc::ast::types::BinaryOp::Shr => "$shr",
            };
            *cells.entry((name, width)).or_insert(0) += 1;
        }

        if let Some(Some(un)) = registry.unary_ops.get(idx) {
            let width = get_width(i as u32);
            let name = match un.op {
                mirrc::ast::types::UnaryOp::Not | mirrc::ast::types::UnaryOp::BitwiseNot => "$not",
                mirrc::ast::types::UnaryOp::Neg => "$neg",
            };
            *cells.entry((name, width)).or_insert(0) += 1;
        }
    }

    if let Some(tn) = &result.temporal_netlist {
        for guard in &tn.guards {
            match guard {
                mirrc::temporal::low_level_ir::CompiledGuard::ShiftRegister(sr) => {
                    dff_bits += sr.delay_cycles as u32;
                    *cells.entry(("$dff_sr", 1)).or_insert(0) += sr.delay_cycles as usize;
                }
                mirrc::temporal::low_level_ir::CompiledGuard::Counter(c) => {
                    *cells.entry(("$counter", c.counter_width())).or_insert(0) += 1;
                }
                mirrc::temporal::low_level_ir::CompiledGuard::DynamicCounter(dc) => {
                    *cells.entry(("$dyn_counter", dc.counter_width())).or_insert(0) += 1;
                }
                _ => {}
            }
        }
    }

    eprintln!("MIRR Compile: {}", module_name);
    eprintln!("  Signals: {}  Guards: {}  Reflexes: {}", signal_count, guard_count, reflex_count);
    
    eprintln!("\n  -- Detailed Hardware Cell Usage (Yosys Format) --");
    eprintln!("    {:>9} wires", signal_count);
    eprintln!("    {:>9} wire bits", total_wire_bits);
    
    let mut sorted_cells: Vec<_> = cells.into_iter().collect();
    // Sort by name alphabetically, then by width ascending
    sorted_cells.sort_by(|((name_a, width_a), _), ((name_b, width_b), _)| {
        name_a.cmp(name_b).then(width_a.cmp(width_b))
    });

    for ((name, width), count) in sorted_cells {
        eprintln!("    {:>9}   {}_{}", count, name, width);
    }
    eprintln!("    {:>9}   $dff ({} bits total)", dff_bits, dff_bits);
    eprintln!("");
INNER_EOF
cat src/bin/mirr-compile/summary.rs | sed -n '106,121p' >> temp_summary.rs
mv temp_summary.rs src/bin/mirr-compile/summary.rs
