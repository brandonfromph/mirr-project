use std::collections::HashMap;

fn expand_string(s: &str, signal_env: &HashMap<String, String>) -> String {
    let mut res = s.to_string();
    for (k, v) in signal_env {
        res = res.replace(&format!("${{{}}}", k), v);
        if !matches!(k.as_str(), "true" | "false" | "clk" | "rst_n") {
            if &res == k {
                res = v.clone();
            } else if let Some(bracket_idx) = res.find('[') {
                if &res[..bracket_idx] == k {
                    res = format!("{}{}", v, &res[bracket_idx..]);
                }
            }
        }
    }
    res
}

fn main() {
    let mut env = HashMap::new();
    env.insert("tx_valid".to_string(), "port_tx_valid".to_string());
    println!("{}", expand_string("tx_valid[1]", &env));
}
