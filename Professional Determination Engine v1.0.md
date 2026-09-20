The user (the business owner) creates their own framework: a Destination that states what they want, a Business Reality that records what is true today, Scenarios that explore what might work, and a Confidence signal that measures how well the two align. A qualified professional then judges whether each stated goal is achievable. This engine is the analytical workbench that makes that judgment fast, structured, and attributable — it does the comparison, proposes the category and the modification path, and leaves the conclusion to the professional.

# Professional Determination Engine v1.0

## 1. Purpose

The Professional Determination Engine analyzes a user's stated objectives and goals against the framework the user has created, and produces a determination for each goal:

* **Feasible** — achievable as specified under the current framework.
* **Feasible with modifications** — the intent is achievable, but not as literally specified; the engine lists the specific modifications that would make it achievable.
* **Infeasible** — impossible or unrealistic under the current circumstances; the engine explains why.

Every determination carries a brief rationale that cites the framework facts that drove it. The engine is built **for use by professionals**: it produces a draft determination; the professional reviews it, may adjust it, and adopts it as their own conclusion. The adopted determination is attributed to that professional and never silently becomes platform truth.

---

# 2. CORE PRINCIPLE

This engine operates inside the locked three decision layers:

> **Owner Objective** (what the owner wants) / **Platform Scenario** (what might work under stated assumptions) / **Professional Determination** (what a qualified professional concludes).

The owner brings the goal. The platform organizes it into a framework. The engine compares the goal to that framework. The professional concludes.

Three rules constrain everything below:

* **The platform is a navigator, not an advisor** (Load-Bearing Principle 5). The engine compares and classifies; it does not give legal, tax, valuation, investment, or financing advice. Where a conclusion depends on professional expertise, the engine routes it to a qualified professional through Professional Review.
* **Professional Determination never becomes platform truth.** A determination is attributed to the professional who adopted it. It informs the journey; it does not overwrite the owner's objectives or the platform's scenarios.
* **Nonnegotiables are hard constraints** (Load-Bearing Principle 4). A must-have is never silently relaxed, ignored, or overridden. When a goal conflicts with a nonnegotiable, the conflict is surfaced and the owner decides — the engine never resolves it on their behalf.

---

# 3. WHAT THE ENGINE ANALYZES

The engine takes two things:

