//! Temporal guard emission: shift-register, counter, dynamic-counter,
//! reflex assignments, and condition expressions.

#![forbid(unsafe_code)]

use crate::ast::types::BinaryOp;
use crate::emit::verilog::emit_source_comment;
use crate::span::FileTable;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};

use super::MAX_SR_STAGES_INLINE;

pub(super) fn emit_temporal_logic_standalone(
    registry: &crate::ecs::Registry,
    netlist: &TemporalNetlist,
    _ft: &FileTable,
    out: &mut String,
) {
    out.push_str("  // ── Temporal Guards ──\n\n");

    for guard in &netlist.guards {
        let mut clock_domain = "clk";
        for i in 0..registry.names.len() {
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
                out.push_str(&format!("  // Complex guard: {} (sub-guards combined)\n", cx.name));
                out.push_str(&format!(
                    "  assign {} = {};\n\n",
                    cx.output_signal,
                    emit_logic_expr(&cx.combination_logic),
                ));
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
                out.push_str(&format!("  {} {}_d{};\n", type_str, sig_name, delay));

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
            let sig_name =
                registry.resolve_name(registry.names[sig_ent.0 as usize].as_ref().unwrap().0);
            out.push_str(&format!("    {}_d{} = '0;\n", sig_name, delay));
        }
        out.push_str("  end\n\n");
    }

    for (clock, prevs) in prev_groups_by_clock {
        out.push_str(&format!("  // Delay line updates for prev() references (@{})\n", clock));
        out.push_str(&format!("  always_ff @(posedge {} or negedge rst_n) begin\n", clock));
        out.push_str("    if (!rst_n) begin\n");
        for &(sig_ent, delay) in &prevs {
            let sig_name =
                registry.resolve_name(registry.names[sig_ent.0 as usize].as_ref().unwrap().0);
            out.push_str(&format!("      {}_d{} <= '0;\n", sig_name, delay));
        }
        out.push_str("    end else begin\n");
        for &(sig_ent, delay) in &prevs {
            let sig_name =
                registry.resolve_name(registry.names[sig_ent.0 as usize].as_ref().unwrap().0);
            if delay == 1 {
                out.push_str(&format!("      {}_d1 <= {};\n", sig_name, sig_name));
            } else {
                out.push_str(&format!(
                    "      {}_d{} <= {}_d{};\n",
                    sig_name,
                    delay,
                    sig_name,
                    delay - 1
                ));
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
                    out.push_str(&format!(
                        "  // Complex guard: {} (sub-guards combined)\n",
                        cx.name
                    ));
                    out.push_str(&format!(
                        "  assign {} = {};\n\n",
                        cx.output_signal,
                        emit_logic_expr(&cx.combination_logic),
                    ));
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
                        signal_to_reflexes.entry(target_name.to_string()).or_default().push(i);
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
                    out.push_str(&format!("  {attr}\n"));
                }
            }

            out.push_str(&format!("  // Unified Reflex Block for: {sig_name} (@{clock_domain})\n"));
            out.push_str("  initial begin\n");
            out.push_str(&format!("    {} = '0;\n", sig_name));
            out.push_str("  end\n");
            out.push_str(&format!(
                "  always_ff @(posedge {clock_domain} or negedge rst_n) begin\n"
            ));
            out.push_str("    if (!rst_n) begin\n");
            out.push_str(&format!("      {} <= '0;\n", sig_name));
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
                                    out.push_str(&format!(
                                        "      if ({}) {} <= {};\n",
                                        guard_cond,
                                        sig_name,
                                        super::emit_expr_inline(asgn.value, registry, sync_map),
                                    ));
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

        out.push_str(&format!("  // Clock Domain: {}\n", cd));
        out.push_str(&format!("  logic [31:0] {};\n\n", state_reg));

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
                    out.push_str(&format!(
                        "  logic [{}:0] op_{}_res;\n",
                        width.saturating_sub(1),
                        i
                    ));
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
            let sched = registry.hls_schedules[first_op_idx].as_ref().unwrap();
            let kind = sched.resource;

            // Find max width among all operations sharing this resource
            let mut max_width = 1;
            for &idx in ops {
                if let Some(tc) = &registry.types[idx] {
                    max_width = max_width.max(tc.0.core.width());
                }
            }

            out.push_str(&format!("  // Shared Resource: {:?} (ID: {})\n", kind, binding_id));
            out.push_str(&format!(
                "  logic [{}:0] shared_{}_{}_in_A;\n",
                max_width.saturating_sub(1),
                kind,
                binding_id
            ));
            if !matches!(kind, crate::hls::ResourceKind::Not | crate::hls::ResourceKind::Negate) {
                out.push_str(&format!(
                    "  logic [{}:0] shared_{}_{}_in_B;\n",
                    max_width.saturating_sub(1),
                    kind,
                    binding_id
                ));
            }
            out.push_str(&format!(
                "  logic [{}:0] shared_{}_{}_out;\n",
                max_width.saturating_sub(1),
                kind,
                binding_id
            ));

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
                out.push_str(&format!(
                    "  assign shared_{}_{}_out = {}(shared_{}_{}_in_A);\n",
                    kind, binding_id, op_str, kind, binding_id
                ));
            } else {
                out.push_str(&format!(
                    "  assign shared_{}_{}_out = shared_{}_{}_in_A {} shared_{}_{}_in_B;\n",
                    kind, binding_id, kind, binding_id, op_str, kind, binding_id
                ));
            }

            // Emit MUXes
            out.push_str("  always_comb begin\n");
            out.push_str(&format!("    case ({})\n", state_reg));

            for &idx in ops {
                let cycle = registry.hls_schedules[idx].as_ref().unwrap().earliest;
                out.push_str(&format!("      {}: begin\n", cycle));

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
                    out.push_str(&format!(
                        "        shared_{}_{}_in_A = {};\n",
                        kind,
                        binding_id,
                        get_operand_str(binary.left.0)
                    ));
                    out.push_str(&format!(
                        "        shared_{}_{}_in_B = {};\n",
                        kind,
                        binding_id,
                        get_operand_str(binary.right.0)
                    ));
                } else if let Some(unary) = &registry.unary_ops[idx] {
                    out.push_str(&format!(
                        "        shared_{}_{}_in_A = {};\n",
                        kind,
                        binding_id,
                        get_operand_str(unary.operand.0)
                    ));
                }
                out.push_str("      end\n");
            }
            out.push_str("      default: begin\n");
            out.push_str(&format!("        shared_{}_{}_in_A = '0;\n", kind, binding_id));
            if !matches!(kind, crate::hls::ResourceKind::Not | crate::hls::ResourceKind::Negate) {
                out.push_str(&format!("        shared_{}_{}_in_B = '0;\n", kind, binding_id));
            }
            out.push_str("      end\n");
            out.push_str("    endcase\n");
            out.push_str("  end\n\n");
        }

        // 5. Emit the FSM block.
        let mut sorted_cycles: Vec<u32> = cycle_to_ops.keys().cloned().collect();
        sorted_cycles.sort();

        out.push_str(&format!("  always_ff @(posedge {} or negedge rst_n) begin\n", cd));
        out.push_str("    if (!rst_n) begin\n");
        out.push_str(&format!("      {} <= 0;\n", state_reg));
        out.push_str("    end else begin\n");
        out.push_str(&format!("      case ({})\n", state_reg));

        for cycle in sorted_cycles {
            out.push_str(&format!("        {}: begin\n", cycle));
            let op_indices = &cycle_to_ops[&cycle];
            for &idx in op_indices {
                // Check if this operation has a binding
                if let Some(binding) = &registry.hls_bindings[idx] {
                    let kind = registry.hls_schedules[idx].as_ref().unwrap().resource;
                    out.push_str(&format!(
                        "          op_{}_res <= shared_{}_{}_out;\n",
                        idx, kind, binding.physical_resource_id
                    ));
                } else {
                    out.push_str(&format!("          // Error: Operation {} unbound\n", idx));
                }

                // Assign target signal if this entity is assigned to one
                for asgn_idx in 0..registry.active_entities() {
                    if let Some(asgn) = &registry.assignment_comps[asgn_idx] {
                        if asgn.value.0 == idx as u32 {
                            if let Some(nc) = &registry.names[asgn.target.0 as usize] {
                                out.push_str(&format!(
                                    "          {} <= op_{}_res;\n",
                                    registry.resolve_name(nc.0),
                                    idx
                                ));
                            }
                        }
                    }
                }
            }

            // State transition.
            if cycle < max_cycle {
                out.push_str(&format!("          {} <= {};\n", state_reg, cycle + 1));
            } else {
                out.push_str(&format!("          {} <= 0;\n", state_reg));
            }
            out.push_str("        end\n");
        }

        out.push_str(&format!("        default: {} <= 0;\n", state_reg));
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
        out.push_str(&format!(
            "  // Guard: {} (len {}, bytes {:?}) — {} for {} cycle (combinational)\n",
            sr.name,
            sr.name.len(),
            sr.name.as_bytes(),
            cond_desc,
            sr.delay_cycles
        ));
        out.push_str(&format!("  logic {}_cond;\n", sr.name));
        out.push_str(&format!(
            "  assign {}_cond = {};\n",
            sr.name,
            emit_condition_expr(&sr.condition_kind),
        ));
        out.push_str(&format!("  assign {} = {}_cond;\n\n", sr.output_signal, sr.name));
        return;
    }

    out.push_str(&format!(
        "  // Guard: {} (len {}, bytes {:?}) — {} for {} cycles\n",
        sr.name,
        sr.name.len(),
        sr.name.as_bytes(),
        cond_desc,
        sr.delay_cycles
    ));

    let stage_count = sr.delay_cycles.min(MAX_SR_STAGES_INLINE);

    // Declare the shift register.
    out.push_str(&format!("  logic [{}:0] {}_sr;\n", stage_count.saturating_sub(1), sr.name,));
    out.push_str("  initial begin\n");
    out.push_str(&format!("    {}_sr = '0;\n", sr.name));
    out.push_str("  end\n");

    // Condition wire.
    out.push_str(&format!("  logic {}_cond;\n", sr.name));
    out.push_str(&format!(
        "  assign {}_cond = {};\n",
        sr.name,
        emit_condition_expr(&sr.condition_kind),
    ));

    // Shift register always_ff block.
    out.push_str(&format!("  always_ff @(posedge {clock} or negedge rst_n) begin\n"));
    out.push_str(&format!("    if (!rst_n)\n      {}_sr <= '0;\n", sr.name));
    out.push_str(&format!(
        "    else\n      {0}_sr <= {{{0}_cond, {0}_sr[{1}:1]}};\n",
        sr.name,
        stage_count.saturating_sub(1),
    ));
    out.push_str("  end\n");

    // Output: guard fires when all stages are 1.
    out.push_str(&format!("  assign {} = &{}_sr;\n\n", sr.output_signal, sr.name,));
}

