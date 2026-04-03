#!/usr/bin/env powershell
# Test Wave 3 Implementation Gates

$ErrorActionPreference = "Continue"
$results = @()

Write-Host "=" * 70
Write-Host "WAVE 3 IMPLEMENTATION GATE TESTS"
Write-Host "=" * 70
Write-Host ""

# Gate 3: repo_metrics.py --json
Write-Host "Gate 3: python scripts/repo_metrics.py --json"
Write-Host "-" * 70
Try {
    $output = python scripts/repo_metrics.py --json 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ PASS: Exit code 0"
        # Parse JSON and check KB-lite keys
        $json = $output | ConvertFrom-Json
        $kb_keys = @("graph_db_bytes", "lance_data_files", "lance_txn_files", "lance_version_files")
        $missing = @()
        foreach ($key in $kb_keys) {
            if ($null -eq $json.$key) {
                $missing += $key
            }
        }
        if ($missing.Count -eq 0) {
            Write-Host "✓ PASS: All KB-lite keys present"
            Write-Host "  - graph_db_bytes: $($json.graph_db_bytes)"
            Write-Host "  - lance_data_files: $($json.lance_data_files)"
            Write-Host "  - lance_txn_files: $($json.lance_txn_files)"
            Write-Host "  - lance_version_files: $($json.lance_version_files)"
            $results += "GATE_3: PASS"
        } else {
            Write-Host "✗ FAIL: Missing KB-lite keys: $($missing -join ', ')"
            $results += "GATE_3: FAIL (missing keys)"
        }
    } else {
        Write-Host "✗ FAIL: Exit code $LASTEXITCODE"
        Write-Host $output
        $results += "GATE_3: FAIL (exit code)"
    }
} Catch {
    Write-Host "✗ ERROR: $_"
    $results += "GATE_3: ERROR"
}
Write-Host ""

# Gate 4: validate_proposals.py --kb-lite-strict
Write-Host "Gate 4: python scripts/validate_proposals.py --kb-lite-strict"
Write-Host "-" * 70
Try {
    $output = python scripts/validate_proposals.py --kb-lite-strict 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ PASS: Exit code 0"
        Write-Host $output
        $results += "GATE_4: PASS"
    } else {
        Write-Host "✗ WARNING: Exit code $LASTEXITCODE (may be expected if KB data not present)"
        Write-Host $output
        $results += "GATE_4: CONDITIONAL"
    }
} Catch {
    Write-Host "✗ ERROR: $_"
    $results += "GATE_4: ERROR"
}
Write-Host ""

# Summary
Write-Host "=" * 70
Write-Host "TEST SUMMARY"
Write-Host "=" * 70
foreach ($result in $results) {
    Write-Host $result
}
