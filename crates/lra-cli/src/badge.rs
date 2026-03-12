#![forbid(unsafe_code)]

use crate::validate;

/// Print the shields.io badge URL for the detected tier. Returns exit code.
pub fn run(path: &str) -> i32 {
    let tier = validate::tier_for(path);
    let (label, color) = match tier {
        validate::Tier::Gold => ("gold", "ffd700"),
        validate::Tier::Silver => ("silver", "c0c0c0"),
        validate::Tier::Bronze => ("bronze", "cd7f32"),
        validate::Tier::None => ("none", "e05d44"),
    };

    println!("https://img.shields.io/badge/LRA--1.0-{}-{}", label, color);

    0
}