fn emit_counter_guard(
    cg: &crate::temporal::low_level_ir::CounterGuard,
    clock: &str,
    out: &mut String,
) {
    let cond_desc = cg.condition_kind.describe();
    let width = cg.counter_width();
    out.push_str(&format!(
        "  // Guard: {} — {} for {} cycles (counter)\n",
        cg.name, cond_desc, cg.target_count
    ));

    // Counter register.
    out.push_str(&format!("  logic [{}:0] {};\n", width.saturating_sub(1), cg.counter_signal,));
    out.push_str("  initial begin\n");
    out.push_str(&format!("    {} = '0;\n", cg.counter_signal));
    out.push_str("  end\n");

    // Condition wire.
    out.push_str(&format!("  logic {}_cond;\n", cg.name));
    out.push_str(&format!(
        "  assign {}_cond = {};\n",
        cg.name,
        emit_condition_expr(&cg.condition_kind),
    ));

    // Counter always_ff block.
    out.push_str(&format!("  always_ff @(posedge {clock} or negedge rst_n) begin\n"));
    out.push_str(&format!("    if (!rst_n)\n      {} <= '0;\n", cg.counter_signal));
    out.push_str(&format!("    else if (!{}_cond)\n      {} <= '0;\n", cg.name, cg.counter_signal));
    out.push_str(&format!(
        "    else if ({0} < {1})\n      {0} <= {0} + 1;\n",
        cg.counter_signal, cg.target_count,
    ));
    out.push_str("  end\n");

    // Output: guard fires when counter reaches target.
    out.push_str(&format!(
        "  assign {} = ({} >= {});\n\n",
        cg.output_signal, cg.counter_signal, cg.target_count,
    ));
}

