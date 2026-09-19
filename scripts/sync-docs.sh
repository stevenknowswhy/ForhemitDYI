#!/usr/bin/env bash
#
# sync-docs.sh — commit every pending design document, one document per commit.
#
# Companion to .githooks/pre-push. Where the guard refuses a push that would
# ship a stale tree, this script clears the backlog for you while honouring the
# repo's one-document-per-commit convention.
#
#   ./scripts/sync-docs.sh              preview, then ask for confirmation
#   ./scripts/sync-docs.sh --yes        skip the confirmation
#   ./scripts/sync-docs.sh --dry-run    preview only, commit nothing
#
set -uo pipefail

ASSUME_YES=0
DRY_RUN=0
for arg in "$@"; do
  case "$arg" in
    -y|--yes)     ASSUME_YES=1 ;;
    -n|--dry-run) DRY_RUN=1 ;;
    -h|--help)    sed -n '2,12p' "$0"; exit 0 ;;
    *) printf 'Unknown option: %s\n' "$arg" >&2; exit 2 ;;
  esac
done

cd "$(git rev-parse --show-toplevel)" 2>/dev/null || {
  printf 'Not inside a git repository.\n' >&2; exit 1
}

# Filename -> commit subject fragment, matching this repo's existing style:
#   "Capital - Financing Engine v1.0.md" -> "Capital / Financing Engine v1.0"
#   "Integration Engine.md"              -> "Integration Engine"
subject_for() {
  printf '%s' "${1%.md}" | sed 's| - | / |g'
}

# Collect pending entries as "<XY><space><path>" straight from porcelain.
pending=()
while IFS= read -r line; do
  [ -n "$line" ] || continue
  pending+=("$line")
done < <(git status --porcelain --untracked-files=all)

if [ "${#pending[@]}" -eq 0 ]; then
  printf '\n  Working tree is clean — nothing to sync.\n\n'
  exit 0
fi

# Build the plan.
verbs=(); paths=(); subjects=()
for entry in "${pending[@]}"; do
  status="${entry:0:2}"
  path="${entry:3}"
  case "$status" in
    '??')  verb='Add' ;;
    *D*)   verb='Remove' ;;
    *)     verb='Update' ;;
  esac
  verbs+=("$verb")
  paths+=("$path")
  subjects+=("$(subject_for "$(basename "$path")")")
done

printf '\n  %s pending file(s) — will be committed individually:\n\n' "${#paths[@]}"
for i in "${!paths[@]}"; do
  printf '      %-7s %s\n' "${verbs[$i]}" "${subjects[$i]}"
  printf '              %s\n' "${paths[$i]}"
done
printf '\n'

if [ "$DRY_RUN" -eq 1 ]; then
  printf '  Dry run — nothing was committed.\n\n'
  exit 0
fi

if [ "$ASSUME_YES" -ne 1 ]; then
  printf '  Commit these %s file(s) now? [y/N] ' "${#paths[@]}"
  read -r reply
  case "$reply" in
    y|Y|yes|YES) ;;
    *) printf '\n  Aborted — nothing was committed.\n\n'; exit 0 ;;
  esac
  printf '\n'
fi

# Execute: stage, then commit each file on its own.
made=0
for i in "${!paths[@]}"; do
  path="${paths[$i]}"
  subject="${verbs[$i]} ${subjects[$i]}"

  if ! git add -A -- "$path"; then
    printf '  FAILED to stage: %s\n' "$path" >&2
    exit 1
  fi
  if git diff --cached --quiet -- "$path"; then
    printf '  skipped (no staged change): %s\n' "$path"
    continue
  fi
  if ! git commit -q -m "$subject"; then
    printf '  FAILED to commit: %s\n' "$path" >&2
    exit 1
  fi
  printf '  committed  %s\n' "$subject"
  made=$((made + 1))
done

printf '\n  %s commit(s) created. Push when ready:\n\n      git push\n\n' "$made"
