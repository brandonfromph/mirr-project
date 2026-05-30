#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;
use std::process::Command;

fn wasm_source() -> String {
    let path = Path::new("crates/mirr-wasm/src/lib.rs");
    fs::read_to_string(path).expect("mirr-wasm lib.rs must be readable")
}

#[test]
fn rwfi2_wasm_exports_parity_entrypoints() {
    let src = wasm_source();
    assert!(src.contains("pub fn compile_target"));
    assert!(src.contains("pub fn compile_verilog_with_options"));
    assert!(src.contains("pub fn compile_json_netlist"));
}

#[test]
fn rwfi2_wasm_exports_mapek_and_cert_paths() {
    let src = wasm_source();
    assert!(src.contains("pub fn compile_mapek_rtl"));
    assert!(src.contains("pub fn compile_cert"));
}

#[test]
fn rwfi2_wasm_target_validation_contract_exists() {
    let src = wasm_source();
    assert!(src.contains("Valid targets: verilog, firrtl, rspu, json, sexpr, dot"));
}

#[test]
fn rwfi2_js_wrapper_compile_target_unknown_returns_structured_error() {
    if !Path::new("paper/demos/mirr_wasm_bg.wasm").exists() {
        println!("Skipping WASM wrapper test because mirr_wasm_bg.wasm is missing (expected on non-wasm-build CI runners).");
        return;
    }

    let script = r#"
        (async () => {
            const fs = require('fs');
            const wasm = await import('./paper/demos/mirr_wasm.js');
            await wasm.default(fs.readFileSync('./paper/demos/mirr_wasm_bg.wasm'));
            const raw = wasm.compile_target('', 'definitely_not_a_target');
            const parsed = JSON.parse(raw);

            if (parsed.type !== 'Err') {
                throw new Error('Expected Err envelope from compile_target shim');
            }
            if (!Array.isArray(parsed.errors) || parsed.errors.length === 0) {
                throw new Error('Expected non-empty structured errors array');
            }

            const first = parsed.errors[0] || {};
            if (!String(first.help || '').includes('Valid targets')) {
                throw new Error('Expected valid target help text in structured error');
            }
        })().catch((err) => {
            console.error(err && err.message ? err.message : String(err));
            process.exit(1);
        });
    "#;

    let output = Command::new("node")
        .arg("-e")
        .arg(script)
        .output()
        .expect("node must be available to validate paper wasm wrapper contract");

    assert!(
        output.status.success(),
        "wrapper contract script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn rwfi2_paper_consumer_supports_structured_wasm_envelopes() {
    let path = Path::new("paper/paper.js");
    let src = fs::read_to_string(path).expect("paper.js must be readable");

    assert!(
        src.contains("function normalizeCompilerResult"),
        "paper consumer must normalize compiler envelope"
    );
    assert!(
        src.contains("parsed.type === 'Ok'"),
        "paper consumer must handle structured Ok envelope"
    );
    assert!(
        src.contains("parsed.type === 'Err'"),
        "paper consumer must handle structured Err envelope"
    );
}
