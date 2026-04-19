@echo off
REM Proposal 097 RWFI2 gate wrapper; delegates to the canonical Rust closeout command.
set "REPO_ROOT=%~dp0"
cd /d "%REPO_ROOT%"
set "CARGO_TARGET_DIR=%REPO_ROOT%target\ci-wave"
set "CI=1"
if not exist "%CARGO_TARGET_DIR%" mkdir "%CARGO_TARGET_DIR%"
powershell -NoProfile -ExecutionPolicy Bypass -File "%REPO_ROOT%scripts\preflight-gate.ps1" -RepoRoot "%REPO_ROOT%" -ExpectedTargetDir "%CARGO_TARGET_DIR%"
if errorlevel 1 exit /b %ERRORLEVEL%
cargo run --bin mirr-general -- ci --format json
exit /b %ERRORLEVEL%
