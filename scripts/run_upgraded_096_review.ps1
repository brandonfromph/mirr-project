$repoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location $repoRoot
Write-Host "Use proposals/096-STRESS-TEST-PROMPT-EXACT-WITH-COVERAGE-ADDENDUM.txt for each subagent prompt." -ForegroundColor Yellow
Write-Host "After collecting subagent outputs into report files, run:" -ForegroundColor Yellow
Write-Host "python scripts/review_coverage_gate.py --repo . --reports <report1> <report2> ... <report8>" -ForegroundColor Cyan
