#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

const MAX_PROJECT_NAME_BYTES: usize = 64;

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

// GPL-3.0-or-later license header — embedded directly so the crate is self-contained
const TEMPLATE_LICENSE: &str = "\
                                 GNU General Public License\n\
                           Version 2.0, January 2004\n\
                        https://www.gnu.org/licenses/\n\n\
   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION\n\n\
   See https://www.gnu.org/licenses/gpl-3.0.txt for the full license text.\n";

fn validate_project_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("project name must not be empty");
    }
    if name.len() > MAX_PROJECT_NAME_BYTES {
        return Err("project name is too long");
    }
    if name == "." || name == ".." {
        return Err("project name must not be relative path markers");
    }
    if name.starts_with('.') {
        return Err("project name must not start with '.'");
    }
    if name.contains('/') || name.contains('\\') || name.contains(':') {
        return Err("project name must not contain path separators");
    }
    if !name.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_') {
        return Err("project name must use only [A-Za-z0-9_-]");
    }

    Ok(())
}

/// Scaffold a new LRA project. Returns exit code.
pub fn run(name: &str) -> i32 {
    if let Err(message) = validate_project_name(name) {
        eprintln!("Error: {}", message);
        return 1;
    }

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
    println!("  LICENSE        — GPL-3.0-or-later");
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

#[cfg(test)]
mod tests {
    use super::validate_project_name;

    #[test]
    fn validate_project_name_accepts_safe_values() {
        assert!(validate_project_name("paper_2026").is_ok());
        assert!(validate_project_name("paper-2026").is_ok());
    }

    #[test]
    fn validate_project_name_rejects_path_like_values() {
        assert!(validate_project_name("../outside").is_err());
        assert!(validate_project_name("nested/project").is_err());
        assert!(validate_project_name("nested\\project").is_err());
        assert!(validate_project_name("C:drive").is_err());
        assert!(validate_project_name(".hidden").is_err());
    }

    #[test]
    fn validate_project_name_rejects_unsafe_symbols() {
        assert!(validate_project_name("paper name").is_err());
        assert!(validate_project_name("paper.name").is_err());
        assert!(validate_project_name("").is_err());
    }
}
