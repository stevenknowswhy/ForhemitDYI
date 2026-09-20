#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

new_repo() {
  local name="$1"
  local repo="$TMP/$name"
  mkdir -p "$repo"
  cp "$ROOT/scripts/sync-docs.sh" "$repo/sync-docs.sh"
  cd "$repo"
  git init -q
  git config user.email test@example.com
  git config user.name "Repository Test"
  git add sync-docs.sh
  git commit -q -m bootstrap
}

assert_single_path_commit() {
  local revision="$1"
  local expected="$2"
  local actual
  actual="$(git show --pretty='' --name-only "$revision" | sed '/^$/d' | paste -sd '|')"
  test "$actual" = "$expected" || {
    printf 'Expected %s to contain %s; got %s\n' "$revision" "$expected" "$actual" >&2
    exit 1
  }
}

# Multiple pre-staged files remain isolated.
new_repo staged
printf 'one\n' > one.md
printf 'two\n' > two.md
git add one.md two.md
git commit -q -m initial
printf 'changed one\n' > one.md
printf 'changed two\n' > two.md
git add one.md two.md
bash sync-docs.sh --yes >/dev/null
assert_single_path_commit HEAD~1 one.md
assert_single_path_commit HEAD two.md

# Mixed staged and unstaged files are both committed, separately.
new_repo mixed
printf 'a\n' > staged.md
printf 'b\n' > unstaged.md
git add staged.md unstaged.md
git commit -q -m initial
printf 'staged change\n' > staged.md
printf 'unstaged change\n' > unstaged.md
git add staged.md
bash sync-docs.sh --yes >/dev/null
assert_single_path_commit HEAD~1 staged.md
assert_single_path_commit HEAD unstaged.md

# Spaces are preserved literally.
new_repo spaces
printf 'space\n' > "file with spaces.md"
bash sync-docs.sh --yes >/dev/null
assert_single_path_commit HEAD "file with spaces.md"

# A rename remains one commit containing source and destination.
new_repo rename
printf 'rename\n' > old.md
git add old.md
git commit -q -m initial
git mv old.md "new name.md"
bash sync-docs.sh --yes >/dev/null
rename_status="$(git show --format='' --name-status HEAD)"
case "$rename_status" in
  R*"old.md"$'\t'"new name.md") ;;
  *)
    printf 'Unexpected rename status: %s\n' "$rename_status" >&2
    exit 1
    ;;
esac

# A deletion is committed.
new_repo deletion
printf 'remove\n' > remove.md
git add remove.md
git commit -q -m initial
rm remove.md
bash sync-docs.sh --yes >/dev/null
test "$(git show --format='' --name-status HEAD)" = $'D\tremove.md'

# A hook-generated file is discovered after the first commit and gets its own
# follow-up commit rather than joining or being left behind.
new_repo hook
printf 'source\n' > source.md
printf 'report v1\n' > analysis.md
git add source.md analysis.md
git commit -q -m initial
printf 'source changed\n' > source.md
cat > .git/hooks/pre-commit <<'HOOK'
#!/usr/bin/env bash
set -eu
if [ ! -f .git/hook-ran ]; then
  : > .git/hook-ran
  printf 'report v2\n' > analysis.md
  git add analysis.md
fi
HOOK
chmod +x .git/hooks/pre-commit
bash sync-docs.sh --yes >/dev/null
generated_commit_paths="$(git show --pretty='' --name-only HEAD | sed '/^$/d' | sort | paste -sd '|')"
test "$generated_commit_paths" = "analysis.md|source.md" || {
  printf 'Unexpected hook-generated commit paths: %s\n' "$generated_commit_paths" >&2
  exit 1
}
test -z "$(git status --porcelain)"

printf 'sync-docs regression suite passed\n'