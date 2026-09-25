---
name: local-dev
---

# local-dev — ForhemitDYI

Record of the LOCAL-DEV onboarding run for this repository.

## What this repo is

A docs-only "design bible": 57+ Markdown design documents for an
ownership-transition platform, plus guardrail tooling. There is no
application, no dependency manifest, no Docker/Compose, no external
services — local dev means the documentation toolchain, not a dev stack.

## Environment

- Python 3.13.14 (stdlib only; no pip installs needed)
- GNU bash 5.2
- Git hooks must be armed once per clone: `./scripts/setup.sh`
  (sets `core.hooksPath=.githooks` and the `git sync` alias)

## Setup performed (2026-09-24 onboarding)

1. `./scripts/setup.sh` — armed guardrails. Confirmed
   `core.hooksPath=.githooks`, `git sync -> !./scripts/push.sh`.
2. `./scripts/refresh-analysis.sh` — exit 0; committed gap analysis is
   current, working tree left clean (no rewrite needed).
3. `python3 scripts/doc-graph.py --help` — 59 documents, 512 boundary
   edges, 96 link edges parsed.
4. `python3 scripts/doc-graph.py --check` — all 13 checks clean, exit 0.

## Definition of healthy

- `python3 scripts/doc-graph.py --check` exits 0 ("all 13 checks clean").
- `./scripts/refresh-analysis.sh` exits 0 and leaves the tree clean when
  the report is current.
- `git status` clean after the above.

## Gotchas

- Bare `git push` silently skips the pre-push hook when there are no new
  commits; use `git sync` / `./scripts/push.sh` instead.
- A missing `python3` degrades the pre-commit hook to a warning (stale
  report) rather than a block — don't rely on commit failures to detect it.
- Generated artifacts `analysis/graph.json` and `analysis/engine-graph.mmd`
  are gitignored; regenerate with `scripts/doc-graph.py`.

## No services

Nothing to install, migrate, seed, or start. No ports. No env vars required
(`DOC_GRAPH_PYTHON` is an optional interpreter override).
