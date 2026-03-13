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
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
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
            Some(ref sim) => match serde_json::to_string(sim) {
                Ok(json) => ok_json(&json),
                Err(e) => err_json(&e.to_string()),
            },
            None => err_json("No R-SPU simulation result produced"),
        },
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn simulate_mapek(source: &str, ticks: u32) -> String {
    if let Err(msg) = check_length(source) {
        return err_json(&msg);
    }
    // TODO: Wire ticks to MAPE-K simulation when PipelineConfig supports it
    let _ = ticks;
    let config = PipelineConfig { temporal: true, mape_k: true, ..default_config() };
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
                "simplified": result.simplify_stats.is_some(),
                "width_inferred": result.width_result.is_some(),
                "temporal_lowered": result.temporal_netlist.is_some(),
            });
            stages.to_string()
        }
        Err(errors) => err_json(&format_pipeline_errors(&errors)),
    }
}

#[wasm_bindgen]
pub fn proof_status() -> String {
    serde_json::json!({
        "total_theorems": 55,
        "mechanized": 53,
        "admitted": 2,
        "mechanization_rate": "96.4%",
        "proof_files": 14,
        "proof_lines": 1833,
        "admitted_proofs": ["solver_terminates", "step_one_monotone"]
    })
    .to_string()
}
