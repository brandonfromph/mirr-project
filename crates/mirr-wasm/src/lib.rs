#![recursion_limit = "256"]
#![forbid(unsafe_code)]
#![deny(warnings)]

use wasm_bindgen::prelude::*;

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

/// Maximum source length accepted by the WASM API.
const MAX_SOURCE_BYTES: usize = 65_536;

fn ok_json(value: &str) -> String {
    serde_json::json!({"ok": value}).to_string()
}

fn err_json(value: &str) -> String {
    serde_json::json!({"err": value}).to_string()
}

fn default_config() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: true,
        sat_simplify: false,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        mape_k_partition: None,
        mape_k_ticks: None,
        retiming: false,
        totality: false,
        symbolic: false,
        emit_mape_k_rtl: false,
    }
}

fn check_length(source: &str) -> Result<(), String> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(format!(
            "Source exceeds maximum size: {} bytes (limit: {})",
            source.len(),
            MAX_SOURCE_BYTES
        ));
    }
    Ok(())
}

fn format_pipeline_errors(errors: &nasa_rust_project::error::PipelineErrors) -> String {
    errors.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
}

#[wasm_bindgen(start)]
pub fn wasm_init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn compile_verilog(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = default_config();
    match run_pipeline(source, &config) {
        Ok(result) => {
            let sv = nasa_rust_project::emit::verilog::emit_sv(&result);
            ok_json(&sv)
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn compile_firrtl(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = default_config();
    match run_pipeline(source, &config) {
        Ok(result) => {
            let firrtl = nasa_rust_project::emit::firrtl::emit_firrtl(&result);
            ok_json(&firrtl)
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn compile_sexpr(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = default_config();
    match run_pipeline(source, &config) {
        Ok(result) => {
            let sexpr = nasa_rust_project::emit::sexpr::emit_sexpr(&result);
            ok_json(&sexpr)
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn compile_dot(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = default_config();
    match run_pipeline(source, &config) {
        Ok(result) => {
            let dot = nasa_rust_project::emit::dot::emit_module_dot(&result);
            ok_json(&dot)
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn compile_rspu(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = PipelineConfig { rspu: true, temporal: true, ..default_config() };
    match run_pipeline(source, &config) {
        Ok(result) => match nasa_rust_project::emit::rspu::emit_rspu(&result) {
            Ok(program) => ok_json(&program.emit_asm()),
            Err(e) => err_json(&e.to_string()),
        },
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn compile_verilog_sat(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = PipelineConfig { sat_simplify: true, ..default_config() };
    match run_pipeline(source, &config) {
        Ok(result) => {
            let sv = nasa_rust_project::emit::verilog::emit_sv(&result);
            ok_json(&sv)
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn simulate_waveform(source: &str, cycles: u32) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let capped_cycles = cycles.min(1024);
    let config = PipelineConfig { temporal: true, ..default_config() };
    match run_pipeline(source, &config) {
        Ok(result) => {
            use nasa_rust_project::temporal::low_level_ir::CompiledGuard;

            let module = &result.program.module;
            let mut signals = Vec::new();

            // 1. Input signals — simulate with a deterministic pattern based on signal index
            for (si, sig) in module.signals.iter().enumerate() {
                if !matches!(sig.kind, nasa_rust_project::ast::types::SignalKind::Input) {
                    continue;
                }
                let is_bool =
                    matches!(sig.ty.core, nasa_rust_project::ast::types::SignalType::Bool);
                if is_bool {
                    // Bool inputs: toggle with a period derived from signal index
                    let period = (si as u32 + 2).min(64);
                    let values: Vec<u8> = (0..capped_cycles)
                        .map(|c| if (c / period) % 2 == 0 { 1u8 } else { 0u8 })
                        .collect();
                    signals.push(serde_json::json!({
                        "name": sig.name,
                        "width": 1,
                        "kind": "input",
                        "values": values,
                    }));
                } else {
                    // Numeric inputs: ramp from 0 with wrap at a boundary
                    let max_val: u64 = match &sig.ty.core {
                        nasa_rust_project::ast::types::SignalType::Unsigned(w) => {
                            1u64.checked_shl(*w).unwrap_or(256).saturating_sub(1)
                        }
                        _ => 255,
                    };
                    let values: Vec<u64> = (0..capped_cycles)
                        .map(|c| (c as u64 * (si as u64 + 1)) % (max_val + 1))
                        .collect();
                    signals.push(serde_json::json!({
                        "name": sig.name,
                        "width": match &sig.ty.core {
                            nasa_rust_project::ast::types::SignalType::Unsigned(w) => *w,
                            _ => 8,
                        },
                        "kind": "input",
                        "values": values,
                    }));
                }
            }

            // 2. Guard output signals — actual temporal behavior from compiled guards
            if let Some(ref nl) = result.temporal_netlist {
                for guard in &nl.guards {
                    let (name, delay) = match guard {
                        CompiledGuard::ShiftRegister(sr) => (&sr.output_signal, sr.delay_cycles),
                        CompiledGuard::Counter(c) => (&c.output_signal, c.target_count),
                        CompiledGuard::Complex(cx) => (&cx.output_signal, 1),
                        CompiledGuard::DynamicCounter(dc) => {
                            (&dc.output_signal, dc.max_delay.min(64))
                        }
                    };
                    // Guard activates after `delay` cycles of condition being true,
                    // then stays active until condition goes false for `delay` cycles.
                    // Simulate with condition assumed true from cycle 0.
                    let values: Vec<u8> = (0..capped_cycles)
                        .map(|c| if c as u64 >= delay { 1u8 } else { 0u8 })
                        .collect();
                    signals.push(serde_json::json!({
                        "name": name,
                        "width": 1,
                        "kind": "guard",
                        "delay_cycles": delay,
                        "values": values,
                    }));
                }
            }

            // 3. Output signals — driven by reflexes, active when their guard is active
            for sig in &module.signals {
                if !matches!(sig.kind, nasa_rust_project::ast::types::SignalKind::Output) {
                    continue;
                }
                // Find the earliest guard that drives this output via a reflex
                let mut earliest_delay: Option<u64> = None;
                for reflex in &module.reflexes {
                    let drives_this = reflex.assignments.iter().any(|a| a.target == sig.name);
                    if !drives_this {
                        continue;
                    }
                    if let Some(ref nl) = result.temporal_netlist {
                        for gn in &reflex.guard_names {
                            for guard in &nl.guards {
                                let (guard_name, delay) = match guard {
                                    CompiledGuard::ShiftRegister(sr) => (&sr.name, sr.delay_cycles),
                                    CompiledGuard::Counter(c) => (&c.name, c.target_count),
                                    CompiledGuard::Complex(cx) => (&cx.name, 1u64),
                                    CompiledGuard::DynamicCounter(dc) => {
                                        (&dc.name, dc.max_delay.min(64))
                                    }
                                };
                                if guard_name == gn {
                                    earliest_delay = Some(match earliest_delay {
                                        Some(d) => d.min(delay),
                                        None => delay,
                                    });
                                }
                            }
                        }
                    }
                }
                let delay = earliest_delay.unwrap_or(0);
                let is_bool =
                    matches!(sig.ty.core, nasa_rust_project::ast::types::SignalType::Bool);
                if is_bool {
                    let values: Vec<u8> = (0..capped_cycles)
                        .map(|c| if c as u64 >= delay { 1u8 } else { 0u8 })
                        .collect();
                    signals.push(serde_json::json!({
                        "name": sig.name,
                        "width": 1,
                        "kind": "output",
                        "values": values,
                    }));
                } else {
                    let values: Vec<u64> = (0..capped_cycles)
                        .map(|c| if c as u64 >= delay { 1u64 } else { 0u64 })
                        .collect();
                    signals.push(serde_json::json!({
                        "name": sig.name,
                        "width": match &sig.ty.core {
                            nasa_rust_project::ast::types::SignalType::Unsigned(w) => *w,
                            _ => 8,
                        },
                        "kind": "output",
                        "values": values,
                    }));
                }
            }

            let waveform = serde_json::json!({
                "total_cycles": capped_cycles,
                "signals": signals,
            });
            ok_json(&waveform.to_string())
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn compile_graph_data(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = default_config();
    match run_pipeline(source, &config) {
        Ok(result) => {
            let mut nodes = Vec::new();
            let mut edges = Vec::new();

            let module = &result.program.module;
            for sig in &module.signals {
                let node_type = match &sig.kind {
                    nasa_rust_project::ast::types::SignalKind::Input => "Input",
                    nasa_rust_project::ast::types::SignalKind::Output => "Output",
                    nasa_rust_project::ast::types::SignalKind::Internal => "Internal",
                };
                nodes.push(serde_json::json!({
                    "id": sig.name,
                    "label": sig.name,
                    "type": node_type,
                }));
            }

            for guard in &module.guards {
                nodes.push(serde_json::json!({
                    "id": guard.name,
                    "label": guard.name,
                    "type": "Guard",
                }));
            }

            for reflex in &module.reflexes {
                for gn in &reflex.guard_names {
                    for assign in &reflex.assignments {
                        edges.push(serde_json::json!({
                            "from": gn,
                            "to": assign.target,
                            "label": "triggers",
                        }));
                    }
                }
            }

            let graph = serde_json::json!({ "nodes": nodes, "edges": edges });
            ok_json(&graph.to_string())
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn infer_widths(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = default_config();
    match run_pipeline(source, &config) {
        Ok(result) => match nasa_rust_project::emit::json_netlist::emit_json(&result) {
            Ok(json_str) => ok_json(&json_str),
            Err(e) => err_json(&e.to_string()),
        },
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn simulate_rspu(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = PipelineConfig { rspu: true, temporal: true, simulate: true, ..default_config() };
    match run_pipeline(source, &config) {
        Ok(result) => match result.sim_result {
            Some(ref sim) => {
                let mut text = String::new();
                text.push_str("=== R-SPU Simulation Results ===\n\n");
                text.push_str(&format!("Cycles executed: {}\n", sim.cycles));
                text.push_str(&format!(
                    "Status: {}\n\n",
                    if sim.halted {
                        "Halted (normal)"
                    } else if sim.exception.is_some() {
                        "Exception"
                    } else {
                        "Completed"
                    }
                ));

                if !sim.outputs.is_empty() {
                    text.push_str("Output Ports:\n");
                    let mut ports: Vec<_> = sim.outputs.iter().collect();
                    ports.sort_by_key(|(k, _)| *k);
                    for (port_id, tagged_word) in &ports {
                        text.push_str(&format!(
                            "  Port {:>3}: value={:<10} tag={}\n",
                            port_id, tagged_word.value, tagged_word.tag
                        ));
                    }
                    text.push('\n');
                }

                if !sim.property_violations.is_empty() {
                    text.push_str(&format!(
                        "Property Violations: {}\n",
                        sim.property_violations
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                } else {
                    text.push_str("Property Violations: none\n");
                }

                if let Some(ref exc) = sim.exception {
                    text.push_str(&format!("Exception: {exc:?}\n"));
                }

                ok_json(&text)
            }
            None => err_json("No R-SPU simulation result produced. Ensure source compiles to R-SPU instructions."),
        },
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn simulate_mapek(source: &str, ticks: u32) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = PipelineConfig {
        temporal: true,
        mape_k: true,
        mape_k_ticks: Some(ticks.min(10_000)),
        ..default_config()
    };
    match run_pipeline(source, &config) {
        Ok(result) => match &result.mape_k_result {
            Some(res) => match serde_json::to_string(res) {
                Ok(json) => ok_json(&json),
                Err(e) => err_json(&e.to_string()),
            },
            None => err_json("MAPE-K simulation produced no result"),
        },
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn mirr_version() -> String {
    ok_json(env!("CARGO_PKG_VERSION"))
}

#[wasm_bindgen]
pub fn compile_pipeline_stages(source: &str) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    let config = default_config();
    match run_pipeline(source, &config) {
        Ok(result) => {
            let stages = serde_json::json!({
                "parsed": true,
                "validated": true,
                "expanded": true,
                "simplified": result.simplify_stats.is_some(),
                "typechecked": result.type_map.is_some(),
                "width_inferred": result.width_result.is_some(),
                "temporal_lowered": result.temporal_netlist.is_some(),
                "emitted": true,
            });
            stages.to_string()
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn proof_status() -> String {
    // Build-time snapshot — WASM cannot read .v files at runtime.
    // Real Coq theorem/lemma/corollary names from proofs/, verified 2026-03-14.
    // 12 proof files, 55 theorems/lemmas/corollaries, 3 Admitted.
    serde_json::json!({
        "ok": [
            { "name": "add_sound", "file": "Constraint.v", "kind": "Theorem", "status": "Proven" },
            { "name": "mul_sound", "file": "Constraint.v", "kind": "Theorem", "status": "Proven" },
            { "name": "sub_sound", "file": "Constraint.v", "kind": "Theorem", "status": "Proven" },
            { "name": "shift_sound", "file": "Constraint.v", "kind": "Theorem", "status": "Proven" },
            { "name": "negate_unsigned_sound", "file": "Constraint.v", "kind": "Theorem", "status": "Proven" },
            { "name": "flatten_postorder", "file": "Flatten.v", "kind": "Theorem", "status": "Proven" },
            { "name": "no_self_reference", "file": "Flatten.v", "kind": "Corollary", "status": "Proven" },
            { "name": "e2e_solver_sound", "file": "Integration.v", "kind": "Theorem", "status": "Proven" },
            { "name": "min_bits_0", "file": "MinBits.v", "kind": "Lemma", "status": "Proven" },
            { "name": "min_bits_S", "file": "MinBits.v", "kind": "Lemma", "status": "Proven" },
            { "name": "div2_lt_n", "file": "MinBits.v", "kind": "Lemma", "status": "Proven" },
            { "name": "le_double_div2", "file": "MinBits.v", "kind": "Lemma", "status": "Proven" },
            { "name": "min_bits_correct", "file": "MinBits.v", "kind": "Theorem", "status": "Proven" },
            { "name": "min_bits_minimal", "file": "MinBits.v", "kind": "Theorem", "status": "Admitted" },
            { "name": "state_le_refl", "file": "Monotone.v", "kind": "Lemma", "status": "Proven" },
            { "name": "state_le_trans", "file": "Monotone.v", "kind": "Lemma", "status": "Proven" },
            { "name": "lookup_monotone", "file": "Monotone.v", "kind": "Lemma", "status": "Proven" },
            { "name": "monotonicity", "file": "Monotone.v", "kind": "Theorem", "status": "Proven" },
            { "name": "update_preserves_le", "file": "Monotone.v", "kind": "Lemma", "status": "Proven" },
            { "name": "one_step_monotone", "file": "Monotone.v", "kind": "Lemma", "status": "Proven" },
            { "name": "evaluate_monotone", "file": "Monotone.v", "kind": "Theorem", "status": "Proven" },
            { "name": "classify_sound", "file": "SCC/Classify.v", "kind": "Theorem", "status": "Proven" },
            { "name": "nonexpansive_bounded", "file": "SCC/Classify.v", "kind": "Corollary", "status": "Proven" },
            { "name": "nonexpansive_convergence", "file": "SCC/Nonexpansive.v", "kind": "Theorem", "status": "Proven" },
            { "name": "lookup_le_fold_max", "file": "SCC/Nonexpansive.v", "kind": "Lemma", "status": "Proven" },
            { "name": "exists_pos_implies_max_ge_1", "file": "SCC/Nonexpansive.v", "kind": "Lemma", "status": "Proven" },
            { "name": "nonexpansive_max_bound", "file": "SCC/Nonexpansive.v", "kind": "Lemma", "status": "Proven" },
            { "name": "tarjan_correct", "file": "SCC/Tarjan.v", "kind": "Theorem", "status": "Proven" },
            { "name": "solver_terminates", "file": "Solver.v", "kind": "Theorem", "status": "Admitted" },
            { "name": "lookup_update_same", "file": "Solver.v", "kind": "Lemma", "status": "Proven" },
            { "name": "lookup_update_other", "file": "Solver.v", "kind": "Lemma", "status": "Proven" },
            { "name": "update_le_preserves", "file": "Solver.v", "kind": "Lemma", "status": "Proven" },
            { "name": "update_both_monotone", "file": "Solver.v", "kind": "Lemma", "status": "Proven" },
            { "name": "eval_none_propagates", "file": "Solver.v", "kind": "Lemma", "status": "Proven" },
            { "name": "step_one_monotone", "file": "Solver.v", "kind": "Lemma", "status": "Admitted" },
            { "name": "apply_constraints_state_monotone", "file": "Solver.v", "kind": "Lemma", "status": "Proven" },
            { "name": "apply_constraints_monotone_fixpoint", "file": "Solver.v", "kind": "Lemma", "status": "Proven" },
            { "name": "fixpoint_least", "file": "Solver.v", "kind": "Theorem", "status": "Proven" },
            { "name": "truncation_correct_positive", "file": "Truncation.v", "kind": "Theorem", "status": "Proven" },
            { "name": "truncation_correct_negative", "file": "Truncation.v", "kind": "Theorem", "status": "Proven" },
            { "name": "truncation_dec", "file": "Truncation.v", "kind": "Lemma", "status": "Proven" },
            { "name": "bounded_testbit", "file": "rspu/Encoding.v", "kind": "Lemma", "status": "Proven" },
            { "name": "s_type_imm_roundtrip", "file": "rspu/Encoding.v", "kind": "Theorem", "status": "Proven" },
            { "name": "opcode_roundtrip", "file": "rspu/Encoding.v", "kind": "Theorem", "status": "Proven" },
            { "name": "r_type_dst_roundtrip", "file": "rspu/Encoding.v", "kind": "Theorem", "status": "Proven" },
            { "name": "r_type_src1_roundtrip", "file": "rspu/Encoding.v", "kind": "Theorem", "status": "Proven" },
            { "name": "r_type_funct_roundtrip", "file": "rspu/Encoding.v", "kind": "Theorem", "status": "Proven" },
            { "name": "i_type_imm_roundtrip", "file": "rspu/Encoding.v", "kind": "Theorem", "status": "Proven" },
            { "name": "tags_compatible_sym", "file": "rspu/TaggedWord.v", "kind": "Theorem", "status": "Proven" },
            { "name": "mov_preserves_tag", "file": "rspu/TaggedWord.v", "kind": "Theorem", "status": "Proven" },
            { "name": "mov_preserves_value", "file": "rspu/TaggedWord.v", "kind": "Theorem", "status": "Proven" },
            { "name": "load_imm_tag", "file": "rspu/TaggedWord.v", "kind": "Theorem", "status": "Proven" },
            { "name": "load_imm_value", "file": "rspu/TaggedWord.v", "kind": "Theorem", "status": "Proven" },
            { "name": "uninitialized_not_initialized", "file": "rspu/TaggedWord.v", "kind": "Theorem", "status": "Proven" },
            { "name": "initialized_after_load", "file": "rspu/TaggedWord.v", "kind": "Theorem", "status": "Proven" }
        ],
        "snapshot": "build-time",
        "note": "Static snapshot compiled into WASM. Cannot verify proofs at runtime.",
        "total_theorems": 55,
        "mechanized": 52,
        "admitted": 3,
        "mechanization_rate": "94.5%",
        "proof_files": 12,
        "admitted_proofs": [
            "min_bits_minimal (MinBits.v — recursive corner case)",
            "solver_terminates (Solver.v — potential function argument)",
            "step_one_monotone (Solver.v — length obligation)"
        ]
    })
    .to_string()
}
