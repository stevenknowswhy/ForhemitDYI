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

| Metric | 2026-09-20 10:40 | Target |
| --- | --- | --- |
| `boundary-tier-none` — docs with no boundary section | **0** ✅ | 0 |
| `boundary-tier-prose` — boundary in prose only | **0** ✅ | 0 |
| boundary tables | **34** | 34 |
| `never-named` — engine named by no boundary table | **0** ✅ | 0 |
| `naming-drift` — one engine, several raw names | **0** ✅ | 0 |
| `structural-ambiguity` | **0** ✅ | decisions made (not necessarily 0) |
| `declaration-consistency`, `readme-roster`, `no-dangling`, `stale-absence`, `undeclared-reference`, `no-orphans`, `no-layer-violation`, `coverage` | 0 | 0 |
| boundary edges | **495** | — |

Note `boundary-tier-none` + `boundary-tier-prose` + boundary tables = **34**, which is
the total number of engine documents. That is where the target of 34 comes from.

---

## Track A — Boundary coverage (mechanical; no decisions needed)

### A1. Add a boundary table to the 6 docs that have none

**Probed 2026-09-19 — this is not one task, it is two.** The question asked was: *is the
boundary already decided somewhere, or would writing the table force a decision?* Each of the
six was checked for explicit ownership language (`does not own`, `not responsible for`,
`does not do`, `never … own/decide/determine`) and for an existing boundary section.

#### A1a — the boundary is already written; transcribe it (4 docs, mechanical)

- [x] `Destination Engine v1.0.md` — §47 Rule 10 *"Downstream engines consume the destination but do not own it"*; §47 NORTH STAR names Business Reality, Research, Confidence, Scenario and professionals; §48 ONE-SENTENCE DEFINITION supplies the positive half plus the explicit split *"the Destination Engine owns the owner's intent, while the Journey Engine owns the conversation that discovers it."* **(9 rows)**
- [x] `Marketplace Engine v1.0.md` — §61 WHAT THE MARKETPLACE ENGINE DOES NOT DO (10 items) supplies the negative half; §2's seven CORE MARKETPLACE PRINCIPLES supply the positive half; §18 *"matching should never override user preference."* **(11 rows)**
- [x] `Vendor Administration - Vetting Engine v1.0.md` — §61 WHAT THIS ENGINE DOES NOT DO (10 items) plus its own summary line *"It manages marketplace eligibility and trust information."* Closing section is §63 NORTH STAR. **(15 rows)**
- [x] `Business Reality Engine v1.0 - Current State Business Assessment.md` — thinnest of the four but present: its closing line *"Business Reality becomes the factual counterweight to Destination. Neither is allowed to overwrite the other."* plus §19 CURRENT STATE SHOULD NEVER CHANGE THE DESTINATION AUTOMATICALLY and §34 RELATIONSHIP WITH GOAL-TO-REALITY CONFIDENCE. It is also named by **19** other boundary tables — the most-referenced engine in the corpus — so its boundary is already described extensively from the outside. **(14 rows)**

#### A1b — boundary undecided? REVISED 2026-09-19: the boundary *is* decided; the DOCUMENT is not (2 docs)

**Correction.** Both were classified "boundary undecided". Checking what the corpus already says about them shows the **boundary is decided in both cases** — it is the **document that is unsettled**. That is a smaller and different problem, and it moves the blocker from C1 to C3.

- [x] `Stakeholder Document & Visibility Architecture.md` — **DONE 2026-09-20** (10 rows): self-row `**Stakeholder / Relationship**` owns who participates and what each stakeholder may see; `## Hard Boundary` locks it against Consent & Access (authorization) and Identity & Access (identities). The §7 relationship-layer-vs-visibility-layer question is unchanged — it is documentation scope (Track C3), not boundary.
  - `Identity & Access Engine.md` (pre-existing): *"Why a person or organization participates in a transaction"*
  - `Consent & Access Engine.md`: *"Who participates in the transaction and what they may see"*
  - `Communication Engine.md` §337: *"Participants should be references to the **Stakeholder / Relationship Engine** rather than duplicate identity records."*

  `Complete Architecture` §7 is headed **"Stakeholder / Relationship Engine"** and marked *Partly built*: *"covers the document-visibility layer. The relationship layer this section proposes — who participates, and why — is still not its own document."*

  **So the open question is documentation scope, not architecture:** does the relationship layer get its own document, or is it folded into the visibility document? The boundary table can be written either way — it is the same boundary. The visibility layer's claim that its three levels *"are data-permission policies"* does overlap Consent & Access; the three tables above already resolve it — **Stakeholder owns who participates and what they may see; Consent & Access owns the authorization decision that permits it.**

