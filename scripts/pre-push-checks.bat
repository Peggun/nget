@echo off
setlocal enabledelayedexpansion

echo Running rustfmt...
cargo fmt --all -- --check || exit /b

echo Running Clippy...
cargo clippy --all-targets --all-features -- -D warnings || exit /b

echo Running tests...
cargo test --all || exit /b

echo Auditing dependencies...
cargo install cargo-audit >nul 2>&1 || echo cargo-audit already installed
cargo audit || exit /b

echo Building documentation...
cargo doc --no-deps || exit /b

echo All checks passed! 🚀