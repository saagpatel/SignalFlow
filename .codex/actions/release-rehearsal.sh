#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=.codex/actions/_artifact_env.sh
source "$SCRIPT_DIR/_artifact_env.sh"

cd "$REPO_ROOT"

echo "==> Running canonical verify gate"
bash .codex/scripts/run_verify_commands.sh

echo "==> Building macOS desktop bundle"
pnpm tauri build

echo "==> Packaging macOS app archive"
pnpm release:pack-macos-app

echo "==> Preparing release notes"
pnpm release:notes

echo "==> Generating checksums"
pnpm release:checksum

echo "==> Validating macOS artifacts"
pnpm release:validate-macos

echo "==> Refreshing perf summary"
pnpm perf:summary

echo "Release rehearsal complete. Review src-tauri/target/release/bundle and .perf-results/summary.json."
