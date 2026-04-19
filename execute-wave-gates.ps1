$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $repoRoot
$expectedTargetDir = Join-Path $repoRoot "target/ci-wave"
New-Item -ItemType Directory -Force -Path $expectedTargetDir | Out-Null
$env:CARGO_TARGET_DIR = $expectedTargetDir
$env:CI = "1"
& (Join-Path $repoRoot "scripts/preflight-gate.ps1") -RepoRoot $repoRoot -ExpectedTargetDir $expectedTargetDir
Write-Host "Running canonical Proposal 096 closeout gate bundle..." -ForegroundColor Green
cargo run --bin mirr-general -- ci --format json
exit $LASTEXITCODE
