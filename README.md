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
| [Destination Engine v1.0](Destination%20Engine%20v1.0.md) | The standalone engine that owns the owner's intent: destination vs. objective / priority / nonnegotiable / preference, the full `DesiredOutcome` data model, statuses, the financial / ownership / personal / timing / preservation / avoidance components, priority and constraint maps, narrative generation, versioning and change events, error prevention, the reusable Journey-Builder mapping, the "My Destination" review card, and the 10 final architectural rules. |

### 4. Professional Network & Marketplace

| Document | What it covers |
| --- | --- |
| [Professional Choice — Bring Your Own or Use Our Curated Network](PROFESSIONAL%20CHOICE%20-%20BRING%20YOUR%20OWN%20OR%20USE%20OUR%20CURATED%20NETWORK.md) | The BYO-vs-curated pattern for every professional category, capital providers, the seller-note buyer directory, and the match-vs-offer distinction. |
| [Professional Role-Fit and Team Optimization](PROFESSIONAL%20ROLE-FIT%20AND%20TEAM%20OPTIMIZATION.md) | The Role-Fit Check for the owner's existing professionals, the "right person, right job" framework, the responsibility matrix, and gap/overlap detection. |
| [Professional Marketplace Engine v1.0](Professional%20Marketplace%20Engine%20v1.0.md) | Curated-not-open marketplace: no pay-to-play, organization-first model, vetting and re-verification, review moderation, contextual matching, and visibility controls. |
| [Marketplace Engine v1.0](Marketplace%20Engine%20v1.0.md) | The curated discovery-and-matching system, separated from vendor governance: the seven marketplace principles (curated not open, no pay-to-play, user chooses, relevance over volume, organizations ≠ people, structured profiles, ratings inform), the `Organization` / `Professional` / Specialty / Service objects, the hierarchical specialty taxonomy, the structured experience and credential models, `MarketplaceListing`, the curation layer (curation is not ranking), contextual matching with `MarketplaceMatch` and `MatchExplanation`, the standalone visibility engine and rule-driven eligibility, ratings / comments / verified platform experience and moderation, vendor-vs-platform authority, comparison without a winner, the selection handoff to the Stakeholder Engine, analytics and demand-gap analysis, profile completeness and freshness, `CurationRecord`, marketplace versioning, and the explicit "what the Marketplace Engine does not do" list. |
| [Vendor Administration / Vetting Engine v1.0](Vendor%20Administration%20-%20Vetting%20Engine%20v1.0.md) | The back-office governance counterpart to the Marketplace Engine: the twelve primary objects (`VendorApplication`, `OrganizationReview`, `CredentialRecord`, `ExperienceClaim`, `VettingCase`, `ApprovalRecord`, `Complaint`, `ReviewModerationCase`, `VisibilityAction`…), the application lifecycle, credential and experience verification (provider claim vs. platform verification), the vetting checklist and decision model, approval-with-restrictions, public claim control and the verification badge taxonomy, publication and fine-grained visibility controls, suspension and appeal workflows, re-verification and freshness states, the admin "what changed?" view, complaints and review moderation, the conflict-of-interest record and the enforceable no-paid-placement rule, the governance events consumed by the Marketplace Engine, administrator override and auditability, and configurable per-category vetting rules. |

### 5. Evidence, Confidence & Documents

| Document | What it covers |
| --- | --- |
| [Business Reality Engine v1.0 — Current State / Business Assessment](Business%20Reality%20Engine%20v1.0%20-%20Current%20State%20Business%20Assessment.md) | The factual counterpart to the Destination Engine: the `BusinessCurrentState` object, field-level provenance, the three information levels (stated / supported / verified), financial trends, owner-dependency and concentration flags, versioning, and the two-snapshot view ("Where You Want to Go" vs. "Where You Are Today"). |
| [Expanded Reality Architecture — Document Intelligence and Fact Verification](Expanded%20Reality%20Architecture%20-%20Document%20Intelligence%20and%20Fact%20Verification.md) | The expanded reality pipeline, split across three cooperating engines: **Document Intelligence / Extraction** (reads PDFs, spreadsheets, documents, scans and extracts candidate facts), **Business Reality** (builds the current-state model), and **Fact Verification & Conflict** (compares sources, manages conflicts, records owner verification). Adds full fact provenance, verification states, definition/period preservation, first-class Fact Conflict objects, the fact graph, and the rule that *owner verified ≠ professionally verified*. |
| [Goal-to-Reality Confidence Research Engine](Goal-to-Reality%20Confidence%20Research%20Engine.md) | The research agent, privacy-first research inputs, the evidence ledger, multi-dimensional confidence, non-monotonic confidence, and no false precision. |
| [Stakeholder Document & Visibility Architecture](Stakeholder%20Document%20&%20Visibility%20Architecture.md) | "One source, many views": the three visibility levels as data-permission policies, per-stakeholder packages, and the document visibility matrix. |
| [Document Readiness & Checklist Engine v1.0](Document%20Readiness%20&%20Checklist%20Engine%20v1.0.md) | The dynamic checklist: document categories and statuses, local-first availability, professional-specific requirements, duplication control, and package readiness. |

