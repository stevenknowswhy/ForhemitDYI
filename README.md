# ForhemitDYI

Design documentation for an **ownership-transition platform** that helps small and lower-middle-market business owners explore selling their company to their employees.

The documents in this repository are the working design record. They were written as a running design conversation and are kept verbatim; several are explicitly marked as *locked* product principles that later documents build on.

---

## The Core Thesis

> A guided, local-first platform that helps business owners define their ownership-transition goals, explore potential paths, assemble the right professional team, and produce actionable Professional Review Packages for qualified advisors to validate and execute.

The owner brings the goal. The platform organizes the goal. Professionals determine the path. The transaction team executes it.

---

## Current Product Direction

The first build is intentionally narrower than the full architecture described in this repository.

| Dimension | First vertical slice |
| --- | --- |
| User | Controlling owner of a privately held operating business |
| Jurisdiction | California, plus applicable US federal requirements |
| Transition horizon | 1–3 years |
| Path explored | ESOP exploration |
| Professional model | Bring your own professional first |
| End point | Professional feedback and owner acknowledgment |
| Excluded | ESOP implementation, transaction execution, financing execution, and closing |

The slice has five stages:

1. **Destination** — capture the owner's desired outcome, priorities, nonnegotiables, and preferences without rewriting owner intent.
2. **Business Snapshot** — collect structured facts, provenance, freshness, conflicts, and missing evidence.
3. **ESOP Scenario Exploration** — build a clearly labeled scenario under explicit assumptions; do not present feasibility, valuation, or advice as established fact.
4. **Professional Review Package** — assemble a versioned, purpose-limited package and disclose exactly what will be shared, with whom, and why.
5. **Professional Review** — record qualified-professional feedback separately from platform output, then obtain owner acknowledgment.

This is the first buildable slice—not a claim that the broader engine architecture is already implemented.

### How Jev fits

[Jev](Jev%20Decision%20Layer%20Architecture.md) is a typed, probability-scored decision primitive used **inside** the module that owns a workflow step. It is not a new engine, an advisor, or a source of domain truth.

The initial Jev opportunities are deliberately assistive:

| Workflow point | Jev may help with | Jev may not decide |
| --- | --- | --- |
| Destination | Completeness, contradiction, and next-clarification signals | Owner intent or whether a nonnegotiable may be relaxed |
| Journey | A reversible next-step signal among caller-permitted steps | Access, authorization, or advancement past deterministic gates |
| Scenario | Readiness, missing-context, and escalation signals | Feasibility, valuation, tax, legal, financing, or fiduciary conclusions |
| Review package | Unsupported-claim, attribution, conflict, and advice-like-language QA | Disclosure authorization or whether sharing is permitted |
| Professional review | Organizing already-recorded feedback | A professional determination or the owner's decision |

Every Jev call must use a registered, versioned question; purpose-specific external-AI consent; minimized inputs; an immutable disclosure record; an auditable evaluation record; and an explicit human/manual fallback. Deterministic rules always take precedence.

### Release sequence

1. **Release 0 — deterministic vertical slice, no Jev dependency.** Implement persistence, domain invariants, deterministic gates, consent, disclosure-before-transmission, minimization, invalidation, audit/outbox behavior, and the complete manual fallback path.
2. **Release 1 — Jev shadow mode.** Run selected registered questions without changing workflow state; compare results with labeled human outcomes and measure calibration, abstention, fallback, false advancement, and hard-boundary violations.
3. **Release 2 — bounded assistance.** Enable only calibrated, low-risk, reversible behavior. Keep professional conclusions, owner decisions, access, policy, consent, exact calculations, and protected actions outside Jev authority.

The system must remain usable when Jev is unavailable, unauthorized, disabled, or replaced.

---

## The Load-Bearing Principles

These recur across nearly every document and should be treated as constraints on any implementation:

