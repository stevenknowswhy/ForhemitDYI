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

| Metric | 2026-09-19 18:24 | Target |
| --- | --- | --- |
| `boundary-tier-none` — docs with no boundary section | 6 | 0 |
| `boundary-tier-prose` — boundary in prose only | **0** ✅ | 0 |
| boundary tables | **27** | 33 |
| `never-named` — engine named by no boundary table | **0** ✅ | 0 |
| `naming-drift` — one engine, several raw names | 2 | 0 |
| `structural-ambiguity` | 5 | decisions made (not necessarily 0) |
| `declaration-consistency`, `readme-roster`, `no-dangling`, `stale-absence`, `undeclared-reference`, `no-orphans`, `no-layer-violation`, `coverage` | 0 | 0 |
| boundary edges | **403** | — |

Note `boundary-tier-none` + `boundary-tier-prose` + boundary tables = **33**, which is
the total number of engine documents. That is where the target of 33 comes from.

---

## Track A — Boundary coverage (mechanical; no decisions needed)

### A1. Add a boundary table to the 6 docs that have none

**Probed 2026-09-19 — this is not one task, it is two.** The question asked was: *is the
boundary already decided somewhere, or would writing the table force a decision?* Each of the
six was checked for explicit ownership language (`does not own`, `not responsible for`,
`does not do`, `never … own/decide/determine`) and for an existing boundary section.

#### A1a — the boundary is already written; transcribe it (4 docs, mechanical)

- [ ] `Destination Engine v1.0.md` — §47 Rule 10 *"Downstream engines consume the destination but do not own it"*; §47 NORTH STAR names Business Reality, Research, Confidence, Scenario and professionals; §48 ONE-SENTENCE DEFINITION supplies the positive half plus the explicit split *"the Destination Engine owns the owner's intent, while the Journey Engine owns the conversation that discovers it."*
- [ ] `Marketplace Engine v1.0.md` — §61 WHAT THE MARKETPLACE ENGINE DOES NOT DO (10 items) supplies the negative half; §2's seven CORE MARKETPLACE PRINCIPLES supply the positive half; §18 *"matching should never override user preference."*
- [ ] `Vendor Administration - Vetting Engine v1.0.md` — §61 WHAT THIS ENGINE DOES NOT DO (10 items) plus its own summary line *"It manages marketplace eligibility and trust information."* Closing section is §63 NORTH STAR.
- [ ] `Business Reality Engine v1.0 - Current State Business Assessment.md` — thinnest of the four but present: its closing line *"Business Reality becomes the factual counterweight to Destination. Neither is allowed to overwrite the other."* plus §19 CURRENT STATE SHOULD NEVER CHANGE THE DESTINATION AUTOMATICALLY and §34 RELATIONSHIP WITH GOAL-TO-REALITY CONFIDENCE. It is also named by **19** other boundary tables — the most-referenced engine in the corpus — so its boundary is already described extensively from the outside.

#### A1b — the boundary is genuinely undecided; the table would force a decision (2 docs, BLOCKED)

- [ ] `Journey Builder Architecture & Employee Ownership Journey.md` — has a positive definition (§28 Product Architecture in One Sentence) but **no statement of what it does not own**, and only 2 other tables name it. Its own identity is unresolved.
- [ ] `Stakeholder Document & Visibility Architecture.md` — asserts its three visibility levels *"are data-permission policies"*, which overlaps Consent & Access's remit, and the document covers only the visibility layer while a relationship layer is proposed. No ownership language at all.

**These two are not merely "not yet done" — they are blocked on Track C1.** Both are already
on the `structural-ambiguity` list, for exactly the reason that makes their boundary
unwritable:

> * Stakeholder maps to `Stakeholder Document & Visibility Architecture.md`, but that document is about document visibility; section 7 describes a Stakeholder / Relationship engine that owns who participates and why
> * Journey maps to `Journey Builder Architecture & Employee Ownership Journey.md` out of three candidate journey documents

**Writing a boundary table for an engine whose identity is undecided would freeze an
undecided structure into the graph.** The table would assert a boundary that C1 may then move
or rename, and the analyzer would treat it as settled. Resolve C1 first; A1b then becomes
mechanical like the rest.

**The correlation worth remembering: the A1 documents that need authoring are precisely the
ones `structural-ambiguity` already flags.** When a mechanical track stalls, check whether the
analyzer has already explained why.

### A2. Convert the 8 prose-only boundaries into tables — ✅ DONE 2026-09-19

All eight were transcribed rather than authored: each already described its boundary in
prose, so the rows came from the document's own "What This Engine Owns / Does Not Own"
pair and its "Architectural Lock" section. See Completed below.

---

## Track B — Vocabulary hygiene (mechanical)

