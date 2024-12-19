#!/bin/bash

set -e  # Exit immediately on any error

echo "Running rustfmt..."
cargo fmt --all -- --check

echo "Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running tests..."
cargo test --all

echo "Auditing dependencies..."
cargo install cargo-audit > /dev/null 2>&1 || echo "cargo-audit already installed"
cargo audit

echo "Building documentation..."
cargo doc --no-deps

echo "All checks passed! 🚀"