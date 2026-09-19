Absolutely. I’d make the data model **more rigorous than the architectural prose**, because this is where we need to prevent future engines from accidentally collapsing facts, assumptions, models, professional determinations, and owner decisions into the same data.

The source establishes the key requirement that scenarios must be versioned, reproducible, traceable to Destination/Business Reality/Financial Model versions, and able to preserve assumptions, unknowns, conflicts, evidence, and professional feedback separately.

# Scenario Engine Data Model / Schema Specification

## 1. Schema Design Principles

The Scenario Engine data model should follow eight rules:

1. **Immutable historical versions**
2. **Explicit provenance**
3. **References instead of duplicated authority**
4. **Typed assumptions and constraints**
5. **First-class unknowns and conflicts**
6. **Branchable scenarios**
7. **Reproducible snapshots**
8. **No field that implies a recommendation**

The most important distinction is:

```text
FACT
  ≠
ASSUMPTION
  ≠
MODEL RESULT
  ≠
PROFESSIONAL DETERMINATION
  ≠
OWNER DECISION
```

Those should remain separate throughout the schema.

---

# 2. High-Level Entity Model

The Scenario Engine can be represented as:

```text
ScenarioFamily
      │
      ├── ScenarioVersion
      │       │
      │       ├── Assumptions
      │       ├── Constraints
      │       ├── Unknowns
      │       ├── Conflicts
      │       ├── Dependencies
      │       ├── EvidenceLinks
      │       ├── ResearchLinks
      │       ├── FinancialModelLinks
      │       ├── FinancingLinks
      │       ├── SellerNoteLinks
      │       ├── ProfessionalReviewRefs
      │       └── Snapshot
      │
      ├── ScenarioBranch
      │       └── ScenarioFamily
      │
      └── ScenarioHistory

ScenarioComparison
      └── ScenarioVersion[]

ScenarioHandoff
      └── ScenarioVersion
```

This gives us a clean separation between **identity**, **version**, and **contents**.

---

# 3. Scenario Family

A Scenario Family represents one conceptual path through its entire history.

For example:

```text
Employee Ownership + Seller Financing
```

might contain:

```text
Family S-001

S-001 v1
   ↓
S-001 v2
   ↓
S-001 v3
```

while a what-if branch becomes:

```text
S-001
 ├── Base
 ├── $8M valuation branch
 └── $12M valuation branch
```

### Schema

```text
ScenarioFamily
-----------------------------
scenario_family_id
business_id
name
scenario_type
created_at
created_by
archived_at
archive_reason
```

### Important

`ScenarioFamily` does **not** contain mutable scenario assumptions.

Those belong to versions.

---

# 4. Scenario Version

This is the primary historical object.

```text
ScenarioVersion
-----------------------------
scenario_version_id
scenario_family_id
version_number
parent_version_id

name
description
scenario_type

destination_version_id
business_reality_version_id

readiness_status
lifecycle_status

created_at
created_by

change_reason
supersedes_version_id

snapshot_id
```

### Example

```text
scenario_family_id:
SF-001

scenario_version_id:
SV-001-v3

version_number:
3

parent_version_id:
SV-001-v2

destination_version_id:
DEST-v4

business_reality_version_id:
BR-v8

readiness_status:
MODELABLE

lifecycle_status:
REVISED
```

---

# 5. Why Versioning Is Mandatory

The source explicitly requires historical scenarios to remain understandable even after Business Reality changes.

Therefore:

```text
Scenario v2
```

must never silently become:

```text
Scenario v2 using today's data
```

Instead:

```text
Scenario v2
   ↓
Historical snapshot

Scenario v3
   ↓
Current information
```

This is one of the most important integrity rules in the entire platform.

---

# 6. Scenario Snapshot

A snapshot defines the information state from which the scenario was constructed.

```text
ScenarioSnapshot
-----------------------------
snapshot_id

scenario_version_id

destination_version_id
business_reality_version_id

financial_model_version_id

research_snapshot_id

evidence_snapshot_id

financing_reference_set
seller_note_reference_set

professional_review_reference_set

created_at
```

The snapshot should reference authoritative versions rather than duplicate entire source systems.

---

# 7. Scenario Assumption

This is one of the most important entities.

```text
ScenarioAssumption
-----------------------------
assumption_id
scenario_version_id

category
name
description

value_type
value

unit
currency
range_min
range_max

source_type
source_reference

source_date

verification_status
provenance_status

owner_provided
research_supported
professionally_supplied
professionally_reviewed

effective_from
effective_to

created_at
created_by
```

