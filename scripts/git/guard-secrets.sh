#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
if ! command -v gitleaks >/dev/null 2>&1; then
  if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    echo "Skipping local gitleaks enforcement in GitHub Actions."
    echo "Secrets scanning is handled by the dedicated secrets workflow."
    exit 0
  fi
  echo "gitleaks not found. Install gitleaks to enforce secret scanning."
  exit 1
fi

gitleaks protect --staged --redact