1. **Destination first.** Every journey begins by capturing the owner's *Desired Outcome* — not a transaction structure. The destination is the persistent North Star.
2. **Three decision layers, never blended.** Owner Objective (what the owner wants) / Platform Scenario (what might work under stated assumptions) / Professional Determination (what a qualified professional concludes).
3. **Make complexity feel simple.** Question → 2–3 choices → consequence → next decision. Progressive disclosure; never force false certainty.
4. **Nonnegotiables are hard constraints.** A must-have is never silently relaxed, ignored, or overridden — conflicts are surfaced and the owner decides.
5. **The platform is a navigator, not an advisor.** It prepares, organizes, models, and translates. It does not give legal, tax, valuation, investment, or financing advice.
6. **Local-first by default.** Sensitive business information stays on the owner's machine; sharing is explicit, authorized, and revocable.
7. **No pay-to-play.** Professionals do not pay for inclusion, placement, ranking, or favorable treatment.
8. **No engine owns the entire transaction.** Specialized engines cooperate through structured contracts, each independently replaceable without rewriting the platform.

---

## The Documents

**→ [Document Index](DOCUMENT-INDEX.md)** — the complete annotated roster: all 61 design documents, each with a description of what it covers.

They are organized into thirteen groups, each a layer of the platform, a phase of the design work, or a cross-cutting implementation concern:

| Group | Docs | What it covers |
| --- | --- | --- |
| Foundations & Thesis | 3 | The platform thesis, the original brief, and the locked destination-first core. |
| Core Product Principles | 4 | The UX and decision-layer principles that constrain every screen. |
| Journey Design | 5 | How a journey is defined, built, and walked — through the Employee Ownership journey. |
| Professional Network & Marketplace | 4 | Curated discovery, role fit, and the vendor governance behind it. |
| Evidence, Confidence & Documents | 5 | Turning reality and documents into verifiable fact, confidence, and readiness. |
| Exploration & Modeling | 4 | The decision-layer engines that build representations rather than act — what it might be worth, what the numbers look like, and which paths exist. |
| Architecture Principles & Platform Overview | 5 | The architectural rules, the full engine roster, and the build roadmap. |
| Transaction & Execution Engines | 11 | The engines that move a transaction from intent to closed — and beyond. |
| Platform Infrastructure & Governance | 9 | The private, secure, auditable foundation every engine depends on. |
| Publishing & External Surfaces | 2 | Public content, and the external sites that carry it. |
| Strategic Expansion | 1 | Adjacent business lines beyond the core platform. |
| Jev Decision Layer | 5 | Safe typed-decision architecture, the California ESOP vertical slice and implementation shape, the migration checklist, and the adversarial second-pass review. |
| Superseded | 3 | Documents replaced by a newer one, kept for provenance. |
| **Total** | **61** | |

---

## Where to Start

Four reading paths, depending on what you need.

**New to the project** — [The Emerging Thesis](The%20Emerging%20Thesis.md) for the one-sentence framing, then the [Brainstorming Brief](Brainstorming%20Brief%20-%20Build%20a%20Platform%20That%20Helps%20Business%20Owners%20Sell%20Their%20Business%20to%20Their%20Employees.md) for the original 36-section exploration.

**The architecture** — [Core Architecture Principle](Core%20Architecture%20Principle%20-%20No%20Engine%20Owns%20the%20Entire%20Transaction.md) → [Standalone Engine Architecture](Standalone%20Engine%20Architecture%20-%20Ownership%20Transition%20Engine%20Platform.md) → [Complete Architecture](Complete%20Architecture%20-%20Decision%20Engines,%20Transaction%20Engines,%20Platform%20Infrastructure.md) for the full engine roster.

**The product experience** — [Three Decision Layers](THREE%20DECISION%20LAYERS.md) → [Make Complexity Feel Simple](THE%20MOST%20IMPORTANT%20PRODUCT%20PRINCIPLE%20-%20MAKE%20COMPLEXITY%20FEEL%20SIMPLE.md) → [Goal-Driven Transaction Journey](GOAL-DRIVEN%20TRANSACTION%20JOURNEY.md).

**A specific engine** — go straight to the [Document Index](DOCUMENT-INDEX.md).

**Building with Jev** — start with [Jev Decision Layer Architecture](Jev%20Decision%20Layer%20Architecture.md), implement the [California ESOP Exploration Vertical Slice](California%20ESOP%20Exploration%20Vertical%20Slice.md) using its [Implementation Architecture](California%20ESOP%20Vertical%20Slice%20Implementation%20Architecture.md), then use the [Jev Repository Migration Checklist](Jev%20Repository%20Migration%20Checklist.md) and executable contracts in [`jev/`](jev/README.md) and [`contracts/`](contracts/README.md).

