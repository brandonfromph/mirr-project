$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $repoRoot
$env:CARGO_TARGET_DIR = Join-Path $repoRoot "target/proposal-096-run"
$env:CI = "1"
Write-Host "Running canonical Proposal 096 closeout gate bundle..." -ForegroundColor Green
cargo run --bin mirr-general -- ci --format json
exit $LASTEXITCODE
