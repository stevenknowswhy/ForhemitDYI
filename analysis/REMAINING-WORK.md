# Remaining Work — ForhemitDYI design bible

**Hand-maintained.** Unlike `analysis/GAP-ANALYSIS.md`, which `scripts/doc-graph.py`
regenerates on every commit, this file is written by hand and is never regenerated.
It lives under `analysis/` rather than the repo root on purpose: `load_docs()` globs
only `*.md` at the root, so the root stays exactly "the design bible" and this file
cannot affect any check.

Purpose: a durable, compaction-proof backlog. If a session is summarized or
compacted, read this file and run the analyzer — that is enough to resume.

---

## How to check progress

```bash
/Users/stephenstokes/.workbuddy-ai/binaries/python/versions/3.13.12/bin/python3 scripts/doc-graph.py
```

Compare the output against the snapshot below. A metric that has not moved is a
track that has not been worked.

## Progress snapshot

| Metric | 2026-09-19 18:06 | Target |
| --- | --- | --- |
| `boundary-tier-none` — docs with no boundary section | 10 | 0 |
| `boundary-tier-prose` — boundary in prose only | 8 | 0 |
| boundary tables | 15 | 33 |
| `never-named` — engine named by no boundary table | 2 | 0 |
| `naming-drift` — one engine, several raw names | 2 | 0 |
| `structural-ambiguity` | 5 | decisions made (not necessarily 0) |
| `declaration-consistency`, `readme-roster`, `no-dangling`, `stale-absence`, `undeclared-reference`, `no-orphans`, `no-layer-violation`, `coverage` | 0 | 0 |

Note `boundary-tier-none` + `boundary-tier-prose` + boundary tables = **33**, which is
the total number of engine documents. That is where the target of 33 comes from.

---

## Track A — Boundary coverage (mechanical; no decisions needed)

### A1. Add a boundary table to the 10 docs that have none

Order matters. Do the transaction-layer four first — they are the next edges in the
financing flow (Capital → Underwriting → …).

- [ ] `Professional Review Engine v1.0.md`
- [ ] `Professional Review Package Engine v1.0.md`
- [ ] `Seller-Note Liquidity Engine v1.0.md`
- [ ] `Document Readiness & Checklist Engine v1.0.md`
- [ ] `Business Reality Engine v1.0 - Current State Business Assessment.md`
- [ ] `Destination Engine v1.0.md`
- [ ] `Marketplace Engine v1.0.md`
- [ ] `Journey Builder Architecture & Employee Ownership Journey.md`
- [ ] `Stakeholder Document & Visibility Architecture.md`
- [ ] `Vendor Administration - Vetting Engine v1.0.md`

### A2. Convert the 8 prose-only boundaries into tables

These already *describe* their boundary in prose, so this is transcription rather than
authoring — the cheapest remaining win.

- [ ] `Audit - Provenance Engine.md`
- [ ] `Communication Engine.md`
- [ ] `Consent & Access Engine.md`
- [ ] `Decision Record Engine.md`
- [ ] `Local Vault - Workspace Engine.md`
- [ ] `Notification Engine.md`
- [ ] `Transaction - Orchestration Engine.md`
- [ ] `Workflow Engine.md`

---

## Track B — Vocabulary hygiene (mechanical)

### B1. Resolve the 2 naming-drift cases

Pick **one** raw form per engine and use it everywhere.

- [ ] Blog — currently `**Blog / Publishing**` and `**Blog Engine**`
- [ ] Vendor Administration — currently `**Vendor Administration / Vetting**` and `**Vendor Vetting**`

### B2. Clear the 2 engines that no boundary table names

A self-row does **not** count — self-references are excluded from inbound edges by
design. Each of these needs a row in a *related* document's table.

- [ ] `Billing` — easiest via `Identity & Access Engine.md` (entitlements relate to identity), which already has a table
- [ ] `Journey` — needs a related doc that has a table; `Destination` or `Business Reality` once A1 lands

---

## Track C — Architecture decisions (Stefano's call; not mechanical)

### C1. The 5 structural ambiguities

- [ ] Document Intelligence + Fact Verification share one document
- [ ] Confidence + Evidence Ledger + Research share one document
- [ ] Professional Determination is a node in the section-20 flow diagram, between Professional Review and Document Readiness, but is declared as no engine and sits in no layer
- [ ] `Stakeholder` maps to `Stakeholder Document & Visibility Architecture.md`, but section 7 describes a Stakeholder / Relationship engine that owns who participates and why
- [ ] `Journey` maps to `Journey Builder Architecture & Employee Ownership Journey.md` out of three candidate journey documents

