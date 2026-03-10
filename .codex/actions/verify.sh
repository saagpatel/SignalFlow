#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=.codex/actions/_artifact_env.sh
source "$SCRIPT_DIR/_artifact_env.sh"

cd "$REPO_ROOT"
bash .codex/scripts/run_verify_commands.sh
