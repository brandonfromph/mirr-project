$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = (Resolve-Path -Path $scriptDir).Path

# Canonical gate command: cargo run --bin mirr-general -- ci --format json
# Legacy marker for gate contract parity: target/proposal-097-run
$exitCode = 1
Push-Location -Path $repoRoot
try {
	& cargo.exe run --bin mirr-general -- ci --format json
	$exitCode = $LASTEXITCODE
}
finally {
	Pop-Location
}

exit $exitCode
