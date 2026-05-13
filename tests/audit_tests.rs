#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::tempdir;

    fn get_audit_bin() -> PathBuf {
        PathBuf::from(env!("CARGO_BIN_EXE_mirr-audit"))
    }

    #[test]
    fn test_refinement_gap_e801() {
        let dir = tempdir().expect("Failed to create temp dir");
        let root = dir.path();
        let bin = get_audit_bin();

        // 1. Create a dummy proposal with a struct definition in a code block
        let proposals_dir = root.join("proposals");
        fs::create_dir(&proposals_dir).expect("Failed to create proposals dir");
        let proposal_path = proposals_dir.join("001-TEST-2026-03-28.md");
        fs::write(
            &proposal_path,
            "# Test Proposal\n\n```rust\nstruct MissingStruct {\n    field: u32,\n}\n```\n",
        )
        .expect("Failed to write proposal");

        // 2. Create a dummy src directory with NO implementation of MissingStruct
        let src_dir = root.join("src");
        fs::create_dir(&src_dir).expect("Failed to create src dir");
        let src_path = src_dir.join("main.rs");
        fs::write(&src_path, "fn main() {}\n").expect("Failed to write src");

        // 3. Run mirr-audit in refinement mode
        let output = Command::new(&bin)
            .args(["refinement", "--glob", "src/**/*.rs"])
            .current_dir(root)
            .output()
            .expect("Failed to run mirr-audit");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("STDOUT: {}", stdout);
        println!("STDERR: {}", stderr);

        // 4. Verify that E801 finding is present
        assert!(stdout.contains("E801"), "Output should contain E801 error code. Got: {}", stdout);
        assert!(
            stdout.contains("Refinement Gap: Struct 'MissingStruct'"),
            "Output should mention the missing struct"
        );
    }

    #[test]
    fn test_refinement_no_gap() {
        let dir = tempdir().expect("Failed to create temp dir");
        let root = dir.path();
        let bin = get_audit_bin();

        // 1. Create a dummy proposal with a struct
        let proposals_dir = root.join("proposals");
        fs::create_dir(&proposals_dir).expect("Failed to create proposals dir");
        let proposal_path = proposals_dir.join("001-TEST-2026-03-28.md");
        fs::write(
            &proposal_path,
            "# Test Proposal\n\n```rust\nstruct FoundStruct {\n    field: u32,\n}\n```\n",
        )
        .expect("Failed to write proposal");

        // 2. Create a dummy src directory WITH implementation
        let src_dir = root.join("src");
        fs::create_dir(&src_dir).expect("Failed to create src dir");
        let src_path = src_dir.join("main.rs");
        fs::write(&src_path, "pub struct FoundStruct { pub field: u32 }\n")
            .expect("Failed to write src");

        // 3. Run mirr-audit in refinement mode
        let output = Command::new(&bin)
            .args(["refinement", "--glob", "src/**/*.rs"])
            .current_dir(root)
            .output()
            .expect("Failed to run mirr-audit");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("STDOUT: {}", stdout);
        println!("STDERR: {}", stderr);

        // 4. Verify NO findings
        assert!(
            stdout.contains("Zero-Debt Invariant verified"),
            "Should have no violations. Got: {}",
            stdout
        );
    }
}
