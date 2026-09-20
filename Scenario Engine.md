# Scenario Engine

**Platform:** Ownership Transition Platform
**Engine:** Scenario Engine
**Status:** Architectural Specification
**Implementation:** Not yet implemented
**Primary Principle:** Possible paths, not recommendations

---

## 1. Purpose

The Scenario Engine creates, models, compares, versions, and explains possible ownership-transition paths from the owner's current business reality toward the owner's desired destination.

It exists to answer:

> **What might happen if we pursued this path under these assumptions?**

It does not determine what the owner should do, what a professional should recommend, or whether a transaction is professionally feasible.

The Scenario Engine transforms:

**Destination + Business Reality + Evidence + Research + Explicit Assumptions**

into:

**Structured Scenarios + Comparisons + What-If Branches + Identified Unknowns + Identified Conflicts + Professional Review Packages + Transaction Handoff Context**

The engine must remain understandable to a nontechnical business owner while retaining enough structure for professional review, auditability, reproducibility, and eventual transaction execution.

---

# 2. Sole Responsibility

The Scenario Engine's sole responsibility is:

> **To represent and explore possible paths between the owner's current reality and desired destination using explicit assumptions, evidence, constraints, models, and dependencies, while preserving uncertainty and leaving professional and owner decisions to their respective authorities.**

The engine owns:

* scenario construction
* scenario structure
* scenario assumptions
* scenario constraints
* scenario dependencies
* scenario comparisons
* scenario branches
* scenario versions
* scenario status
* scenario impact analysis
* scenario readiness
* scenario explanation
* scenario-to-engine references
* scenario history

The engine does not own:

* owner objectives
* business facts
* professional determinations
* financial calculations
* financing approval
* legal conclusions
* tax conclusions
* valuation conclusions
* transaction execution
* owner decisions
* platform-wide permissions
* platform-wide audit history

---

# 3. Architectural Boundary

The core architecture is:

```text
                 OWNER
                   │
                   ▼
           ┌─────────────────┐
           │ Destination     │
           │ Where do I want │
           │ to go?          │
           └────────┬────────┘
                    │
                    ▼
           ┌─────────────────┐
           │ Scenario Engine │
           │ What paths might│
           │ connect the two?│
           └────────┬────────┘
                    │
        ┌───────────┼────────────┐
        ▼           ▼            ▼
 Financial      Capital       Research
 Modeling       / Financing   / Evidence
        │           │            │
        └───────────┼────────────┘
                    │
                    ▼
           ┌─────────────────┐
           │ Professional    │
           │ Review          │
           └────────┬────────┘
                    │
                    ▼
           ┌─────────────────┐
           │ Decision Record │
           │ What did owner  │
           │ decide?         │
           └────────┬────────┘
                    │
                    ▼
           ┌─────────────────┐
           │ Transaction /   │
           │ Orchestration   │
           └─────────────────┘
```

The Scenario Engine is therefore a **coordination and representation engine**, not a transaction-control engine.

---

# 4. Three Decision Layers

These layers are architecturally distinct.

## Layer 1: Owner Objective

The owner defines what they want.

Examples:

* Sell to employees
* Retire within 18 months
* Receive $3M at closing
* Receive $75K–$100K annually for 5–10 years
* Preserve employee jobs
* Retain some ownership
* Minimize post-closing involvement

The Destination Engine owns this layer.

---

## Layer 2: Platform Scenario

The Scenario Engine represents a possible path.

Examples:

* ESOP
* Direct employee purchase
* Management buyout
* Staged ownership
* Seller-financed acquisition
* Hybrid transition

A scenario is neither a recommendation nor a professional conclusion.

---

## Layer 3: Professional Determination

A qualified professional determines what should happen within their scope.

The professional may:

* validate assumptions
* reject assumptions
* modify a structure
* identify risks
* request additional information
* recommend another approach
* determine that a scenario is inappropriate
* request another scenario

The Scenario Engine stores references to these determinations but cannot transform them into platform-generated conclusions.

---

# 5. Architectural Relationships

## 5.1 Destination Engine

### Destination provides

* owner objective
* desired outcome
* priorities
* preferences
* nonnegotiables
* things to avoid
* timeframes

### Scenario consumes

A specific immutable Destination version.

### Scenario must retain

```text
destination_id
destination_version_id
destination_snapshot_reference
```

Changing the Destination does not rewrite historical scenarios.

---

# 6. Journey Engine

The Journey Engine provides context about where the owner is in the platform journey.

It may provide:

* current stage
* questions answered
* branching context
* progress
* relevant journey state

The Scenario Engine may use journey context to determine what information should be surfaced next.

It must not make the Journey Engine responsible for scenario logic.

---

# 7. Business Reality Engine

Business Reality represents the current state of the business.

It may contain:

* financial facts
* ownership facts
* operational facts
* business characteristics
* document-derived information
* owner-provided information
* verified and unverified facts

The Scenario Engine must preserve the status of every fact upon which an assumption depends.

A scenario must never silently convert:

> Unverified business fact

into:

> Verified scenario input.

Historical scenarios retain their original Business Reality version.

---

# 8. Fact Verification / Conflict Engine

This engine provides:

* fact status
* source
* verification status
* conflicting facts
* owner-reported information
* document-supported information
* professional verification

The Scenario Engine consumes this information.

If an assumption depends on conflicting facts, the scenario must explicitly surface that dependency.

Example:

> **Data conflict:** Revenue is reported as $8.3M in the accounting export and $10M in an owner-provided document.

The Scenario Engine does not choose between them.

---

# 9. Research Engine

Research provides external information such as:

* transaction norms
* industry benchmarks
* financing environment
* regulatory information
* market information
* supporting evidence
* contradictory evidence

