#![forbid(unsafe_code)]

#[path = "../src/bin/mirr_general/parity.rs"]
mod parity;

use parity::{run_consumer_parity, ParityRecord, ParitySubsystem};
use std::process::Command;

#[test]
fn run_consumer_parity_returns_err_when_any_record_has_success_false() {
    let records = [ParityRecord {
        subsystem: ParitySubsystem::CliVsWasm,
        success: false,
        detail: "broken".to_string(),
    }];

    assert!(run_consumer_parity(&records).is_err());
}

#[test]
fn run_consumer_parity_returns_ok_when_all_records_have_success_true() {
    let records = [
        ParityRecord {
            subsystem: ParitySubsystem::CliVsWasm,
            success: true,
            detail: "ok-1".to_string(),
        },
        ParityRecord {
            subsystem: ParitySubsystem::CompilerVsVscode,
            success: true,
            detail: "ok-2".to_string(),
        },
    ];

    assert!(run_consumer_parity(&records).is_ok());
}

#[test]
fn parity_record_carries_detail_string_verbatim() {
    let detail = String::from("detail text is preserved verbatim");
    let record = ParityRecord {
        subsystem: ParitySubsystem::CompilerVsVscode,
        success: true,
        detail: detail.clone(),
    };

    assert_eq!(record.detail, detail);
}

#[test]
fn parity_subsystem_variants_are_distinct_when_debug_formatted() {
    let _ = parity::verify_cli_wasm_parity as fn(&std::path::Path) -> std::io::Result<ParityRecord>;
    let _ = parity::verify_vscode_contract as fn() -> std::io::Result<ParityRecord>;

    let cli = format!("{:?}", ParitySubsystem::CliVsWasm);
    let vscode = format!("{:?}", ParitySubsystem::CompilerVsVscode);

    assert_ne!(cli, vscode);
}

#[test]
fn single_failing_record_in_list_causes_err_regardless_of_position() {
    let first_fail = [
        ParityRecord {
            subsystem: ParitySubsystem::CliVsWasm,
            success: false,
            detail: "first".to_string(),
        },
        ParityRecord {
            subsystem: ParitySubsystem::CompilerVsVscode,
            success: true,
            detail: "second".to_string(),
        },
    ];
    let second_fail = [
        ParityRecord {
            subsystem: ParitySubsystem::CliVsWasm,
            success: true,
            detail: "first".to_string(),
        },
        ParityRecord {
            subsystem: ParitySubsystem::CompilerVsVscode,
            success: false,
            detail: "second".to_string(),
        },
    ];

    assert!(run_consumer_parity(&first_fail).is_err());
    assert!(run_consumer_parity(&second_fail).is_err());
}

#[test]
fn empty_record_list_returns_ok() {
    let records: [ParityRecord; 0] = [];

    assert!(run_consumer_parity(&records).is_ok());
}

#[test]
fn parity_router_rejects_bare_run_parity_without_all_flag() {
    let binary = env!("CARGO_BIN_EXE_mirr-general");
    let output = Command::new(binary)
        .args(["run", "parity"])
        .output()
        .expect("mirr-general binary must run");

    assert!(!output.status.success());
}
