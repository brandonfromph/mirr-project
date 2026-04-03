# PHASE 1: Baseline test capture script
$logFile = "proposals/evidence/096/cargo-test-all-baseline.log"
Remove-Item $logFile -Force -ErrorAction SilentlyContinue

Write-Host "Starting cargo test --all at $(Get-Date -Format 'u')"

# Run the test directly without profile interference
$exit_code = 0
try {
    & "C:\Users\elvie\.cargo\bin\cargo.exe" test --all 2>&1 | Out-File -FilePath $logFile -Encoding UTF8
    $exit_code = $LASTEXITCODE
} catch {
    $exit_code = -1
    $_.Exception.Message | Out-File -FilePath $logFile -Encoding UTF8 -Append
}

$timestamp = Get-Date -Format 'o'
$rustc_version = & "C:\Users\elvie\.cargo\bin\rustc.exe" --version 2>&1

$exitCodeLine = "Exit code: $exit_code"
"" | Out-File -FilePath $logFile -Encoding UTF8 -Append
$exitCodeLine | Out-File -FilePath $logFile -Encoding UTF8 -Append

Write-Host "Completed at $(Get-Date -Format 'u')"
Write-Host "Log written to: $logFile"
Write-Host "File size: $(Get-Item $logFile | % {$_.Length}) bytes"
Write-Host "Exit code: $exit_code"
