# CI Gate Individual Steps Test
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $repoRoot
$expectedTargetDir = Join-Path $repoRoot "target/ci-wave"
New-Item -ItemType Directory -Force -Path $expectedTargetDir | Out-Null
$env:CARGO_TARGET_DIR = $expectedTargetDir
& (Join-Path $repoRoot "scripts/preflight-gate.ps1") -RepoRoot $repoRoot -ExpectedTargetDir $expectedTargetDir

$steps = @(
    @{ name = "fmt check"; cmd = "cargo fmt --all -- --check" },
    @{ name = "clippy"; cmd = "cargo clippy --all-targets -- -D warnings" },
    @{ name = "test all"; cmd = "cargo test --all 2>&1 | Out-Null" },
    @{ name = "test self_hosting"; cmd = "cargo test --test self_hosting_parity_tests 2>&1 | Out-Null" },
    @{ name = "test lra-cli"; cmd = "cargo test -p lra-cli 2>&1 | Out-Null" },
    @{ name = "check mirr-wasm"; cmd = "cargo check -p mirr-wasm" }
)

Write-Host "Running CI Gate Individual Steps..."
$failed = $false
$results = @()

foreach ($step in $steps) {
    Write-Host "`nStep: $($step.name)" -ForegroundColor Cyan
    $result = Invoke-Expression $step.cmd 2>&1
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        Write-Host "PASS" -ForegroundColor Green
        $results += "$($step.name): PASS"
    } else {
        Write-Host "FAIL (exit code: $exitCode)" -ForegroundColor Red
        $results += "$($step.name): FAIL"
        $failed = $true
    }
}

Write-Host "`n=== SUMMARY ===" -ForegroundColor Yellow
$results | ForEach-Object { Write-Host $_ }

if ($failed) {
    Write-Host "`nSome steps FAILED" -ForegroundColor Red
    exit 1
} else {
    Write-Host "`nAll steps PASSED" -ForegroundColor Green
}