1. **The goal set** — the owner's stated objectives and goals, drawn from the Destination they created (desired outcome, owner objectives, priorities, nonnegotiables, and preferences).
2. **The framework the user created** — the structured representations the owner (and the platform's decision engines) have built: Business Reality (current-state facts), Scenario (potential paths), Financial Modeling (projections), Valuation (value), Confidence (goal-alignment signal), Evidence Ledger (claims and their evidence), Decision Record (prior decisions), and Research (external evidence).

The determination is the result of comparing the first against the second, goal by goal.

---

# 4. THE FRAMEWORK THE USER CREATES

The engine reads these framework components. It does not own any of them; it consumes their current state as inputs.

| Framework component | What it contributes to the determination |
| --- | --- |
| **Destination** | The goals, objectives, priorities, nonnegotiables, and preferences being tested. |
| **Business Reality** | The current-state business facts the goals are tested against — the ground truth of "what is true today." |
| **Scenario** | The potential transaction paths; establishes what is structurally possible. |
| **Financial Modeling** | Calculations and projections under stated assumptions; establishes what is fundable. |
| **Valuation** | Value range and supporting analysis; bounds what the goals can realistically rest on. |
| **Confidence** | The goal-to-reality alignment signal; flags goals that need deeper professional review. |
| **Evidence Ledger** | Claims and their supporting evidence; lets the engine cite sources rather than assert. |
| **Decision Record** | Prior owner decisions and rationale; prevents re-litigating settled questions. |
| **Research** | External evidence about the business and its market; supports or contradicts assumptions. |

---

# 5. DETERMINATION INPUTS

The engine is invoked with a `GoalDeterminationRequest`:

## `GoalDeterminationRequest`

* `requestId` — unique identifier for this determination run.
* `goalRefs` — the specific goals (or the whole goal set) to be assessed.
* `frameworkSnapshot` — versioned references to the Destination, Business Reality, Scenario, Financial Modeling, Valuation, Confidence, Evidence Ledger, Decision Record, and Research states at the moment of assessment. The determination is always made against a captured snapshot, never against a moving live state, so the rationale is reproducible.
* `professionalContext` — the professional performing the review (role, specialty, engagement), used for attribution and for routing expertise-specific questions.
* `scope` — whether this is a full goal-set assessment or a single-goal re-determination.

---

# 6. THE DETERMINATION MODEL

For each goal the engine produces exactly one `GoalDetermination` with a single category from the closed set:

* `FEASIBLE`
* `FEASIBLE_WITH_MODIFICATIONS`
* `INFEASIBLE`

The three categories are mutually exclusive and exhaustive. A goal is never left "unclassified." If the engine cannot reach a conclusion (insufficient framework, missing Business Reality, unreconciled conflict), it returns `INFEASIBLE` with a rationale that names the missing input — it does not guess.

---

# 7. FEASIBILITY CATEGORIES

**Feasible.** The goal is achievable as specified. Every success criterion is supported by the framework: it is consistent with Business Reality, at least one Scenario path supports it, Financial Modeling and Valuation do not contradict it, and no nonnegotiable is violated. The rationale cites the framework facts that confirm it.

**Feasible with modifications.** The goal's *intent* is achievable, but not in its literal specified form. One or more bounded changes — to framing, structure, assumptions, timeline, scope, or financing — would make it achievable without violating a nonnegotiable. The engine enumerates those modifications. The category is chosen when a real, specific path to "yes" exists; vague "maybe with changes" is not enough.

**Infeasible.** Under the current circumstances the goal cannot be achieved as specified, and no bounded modification within the owner's stated nonnegotiables makes it achievable. It is impossible or unrealistic. The rationale cites the specific framework facts — for example, Business Reality contradicts it, Valuation sits below the owner's stated minimum, or Financial Modeling shows it is not fundable at the required terms.

---

# 8. ANALYSIS METHOD

The engine applies the same sequence to every goal:

1. **Normalize the goal.** Extract its objective, success criteria, constraints, and the nonnegotiables it touches.
2. **Internal consistency.** Does the goal contradict itself or another stated objective? A self-contradiction is `INFEASIBLE` with the conflict named.
3. **Test against Business Reality.** Is the goal consistent with what is true today? A goal that assumes facts the Business Reality record contradicts is `INFEASIBLE` or `FEASIBLE_WITH_MODIFICATIONS` depending on whether a modification resolves the gap.
4. **Test against the possibility space.** Does at least one Scenario path support the goal? Do Financial Modeling and Valuation permit it? If no path exists, `INFEASIBLE`.
5. **Nonnegotiable check.** Does the goal (or any of its modifications) violate a nonnegotiable? Nonnegotiables are never silently relaxed. A violation is surfaced as a conflict for the owner to decide; the engine does not resolve it.
6. **Confidence signal.** The Goal-to-Reality Confidence provides an alignment signal. Low confidence routes the goal to deeper professional review but does not by itself make it `INFEASIBLE`.
7. **Synthesize.** Choose the single category. If `FEASIBLE_WITH_MODIFICATIONS`, enumerate the modifications. Write the rationale citing specific framework facts.

---

# 9. MODIFICATION SYNTHESIS

For `FEASIBLE_WITH_MODIFICATIONS`, the engine proposes concrete, bounded modifications. Each modification names what changes, why it resolves the gap, and which framework fact it responds to.

Examples the engine may propose:

* Extend the timeline (e.g., "phase the transition over 36 months instead of 18").
* Adjust scope (e.g., "retain 90% of the workforce rather than 100%").
* Substitute structure (e.g., "an ESOP-weighted structure instead of an outright third-party sale").
* Recast the financing (e.g., "a smaller seller note supported by an SBA guarantee").
* Reconcile an assumption (e.g., "revenue assumed at $X conflicts with Business Reality; restate at $Y").

Modifications are proposals, not decisions. The professional and the owner decide which, if any, to adopt. Modifications never silently relax a nonnegotiable — a modification that would require dropping a nonnegotiable is surfaced as a conflict, not offered as a quiet fix.

---

# 10. RATIONALE REQUIREMENT

Every determination carries a brief rationale. A determination without a rationale is invalid and is rejected before it reaches a professional.

The rationale references the specific framework facts that drove the category:

* *Feasible:* "Consistent with Business Reality (EBITDA $X); Scenario S-3 supports the structure; Valuation range $A–$B above the owner's minimum."
* *Feasible with modifications:* "Intent achievable; Business Reality shows current staffing cannot support the 100% retention goal — modify to 90% (see Reality R-12)."
* *Infeasible:* "Business Reality shows current EBITDA cannot support the stated seller note; Valuation range $A–$B below the owner's stated minimum of $C. No bounded modification within nonnegotiables reaches feasibility."

The rationale is drawn from the framework snapshot and the Evidence Ledger so it is checkable, not asserted.

---

# 11. THE GOAL DETERMINATION OBJECT

## `GoalDetermination`

* `determinationId` — unique identifier.
* `goalRef` — the goal this determination addresses.
* `category` — `FEASIBLE` | `FEASIBLE_WITH_MODIFICATIONS` | `INFEASIBLE`.
* `rationale` — the brief, framework-citing explanation (required).
* `modifications` — the list of proposed modifications (empty unless `FEASIBLE_WITH_MODIFICATIONS`).
* `confidenceNote` — how the Confidence signal informed the call.
* `frameworkSnapshotRef` — the snapshot the determination was made against.
* `state` — `DRAFT` → `PROFESSIONAL_REVIEWED` → `ADOPTED` | `REJECTED` | `OVERRIDDEN`.
* `attributedTo` — the professional who adopted the determination (empty while `DRAFT`).
* `createdAt`, `adoptedAt` — timestamps.

---

# 12. PROFESSIONAL OVERSIGHT & ADOPTION

The engine is used by a professional, not autonomously by the platform.

* The engine produces a **draft** `GoalDetermination` for each goal.
* The professional reviews the draft — the category, the modifications, and the rationale.
* The professional may adjust any field, recording why, and then **adopts** it as their determination.
* An adopted determination carries `attributedTo` and is stored as a `ProfessionalDetermination` by the Professional Review Engine. It is the professional's conclusion, not the engine's, and never becomes platform truth.

The engine may mark a draft `INFEASIBLE` or surface a conflict, but only the professional (and, for nonnegotiable conflicts, the owner) makes the final call.

---

# 13. DETERMINATION STATES

* `DRAFT` — produced by the engine, not yet reviewed by a professional.
* `PROFESSIONAL_REVIEWED` — a professional has read and considered it.
* `ADOPTED` — the professional adopts it as their determination; `attributedTo` is set.
* `REJECTED` — the professional disagrees with the engine's draft and records their own conclusion elsewhere.
* `OVERRIDDEN` — the professional changes the category or modifications, recording the reason; the adopted version supersedes the draft.

A determination is only ever "live" once `ADOPTED`. Drafts inform the professional; they do not inform the journey.

---

# 14. STRUCTURED ASSESSMENT OUTPUT

The aggregate result is a `DeterminationAssessment`:

## `DeterminationAssessment`

* `assessmentId` — unique identifier.
* `requestRef` — the originating `GoalDeterminationRequest`.
* `determinations` — the list of `GoalDetermination` objects, one per goal.
* `summary` — counts by category, the headline findings, and the consolidated modifications queue (every modification proposed across the goal set, de-duplicated).
* `openConflicts` — nonnegotiable or framework conflicts the owner must decide.
* `frameworkSnapshotRef` — the snapshot the whole assessment was made against.

The assessment is the artifact a professional hands back into the journey: each goal, classified, with a reason and a path.

---

# 15. ENGINE INPUTS

The engine reads:

* Destination — goals, objectives, nonnegotiables, preferences.
* Business Reality — current-state facts.
* Scenario — potential paths.
* Financial Modeling — projections.
* Valuation — value analysis.
* Confidence — goal-alignment signal.
* Evidence Ledger — claims and evidence.
* Decision Record — prior decisions.
* Research — external evidence.

It reads these as versioned snapshots, not as a live feed, so a determination is reproducible.

---

# 16. ENGINE OUTPUTS

The engine produces:

* `GoalDeterminationRequest` acknowledgment.
* One `GoalDetermination` per goal (draft).
* A `DeterminationAssessment` aggregating them.
* An open-conflicts list for owner decisions.
* A modifications queue for the professional and owner to act on.

The outputs are handed to Professional Review for attribution and to the journey for use.

---

# 17. PROFESSIONAL DETERMINATION + PROFESSIONAL REVIEW

Professional Review is the collaboration and attribution system; this engine is the analytical workbench. The engine produces the draft determination; Professional Review records it as an attributed `ProfessionalDetermination`, preserves the professional's sign-off, and keeps it from becoming platform truth. The two are deliberately separate: the engine may conclude; only Professional Review may attribute.

---

# 18. PROFESSIONAL DETERMINATION + DESTINATION ENGINE

Destination owns the owner's desired outcome and the goals being tested. The engine reads the goal set from Destination and writes its determinations back as input to the evolving journey. The engine never edits the owner's objectives; an `INFEASIBLE` or `FEASIBLE_WITH_MODIFICATIONS` result is information for the owner, not a change to what they want.

---

# 19. PROFESSIONAL DETERMINATION + BUSINESS REALITY ENGINE

Business Reality owns the current-state facts the goals are tested against. It is the ground truth of "what is true today." The engine treats a Business Reality record as authoritative evidence; a goal that contradicts it is `INFEASIBLE` or `FEASIBLE_WITH_MODIFICATIONS` accordingly. Business Reality and Destination are explicitly never allowed to overwrite each other — the determination simply reports the gap.

---

# 20. PROFESSIONAL DETERMINATION + SCENARIO ENGINE

Scenario owns the potential paths; it says what to examine, not whether a goal is achievable. The engine uses Scenario to establish what is structurally possible and to source the modifications it proposes. A goal with no supporting Scenario path is `INFEASIBLE`.

---

# 21. PROFESSIONAL DETERMINATION + FINANCIAL MODELING ENGINE

Financial Modeling calculates what the numbers look like under stated assumptions. The engine uses those calculations to test fundability and to propose financing-related modifications. It does not perform the modeling; it consumes the model's outputs as framework facts.

---

# 22. PROFESSIONAL DETERMINATION + VALUATION ENGINE

Valuation owns value determinations made by qualified professionals. The engine uses the valuation range to bound what the goals can rest on. A goal that requires value above the valuation range (and above the owner's stated minimum) is `INFEASIBLE` with that fact cited. The engine does not perform valuation; it cites the professional's valuation.

---

# 23. PROFESSIONAL DETERMINATION + CONFIDENCE ENGINE

Confidence measures goal-to-reality alignment. The engine reads the Confidence signal as a flag: low confidence routes a goal to deeper professional review and strengthens the rationale's call for evidence, but it does not by itself determine the category.

---

# 24. PROFESSIONAL DETERMINATION + EVIDENCE LEDGER

The Evidence Ledger preserves claims and their supporting evidence. The engine cites Ledger entries in its rationales so every determination is checkable rather than asserted. The engine does not own the evidence; it references it.

---

# 25. PROFESSIONAL DETERMINATION + DECISION RECORD

The Decision Record preserves the owner's prior decisions and rationale. The engine reads it to avoid re-litigating settled questions and to respect decisions already made. A new determination that conflicts with a recorded decision surfaces the conflict rather than overriding it.

---

# 26. PROFESSIONAL DETERMINATION + RESEARCH ENGINE

Research investigates assumptions and external evidence. The engine uses Research output to support or contradict the goal's assumptions. Where a conclusion hinges on external evidence the Research Engine has not yet gathered, the engine flags the gap in its rationale.

---

# 27. PROFESSIONAL DETERMINATION + DOCUMENT READINESS

Each determination implies document needs — an `INFEASIBLE` goal may still require records explaining why; a `FEASIBLE_WITH_MODIFICATIONS` goal requires the documents behind its modifications. The engine hands its assessment to Document Readiness so the right documents are requested, but it does not own document status.

---

# 28. PROFESSIONAL DETERMINATION + MARKETPLACE ENGINE

The Marketplace finds curated professionals and verifies role fit. When a determination depends on professional expertise the engine cannot supply (legal, tax, valuation), it routes the question to a qualified professional through Marketplace and Professional Review. The engine does not select the professional for the owner.

---

# 29. PROFESSIONAL DETERMINATION + TRANSACTION / ORCHESTRATION

Transaction / Orchestration owns the execution plan. Adopted determinations inform the plan — what is feasible shapes what gets scheduled — but the engine does not execute or decide the transaction. It feeds the orchestration; it is not the orchestration.

---

# 30. PROFESSIONAL DETERMINATION + COMMUNICATION

Communication captures the conversation about determinations — questions, explanations, and the owner's reactions. The engine's structured output is the source material; Communication owns the dialogue around it, not the determination itself.

---

# 31. PROFESSIONAL DETERMINATION + CONSENT & ACCESS

Consent & Access governs who may see which determination, for what purpose, until when. A determination is sensitive (it states what is and is not achievable); the engine records the attribution and leaves visibility to Consent & Access. The engine does not decide who may see it.

---

# 32. PROFESSIONAL DETERMINATION + LOCAL VAULT

The Local Vault holds the private source documents the framework is built from. The engine reads framework state through the Vault's boundaries; it does not own or expose the underlying documents.

---

# 33. PROFESSIONAL DETERMINATION + AUDIT / PROVENANCE

Audit / Provenance records the history of determinations — who produced the draft, who adopted it, when, and against which framework snapshot. The engine's draft and the professional's adoption are both provenance events. The engine does not own the history; it is recorded by Audit.

---

# 34. PROFESSIONAL DETERMINATION + POLICY / COMPLIANCE

Policy / Compliance governs how engines may operate — including when professional review is required. The engine respects policy gates (for example, "professional review required before a determination is adopted"). The engine does not set policy; it complies with it.

---

# 35. ENGINE INPUTS AND OUTPUTS SUMMARY

* **Reads:** Destination, Business Reality, Scenario, Financial Modeling, Valuation, Confidence, Evidence Ledger, Decision Record, Research (as versioned snapshots).
* **Writes:** `GoalDetermination` (draft), `DeterminationAssessment`, open-conflicts list, modifications queue.
* **Hands to:** Professional Review (attribution), Document Readiness (document needs), the journey (use).

---

# 36. WHAT THIS ENGINE DOES NOT DO

* Give legal, tax, valuation, investment, or financing advice — the platform is a navigator, not an advisor; those conclusions come from qualified professionals via Professional Review.
* Become platform truth — an adopted determination is attributed to a professional and informs the journey; it does not overwrite objectives or scenarios.
* Relax a nonnegotiable — conflicts are surfaced for the owner to decide.
* Own the framework it analyzes — Destination, Business Reality, Scenario, and the rest remain owned by their engines.
* Perform valuation, financial modeling, or research — it consumes their outputs.
* Decide the transaction — it informs Transaction / Orchestration; it does not execute.
* Replace Professional Review — it produces the draft; Professional Review records and attributes it.

---

# 37. ENGINE NORTH STAR

A professional can open any owner's goal set and, in minutes, see each goal classified as feasible, feasible-with-modifications, or infeasible — every one with a rationale drawn from the framework the owner created and, where needed, a concrete modification path. The conversation moves from "is this possible?" to "what would make it possible?"

---

# 38. ARCHITECTURAL BOUNDARY SUMMARY

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Professional Determination** | The analytical comparison of each goal against the user-created framework, the feasible / feasible-with-modifications / infeasible category, the rationale, and the proposed modification path | Professional advice itself, or converting a determination into platform truth |
| **Professional Review** | Recording and attributing the adopted determination | Producing the analytical draft |
| **Destination** | The owner's goals and objectives being tested | The determination of whether they are achievable |
| **Business Reality** | The current-state facts goals are tested against | The determination |
| **Scenario** | Potential paths that establish possibility | The feasibility call |
| **Financial Modeling** | Projections the engine tests against | The determination |
| **Valuation** | Value range the engine bounds against | The determination |
| **Confidence** | The goal-alignment signal | The determination category |
| **Evidence Ledger** | Claims and evidence the rationale cites | The determination |
| **Decision Record** | Prior decisions the engine respects | Overriding a recorded decision |
| **Research** | External evidence the engine uses | The determination |
| **Document Readiness** | Document needs implied by a determination | The determination |
| **Marketplace** | Professional discovery and role fit | Selecting the professional for the owner |
| **Transaction / Orchestration** | The execution plan informed by determinations | The determination |
| **Communication** | The conversation about determinations | The determination itself |
| **Consent & Access** | Who may see a determination, for what purpose | The determination content |
| **Local Vault** | Private source documents behind the framework | Determination semantics |
| **Audit / Provenance** | The history of drafts and adoptions | The determinations themselves |
| **Policy / Compliance** | The rules governing when review is required | Professional judgment |

## Hard Boundary

**The engine analyzes and classifies; it does not advise and it does not conclude on the platform's behalf.** A draft determination becomes a professional's conclusion only when a qualified professional adopts it and is attributed as its author. It never becomes platform truth, and it never relaxes a nonnegotiable.

---

# 39. ONE-SENTENCE DEFINITION

> **The Professional Determination Engine is the analytical workbench professionals use to test each owner-stated goal against the framework the owner has created — Destination, Business Reality, Scenario, Financial Modeling, Valuation, and Confidence — and produce an attributed determination of feasible, feasible-with-modifications, or infeasible, with a rationale and a modification path for every goal.**