Research must retain provenance.

A scenario assumption may therefore be displayed as:

> **Research-supported assumption**

or:

> **Research-challenged assumption**

Research never automatically becomes fact.

---

# 10. Evidence Ledger

Every material external assumption should be traceable to evidence.

Evidence metadata should include:

* evidence ID
* source
* source type
* publication date
* retrieval date
* applicable date
* geography
* population
* scope
* applicability
* supporting evidence
* contradictory evidence
* provenance status

The Scenario Engine references evidence rather than duplicating the Evidence Ledger.

---

# 11. Financial Modeling Integration

Financial Modeling owns mathematical calculations.

It handles:

* purchase price
* cash flow
* proceeds
* debt service
* seller notes
* ownership allocations
* projections
* sensitivity
* what-if calculations

The Scenario Engine owns:

> **Which financial model applies to this scenario?**

Financial Modeling owns:

> **What are the mathematical results?**

A scenario should therefore contain references such as:

```text
financial_model_id
financial_model_version_id
financial_model_snapshot
```

The Scenario Engine must never duplicate financial formulas merely to display a result.

---

# 12. Capital / Financing Integration

Capital / Financing owns:

* funding structures
* capital sources
* financing requirements
* indicative financing terms
* capital gaps
* financing requests

The Scenario Engine may contain financing assumptions but does not become the financing system.

Example:

```text
Scenario
    ↓
Financing Requirement: $4M
    ↓
Capital Engine
    ↓
Financing Request
    ↓
Scenario references resulting information
```

---

# 13. Seller-Note Integration

The Seller-Note Engine owns:

* note structure
* payment schedule
* note economics
* payment history
* liquidity alternatives
* potential note-sale scenarios

A scenario can reference a seller-note structure.

It does not own the note lifecycle.

---

# 14. Professional Review Integration

Professionals may:

* request changes
* reject assumptions
* request information
* request alternate scenarios
* provide determinations

The Scenario Engine preserves these as attributed external inputs.

It must distinguish:

```text
Platform Assumption
Professional Comment
Professional Requirement
Professional Determination
Owner Decision
```

These must never collapse into one field.

---

# 15. Professional Review Package Integration

The Professional Review Package Engine determines what information is appropriate for a specific professional.

The Scenario Engine supplies:

* scenario summary
* owner objectives
* constraints
* assumptions
* research references
* evidence
* financial model references
* unresolved questions
* conflicts
* known unknowns

The Package Engine owns disclosure and visibility decisions.

---

# 16. Confidence / Goal Alignment Integration

The Scenario Engine supplies factual inputs such as:

* objective alignment
* information completeness
* evidence quality
* research freshness
* professional review status
* unresolved conflicts

The Confidence / Goal Alignment Engine determines the resulting confidence signal.

The Scenario Engine must not create:

* probability of success
* universal scenario score
* recommendation score
* feasibility score

---

# 17. Decision Record Integration

Scenario:

> Employee purchase with seller financing.

Decision Record:

> Owner wants to explore this further because preserving employee jobs is more important to them than maximizing immediate cash.

The Scenario Engine stores the scenario.

The Decision Record Engine stores the owner's reasoning and decision.

This separation is mandatory.

---

# 18. Transaction / Orchestration Integration

Once an owner and appropriate professionals decide to move forward:

```text
Selected Scenario Version
        ↓
Transaction Handoff
        ↓
Transaction / Orchestration
```

The transaction references the source scenario.

The Scenario Engine does not become the transaction-management system.

---

# 19. Core Domain Objects

The Scenario Engine should use a set of related objects rather than placing everything inside one enormous Scenario record.

Core objects:

1. Scenario
2. Scenario Version
3. Scenario Snapshot
4. Scenario Assumption
5. Scenario Constraint
6. Scenario Unknown
7. Scenario Conflict
8. Scenario Dependency
9. Scenario Branch
10. Scenario Comparison
11. Scenario Impact
12. Scenario Evidence Link
13. Scenario Research Link
14. Scenario Financial Model Link
15. Scenario Financing Link
16. Scenario Seller-Note Link
17. Scenario Professional Review Reference
18. Scenario Status History
19. Scenario Selection Record
20. Scenario Handoff Record

---

# 20. Scenario Object

Conceptual structure:

```text
Scenario
├── scenario_id
├── scenario_family_id
├── scenario_name
├── scenario_type
├── description
├── owner_objective_reference
├── destination_reference
├── business_reality_reference
├── assumptions
├── constraints
├── unknowns
├── conflicts
├── dependencies
├── risks
├── evidence_references
├── research_references
├── financial_model_references
├── financing_references
├── seller_note_references
├── professional_review_references
├── stakeholders
├── readiness_status
├── lifecycle_status
├── created_at
├── updated_at
└── current_version_id
```

---

# 21. Scenario Family

A Scenario Family groups related versions and branches.

Example:

```text
Scenario Family A
│
├── A v1
├── A v2
├── A v3
│
├── A Branch 1
│   ├── A1 v1
│   └── A1 v2
│
└── A Branch 2
    └── A2 v1
```

This prevents branches from becoming disconnected scenarios with no lineage.

---

# 22. Scenario Version

Every material change creates a new version.

A version contains:

* version number
* parent version
* changed fields
* change reason
* changed-by identity
* timestamp
* Destination version
* Business Reality version
* Financial Model version
* research references
* professional feedback available at that point
* assumption set
* constraint set
* unknown set
* conflict set

Historical versions are immutable.

---

# 23. Scenario Snapshot

A snapshot makes the scenario reproducible.

It captures the references required to understand what information existed at that time.

Minimum snapshot:

```text
Destination Version
Business Reality Version
Research References
Evidence References
Financial Model Version
Financing References
Seller-Note References
Professional Feedback References
Scenario Assumptions
Scenario Constraints
Scenario Unknowns
Scenario Conflicts
```

A snapshot does not necessarily duplicate every external object. It records the authoritative version/reference required to reproduce the scenario.

---

# 24. Scenario Types

The taxonomy must be extensible.

Initial examples:

```text
ESOP
DIRECT_EMPLOYEE_PURCHASE
MANAGEMENT_BUYOUT
EMPLOYEE_OWNED_ACQUISITION_ENTITY
STAGED_OWNERSHIP
SELLER_FINANCED_EMPLOYEE_ACQUISITION
HYBRID
RETAIN_AND_TRANSITION
OTHER
```

The taxonomy must not hard-code assumptions about ESOPs.

New structures can be introduced without rewriting the Scenario Engine.

---

# 25. Scenario Assumptions

Every material scenario assumption must be explicit.

Example:

```text
Assumption:
Business valuation assumed at $10M.
```

Each assumption should include:

```text
assumption_id
category
value
unit
source_reference
source_date
created_date
verification_status
confidence_status
owner_provided
research_supported
professionally_reviewed
scenario_assumption
effective_from
effective_to
notes
```

An assumption is not a fact merely because it has a source.

---

# 26. Assumption Categories

Recommended categories:

* Business
* Financial
* Ownership
* Market
* Financing
* Tax
* Legal
* Operational
* Timing
* Employee
* Governance
* Seller
* Buyer
* External Environment

Legal and tax assumptions require particular care.

The platform may represent:

> Professional-supplied tax assumption

It must not independently conclude:

> This transaction has tax consequence X.

---

# 27. Constraint Model

Constraints must be typed.

## Hard Constraint

Cannot be violated without explicit owner change.

## Strong Preference

Important but potentially negotiable.

## Preference

Desired but flexible.

## Unknown

Not yet determined.

## Professional Requirement

Requirement identified by a qualified professional.

## External Constraint

Constraint caused by an external condition.

These are not interchangeable.

---

# 28. Nonnegotiables

A nonnegotiable is a hard owner constraint.

Examples:

* minimum closing proceeds
* maximum personal guarantee
* required employee ownership
* maximum transition period
* prohibition on strategic sale

The Scenario Engine must never silently modify a nonnegotiable.

If a scenario conflicts:

> ⚠️ **Nonnegotiable conflict**

Then display:

* the requirement
* the scenario value
* the difference
* possible next actions

Possible actions:

* Explore another scenario
* Adjust assumptions
* Explicitly change requirement

Changing the requirement creates a new Destination version.

---

# 29. Avoidance Constraints

Things the owner wants to avoid are distinct from positive objectives.

Example:

> Owner does not want personal guarantees.

Scenario:

> Personal guarantee required.

Display:

> ⚠️ **Conflicts with stated avoidance**

Do not silently reinterpret this as a preference.

---

# 30. Unknowns

Unknowns are first-class objects.

Examples:

* valuation unknown
* financing terms unknown
* employee participation unknown
* tax treatment requires professional determination
* governance structure unknown

Every unknown should have:

```text
unknown_id
description
category
importance
blocking_level
owner
resolution_status
required_action
source_dependency
```

Unknowns must not prevent preliminary scenarios unless the missing information genuinely makes the scenario impossible to represent.

---

# 31. Missing Information Prioritization

Missing information should be classified as:

### Critical

Without it, the scenario cannot be meaningfully modeled.

### Important

Would materially improve the scenario.

### Helpful

Would improve precision but is not required.

This prevents the platform from turning every scenario into a document-request avalanche.

---

# 32. Conflicts

Conflicts are typed.

## Owner Objective Conflict

Scenario does not meet a stated objective.

## Nonnegotiable Conflict

Scenario violates a must-have.

## Data Conflict

Underlying business facts disagree.

## Research Conflict

Evidence supports competing assumptions.

## Professional Conflict

Professionals provide differing views.

## Model Conflict

Different assumptions produce materially different outcomes.

These must remain distinct because they require different responses.

---

# 33. Professional Disagreement

If:

Professional A:

> Structure may work.

Professional B:

> Structure requires significant modification.

The Scenario Engine stores both.

It must not:

* average them
* vote between them
* determine which professional is correct
* convert disagreement into a score

Display:

> **Professional views differ.**

The Professional Review system remains responsible for resolving or managing professional disagreement.

---

# 34. Scenario Readiness

Use **Scenario Readiness**, not Deal Feasibility.

Possible states:

```text
PRELIMINARY
INFORMATION_NEEDED
MODELABLE
READY_FOR_COMPARISON
READY_FOR_PROFESSIONAL_REVIEW
UNDER_PROFESSIONAL_REVIEW
REVISED
SUPERSEDED
ARCHIVED
```

A scenario can be:

> Modelable

while still being:

> Professionally unvalidated.

---

# 35. Lifecycle

Recommended lifecycle:

```text
Idea
  ↓
Draft
  ↓
Preliminary
  ↓
Modelable
  ↓
Comparable
  ↓
Ready for Professional Review
  ↓
Under Review
  ↓
Revised
  ↓
Selected for Further Exploration
  ↓
Transaction Handoff
  ↓
Historical
```

A scenario may branch at any point.

Archived scenarios remain accessible.

---

# 36. Scenario Comparison

Comparison is a core capability.

The owner may compare:

| Dimension               | Scenario A     | Scenario B     | Scenario C   |
| ----------------------- | -------------- | -------------- | ------------ |
| Ownership outcome       | Employee-owned | Management-led | Hybrid       |
| Closing proceeds        | Model result   | Model result   | Model result |
| Seller financing        | Yes            | No             | Partial      |
| Transition timing       | 18 months      | 30 months      | 24 months    |
| Employee ownership      | High           | Low            | Medium       |
| Key unknown             | Valuation      | Financing      | Governance   |
| Nonnegotiable conflicts | None           | 1              | None         |

The interface must never declare a winner.

---

# 37. Comparison Dimensions

The system should support:

* owner objectives
* financial outcomes
* ownership outcomes
* personal outcomes
* business outcomes
* timing
* financing structure
* seller-note exposure
* uncertainty
* information completeness
* evidence quality
* research support
* nonnegotiable conflicts
* professional review status
* outstanding questions

The owner should be able to choose which dimensions matter to the comparison.

---

# 38. Objective Alignment

The system may calculate factual alignment.

Example:

Owner objective:

> Retire within 18 months.

Scenario:

> Estimated transition period: 24 months.

Display:

> **Does not currently meet stated timeframe.**

If:

> Estimated transition period: 12–18 months.

Display:

> **Currently aligns with stated timeframe.**

Do not use:

* good
* bad
* best
* worst
* optimal

---

# 39. Scenario Scoring Boundary

Do not create:

> Scenario Score: 87/100

A universal score creates false precision and implies authority the engine does not possess.

Instead expose independent dimensions:

* objective alignment
* information completeness
* evidence quality
* financial model completeness
* nonnegotiable conflicts
* professional review status
* key unknowns

These are observations, not a ranking system.

---

# 40. What-If Exploration

What-if exploration is a signature capability.

Example:

> What happens if valuation is $8M instead of $10M?

The Scenario Engine creates a branch.

```text
Scenario A
│
├── Base Case
├── $8M valuation
└── $12M valuation
```

Financial Modeling calculates the numerical effects.

Scenario Engine records:

* changed assumption
* parent scenario
* branch lineage
* affected outcomes
* new conflicts
* new unknowns

The base scenario remains unchanged.

---

# 41. Scenario Branch

A branch contains:

```text
branch_id
parent_scenario_version_id
changed_assumptions
resulting_model_references
changed_outcomes
new_conflicts
new_unknowns
created_at
created_by
```

Branches are immutable historical paths once created.

---

# 42. Sensitivity

The Scenario Engine asks:

> Which assumptions materially change this scenario?

Financial Modeling performs the calculation.

Scenario Engine presents the implications.

Examples:

> Closing proceeds are highly sensitive to valuation.

> Seller-note burden is sensitive to interest rate.

> Employee ownership percentage is sensitive to purchase price.

The Scenario Engine does not independently calculate sensitivity.

---

# 43. Stress Testing

Support modeled condition sets such as:

* Conservative Case
* Base Case
* Strong Case
* User-Defined Stress Case

These are conditions, not predictions.

Correct:

> **Conservative assumption set**

Incorrect:

> Conservative prediction

---

# 44. Dependency Graph

Scenario dependencies should be represented explicitly.

Example:

```text
Business Revenue
       ↓
Valuation Assumption
       ↓
Purchase Price
       ↓
Financing Requirement
       ↓
Seller Proceeds
       ↓
Retirement Income
```

If valuation changes, the engine can identify downstream dependencies.

Financial Modeling calculates numerical effects.

Scenario explains the dependency chain.

---

# 45. Impact Analysis

When upstream information changes, the engine identifies potentially affected scenarios.

Example:

> Business Reality changed.

Display:

> **3 scenarios may be affected.**

Then show:

* changed assumption
* dependent scenario
* affected financial model
* affected nonnegotiable
* affected objective alignment
* required review

Do not silently recalculate historical scenarios.

---

# 46. Scenario Refresh

The system may offer:

> **Refresh scenario using current information**

Refreshing creates a new version.

Example:

```text
Scenario A v2
    ↓
Business Reality v8 introduced
    ↓
Refresh requested
    ↓
Scenario A v3
```

The historical v2 remains unchanged.

---

# 47. Scenario Freshness

Every scenario should show its information vintage.

Example:

> Last refreshed: September 19, 2026

> Based on Business Reality v8

> Financial Model v5

> Research reviewed September 12

This prevents stale scenarios from appearing current.

---

# 48. Research Attachment

Research is attached to assumptions and questions.

Example:

```text
Assumption:
Employee ownership financing is available under these conditions.

Supporting Evidence:
Source A
Source B
Source C

Contradictory Evidence:
Source D
```

The owner can inspect the evidence.

Research must preserve:

* source
* date
* scope
* geography
* population
* applicability
* freshness

---

# 49. Scenario Explanation

Every scenario needs a human-readable explanation.

Example:

> **Employee Ownership + Seller Financing**
>
> This scenario models a transition in which employees acquire a majority ownership interest, with part of the purchase price financed through a seller note.
>
> It currently assumes a $10M valuation, $3M paid at closing, and $2M seller financing.
>
> These assumptions have not yet been professionally validated.

The explanation must be generated from structured scenario data rather than becoming a separate uncontrolled narrative.

---

# 50. Explainability

Every major output should answer:

> **Why does the system show this?**

Example:

> Closing proceeds are modeled at $3M because the scenario currently assumes a $10M purchase price and $7M of non-cash or financed consideration.

The user should be able to drill from:

**Displayed Result → Scenario Assumption → Financial Model → Source Inputs**

---

# 51. Scenario Interaction Design

Do not begin with a giant scenario-builder form.

Start with a small number of meaningful choices.

Example:

### What are you trying to accomplish?

* Sell to employees
* Sell to management
* Explore both

Then:

### How much control do you want to retain?

* Exit completely
* Retain some ownership
* Not sure

Then:

### How quickly do you want to transition?