- [x] `Journey Builder Architecture & Employee Ownership Journey.md` — **DONE 2026-09-20** (12 rows): self-row `**Journey**` owns the conversation that discovers the owner's intent and the journey framework; `## Hard Boundary` locks it against Destination (the intent), Transaction / Orchestration (the execution system layered onto the same framework), and Professional Determination (the conclusions). The boundary was likewise already decided:
  - `Document Readiness & Checklist Engine v1.0.md` and `Professional Review Engine v1.0.md`, identically: *"The evolving ownership journey and its stages"*
  - `Destination Engine v1.0.md` §48: *"the Destination Engine owns the owner's intent, while the Journey Engine owns the conversation that discovers it."*

  **The real blocker is supersession, not identity.** The current home document is headed `## Version 0.1` while `Employee Ownership Journey v0.2 - Optimized Guided Journey.md` exists, and **four** journey-named documents sit in the corpus. That is Track C3.

**Revised conclusion: A1b's blocker is document identity and supersession (Track C3), not engine identity (Track C1).** That is a smaller decision — *which document is current, and retire the rest* — and C3 already recommends it. C1 stays open for `Professional Determination` (the genuinely undecided item) and for the two multi-engine register entries.

**UPDATE 2026-09-20 — both A1b boundary tables are now written.** The tables were never blocked on the boundary: it was decided in both cases. What remains is the C3 question the tables do not answer — which journey document is current (four candidates; the current home is headed `## Version 0.1`), and whether the Stakeholder relationship layer gets its own document. **Track A (boundary coverage) is now COMPLETE: `boundary-tier-none` = 0.**

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

