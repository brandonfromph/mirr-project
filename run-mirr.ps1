param(
    [Parameter(Position = 0)]
    [string]$InputPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = $PSScriptRoot
if (-not $repoRoot) {
    throw "Could not determine repository root from script location."
}

if (-not $InputPath) {
    $InputPath = Join-Path $repoRoot "examples\neonatal_respirator.mirr"
    Write-Host "No input path provided. Using default example: $InputPath"
}

$resolvedInput = if (Test-Path $InputPath) {
    (Resolve-Path $InputPath).Path
} else {
    throw "Input MIRR file not found: $InputPath"
}

$exeCandidates = @(
    (Join-Path $repoRoot "target\debug\nasa-rust-project.exe"),
    (Join-Path $repoRoot "target\release\nasa-rust-project.exe")
)

$exePath = $exeCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $exePath) {
    throw "No built executable found. Build once with 'cargo build' when Cargo is available, or place the binary at target/debug or target/release."
}

& $exePath $resolvedInput
exit $LASTEXITCODE
