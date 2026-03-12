#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

/// Creates a minimal Bronze-compliant HTML.
fn bronze_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Test Paper</title>
  <meta name="description" content="A test paper">
  <meta name="lra:version" content="1.0">
</head>
<body>
<section id="abstract"><h2>Abstract</h2></section>
<section id="claims"><ol><li data-lra-claim="1">Claim 1</li></ol></section>
<section id="references"><h2>References</h2></section>
<section id="citation"><h2>Citation</h2></section>
</body>
</html>"#
        .to_string()
}

/// Add Silver elements to existing HTML.
fn make_silver(html: &str) -> String {
    html.replace(
        "</body>",
        r#"<noscript><p>JS required</p></noscript>
<section class="demo"><h2>Demo</h2></section>
</body>"#,
    )
}

/// Add Gold elements to existing HTML.
fn make_gold(html: &str) -> String {
    html.replace("data-lra-claim=\"1\"", "data-lra-claim=\"1\" data-lra-evidence=\"demo-1\"")
        .replace("</body>", "<div aria-live=\"polite\">output</div>\n</body>")
}

fn setup_dir(dir: &Path, html: &str, silver: bool, gold: bool) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("index.html"), html).unwrap();
    fs::write(dir.join("LICENSE"), "GNU GENERAL PUBLIC LICENSE\nVersion 3\n").unwrap();
    fs::write(dir.join("CITATION.cff"), "cff-version: 1.2.0\ntitle: Test\nlicense: GPL-3.0\n")
        .unwrap();
    fs::write(dir.join("paper.css"), "/* placeholder */\n").unwrap();

    if silver {
        fs::write(dir.join("paper.js"), "// paper.js\n").unwrap();
    }
    if gold {
        fs::create_dir_all(dir.join("demos")).unwrap();
        fs::write(dir.join("demos/test.wasm"), [0u8, 0x61, 0x73, 0x6d]).unwrap();
    }
}

#[test]
fn test_validate_bronze() {
    let dir = std::env::temp_dir().join("lra_test_bronze");
    let _ = fs::remove_dir_all(&dir);

    let html = bronze_html();
    setup_dir(&dir, &html, false, false);

    let html_path = dir.join("index.html");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("validate")
        .arg(html_path.to_str().unwrap())
        .output()
        .expect("failed to run lra validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("BRONZE"), "Expected BRONZE tier, got:\n{}", stdout);
    assert!(output.status.success());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_validate_silver() {
    let dir = std::env::temp_dir().join("lra_test_silver");
    let _ = fs::remove_dir_all(&dir);

    let html = make_silver(&bronze_html());
    setup_dir(&dir, &html, true, false);

    let html_path = dir.join("index.html");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("validate")
        .arg(html_path.to_str().unwrap())
        .output()
        .expect("failed to run lra validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SILVER"), "Expected SILVER tier, got:\n{}", stdout);
    assert!(output.status.success());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_validate_gold() {
    let dir = std::env::temp_dir().join("lra_test_gold");
    let _ = fs::remove_dir_all(&dir);

    let html = make_gold(&make_silver(&bronze_html()));
    setup_dir(&dir, &html, true, true);

    let html_path = dir.join("index.html");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("validate")
        .arg(html_path.to_str().unwrap())
        .output()
        .expect("failed to run lra validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("GOLD"), "Expected GOLD tier, got:\n{}", stdout);
    assert!(output.status.success());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_validate_failure_missing_abstract() {
    let dir = std::env::temp_dir().join("lra_test_fail");
    let _ = fs::remove_dir_all(&dir);

    // HTML missing abstract section
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Bad Paper</title>
  <meta name="description" content="Missing sections">
  <meta name="lra:version" content="1.0">
</head>
<body>
<section id="claims"><ol><li data-lra-claim="1">Claim</li></ol></section>
<section id="references"></section>
<section id="citation"></section>
</body>
</html>"#;

    setup_dir(&dir, html, false, false);

    let html_path = dir.join("index.html");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("validate")
        .arg(html_path.to_str().unwrap())
        .output()
        .expect("failed to run lra validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[FAIL]"), "Expected FAIL check, got:\n{}", stdout);
    assert!(stdout.contains("NONE"), "Expected NONE tier, got:\n{}", stdout);
    assert!(!output.status.success());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_init_creates_files() {
    let dir = std::env::temp_dir().join("lra_test_init_parent");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("init")
        .arg("my-paper")
        .current_dir(&dir)
        .output()
        .expect("failed to run lra init");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created LRA project"), "Expected success, got:\n{}", stdout);
    assert!(output.status.success());

    let project = dir.join("my-paper");
    assert!(project.join("index.html").exists());
    assert!(project.join("paper.css").exists());
    assert!(project.join("paper.js").exists());
    assert!(project.join("sw.js").exists());
    assert!(project.join("CITATION.cff").exists());
    assert!(project.join("LICENSE").exists());
    assert!(project.join("spec/LRA-1.0.md").exists());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_badge_output() {
    let dir = std::env::temp_dir().join("lra_test_badge");
    let _ = fs::remove_dir_all(&dir);

    let html = make_gold(&make_silver(&bronze_html()));
    setup_dir(&dir, &html, true, true);

    let html_path = dir.join("index.html");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_lra"))
        .arg("badge")
        .arg(html_path.to_str().unwrap())
        .output()
        .expect("failed to run lra badge");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("shields.io/badge/LRA--1.0-gold-ffd700"),
        "Expected gold badge URL, got:\n{}",
        stdout
    );
    assert!(output.status.success());

    let _ = fs::remove_dir_all(&dir);
}
