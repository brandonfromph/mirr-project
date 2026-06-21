use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parses a VCD file and returns a map of Signal Name -> Final Value.
/// Only stores the final recorded value for each signal.
pub fn parse_vcd_final_state(vcd_path: &Path) -> std::io::Result<HashMap<String, String>> {
    let file = File::open(vcd_path)?;
    let reader = BufReader::new(file);

    let mut id_to_name: HashMap<String, String> = HashMap::new();
    let mut current_state: HashMap<String, String> = HashMap::new();

    let mut in_definitions = true;

    for line_res in reader.lines() {
        let line = line_res?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if in_definitions {
            if trimmed.starts_with("$var") {
                // Example: $var wire 1 n3 b $end
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 5 {
                    let id = parts[3].to_string();
                    let name = parts[4].to_string();
                    id_to_name.insert(id, name);
                }
            } else if trimmed.starts_with("$enddefinitions") {
                in_definitions = false;
            }
        } else {
            // Processing state changes
            if trimmed.starts_with('#') {
                // Time step, we just advance
                continue;
            }

            if trimmed.starts_with('b') || trimmed.starts_with('B') {
                // Vector format: b<val> <id>
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() == 2 {
                    let val = parts[0][1..].to_string(); // strip 'b'
                    let id = parts[1].to_string();
                    current_state.insert(id, val);
                }
            } else if trimmed.starts_with('0')
                || trimmed.starts_with('1')
                || trimmed.starts_with('x')
                || trimmed.starts_with('z')
            {
                // Scalar format: <val><id>
                let val = trimmed[0..1].to_string();
                let id = trimmed[1..].trim().to_string();
                current_state.insert(id, val);
            }
        }
    }

    // Map internal IDs to actual MIRR signal names
    let mut final_state = HashMap::new();
    for (id, val) in current_state {
        if let Some(name) = id_to_name.get(&id) {
            final_state.insert(name.clone(), val);
        }
    }

    Ok(final_state)
}