---

# 8. Assumption Value Model

Avoid storing every value as a string.

Instead support typed values.

```text
value_type
----------------
NUMBER
CURRENCY
PERCENTAGE
DATE
DATE_RANGE
DURATION
BOOLEAN
TEXT
ENUM
RANGE
REFERENCE
```

Example:

```text
value_type: CURRENCY
value: 10000000
currency: USD
```

rather than:

```text
value: "$10M"
```

This lets Financial Modeling consume the assumption cleanly.

---

# 9. Assumption Provenance

The schema should distinguish how an assumption entered the system.

```text
provenance_status
-------------------------
OWNER_REPORTED
DOCUMENT_SUPPORTED
RESEARCH_SUPPORTED
PROFESSIONALLY_SUPPLIED
SCENARIO_ASSUMED
SYSTEM_DERIVED
```

This is critical.

For example:

```text
Valuation = $10M

provenance:
OWNER_REPORTED

scenario_status:
SCENARIO_ASSUMED

verification:
UNVERIFIED
```

The system can therefore say:

> The scenario assumes a $10M valuation based on owner-provided information that has not yet been professionally validated.

That is dramatically safer than simply storing:

```text
valuation = 10000000
```

---

# 10. Assumption Categories

Use a controlled but extensible taxonomy.

```text
BUSINESS
FINANCIAL
OWNERSHIP
MARKET
FINANCING
TAX
LEGAL
OPERATIONAL
TIMING
EMPLOYEE
GOVERNANCE
SELLER
BUYER
EXTERNAL_ENVIRONMENT
OTHER
```

The category is descriptive, not an indication that the platform has professional authority over that domain.

---

# 11. Verification Status

Separate verification from provenance.

```text
VerificationStatus
-------------------------
UNKNOWN
UNVERIFIED
PARTIALLY_VERIFIED
VERIFIED
PROFESSIONALLY_VERIFIED
CONTESTED
REJECTED
```

This allows:

```text
Owner-provided
+
Unverified
```

to coexist with:

```text
Professional-supplied
+
Professionally verified
```

---

# 12. Scenario Constraint

Constraints should not be generic text attached to a scenario.

```text
ScenarioConstraint
-----------------------------
constraint_id
scenario_version_id

constraint_type

name
description

value_type
value
unit
currency

source
source_reference

status

created_at
created_by
```

### Constraint types

```text
HARD
STRONG_PREFERENCE
PREFERENCE
UNKNOWN
PROFESSIONAL_REQUIREMENT
EXTERNAL_CONSTRAINT
```

---

# 13. Nonnegotiable

Because nonnegotiables have special behavior, I would model them explicitly rather than merely relying on:

```text
constraint_type = HARD
```

Use:

```text
ScenarioNonnegotiable
-----------------------------
nonnegotiable_id
scenario_version_id

destination_nonnegotiable_id

description
value

status

conflict_status

created_at
```

The `destination_nonnegotiable_id` maintains the authoritative relationship to the Destination Engine.

---

# 14. Why This Matters

Suppose:

```text
Owner minimum closing proceeds:
$3M
```

Scenario:

```text
Modeled closing proceeds:
$2.4M
```

The scenario should create a structured conflict:

```text
NonnegotiableConflict
```

rather than simply putting:

```text
warning = true
```

That gives us a durable relationship between:

```text
Owner requirement
        ↓
Scenario assumption/model result
        ↓
Conflict
```

---

# 15. Scenario Unknown

Unknowns are first-class objects.

```text
ScenarioUnknown
-----------------------------
unknown_id
scenario_version_id

category
description

importance
blocking_level

resolution_status

required_action
owner
source_dependency

created_at
resolved_at
resolved_by
resolution_reference
```

### Importance

```text
CRITICAL
IMPORTANT
HELPFUL
```

### Resolution status

```text
OPEN
IN_PROGRESS
RESOLVED
WAIVED
SUPERSEDED
```

---

# 16. Scenario Conflict

Conflicts need their own entity.

```text
ScenarioConflict
-----------------------------
conflict_id
scenario_version_id

conflict_type

severity

description

source_reference
affected_object_type
affected_object_id

status

resolution_type
resolution_reference

created_at
resolved_at
resolved_by
```

### Conflict types

```text
OWNER_OBJECTIVE
NONNEGOTIABLE
DATA
RESEARCH
PROFESSIONAL
MODEL
DEPENDENCY
OTHER
```

---

