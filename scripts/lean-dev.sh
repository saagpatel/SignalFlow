#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LEAN_TMP_DIR="$(mktemp -d -t signalflow-lean-dev-XXXXXX)"
CLEANED_UP=0

cleanup() {
  if [[ "$CLEANED_UP" == "1" ]]; then
    return
  fi
  CLEANED_UP=1

  local exit_code=$?
  if [[ "${LEAN_DEV_KEEP_TEMP:-0}" == "1" ]]; then
    echo "[lean-dev] keeping temp dir: $LEAN_TMP_DIR"
  else
    rm -rf "$LEAN_TMP_DIR"
    echo "[lean-dev] removed temp dir"
  fi
  exit "$exit_code"
}

trap cleanup EXIT INT TERM

export VITE_CACHE_DIR="$LEAN_TMP_DIR/vite-cache"
export CARGO_TARGET_DIR="$LEAN_TMP_DIR/cargo-target"

mkdir -p "$VITE_CACHE_DIR" "$CARGO_TARGET_DIR"

echo "[lean-dev] using temporary cache dir: $LEAN_TMP_DIR"

echo "[lean-dev] starting app with: pnpm tauri dev $*"
cd "$ROOT_DIR"
pnpm tauri dev "$@"