- [x] Blog — `**Blog Engine**` (in the WordPress Management table) standardized to `**Blog / Publishing**` (the Blog doc's own self-row form)
- [x] Vendor Administration — `**Vendor Vetting**` (in the Billing table) standardized to `**Vendor Administration / Vetting**` (now dominant, matches the filename)

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

### C1. The 5 structural ambiguities — ✅ ALL RESOLVED 2026-09-20

**Decision inputs, measured 2026-09-19.** Three things were measured before proposing an order.

**1. The blast radius is small — and uneven.** Rows in *existing* boundary tables that name each engine:

| Engine | Named by | Prose mentions |
| --- | --- | --- |
| `Journey` | **2** tables | 40 documents |
| `Stakeholder` | **3** tables | 21 documents |
| `Professional Review` (settled, for contrast) | **24** tables | — |

Resolving Journey or Stakeholder therefore costs **5 table rows** today. That number only grows as more tables are written — an argument for deciding *before* A1a, not after. It is also small enough that the cost of getting the order wrong is bounded either way.

**2. The five items are not the same kind of thing.** Reading the check itself:

- **2 items are computed structural facts** — *"N engines share one document"*, derived generically from `ENGINE_HOME` (`len(ks) > 1`). Any multi-engine document is flagged, with **no suppression mechanism**. But the corpus has *decided* to allow multi-engine documents: both instances now carry N self-rows, exactly as the recipe prescribes. So these two are a **decision with no way to record it** — the identical shape to `Security` before `CROSS_CUTTING` existed. Fix: a register analogous to `LAYERLESS`, e.g. `MULTI_ENGINE_DOCS`, so "deliberately multi-engine" stops reading as an open question.
- **1 item is computed and genuinely open** — *Professional Determination is a node in the section-20 flow diagram but no engine*. **RESOLVED 2026-09-19:** the engine is now built (`Professional Determination Engine v1.0.md`), wired into `ENGINE_HOME` / `LAYERS` / `ALIASES`, and the `structural-ambiguity` gating was fixed so the finding clears once the engine is declared (it now also checks `ENGINE_HOME`). The §20 node still names it; that is expected.
- **2 items are hardcoded strings, not detections.** The Stakeholder and Journey findings are literal `collapsed.append(...)` calls in `run_checks()`. Nothing computes them, so **no amount of document editing will ever clear them** — they are a to-do list embedded in the analyzer, resolvable only by editing the script. Worth knowing before treating them as findings that respond to work.

**3. So C1 is two tasks, not one:** a **decision** (what are Journey, Stakeholder and Professional Determination?) and a **tooling** task (add a multi-engine register; delete the two hardcoded strings once decided).

- [x] Document Intelligence + Fact Verification share one document → **RESOLVED 2026-09-20:** registered in `MULTI_ENGINE_DOCS` with a stated reversal condition ("split only if either engine ever gains a home document of its own").
- [x] Confidence + Evidence Ledger + Research share one document → **RESOLVED 2026-09-20:** registered in `MULTI_ENGINE_DOCS`, same form.
- [x] Professional Determination is a node in the section-20 flow diagram, between Professional Review and Document Readiness → **RESOLVED 2026-09-19:** engine built and wired (`Professional Determination Engine v1.0.md`); the `structural-ambiguity` gating now also checks `ENGINE_HOME`, so the finding clears. The §20 node still names it; expected.
- [x] `Stakeholder` maps to `Stakeholder Document & Visibility Architecture.md`, but section 7 describes a Stakeholder / Relationship engine that owns who participates and why → **the hardcoded string is GONE (2026-09-20).** It was not a detection: no document edit could ever clear it. The underlying question is documentation scope and now lives in **C3**.
- [x] `Journey` maps to `Journey Builder Architecture & Employee Ownership Journey.md` out of three candidate journey documents → **the hardcoded string is GONE (2026-09-20)**, same reasoning. The underlying question is supersession and now lives in **C3**.

**What the C1 tooling change actually did** (`scripts/doc-graph.py`, 2026-09-20):

- Added `MULTI_ENGINE_DOCS`, a register analogous to `LAYERLESS`. A multi-engine document is ambiguous only while it is *unexplained*; the register is where the explanation lives, so "deliberately multi-engine" stops reading as an open question.
- Registered documents are still **printed** in section 6 under "Multi-engine documents on record — decided, so not counted above". Suppression is not silence: the decision stays visible in the report.
- Deleted both hardcoded `collapsed.append(...)` strings. A finding that no amount of work can clear is a to-do list embedded in a detector, and it trains you to ignore the detector.
- Extended `declaration-consistency` to validate the register, so a **stale** entry surfaces instead of quietly suppressing a real finding. An exemption set nobody checks is just a way to hide things.

**Regression probes run** (a check that finds nothing is indistinguishable from a broken one):

| Probe | Expected | Result |
| --- | --- | --- |
| Point an extra engine at an **unregistered** document | fires | ✅ `Scenario, Valuation -> one document (Scenario Engine.md)` |
| Add a **stale** `MULTI_ENGINE_DOCS` entry | fires | ✅ `…but only one engine (Scenario) has it as home` |
| Remove `Professional Determination` from `ENGINE_HOME` | fires | ✅ the §20 flow-diagram detector re-fires |

`structural-ambiguity` 4 → 0, and **every check in the analyzer is now 0.**

*Note (2026-09-19): building Professional Determination also cleared a `stale-absence` finding — `Scenario Engine.md` had an example heading `### Missing Professional Determination` that the check now correctly read as "this engine is absent". Renamed to `### Awaiting Professional Determination` (commit d23f9c0). This is the check working: a "Missing X" heading naming a now-built engine is a real signal, fixed in the doc, not the check.*

### C2. Two "Engine"-titled documents declared nowhere in `ENGINE_HOME`

- [ ] `Professional Marketplace Engine v1.0.md` — the curation principle; distinct from `Marketplace Engine v1.0.md`, which is the discovery layer
- [ ] `Journey-Builder Rules and Journey Definition Engine.md`

Declaring them raises `boundary-tier-none` by 2 and `never-named` by 2 unless they also
gain boundary sections. Decide: declare, rename, or retire.

### C3. Version-marked / superseded documents

- [ ] `Journey Builder Architecture & Employee Ownership Journey.md` is headed "## Version 0.1" while `Employee Ownership Journey v0.2 - Optimized Guided Journey.md` exists — six journey-ish docs in total. *(Migrated here from C1 on 2026-09-20: it was a hardcoded analyzer string no document edit could clear.)*
- [ ] **Stakeholder / Relationship: does the relationship layer get its own document?** `Complete Architecture` §7 marks it *Partly built* — the visibility layer exists, the "who participates, and why" layer does not. The boundary table is written either way (it is the same boundary); this is a scope decision. *(Migrated here from C1 on 2026-09-20, same reason.)*
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
- [x] **A1a — the four remaining boundary summaries** (2026-09-19): `Destination` (9 rows), `Marketplace` (11 rows), `Vendor Administration / Vetting` (15 rows), `Business Reality` (14 rows). Each was transcription from the document's own "does not own" / "Architectural Lock" / NORTH STAR, not authoring. Tables 27 → 31, edges 403 → 452 (+49, exactly the row count), `boundary-tier-none` 6 → 2. Commits `c9ab3ef`, `51338d5`, `2d6c04b`, `3a753f9`.
- [x] **C1 — `structural-ambiguity` cleared** (2026-09-20): added the `MULTI_ENGINE_DOCS` register (analogous to `LAYERLESS`) for the two deliberately multi-engine documents, deleted the two hardcoded Stakeholder/Journey strings — which no document edit could ever clear — and extended `declaration-consistency` to validate the register so a stale entry surfaces. Registered docs are still printed in report section 6 as "decided, so not counted above". Three regression probes confirm the detectors still fire. **4 → 0, and every check in the analyzer is now 0.**
- [x] **B1 — naming-drift resolved** (2026-09-19): `**Vendor Vetting**` → `**Vendor Administration / Vetting**` (Billing table) and `**Blog Engine**` → `**Blog / Publishing**` (WordPress Management table). `naming-drift` 2 → 0. Commits `79309ad`, `e7aebf4`. Note: a stray untracked archive (`1905.md`) was created by a manual `scripts/doc-graph.py` run used to confirm the fix; it was committed separately (`e875d84`) — lesson reinforced: verify with `/tmp/verify.py` (parser-only), never the publisher, to avoid orphan archives.

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
