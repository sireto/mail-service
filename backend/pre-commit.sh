#!/usr/bin/env bash

set -euo pipefail

echo "Checking Rust formatting..."
cargo fmt --all --check

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Checking backend compilation..."
cargo check --all-targets --all-features
