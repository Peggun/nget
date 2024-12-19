@echo off
setlocal enabledelayedexpansion

echo Debugging pre-push-checks.bat...

rem Print the arguments passed to the script (from Git)
echo Git arguments: %*

rem Run rustfmt
echo Running rustfmt...
cargo fmt --all -- --check || (
    echo rustfmt failed.
    exit /b
)

rem Run Clippy
echo Running Clippy...
cargo clippy --all-targets --all-features -- -D warnings || (
    echo Clippy failed.
    exit /b
)

rem Run tests
echo Running tests...
cargo test --all || (
    echo Tests failed.
    exit /b
)

rem Audit dependencies
echo Auditing dependencies...
cargo install cargo-audit >nul 2>&1 || echo cargo-audit already installed
cargo audit || (
    echo Audit failed.
    exit /b
)

rem Build documentation
echo Building documentation...
cargo doc --no-deps || (
    echo Documentation build failed.
    exit /b
)

echo All checks passed! 🚀
