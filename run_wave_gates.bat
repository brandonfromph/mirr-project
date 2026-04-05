@echo off
REM Proposal 097 RWFI2 gate wrapper; delegates to the canonical Rust closeout command.
set "REPO_ROOT=%~dp0"
cd /d "%REPO_ROOT%"
set "CARGO_TARGET_DIR=%REPO_ROOT%target\proposal-097-run"
set "CI=1"
cargo run --bin mirr-general -- ci --format json
exit /b %ERRORLEVEL%
