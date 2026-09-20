# California ESOP Exploration Vertical Slice

## 1. Decision Status

This document locks the first buildable Forhemit product slice.

| Decision | Locked scope |
| --- | --- |
| Owner persona | A controlling owner of a privately held California operating business who is considering a transition in 1–3 years |
| Transaction shape | Exploration of an Employee Stock Ownership Plan (ESOP) through qualified Professional Review |
| Jurisdiction | California, together with applicable United States federal requirements |
| Professional model | Bring-your-own qualified professional first |
| End boundary | Owner acknowledgment of professional feedback or determination |
| Implementation posture | Exploratory planning, not ESOP implementation, transaction execution, legal compliance, tax advice, valuation, financing, or closing |

California and federal references define the initial policy and professional-review context. They do not make the platform a legal or tax authority. Applicability, qualification, fiduciary, valuation, tax, securities, labor, and transaction conclusions remain with qualified professionals.

---

## 2. Persona and Entry Conditions

The initial user:

- controls a privately held operating business
- identifies California as the business's primary operating or transaction jurisdiction
- is considering transition in approximately 1–3 years
- wants to explore meaningful employee ownership
- has not yet selected an ESOP as a final transaction decision
- may have incomplete or approximate business information
- will invite an existing qualified professional rather than select from a Forhemit marketplace

The journey must not imply that the business is suitable for an ESOP, that an ESOP is preferable to another structure, or that a professional review is a legal sign-off.

---

## 3. User Outcome

At the end of the slice, the owner has:

1. a versioned desired destination
2. a sourced, uncertainty-preserving business snapshot
3. one attributed exploratory ESOP scenario
4. a purpose-limited package authorized for a named professional
5. structured questions, feedback, requests, or determinations from that professional
6. an explicit owner acknowledgment and a safe next route

The slice answers:

> What do I want, what do we currently know, what would an ESOP path require us to investigate, and what does my qualified professional say we should examine next?

It does not answer:

> Is an ESOP legally, financially, or tax-wise appropriate, feasible, compliant, or recommended?

---

## 4. Explicit Non-Goals

The first slice does not include:

- transaction execution or closing
- ESOP plan design, trust formation, administration, or fiduciary execution
- legal, tax, valuation, investment, lending, securities, or ERISA determinations
- calculation of valuation, tax treatment, financing capacity, or transaction proceeds
- selection or ranking of a winning structure
- comparison intended to recommend an ESOP over another path
- curated-professional marketplace or marketplace monetization
- vendor approval, credential verification, or claims that a professional is qualified
- capital-provider discovery, underwriting, financing requests, or financing execution
- seller-note creation, servicing, sale, or liquidity marketplace
- employee solicitation, voting, enrollment, allocation, or ownership-lifecycle administration
- automated disclosure, consent, access, or policy decisions
- automatic transaction handoff after professional review
- raw-document submission to Jev

Any work beyond the Professional Review boundary requires a separately approved slice.

---

## 5. Flow

```text
Destination
→ Business Snapshot
→ ESOP Scenario Exploration
→ Professional Review Package
→ Professional Review
→ Owner Acknowledgment / Safe Next Route
```

Every transition is governed first by deterministic policy, consent, and completeness gates. Jev may provide a typed signal only within the authority declared for its registered question. It cannot override a failed hard gate.

---

## 6. Stage Contract: Destination

### User outcome

The owner can review and confirm a versioned statement of desired timing, involvement, liquidity preferences, employee outcome, legacy goals, risk tolerance, preferences, and nonnegotiables.

### Minimum persisted objects

- `DesiredOutcome`
- owner objective and priority records
- `Nonnegotiable`
- preference and avoidance records
- destination version and source references
- owner confirmation record
- applicable consent/disclosure acknowledgment

### Owning module

Destination, with Journey coordinating presentation.

### Deterministic gates

Before advancing:

- owner and business references exist
- exploratory/not-advice disclosure is acknowledged
- desired transition timeframe is recorded
- employee-ownership intent is recorded
- desired post-transition involvement is recorded or explicitly unknown
- every hard nonnegotiable is explicitly owner-authored
- unresolved contradictions between nonnegotiables block automatic advancement
- the owner confirms the destination version

### Jev questions

- `destination.completeness`
- `destination.nonnegotiable_conflict`
- `destination.next_clarification`

### Maximum Jev authority

- completeness: `signal_only`
- conflict: `signal_only`
- next clarification: `human_confirmed`

Jev may identify a likely missing dimension or apparent conflict. It may not write, alter, rank, or relax owner intent.

### Human authority

Only the owner confirms objectives, priorities, preferences, avoidances, and nonnegotiables.

### Audit events

