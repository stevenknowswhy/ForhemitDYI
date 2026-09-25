# ForhemitDYI — Obvious agent guidance

Design-documentation repository ("design bible") for an ownership-transition
platform. The content is the product at this stage: one Markdown document per
design engine, kept verbatim, guarded by repo tooling. There is **no
application** here — no app server, no database, no services to start.

## Stack

- Content: Markdown (57+ design documents at repo root)
- Tooling: Bash scripts + a Python 3 (stdlib-only) analyzer
- Package manager: none — nothing to install
- External services: none
- Env vars: none required; optional `DOC_GRAPH_PYTHON` (path to python
  interpreter, defaults to `python3`)

## Commands

Run from repo root:

| Command | Purpose |
| --- | --- |
| `./scripts/setup.sh` | Arm guardrails after a fresh clone (sets `core.hooksPath=.githooks`, `git sync` alias). Run once per clone — already done in this sandbox. |
| `python3 scripts/doc-graph.py --check` | Run the 13 structural checks over the doc corpus. Exit 0 = clean. |
| `./scripts/refresh-analysis.sh` | Regenerate `analysis/GAP-ANALYSIS.md` and stage it if changed. Runs automatically on every commit (pre-commit hook). |
| `./scripts/check-pending.sh` | Report uncommitted content; exit 1 if any exists. |
| `./scripts/push.sh` (alias `git sync`) | Check for uncommitted work, then push. Use this instead of bare `git push`. |
| `./scripts/sync-docs.sh` | Commit every pending doc, one document per commit (repo convention). |

Git hooks are active in this sandbox (`core.hooksPath=.githooks`): pre-commit
regenerates the gap analysis; pre-push refuses to push `main` with uncommitted
files.

## Codebase map

Tiny repo — map inlined here (top-level depth 2):

| Path | Contents |
| --- | --- |
| `/` (root) | The design corpus: one Markdown doc per engine, plus `README.md` (thesis + load-bearing principles) and `DOCUMENT-INDEX.md` (annotated roster of all docs). |
| `analysis/` | `GAP-ANALYSIS.md` — versioned, machine-generated design report; `archive/` — superseded report versions by timestamp. `graph.json` / `engine-graph.mmd` are generated artifacts (gitignored). |
| `scripts/` | `setup.sh` (arm guardrails), `doc-graph.py` (corpus analyzer, 13 checks), `refresh-analysis.sh`, `check-pending.sh`, `push.sh`, `sync-docs.sh`. |
| `.githooks/` | `pre-commit`, `pre-push`, `README.md` — the guardrails wired by `setup.sh`. |

## Local verification

Primary flow: `python3 scripts/doc-graph.py --check` must exit 0 with
"all 13 checks clean" (roster agreement, boundary quality, housekeeping).
Secondary: `./scripts/refresh-analysis.sh` exits 0 and leaves the working tree
clean when the committed report is current.

## Repo conventions

- Commit one document per commit: `git add "<file>" && git commit -m "Add <Title>"`.
- Never bypass the guardrails casually; bypass is deliberate only
  (`--no-verify`).
- Design docs are kept verbatim — edits are new documents or supersessions,
  not silent rewrites.

## Snapshot

- Snapshot ID: `bwndapti4ks2s57yj50y:default`
- Captured: 2026-09-24T18:28:28.756Z (onboarding run, toolchain verified healthy)
