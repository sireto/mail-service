#!/usr/bin/env bash

set -euo pipefail

echo "Checking Rust formatting..."
cargo fmt --check

echo "Checking backend compilation..."
cargo check --all-targets --all-features