- `DestinationCreated`
- `DestinationVersionCreated`
- `DestinationObjectiveChanged`
- `DestinationNonnegotiableChanged`
- `DestinationConflictFlagged`
- `DestinationConfirmed`
- `JevEvaluationRecorded`
- `JevEvaluationInvalidated`

### Invalidation triggers

Invalidate related Jev evaluations when:

- the destination version changes
- any objective, priority, preference, avoidance, or nonnegotiable changes
- consent is revoked or its purpose changes
- the question, threshold policy, or model version changes

### Acceptance tests

- a vague destination remains incomplete without invented goals
- contradictory nonnegotiables prevent automatic advancement
- a Jev conflict signal cannot change a nonnegotiable
- an owner can reject a suggested clarification and continue through the manual route
- every displayed Jev signal identifies its question/version and owning module
- revoking external-AI consent removes Jev from the path without blocking manual completion

### Telemetry

- destination completion rate
- median owner-confirmed revisions
- apparent-conflict signal rate
- Jev abstention and fallback rate
- owner acceptance/rejection of clarification prompts
- time to owner-confirmed destination
- manual-path completion rate

---

## 7. Stage Contract: Business Snapshot

### User outcome

The owner can review what the platform currently knows about the business, where each fact came from, how current it is, and what remains unknown.

### Minimum persisted objects

- `BusinessCurrentState`
- business identity and jurisdiction
- legal-entity type as owner-stated or source-supported
- approximate revenue and operating cash-flow/EBITDA ranges
- employee count
- ownership and management summary
- owner-dependence and management-depth indicators
- debt range or unknown state
- `Fact` records with provenance, information status, sensitivity, and effective date
- current-state version and owner confirmation

### Owning module

Business Reality.

### Deterministic gates

Before scenario creation:

- business and owner references exist
- California applicability is owner-stated and flagged for professional confirmation
- required Level 1 snapshot fields are present or explicitly unknown
- every material fact has a source and information status
- stale, contradictory, or unsupported inputs remain visibly labeled
- exact values are not required where a range supports exploration
- no professional verification is inferred from owner entry or document presence
- the owner confirms the snapshot version used by the scenario

### Jev question

None in the initial slice.

Completeness, provenance presence, freshness, range validation, and contradictions in structured facts are deterministic. Later Jev fact-conflict triage requires a separately registered question and benchmark.

### Maximum Jev authority

`no_role`

### Human/professional authority

The owner confirms owner-stated facts. Qualified professionals verify matters within their scope. Neither confirmation converts all facts into universal truth.

### Audit events

- `BusinessCurrentStateCreated`
- `BusinessCurrentStateVersionCreated`
- `FactRecorded`
- `FactSourceAttached`
- `FactConflictDetected`
- `FactMarkedUnknown`
- `BusinessCurrentStateConfirmed`

### Invalidation triggers

The snapshot and downstream scenario are affected when:

- a material fact or source changes
- a fact becomes stale, conflicted, or professionally verified
- business jurisdiction or entity type changes
- the owner confirms a new current-state version

### Acceptance tests

- unknown values remain first-class and do not become zero
- every material displayed fact exposes provenance and status
- unsupported owner estimates remain labeled as stated
- contradictory facts are preserved rather than silently merged
- the owner can complete the initial snapshot using ranges
- no Jev request is made from this stage

### Telemetry

- Level 1 snapshot completion rate
- unknown rate by field
- provenance coverage
- stale/conflicted fact rate
- time to owner-confirmed snapshot
- downstream invalidation count

---

## 8. Stage Contract: ESOP Scenario Exploration

### User outcome

The owner can inspect one exploratory ESOP path, understand why it is being explored, see its assumptions and unknowns, and identify questions requiring qualified review.

### Minimum persisted objects

- scenario and immutable `ScenarioVersion`
- references to destination and current-state versions
- scenario type `esop_exploration`
- assumptions with source and status
- constraints and nonnegotiables
- known unknowns and conflicts
- professional-review requirements
- `ScenarioReadiness`
- owner review/acknowledgment record

### Owning module

Scenario.

### Deterministic gates

Before owner review:

- referenced destination and snapshot versions exist and are current
- scenario is labeled exploratory and not a recommendation
- assumptions, unknowns, conflicts, and source references are visible
- no calculated tax, valuation, financing, legal, or fiduciary conclusion is presented
- California and federal applicability questions are routed to qualified review
- all destination nonnegotiables are shown with alignment/conflict status
- scenario readiness is never labeled feasibility or probability of success

Before package creation:

- the owner acknowledges the scenario as an exploration
- readiness is at least `READY_FOR_PROFESSIONAL_REVIEW` under deterministic requirements
- no unresolved hard policy, consent, or nonnegotiable conflict permits automatic advancement

### Jev questions

- `scenario.readiness`
- `scenario.next_route`

### Maximum Jev authority

`signal_only`

