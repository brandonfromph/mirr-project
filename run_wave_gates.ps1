$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = (Resolve-Path -Path $scriptDir).Path

# Canonical gate command: cargo run --bin mirr-general -- ci --format json
# Legacy marker for gate contract parity: target/proposal-097-run
$exitCode = 1
Push-Location -Path $repoRoot
try {
	$expectedTargetDir = Join-Path $repoRoot "target/ci-wave"
	New-Item -ItemType Directory -Force -Path $expectedTargetDir | Out-Null
	$env:CARGO_TARGET_DIR = $expectedTargetDir
	& (Join-Path $repoRoot "scripts/preflight-gate.ps1") -RepoRoot $repoRoot -ExpectedTargetDir $expectedTargetDir
	& cargo.exe run --bin mirr-general -- ci --format json
	$exitCode = $LASTEXITCODE
}
finally {
	Pop-Location
}

exit $exitCode
