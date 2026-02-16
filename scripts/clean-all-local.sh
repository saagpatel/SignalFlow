#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"

./scripts/clean-heavy.sh

if [[ -e "node_modules" ]]; then
  rm -rf node_modules
  echo "removed node_modules"
else
  echo "missing node_modules"
fi