**Running Release 0** — see the [deterministic workflow core](app/README.md) and run `python -m unittest discover -s tests -p 'test_*.py' -v`.

---

## Architecture at a Glance

```
DESIRED OUTCOME / DESTINATION
        ↓
OWNER OBJECTIVES
        ↓
PRIORITIES
        ↓
NONNEGOTIABLES + PREFERENCES
        ↓
BUSINESS FACTS
        ↓
PLATFORM SCENARIOS
        ↓
WHAT-IF EXPLORATION
        ↓
PROFESSIONAL REVIEW
        ↓
PROFESSIONAL DETERMINATION
        ↓
REFINED DESTINATION
        ↓
TRANSACTION EXECUTION
```

Independently operating engines, communicating through versioned contracts, organized into three layers:

**Decision engines** — Journey · Destination · Business Reality · Document Intelligence / Extraction · Fact Verification & Conflict · Research · Evidence Ledger · Confidence · Scenario · Financial Modeling · Valuation · Marketplace

**Transaction engines** — Capital · Underwriting · Seller Note Liquidity · Professional Review · Professional Determination · Document Readiness · Review Package · Transaction · Stakeholder · Workflow · Communication · Closing · Ownership Lifecycle

**Platform infrastructure** — Local Vault · Identity & Access · Consent & Access · Policy · Audit · Decision Record · Notification · Integration · Vendor Administration · Billing

**Cross-cutting, not an engine** — Security. It is a property every engine must have rather than a component with its own boundary, so it has no document of its own; the responsibility is distributed across Identity & Access, Policy / Compliance, Consent & Access, Local Vault, Integration, and Audit / Provenance. See [Complete Architecture](Complete%20Architecture%20-%20Decision%20Engines,%20Transaction%20Engines,%20Platform%20Infrastructure.md) §20 for where each part lives.

**Cross-cutting typed decision layer, not an engine** — Jev. Existing engines may use Jev for bounded classification, rubric scoring, prioritization, and reversible routing. Jev never owns domain truth, authorization, exact calculations, owner intent, or professional determinations. See [Jev Decision Layer Architecture](Jev%20Decision%20Layer%20Architecture.md).

---

## Status

**Release 0 implementation foundation.** The repository now contains a runnable, tested Python/SQLite workflow core in [`app/`](app/README.md), in addition to the design specifications and executable contracts.

Implemented in the current foundation:

- immutable versions for destination, business snapshot, scenario/readiness, professional consent, review package, disclosure, and professional response;
- deterministic owner and assigned-professional authority checks with no Jev runtime dependency;
- separate private-workspace and professional-collaboration SQLite boundaries;
- disclosure-before-delivery, current-consent enforcement, revocation-safe delivery, and source-version invalidation;
- transactional audit/outbox writes, inbox deduplication, command idempotency, and hash-linked audit events;
- exact professional attribution and owner acknowledgment without inferred agreement or automatic transaction handoff;
- runtime validation of persisted domain objects against the canonical JSON Schema definitions;
- a responsive, loopback-only owner UI over the development API, with the complete five-stage manual path;
- a production [identity and session design](app/IDENTITY-AND-SESSIONS.md) that keeps the current actor header explicitly development-only;
- 25 workflow and HTTP tests plus a real-browser five-stage smoke test, alongside the existing adversarial contract and repository validation suites.

This is **not yet a production-ready product**. The local UI uses synthetic/development identities only. Production authentication, encryption and key management, the document vault, background workers, retention/deletion, backup/recovery, production observability, threat modeling, incident response, and independent security testing remain outstanding.

The deterministic core is exposed through a loopback-only [development API and owner UI](app/API.md), with the future production boundary specified in the [identity and session design](app/IDENTITY-AND-SESSIONS.md). The next milestone is replacing development identity scaffolding with a tested server-side session implementation while retaining loopback-only operation. Jev follows in shadow mode only after the complete manual path and operational controls are ready for evaluation.

The broader documents remain the long-range architecture. They should not be read as implemented features or as a commitment to build every engine before validating the first slice.

The documents deliberately contain open questions requiring professional legal, tax, securities, lending, privacy, security, and fiduciary review. Nothing here constitutes legal, tax, investment, valuation, or financing advice.
