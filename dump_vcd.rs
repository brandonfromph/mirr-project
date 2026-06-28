use std::path::Path;
mod diagnostic {
    pub mod vcd_parser;
}

fn main() {
    let path = Path::new("reflex_soc/reflex_soc_bmc/engine_0/trace.vcd");
    match diagnostic::vcd_parser::parse_vcd_state_at_step(path, Some(3)) {
        Ok(state) => {
            for (k, v) in state {
                if k.contains("id_ex_op") || k.contains("trap") || k.contains("is_invalid") || k.contains("rx_valid") || k.contains("current_instr") || k.contains("tx_valid") {
                    println!("{}: {}", k, v);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