* Under 18 months
* 18–36 months
* Flexible

The system progressively constructs the scenario.

---

# 52. Scenario Builder Interaction Types

Supported interaction types:

* single select
* multi-select
* ranking
* trade-off
* range
* numeric input
* what-if
* confidence
* Not Sure / Help Me Decide
* optional explanation

The interaction type should match the information being requested.

---

# 53. Progressive Disclosure

The interface should initially expose only information necessary for the current decision.

Advanced information appears when relevant.

Example:

```text
Owner Goal
   ↓
Ownership preference
   ↓
Timing
   ↓
Financing preference
   ↓
Scenario generated
   ↓
Unknowns revealed
   ↓
Advanced assumptions
   ↓
Financial model
```

This allows the owner to explore without becoming an accidental investment banker.

---

# 54. Local-First Architecture

The Scenario Engine must operate with locally available data when possible.

Potential local inputs:

* Business Reality
* financial documents
* assumptions
* models
* research records
* scenario history

Sensitive information should not automatically leave the local environment.

Only explicitly authorized information should be shared externally.

---

# 55. Research Privacy

Before external research:

* minimize personal identifiers
* remove unnecessary sensitive information
* use only necessary information
* record what information was used
* preserve research provenance

Do not claim perfect anonymization.

---

# 56. Security and Permissions

The Scenario Engine integrates with:

* Identity & Access
* Consent & Access
* Local Vault
* Professional Review Package
* Audit / Provenance

It must not create a parallel permission system.

Scenario records may contain:

* financial information
* ownership information
* financing assumptions
* professional comments
* strategic plans

Therefore access must be inherited from platform-level authorization.

---

# 57. Auditability

Platform-wide Audit / Provenance owns the authoritative audit trail.

Every material scenario change should produce an auditable record containing:

```text
actor
timestamp
scenario_id
scenario_version
field_changed
previous_value
new_value
reason
source
```

The Scenario Engine emits the relevant event.

Audit infrastructure records it.

---

# 58. Event Model

Recommended events:

```text
ScenarioCreated
ScenarioVersionCreated
ScenarioUpdated
ScenarioAssumptionChanged
ScenarioConstraintAdded
ScenarioConflictDetected
ScenarioUnknownAdded
ScenarioResearchAttached
ScenarioFinancialModelAttached
ScenarioFinancingAttached
ScenarioSellerNoteAttached
ScenarioCompared
ScenarioWhatIfCreated
ScenarioRefreshRequested
ScenarioReadyForProfessionalReview
ScenarioProfessionalFeedbackReceived
ScenarioSelectedForFurtherExploration
ScenarioArchived
ScenarioAffectedByRealityChange
ScenarioAffectedByDestinationChange
ScenarioReadyForTransactionHandoff
```

Events may trigger:

* workflow
* notifications
* confidence recalculation
* professional review
* decision records
* audit
* transaction handoff

The Scenario Engine does not directly orchestrate those downstream systems.

---

# 59. API / Engine Contract

Conceptual contract:

```text
createScenario()
cloneScenario()
createScenarioVersion()
getScenario()
getScenarioVersion()
updateScenario()

addAssumption()
updateAssumption()
removeAssumption()

addConstraint()
updateConstraint()

addUnknown()
resolveUnknown()

addConflict()
resolveConflict()

addDependency()
getDependencyGraph()

attachEvidence()
attachResearch()
attachFinancialModel()
attachFinancing()
attachSellerNote()

compareScenarios()
createWhatIfBranch()

requestScenarioRefresh()
getScenarioImpact()

requestProfessionalReview()
recordProfessionalFeedbackReference()

selectScenarioForFurtherExploration()
archiveScenario()

getScenarioHistory()
getScenarioSnapshot()
getScenarioReadiness()
getScenarioExplanation()

prepareTransactionHandoff()
```

Exact implementation contracts should be finalized during technical design.

---

# 60. Command vs Query Boundary

The API should distinguish commands from read operations.

## Commands

Change state:

```text
createScenario
updateScenario
addAssumption
addConstraint
createWhatIfBranch
requestScenarioRefresh
archiveScenario
selectScenarioForFurtherExploration
```

## Queries

Read state:

```text
getScenario
getScenarioHistory
getScenarioImpact
getScenarioSnapshot
compareScenarios
getScenarioReadiness
```

This supports auditability and future event-driven architecture.

---

# 61. Scenario Selection

When the owner says:

> I want to explore Scenario B further.

The system records:

> **Owner selected Scenario B for further exploration.**

It does not record:

> Scenario B was recommended.

Selection is an owner action.

Decision Record may capture the reason.

---

# 62. Scenario Archive

Scenarios should not be deleted merely because they are not selected.

They become archived.

The owner can later see:

> What we considered six months ago.

This supports:

* decision history
* professional review
* reconsideration
* comparison
* auditability

---

# 63. Scenario and Destination Changes

If Destination changes:

```text
Destination v3
Desired proceeds = $3M

Destination v4
Desired proceeds = $2M
```

Existing scenarios remain tied to v3.

New scenario versions may be created against v4.

Existing history is never rewritten.

---

# 64. Scenario and Business Reality Changes

If:

```text
Revenue:
$10M → $8.3M
```

Affected scenarios are flagged:

> **Scenario may be affected by updated business information.**

The system does not silently recalculate the historical scenario.

It offers:

> Refresh scenario using current information.

That creates a new version.

---

# 65. Professional Feedback

Professional requests may create new scenario versions.

Example:

> Professional requests an alternative with 40% seller financing.

The Scenario Engine creates:

> Scenario B v2

The request is retained as professional-originated context.

The resulting scenario remains a platform scenario until the professional makes a professional determination.

---

