#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
branch="$(git rev-parse --abbrev-ref HEAD)"
pattern='^codex/(feat|fix|chore|refactor|docs|test|perf|ci|spike|hotfix)/[a-z0-9]+(-[a-z0-9]+)*$'

if [[ "${GITHUB_ACTIONS:-}" == "true" && "$branch" == "HEAD" ]]; then
  echo "Skipping branch-name enforcement for detached HEAD in GitHub Actions."
  exit 0
fi

if [[ "$branch" == "main" || "$branch" == "master" ]]; then
  echo "Direct work on $branch is blocked."
  exit 1
fi

if ! [[ "$branch" =~ $pattern ]]; then
  echo "Invalid branch: $branch"
  echo "Expected: codex/<type>/<slug>"
  exit 1
fi
