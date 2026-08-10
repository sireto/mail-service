#!/usr/bin/env bash

set -euo pipefail

echo "Checking frontend formatting..."
yarn format:check

echo "Running ESLint..."
yarn lint