# 66. Scenario-to-Transaction Handoff

The handoff should include:

* selected scenario version
* owner objectives
* assumptions
* financial model references
* financing references
* professional determinations
* outstanding conditions
* relevant research
* known risks
* unknowns
* nonnegotiables
* required professional actions

The handoff should reference the source scenario rather than blindly copying all data.

---

# 67. Transaction Boundary

Once Transaction / Orchestration takes over, the Scenario Engine steps back.

Scenario remains available for:

* historical reference
* what-if exploration
* alternate scenarios
* changed assumptions

It does not manage:

* closing tasks
* document execution
* participant scheduling
* funding
* transaction milestones

---

# 68. Error and Exception Handling

The engine should never hide an incomplete or contradictory state.

Examples:

### Missing Dependency

> Financial model required before displaying modeled proceeds.

### Conflicting Fact

> Two source facts disagree on annual revenue.

### Stale Research

> This assumption relies on research outside the configured freshness window.

### Awaiting Professional Determination

> Tax treatment requires professional review.

### Broken Reference

> Referenced Financial Model v4 is unavailable.

### Invalid Constraint

> Scenario cannot be evaluated because the owner's minimum proceeds requirement has no defined currency.

### Outdated Scenario

> Business Reality has changed since this scenario was created.

The correct response is to surface the condition, not silently repair it.

---

# 69. Graceful Degradation

A scenario should remain useful even when incomplete.

For example:

```text
Valuation: Unknown
Financing: Unknown
Tax treatment: Pending professional determination
Employee participation: Unknown
```

The system can still present:

> **Preliminary scenario**

and explain:

> What is known
> What is assumed
> What remains unknown

---

# 70. Robustness Requirements

The engine must support:

* incomplete information
* conflicting information
* changing objectives
* changing business facts
* stale research
* multiple professional views
* multiple scenario branches
* historical reproducibility
* scenario refresh
* professional feedback
* disconnected external services
* partial financial models
* unavailable research sources

No single missing input should collapse the entire scenario system unless that input is genuinely required for the requested operation.

---

# 71. Architectural Anti-Patterns

The following are prohibited.

## 71.1 Universal Scenario Score

Do not create:

> Scenario A = 87/100

---

## 71.2 Hidden Recommendation

Do not display neutral data in a way that secretly encodes:

> Scenario A is the preferred option.

---

## 71.3 Winner Selection

Do not create:

> Winner: Scenario A

---

## 71.4 Silent Assumption Conversion

Never convert:

> Owner-provided estimate

into:

> Verified fact

---

## 71.5 Silent Conflict Resolution

Never select one conflicting data source without recording the conflict.

---

## 71.6 Historical Mutation

Never allow current Business Reality to silently rewrite historical scenarios.

---

## 71.7 Financial Duplication

Do not recreate Financial Modeling calculations inside Scenario Engine.

---

## 71.8 Professional Simulation

Do not generate statements that imply legal, tax, valuation, accounting, lending, or other professional determination.

---

## 71.9 Transaction Creep

Do not turn Scenario Engine into a closing or workflow-management engine.

---

## 71.10 Giant Scenario Questionnaire

Do not expose every possible question simultaneously.

---

## 71.11 Monolithic Scenario Object

Do not place assumptions, evidence, financial calculations, professional determinations, decisions, and transaction tasks into one giant mutable object.

---

## 71.12 Recommendation by Presentation

Avoid visual designs that implicitly rank scenarios through:

* default highlighting
* winner badges
* unexplained green/red scoring
* sorted "best match" ordering
* recommendation labels

Comparison must remain informational.

---

# 72. Scenario Transparency Rules

Every scenario should make five things immediately visible:

### 1. What is known?

Verified or appropriately sourced inputs.

### 2. What is assumed?

Explicit scenario assumptions.

### 3. What is unknown?

Missing information.

### 4. What conflicts?

Objectives, constraints, facts, evidence, models, or professional views.

### 5. What requires professional determination?

Items outside platform authority.

This becomes the core transparency pattern.

---

# 73. Scenario Readiness View

A scenario dashboard should communicate:

```text
Scenario:
Employee Ownership + Seller Financing

Readiness:
Modelable

Destination:
Retire within 18 months

Objective Alignment:
3 aligned
1 partial
1 unresolved

Nonnegotiable Conflicts:
0

Critical Unknowns:
2

Research:
4 supporting
1 contradictory

Financial Model:
Available

Professional Review:
Not yet reviewed
```

This is a factual state representation, not a recommendation.

---

# 74. Scenario Comparison UX

The comparison interface should begin with a small set of dimensions.

Example:

```text
Compare by:

☑ Owner objectives
☑ Timing
☑ Closing proceeds
☑ Ownership
☐ Seller-note exposure
☐ Research support
☑ Unknowns
☑ Nonnegotiables
```

The owner controls what they want to inspect.

---

# 75. Scenario Detail UX

Recommended hierarchy:

```text
Scenario Summary
    ↓
Why this scenario exists
    ↓
What it assumes
    ↓
What the model shows
    ↓
What is unknown
    ↓
What conflicts
    ↓
Evidence
    ↓
Research
    ↓
Professional review
    ↓
What-if exploration
    ↓
Version history
```

This preserves progressive disclosure.

---

# 76. "Help Me Decide" Boundary

The platform may support:

> **Not Sure / Help Me Decide**

But this means helping the owner understand trade-offs, not selecting an outcome.

For example:

> If preserving employee ownership is a high priority, these scenarios differ in how much employee ownership they currently model.

The system may explain consequences.

It must not conclude:

> Therefore choose Scenario A.

---

# 77. Goal Alignment Without Recommendation

The engine may report:

