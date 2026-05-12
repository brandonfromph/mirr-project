@echo off
setlocal enabledelayedexpansion

set "REPO_ROOT=%~dp0"
if "%REPO_ROOT:~-1%"=="\" set "REPO_ROOT=%REPO_ROOT:~0,-1%"

set "CARGO_TARGET_DIR=%REPO_ROOT%\target\proposal-097-run"
cd /d "%REPO_ROOT%"

cargo run --bin mirr-general -- ci --format json
