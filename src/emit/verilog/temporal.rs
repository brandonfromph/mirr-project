//! Temporal guard emission: shift-register, counter, dynamic-counter,
//! reflex assignments, and condition expressions.

#![forbid(unsafe_code)]

use crate::ast::types::BinaryOp;
use crate::emit::verilog::emit_source_comment;
use crate::span::FileTable;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};
use std::fmt::Write;

use super::MAX_SR_STAGES_INLINE;

pub(super) fn emit_temporal_logic_standalone(
    registry: &crate::ecs::Registry,
    netlist: &TemporalNetlist,
    _ft: &FileTable,
    out: &mut String,
) {
    out.push_str("  // ── Temporal Guards ──\n\n");

    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::EntityId(i as u32))
        } else {
            None
        }
    });

    for guard in &netlist.guards {
        let mut clock_domain = "clk";
        for i in 0..registry.names.len() {
            if let Some(top_id) = top_module_id {
                if let Some(crate::ecs::components::ModuleComponent(parent_id)) =
                    &registry.modules[i]
                {
                    if *parent_id != top_id {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let (Some(nc), Some(kind_comp)) = (&registry.names[i], &registry.kinds[i]) {
                if registry.resolve_name(nc.0) == guard.name()
                    && kind_comp.0 == crate::ecs::EntityKind::GUARD
                {
                    if let Some(tc) = &registry.types[i] {
                        if let Some(cd) = tc.0.annotations.clock_domain.as_deref() {
                            clock_domain = cd;
                        }
                    }
                    break;
                }
            }
        }

        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                emit_shift_register_guard(sr, clock_domain, out);
            }
            CompiledGuard::Counter(cg) => {
                emit_counter_guard(cg, clock_domain, out);
            }
            CompiledGuard::Complex(cx) => {
                writeln!(out, "  // Complex guard: {} (sub-guards combined)", cx.name).unwrap();
                write!(out, "  assign {} = ", cx.output_signal).unwrap();
                emit_logic_expr(&cx.combination_logic, out);
                writeln!(out, ";\n").unwrap();
            }
            CompiledGuard::DynamicCounter(dc) => {
                emit_dynamic_counter_guard(dc, clock_domain, out);
            }
        }
    }
}