```text
Objective:
Receive at least $3M at closing.

Scenario:
$2.4M modeled.

Status:
Does not currently meet stated objective.
```

This is factual.

It must not translate that into:

> Therefore this scenario is inferior.

---

# 78. Scenario Freshness and Version Display

Every scenario view should make its information vintage visible.

Example:

```text
Scenario A v3

Created:
September 19, 2026

Destination:
v4

Business Reality:
v8

Financial Model:
v5

Research:
Reviewed September 12, 2026

Professional Feedback:
2 items available
```

The user should never have to guess whether a scenario is stale.

---

# 79. Scenario History

The history interface should answer:

> What changed?

> Why did it change?

> Who changed it?

> What information changed?

> What outcomes changed?

Example:

```text
Scenario A v2 → v3

Changed:
Valuation assumption

Previous:
$10M

New:
$8.5M

Reason:
Updated valuation input

Source:
Business Reality v8

Impact:
Financial Model v5 recalculated

New conflict:
Closing proceeds below owner minimum
```

---

# 80. Reproducibility Requirement

A historical scenario must remain understandable even after underlying systems evolve.

Example:

```text
Scenario A v2
created using:

Destination v3
Business Reality v5
Financial Model v3
Research snapshot R12
Professional feedback available at creation
```

Later changes do not mutate the historical record.

---

# 81. Scenario Data Integrity

Every reference should identify the version used.

Avoid:

```text
financial_model_id = 123
```

Prefer:

```text
financial_model_id = 123
financial_model_version = 5
```

Similarly:

```text
destination_version
business_reality_version
research_snapshot
professional_review_reference
```

This is essential to reproducibility.

---

# 82. Scenario Dependency Integrity

Dependencies should form a traceable graph.

For every material output:

```text
Output
  ↓
Model Result
  ↓
Scenario Assumption
  ↓
Source Fact / Research / Owner Input
```

The user should be able to trace the chain backward.

---

# 83. Scenario Provenance

Provenance should distinguish:

```text
OWNER_REPORTED
DOCUMENT_SUPPORTED
SYSTEM_DERIVED
RESEARCH_SUPPORTED
PROFESSIONALLY_SUPPLIED
PROFESSIONALLY_DETERMINED
SCENARIO_ASSUMED
MODEL_DERIVED
```

These statuses should not be collapsed into a generic confidence number.

---

# 84. Scenario Integrity Rules

The engine should reject or flag:

* assumptions without provenance where provenance is required
* scenario versions referencing nonexistent versions
* historical versions being modified
* conflicts marked resolved without resolution evidence
* professional determinations overwritten by platform edits
* Destination versions changing without new scenario context
* financial results displayed without a model reference
* stale model references presented as current
* nonnegotiables silently altered

---

# 85. Scenario Security Model

Security is inherited from platform-level systems.

The Scenario Engine should enforce domain-level access checks but should not invent its own identity model.

Sensitive scenario data includes:

* financial assumptions
* valuation assumptions
* financing
* ownership
* seller information
* employee information
* professional commentary
* strategic objectives

External disclosure must require explicit authorization.

---

# 86. Transaction Handoff Object

The Scenario-to-Transaction handoff should conceptually contain:

```text
handoff_id
source_scenario_id
source_scenario_version
destination_version
business_reality_version
owner_objectives
selected_assumptions
financial_model_references
financing_references
seller_note_references
professional_determinations
outstanding_conditions
known_unknowns
known_conflicts
nonnegotiables
required_actions
created_at
```

The transaction system becomes authoritative after handoff.

---

# 87. Scenario Does Not Own the Decision

The Scenario Engine can record:

> Owner selected Scenario B for further exploration.

It cannot record:

> Scenario B was the correct decision.

Decision authority remains with the owner and appropriate professionals.

---

# 88. Scenario Does Not Own Professional Authority

The Scenario Engine may state:

> Professional review pending.

It may state:

> Professional A and Professional B provided different views.

It cannot state:

> Professional A is correct.

unless that determination is itself supplied by an authoritative professional process.

---

# 89. Scenario Does Not Own Feasibility

Use:

> Scenario Readiness

not:

> Deal Feasibility.

A scenario may be:

> Modelable

while remaining:

> Professionally unresolved.

---

# 90. Architectural Test Cases

The implementation should eventually pass at least these conceptual tests.

## Test 1: Destination Change

Destination v3 → v4.

Expected:

* existing scenarios unchanged
* new scenario versions may reference v4
* history preserved

---

## Test 2: Business Reality Change

Revenue $10M → $8.3M.

Expected:

* affected scenarios identified
* historical scenarios unchanged
* refresh offered
* new scenario version created upon refresh

---

## Test 3: Nonnegotiable Conflict

Minimum closing proceeds = $3M.

Scenario result = $2.4M.

Expected:

> Nonnegotiable conflict.

No silent modification.

---

## Test 4: Conflicting Facts

Two revenue sources disagree.

Expected:

> Data conflict.

No automatic selection without an authorized resolution process.

---

## Test 5: Professional Disagreement

Professional A and B disagree.

Expected:

> Professional views differ.

No averaging.

---

## Test 6: What-If

Valuation changes from $10M to $8M.

Expected:

* branch created
* base preserved
* financial model recalculated externally
* changed outcomes attached

---

## Test 7: Historical Reproducibility

Business Reality advances from v5 to v8.

Expected:

Historical scenario created against v5 remains reproducible.

---

## Test 8: Recommendation Boundary

Scenario comparison contains factual differences.

Expected:

No winner, score, ranking, or recommendation.

---

# 91. Observability

The engine should expose operational telemetry for:

* scenario creation
* scenario version creation
* failed model references
* unresolved dependencies
* stale scenarios
* conflict frequency
* refresh frequency
* professional review requests
* scenario branching
* handoff events

