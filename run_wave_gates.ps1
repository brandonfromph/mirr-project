$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = $scriptDir

$env:CARGO_TARGET_DIR = Join-Path $repoRoot 'target/proposal-097-run'
Set-Location $repoRoot

cargo run --bin mirr-general -- ci --format json
