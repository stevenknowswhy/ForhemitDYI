# Repository guardrails

Keeps the design bible fully committed before anything reaches GitHub.

## Files

| File | Purpose |
|---|---|
| `.githooks/pre-push` | Automatic guard. Blocks a push to `main` while files are uncommitted. |
| `.githooks/pre-commit` | Automatic guard. Regenerates the versioned gap analysis before committing. |
| `scripts/check-pending.sh` | The uncommitted-content check. Shared by the hook and the push wrapper. |
| `scripts/refresh-analysis.sh` | The analysis refresh. Called by the pre-commit hook; runnable by hand. |
| `scripts/doc-graph.py` | The analyzer behind the gap analysis report. |
| `scripts/push.sh` | Explicit guard. Checks **always**, then pushes. Wired up as `git sync`. |
| `scripts/sync-docs.sh` | Clears the backlog — commits every pending doc, one per commit. |
| `scripts/setup.sh` | Arms both hooks and the `git sync` alias. Run once per clone. |

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

## The versioned gap analysis

`analysis/GAP-ANALYSIS.md` is a committed design record: a dependency graph of
the whole corpus, built by `scripts/doc-graph.py` from the boundary tables, the
section-20 flow diagram, and the layer roster.

Committing a generated file buys a readable record at the cost of **drift**.
Edit any document and the committed report quietly describes the old corpus.
Nothing in git notices.

`.githooks/pre-commit` closes that. Before each commit it regenerates the
report and stages the result:

```
$ git commit -m "Add Scenario Engine"
  Gap analysis refreshed — the report no longer matched the corpus:
      analysis/archive/GAP-ANALYSIS-2026-09-19-1605.md
      analysis/GAP-ANALYSIS.md
  Staged. The report now describes the tree you are committing.
```

The outgoing version is **archived, not discarded** — moved to
`analysis/archive/` under the timestamp it carried, so you can read the gaps as
they stood on any given date. The current report always lives at the stable
path `analysis/GAP-ANALYSIS.md`.

**When the report is already accurate, nothing happens.** No rewrite, no
archive, no staging — an unrelated commit passes straight through. That is what
makes it safe to run on every commit.

To refresh by hand, without committing:

```sh
./scripts/refresh-analysis.sh
```

If `python3` is missing the hook prints a note and lets the commit through,
rather than blocking all work on a machine without an interpreter. If `python3`
is present but the analyzer fails, the commit is blocked.

## Deliberate bypass

```sh
git commit --no-verify     # skip the analysis refresh
git push   --no-verify     # skip the uncommitted-content guard
```

`--no-verify` skips all hooks, so it bypasses the guards. Both should be a
conscious choice, not an accident.
