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

## Document Index

### 1. Foundations & Thesis

| Document | What it covers |
| --- | --- |
| [The Emerging Thesis](The%20Emerging%20Thesis.md) | The one-sentence description of the platform, the "professional review packages" principle, and the owner-objective / platform-scenario / professional-determination separation. |
| [Brainstorming Brief](Brainstorming%20Brief%20-%20Build%20a%20Platform%20That%20Helps%20Business%20Owners%20Sell%20Their%20Business%20to%20Their%20Employees.md) | The original 36-section brief: mission, transaction concepts, readiness, valuation, financing, marketplaces, red-teaming, regulatory analysis, business model, and the deliverable spec. |
| [Destination First — Locked Core Architecture](Destination%20First%20-%20Locked%20Core%20Architecture.md) | Locks the Destination as the first object created by every journey and the North Star for the whole application. |

### 2. Core Product Principles

| Document | What it covers |
| --- | --- |
| [Make Complexity Feel Simple](THE%20MOST%20IMPORTANT%20PRODUCT%20PRINCIPLE%20-%20MAKE%20COMPLEXITY%20FEEL%20SIMPLE.md) | The 20 UX principles: two-or-three-choice rule, progressive revelation, reversible decisions, journey map, expert layers, the core UX test. |
| [Three Decision Layers](THREE%20DECISION%20LAYERS.md) | Formalizes Owner Objective / Platform Scenario / Professional Determination, the 5-part review package, plan status labels, and the core data model. |
| [Nonnegotiable / Must-Have Constraint](NONNEGOTIABLE%20-%20MUST-HAVE%20CONSTRAINT.md) | The nonnegotiable toggle, hard constraints, scenario filtering, conflict surfacing, and the three preference states. |
| [Interaction Toolbox and Hierarchy](Interaction%20Toolbox%20and%20Hierarchy.md) | The interaction vocabulary (single/multi-select, ranking, trade-off spectrum, percentage allocation, "not sure / help me decide") and the 7-level hierarchy. |

### 3. Journey Design

| Document | What it covers |
| --- | --- |
| [Goal-Driven Transaction Journey](GOAL-DRIVEN%20TRANSACTION%20JOURNEY.md) | Why the journey starts from the owner's goals, the draft plan, the "My Ownership Transition Brief" review package, and the core product loop. |
| [Journey-Builder Rules and Journey Definition Engine](Journey-Builder%20Rules%20and%20Journey%20Definition%20Engine.md) | Locks the journey-builder rules and introduces the Journey Definition Engine so new journeys don't require rebuilding the app. |
| [Journey Builder Architecture & Employee Ownership Journey](Journey%20Builder%20Architecture%20&%20Employee%20Ownership%20Journey.md) | Full Journey Builder / Journey Engine architecture, node types, question metadata, storage scopes, and the STAGE 0–19 Employee Ownership journey. |
| [Employee Ownership Journey v0.2 — Optimized Guided Journey](Employee%20Ownership%20Journey%20v0.2%20-%20Optimized%20Guided%20Journey.md) | The optimized journey: negative constraints ("what to avoid"), "ask only when relevant" rules, information value, question exit, and the one-page executive summary. |
| [Employee Ownership Journey — Start With the Desired End State](EMPLOYEE%20OWNERSHIP%20JOURNEY%20-%20Start%20With%20the%20Desired%20End%20State.md) | The destination-first journey: "Picture Your Success", the Destination Card, the owner's success-story narrative, and the `DesiredOutcome` object. |
| [Destination Builder v1.0](Employee%20Ownership%20Journey%20-%20Destination%20Builder%20v1.0.md) | The full 13-screen Destination Builder spec, the `DesiredOutcome` object tree, destination versioning, and the completeness indicator. |

### 4. Professional Network & Marketplace

| Document | What it covers |
| --- | --- |
| [Professional Choice — Bring Your Own or Use Our Curated Network](PROFESSIONAL%20CHOICE%20-%20BRING%20YOUR%20OWN%20OR%20USE%20OUR%20CURATED%20NETWORK.md) | The BYO-vs-curated pattern for every professional category, capital providers, the seller-note buyer directory, and the match-vs-offer distinction. |
| [Professional Role-Fit and Team Optimization](PROFESSIONAL%20ROLE-FIT%20AND%20TEAM%20OPTIMIZATION.md) | The Role-Fit Check for the owner's existing professionals, the "right person, right job" framework, the responsibility matrix, and gap/overlap detection. |
| [Professional Marketplace Engine v1.0](Professional%20Marketplace%20Engine%20v1.0.md) | Curated-not-open marketplace: no pay-to-play, organization-first model, vetting and re-verification, review moderation, contextual matching, and visibility controls. |

