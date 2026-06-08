use mirrc::emit;
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ArsenalWasm {
    ir_version: String,
}

impl Default for ArsenalWasm {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl ArsenalWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { ir_version: "0.3.0".to_string() }
    }

    pub fn get_law(&self, category: &str) -> String {
        match category {
            "rspu" => format!(
                "IR {} | Instructions: 4096, Registers: 256, Opcode: 37 used",
                self.ir_version
            ),
            "p10" => "No recursion, No unsafe, Bounded loops".to_string(),
            _ => "Unknown category".to_string(),
        }
    }

    pub fn validate_wave_hash(&self, signed_hash: &str, actual_hash: &str) -> bool {
        signed_hash == actual_hash
    }

    // Compiler-backed validation path used by RWFI2 gates.
    pub fn validate_compile_contract(&self, source: &str, target: &str) -> String {
        let mut config = PipelineConfig::default();
        if target == "rspu" {
            config.rspu = true;
            config.temporal = true;
        }

        match run_pipeline(source, &config) {
            Ok(result) => {
                let rendered = match target {
                    "verilog" | "sv" => emit::verilog::emit_sv(&result),
                    "firrtl" => emit::firrtl::emit_firrtl(&result),
                    "json" => match emit::json_netlist::emit_json(&result) {
                        Ok(json) => json,
                        Err(e) => {
                            return serde_json::json!({
                                "ok": false,
                                "target": target,
                                "error": format!("JSON netlist serialization failed: {}", e),
                            })
                            .to_string();
                        }
                    },
                    "sexpr" | "s-expr" | "sexp" => emit::sexpr::emit_sexpr(&result),
                    "dot" => emit::dot::emit_module_dot(&result),
                    "rspu" => match &result.rspu_program {
                        Some(program) => program.emit_asm(),
                        None => {
                            return serde_json::json!({
                                "ok": false,
                                "target": target,
                                "error": "R-SPU output was not generated",
                            })
                            .to_string();
                        }
                    },
                    _ => {
                        return serde_json::json!({
                            "ok": false,
                            "target": target,
                            "error": format!(
                                "Unknown compile target: {}. Allowed targets: verilog, firrtl, rspu, json, sexpr, dot.",
                                target
                            ),
                            "valid_targets": ["verilog", "firrtl", "rspu", "json", "sexpr", "dot"],
                        })
                        .to_string();
                    }
                };

                serde_json::json!({
                    "ok": true,
                    "target": target,
                    "bytes": rendered.len(),
                    "ir_version": self.ir_version,
                })
                .to_string()
            }
            Err(errors) => {
                let diagnostics: Vec<String> = errors
                    .errors
                    .iter()
                    .map(|e| {
                        let d = e.to_diagnostic();
                        format!(
                            "{}: {}",
                            d.code.unwrap_or_else(|| "UNKNOWN".to_string()),
                            d.message
                        )
                    })
                    .collect();
                serde_json::json!({
                    "ok": false,
                    "target": target,
                    "errors": diagnostics,
                })
                .to_string()
            }
        }
    }
}
