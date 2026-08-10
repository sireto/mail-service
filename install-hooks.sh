#!/usr/bin/env bash

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Run this script from inside the mail-service Git repository." >&2
  exit 1
}

hooks_dir="$(git -C "$repo_root" rev-parse --git-path hooks)"
source_hook="$repo_root/hooks/pre-commit.sh"
target_hook="$hooks_dir/pre-commit"

if [ ! -f "$source_hook" ]; then
  echo "Pre-commit hook source not found: $source_hook" >&2
  exit 1
fi

mkdir -p "$hooks_dir"
cp "$source_hook" "$target_hook"
chmod +x "$target_hook"

echo "Installed pre-commit hook at $target_hook"
