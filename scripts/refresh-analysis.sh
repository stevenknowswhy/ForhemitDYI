#!/usr/bin/env bash
#
# refresh-analysis.sh — keep the versioned gap analysis honest.
#
# analysis/GAP-ANALYSIS.md is committed, so it can drift: edit a document and
# the committed report silently describes the previous corpus. This runs
# before every commit, regenerates the report, and stages the result. The
# outgoing version is archived to analysis/archive/ under the timestamp it
# carried, so no version is ever lost.
#
# If the report is already accurate AND already staged this does nothing at all
# — no rewrite, no archive, no staging. That is what makes it safe to run on
# every commit. If the report is accurate on disk but stale in the index it is
# staged, because a commit ships the index, not the working tree.
#
#   exit 0  report is current (regenerated and staged, or already up to date)
#   exit 1  regeneration failed — the commit is blocked
#
# Called by .githooks/pre-commit. Deliberate bypass: git commit --no-verify
#
set -uo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null)" || exit 0
cd "$root" || exit 0

python_bin="${DOC_GRAPH_PYTHON:-python3}"

# A missing interpreter is a warning, not a wall. Blocking every commit on a
# machine without python3 would be worse than a report that lags.
if ! command -v "$python_bin" >/dev/null 2>&1; then
  printf '\n'
  printf '  NOTE — the gap analysis was not refreshed.\n'
  printf '\n'
  printf '      %s not found on PATH, so analysis/GAP-ANALYSIS.md may now be\n' "$python_bin"
  printf '      stale. Set DOC_GRAPH_PYTHON, or refresh it later with:\n'
  printf '\n'
  printf '      ./scripts/refresh-analysis.sh\n'
  printf '\n'
  exit 0
fi

# --staged-files prints the paths whose CONTENT the regeneration changed, and
# nothing when the report on disk is already accurate. It regenerates as a
# side effect.
staged="$("$python_bin" scripts/doc-graph.py --staged-files)" || {
  printf '\n'
  printf '  COMMIT BLOCKED — scripts/doc-graph.py failed.\n'
  printf '\n'
  printf '  Fix the error above, then commit again. To bypass deliberately:\n'
  printf '\n'
  printf '      git commit --no-verify\n'
  printf '\n'
  exit 1
}

# Regeneration compares the report against the working tree, but a commit ships
# the INDEX, and those two can disagree. Run ./scripts/refresh-analysis.sh by
# hand, then commit without staging the result, and regeneration changes
# nothing while the commit still carries the previous report — exactly the
# drift this guard exists to prevent. So stage the report whenever the index
# has fallen behind it.
report="analysis/GAP-ANALYSIS.md"
if ! git diff --quiet -- "$report"; then
  staged="$(printf '%s\n%s' "$staged" "$report")"
fi

staged="$(printf '%s\n' "$staged" | sed '/^[[:space:]]*$/d' | sort -u)"

[ -n "$staged" ] || exit 0

printf '\n'
printf '  Gap analysis refreshed — the committed report no longer matched the corpus:\n'
printf '\n'
printf '%s\n' "$staged" | sed 's/^/      /'
printf '\n'

while IFS= read -r f; do
  [ -n "$f" ] || continue
  git add -- "$f"
done <<< "$staged"

printf '  Staged. The report now describes the tree you are committing.\n'
printf '\n'

exit 0
