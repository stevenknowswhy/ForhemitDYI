# ForhemitDYI

Design documentation for an **ownership-transition platform** that helps small and lower-middle-market business owners explore selling their company to their employees.

The documents in this repository are the working design record. They were written as a running design conversation and are kept verbatim; several are explicitly marked as *locked* product principles that later documents build on.

---

## The Core Thesis

> A guided, local-first platform that helps business owners define their ownership-transition goals, explore potential paths, assemble the right professional team, and produce actionable Professional Review Packages for qualified advisors to validate and execute.

The owner brings the goal. The platform organizes the goal. Professionals determine the path. The transaction team executes it.

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

**→ [Document Index](DOCUMENT-INDEX.md)** — the complete annotated roster: all 57 design documents, each with a description of what it covers.

They are organized into twelve groups, each a layer of the platform or a phase of the design work:

| Group | Docs | What it covers |
| --- | --- | --- |
| Foundations & Thesis | 3 | The platform thesis, the original brief, and the locked destination-first core. |
| Core Product Principles | 4 | The UX and decision-layer principles that constrain every screen. |
| Journey Design | 5 | How a journey is defined, built, and walked — through the Employee Ownership journey. |
| Professional Network & Marketplace | 4 | Curated discovery, role fit, and the vendor governance behind it. |
| Evidence, Confidence & Documents | 5 | Turning reality and documents into verifiable fact, confidence, and readiness. |
| Exploration & Modeling | 4 | The decision-layer engines that build representations rather than act — what it might be worth, what the numbers look like, and which paths exist. |
| Architecture Principles & Platform Overview | 5 | The architectural rules, the full engine roster, and the build roadmap. |
| Transaction & Execution Engines | 12 | The engines that move a transaction from intent to closed — and beyond. |
| Platform Infrastructure & Governance | 9 | The private, secure, auditable foundation every engine depends on. |
| Publishing & External Surfaces | 2 | Public content, and the external sites that carry it. |
| Strategic Expansion | 1 | Adjacent business lines beyond the core platform. |
| Superseded | 3 | Documents replaced by a newer one, kept for provenance. |
| **Total** | **57** | |

---

## Where to Start

Four reading paths, depending on what you need.

**New to the project** — [The Emerging Thesis](The%20Emerging%20Thesis.md) for the one-sentence framing, then the [Brainstorming Brief](Brainstorming%20Brief%20-%20Build%20a%20Platform%20That%20Helps%20Business%20Owners%20Sell%20Their%20Business%20to%20Their%20Employees.md) for the original 36-section exploration.

**The architecture** — [Core Architecture Principle](Core%20Architecture%20Principle%20-%20No%20Engine%20Owns%20the%20Entire%20Transaction.md) → [Standalone Engine Architecture](Standalone%20Engine%20Architecture%20-%20Ownership%20Transition%20Engine%20Platform.md) → [Complete Architecture](Complete%20Architecture%20-%20Decision%20Engines,%20Transaction%20Engines,%20Platform%20Infrastructure.md) for the full engine roster.

**The product experience** — [Three Decision Layers](THREE%20DECISION%20LAYERS.md) → [Make Complexity Feel Simple](THE%20MOST%20IMPORTANT%20PRODUCT%20PRINCIPLE%20-%20MAKE%20COMPLEXITY%20FEEL%20SIMPLE.md) → [Goal-Driven Transaction Journey](GOAL-DRIVEN%20TRANSACTION%20JOURNEY.md).

**A specific engine** — go straight to the [Document Index](DOCUMENT-INDEX.md).

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

**Transaction engines** — Capital · Underwriting · Seller Note Liquidity · Professional Review · Professional Determination · Document Readiness · Review Package · Transaction · Stakeholder · Stakeholder Disclosure · Workflow · Communication · Closing · Ownership Lifecycle

**Platform infrastructure** — Local Vault · Identity & Access · Consent & Access · Policy · Audit · Decision Record · Notification · Integration · Vendor Administration · Billing

**Cross-cutting, not an engine** — Security. It is a property every engine must have rather than a component with its own boundary, so it has no document of its own; the responsibility is distributed across Identity & Access, Policy / Compliance, Consent & Access, Local Vault, Integration, and Audit / Provenance. See [Complete Architecture](Complete%20Architecture%20-%20Decision%20Engines,%20Transaction%20Engines,%20Platform%20Infrastructure.md) §20 for where each part lives.

---

## Status

Early-stage design. No application code yet — these documents are the conceptual architecture that precedes it.

The first published journey will be **Employee Ownership**, with the first MVP release scoped to stop at *Professional Review* rather than attempting to automate closing.

The documents deliberately contain open questions requiring professional legal, tax, securities, lending, and fiduciary review. Nothing here constitutes legal, tax, investment, valuation, or financing advice.

## License

This project is proprietary — Copyright (c) 2026 Forhemit, all rights reserved. Use, modification, and redistribution are prohibited without written permission from Forhemit; see [LICENSE](LICENSE).
