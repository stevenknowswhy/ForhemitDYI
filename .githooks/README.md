# Repository guardrails

Keeps the design bible fully committed before anything reaches GitHub.

## Files

| File | Purpose |
|---|---|
| `.githooks/pre-push` | Automatic guard. Blocks a push to `main` while files are uncommitted. |
| `scripts/check-pending.sh` | The check itself. Shared by the hook and the push wrapper. |
| `scripts/push.sh` | Explicit guard. Checks **always**, then pushes. Wired up as `git sync`. |
| `scripts/sync-docs.sh` | Clears the backlog — commits every pending doc, one per commit. |
| `scripts/setup.sh` | Arms the hook and the `git sync` alias. Run once per clone. |

## Why this exists

This repo holds one Markdown document per design engine, committed one file at a time. Work sessions routinely end with freshly written docs that were never staged, so a bare `git push` silently ships a stale tree — and the only way to notice was to remember to check.

The guardrails convert that from a habit you have to recall into a property of the system.

## Arming (required once per clone)

Git hooks and aliases live in the unversioned `.git/` directory — for security reasons, cloning a repo never activates them. After a fresh clone:

```sh
./scripts/setup.sh
```

That sets `core.hooksPath=.githooks`, adds the `git sync` alias, and marks the scripts executable.

To disarm:

```sh
git config --local --unset core.hooksPath
git config --local --unset alias.sync
```

## Use `git sync`, not `git push`

This is the important part.

**A pre-push hook alone is not enough.** Git only invokes `pre-push` when it actually has something to send. If the working tree has no new commits, git short-circuits:

```
$ git push
Everything up-to-date      # exit 0 — hook never ran
```

That is exactly the moment an uncommitted backlog is invisible. You'd see "Everything up-to-date", assume you were synced, and leave documents behind. The guard would have made things *worse* by creating false confidence.

`git sync` closes it. It runs the check unconditionally, then delegates to `git push`:

```
$ git push                  # no new commits
Everything up-to-date

$ git sync                  # same situation
  PUSH PAUSED — uncommitted work would be left behind:
  1 uncommitted file(s) in the working tree:
      ?? "Closing Engine v1.0.md"
  Nothing was pushed.
```

The hook still runs underneath, so a push that *does* have commits to send is guarded either way. The two paths now behave identically.

## Behaviour

The guard targets **`main` only**. Feature and WIP branches push freely.

It checks the tree with `git status --porcelain --untracked-files=all`, which covers untracked, modified, staged and deleted files in one pass. Ignored paths (`.workbuddy-ai/`, `.DS_Store`) are excluded by git itself, so they never trigger it.

- **Tree clean** → push proceeds silently.
- **Anything pending** → push is stopped, pending files are listed, both ways forward are printed.

## Clearing a backlog

Commit the documents yourself, one at a time (the repo convention):

```sh
git add "Closing Engine v1.0.md" && git commit -m "Add Closing Engine v1.0"
```

Or let the sweep do the whole backlog, deriving subjects from filenames:

```sh
./scripts/sync-docs.sh              # preview, then confirm
./scripts/sync-docs.sh --yes        # skip the confirmation
./scripts/sync-docs.sh --dry-run    # preview only
```

The sweep maps `Capital - Financing Engine v1.0.md` to `Add Capital / Financing Engine v1.0`, reproducing the existing commit history exactly. New files get `Add`, tracked edits get `Update`, deletions get `Remove`.

Typical session end:

```sh
./scripts/sync-docs.sh && git sync
```

## Deliberate bypass

```sh
git push --no-verify
```

`--no-verify` skips all hooks, so it bypasses the guard. It should be a conscious choice, not an accident.
