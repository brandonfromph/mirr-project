use nasa_rust_project::pipeline::run_pipeline;

#[test]
fn test_print_expanded_source() {
    let source = std::fs::read_to_string("rspu_chip/rspu_top.mirr").expect("failed to read");
    let mut expanded = String::with_capacity(source.len() * 2);
    let mut in_quotes = false;
    let mut in_comment = false;
    let mut in_interpolation = false;
    for ch in source.chars() {
        match ch {
            '"' if !in_comment && !in_interpolation => in_quotes = !in_quotes,
            '/' if !in_quotes && !in_interpolation => {
                if in_comment {
                    in_comment = false;
                } else {
                    in_comment = true;
                }
            }
            '\n' => {
                in_comment = false;
            }
            '$' if !in_quotes && !in_comment => {
                in_interpolation = true;
            }
            '}' if in_interpolation => {
                in_interpolation = false;
            }
            ';' | '{' | '}' if !in_quotes && !in_comment && !in_interpolation => {
                expanded.push(ch);
                expanded.push('\n');
                continue;
            }
            _ => {}
        }
        expanded.push(ch);
    }

    for (i, line) in expanded.lines().enumerate() {
        println!("{:3}: {}", i + 1, line);
    }
}
