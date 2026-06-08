#![forbid(unsafe_code)]

use mirrc::parser::parse_mirr;
use std::fs;
use std::path::Path;

#[test]
fn verify_stress_suite_compilation() {
    let suite_dir = Path::new("tests/stress_suite");
    if !suite_dir.exists() {
        return;
    }

    let mut count = 0;
    for entry in fs::read_dir(suite_dir).expect("Failed to read stress suite") {
        let entry = entry.expect("Invalid directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("mirr") {
            let source = fs::read_to_string(&path).expect("Failed to read mirr file");
            match parse_mirr(&source) {
                Ok(_) => {
                    count += 1;
                }
                Err(e) => {
                    panic!("Stress test failed to compile: {:?} - Error: {:?}", path, e);
                }
            }
        }
    }
    println!("Successfully verified {} stress tests.", count);
    assert!(count >= 100, "Expected at least 100 stress tests, found {}", count);
}