### 5. Evidence, Confidence & Documents

| Document | What it covers |
| --- | --- |
| [Business Reality Engine v1.0 — Current State / Business Assessment](Business%20Reality%20Engine%20v1.0%20-%20Current%20State%20Business%20Assessment.md) | The factual counterpart to the Destination Engine: the `BusinessCurrentState` object, field-level provenance, the three information levels (stated / supported / verified), financial trends, owner-dependency and concentration flags, versioning, and the two-snapshot view ("Where You Want to Go" vs. "Where You Are Today"). |
| [Goal-to-Reality Confidence Research Engine](Goal-to-Reality%20Confidence%20Research%20Engine.md) | The research agent, privacy-first research inputs, the evidence ledger, multi-dimensional confidence, non-monotonic confidence, and no false precision. |
| [Stakeholder Document & Visibility Architecture](Stakeholder%20Document%20&%20Visibility%20Architecture.md) | "One source, many views": the three visibility levels as data-permission policies, per-stakeholder packages, and the document visibility matrix. |
| [Document Readiness & Checklist Engine v1.0](Document%20Readiness%20&%20Checklist%20Engine%20v1.0.md) | The dynamic checklist: document categories and statuses, local-first availability, professional-specific requirements, duplication control, and package readiness. |

### 6. Architecture & Privacy

| Document | What it covers |
| --- | --- |
| [Core Architecture Principle — No Engine Owns the Entire Transaction](Core%20Architecture%20Principle%20-%20No%20Engine%20Owns%20the%20Entire%20Transaction.md) | The locked rule that no engine owns the whole transaction: one clearly defined responsibility each, structured contracts between engines, and replaceability without rewriting the platform. |
| [Standalone Engine Architecture — Ownership Transition Engine Platform](Standalone%20Engine%20Architecture%20-%20Ownership%20Transition%20Engine%20Platform.md) | The hard architectural principle: independent engines cooperating through versioned data contracts, "no engine may impersonate another," and explainable outputs. |
| [Complete Architecture — Decision Engines, Transaction Engines, Platform Infrastructure](Complete%20Architecture%20-%20Decision%20Engines,%20Transaction%20Engines,%20Platform%20Infrastructure.md) | The full end-to-end roster: the 10 missing engines (Business Reality, Financial Modeling, Capital, Seller Note Liquidity, Professional Review, Transaction, Stakeholder, Consent & Access, Local Vault, Workflow, Communication, Notification, Policy, Audit, Integration, Ownership Lifecycle, Vendor Administration, Identity & Access, Billing), the Decision Record Engine, the 14-stage lifecycle, and the MVP boundary. |
| [Hybrid Local-First Architecture](HYBRID%20LOCAL-FIRST%20ARCHITECTURE.md) | The local/online split: encrypted local document vault, explicit document sharing, three portals, the transaction knowledge engine, and offline capability. |

### 7. Strategic Expansion

| Document | What it covers |
| --- | --- |
| [Seller-Note Liquidity Marketplace](Critical%20Strategic%20Expansion%20-%20Seller-Note%20Liquidity%20Marketplace.md) | The seller-note marketplace as a potentially distinct business line: buyers, pricing, marketplace mechanics, conflicts of interest, and regulatory questions. |

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

**Decision engines** — Journey · Destination · Business Reality · Research · Evidence Ledger · Confidence · Scenario · Financial Modeling

**Transaction engines** — Capital · Seller Note Liquidity · Professional Review · Document Readiness · Review Package · Transaction · Stakeholder · Workflow · Communication · Ownership Lifecycle

**Platform infrastructure** — Local Vault · Identity & Access · Consent & Access · Policy · Audit · Decision Record · Notification · Integration · Vendor Administration · Billing · Security

---

## Status

Early-stage design. No application code yet — these documents are the conceptual architecture that precedes it.

The first published journey will be **Employee Ownership**, with the first MVP release scoped to stop at *Professional Review* rather than attempting to automate closing.

The documents deliberately contain open questions requiring professional legal, tax, securities, lending, and fiduciary review. Nothing here constitutes legal, tax, investment, valuation, or financing advice.
