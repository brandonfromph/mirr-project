use crate::ast::program::TargetConfig;

/// Generates a Synopsys Design Constraints (SDC) file for the OpenLANE physical design flow.
/// The constraints enforce zero-jitter, hard real-time requirements for the R-SPU.
pub fn generate_sdc_config(_target: Option<&TargetConfig>) -> String {
    // We currently default to a 10.0ns (100MHz) clock.
    // In the future, this can be parsed directly from the TargetConfig
    // if a `clock_period` parameter is added to the Liquid Target Profile syntax.
    let clock_period = 10.0;

    format!(
        r#"# Auto-generated SDC for MIRR R-SPU OpenLANE Tape-out
# Enforcing zero-jitter spatial hardware timing

create_clock -name clk -period {clock_period} [get_ports clk]

# Standard input/output delays (20% of clock period)
set_input_delay  2.0 -clock clk [all_inputs]
set_output_delay 2.0 -clock clk [all_outputs]

# Prevent hold violations on the reset line
set_false_path -from [get_ports rst_n]
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sdc_config() {
        let sdc = generate_sdc_config(None);
        assert!(sdc.contains("create_clock -name clk -period 10"));
        assert!(sdc.contains("set_input_delay"));
    }
}