### C2. Two "Engine"-titled documents declared nowhere in `ENGINE_HOME`

- [ ] `Professional Marketplace Engine v1.0.md` — the curation principle; distinct from `Marketplace Engine v1.0.md`, which is the discovery layer
- [ ] `Journey-Builder Rules and Journey Definition Engine.md`

Declaring them raises `boundary-tier-none` by 2 and `never-named` by 2 unless they also
gain boundary sections. Decide: declare, rename, or retire.

### C3. Version-marked / superseded documents

- [ ] `Journey Builder Architecture & Employee Ownership Journey.md` is headed "## Version 0.1" while `Employee Ownership Journey v0.2 - Optimized Guided Journey.md` exists — six journey-ish docs in total
- [ ] Two marketplace documents
- [ ] `Standalone Engine Architecture - Ownership Transition Engine Platform.md` overlaps `Core Architecture Principle - No Engine Owns the Entire Transaction.md`

Recommended: retire or banner superseded documents rather than leaving version markers.
One current document per concept.

---

## Track D — Tooling and process hardening

- [ ] **Latent: `never-named` groups by home document.** A multi-engine doc whose engines are named unevenly hides the unnamed one. Per-engine and per-home agree today (both `{Journey, Billing}`), so this is latent, not currently wrong. Harden only if a multi-engine doc ever becomes the sole home of an unnamed engine.
- [ ] **A "definition of done" for a new engine.** Adding one requires: the document, a boundary table, `DOCUMENT-INDEX.md`, the README group table, the README roster, and `ENGINE_HOME` / `LAYERS` / `ALIASES`. Most of this is now enforced by `declaration-consistency`, `readme-roster` and `coverage` — consider a scaffolder.
- [ ] **Per-engine status field** (designed / reviewed / built). The "Missing:" confusion happened because status had to be inferred from prose.
- [ ] **Prioritise by the MVP path, not by file order.** The README scopes the MVP to stop at *Professional Review*, so boundary completeness matters most along journey → destination → business reality → scenario → financial modeling → valuation → professional review.

---

## Completed (do not redo)

- [x] `Underwriting Engine.md` written (2,120 lines, 79 sections) and wired in — `61fd4fa`, `7fc06f6`, `16214fb`
- [x] `Capital - Financing Engine v1.0.md` boundary summary (19 rows) — `7ce8f1f`
- [x] `readme-roster` check added (the README prose roster was read by nothing) — `705a9e3`
- [x] `Goal-to-Reality Confidence Research Engine.md` boundary summary (18 rows, 3 engines) — `0203cea`
- [x] `Expanded Reality Architecture - Document Intelligence and Fact Verification.md` boundary summary (15 rows, 2 engines) — `62e1cf6`
- [x] `declaration-consistency` check + `LAYERLESS`; self-referencing boundary rows no longer count as inbound; `Security` removed from the README roster and from `LAYERS`

---

## The recipe (so it need not be re-derived)

**Adding a boundary table to an existing document:**

1. Derive the vocabulary from the analyzer, not from memory — print canonical name → set of raw variants actually in use, then copy the dominant form verbatim into every row.
2. Place it near the end, *before* the closing section ("Final Principle" / "North Star" / "One-Sentence Definition").
3. Renumber the heading you displaced.
4. Match the document's own style — some separate sections with `---`, some do not; some number in caps, some in mixed case.
5. **Confirm `naming-drift` did not change.** If it grew, you invented a raw form. This is the best self-check the recipe has.
6. Boundary edges should increase by **exactly** your row count. If not, a row failed to resolve.
7. A document declaring **N** engines needs **N** self-rows, not one.

**Committing:**

- One document per commit. Subject line only — no body, no attribution footer.
- `./scripts/refresh-analysis.sh`, then commit. Push with **`git sync`** (= `scripts/push.sh`), never plain `git push`.
- Verify: `git rev-parse HEAD origin/main` must match, and `git status --short` must be empty.

**Hazards (all hit at least once):**

- macOS BSD `grep` needs `-E` for alternation; a bare `|` pattern silently matches nothing and reads as "no hits". Use the Grep tool instead.
- The Edit tool has reported success without persisting in this project. Verify structural edits by reading the value back.
- An "Add a document" commit cannot be kept report-free — the pre-commit hook always drags the regenerated report into the first commit. No commit ordering fixes this; prefer document → index → analyzer.
