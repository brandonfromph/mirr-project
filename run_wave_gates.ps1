$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $repoRoot
$env:CARGO_TARGET_DIR = Join-Path $repoRoot "target/proposal-097-run"
Write-Host "Running canonical Proposal 097 RWFI2 gate bundle..." -ForegroundColor Green
cargo run --bin mirr-general -- ci --format json
exit $LASTEXITCODE
