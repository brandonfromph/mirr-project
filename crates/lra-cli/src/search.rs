#![forbid(unsafe_code)]

use std::path::Path;

use crate::registry;
use crate::util::bounded_read_to_string;

pub fn run(query: &str, registry_path: &str) -> i32 {
    let json = bounded_read_to_string(Path::new(registry_path));
    if json.is_empty() {
        eprintln!("Error: cannot read registry at {}", registry_path);
        return 1;
    }
    let reg = match registry::parse_registry(&json) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 1;
        }
    };
    let results = registry::search_entries(&reg, query);
    if results.is_empty() {
        println!("No papers found matching \"{}\"", query);
        return 0;
    }
    println!("Found {} paper(s) matching \"{}\":\n", results.len(), query);
    let mut i = 0;
    let max = results.len();
    while i < max {
        let entry = results[i];
        println!("  {} \u{2014} {}", entry.title, entry.url);
        println!("    hash: {}", entry.hash);
        println!("    keywords: {}", entry.keywords.join(", "));
        if !entry.depends.is_empty() {
            println!("    depends: {} paper(s)", entry.depends.len());
        }
        println!();
        i += 1;
    }
    0
}