# 17. Conflict Severity

Severity should describe the condition, not produce a scenario score.

```text
INFORMATIONAL
ATTENTION
MATERIAL
BLOCKING
```

This is a useful distinction from prohibited universal scenario scoring.

---

# 18. Scenario Dependency

Dependencies make the scenario explainable.

```text
ScenarioDependency
-----------------------------
dependency_id
scenario_version_id

upstream_type
upstream_id

downstream_type
downstream_id

relationship_type

materiality

created_at
```

Example:

```text
Business Revenue
      ↓
Valuation Assumption
      ↓
Purchase Price
      ↓
Financing Requirement
```

---

# 19. Dependency Types

Examples:

```text
DEPENDS_ON
DERIVED_FROM
AFFECTS
REQUIRES
CONSTRAINS
TRIGGERS
```

This creates a graph rather than a flat list.

---

# 20. Evidence Link

The Scenario Engine should not own evidence.

It should reference it.

```text
ScenarioEvidenceLink
-----------------------------
link_id
scenario_version_id

evidence_id

relationship_type

supports
contradicts
contextualizes

created_at
created_by
```

A better implementation would make `relationship_type` one enum rather than three booleans:

```text
SUPPORTS
CONTRADICTS
CONTEXTUALIZES
```

---

# 21. Research Link

Similarly:

```text
ScenarioResearchLink
-----------------------------
link_id
scenario_version_id

research_finding_id

relationship_type

applicability
freshness_status

created_at
```

This allows:

```text
Research finding
      ↓
Scenario assumption
```

without copying research into Scenario.

---

# 22. Assumption-to-Evidence Relationship

This deserves a direct relationship.

```text
AssumptionEvidenceLink
-----------------------------
assumption_id
evidence_id

relationship_type
relevance
created_at
```

This allows the UI to answer:

> Why does this scenario assume $10M?

and drill into:

```text
$10M assumption
     ↓
Owner statement
     ↓
Document
     ↓
Professional valuation
```

---

# 23. Financial Model Reference

The Scenario Engine should reference, not reproduce, financial models.

```text
ScenarioFinancialModelLink
-----------------------------
link_id
scenario_version_id

financial_model_id
financial_model_version_id

relationship_type

created_at
```

### Relationship types

```text
PRIMARY_MODEL
ALTERNATIVE_MODEL
WHAT_IF_MODEL
STRESS_MODEL
SENSITIVITY_MODEL
```

---

# 24. Financial Model Output Reference

Scenario should also retain references to specific outputs.

```text
ScenarioModelOutputReference
-----------------------------
reference_id
scenario_version_id

financial_model_version_id

output_id
output_name

display_value
unit

retrieved_at
```

The authoritative calculation remains in Financial Modeling.

The displayed result is a cached/reference value for UX and reproducibility.

---

# 25. Financing Reference

```text
ScenarioFinancingLink
-----------------------------
link_id
scenario_version_id

financing_request_id
financing_version_id

relationship_type

created_at
```

Possible relationships:

```text
REQUIRED
OPTIONAL
ALTERNATIVE
WHAT_IF
```

---

# 26. Seller Note Reference

```text
ScenarioSellerNoteLink
-----------------------------
link_id
scenario_version_id

seller_note_id
seller_note_version_id

relationship_type

created_at
```

Again, the Scenario Engine does not become the seller-note ledger.

---

# 27. Professional Review Reference

Professional review needs careful separation.

```text
ScenarioProfessionalReviewReference
-------------------------------------
reference_id
scenario_version_id

professional_review_id

review_status

professional_role
professional_identity_reference

feedback_reference

created_at
```

The professional's actual determination should remain owned by Professional Review.

---

# 28. Professional Feedback

Do not duplicate the professional's authoritative record.

Instead:

```text
ProfessionalFeedbackReference
-----------------------------
reference_id
professional_review_id

feedback_id

feedback_type
created_at
```

Possible feedback types:

```text
REQUEST_INFORMATION
REQUEST_SCENARIO_CHANGE
IDENTIFY_RISK
CHALLENGE_ASSUMPTION
PROVIDE_REQUIREMENT
PROVIDE_DETERMINATION
REQUEST_ALTERNATIVE
```

---

# 29. Professional Determination Boundary

A critical rule:

```text
ScenarioProfessionalReviewReference
```

may say:

> Professional determination exists.

It must not transform that into:

```text
scenario_status = APPROVED
```

unless the platform explicitly defines an authoritative approval state owned by the appropriate professional workflow.

