# MIRR Self-Hosting Build Script (PowerShell)
# Usage: ./build_selfhost.ps1

Write-Host "[INFO] Step 1: Build MIRR-in-MIRR compiler using Rust reference"
$mirrFiles = @(
    "compiler_mirr/lexer.mirr",
    "compiler_mirr/parser.mirr",
    "compiler_mirr/semantic.mirr",
    "compiler_mirr/temporal_lowering.mirr",
    "compiler_mirr/emitter.mirr"
)
foreach ($file in $mirrFiles) {
    cargo run --bin nasa-rust-project -- --selfhost-compile $file
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[FAIL] Rust reference failed on $file" -ForegroundColor Red
        exit 1
    }
}
Write-Host "[INFO] MIRR-in-MIRR compiler built using Rust reference."

# Step 2: Use MIRR-in-MIRR compiler to build itself (bootstrapping)
$mirrSelfhostBin = "target/selfhosted_mirr_compiler.exe"

if (Test-Path $mirrSelfhostBin) {
    foreach ($file in $mirrFiles) {
        & $mirrSelfhostBin $file
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[FAIL] MIRR-in-MIRR compiler failed on $file" -ForegroundColor Red
            exit 1
        }
    }
    Write-Host "[INFO] MIRR-in-MIRR compiler built itself (bootstrapped)."
} else {
    Write-Host ("[WARN] Self-hosted binary not found at " + $mirrSelfhostBin + " - skipping self-host bootstrap stage.") -ForegroundColor Yellow
    Write-Host "[INFO] To run the self-host stage, place the self-hosted compiler at the above path or update the script."
}

# Step 3: Validate output (diff, hash, etc.)
# (Add validation logic as needed)
Write-Host "[SUCCESS] MIRR self-hosting pipeline completed."