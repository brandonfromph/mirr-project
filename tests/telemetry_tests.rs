#[cfg(test)]
mod tests {
    use mirrc::pipeline::{run_pipeline, PipelineConfig};

    #[test]
    fn test_telemetry_node_density() {
        let source = "module m { signal x: in bool; guard g { when x for 1 cycles; } }";
        let config = PipelineConfig { typecheck: true, ..PipelineConfig::default() };

        let result = run_pipeline(source, &config).expect("Pipeline failed");

        // Phase 1: Verify node density tracking via parsed program stats
        assert!(
            !result.program.as_ref().unwrap().module.signals.is_empty(),
            "Module should have at least one signal"
        );
        println!(
            "[TELEMETRY] Node Density: {}",
            result.program.as_ref().unwrap().module.signals.len()
        );
    }

    #[test]
    fn test_telemetry_solver_rounds() {
        let source = "module m { signal x: out unsigned<8>; signal y: in bool; guard g { when y for 1 cycles; } reflex r { on g { x = 1; } } }";
        let config = PipelineConfig { width: true, ..PipelineConfig::default() };

        let result = run_pipeline(source, &config).expect("Pipeline failed");

        // Phase 1: Verify solver round tracking via width_result
        let solver_rounds = result.width_stats.as_ref().map(|w| w.propagation_rounds).unwrap_or(0);
        assert!(solver_rounds > 0, "Width solver rounds should be tracked");
        println!("[TELEMETRY] Solver Rounds: {}", solver_rounds);
    }
}
