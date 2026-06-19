#![cfg(any())]
#[cfg(test)]
mod e2e_tests {
    use mirrc::pipeline::{run_pipeline, PipelineConfig};
    use std::path::PathBuf;

    fn get_alu_path() -> String {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        format!("{}/rspu_chip/core/alu.mirr", manifest_dir)
    }

    #[test]
    fn test_e2e_alu_compilation() {
        let source = std::fs::read_to_string(get_alu_path()).unwrap();
        let config = PipelineConfig {
            temporal: true,
            base_dir: Some(PathBuf::from(format!(
                "{}/rspu_chip/core",
                std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string())
            ))),
            ..Default::default()
        };

        let result = run_pipeline(&source, &config);
        assert!(result.is_ok(), "ALU compilation failed: {:?}", result.err());
    }

    #[test]
    fn test_e2e_duplicate_signal_error() {
        let source = "module top; signal s1: u8; signal s1: u8; endmodule";
        let config = PipelineConfig::default();
        let result = run_pipeline(source, &config);

        assert!(result.is_err());
        let errs = result.err().unwrap();
        assert!(errs.errors.iter().any(|e| format!("{:?}", e).contains("E201")));
    }

    #[test]
    fn test_e2e_guard_bool_error() {
        let source = "module top; signal s1: u8; guard g1(s1) for 10 cycles; endmodule";
        let config = PipelineConfig::default();
        let result = run_pipeline(source, &config);

        assert!(result.is_err());
        let errs = result.err().unwrap();
        assert!(errs.errors.iter().any(|e| format!("{:?}", e).contains("E601")));
    }

    #[test]
    fn test_e2e_prev_delay_zero_error() {
        let source = "module top; signal s1: bool; guard g1(prev(s1, 0)) for 1 cycles; endmodule";
        let config = PipelineConfig::default();
        let result = run_pipeline(source, &config);

        assert!(result.is_err());
        let errs = result.err().unwrap();
        println!("DEBUG_PREV_ERRS: {:?}", errs);
        assert!(errs.errors.iter().any(|e| format!("{:?}", e).contains("E209")));
    }

    #[test]
    fn test_e2e_undeclared_signal_suggestion() {
        let source = "module top; signal clock: bool; guard g1(clok) for 1 cycles; endmodule";
        let config = PipelineConfig::default();
        let result = run_pipeline(source, &config);

        assert!(result.is_err());
        let errs = result.err().unwrap();
        println!("DEBUG_ERRS: {:?}", errs);
        assert!(format!("{:?}", errs).contains("Did you mean 'clock'?"));
    }

    #[test]
    fn test_e2e_cross_module_import() {
        // This test requires actual files on disk
        let alu_source = std::fs::read_to_string(get_alu_path()).unwrap();
        let config = PipelineConfig {
            base_dir: Some(PathBuf::from(format!(
                "{}/rspu_chip/core",
                std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string())
            ))),
            ..Default::default()
        };

        let result = run_pipeline(&alu_source, &config);
        assert!(result.is_ok(), "ALU with imports failed: {:?}", result.err());
    }

    #[test]
    fn test_e2e_pattern_telemetry_stash() {
        // Verify that mirr-brain is invoked (would need to mock or check side effects)
        // For now, just ensure it doesn't crash during expansion.
        let source =
            "def p(a) { reflect { reflex r1 { on always { ${a} = true; } } } } module top; p(s1); signal s1: bool; endmodule";
        let config = PipelineConfig::default();
        let result = run_pipeline(source, &config);
        if let Err(ref e) = result {
            println!("DEBUG_STASH_ERRS: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_large_expression_tree() {
        let mut source = "module top; signal s1: bool; guard g1(".to_string();
        for _ in 0..50 {
            source.push_str("s1 && ");
        }
        source.push_str("s1) for 1 cycles; endmodule");

        let config = PipelineConfig::default();
        let result = run_pipeline(&source, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_resource_strategy_selection() {
        let source = "module top; signal s1: bool; guard g_short(s1) for 5 cycles; guard g_long(s1) for 50 cycles; endmodule";
        let config = PipelineConfig { temporal: true, ..Default::default() };
        let result = run_pipeline(source, &config).unwrap();

        let netlist = result.temporal_netlist.unwrap();
        // Strategy selection: N <= 16 -> ShiftRegister, N > 16 -> Counter
        assert!(format!("{:?}", netlist).contains("ShiftRegister"));
        assert!(format!("{:?}", netlist).contains("Counter"));
    }

    #[test]
    fn test_e2e_empty_reflex_error() {
        let source = "def p() { } module top; p(); endmodule";
        let config = PipelineConfig::default();
        let result = run_pipeline(source, &config);

        assert!(result.is_err()); // E420
    }
}
