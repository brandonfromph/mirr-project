use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parses a VCD file and returns a map of Signal Name -> Value.
/// If `target_step` is provided, it extracts the pre-edge state that triggered the failure
/// at that SMT step. Otherwise, it returns the final recorded state.
pub fn parse_vcd_state_at_step(
    vcd_path: &Path,
    target_step: Option<usize>,
) -> std::io::Result<HashMap<String, String>> {
    let file = File::open(vcd_path)?;
    let reader = BufReader::new(file);

    let mut id_to_name: HashMap<String, String> = HashMap::new();
    let mut smt_step_id: Option<String> = None;

    let mut current_state: HashMap<String, String> = HashMap::new();
    let mut last_timestamp_state: HashMap<String, String> = HashMap::new();
    let mut target_state: Option<HashMap<String, String>> = None;

    let mut current_smt_step: Option<usize> = None;
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
                    id_to_name.insert(id.clone(), name.clone());
                    if name == "smt_step" {
                        smt_step_id = Some(id);
                    }
                }
            } else if trimmed.starts_with("$enddefinitions") {
                in_definitions = false;
            }
        } else {
            // Processing state changes
            if trimmed.starts_with('#') {
                // Time step advances.
                // For target_step == 0, the failure is prior to any clock edge. We can
                // capture the state exactly after the initial `#0` step initializes.
                if let Some(target) = target_step {
                    if target == 0 && current_smt_step == Some(0) && target_state.is_none() {
                        target_state = Some(current_state.clone());
                    }
                }

                last_timestamp_state = current_state.clone();
                continue;
            }

            let (val, id) = if trimmed.starts_with('b') || trimmed.starts_with('B') {
                // Vector format: b<val> <id>
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() == 2 {
                    (parts[0][1..].to_string(), parts[1].to_string())
                } else {
                    continue;
                }
            } else if trimmed.starts_with('0')
                || trimmed.starts_with('1')
                || trimmed.starts_with('x')
                || trimmed.starts_with('z')
            {
                // Scalar format: <val><id>
                (trimmed[0..1].to_string(), trimmed[1..].trim().to_string())
            } else {
                continue;
            };

            current_state.insert(id.clone(), val.clone());

            // Check if this variable update is the `smt_step`
            if Some(&id) == smt_step_id.as_ref() {
                if let Ok(step_val) = usize::from_str_radix(&val, 2) {
                    current_smt_step = Some(step_val);

                    if let Some(target) = target_step {
                        // For target_step > 0, the failure happens on the clock edge where
                        // smt_step transitions to target_step. We must capture the state
                        // from the *previous* timestamp (the pre-edge state) that caused it.
                        if step_val == target && target > 0 && target_state.is_none() {
                            target_state = Some(last_timestamp_state.clone());
                        }
                    }
                }
            }
        }
    }

    // Map internal IDs to actual MIRR signal names using the captured target state,
    // or fall back to the final state if no target was reached.
    let state_to_use = target_state.unwrap_or(current_state);
    let mut final_state_map = HashMap::new();

    for (id, val) in state_to_use {
        if let Some(name) = id_to_name.get(&id) {
            final_state_map.insert(name.clone(), val);
        }
    }

    Ok(final_state_map)
}
