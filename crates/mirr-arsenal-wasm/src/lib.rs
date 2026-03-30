use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ArsenalWasm {
    ir_version: String,
}

#[wasm_bindgen]
impl ArsenalWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            ir_version: "0.3.0".to_string(),
        }
    }

    pub fn get_law(&self, category: &str) -> String {
        match category {
            "rspu" => "Instructions: 4096, Registers: 256, Opcode: 37 used".to_string(),
            "p10" => "No recursion, No unsafe, Bounded loops".to_string(),
            _ => "Unknown category".to_string(),
        }
    }

    pub fn validate_wave_hash(&self, signed_hash: &str, actual_hash: &str) -> bool {
        signed_hash == actual_hash
    }
}
