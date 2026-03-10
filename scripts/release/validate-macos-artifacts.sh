#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
dmg_path="${1:-src-tauri/target/release/bundle/dmg/SignalFlow_*.dmg}"
app_archive_path="${2:-src-tauri/target/release/bundle/macos/SignalFlow.app.tar.gz}"

shopt -s nullglob
dmg_matches=( $dmg_path )
archive_matches=( $app_archive_path )
shopt -u nullglob

if (( ${#dmg_matches[@]} == 0 )); then
  echo "No DMG artifact matched: $dmg_path"
  exit 1
fi

if (( ${#archive_matches[@]} == 0 )); then
  echo "No macOS app archive matched: $app_archive_path"
  exit 1
fi

dmg="${dmg_matches[0]}"
app_archive="${archive_matches[0]}"
mount_dir="$(mktemp -d /tmp/signalflow-dmg.XXXXXX)"

cleanup() {
  if mount | grep -q "$mount_dir"; then
    hdiutil detach "$mount_dir" >/dev/null 2>&1 || true
  fi
  rm -rf "$mount_dir"
}
trap cleanup EXIT

echo "Validating DMG: $dmg"
hdiutil attach "$dmg" -nobrowse -readonly -mountpoint "$mount_dir" >/dev/null
if [[ ! -d "$mount_dir/SignalFlow.app" ]]; then
  echo "Mounted DMG does not contain SignalFlow.app"
  exit 1
fi
hdiutil detach "$mount_dir" >/dev/null

echo "Validating app archive: $app_archive"
if ! tar -tzf "$app_archive" | grep -q '^SignalFlow\.app/'; then
  echo "macOS app archive does not contain SignalFlow.app"
  exit 1
fi

echo "macOS artifacts validated successfully."