---

# 30. Scenario Branch

Branches should be explicit.

```text
ScenarioBranch
-----------------------------
branch_id

parent_scenario_version_id
child_scenario_family_id

branch_type

reason

changed_assumptions

created_at
created_by
```

### Branch types

```text
WHAT_IF
ALTERNATIVE
STRESS_CASE
PROFESSIONAL_REQUEST
OWNER_REQUEST
```

---

# 31. Branch Lineage

Example:

```text
Scenario A v3
     │
     ├── What-if: $8M valuation
     │       ↓
     │    Scenario A1 v1
     │
     ├── What-if: $12M valuation
     │       ↓
     │    Scenario A2 v1
     │
     └── Base
```

The parent must never be modified by creation of a branch.

---

# 32. Scenario Comparison

Comparison should itself be a durable object.

```text
ScenarioComparison
-----------------------------
comparison_id

business_id

name

scenario_version_ids[]

comparison_dimensions[]

created_at
created_by

status
```

The comparison should capture **what was compared**, not declare which scenario won.

---

# 33. Comparison Dimension

```text
ComparisonDimension
-----------------------------
dimension_id
comparison_id

dimension_type
label

display_order

created_at
```

Possible dimensions:

```text
OWNER_OBJECTIVE
FINANCIAL
OWNERSHIP
PERSONAL
BUSINESS
TIMING
FINANCING
SELLER_NOTE
UNCERTAINTY
INFORMATION_COMPLETENESS
EVIDENCE
NONNEGOTIABLE
PROFESSIONAL_REVIEW
OUTSTANDING_QUESTIONS
```

---

# 34. Comparison Result

```text
ScenarioComparisonResult
-----------------------------
result_id
comparison_id
scenario_version_id
dimension_id

result_type
value
status

source_reference
created_at
```

Example:

```text
dimension:
RETIREMENT_TIMING

scenario:
A

result:
24 months

status:
DOES_NOT_CURRENTLY_ALIGN
```

That is factual rather than evaluative.

---

# 35. Scenario Impact

When upstream data changes, impact should be represented explicitly.

```text
ScenarioImpact
-----------------------------
impact_id

source_change_type
source_change_id

scenario_version_id

affected_object_type
affected_object_id

impact_type
impact_description

severity

detected_at

status
```

Possible impact types:

```text
ASSUMPTION_AFFECTED
MODEL_AFFECTED
OBJECTIVE_ALIGNMENT_AFFECTED
NONNEGOTIABLE_AFFECTED
UNKNOWN_AFFECTED
CONFLICT_AFFECTED
RESEARCH_FRESHNESS_AFFECTED
```

---

# 36. Scenario Status History

Lifecycle status changes should be historical.

```text
ScenarioStatusHistory
-----------------------------
status_history_id
scenario_version_id

previous_status
new_status

reason

changed_by
changed_at

source_reference
```

---

# 37. Scenario Selection Record

The owner selecting a scenario for additional exploration is distinct from making a transaction decision.

```text
ScenarioSelectionRecord
-----------------------------
selection_id

scenario_version_id

selection_type

selected_by
selected_at

reason

decision_record_reference
```

### Selection types

```text
FURTHER_EXPLORATION
PROFESSIONAL_REVIEW
TRANSACTION_HANDOFF
ARCHIVE
```

This preserves the distinction established in the source between selecting a scenario for exploration and recommending it.

---

# 38. Transaction Handoff

The handoff becomes its own immutable object.

```text
ScenarioTransactionHandoff
-----------------------------
handoff_id

scenario_version_id

destination_version_id
business_reality_version_id

financial_model_version_id

owner_objective_snapshot

assumption_references
constraint_references
unknown_references
conflict_references

professional_determination_references

financing_references
seller_note_references
research_references

required_actions

created_at
created_by

handoff_status
```

---

# 39. Handoff Status

```text
PREPARING
READY
TRANSMITTED
ACKNOWLEDGED
SUPERSEDED
CANCELLED
```

The transaction system becomes authoritative after handoff.

---

# 40. Scenario Lifecycle Enum

I would separate **readiness** from **lifecycle**.

### Lifecycle

```text
IDEA
DRAFT
PRELIMINARY
MODELABLE
COMPARABLE
UNDER_PROFESSIONAL_REVIEW
REVISED
SELECTED_FOR_FURTHER_EXPLORATION
HANDED_OFF
HISTORICAL
ARCHIVED
SUPERSEDED
```

### Readiness