### 6. Architecture & Privacy

| Document | What it covers |
| --- | --- |
| [Core Architecture Principle — No Engine Owns the Entire Transaction](Core%20Architecture%20Principle%20-%20No%20Engine%20Owns%20the%20Entire%20Transaction.md) | The locked rule that no engine owns the whole transaction: one clearly defined responsibility each, structured contracts between engines, and replaceability without rewriting the platform. |
| [Standalone Engine Architecture — Ownership Transition Engine Platform](Standalone%20Engine%20Architecture%20-%20Ownership%20Transition%20Engine%20Platform.md) | The hard architectural principle: independent engines cooperating through versioned data contracts, "no engine may impersonate another," and explainable outputs. |
| [Complete Architecture — Decision Engines, Transaction Engines, Platform Infrastructure](Complete%20Architecture%20-%20Decision%20Engines,%20Transaction%20Engines,%20Platform%20Infrastructure.md) | The full end-to-end roster: the 10 missing engines (Business Reality, Financial Modeling, Capital, Seller Note Liquidity, Professional Review, Transaction, Stakeholder, Consent & Access, Local Vault, Workflow, Communication, Notification, Policy, Audit, Integration, Ownership Lifecycle, Vendor Administration, Identity & Access, Billing), the Decision Record Engine, the 14-stage lifecycle, and the MVP boundary. |
| [Capital / Financing Engine v1.0](Capital%20-%20Financing%20Engine%20v1.0.md) | The transaction-layer engine that coordinates capital: the `CapitalPlan` object, the capital stack, sources and uses, source/use reconciliation and the `CapitalGap` object, owner financing preferences, employee capital as an explicit assumption, structure-neutral ESOP financing, capital-provider matching (contextual, never ranked), financing requests and provider packages, request statuses, the `FinancingProposal` (indicative terms), financing readiness, plan versioning and the capital event model, what-if capital stacks, deadline tracking, and the hard rules — no ranking, no automatic financing decisions, no exposing the owner's full record. |
| [Seller-Note Liquidity Engine v1.0](Seller-Note%20Liquidity%20Engine%20v1.0.md) | The engine that turns an outstanding seller-financed receivable into a controlled, owner-directed liquidity exploration: the `SellerNote` object, note ingestion and verification, note data quality, the keep / partial / full paths, the `NoteLiquidityScenario`, modeled (never declared) pricing, buyer profiles and matching with "why you're seeing this buyer," the Summary / Detailed / Comprehensive disclosure policy, `NoteOffer` indicative bids, offer comparison without ranking, the professional review package, the transfer / closing workflow, assignment and servicing separation, the configurable regulatory operating modes (A–E), the no-hidden-transaction-compensation rule, buyer vetting, fraud safeguards, the note lifecycle and marketplace flywheel, and the deliberately narrow seller- and buyer-side MVPs. |
| [Professional Review Engine v1.0](Professional%20Review%20Engine%20v1.0.md) | The engine that turns the platform from a planning tool into a professional collaboration system: the `ProfessionalReview` object and review scope, invitations and their states, `ProfessionalAssignment` and the responsibility matrix, the focused professional workspace, `ProfessionalQuestion`, `ProfessionalInformationRequest` wired to Document Readiness, structured professional feedback, `ProfessionalDetermination` (attributed, and never normalized into platform truth), `ProfessionalRequestedChange` with the professional-request vs. owner-decision separation, multi-professional review and conflicting opinions handled without picking a winner, review status and dual dashboards, review dependencies, the review event model, integrations with the Journey / Destination / Scenario / Research / Confidence / Document Readiness / Marketplace engines, review package versioning and change summaries, owner acknowledgment, professional sign-off, escalation and dispute handling, confidentiality and professional-only vs. owner-only notes, decision provenance, and review analytics. |
| [Consent & Access Engine](Consent%20&%20Access%20Engine.md) | The authorization and consent policy layer — *"possession does not equal permission"*: the Actor / Resource / Purpose / Permission / Consent / Access Grant / Expiration / Revocation / Access Policy model, what it deliberately does **not** own, the eight-level permission hierarchy (No Access → Existence Only → Summary → Detailed → Full View → Download/Export → Edit → Share/Delegate), granular access scope with visible inheritance, the four sensitivity classes, recorded purpose, duration and first-class revocation, version-specific sharing, package creation ≠ sharing, the owner approval flow, local-first behaviour and the no-silent-cloud-escape-hatch rule, data minimization, nonnegotiable conflicts, access requests and fail-safe conflict rules, the core data objects (`AccessRequest`, `ConsentRecord`, `AccessGrant`, `DisclosureRecord`, `AccessPolicy`), relationships to Audit / Professional Review / Document Readiness / Marketplace, the Access Center and the "what have I shared / who can see this / why can they see this" views, no permission inference, facts-vs-source-document access, AI-and-integrations-as-actors, external sharing guardrails, the engine contract (ALLOW / ALLOW_WITH_RESTRICTIONS / REQUIRE_CONSENT / DENY / EXPIRED / REVOKED / POLICY_CONFLICT), and the 18 locked architectural requirements. |
| [Local Vault / Workspace Engine](Local%20Vault%20-%20Workspace%20Engine.md) | The platform's private information boundary — *"the owner's private workspace, not a cloud drive with a local cache"*: the three operating modes (Fully Local / Hybrid / Explicitly Shared), the vault boundary diagram, vault ownership and workspace structure, the layered encryption model (auth → workspace authorization → vault key → document key → encrypted document) with key separation, file ingestion ("imported ≠ uploaded ≠ shared"), Document Intelligence at the edge and the candidate-fact boundary, file provenance, mandatory versioning and immutable source preservation, local and semantic search, offline operation and offline-change sync, first-class sync-conflict objects, the four-way sync classification and selective sync, explicit version-specific sharing (a copy, never the private master), local analysis and the five-level cloud-AI boundary, AI processing consent, workspace snapshots and branching, document collections, retention / trash / recovery, device management, backup vs. sync, health and integrity checks, the local workspace database, cloud metadata minimization, local-to-cloud fact promotion, the engine and sharing contracts, the explicit "what the Vault should never do" list, the 12-item minimum viable Vault, and the 20 locked architectural requirements. |
| [Transaction / Orchestration Engine](Transaction%20-%20Orchestration%20Engine.md) | The execution-spine engine that coordinates work without performing it — *"the engine that makes the platform move from 'we know what we intend to do' to 'we are systematically doing it'"*: the `Transaction` object and execution graph, transaction-type extensibility, template vs. plan, the 10-stage model, milestones, tasks with RACI-style ownership, dependency types and conditional branching, deadlines with confidence states, dynamic critical-path calculation, blockers and their hierarchy, participant status and the responsibility map, requirements vs. tasks, aggregated Closing Readiness, transaction state and transitions, reopening, scenario changes as versioned plan revisions, professional-determination and owner-decision boundaries, event-driven task generation, integrations (Workflow, Notifications, Document Readiness, Capital, Seller-Note, Professional Review, Consent, Audit), the transaction timeline and dashboard, progress vs. readiness, risk signals, role-specific and participant views, escalation and deadline-change effects, cancellation and supersession, the closing event and post-close handoff to Ownership Lifecycle, the core data objects, plan versioning, the decision-record relationship, transaction integrity rules, the engine contract, and the 26 locked architectural requirements. |
| [Workflow Engine](Workflow%20Engine.md) | The domain-neutral execution infrastructure beneath everything else — *"the plumbing with a brain"*, deliberately more generic than Transaction / Orchestration: **Transaction / Orchestration owns transaction meaning; Workflow owns execution mechanics.** Covers the event-driven model and the event/trigger/condition/action chain, the event catalogue and structure (event ID, version, source engine, actor, workspace, entity, correlation/causation IDs, sensitivity classification, schema version), event immutability and versioned event contracts, the Event vs. Command vs. Task distinction, the eight trigger types (event / schedule / deadline / state / threshold / condition / human / external), explicit conditions, `WorkflowDefinition` vs. `WorkflowInstance` (reusable blueprint vs. actual execution), task infrastructure and the seven task types (human / professional / system / integration / approval / waiting / review), the twelve task states, generic dependencies and `DependencySatisfied` emission, first-class waiting, recurring actions and recurrence types, escalations with the four-level framework and the "escalation does not mean blame" rule, retries and backoff, idempotency keys and the "at-least-once delivery + idempotent handling" foundation (the exactly-once illusion), correlation and causation, workflow explainability ("why did I get this task?"), human override and the automatic-vs-manual distinction, protected actions requiring authorization, workflow permissions, the workflow templates (Professional Review / Financing / Document Request / Closing), workflow composition and nested workflows, explicit completion criteria and partial completion, failure handling and the dead-letter / failed work queue, scheduling (absolute / relative / business-day / time-zone-aware, due date vs. execution time), time-based event generation, the worked examples (Valuation Completed / Consent Revoked / Scenario Changed / Document Conflict Detected), the Confidence / Notification / Audit / Transaction-Orchestration integrations, generic reusability beyond business sales, the fourteen core data objects, workflow state, the engine contract with its explainability read operations, the data and security boundaries and event-payload minimization, transactional reliability (outbox/inbox), ordering, duplicate and out-of-order events, manual intervention, the ten testing categories, workflow observability and the user-facing explainability surface, automation transparency, template versioning and explicit migration, protected workflow categories, the owner experience, and the 28-item architectural lock. |
| [Communication Engine](Communication%20Engine.md) | The engine that captures **what the people involved are actually saying to each other** — the "human nervous system" of the transaction — while keeping a strict separation between Conversation / Task / Request / Professional determination / Notification / Document. Covers the core principle that *communication records conversation, it does not own the work being discussed*; the owned objects (conversations, messages, threads, questions, requests, comments, participants, read state); the explicit exclusions; the four critical boundaries (Communication vs. Notification, vs. Task, vs. Professional Determination, vs. Decision); the `Conversation` model and its nine types; explicit conversation visibility (transaction-wide / team / private / professional-private / internal platform); participants as references to the Stakeholder / Relationship Engine; the `Message` object and message types; first-class `Question` and `Request` objects with their own state machines; contextual comments that inherit their parent's visibility; threading; conversation linking and the context panel; secure messaging; external participants who see only their assigned matters; the email relationship and external-message capture; message status / editing / deletion (Hide vs. Delete vs. Archive vs. Permanently destroy); attachments that cannot bypass the Local Vault / Consent architecture; sensitive messages; message-level vs. conversation-level access; the no-silent-forwarding rule; participant changes and revocation; the question→task, request→requirement, and question→professional-review bridges; the transaction timeline and search (access-filtered before results); communication history and the "how did we get here?" view; the "why am I seeing this?" and "who can see this?" explainability surfaces; notification events and preferences; conversation priority; escalation; private professional conversations; employee-facing communications; optional templates; meeting integration; message-to-structure conversion; the professional-conversation-to-determination boundary; AI assistance and AI privacy (AI interpretations never blurred with actual messages); conversation summaries and the transaction communication digest; auditability and policy-driven retention; the separate conversation / message / question / request state models; the ten core data objects; the engine contract and emitted events; the eight security rules; failure handling; the cross-architecture relationship diagram; the "does this need to become structured work?" product behavior; and the 24-item architectural lock. |
| [Professional Review Package Engine v1.0](Professional%20Review%20Package%20Engine%20v1.0.md) | The "one source of truth, many purpose-built packages" engine: the `ProfessionalReviewPackage` object, the three disclosure levels (Summary / Detailed / Comprehensive) defined as data-access policies rather than page counts, `StakeholderProfile`, the reusable package templates and per-role content, explicit package purpose, leading with the destination plus the owner's must-haves and avoidances, the scenario and research labelling rules (platform research vs. professional determination), the goal-to-reality status section, the "Questions for You" generator, document referenced vs. attached vs. shared-separately, the owner approval screen with per-category opt-in, the package generation rules and the no-unattributed-AI-claims rule, versioning and change summaries, expiration and revocation, the stakeholder default matrix, the template engine, readiness and automated quality checks with pre-share red flags, narrative style rules and disclaimers, and the Disclosure and **Exclusion Manifests**. |
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

**Decision engines** — Journey · Destination · Business Reality · Document Intelligence / Extraction · Fact Verification & Conflict · Research · Evidence Ledger · Confidence · Scenario · Financial Modeling · Marketplace

**Transaction engines** — Capital · Seller Note Liquidity · Professional Review · Document Readiness · Review Package · Transaction · Stakeholder · Workflow · Communication · Ownership Lifecycle

**Platform infrastructure** — Local Vault · Identity & Access · Consent & Access · Policy · Audit · Decision Record · Notification · Integration · Vendor Administration · Billing · Security

---

## Status

Early-stage design. No application code yet — these documents are the conceptual architecture that precedes it.

The first published journey will be **Employee Ownership**, with the first MVP release scoped to stop at *Professional Review* rather than attempting to automate closing.

The documents deliberately contain open questions requiring professional legal, tax, securities, lending, and fiduciary review. Nothing here constitutes legal, tax, investment, valuation, or financing advice.
