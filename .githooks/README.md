# `.githooks/` — repository guardrails

## What lives here

| File | Purpose |
|---|---|
| `pre-push` | Refuses to push `main` while any file is left uncommitted. |

Companion script: [`scripts/sync-docs.sh`](../scripts/sync-docs.sh) — clears the backlog the guard would catch, one document per commit.

## Why this exists

This repo holds one Markdown document per design engine, committed one file at a time. Work sessions routinely end with freshly written docs that were never staged, so a bare `git push` silently ships a stale tree to GitHub — and the only way to notice was to remember to check.

The guard converts that from a habit you have to recall into a property of the system.

## Arming the hooks (required once per clone)

Git hooks are not activated by cloning — `.git/hooks/` and `core.hooksPath` both live in the *unversioned* `.git/` directory, for security reasons. After a fresh clone, run:

```sh
git config --local core.hooksPath .githooks
```

Verify it took:

```sh
git config --get core.hooksPath     # -> .githooks
```

To disarm:

```sh
git config --local --unset core.hooksPath
```

## Behaviour

The guard fires on **`git push` targeting `main` only**. Feature and WIP branches push freely.

It checks the working tree with `git status --porcelain`, which covers untracked, modified, staged and deleted files in one pass. Ignored paths (`.workbuddy-ai/`, `.DS_Store`) are excluded by git itself, so they never trigger it.

- **Tree clean** → push proceeds silently.
- **Anything pending** → push is blocked, the pending files are listed, and the two ways forward are printed.

## Deliberate bypass

```sh
git push --no-verify
```

`--no-verify` skips all hooks. That is the intended escape hatch — it should be a conscious choice, not an accident.

## The two ways forward when blocked

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

The sweep maps `Capital - Financing Engine v1.0.md` to the subject `Add Capital / Financing Engine v1.0`, matching the existing commit history. New files get `Add`, tracked edits get `Update`, deletions get `Remove`.