### B1. Resolve the 2 naming-drift cases

Pick **one** raw form per engine and use it everywhere.

- [ ] Blog — currently `**Blog / Publishing**` and `**Blog Engine**`
- [ ] Vendor Administration — currently `**Vendor Administration / Vetting**` and `**Vendor Vetting**`

### B2. Clear the 1 engine that no boundary table names — ✅ DONE 2026-09-19

`Billing` was cleared by the **Notification** boundary table's `**Billing / Commercial**`
row, not by `Identity & Access` as originally planned. `never-named` is now **0**.

The lesson is worth keeping: **`never-named` is cleared from the other side, by a related
document naming the engine — never by the engine's own document.** Both times it has been
cleared (Journey, then Billing) it happened as a *side effect* of writing a boundary table
for an unrelated engine, not by targeting it. If you need to clear one, write the table for
the engine it is most naturally adjacent to and it will fall out on its own.

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
- [x] **A1 — the four transaction-layer boundary summaries** (2026-09-19): `Professional Review Engine v1.0` (19 rows, and it also cleared `Journey` from `never-named`), `Professional Review Package Engine v1.0` (19 rows), `Seller-Note Liquidity Engine v1.0` (17 rows), `Document Readiness & Checklist Engine v1.0` (19 rows). Tables 15 → 19, edges 198 → 272, `boundary-tier-none` 10 → 6, `never-named` 2 → 1.
- [x] **A2 — the eight prose-only boundary summaries** (2026-09-19): `Audit / Provenance` (15 rows), `Communication` (15), `Consent & Access` (17), `Decision Record` (15), `Local Vault / Workspace` (15), `Notification` (16), `Transaction / Orchestration` (19), `Workflow` (19). Tables 19 → 27, boundary edges 272 → 403, **`boundary-tier-prose` 8 → 0**. Commits `0effe66`, `660f557`, `cc2ea91`, `8199702`, `e9fba92`, `a8a9249`, `2b7fbde`, `d4b5b7c`.
- [x] **B2 — `never-named` cleared** (2026-09-19): the new Notification table names `Billing / Commercial`. 1 → 0. A free consequence of A2, not separately targeted.

---

## The recipe (so it need not be re-derived)

**Adding a boundary table to an existing document:**

1. Derive the vocabulary from the analyzer, not from memory — print canonical name → set of raw variants actually in use, then copy the dominant form verbatim into every row.
2. **Place it as the second-to-last section**, immediately before the document's final section. Verified against all 19 pre-existing tables: in every one, the summary sits directly before the closing section. Not "somewhere near the end" — the section *before* the closing one.
3. **Put the self-row first.** 15 of the 19 existing tables do (Underwriting is the lone outlier). One row per declared engine, then the related engines.
4. Renumber the heading you displaced.
5. Match the document's own style — some separate sections with `---`, some do not; some number in caps (`# 44. ARCHITECTURAL BOUNDARY SUMMARY`), some in title case (`# 73. Architectural Boundary Summary`). Read the *document*, not the house style. Section shape: `# N. …Boundary Summary` → table → `## Hard Boundary` prose → the closing section.
6. **Confirm `naming-drift` did not change.** If it grew, you invented a raw form. This is the best self-check the recipe has — it caught a real one: a row written as `**Stakeholder**` when the corpus's only established form is `**Stakeholder / Relationship**`.
7. Boundary edges should increase by **exactly** your row count. If not, a row failed to resolve.
8. A document declaring **N** engines needs **N** self-rows, not one.
9. **Verify with the parser, not the publisher.** Running `scripts/doc-graph.py` publishes the report and archives the outgoing version. Doing that on every intermediate attempt litters `analysis/archive/` with reports of states that never shipped, which then have to be deleted by hand. Read `parse_boundary_tables()` directly while iterating; publish once, when the edit is final.
10. Existing tables run **8–19 rows**. Hub engines (Capital, Professional Review, Transaction / Orchestration, Workflow) sit at 19; infrastructure engines at 8–12.

**Committing:**

- One document per commit. Subject line only — no body, no attribution footer.
- `./scripts/refresh-analysis.sh`, then commit. Push with **`git sync`** (= `scripts/push.sh`), never plain `git push`.
- Verify: `git rev-parse HEAD origin/main` must match, and `git status --short` must be empty.

**Hazards (all hit at least once):**

- macOS BSD `grep` needs `-E` for alternation; a bare `|` pattern silently matches nothing and reads as "no hits". Use the Grep tool instead.
- The Edit tool has reported success without persisting in this project. Verify structural edits by reading the value back.
- An "Add a document" commit cannot be kept report-free — the pre-commit hook always drags the regenerated report into the first commit. No commit ordering fixes this; prefer document → index → analyzer.
