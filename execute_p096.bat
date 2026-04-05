@echo off
REM Execute Proposal 096 - canonical gate wrapper
set "REPO_ROOT=%~dp0"
cd /d "%REPO_ROOT%"
set "CARGO_TARGET_DIR=%REPO_ROOT%target\proposal-096-run"
set "CI=1"
cargo run --bin mirr-general -- ci --format json
exit /b %ERRORLEVEL%
