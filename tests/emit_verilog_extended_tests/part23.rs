use super::*;

// Section 23: Output Line Count Sanity
// ===========================================================================

#[test]
fn output_has_reasonable_line_count() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);
    let lines = count_lines_bounded(&sv);

    assert!(lines > 5, "minimal module output must have at least 5 lines, got {}", lines);
    assert!(
        lines < MAX_OUTPUT_LINES,
        "output must not exceed {} lines, got {}",
        MAX_OUTPUT_LINES,
        lines
    );
}

#[test]
fn property_heavy_output_bounded() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);
    let lines = count_lines_bounded(&sv);

    assert!(lines > 20, "property-heavy module must have substantial output, got {} lines", lines);
    assert!(lines < MAX_OUTPUT_LINES, "output must stay within bounds, got {} lines", lines);
}

// ===========================================================================