pub(super) fn emit_temporal_logic_ecs(
    registry: &crate::ecs::Registry,
    netlist: &TemporalNetlist,
    _sync_map: &std::collections::HashMap<String, String>,
    ft: &FileTable,
    out: &mut String,
) {
    out.push_str("  // ── Temporal Guards ──\n\n");

    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::EntityId(i as u32))
        } else {
            None
        }
    });

    // Declare registers for ALL prev() back-references found in the module
    let mut seen_prevs = std::collections::HashSet::new();
    for i in 0..registry.reflex_comps.len() {
        if let Some(top_id) = top_module_id {
            if let Some(crate::ecs::components::ModuleComponent(parent_id)) = &registry.modules[i] {
                if *parent_id != top_id {
                    continue;
                }
            } else {
                continue;
            }
        }

        if let Some(reflex) = &registry.reflex_comps[i] {
            for asgn_ent in &reflex.assignments {
                if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                    collect_prevs_ecs(asgn.value, registry, &mut seen_prevs);
                }
            }
        }
    }

    let mut sorted_prevs: Vec<_> = seen_prevs.into_iter().collect();
    sorted_prevs.sort();

    let mut prev_groups_by_clock: std::collections::HashMap<
        String,
        Vec<(crate::ecs::EntityId, u64)>,
    > = std::collections::HashMap::new();

    for &(sig_ent, delay) in &sorted_prevs {
        if let Some(nc) = &registry.names[sig_ent.0 as usize] {
            if let Some(type_comp) = &registry.types[sig_ent.0 as usize] {
                let sig_name = registry.resolve_name(nc.0);
                let type_str = crate::emit::sv_type(&type_comp.0.signal_type());
                writeln!(out, "  {} {}_d{};", type_str, sig_name, delay).unwrap();

                let mut clock_domain = "clk";
                if let Some(cd) = type_comp.0.annotations.clock_domain.as_deref() {
                    clock_domain = cd;
                }
                prev_groups_by_clock
                    .entry(clock_domain.to_string())
                    .or_default()
                    .push((sig_ent, delay));
            }
        }
    }
    out.push('\n');

    for prevs in prev_groups_by_clock.values() {
        out.push_str("  // Physical Power-On Reset Initialization\n");
        out.push_str("  initial begin\n");
        for &(sig_ent, delay) in prevs {
            let sig_name = registry.names[sig_ent.0 as usize]
                .as_ref()
                .map(|nc| registry.resolve_name(nc.0))
                .unwrap_or("ERR_MISSING_NAME");
            writeln!(out, "    {}_d{} = '0;", sig_name, delay).unwrap();
        }
        out.push_str("  end\n\n");
    }

    for (clock, prevs) in prev_groups_by_clock {
        writeln!(out, "  // Delay line updates for prev() references (@{})", clock).unwrap();
        writeln!(out, "  always_ff @(posedge {} or negedge rst_n) begin", clock).unwrap();
        out.push_str("    if (!rst_n) begin\n");
        for &(sig_ent, delay) in &prevs {
            let sig_name = registry.names[sig_ent.0 as usize]
                .as_ref()
                .map(|nc| registry.resolve_name(nc.0))
                .unwrap_or("ERR_MISSING_NAME");
            writeln!(out, "      {}_d{} <= '0;", sig_name, delay).unwrap();
        }
        out.push_str("    end else begin\n");
        for &(sig_ent, delay) in &prevs {
            let sig_name = registry.names[sig_ent.0 as usize]
                .as_ref()
                .map(|nc| registry.resolve_name(nc.0))
                .unwrap_or("ERR_MISSING_NAME");
            if delay == 1 {
                writeln!(out, "      {}_d1 <= {};", sig_name, sig_name).unwrap();
            } else {
                writeln!(out, "      {}_d{} <= {}_d{};", sig_name, delay, sig_name, delay - 1)
                    .unwrap();
            }
        }
        out.push_str("    end\n");
        out.push_str("  end\n\n");
    }

    let mut emitted_shift_registers = std::collections::HashSet::new();

    for guard in &netlist.guards {
        // Find the guard entity to get its span and clock domain
        let mut span = None;
        let mut guard_module_id = None;
        let mut clock_domain = "clk";
        for i in 0..registry.names.len() {
            if let Some(top_id) = top_module_id {
                if let Some(crate::ecs::components::ModuleComponent(parent_id)) =
                    &registry.modules[i]
                {
                    if *parent_id != top_id {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let (Some(nc), Some(kind_comp)) = (&registry.names[i], &registry.kinds[i]) {
                if registry.resolve_name(nc.0) == guard.name()
                    && kind_comp.0 == crate::ecs::EntityKind::GUARD
                {
                    span = registry.spans[i].as_ref().map(|s| &s.0);
                    if let Some(crate::ecs::components::ModuleComponent(parent_id)) =
                        &registry.modules[i]
                    {
                        guard_module_id = Some(*parent_id);
                    }
                    if let Some(tc) = &registry.types[i] {
                        if let Some(cd) = tc.0.annotations.clock_domain.as_deref() {
                            clock_domain = cd;
                        }
                    }
                    break;
                }
            }
        }

        if let (Some(top_id), Some(g_mod_id)) = (top_module_id, guard_module_id) {
            if g_mod_id != top_id {
                continue;
            }
        }

        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                if emitted_shift_registers.insert(sr.name.clone()) {
                    emit_source_comment(span, ft, out);
                    emit_shift_register_guard(sr, clock_domain, out);
                }
            }
            CompiledGuard::Counter(cg) => {
                emit_source_comment(span, ft, out);
                emit_counter_guard(cg, clock_domain, out);
            }
            CompiledGuard::Complex(cx) => {
                if emitted_shift_registers.insert(cx.name.clone()) {
                    emit_source_comment(span, ft, out);
                    writeln!(out, "  // Complex guard: {} (sub-guards combined)", cx.name).unwrap();
                    write!(out, "  assign {} = ", cx.output_signal).unwrap();
                    emit_logic_expr(&cx.combination_logic, out);
                    writeln!(out, ";\n").unwrap();
                }
            }
            CompiledGuard::DynamicCounter(dc) => {
                emit_source_comment(span, ft, out);
                emit_dynamic_counter_guard(dc, clock_domain, out);
            }
        }
    }
}

