@echo off
echo Executing `cargo test --verbose`
cargo test --verbose

REM Checking the return code.
if %errorlevel% neq 0 (
    echo `cargo test --verbose` Run failed.
    exit /b %errorlevel%
)

echo Executing `cargo clippy`
cargo clippy

REM Checking the return code.
if %errorlevel% neq 0 (
    echo `cargo clippy` Run failed.
    exit /b %errorlevel%
)

echo All commands executed successfully.