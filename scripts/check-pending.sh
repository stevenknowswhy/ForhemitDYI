#!/usr/bin/env bash
#
# check-pending.sh — report uncommitted content; exit 1 if any exists.
#
# Shared by .githooks/pre-push (the automatic guard) and scripts/push.sh (the
# explicit guard), so both paths print identical guidance.
#
#   exit 0  working tree is clean
#   exit 1  pending files exist (listed on stdout)
#
# Optional first argument is a headline printed above the file list.
#
set -uo pipefail

headline="${1:-}"

cd "$(git rev-parse --show-toplevel)" 2>/dev/null || exit 0

# --porcelain covers untracked, modified, staged and deleted in one pass.
# Ignored paths (.workbuddy-ai/, .DS_Store) are excluded by git itself.
dirty="$(git status --porcelain --untracked-files=all)"
[ -n "$dirty" ] || exit 0

count="$(printf '%s\n' "$dirty" | grep -c .)"

printf '\n'
if [ -n "$headline" ]; then
  printf '  %s\n' "$headline"
  printf '\n'
fi
printf '  %s uncommitted file(s) in the working tree:\n' "$count"
printf '\n'
git status --short --untracked-files=all | sed 's/^/      /'
printf '\n'
printf '  Commit them one document at a time (repo convention):\n'
printf '\n'
printf '      git add "<file>" && git commit -m "Add <Title>"\n'
printf '\n'
printf '  Or let the sweep commit the whole backlog for you:\n'
printf '\n'
printf '      ./scripts/sync-docs.sh\n'
printf '\n'

exit 1
