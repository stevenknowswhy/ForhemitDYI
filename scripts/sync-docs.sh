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

# Read porcelain with NUL delimiters so spaces, quotes, tabs, renames and
# deletions are preserved exactly. For rename/copy entries Git emits a second
# path after the destination path.
collect_pending() {
  statuses=()
  paths=()
  old_paths=()

  while IFS= read -r -d '' entry; do
    status="${entry:0:2}"
    path="${entry:3}"
    old_path=""

    if [[ "$status" == *R* || "$status" == *C* ]]; then
      IFS= read -r -d '' old_path || {
        printf 'Malformed rename/copy entry from git status.\n' >&2
        exit 1
      }
    fi

    statuses+=("$status")
    paths+=("$path")
    old_paths+=("$old_path")
  done < <(git status --porcelain=v1 -z --untracked-files=all)
}

verb_for() {
  case "$1" in
    '??') printf 'Add' ;;
    *D*)  printf 'Remove' ;;
    *R*)  printf 'Rename' ;;
    *)    printf 'Update' ;;
  esac
}

collect_pending

if [ "${#paths[@]}" -eq 0 ]; then
  printf '\n  Working tree is clean — nothing to sync.\n\n'
  exit 0
fi

printf '\n  %s pending file(s) — will be committed individually:\n\n' "${#paths[@]}"
for i in "${!paths[@]}"; do
  verb="$(verb_for "${statuses[$i]}")"
  printf '      %-7s %s\n' "$verb" "$(subject_for "$(basename "${paths[$i]}")")"
  if [ -n "${old_paths[$i]}" ]; then
    printf '              %s -> %s\n' "${old_paths[$i]}" "${paths[$i]}"
  else
    printf '              %s\n' "${paths[$i]}"
  fi
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

# Execute dynamically. Hooks may generate and stage a companion artifact (for
# example the versioned gap analysis); Git correctly includes that generated
# artifact with the source change that caused it. Re-read status after every
# commit so any remaining generated work is not silently left staged.
made=0
while true; do
  collect_pending
  [ "${#paths[@]}" -gt 0 ] || break

  status="${statuses[0]}"
  path="${paths[0]}"
  old_path="${old_paths[0]}"
  verb="$(verb_for "$status")"
  subject="$verb $(subject_for "$(basename "$path")")"
  # For a staged rename/copy, adding the destination is sufficient, but the
  # commit pathspec must include both source and destination to preserve the
  # rename as one isolated commit.
  stage_pathspec=("$path")
  commit_pathspec=("$path")
  [ -z "$old_path" ] || commit_pathspec+=("$old_path")

  if ! git add -A -- "${stage_pathspec[@]}"; then
    printf '  FAILED to stage: %s\n' "$path" >&2
    exit 1
  fi
  if git diff --cached --quiet -- "${commit_pathspec[@]}"; then
    printf '  skipped (no staged change): %s\n' "$path"
    continue
  fi

  # `-- <pathspec>` isolates this commit from unrelated pre-staged files.
  if ! git commit -q --only -m "$subject" -- "${commit_pathspec[@]}"; then
    printf '  FAILED to commit: %s\n' "$path" >&2
    exit 1
  fi
  printf '  committed  %s\n' "$subject"
  made=$((made + 1))
done

printf '\n  %s commit(s) created. Push when ready:\n\n      git push\n\n' "$made"