Jev may score review readiness or suggest evidence, clarification, conflict resolution, professional review, owner review, or human triage. It cannot declare ESOP suitability, feasibility, compliance, or preference.

### Human/professional authority

The owner decides whether to continue exploring. Qualified professionals make legal, tax, valuation, fiduciary, financing, and other professional determinations.

### Audit events

- `ScenarioCreated`
- `ScenarioVersionCreated`
- `ScenarioAssumptionChanged`
- `ScenarioConflictDetected`
- `ScenarioUnknownAdded`
- `ScenarioAffectedByRealityChange`
- `ScenarioAffectedByDestinationChange`
- `ScenarioReadyForProfessionalReview`
- `ScenarioAcknowledgedForExploration`
- `JevEvaluationRecorded`
- `JevEvaluationInvalidated`

### Invalidation triggers

Invalidate readiness and route evaluations when:

- destination or current-state source versions change
- scenario assumptions, constraints, conflicts, or unknowns change
- a professional response changes scenario state
- consent, question, threshold policy, or model version changes

### Acceptance tests

- scenario language cannot represent an ESOP as recommended
- failed deterministic gates override a high Jev readiness result
- close route probabilities produce human triage
- unsupported assumptions remain visible
- a destination/current-state change invalidates prior readiness
- owner acknowledgment does not become a transaction decision
- professional questions are separated from platform assumptions

### Telemetry

- scenario version count
- readiness distribution
- evidence/clarification/professional-review route distribution
- Jev abstention and human-triage rate
- hard-gate override count
- owner acknowledgment rate
- invalidation-to-reevaluation latency

---

## 9. Stage Contract: Professional Review Package

### User outcome

The owner can inspect and authorize a purpose-limited package for one named, bring-your-own professional.

### Minimum persisted objects

- `ProfessionalReviewPackage`
- package version
- named recipient and stated professional role
- review purpose and scope
- included/excluded information manifests
- included source/version references
- `ConsentRecord`
- `DisclosureRecord`
- owner authorization
- expiration and revocation state
- package quality-evaluation reference

### Owning module

Professional Review Package. Consent & Access remains authoritative for disclosure.

### Deterministic gates

Before sharing:

- recipient identity and claimed role are recorded without implying verification
- review purpose and requested scope are explicit
- the package references immutable destination, snapshot, and scenario versions
- included and excluded categories are visible
- sensitive data is minimized for the recipient purpose
- owner authorization is explicit, current, recipient-specific, purpose-specific, and version-specific
- Consent & Access returns an allowed result
- expiration and revocation behavior are defined
- Jev never receives raw package documents in the initial slice
- a passing QA signal cannot authorize sharing

### Jev question

- `review_package.boundary_qa`

### Maximum Jev authority

`signal_only`

### Human/professional authority

The owner authorizes disclosure. The recipient decides whether to accept the engagement and controls their professional response.

### Audit events

- `ProfessionalReviewPackageCreated`
- `ProfessionalReviewPackageVersionCreated`
- `PackageQualityFlagged`
- `ConsentGranted`
- `ConsentRevoked`
- `DisclosureAuthorized`
- `PackageShared`
- `PackageAccessed`
- `JevEvaluationRecorded`
- `JevEvaluationInvalidated`

### Invalidation triggers

Invalidate QA and sharing readiness when:

- included content or source versions change
- recipient, role, purpose, scope, or visibility changes
- consent expires or is revoked
- scenario materiality changes
- the question, policy, or model version changes

### Acceptance tests

- no package can be shared without deterministic Consent & Access approval
- Jev cannot add content, broaden visibility, or authorize disclosure
- unsupported or recommendation-like language is flagged for human review
- recipient-purpose mismatch does not silently remove or add information
- revocation blocks future access under the relevant access model
- package history preserves exactly what was shared

### Telemetry

- package preparation time
- included/excluded category counts
- owner revision count before authorization
- QA issue distribution
- QA abstention/human-triage rate
- disclosure denial/revocation rate
- package access acknowledgment rate

---

## 10. Stage Contract: Professional Review

### User outcome

The owner and one named professional can exchange scoped questions, information requests, feedback, requested changes, and attributed determinations, after which the owner acknowledges the response and chooses a safe next route.

### Minimum persisted objects

- professional assignment and review scope
- immutable package-version reference
- questions and information requests
- professional feedback
- `ProfessionalDetermination`
- requested changes
- review status and version history
- owner acknowledgment
- next-route decision record

### Owning module

Professional Review.

### Deterministic gates

Before review completion:

- professional response is attributed to the responding identity and role
- determination text is recorded exactly or explicitly structured/confirmed by the professional
- platform summaries cannot replace the source response
- unresolved requests and conflicts remain visible
- owner acknowledgment is distinct from agreement
- the platform does not convert professional judgment into platform truth
- no transaction handoff occurs in this slice