fn collect_prevs_ecs(
    root: crate::ecs::EntityId,
    registry: &crate::ecs::Registry,
    seen: &mut std::collections::HashSet<(crate::ecs::EntityId, u64)>,
) {
    let mut stack = Vec::new();
    stack.push(root);
    let mut visited = 0;
    while let Some(id) = stack.pop() {
        visited += 1;
        if visited > 512 {
            break;
        }
        let idx = id.0 as usize;
        if let Some(p) = &registry.prev_ops[idx] {
            let mut target_sig = p.signal;
            if let Some(sig_ref) = &registry.signal_refs[p.signal.0 as usize] {
                target_sig = sig_ref.0;
            }
            seen.insert((target_sig, p.delay));
            stack.push(p.signal);
        } else if let Some(b) = &registry.binary_ops[idx] {
            stack.push(b.left);
            stack.push(b.right);
        } else if let Some(u) = &registry.unary_ops[idx] {
            stack.push(u.operand);
        } else if let Some(m) = &registry.muxes[idx] {
            stack.push(m.select);
            stack.push(m.true_val);
            stack.push(m.false_val);
        }
    }
}

pub(super) fn emit_reflex_logic_ecs(
    registry: &crate::ecs::Registry,
    dsp_reflexes: &std::collections::HashSet<String>,
    dsp_attr: Option<&str>,
    sync_map: &std::collections::HashMap<String, String>,
    ft: &FileTable,
    out: &mut String,
) {
    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::EntityId(i as u32))
        } else {
            None
        }
    });

    // Group assignments by target signal
    let mut signal_to_reflexes: std::collections::HashMap<String, Vec<usize>> =
        std::collections::HashMap::new();

    // Track guard names used in reflexes to declare their _out wires
    let mut guard_names_used: std::collections::HashSet<String> = std::collections::HashSet::new();

    for i in 0..registry.reflex_comps.len() {
        if let Some(top_id) = top_module_id {
            if let Some(crate::ecs::components::ModuleComponent(parent_id)) = &registry.modules[i] {
                if *parent_id != top_id {
                    continue;
                }
            } else {
                continue;
            }
        }

        if let Some(reflex) = &registry.reflex_comps[i] {
            for g_ent in &reflex.guards {
                if let Some(nc) = &registry.names[g_ent.0 as usize] {
                    let gname = registry.resolve_name(nc.0);
                    if gname != "always" {
                        guard_names_used.insert(gname.to_string());
                    }
                }
            }
            for asgn_ent in &reflex.assignments {
                if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                    if let Some(target_nc) = &registry.names[asgn.target.0 as usize] {
                        let target_name = registry.resolve_name(target_nc.0);
                        let refs = signal_to_reflexes.entry(target_name.to_string()).or_default();
                        if !refs.contains(&i) {
                            refs.push(i);
                        }
                    }
                }
            }
        }
    }

    if signal_to_reflexes.is_empty() && guard_names_used.is_empty() {
        return;
    }

    out.push_str("  // ── Reflex Assignments ──\n\n");

    out.push_str("  // ── Reflex Signal Drivers ──\n\n");

    // Emit HLS Logic if any entities have HLS schedules
    let mut has_hls = false;
    for i in 0..registry.active_entities() {
        if registry.hls_schedules[i].is_some() {
            has_hls = true;
            break;
        }
    }

    if has_hls {
        emit_hls_logic_ecs(registry, ft, sync_map, out);
    } else {
        // Sort signals by name for deterministic emission.
        let mut signals: Vec<String> = signal_to_reflexes.keys().cloned().collect();
        signals.sort();

        for sig_name in signals {
            let reflex_indices = &signal_to_reflexes[&sig_name];

            // Find the clock domain for this target signal.
            let mut clock_domain = "clk";
            for i in 0..registry.names.len() {
                if let Some(top_id) = top_module_id {
                    if let Some(crate::ecs::components::ModuleComponent(parent_id)) =
                        &registry.modules[i]
                    {
                        if *parent_id != top_id {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                if let Some(nc) = &registry.names[i] {
                    if registry.resolve_name(nc.0) == sig_name {
                        if let Some(tc) = &registry.types[i] {
                            if let Some(cd) = tc.0.annotations.clock_domain.as_deref() {
                                clock_domain = cd;
                            }
                        }
                        break;
                    }
                }
            }

            // Emit DSP synthesis attribute if ANY reflex for this signal contains a multiply.
            if let Some(attr) = dsp_attr {
                let mut has_dsp = false;
                for &ri in reflex_indices {
                    if let Some(nc) = &registry.names[ri] {
                        if dsp_reflexes.contains(registry.resolve_name(nc.0)) {
                            has_dsp = true;
                            break;
                        }
                    }
                }
                if has_dsp {
                    writeln!(out, "  {attr}").unwrap();
                }
            }

            writeln!(out, "  // Unified Reflex Block for: {sig_name} (@{clock_domain})").unwrap();
            out.push_str("  initial begin\n");
            writeln!(out, "    {} = '0;", sig_name).unwrap();
            out.push_str("  end\n");
            writeln!(out, "  always_ff @(posedge {clock_domain} or negedge rst_n) begin").unwrap();
            out.push_str("    if (!rst_n) begin\n");
            writeln!(out, "      {} <= '0;", sig_name).unwrap();
            out.push_str("    end else begin\n");

            // Priority-ordered assignments
            for &ri in reflex_indices {
                if let Some(reflex) = &registry.reflex_comps[ri] {
                    let mut guard_parts = Vec::new();
                    for g_ent in &reflex.guards {
                        if let Some(nc) = &registry.names[g_ent.0 as usize] {
                            guard_parts.push(format!("{}_out", registry.resolve_name(nc.0)));
                        }
                    }
                    let guard_cond = if guard_parts.is_empty() {
                        "1'b1".to_string()
                    } else {
                        guard_parts.join(" && ")
                    };

                    for asgn_ent in &reflex.assignments {
                        if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                            if let Some(target_nc) = &registry.names[asgn.target.0 as usize] {
                                let target_name = registry.resolve_name(target_nc.0);
                                if target_name == sig_name {
                                    let span = registry.spans[ri].as_ref().map(|s| &s.0);
                                    emit_source_comment(span, ft, out);
                                    writeln!(
                                        out,
                                        "      if ({}) {} <= {};",
                                        guard_cond,
                                        sig_name,
                                        super::emit_expr_inline(asgn.value, registry, sync_map),
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
            }
            out.push_str("    end\n");
            out.push_str("  end\n\n");
        }
    }
}

fn emit_hls_logic_ecs(
    registry: &crate::ecs::Registry,
    _ft: &FileTable,
    sync_map: &std::collections::HashMap<String, String>,
    out: &mut String,
) {
    out.push_str("  // ── HLS Finite State Machine & Shared Resources (MEGA-12) ──\n\n");

    // 1. Group HLS entities by clock domain.
    let mut domain_entities: std::collections::HashMap<String, Vec<usize>> =
        std::collections::HashMap::new();

    for i in 0..registry.active_entities() {
        if registry.hls_schedules[i].is_some() || registry.hls_dataflow[i].is_some() {
            let mut clock_domain = "clk";
            if let Some(tc) = &registry.types[i] {
                if let Some(cd) = tc.0.annotations.clock_domain.as_deref() {
                    clock_domain = cd;
                }
            }
            domain_entities.entry(clock_domain.to_string()).or_default().push(i);
        }
    }

    let mut sorted_domains: Vec<String> = domain_entities.keys().cloned().collect();
    sorted_domains.sort();

    for cd in sorted_domains {
        let entities = &domain_entities[&cd];
        let state_reg =
            if cd == "clk" { "hls_state".to_string() } else { format!("hls_state_{}", cd) };

        writeln!(out, "  // Clock Domain: {}", cd).unwrap();
        write!(out, "  logic [31:0] {};\n\n", state_reg).unwrap();

        // 2. Group operations by cycle and collect bindings.
        let mut cycle_to_ops: std::collections::HashMap<u32, Vec<usize>> =
            std::collections::HashMap::new();
        let mut bindings_map: std::collections::HashMap<u32, Vec<usize>> =
            std::collections::HashMap::new();

        let mut max_cycle = 0;
        for &i in entities {
            if let Some(sched) = &registry.hls_schedules[i] {
                cycle_to_ops.entry(sched.earliest).or_default().push(i);
                if sched.earliest > max_cycle {
                    max_cycle = sched.earliest;
                }
                if let Some(binding) = &registry.hls_bindings[i] {
                    bindings_map.entry(binding.physical_resource_id).or_default().push(i);
                }
            }
        }

        // 3. Declare intermediate wires for every HLS operation's result (registers).
        for &i in entities {
            if registry.hls_dataflow[i].is_some() {
                if let Some(tc) = &registry.types[i] {
                    let width = tc.0.core.width();
                    writeln!(out, "  logic [{}:0] op_{}_res;", width.saturating_sub(1), i).unwrap();
                }
            }
        }
        out.push('\n');

        // 4. Emit Shared Functional Units and Input MUXes.
        let mut sorted_binding_ids: Vec<u32> = bindings_map.keys().cloned().collect();
        sorted_binding_ids.sort();

        for &binding_id in &sorted_binding_ids {
            let ops = &bindings_map[&binding_id];
            if ops.is_empty() {
                continue;
            }

            let first_op_idx = ops[0];
            let kind = registry.hls_schedules[first_op_idx]
                .as_ref()
                .map(|s| s.resource)
                .unwrap_or(crate::hls::ResourceKind::Add); // Fallback to avoid panic

            // Find max width among all operations sharing this resource
            let mut max_width = 1;
            for &idx in ops {
                if let Some(tc) = &registry.types[idx] {
                    max_width = max_width.max(tc.0.core.width());
                }
            }

            writeln!(out, "  // Shared Resource: {:?} (ID: {})", kind, binding_id).unwrap();
            writeln!(
                out,
                "  logic [{}:0] shared_{}_{}_in_A;",
                max_width.saturating_sub(1),
                kind,
                binding_id
            )
            .unwrap();
            if !matches!(kind, crate::hls::ResourceKind::Not | crate::hls::ResourceKind::Negate) {
                writeln!(
                    out,
                    "  logic [{}:0] shared_{}_{}_in_B;",
                    max_width.saturating_sub(1),
                    kind,
                    binding_id
                )
                .unwrap();
            }
            writeln!(
                out,
                "  logic [{}:0] shared_{}_{}_out;",
                max_width.saturating_sub(1),
                kind,
                binding_id
            )
            .unwrap();

            // Emit the actual hardware block
            let op_str = match kind {
                crate::hls::ResourceKind::Add => "+",
                crate::hls::ResourceKind::Sub => "-",
                crate::hls::ResourceKind::Mul => "*",
                crate::hls::ResourceKind::And => "&",
                crate::hls::ResourceKind::Or => "|",
                crate::hls::ResourceKind::Xor => "^",
                crate::hls::ResourceKind::Eq => "==",
                crate::hls::ResourceKind::Ne => "!=",
                crate::hls::ResourceKind::Lt => "<",
                crate::hls::ResourceKind::Le => "<=",
                crate::hls::ResourceKind::Gt => ">",
                crate::hls::ResourceKind::Ge => ">=",
                crate::hls::ResourceKind::Shl => "<<",
                crate::hls::ResourceKind::Shr => ">>",
                crate::hls::ResourceKind::Not => "!",
                crate::hls::ResourceKind::Negate => "-",
            };

            if matches!(kind, crate::hls::ResourceKind::Not | crate::hls::ResourceKind::Negate) {
                writeln!(
                    out,
                    "  assign shared_{}_{}_out = {}(shared_{}_{}_in_A);",
                    kind, binding_id, op_str, kind, binding_id
                )
                .unwrap();
            } else {
                writeln!(
                    out,
                    "  assign shared_{}_{}_out = shared_{}_{}_in_A {} shared_{}_{}_in_B;",
                    kind, binding_id, kind, binding_id, op_str, kind, binding_id
                )
                .unwrap();
            }
            // Emit MUXes
            out.push_str("  always_comb begin\n");
            writeln!(out, "    case ({})", state_reg).unwrap();

            for &idx in ops {
                let cycle = registry.hls_schedules[idx].as_ref().map(|s| s.earliest).unwrap_or(0);
                writeln!(out, "      {}: begin", cycle).unwrap();

                let get_operand_str = |op_entity_id: u32| -> String {
                    let op_idx = op_entity_id as usize;
                    if registry.hls_schedules[op_idx].is_some() {
                        format!("op_{}_res", op_entity_id)
                    } else if let Some(lit) = &registry.literals[op_idx] {
                        match &lit.0 {
                            crate::ast::types::LiteralValue::Integer(n) => format!("{}", n),
                            crate::ast::types::LiteralValue::Bool(b) => {
                                if *b {
                                    "1'b1".to_string()
                                } else {
                                    "1'b0".to_string()
                                }
                            }
                        }
                    } else if let Some(nc) = &registry.names[op_idx] {
                        let name = registry.resolve_name(nc.0).to_string();
                        sync_map.get(&name).cloned().unwrap_or(name)
                    } else {
                        format!("op_{}_res", op_entity_id) // Fallback
                    }
                };

                if let Some(binary) = &registry.binary_ops[idx] {
                    writeln!(
                        out,
                        "        shared_{}_{}_in_A = {};",
                        kind,
                        binding_id,
                        get_operand_str(binary.left.0)
                    )
                    .unwrap();
                    writeln!(
                        out,
                        "        shared_{}_{}_in_B = {};",
                        kind,
                        binding_id,
                        get_operand_str(binary.right.0)
                    )
                    .unwrap();
                } else if let Some(unary) = &registry.unary_ops[idx] {
                    writeln!(
                        out,
                        "        shared_{}_{}_in_A = {};",
                        kind,
                        binding_id,
                        get_operand_str(unary.operand.0)
                    )
                    .unwrap();
                }
                out.push_str("      end\n");
            }
            out.push_str("      default: begin\n");
            writeln!(out, "        shared_{}_{}_in_A = '0;", kind, binding_id).unwrap();
            if !matches!(kind, crate::hls::ResourceKind::Not | crate::hls::ResourceKind::Negate) {
                writeln!(out, "        shared_{}_{}_in_B = '0;", kind, binding_id).unwrap();
            }
            out.push_str("      end\n");
            out.push_str("    endcase\n");
            out.push_str("  end\n\n");
        }

        // 5. Emit the FSM block.
        let mut sorted_cycles: Vec<u32> = cycle_to_ops.keys().cloned().collect();
        sorted_cycles.sort();

        writeln!(out, "  always_ff @(posedge {} or negedge rst_n) begin", cd).unwrap();
        out.push_str("    if (!rst_n) begin\n");
        writeln!(out, "      {} <= 0;", state_reg).unwrap();
        out.push_str("    end else begin\n");
        writeln!(out, "      case ({})", state_reg).unwrap();

        for cycle in sorted_cycles {
            writeln!(out, "        {}: begin", cycle).unwrap();
            let op_indices = &cycle_to_ops[&cycle];
            for &idx in op_indices {
                // Check if this operation has a binding
                if let Some(binding) = &registry.hls_bindings[idx] {
                    let kind = registry.hls_schedules[idx]
                        .as_ref()
                        .map(|s| s.resource)
                        .unwrap_or(crate::hls::ResourceKind::Add);
                    writeln!(
                        out,
                        "          op_{}_res <= shared_{}_{}_out;",
                        idx, kind, binding.physical_resource_id
                    )
                    .unwrap();
                } else {
                    writeln!(out, "          // Error: Operation {} unbound", idx).unwrap();
                }

                // Assign target signal if this entity is assigned to one
                for asgn_idx in 0..registry.active_entities() {
                    if let Some(asgn) = &registry.assignment_comps[asgn_idx] {
                        if asgn.value.0 == idx as u32 {
                            if let Some(nc) = &registry.names[asgn.target.0 as usize] {
                                writeln!(
                                    out,
                                    "          {} <= op_{}_res;",
                                    registry.resolve_name(nc.0),
                                    idx
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            }

            // State transition.
            if cycle < max_cycle {
                writeln!(out, "          {} <= {};", state_reg, cycle + 1).unwrap();
            } else {
                writeln!(out, "          {} <= 0;", state_reg).unwrap();
            }
            out.push_str("        end\n");
        }

        writeln!(out, "        default: {} <= 0;", state_reg).unwrap();
        out.push_str("      endcase\n");
        out.push_str("    end\n");
        out.push_str("  end\n\n");
    }
}

fn emit_shift_register_guard(
    sr: &crate::temporal::low_level_ir::ShiftRegisterGuard,
    clock: &str,
    out: &mut String,
) {
    let cond_desc = sr.condition_kind.describe();
    // Special case: 0 or 1-cycle guard is purely combinational.
    if sr.delay_cycles <= 1 {
        writeln!(
            out,
            "  // Guard: {} (len {}, bytes {:?}) — {} for {} cycle (combinational)",
            sr.name,
            sr.name.len(),
            sr.name.as_bytes(),
            cond_desc,
            sr.delay_cycles
        )
        .unwrap();
        writeln!(out, "  logic {}_cond;", sr.name).unwrap();
        write!(out, "  assign {}_cond = ", sr.name).unwrap();
        emit_condition_expr(&sr.condition_kind, out);
        writeln!(out, ";").unwrap();
        writeln!(out, "  assign {} = {}_cond;\n", sr.output_signal, sr.name).unwrap();
        return;
    }

    writeln!(
        out,
        "  // Guard: {} (len {}, bytes {:?}) — {} for {} cycles",
        sr.name,
        sr.name.len(),
        sr.name.as_bytes(),
        cond_desc,
        sr.delay_cycles
    )
    .unwrap();

    let stage_count = sr.delay_cycles.min(MAX_SR_STAGES_INLINE);

    // Declare the shift register.
    writeln!(out, "  logic [{}:0] {}_sr;", stage_count.saturating_sub(1), sr.name,).unwrap();
    out.push_str("  initial begin\n");
    writeln!(out, "    {}_sr = '0;", sr.name).unwrap();
    out.push_str("  end\n");

    // Condition wire.
    writeln!(out, "  logic {}_cond;", sr.name).unwrap();
    write!(out, "  assign {}_cond = ", sr.name).unwrap();
    emit_condition_expr(&sr.condition_kind, out);
    writeln!(out, ";").unwrap();

    // Shift register always_ff block.
    writeln!(out, "  always_ff @(posedge {clock} or negedge rst_n) begin").unwrap();
    write!(out, "    if (!rst_n)\n      {}_sr <= '0;\n", sr.name).unwrap();
    write!(
        out,
        "    else\n      {0}_sr <= {{{0}_cond, {0}_sr[{1}:1]}};\n",
        sr.name,
        stage_count.saturating_sub(1),
    )
    .unwrap();
    out.push_str("  end\n");

    // Output: guard fires when all stages are 1.
    write!(out, "  assign {} = &{}_sr;\n\n", sr.output_signal, sr.name,).unwrap();
}

fn emit_counter_guard(
    cg: &crate::temporal::low_level_ir::CounterGuard,
    clock: &str,
    out: &mut String,
) {
    let cond_desc = cg.condition_kind.describe();
    let width = cg.counter_width();
    writeln!(
        out,
        "  // Guard: {} — {} for {} cycles (counter)",
        cg.name, cond_desc, cg.target_count
    )
    .unwrap();

    // Counter register.
    writeln!(out, "  logic [{}:0] {};", width.saturating_sub(1), cg.counter_signal,).unwrap();
    out.push_str("  initial begin\n");
    writeln!(out, "    {} = '0;", cg.counter_signal).unwrap();
    out.push_str("  end\n");

    // Condition wire.
    writeln!(out, "  logic {}_cond;", cg.name).unwrap();
    write!(out, "  assign {}_cond = ", cg.name).unwrap();
    emit_condition_expr(&cg.condition_kind, out);
    writeln!(out, ";").unwrap();

    // Counter always_ff block.
    writeln!(out, "  always_ff @(posedge {clock} or negedge rst_n) begin").unwrap();
    write!(out, "    if (!rst_n)\n      {} <= '0;\n", cg.counter_signal).unwrap();
    write!(out, "    else if (!{}_cond)\n      {} <= '0;\n", cg.name, cg.counter_signal).unwrap();
    write!(
        out,
        "    else if ({0} < {1})\n      {0} <= {0} + 1;\n",
        cg.counter_signal, cg.target_count,
    )
    .unwrap();
    out.push_str("  end\n");

    // Output: guard fires when counter reaches target.
    write!(
        out,
        "  assign {} = ({} >= {});\n\n",
        cg.output_signal, cg.counter_signal, cg.target_count,
    )
    .unwrap();
}

fn emit_dynamic_counter_guard(
    dc: &crate::temporal::low_level_ir::DynamicCounterGuard,
    clock: &str,
    out: &mut String,
) {
    let cond_desc = dc.condition_kind.describe();
    let width = dc.counter_width();
    writeln!(
        out,
        "  // Guard: {} — {} for dynamic delay (max {} cycles)",
        dc.name, cond_desc, dc.max_delay
    )
    .unwrap();

    // Counter register.
    writeln!(out, "  logic [{}:0] {};", width.saturating_sub(1), dc.counter_signal).unwrap();
    out.push_str("  initial begin\n");
    writeln!(out, "    {} = '0;", dc.counter_signal).unwrap();
    out.push_str("  end\n");

    // Dynamic target wire.
    let target_signal = format!("{}_target", dc.name);
    writeln!(out, "  logic [{}:0] {};", width.saturating_sub(1), target_signal).unwrap();
    writeln!(out, "  assign {} = {};", target_signal, dc.delay_expr,).unwrap();

    // Condition wire.
    writeln!(out, "  logic {}_cond;", dc.name).unwrap();
    write!(out, "  assign {}_cond = ", dc.name).unwrap();
    emit_condition_expr(&dc.condition_kind, out);
    writeln!(out, ";").unwrap();

    // Counter always_ff block.
    writeln!(out, "  always_ff @(posedge {clock} or negedge rst_n) begin").unwrap();
    write!(out, "    if (!rst_n)\n      {} <= '0;\n", dc.counter_signal).unwrap();
    write!(out, "    else if (!{}_cond)\n      {} <= '0;\n", dc.name, dc.counter_signal).unwrap();
    write!(
        out,
        "    else if ({0} < {1})\n      {0} <= {0} + 1;\n",
        dc.counter_signal, target_signal
    )
    .unwrap();
    out.push_str("  end\n");

    // Output: guard fires when counter reaches dynamic target.
    write!(
        out,
        "  assign {} = ({} >= {});\n\n",
        dc.output_signal, dc.counter_signal, target_signal
    )
    .unwrap();
}

/// Emit a ConditionKind as an inline SystemVerilog expression.
fn emit_condition_expr(ck: &crate::temporal::low_level_ir::ConditionKind, out: &mut String) {
    use crate::temporal::low_level_ir::ConditionKind;
    match ck {
        ConditionKind::SimpleSignal(s) => write!(out, "{s}").unwrap(),
        ConditionKind::NegatedSignal(s) => write!(out, "!{s}").unwrap(),
        ConditionKind::PrevSignal { signal, .. } => {
            write!(out, "{signal}").unwrap();
        }
        ConditionKind::Comparison { signal, op, value } => {
            let op_str = match op {
                BinaryOp::Eq => "==",
                BinaryOp::Ne => "!=",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                _ => "??",
            };
            let val_str = match value {
                crate::ast::types::LiteralValue::Integer(n) => format!("{n}"),
                crate::ast::types::LiteralValue::Bool(b) => {
                    if *b {
                        "1'b1".to_string()
                    } else {
                        "1'b0".to_string()
                    }
                }
            };
            write!(out, "({signal} {op_str} {val_str})").unwrap();
        }
        ConditionKind::AlwaysTrue => write!(out, "1'b1").unwrap(),
    }
}

/// Emit a LogicExpr as an inline SystemVerilog expression.
fn emit_logic_expr(le: &crate::temporal::low_level_ir::LogicExpr, out: &mut String) {
    use crate::temporal::low_level_ir::LogicExpr;
    match le {
        LogicExpr::Signal(s) => write!(out, "{s}").unwrap(),
        LogicExpr::And(l, r) => {
            write!(out, "(").unwrap();
            emit_logic_expr(l, out);
            write!(out, " && ").unwrap();
            emit_logic_expr(r, out);
            write!(out, ")").unwrap();
        }
        LogicExpr::Or(l, r) => {
            write!(out, "(").unwrap();
            emit_logic_expr(l, out);
            write!(out, " || ").unwrap();
            emit_logic_expr(r, out);
            write!(out, ")").unwrap();
        }
    }
}
