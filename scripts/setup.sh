#!/usr/bin/env bash
#
# setup.sh — arm the repository guardrails after a fresh clone.
#
# Git hooks and aliases live in the unversioned .git/ directory, so cloning the
# repo does not activate them. Run this once per clone.
#
#   ./scripts/setup.sh
#
set -uo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  printf 'Not inside a git repository.\n' >&2
  exit 1
}
cd "$root" || exit 1

git config --local core.hooksPath .githooks
git config --local alias.sync '!./scripts/push.sh'
chmod +x .githooks/pre-push scripts/*.sh

printf '\n  Guardrails armed:\n\n'
printf '      core.hooksPath = %s\n' "$(git config --get core.hooksPath)"
printf '      git sync       -> %s\n' "$(git config --get alias.sync)"
printf '\n'
printf '  Use `git sync` instead of `git push`.\n'
printf '  It checks for uncommitted work even when git has nothing new to send.\n'
printf '\n'