### Jev question

None in the initial slice.

Jev may later support communication triage or package QA under separate registered questions. It never creates, resolves, ranks, or normalizes a professional determination.

### Maximum Jev authority

`no_role`

### Human/professional authority

The professional owns their attributed judgment. The owner acknowledges it and decides whether to revise the destination, revise the scenario, seek another professional view, pause, or end the exploration.

### Audit events

- `ProfessionalInvited`
- `ProfessionalAccepted`
- `AssignmentCreated`
- `QuestionCreated`
- `InformationRequested`
- `InformationReceived`
- `FeedbackSubmitted`
- `DeterminationSubmitted`
- `ChangeRequested`
- `ReviewCompleted`
- `OwnerAcknowledgedProfessionalResponse`
- `OwnerSelectedNextRoute`

### Invalidation triggers

A new package or review round is required when:

- material destination, current-state, or scenario state changes
- the professional requests a revised package
- review scope, recipient, or consent changes
- the owner seeks a second professional view

Existing determinations remain immutable and attributed; they are superseded, not rewritten.

### Acceptance tests

- only the professional can submit or confirm their determination
- owner acknowledgment is not recorded as agreement
- conflicting professional views remain separate
- platform language never changes “professional believes” into “platform concludes”
- revised inputs create a new review/package version
- completion offers no automatic transaction handoff
- the owner can pause or end the journey without selecting a structure

### Telemetry

- invite acceptance rate
- time to first professional response
- information-request count and resolution time
- review rounds per scenario
- professional determination/feedback mix
- owner acknowledgment time
- next-route distribution
- second-opinion rate

---

## 11. Cross-Cutting Jev Invocation Contract

Before every Jev invocation:

1. deterministic authorization and hard gates pass
2. a versioned external-AI consent record—not professional-sharing consent—is active for the registered purpose
3. state is built only from the question's machine-readable input allowlist
4. raw documents, secrets, credentials, unrelated identifiers, customer records, and all registered forbidden fields are blocked
5. nonempty source object/version references, exact input fields, and minimization-policy ID/version are recorded
6. canonical state, request, and criteria hashes are recorded
7. an immutable external-AI disclosure record captures the recipient, purpose, consent version, source versions, minimized fields, policy version, and hashes before the request leaves the private boundary
8. question and threshold-policy versions are fixed
9. maximum authority and fallback are loaded from the registry

After every response:

1. requested alias and returned model version are stored
2. complete typed probabilities and confidence metadata are preserved and checked for coherence
3. the selected answer must exist in the returned probability set and match the registered answer type
4. thresholds are evaluated outside the model
5. the resulting action/effect cannot exceed the registered authority ceiling
6. hard rules override the result
7. low confidence, low margin, unavailable service, invalid response, or invalid consent triggers fallback
8. the selected action, authority effect, fallback, and any human override are recorded
9. Jev is the actor for its output; the triggering human is not
10. source, question, threshold, minimization policy, or consent changes invalidate the evaluation

The manual path must remain usable when Jev is unavailable, unauthorized, abstains, or is disabled.

---

## 12. Release Sequence

### Release 0 — deterministic manual path

Build the complete slice without Jev. Prove that an owner can reach Professional Review safely using deterministic rules and human choices.

### Release 1 — shadow evaluation

Run registered Jev questions without changing the user path. Build labeled evaluation data and compare Jev results with human judgments.

### Release 2 — displayed signals

Show calibrated, explainable signals for approved questions. All signals remain non-authoritative.

### Release 3 — human-confirmed prompts

Allow `destination.next_clarification` to suggest a prompt that the owner explicitly accepts or rejects.

### Release 4 — reversible routing

Consider reversible Journey routing only after question-specific benchmark evidence, policy approval, rollback testing, and production threshold approval.

---

## 13. Slice Definition of Done

The slice is ready for a controlled pilot only when:

- all five stage contracts have executable schemas and contract tests
- deterministic hard gates are implemented and tested
- the manual path works with Jev disabled
- Jev authority is enforced in code
- consent, minimization, audit, invalidation, and fallback controls are implemented
- every active question has a labeled benchmark
- no design threshold is silently treated as production-approved
- security and privacy review covers California/federal data handling and external-AI processing
- qualified counsel reviews user-facing ESOP, California, federal, and professional-boundary language
- no real customer data is used before the security/compliance workstream's production gates are met

---

## 14. Architectural Lock

> The first Forhemit slice helps a controlling California business owner explore an ESOP path and prepare a purpose-limited package for their own qualified professional. Deterministic controls govern truth, access, consent, and hard gates. Jev may contribute bounded typed signals, never owner intent, professional judgment, authorization, calculation, recommendation, or transaction execution.