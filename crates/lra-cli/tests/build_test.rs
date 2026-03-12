#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;
use std::process::Command;
/// Create a unique temp directory for a test and clean any prior run.
fn setup_temp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Remove a temp directory, ignoring errors.
fn cleanup(dir: &Path) {
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_build_minimal() {
    let dir = setup_temp("lra_test_build_minimal");

    let md = r#"---
title: "Test Paper"
author: "Test Author"
date: "2026-01"
description: "A test paper"
url: "https://example.com"
---
## Abstract

This is the abstract.
"#;

    let input_path = dir.join("paper.md");
    let output_path = dir.join("index.html");
    fs::write(&input_path, md).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("build")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to run lra build");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "Expected exit code 0, got {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        stdout,
        stderr,
    );

    assert!(output_path.exists(), "index.html was not created");

    let html = fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("<title>Test Paper</title>"), "Missing <title>");
    assert!(html.contains("<html lang=\"en\">"), "Missing html lang");
    assert!(html.contains("<meta charset=\"UTF-8\">"), "Missing charset meta");
    assert!(html.contains("lra:version"), "Missing lra:version meta");

    cleanup(&dir);
}

#[test]
fn test_build_with_claims() {
    let dir = setup_temp("lra_test_build_claims");

    let md = r##"---
title: "Claims Test"
author: "Test Author"
date: "2026-01"
description: "Testing claims"
url: "https://example.com"
claims:
  - text: "First claim"
    evidence: "#demo-1"
  - text: "Second claim"
---
Abstract section here.
"##;

    let input_path = dir.join("paper.md");
    let output_path = dir.join("index.html");
    fs::write(&input_path, md).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("build")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to run lra build");

    assert!(output.status.success(), "Build failed: {}", String::from_utf8_lossy(&output.stderr));

    let html = fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("data-lra-claim=\"1\""), "Missing claim 1 attribute");
    assert!(html.contains("data-lra-claim=\"2\""), "Missing claim 2 attribute");
    assert!(html.contains("data-lra-evidence=\"demo-1\""), "Missing evidence attribute for demo-1");

    cleanup(&dir);
}

#[test]
fn test_build_with_citations() {
    let dir = setup_temp("lra_test_build_citations");

    let md = r#"---
title: "Citation Test"
author: "Test Author"
date: "2026-01"
description: "Testing citations"
url: "https://example.com"
---
## Introduction

As shown by [@smith2020], and confirmed by [@jones2021], the results hold.
"#;

    let input_path = dir.join("paper.md");
    let output_path = dir.join("index.html");
    fs::write(&input_path, md).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("build")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to run lra build");

    assert!(output.status.success(), "Build failed: {}", String::from_utf8_lossy(&output.stderr));

    let html = fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("<a href=\"#ref-1\""), "Missing citation link #ref-1");
    assert!(html.contains("[1]"), "Missing citation number [1]");
    assert!(html.contains("<section id=\"references\">"), "Missing references section");
    // Both citation keys should appear in the reference list
    assert!(html.contains("smith2020"), "Missing smith2020 in reference list");
    assert!(html.contains("jones2021"), "Missing jones2021 in reference list");

    cleanup(&dir);
}

#[test]
fn test_build_missing_frontmatter() {
    let dir = setup_temp("lra_test_build_no_frontmatter");

    let md = "# Just a heading\n\nNo frontmatter here.\n";

    let input_path = dir.join("paper.md");
    let output_path = dir.join("index.html");
    fs::write(&input_path, md).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("build")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to run lra build");

    assert!(
        !output.status.success(),
        "Expected non-zero exit code for missing frontmatter, got success"
    );

    cleanup(&dir);
}

#[test]
fn test_build_validates_bronze() {
    let dir = setup_temp("lra_test_build_validates_bronze");

    // Markdown with claims, citations, and an explicit <section id="abstract">
    // so the validator can find the abstract section.
    let md = r##"---
title: "Bronze Validation Test"
author: "Test Author"
date: "2026-01"
description: "Testing Bronze compliance end-to-end"
url: "https://example.com"
claims:
  - text: "Primary claim"
    evidence: "#evidence-1"
---
<section id="abstract">

## Abstract

This paper demonstrates Bronze compliance.

</section>

## Introduction

As demonstrated by [@ref2026], the approach works.
"##;

    let input_path = dir.join("paper.md");
    let output_path = dir.join("index.html");
    fs::write(&input_path, md).unwrap();

    // Build the paper
    let build_output = Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("build")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to run lra build");

    assert!(
        build_output.status.success(),
        "Build failed: {}",
        String::from_utf8_lossy(&build_output.stderr),
    );
    assert!(output_path.exists(), "index.html was not created");

    // Create LICENSE, CITATION.cff, and paper.css required for full Bronze
    fs::write(dir.join("LICENSE"), "GNU GENERAL PUBLIC LICENSE\nVersion 3\n").unwrap();
    fs::write(
        dir.join("CITATION.cff"),
        "cff-version: 1.2.0\ntitle: Bronze Test\nlicense: GPL-3.0\n",
    )
    .unwrap();
    fs::write(dir.join("paper.css"), "/* placeholder */\n").unwrap();

    // Validate the built output — should achieve at least Bronze
    let validate_output = Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("validate")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to run lra validate");

    let stdout = String::from_utf8_lossy(&validate_output.stdout);
    assert!(
        validate_output.status.success(),
        "Validate returned non-zero; expected at least Bronze.\nstdout: {}\nstderr: {}",
        stdout,
        String::from_utf8_lossy(&validate_output.stderr),
    );
    assert!(
        stdout.contains("BRONZE") || stdout.contains("SILVER") || stdout.contains("GOLD"),
        "Expected a tier >= BRONZE in output, got:\n{}",
        stdout,
    );

    cleanup(&dir);
}
