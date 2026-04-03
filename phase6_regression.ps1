# PHASE 6: Final Regression Check
Write-Host "PHASE 6: Final Verification Gate" -ForegroundColor Yellow

$checks = @(
    @{ name = "cargo check --all-targets"; cmd = "cargo check --all-targets" },
    @{ name = "cargo fmt --check"; cmd = "cargo fmt --check" },
    @{ name = "cargo clippy --all-targets"; cmd = "cargo clippy --all-targets -- -D warnings" }
)

$resultsFile = "proposals/evidence/096/phase6-regression-check.log"
@() | Out-File -FilePath $resultsFile -Encoding UTF8
$results = @()

foreach ($check in $checks) {
    Write-Host "`nTest: $($check.name)" -ForegroundColor Cyan
    $output = Invoke-Expression $check.cmd 2>&1
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        Write-Host "PASS (exit code: $exitCode)" -ForegroundColor Green
        $msg = "[$exitCode] $($check.name): PASS"
    } else {
        Write-Host "FAIL (exit code: $exitCode)" -ForegroundColor Red
        $msg = "[$exitCode] $($check.name): FAIL"
        $output | Out-String | Write-Host  
    }
    $results += $msg
    $msg | Add-Content $resultsFile
}

Write-Host "`n=== REGRESSION CHECK SUMMARY ===" -ForegroundColor Yellow
$results | ForEach-Object { Write-Host $_ }

Write-Host "`nPhase 6 Complete"
