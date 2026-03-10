#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
dmg_path="${1:-src-tauri/target/release/bundle/dmg/SignalFlow_*.dmg}"

shopt -s nullglob
dmg_matches=( $dmg_path )
shopt -u nullglob

if (( ${#dmg_matches[@]} == 0 )); then
  echo "No signed DMG artifact matched: $dmg_path"
  exit 1
fi

dmg="${dmg_matches[0]}"
mount_dir="$(mktemp -d /tmp/signalflow-signed-dmg.XXXXXX)"

cleanup() {
  if mount | grep -q "$mount_dir"; then
    hdiutil detach "$mount_dir" >/dev/null 2>&1 || true
  fi
  rm -rf "$mount_dir"
}
trap cleanup EXIT

echo "Checking Gatekeeper on DMG: $dmg"
spctl -a -vv --type open "$dmg"

echo "Mounting signed DMG"
hdiutil attach "$dmg" -nobrowse -readonly -mountpoint "$mount_dir" >/dev/null

app_path="$mount_dir/SignalFlow.app"
if [[ ! -d "$app_path" ]]; then
  echo "Mounted DMG does not contain SignalFlow.app"
  exit 1
fi

echo "Checking Gatekeeper on app bundle"
spctl -a -vv --type exec "$app_path"

echo "Checking code signing details"
codesign --verify --deep --strict --verbose=2 "$app_path"

echo "Signed macOS distribution validated successfully."
