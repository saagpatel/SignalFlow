#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Heavy build artifacts that are safe to regenerate.
TARGETS=(
  "dist"
  "dist-ssr"
  "node_modules/.vite"
  "src-tauri/target"
)

cd "$ROOT_DIR"
for target in "${TARGETS[@]}"; do
  if [[ -e "$target" ]]; then
    rm -rf "$target"
    echo "removed $target"
  else
    echo "missing $target"
  fi
done
