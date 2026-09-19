#!/usr/bin/env bash
#
# push.sh — the safe way to push. Checks for uncommitted work FIRST, then pushes.
#
# Why not just `git push`? Because git skips the pre-push hook entirely when it
# has nothing new to send — it prints "Everything up-to-date" and exits 0. That
# is precisely the moment an uncommitted backlog is invisible, and it is the
# failure this repo's guardrails exist to prevent. This wrapper always checks,
# so the "has commits" and "nothing to push" paths behave identically.
#
#   ./scripts/push.sh          check, then push
#   ./scripts/push.sh -- ...   extra args are passed through to git push
#
# Wire it up as `git sync` with ./scripts/setup.sh
#
set -uo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  printf 'Not inside a git repository.\n' >&2
  exit 1
}
cd "$root" || exit 1

if ! "$root/scripts/check-pending.sh" \
     "PUSH PAUSED — uncommitted work would be left behind:"; then
  printf '  Nothing was pushed.\n'
  printf '\n'
  printf '  Clear the backlog, then push again:\n'
  printf '\n'
  printf '      ./scripts/sync-docs.sh && ./scripts/push.sh\n'
  printf '\n'
  exit 1
fi

printf '\n  Working tree clean — pushing.\n\n'
exec git push "$@"
