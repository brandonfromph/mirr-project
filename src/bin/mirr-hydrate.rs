use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Yosys JSON netlist
    input: PathBuf,
    /// Path to output MIRR file
    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Deserialize, Debug)]
struct YosysNetlist {
    modules: HashMap<String, YosysModule>,
}

#[derive(Deserialize, Debug)]
struct YosysModule {
    ports: HashMap<String, YosysPort>,
    cells: HashMap<String, YosysCell>,
    netnames: HashMap<String, YosysNetname>,
}

#[derive(Deserialize, Debug)]
struct YosysPort {
    direction: String,
    bits: Vec<YosysBit>,
}

#[derive(Deserialize, Debug)]
struct YosysCell {
    #[serde(rename = "type")]
    cell_type: String,
    connections: HashMap<String, Vec<YosysBit>>,
}

#[derive(Deserialize, Debug)]
struct YosysNetname {
    bits: Vec<YosysBit>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
enum YosysBit {
    Constant(String),
    Index(u64, u64), // Used in some versions
    Number(u64),     // Used in others
}

fn sanitize_name(name: &str) -> String {
    name.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
        .trim_start_matches(|c: char| !c.is_alphabetic() && c != '_')
        .to_string()
}

fn bit_to_str(bit: &YosysBit, bit_to_sig: &HashMap<YosysBit, (String, usize)>) -> String {
    match bit {
        YosysBit::Constant(c) => match c.as_str() {
            "0" => "0".to_string(),
            "1" => "1".to_string(),
            _ => "0".to_string(),
        },
        _ => {
            if let Some((sig_name, idx)) = bit_to_sig.get(bit) {
                if *idx == 0 && is_scalar(bit, bit_to_sig) {
                    sig_name.clone()
                } else {
                    format!("{}[{}]", sig_name, idx)
                }
            } else {
                "0".to_string()
            }
        }
    }
}

fn is_scalar(bit: &YosysBit, bit_to_sig: &HashMap<YosysBit, (String, usize)>) -> bool {
    bit_to_sig.contains_key(bit)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let content = fs::read_to_string(&args.input)?;
    let netlist: YosysNetlist = serde_json::from_str(&content)?;

    let mut mirr_output = String::new();
    mirr_output.push_str("// Auto-generated MIRR from Yosys JSON (Vector-Corrected)\n\n");

    for (mod_name, module) in netlist.modules {
        mirr_output.push_str(&format!("module {} {{\n", sanitize_name(&mod_name)));

        let mut signal_widths: HashMap<String, usize> = HashMap::new();
        let mut bit_to_sig: HashMap<YosysBit, (String, usize)> = HashMap::new();
        let mut signal_is_port: HashMap<String, bool> = HashMap::new();

        // 1. Map all bits to signals
        for (port_name, port) in &module.ports {
            let name = sanitize_name(port_name);
            signal_widths.insert(name.clone(), port.bits.len());
            signal_is_port.insert(name.clone(), true);
            for (idx, bit) in port.bits.iter().enumerate() {
                bit_to_sig.insert(bit.clone(), (name.clone(), idx));
            }
            let dir = if port.direction == "input" { "in" } else { "out" };
            let ty = if port.bits.len() == 1 {
                "bool".to_string()
            } else {
                format!("u{}", port.bits.len())
            };
            mirr_output.push_str(&format!("    signal {}: {} {};\n", name, dir, ty));
        }

        for (net_name, net) in &module.netnames {
            let name = sanitize_name(net_name);
            if signal_widths.contains_key(&name) {
                continue;
            }
            if net_name.starts_with('$') && !net_name.contains("src") {
                continue;
            }

            signal_widths.insert(name.clone(), net.bits.len());
            for (idx, bit) in net.bits.iter().enumerate() {
                bit_to_sig.insert(bit.clone(), (name.clone(), idx));
            }
            let ty = if net.bits.len() == 1 {
                "bool".to_string()
            } else {
                format!("u{}", net.bits.len())
            };
            mirr_output.push_str(&format!("    signal {}: {};\n", name, ty));
        }

        // 2. Collect drivers for each bit
        let mut comb_drivers: HashMap<String, Vec<Option<String>>> = HashMap::new();
        let mut sync_drivers: HashMap<String, Vec<Option<String>>> = HashMap::new();

        for (name, width) in &signal_widths {
            comb_drivers.insert(name.clone(), vec![None; *width]);
            sync_drivers.insert(name.clone(), vec![None; *width]);
        }

        for (_, cell) in module.cells {
            match cell.cell_type.as_str() {
                "$add" | "$and" | "$or" | "$xor" | "$sub" | "$shl" | "$shr" | "$_AND_"
                | "$_OR_" | "$_XOR_" => {
                    let op = match cell.cell_type.as_str() {
                        "$add" => "+",
                        "$and" | "$_AND_" => "&&",
                        "$or" | "$_OR_" => "||",
                        "$xor" | "$_XOR_" => "^",
                        "$sub" => "-",
                        "$shl" => "<<",
                        "$shr" => ">>",
                        _ => unreachable!(),
                    };
                    let y_bits = &cell.connections["Y"];
                    let a_bits = &cell.connections["A"];
                    let b_bits = &cell.connections["B"];

                    for (i, y_bit) in y_bits.iter().enumerate() {
                        if let Some((sig, idx)) = bit_to_sig.get(y_bit) {
                            let a = bit_to_str(&a_bits[i.min(a_bits.len() - 1)], &bit_to_sig);
                            let b = bit_to_str(&b_bits[i.min(b_bits.len() - 1)], &bit_to_sig);
                            comb_drivers.get_mut(sig).unwrap()[*idx] =
                                Some(format!("({} {} {})", a, op, b));
                        }
                    }
                }
                "$not" | "$logic_not" | "$_NOT_" => {
                    let y_bits = &cell.connections["Y"];
                    let a_bits = &cell.connections["A"];
                    for (i, y_bit) in y_bits.iter().enumerate() {
                        if let Some((sig, idx)) = bit_to_sig.get(y_bit) {
                            let a = bit_to_str(&a_bits[i.min(a_bits.len() - 1)], &bit_to_sig);
                            comb_drivers.get_mut(sig).unwrap()[*idx] = Some(format!("!{}", a));
                        }
                    }
                }
                "$mux" | "$_MUX_" => {
                    let y_bits = &cell.connections["Y"];
                    let a_bits = &cell.connections["A"];
                    let b_bits = &cell.connections["B"];
                    let s_bits = &cell.connections["S"];
                    for (i, y_bit) in y_bits.iter().enumerate() {
                        if let Some((sig, idx)) = bit_to_sig.get(y_bit) {
                            let a = bit_to_str(&a_bits[i.min(a_bits.len() - 1)], &bit_to_sig);
                            let b = bit_to_str(&b_bits[i.min(b_bits.len() - 1)], &bit_to_sig);
                            let s = bit_to_str(&s_bits[0], &bit_to_sig);
                            comb_drivers.get_mut(sig).unwrap()[*idx] =
                                Some(format!("({} && {}) || (!{} && {})", s, b, s, a));
                        }
                    }
                }
                "$dff" | "$dffe" | "$sdff" | "$sdffe" | "$adff" | "$_DFF_P_" | "$_DFF_N_"
                | "$_DFF_PP0_" | "$_DFF_PP1_" | "$_DFF_PN0_" | "$_DFF_PN1_" | "$_DFF_NP0_"
                | "$_DFF_NP1_" | "$_DFF_NN0_" | "$_DFF_NN1_" => {
                    let q_bits = &cell.connections["Q"];
                    let d_bits = &cell.connections["D"];
                    for (i, q_bit) in q_bits.iter().enumerate() {
                        if let Some((sig, idx)) = bit_to_sig.get(q_bit) {
                            let d = bit_to_str(&d_bits[i.min(d_bits.len() - 1)], &bit_to_sig);
                            sync_drivers.get_mut(sig).unwrap()[*idx] = Some(d);
                        }
                    }
                }
                _ => {}
            }
        }

        // 3. Emit reconstructions
        let clk_name = signal_widths
            .keys()
            .find(|k| {
                let kl = k.to_lowercase();
                kl == "clk" || kl == "clock" || kl == "sys_clk"
            })
            .cloned();

        if let Some(ref clk_sig) = clk_name {
            mirr_output
                .push_str(&format!("\n    guard g_clk {{ when {} for 1 cycles; }}\n", clk_sig));
        }

        let mut comb_body = String::new();
        for (name, drivers) in &comb_drivers {
            if signal_is_port.get(name) == Some(&true)
                && module.ports.get(name).unwrap().direction == "input"
            {
                continue;
            }
            if drivers.iter().all(|d| d.is_none()) {
                continue;
            }

            let mut expr_parts = Vec::new();
            for (i, driver) in drivers.iter().enumerate() {
                if let Some(expr) = driver {
                    if i == 0 {
                        expr_parts.push(expr.clone());
                    } else {
                        expr_parts.push(format!("({} << {})", expr, i));
                    }
                }
            }
            if !expr_parts.is_empty() {
                let combined = expr_parts.join(" || ");
                comb_body.push_str(&format!("            {} = {};\n", name, combined));
            }
        }
        if !comb_body.is_empty() {
            mirr_output.push_str("\n    reflex comb_logic {\n        on always {\n");
            mirr_output.push_str(&comb_body);
            mirr_output.push_str("        }\n    }\n");
        }

        let mut sync_body = String::new();
        for (name, drivers) in &sync_drivers {
            if drivers.iter().all(|d| d.is_none()) {
                continue;
            }

            let mut expr_parts = Vec::new();
            for (i, driver) in drivers.iter().enumerate() {
                if let Some(expr) = driver {
                    if i == 0 {
                        expr_parts.push(expr.clone());
                    } else {
                        expr_parts.push(format!("({} << {})", expr, i));
                    }
                }
            }
            if !expr_parts.is_empty() {
                let combined = expr_parts.join(" || ");
                sync_body.push_str(&format!("            {} = {};\n", name, combined));
            }
        }
        if clk_name.is_some() && !sync_body.is_empty() {
            mirr_output.push_str("\n    reflex sync_logic {\n        on g_clk {\n");
            mirr_output.push_str(&sync_body);
            mirr_output.push_str("        }\n    }\n");
        }

        mirr_output.push_str("}\n");
    }

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&args.output, mirr_output)?;

    Ok(())
}
