param(
    [Parameter(Mandatory = $true)]
    [string]$RepoRoot,
    [Parameter(Mandatory = $true)]
    [string]$ExpectedTargetDir,
    [string]$ExpectedWrapper = "sccache"
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$configPath = Join-Path $RepoRoot ".cargo/config.toml"
if (-not (Test-Path $configPath)) {
    throw "preflight error: missing $configPath"
}

if (-not $env:CARGO_TARGET_DIR) {
    throw "preflight error: CARGO_TARGET_DIR is not set"
}

$actualTarget = (Resolve-Path -Path $env:CARGO_TARGET_DIR).Path
$expectedTarget = (Resolve-Path -Path $ExpectedTargetDir).Path
if ($actualTarget -ne $expectedTarget) {
    throw "preflight error: CARGO_TARGET_DIR drift. expected=$expectedTarget actual=$actualTarget"
}

$configText = Get-Content -Path $configPath -Raw
$expectedConfigLine = "rustc-wrapper = `"$ExpectedWrapper`""
if ($configText -notmatch [regex]::Escape($expectedConfigLine)) {
    throw "preflight error: rustc-wrapper drift in .cargo/config.toml"
}

$resolved = Get-Command sccache -ErrorAction Stop
$resolvedPath = $resolved.Source
if ($ExpectedWrapper -ne "sccache") {
    $expectedNorm = $ExpectedWrapper.Replace('/', '\\')
    if ($resolvedPath -ne $expectedNorm) {
        throw "preflight error: sccache PATH resolution drift. expected=$expectedNorm actual=$resolvedPath"
    }
}

Write-Output "preflight ok: wrapper/path/target-dir aligned"