fn emit_dynamic_counter_guard(
    dc: &crate::temporal::low_level_ir::DynamicCounterGuard,
    clock: &str,
    out: &mut String,
) {
    let cond_desc = dc.condition_kind.describe();
    let width = dc.counter_width();
    out.push_str(&format!(
        "  // Guard: {} — {} for dynamic delay (max {} cycles)\n",
        dc.name, cond_desc, dc.max_delay
    ));

    // Counter register.
    out.push_str(&format!("  logic [{}:0] {};\n", width.saturating_sub(1), dc.counter_signal));
    out.push_str("  initial begin\n");
    out.push_str(&format!("    {} = '0;\n", dc.counter_signal));
    out.push_str("  end\n");

    // Dynamic target wire.
    let target_signal = format!("{}_target", dc.name);
    out.push_str(&format!("  logic [{}:0] {};\n", width.saturating_sub(1), target_signal));
    out.push_str(&format!("  assign {} = {};\n", target_signal, dc.delay_expr,));

    // Condition wire.
    out.push_str(&format!("  logic {}_cond;\n", dc.name));
    out.push_str(&format!(
        "  assign {}_cond = {};\n",
        dc.name,
        emit_condition_expr(&dc.condition_kind),
    ));

    // Counter always_ff block.
    out.push_str(&format!("  always_ff @(posedge {clock} or negedge rst_n) begin\n"));
    out.push_str(&format!("    if (!rst_n)\n      {} <= '0;\n", dc.counter_signal));
    out.push_str(&format!("    else if (!{}_cond)\n      {} <= '0;\n", dc.name, dc.counter_signal));
    out.push_str(&format!(
        "    else if ({0} < {1})\n      {0} <= {0} + 1;\n",
        dc.counter_signal, target_signal
    ));
    out.push_str("  end\n");

    // Output: guard fires when counter reaches dynamic target.
    out.push_str(&format!(
        "  assign {} = ({} >= {});\n\n",
        dc.output_signal, dc.counter_signal, target_signal
    ));
}

/// Emit a ConditionKind as an inline SystemVerilog expression.
fn emit_condition_expr(ck: &crate::temporal::low_level_ir::ConditionKind) -> String {
    use crate::temporal::low_level_ir::ConditionKind;
    match ck {
        ConditionKind::SimpleSignal(s) => s.clone(),
        ConditionKind::NegatedSignal(s) => format!("!{s}"),
        ConditionKind::PrevSignal { signal, .. } => {
            // For now, we return the base signal; the temporal compiler
            // handles the extra delay by increasing the SR/Counter depth.
            signal.clone()
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
            format!("({signal} {op_str} {val_str})")
        }
        ConditionKind::AlwaysTrue => "1'b1".to_string(),
    }
}

/// Emit a LogicExpr as an inline SystemVerilog expression.
fn emit_logic_expr(le: &crate::temporal::low_level_ir::LogicExpr) -> String {
    use crate::temporal::low_level_ir::LogicExpr;
    match le {
        LogicExpr::Signal(s) => s.clone(),
        LogicExpr::And(l, r) => format!("({} && {})", emit_logic_expr(l), emit_logic_expr(r)),
        LogicExpr::Or(l, r) => format!("({} || {})", emit_logic_expr(l), emit_logic_expr(r)),
    }
}