Operational telemetry must not become a substitute for business-domain audit history.

---

# 92. Performance Considerations

The engine should favor references and immutable snapshots over duplicating entire external datasets.

Large objects such as:

* financial models
* research corpora
* source documents
* business records

should remain owned by their respective systems.

Scenario stores the minimum references necessary to reproduce context.

---

# 93. Extensibility

The Scenario Engine should allow future ownership-transition structures without architectural changes.

Possible future categories might include:

* cooperative structures
* staged recapitalizations
* partial employee ownership
* family succession
* partner buyouts
* community ownership structures
* hybrid investor/employee structures
* seller-retained minority ownership

The taxonomy must remain configuration-driven rather than hard-coded.

---

# 94. Final Architectural Lock

The following statements are architectural invariants.

> **Destination says where the owner wants to go.**

> **Business Reality says where the business is now.**

> **Research and Evidence say what external information supports or challenges assumptions.**

> **Scenario says what possible paths could connect the two.**

> **Financial Modeling calculates the numbers inside those paths.**

> **Capital / Financing describes potential funding structures.**

> **Seller-Note Engine handles seller-note economics and lifecycle.**

> **Professionals determine what should actually be done within their scope.**

> **Decision Records preserve what the owner ultimately decides.**

> **Transaction / Orchestration executes the chosen path.**

This separation is mandatory.

---

# 95. Concise Architectural Boundary Summary

| Engine                          | Owns                             | Does Not Own            |
|---------------------------------|----------------------------------|-------------------------|
| **Destination**                 | Owner objectives and constraints | Scenarios               |
| **Business Reality**            | Current business facts           | Scenario assumptions    |
| **Research**                    | External evidence                | Scenario decisions      |
| **Evidence Ledger**             | Provenance                       | Scenario lifecycle      |
| **Scenario**                    | Possible paths                   | Recommendations         |
| **Financial Modeling**          | Mathematical results             | Scenario selection      |
| **Capital / Financing**         | Financing structures             | Scenario decisions      |
| **Seller-Note Liquidity**       | Note lifecycle                   | Scenario lifecycle      |
| **Professional Review**         | Professional determinations      | Platform scenarios      |
| **Confidence**                  | Confidence/alignment signal      | Scenario score          |
| **Decision Record**             | Owner decisions/reasoning        | Scenario construction   |
| **Transaction / Orchestration** | Execution                        | Scenario history        |
| **Audit / Provenance**          | Platform audit trail             | Scenario business logic |

---

# 96. Implementation Roadmap

Implementation should occur in phases.

## Phase 1: Domain Foundation

Define and validate:

* Scenario
* Scenario Family
* Scenario Version
* Snapshot
* Assumption
* Constraint
* Unknown
* Conflict
* Dependency

Lock immutable/versioned behavior.

---

## Phase 2: Engine Contracts

Define contracts with:

* Destination
* Business Reality
* Research
* Evidence
* Financial Modeling
* Capital
* Seller Note
* Professional Review
* Decision Record

No implementation should bypass these boundaries.

---

## Phase 3: Scenario Lifecycle

Implement:

```text
Create
→ Draft
→ Preliminary
→ Modelable
→ Comparable
→ Professional Review
→ Revised
→ Further Exploration
→ Handoff
→ Historical
```

---

## Phase 4: Comparison

Implement:

* multi-scenario comparison
* objective alignment
* constraint conflicts
* unknowns
* evidence quality
* professional review state

No universal score.

---

## Phase 5: What-If Engine

Implement:

* branching
* parent-child lineage
* changed assumptions
* model references
* impact tracking

---

## Phase 6: Versioning and Reproducibility

Implement:

* immutable versions
* snapshots
* historical reconstruction
* source-version tracking
* scenario freshness

---

## Phase 7: Professional Review

Implement:

* review requests
* professional feedback references
* professional requirements
* disagreement preservation
* review-package integration

---

## Phase 8: Impact Analysis

Implement:

* Business Reality change detection
* Destination change detection
* dependency traversal
* affected-scenario identification
* refresh workflow

---

## Phase 9: Transaction Handoff

Implement:

* source scenario reference
* handoff package
* outstanding conditions
* professional determinations
* required actions

Then transfer authority to Transaction / Orchestration.

---

## Phase 10: UX Refinement

Only after the underlying architecture is stable, optimize:

* progressive disclosure
* decision cards
* scenario comparison
* what-if interaction
* explanations
* visual dependency maps
* scenario history

The UI should expose the architecture rather than hide it.

---

# 97. Final Definition

The Scenario Engine is:

> **The structured bridge between where an owner is, where the owner wants to go, and the possible paths between those points.**

It creates and preserves scenarios.

It makes assumptions visible.

It exposes uncertainty.

It identifies conflicts.

It connects evidence and models.

It allows alternatives and what-if branches.

It preserves history.

It prepares scenarios for professional review.

It can hand a selected scenario into transaction execution.

But it does **not** decide what the owner should do.

That boundary is the foundation of the engine.

---

# 98. Architectural Principle

The simplest way to remember the architecture is:

```text
DESTINATION
     │
     │  Where do I want to go?
     ▼
SCENARIO ENGINE
     │
     │  What paths could get me there?
     │
     ├───────────────┐
     ▼               ▼
EVIDENCE          MODELS
     │               │
     └───────┬───────┘
             ▼
      PROFESSIONAL REVIEW
             │
             │ What should actually happen?
             ▼
       OWNER DECISION
             │
             ▼
       TRANSACTION
             │
             │ Execute
             ▼
          OUTCOME
```

**The Scenario Engine is the bridge, not the driver.**
