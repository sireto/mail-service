#!/usr/bin/env bash

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
checks=(backend frontend)

for directory in "${checks[@]}"; do
  if git -C "$repo_root" diff --cached --quiet -- "$directory"; then
    echo "No staged changes in $directory; skipping checks."
    continue
  fi

  echo "Staged changes detected in $directory; running pre-commit checks."
  (
    cd "$repo_root/$directory"
    ./pre-commit.sh
  )
done