```text
PRELIMINARY
INFORMATION_NEEDED
MODELABLE
READY_FOR_COMPARISON
READY_FOR_PROFESSIONAL_REVIEW
UNDER_REVIEW
REVISED
SUPERSEDED
ARCHIVED
```

This prevents one overloaded status field from trying to represent two different concepts.

---

# 41. Recommended Core Enumerations

## Scenario Type

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

## Provenance

```text
OWNER_REPORTED
DOCUMENT_SUPPORTED
RESEARCH_SUPPORTED
PROFESSIONALLY_SUPPLIED
SCENARIO_ASSUMED
SYSTEM_DERIVED
MODEL_DERIVED
```

## Verification

```text
UNKNOWN
UNVERIFIED
PARTIALLY_VERIFIED
VERIFIED
PROFESSIONALLY_VERIFIED
CONTESTED
REJECTED
```

## Constraint

```text
HARD
STRONG_PREFERENCE
PREFERENCE
UNKNOWN
PROFESSIONAL_REQUIREMENT
EXTERNAL_CONSTRAINT
```

---

# 42. Recommended Database Relationship

Conceptually:

```text
scenario_family
       │
       │ 1:N
       ▼
scenario_version
       │
       ├──── 1:N ─── scenario_assumption
       ├──── 1:N ─── scenario_constraint
       ├──── 1:N ─── scenario_nonnegotiable
       ├──── 1:N ─── scenario_unknown
       ├──── 1:N ─── scenario_conflict
       ├──── 1:N ─── scenario_dependency
       ├──── 1:N ─── evidence_link
       ├──── 1:N ─── research_link
       ├──── 1:N ─── financial_model_link
       ├──── 1:N ─── financing_link
       ├──── 1:N ─── seller_note_link
       ├──── 1:N ─── professional_review_reference
       ├──── 1:1 ─── scenario_snapshot
       ├──── 1:N ─── status_history
       └──── 1:N ─── impact
```

---

# 43. External Authority Map

This should be treated as a hard schema boundary.

| Data                       | Authoritative Engine        |
| -------------------------- | --------------------------- |
| Owner objective            | Destination                 |
| Nonnegotiable              | Destination                 |
| Current business fact      | Business Reality            |
| Fact verification          | Fact Verification           |
| External evidence          | Evidence Ledger             |
| Research finding           | Research                    |
| Scenario assumption        | Scenario                    |
| Scenario constraint        | Scenario                    |
| Scenario branch            | Scenario                    |
| Financial calculation      | Financial Modeling          |
| Financing structure        | Capital / Financing         |
| Seller-note lifecycle      | Seller Note                 |
| Professional determination | Professional Review         |
| Confidence signal          | Confidence                  |
| Owner decision             | Decision Record             |
| Transaction execution      | Transaction / Orchestration |
| Audit history              | Audit / Provenance          |

The Scenario Engine should **reference authoritative data rather than clone authority**.

---

# 44. Immutability Rules

The following objects should be immutable after creation:

* Scenario Version
* Scenario Snapshot
* Historical Comparison
* Scenario Handoff
* Status History
* Professional Determination Reference
* Historical Branch lineage

Corrections create new records or versions.

They do not rewrite history.

---

# 45. Mutable Objects

The current working draft can remain mutable before version finalization.

For example:

```text
Scenario Draft
```

can be edited.

Once it becomes:

```text
Scenario Version 1
```

the version becomes immutable.

A change produces:

```text
Scenario Version 2
```

This gives the UX flexibility without sacrificing auditability.

---

# 46. Draft vs Version

This distinction is worth locking now.

```text
Scenario Draft
    ↓
User edits
    ↓
Finalize
    ↓
Scenario Version
```

A draft may be incomplete.

A version represents a coherent historical state.

This prevents every keystroke from becoming a formal version.

---

# 47. Version Creation Triggers

Create a new material version when:

* a material assumption changes
* a nonnegotiable changes
* Destination changes
* Business Reality changes and scenario is refreshed
* financial model changes materially
* financing structure changes
* professional feedback causes a scenario change
* scenario structure changes
* branch is created
* owner requests a materially different path

Minor presentation changes do not necessarily require a scenario version.

---

# 48. Scenario Identity Rules

The following should **not** create a new Scenario Family:

> Change valuation from $10M to $9M.

That is likely:

```text
Same Family
New Version
```

But:

> Explore management buyout instead of employee acquisition.

could be:

```text
New Scenario Family
```

unless explicitly modeled as a branch.

