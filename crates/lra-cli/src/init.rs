#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

const TEMPLATE_INDEX: &str = include_str!("../template/index.html");
const TEMPLATE_CSS: &str = include_str!("../template/paper.css");
const TEMPLATE_JS: &str = include_str!("../template/paper.js");
const TEMPLATE_SW: &str = include_str!("../template/sw.js");
const TEMPLATE_CITATION: &str = include_str!("../template/CITATION.cff");
const TEMPLATE_CONTRIBUTING: &str = include_str!("../template/CONTRIBUTING.md");
const TEMPLATE_SPEC: &str = include_str!("../template/spec/LRA-1.0.md");
const TEMPLATE_GITIGNORE: &str = include_str!("../template/.gitignore");
const TEMPLATE_LRA_CARD: &str = include_str!("../template/lra-card.svg");
const TEMPLATE_LRA_CLIENT: &str = include_str!("../template/lra-client.js");

// GPL-3.0 license header — embedded directly so the crate is self-contained
const TEMPLATE_LICENSE: &str = "\
                    GNU GENERAL PUBLIC LICENSE\n\
                       Version 3, 29 June 2007\n\n\
 Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>\n\
 Everyone is permitted to copy and distribute verbatim copies\n\
 of this license document, but changing it is not allowed.\n\n\
 See https://www.gnu.org/licenses/gpl-3.0.txt for the full license text.\n";

/// Scaffold a new LRA project. Returns exit code.
pub fn run(name: &str) -> i32 {
    let root = Path::new(name);

    if root.exists() {
        eprintln!("Error: directory '{}' already exists", name);
        return 1;
    }

    if let Err(e) = scaffold(root) {
        eprintln!("Error: {}", e);
        return 1;
    }

    println!("Created LRA project: {}/", name);
    println!("  index.html     — your paper (edit this!)");
    println!("  paper.css      — styling");
    println!("  paper.js       — interactive layer");
    println!("  sw.js          — service worker");
    println!("  CITATION.cff   — citation metadata");
    println!("  LICENSE        — GPL-3.0");
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  lra serve");
    println!("  # Open http://localhost:8080");

    0
}

fn scaffold(root: &Path) -> std::io::Result<()> {
    fs::create_dir_all(root.join("spec"))?;

    let files: &[(&str, &str)] = &[
        ("index.html", TEMPLATE_INDEX),
        ("paper.css", TEMPLATE_CSS),
        ("paper.js", TEMPLATE_JS),
        ("sw.js", TEMPLATE_SW),
        ("CITATION.cff", TEMPLATE_CITATION),
        ("CONTRIBUTING.md", TEMPLATE_CONTRIBUTING),
        ("spec/LRA-1.0.md", TEMPLATE_SPEC),
        ("LICENSE", TEMPLATE_LICENSE),
        (".gitignore", TEMPLATE_GITIGNORE),
        ("lra-card.svg", TEMPLATE_LRA_CARD),
        ("lra-client.js", TEMPLATE_LRA_CLIENT),
    ];

    for (name, content) in files {
        fs::write(root.join(name), content)?;
    }

    Ok(())
}