The system should preserve lineage rather than arbitrarily fragmenting scenarios.

---

# 49. Data Integrity Rules

The schema should enforce:

### Rule 1

Every Scenario Version references exactly one Destination version.

### Rule 2

Every Scenario Version references the Business Reality version used to create it.

### Rule 3

Every material financial output references a Financial Model version.

### Rule 4

Every material assumption has provenance.

### Rule 5

Every professional determination remains attributed.

### Rule 6

Historical versions cannot be mutated.

### Rule 7

A branch must identify its parent.

### Rule 8

A conflict cannot disappear merely because a new scenario version exists.

### Rule 9

Changing an owner nonnegotiable creates a new Destination version.

### Rule 10

Scenario data cannot become an implicit recommendation.

---

# 50. Minimal Scenario JSON Representation

For conceptual purposes, a Scenario Version might look like:

```json
{
  "scenarioVersionId": "SV-001-v3",
  "scenarioFamilyId": "SF-001",
  "version": 3,

  "name": "Employee Ownership + Seller Financing",
  "type": "SELLER_FINANCED_EMPLOYEE_ACQUISITION",

  "destination": {
    "destinationVersionId": "DEST-v4"
  },

  "businessReality": {
    "versionId": "BR-v8"
  },

  "readiness": "MODELABLE",
  "lifecycle": "REVISED",

  "assumptions": [
    {
      "id": "A-001",
      "category": "FINANCIAL",
      "name": "Business valuation",
      "valueType": "CURRENCY",
      "value": 10000000,
      "currency": "USD",
      "provenance": "OWNER_REPORTED",
      "verification": "UNVERIFIED"
    }
  ],

  "constraints": [],

  "nonnegotiables": [],

  "unknowns": [
    {
      "id": "U-001",
      "description": "Final financing terms",
      "importance": "CRITICAL",
      "status": "OPEN"
    }
  ],

  "conflicts": [],

  "financialModels": [
    {
      "modelId": "FM-005",
      "version": 3,
      "relationship": "PRIMARY_MODEL"
    }
  ],

  "professionalReview": {
    "status": "NOT_REVIEWED"
  }
}
```

This is **schema illustration only**, not implementation code.

---

# 51. The Most Important Structural Decision

I would lock one additional principle that isn't quite explicit enough in the original architecture:

> **A Scenario Version should be a coherent snapshot of assumptions and references, not merely a mutable container of links.**

That distinction will save the project considerable pain later.

Otherwise you eventually get this:

```text
Scenario A
   ↓
Business Reality changes
   ↓
Financial Model changes
   ↓
Research changes
   ↓
Scenario A mysteriously changes
```

Instead:

```text
Scenario A v1
 ├── Destination v2
 ├── Business Reality v5
 ├── Financial Model v3
 └── Research snapshot R7

             ↓

Scenario A v2
 ├── Destination v2
 ├── Business Reality v7
 ├── Financial Model v4
 └── Research snapshot R11
```

Now the system can answer the extremely important question:

> **"What did we actually know when we made this scenario?"**

---

# 52. Final Schema Boundary

The Scenario Engine's data model can therefore be reduced to this:

```text
                    ┌──────────────────┐
                    │ Scenario Family  │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │ Scenario Version │
                    └────────┬─────────┘
                             │
        ┌────────────┬───────┼────────┬────────────┐
        ▼            ▼       ▼        ▼            ▼
   Assumptions  Constraints Unknowns Conflicts Dependencies
        │
        ├──────────── Evidence
        ├──────────── Research
        ├──────────── Financial Model
        ├──────────── Financing
        ├──────────── Seller Note
        └──────────── Professional Review

                             │
                             ▼
                    Scenario Comparison
                             │
                             ▼
                       Owner Selection
                             │
                             ▼
                    Transaction Handoff
```

And the authoritative boundary remains:

> **Scenario owns the representation of a possible path. It does not own the truth of the inputs, the mathematics of the model, the professional determination, the owner's decision, or the transaction itself.**

That is the data-model version of the architecture's central rule: **Scenario is the bridge.**

### Recommended next step

The next specification I would create is **“Scenario Engine Relational Schema + Event Contract”**. That would take this conceptual model one level deeper and define the actual tables/entities, primary keys, foreign keys, cardinalities, indexes, immutable-version rules, and the exact payload structure for events such as `ScenarioCreated`, `ScenarioConflictDetected`, `ScenarioWhatIfCreated`, and `ScenarioAffectedByRealityChange`.
