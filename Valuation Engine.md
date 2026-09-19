# Valuation Engine

## 1. Purpose

The Valuation Engine represents what a business might be worth, under stated methods and stated assumptions, and makes clear when that representation stops being a platform estimate and starts requiring a qualified professional.

It handles:

* Valuation approaches
* Valuation methods
* Valuation assumptions
* Comparable and benchmark selection
* Adjustments, discounts, and premiums
* Valuation ranges
* Valuation reconciliation
* Valuation confidence
* Valuation versions
* Valuation readiness
* Valuation work-product references
* The professional-valuation requirement
* Valuation provenance

Its central question is:

> **“What might this business be worth, and on what basis do we say so?”**

The engine estimates.

It does not opine.

A platform valuation range is a **modeled estimate**. A valuation opinion is a **professional determination**. Those are different objects with different authorities, and the engine must never let one masquerade as the other.

---

# 2. Core Architectural Principle

## A preliminary estimate is not a valuation opinion.

The engine may determine:

> **Under these methods and these assumptions, the indicated value range is $8M–$12M.**

It may determine:

> **The income approach and the market approach currently diverge materially.**

It may determine:

> **This estimate rests on an owner-provided EBITDA figure that has not been verified.**

It must not independently conclude:

> **The business is worth $10M.**

That conclusion, when it is needed for a transaction, belongs to a qualified valuation professional. The engine's job is to prepare, bound, explain, and hand over — not to substitute.

---

# 3. Why This Engine Exists

The platform needs a place where value is represented consistently, without either of two failure modes.

**Failure mode one — the platform invents a number.** A business owner sees "$10M" in the interface and treats it as a fact. It is not a fact. It is an output of chosen methods and chosen assumptions, and it will move when those move.

**Failure mode two — the platform has no number at all.** Without any estimate, the owner cannot explore scenarios, cannot sense-check a desired price, and cannot tell whether a professional valuation is even warranted yet. That pushes every early question to a paid professional, which is exactly the friction the platform exists to remove.

The Valuation Engine exists to hold the middle ground:

> **Enough structure to explore. Enough discipline never to impersonate an appraiser.**

It also exists because three other engines need value as an input and none of them may own it:

* **Financial Modeling** needs a purchase price or value assumption to do arithmetic.
* **Scenario** needs to know which values a path assumes.
* **Capital / Financing** needs to know what is being financed.

Without a Valuation Engine, each of them would quietly grow its own notion of value. That is the anti-pattern this engine prevents.

---

# 4. What This Engine Owns

The engine owns:

* Valuation approaches and methods
* Valuation method selection
* Valuation assumptions
* Comparable selection
* Benchmark selection
* Adjustments
* Discounts and premiums
* Valuation ranges and point estimates
* Valuation reconciliation
* Method divergence
* Valuation confidence
* Valuation readiness and status
* Valuation versions and snapshots
* Valuation provenance
* Valuation freshness
* Valuation work-product references
* The requirement that professional valuation is needed
* Valuation-to-model references
* Valuation-to-scenario references

---

# 5. What This Engine Does NOT Own

It does not own:

* Business facts
* Financial calculations
* Professional valuation opinions
* Professional determinations of any kind
* Tax conclusions
* Legal conclusions
* Accounting conclusions
* Purchase-price decisions
* Negotiation
* Financing approval
* Scenario selection
* Owner decisions
* Transaction execution
* Platform-wide permissions
* Platform-wide audit history

Those belong to other engines. The engine supplies a **bounded estimate with stated provenance** and stops.

---

# 6. The Three Valuation Layers

These layers are architecturally distinct and must never collapse.

## Layer 1: Platform Valuation Estimate

The platform's own modeled estimate.

* Produced by this engine.
* Method-driven and assumption-driven.
* Always expressed with its assumptions visible.
* Always a range unless a point estimate is explicitly justified.
* Never authoritative for a transaction.
* Never described as an appraisal, an opinion, or a determination.

## Layer 2: Professional Valuation Determination

A qualified valuation professional's conclusion.

* Owned by the professional and recorded through Professional Review.
* Referenced by this engine, never authored by it.
* Carries the professional's identity, scope, standard applied, and date.
* Supersedes the platform estimate for any purpose requiring professional standing.

## Layer 3: Owner Price Decision

What the owner decides to ask, accept, or refuse.

* Owned by the owner and recorded through the Decision Record.
* May differ from both the estimate and the determination.
* The engine records the decision's existence and reference, not its wisdom.

Example:

> **Platform estimate:** $8M–$12M.
>
> **Professional determination:** $9.4M, income approach, as of June 30.
>
> **Owner decision:** proceed at $10.5M with seller financing, to preserve employee jobs.

All three are legitimate. They are not the same thing, and the platform must never average them.

---

# 7. Valuation Approaches

The engine supports three standard approaches.

```text
MARKET APPROACH
    What do comparable businesses transact for?

INCOME APPROACH
    What is the present value of expected economic benefit?

ASSET-BASED APPROACH
    What would the underlying assets be worth?

HYBRID / BLENDED
    A weighted combination of the above
```

The engine may apply more than one approach simultaneously.

Applying several approaches is not indecision. It is how a defensible range is built.

---

# 8. Market Approach

The market approach values the business relative to comparable transactions or comparable public companies.

It requires:

* A comparable set
* A selection rationale
* An applicable multiple or multiples
* The metric the multiple applies to
* An as-of date for each comparable
* A stated adjustment basis

Example:

```text
Comparable set: 7 transactions, employee-owned acquisitions, $5M–$20M revenue
Metric: EBITDA
Observed multiple range: 4.0x – 6.0x
Applied to: normalized EBITDA of $1.9M
Indicated range: $7.6M – $11.4M
```

The engine must state the comparable set. A multiple without its comparable set is not a valuation; it is an assertion.

---

# 9. Income Approach

The income approach values the business from the economic benefit it is expected to produce.

It requires:

* A benefit stream
* A projection basis
* A discount rate or capitalization rate
* The rate's derivation
* A terminal or continuing-value treatment

Example:

```text
Benefit stream: normalized free cash flow to equity
Projection basis: Business Reality v8, three-year history, conservative case
Discount rate: 22% (derivation recorded)
Indicated value: $9.1M
```

The rate is an assumption, and the engine must show how it was derived. An unexplained discount rate is the single most common way a valuation becomes unfalsifiable.

---

# 10. Asset-Based Approach

The asset-based approach values the business from its underlying assets.

It may be appropriate where:

* Earnings are volatile or minimal
* The business is asset-intensive
* The business is being assessed as a floor rather than a going concern
* A liquidation or orderly-disposal view is specifically requested

It requires:

* An asset inventory
* A valuation basis per asset class
* Treatment of liabilities
* Treatment of intangibles and goodwill

The engine should state plainly when the asset-based approach is a floor test rather than a value conclusion.

---

# 11. Approach Selection

The engine must record which approach was selected and why.

Example:

```text
Selected: income approach, primary
Selected: market approach, secondary
Excluded: asset-based

Reason for exclusion:
Business value derives from recurring contracted revenue
rather than asset base; asset-based result would understate
a going-concern value and is not presented.
```

An unexplained choice of method is a hidden assumption. The engine must not hide it.

---

# 12. Valuation Assumptions

Every material valuation input is an assumption, and every assumption is explicit.

Each assumption should include:

```text
assumption_id
valuation_id
category
name
value_type
value
unit
currency
source_type
source_reference
source_version
source_date
verification_status
provenance_status
owner_provided
professionally_reviewed
created_at
created_by
```

The engine must never present a valuation output without the assumption set that produced it.

---

# 13. Assumption Classes

Assumptions are typed, because they carry different authority.

```text
HISTORICAL_FACT
CURRENT_FACT
FORECAST_ASSUMPTION
MARKET_BENCHMARK
PLATFORM_ASSUMPTION
PROFESSIONAL_INPUT
OWNER_ASSUMPTION
```

An assumption is not promoted merely by being used.

Example:

> An owner's belief that EBITDA is $2M does not become a verified fact because the engine used it in a calculation.

The engine preserves the class through every downstream reference.

---

# 14. Comparable Selection

Comparables must be selected, recorded, and justified.

Each comparable should include:

```text
comparable_id
valuation_id
comparable_type
source
source_reference
as_of_date
industry
geography
size_metric
size_value
transaction_or_market
multiple_basis
multiple_value
inclusion_status
exclusion_reason
```

The engine must record **excluded** comparables and why. A comparable set that only shows what was included cannot be reviewed.

---

# 15. Benchmark Data

Benchmarks are external reference points, not facts about this business.

They must retain:

* Source
* Publication date
* Retrieval date
* Applicable date
* Geography
* Industry definition
* Population and sample size where available
* Scope and applicability
* Freshness status

Benchmarks come from Research and the Evidence Ledger. The Valuation Engine references them rather than duplicating them.

---

# 16. Adjustments

Raw comparable or benchmark data rarely applies directly. The engine records each adjustment.

Example:

```text
Base multiple: 5.5x
Adjustment: -0.5x  (customer concentration)
Adjustment: +0.3x  (recurring contracted revenue)
Adjustment: -0.2x  (key-person dependency)
Adjusted multiple: 5.1x
```

Each adjustment requires:

* A direction
* A magnitude
* A stated basis
* A source or rationale
* An author

Unrecorded adjustments are indistinguishable from manipulation.

---

# 17. Discounts and Premiums

Discounts and premiums are adjustments with outsized effect and outsized potential for abuse.

Common categories:

```text
MARKETABILITY
CONTROL
MINORITY_INTEREST
KEY_PERSON
CUSTOMER_CONCENTRATION
TRANSFER_RESTRICTION
SYNERGY
```

The engine must:

* Record the category
* Record the magnitude
* Record the derivation
* Never apply a discount or premium silently
* Never allow a discount to be the difference between a desired answer and an indicated one

Where a discount or premium is materially judgmental, the engine must flag it as requiring professional review.

---

# 18. Valuation Range

A range is the engine's default output.

A range should include:

```text
range_id
valuation_id
low_value
mid_value
high_value
currency
range_basis
range_width_rationale
```

The engine must state why the range is as wide or as narrow as it is.

A narrow range is not automatically better. It may simply mean fewer risks were modeled.

---

# 19. Point Estimate vs Range

A point estimate is permitted only where it is justified.

Justification may include:

* A single method with a tight comparable set
* A contractual or formula-driven price
* A professional determination being referenced
* A specific modeled case explicitly requested by the owner

The engine must never:

* Present a point estimate without saying it is a point estimate
* Imply precision the underlying inputs do not support
* Collapse a range into its midpoint and present that as the value

Example of what the engine must not do:

```text
Range: $8M – $12M
Displayed as: $10M
```

---

# 20. Valuation Reconciliation

When several methods are applied, the engine reconciles them.

Reconciliation is a recorded act, not an average.

It should capture:

```text
reconciliation_id
valuation_id
methods_considered
method_weights
reconciliation_basis
divergence_notes
resulting_range
```

The engine must state how weights were chosen. A weighted average whose weights are unexplained is not a reconciliation.

---

# 21. Method Divergence

Divergence is a finding, not a problem to be smoothed away.

Example:

> **Income approach indicates $9.1M.**
>
> **Market approach indicates $15.5M.**
>
> **Divergence: 70% of the lower indication.**

The engine must display:

* Both indications
* The magnitude of divergence
* The likely causes
* Whether the divergence is expected given the business's characteristics
* Whether professional review is required

The engine must not:

* Average the methods to make the divergence disappear
* Discard the inconvenient method without recording why
* Present the blended figure while hiding the inputs

Material divergence is one of the clearest signals that a professional is needed. The engine should treat it as such.

---

# 22. Valuation Confidence

Confidence describes the estimate's foundation, not the business's quality.

The engine may expose independent signals:

* Information completeness
* Fact verification status of inputs
* Comparable set quality and size
* Benchmark freshness
* Method agreement
* Assumption sensitivity
* Professional review status
* Outstanding unknowns

The engine must not produce:

* A single "confidence score" that implies predictive accuracy
* A probability that the valuation is correct
* A statement that the business will sell for the indicated value

Confidence is a property of the estimate. Value is a property of the business. They are not the same axis.

---

# 23. Valuation Readiness

Use **Valuation Readiness**, not "valuation accuracy."

Possible states:

```text
NOT_STARTED
INFORMATION_NEEDED
PRELIMINARY
INDICATIVE
RECONCILED
READY_FOR_PROFESSIONAL_REVIEW
UNDER_PROFESSIONAL_REVIEW
PROFESSIONALLY_DETERMINED
SUPERSEDED
ARCHIVED
```

A valuation can be:

> Indicative

while remaining:

> Professionally unvalidated.

Those are not contradictory.

---

# 24. Lifecycle

Recommended lifecycle:

```text
Requested
  ↓
Information Needed
  ↓
Preliminary
  ↓
Indicative
  ↓
Reconciled
  ↓
Ready for Professional Review
  ↓
Under Professional Review
  ↓
Professionally Determined
  ↓
Referenced by Modeling and Scenario
  ↓
Superseded
  ↓
Historical
```

A superseded valuation remains accessible. It is the record of what was believed at the time.

---

# 25. Valuation Versioning

Every material change creates a new version.

A version contains:

* Version number
* Parent version
* Changed fields
* Change reason
* Changed-by identity
* Timestamp
* Business Reality version referenced
* Assumption set
* Comparable set
* Method selection
* Resulting range
* Professional feedback available at that point

Historical versions are immutable.

---

# 26. Valuation Snapshot

A snapshot makes a valuation reproducible.

Minimum snapshot:

```text
Business Reality Version
Financial Model Version(s) referenced
Research References
Evidence References
Benchmark Set
Comparable Set
Assumption Set
Method Selection
Reconciliation Basis
Professional Feedback References
```

A snapshot records the authoritative references required to reproduce the valuation. It does not duplicate the underlying systems.

---

# 27. Reproducibility

A historical valuation must remain understandable after the business changes.

Example:

```text
Valuation v2
created using:
  Business Reality v5
  Research snapshot R12
  Benchmark set B4
  Comparable set C7
  Assumptions A1–A19
  Methods: income (primary), market (secondary)
```

Later changes do not mutate the historical record.

The system must be able to answer:

> **“What did we think it was worth then, and why?”**

---

# 28. Source Provenance

Provenance should distinguish how a valuation input entered the system.

```text
OWNER_REPORTED
DOCUMENT_SUPPORTED
SYSTEM_DERIVED
RESEARCH_SUPPORTED
BENCHMARK_DERIVED
PROFESSIONALLY_SUPPLIED
PROFESSIONALLY_DETERMINED
VALUATION_ASSUMED
MODEL_DERIVED
```

These statuses must not be collapsed into a generic confidence number.

---

# 29. Data Quality Dependencies

A valuation is only as good as the facts beneath it.

The engine must surface:

* Which inputs are unverified
* Which inputs depend on conflicting sources
* Which inputs are stale
* Which inputs are owner-reported without support
* Which inputs rest on a single source
* Which inputs are outside their applicable period

Example:

> **This estimate depends on EBITDA of $1.9M, which is owner-reported and unverified. Two source documents disagree on revenue.**

The engine presents the estimate and the weakness together. Never the estimate alone.

---

# 30. Stale Valuation

A valuation becomes stale when its inputs have moved.

Triggers include:

* Business Reality changed
* A financial period closed
* Research or benchmarks aged beyond their window
* A comparable set aged out
* Financing structure changed materially
* The business's circumstances changed materially

Example:

> **This valuation was prepared against Business Reality v5. The current version is v8.**

The engine must never silently recalculate a historical valuation.

---

# 31. Valuation Freshness

Every valuation view should show its information vintage.

Example:

```text
Valuation v2
As of: June 30, 2026
Business Reality: v5
Benchmarks: retrieved March 2026
Comparables: transactions through Q4 2025
Research: reviewed September 12, 2026
Professional review: not yet performed
```

This prevents a stale estimate from appearing current.

---

# 32. Valuation Refresh

The system may offer:

> **Refresh valuation using current information**

Refreshing creates a new version.

Example:

```text
Valuation v2
    ↓
Business Reality v8 introduced
    ↓
Refresh requested
    ↓
Valuation v3
```

The historical v2 remains unchanged.

---

# 33. Business Reality Integration

Business Reality owns the facts. The Valuation Engine consumes them.

Business Reality provides:

* Financial history
* Normalized earnings inputs
* Balance sheet information
* Ownership facts
* Operational facts
* Business characteristics
* Verification status per fact

The Valuation Engine must preserve the verification status of every fact it depends on.

A valuation must never silently convert:

> Unverified business fact

into:

> Verified valuation input.

---

# 34. Financial Modeling Integration

Financial Modeling owns arithmetic. The Valuation Engine owns the value that arithmetic is performed on.

The separation is:

> **Valuation asks: what might this business be worth?**
>
> **Financial Modeling asks: given that value, what do the transaction numbers look like?**

The engine must never duplicate financial formulas merely to display a modeled outcome, and Financial Modeling must never derive a business value on its own.

---

# 35. The Valuation → Modeling Contract

Financial Modeling requests a value. The Valuation Engine returns a **referenced valuation version**, never a bare number.

```text
FINANCIAL MODELING
        │
        │  requests: value assumption
        ▼
VALUATION ENGINE
        │
        │  returns:
        │    valuation_id
        │    valuation_version_id
        │    value_range (low / mid / high)
        │    currency
        │    method_summary
        │    readiness_status
        │    confidence_signals
        │    professional_determination_reference (if any)
        ▼
FINANCIAL MODELING
        │
        │  performs arithmetic
        ▼
PURCHASE PRICE / PROCEEDS / DEBT SERVICE / CAPITAL GAP
```

A model must reference the valuation version it used, not a copied number.

Correct:

```text
purchase_price_basis:
  valuation_id: V-001
  valuation_version: 3
  value_used: mid
```

Incorrect:

```text
purchase_price: 10000000
```

The incorrect form cannot be reproduced, audited, or traced when the valuation changes.

In the platform's flow, the value reference normally arrives at Financial Modeling *through the Scenario* — the scenario is what assumes a value (section 37). The contract above is the same either way: however the request is triggered, Financial Modeling resolves it against the Valuation Engine and receives a referenced version rather than a number. Financial Modeling never determines value on its own.

---

# 36. The Modeling → Valuation Contract

The dependency is not one-directional. Financial Modeling's outputs inform whether an indicated value is supportable.

Financial Modeling returns:

* Debt-service requirements at the indicated value
* Debt-service coverage ratios
* Seller-note burden
* Capital gap
* Affordability of seller financing at the indicated value

The Valuation Engine treats these as **evidence about supportability**, not as a recalculation.

Example:

> **At an indicated value of $15.5M, the modeled debt service exceeds the business's historical coverage capacity. The market-approach indication may not be financeable at that level.**

The engine records this as a valuation consideration. It does not adjust the value to make the financing work.

That distinction is essential. A valuation that bends to fit the financing is no longer a valuation.

---

# 37. Scenario Integration

Scenario owns possible paths. Valuation supplies the value a path assumes.

A scenario may reference:

```text
valuation_id
valuation_version_id
value_used (low / mid / high)
```

The Scenario Engine may branch on value:

```text
Scenario A
├── Base Case
├── $8M valuation branch
└── $12M valuation branch
```

The Valuation Engine supplies the bounds for those branches and records which version each branch used.

Changing the valuation does not rewrite historical scenarios. It makes them identifiable as affected.

---

# 38. Capital / Financing Integration

Capital / Financing owns funding structures. Valuation supplies the amount being financed.

```text
Valuation
    ↓
Indicated value
    ↓
Capital / Financing
    ↓
Financing requirement, structure, terms
```

A financing structure built on a valuation range must state which point in the range it assumes. Financing against the high end of a range is a materially different proposition from financing against the low end, and the platform should make that visible.

The Valuation Engine does not approve financing, and Capital / Financing does not determine value.

---

# 39. Seller-Note Integration

The Seller-Note Engine owns note economics and lifecycle. Valuation supplies the consideration the note is part of.

Where a seller note is contemplated, the engine may be asked to consider:

* Whether the indicated value is supportable by the business's cash flow under note terms
* How a note structure affects the effective consideration
* Whether a note-based structure changes the indicated range's applicability

The engine records these as valuation considerations. It does not own the note, its schedule, or its servicing.

---

# 40. Research and Evidence Integration

Research and the Evidence Ledger supply the external basis for a valuation.

They provide:

* Comparable transactions
* Industry multiples
* Market conditions
* Financing environment
* Regulatory considerations
* Supporting evidence
* Contradictory evidence

Research must retain provenance. A valuation assumption may therefore be displayed as:

> **Research-supported assumption**

or:

> **Research-challenged assumption**

Research never automatically becomes fact, and a benchmark never automatically becomes this business's value.

---

# 41. Professional Review Integration

Professional Review owns the assignment and the determination.

The Valuation Engine:

* Signals when professional valuation is required
* Supplies the valuation package for review
* Stores references to the professional's feedback
* Preserves professional feedback as attributed external input
* Records when a professional determination supersedes a platform estimate

It must distinguish:

```text
Platform Estimate
Professional Comment
Professional Requirement
Professional Determination
Owner Decision
```

These must never collapse into one field.

---

# 42. Professional Determination Boundary

Suppose a valuation professional determines:

> **$9.4M, income approach, as of June 30.**

The Valuation Engine records:

* The determination exists
* The professional's identity reference
* The scope and standard applied
* The effective date
* The reference to the authoritative record

It must not transform that into:

```text
platform_valuation_status = APPROVED
```

unless the platform explicitly defines an authoritative approval state owned by the professional workflow.

The engine may say:

> Professional valuation determination exists.

It may not say:

> The platform has concluded the business is worth $9.4M.

---

# 43. When Professional Valuation Is Required

The platform must determine where professional valuation becomes mandatory or strongly advisable, and this depends on transaction structure and jurisdiction.

The engine should flag a professional requirement when any of the following hold:

* The transaction structure requires a valuation opinion
* A jurisdiction requires it
* An ESOP or trustee context requires it
* A lender requires it
* A fiduciary obligation requires it
* The purpose is a formal filing or disclosure
* The indicated range is materially divergent between methods
* The inputs are predominantly unverified
* A party to the transaction requires an independent value
* The owner is relying on the value to make an irreversible decision

A flag is a routing decision, not a conclusion. The engine stops and routes; it does not impersonate the professional.

---

# 44. Valuation Professional Role

Valuation is a professional domain, and the engine must treat it as one.

The engine should reference:

* Professional identity reference
* Role and credential
* Assignment scope
* Standard or framework applied
* Effective date
* Engagement reference
* Determination reference

The platform must never:

* Simulate a valuation professional's judgment
* Attribute a platform estimate to a professional
* Present a platform estimate in a professional's format or voice
* Imply professional review that has not occurred

---

# 45. Decision Record Integration

Valuation:

> **Indicated range: $8M–$12M.**

Decision Record:

> Owner chose to proceed at $10.5M because preserving employee jobs matters more than maximizing immediate cash.

The Valuation Engine stores the valuation.

The Decision Record Engine stores the owner's reasoning and decision.

This separation is mandatory. A price decision is not a valuation, and the platform must never present it as one.

---

# 46. Transaction / Orchestration Integration

Once a transaction is underway, Transaction / Orchestration tracks valuation as a milestone it does not perform.

```text
Business Information Verified
        ↓
Valuation
        ↓
Financing
        ↓
Definitive Documents
        ↓
Closing
```

Transaction / Orchestration needs to know:

* That valuation is required
* Who owns it
* What it depends on
* Whether it is blocking progress
* Whether the required valuation is complete

It does not perform the valuation. The Valuation Engine supplies the work product and status; the professional supplies the determination.

---

# 47. Policy / Compliance Integration

Policy may say:

> A valuation determination by a qualified professional is required before this action.

Valuation provides:

> Professional valuation determination: present, reference V-001-PRO.

Policy evaluates:

> Requirement satisfied.

The action proceeds.

The Valuation Engine does not make the policy decision, and Policy does not determine value.

---

# 48. Confidence Integration

The Valuation Engine supplies factual inputs:

* Information completeness
* Input verification status
* Comparable set quality
* Benchmark freshness
* Method agreement
* Outstanding unknowns
* Professional review status

The Confidence / Goal Alignment Engine determines the resulting confidence signal.

The Valuation Engine must not create:

* Probability that the business will sell at the indicated value
* Probability that the valuation is correct
* A universal valuation quality score

---

# 49. Audit Integration

Platform-wide Audit / Provenance owns the authoritative audit trail.

Every material valuation change should produce an auditable record containing:

```text
actor
timestamp
valuation_id
valuation_version
field_changed
previous_value
new_value
reason
source
```

The Valuation Engine emits the event. Audit infrastructure records it.

---

# 50. Local-First Valuation

Valuation inputs are among the most sensitive data in the platform.

The engine must operate with locally available data when possible.

Potential local inputs:

* Business Reality
* Financial statements
* Normalization adjustments
* Comparable sets
* Assumption sets
* Valuation history

Sensitive information should not automatically leave the local environment. Only explicitly authorized information should be shared externally.

---

# 51. Research Privacy

Before external research or benchmark retrieval:

* Minimize identifying information
* Remove unnecessary sensitive detail
* Use only the information necessary to the query
* Record what information was used
* Preserve research provenance

The engine must not claim perfect anonymization. A comparable search on a distinctive business profile can be identifying even when names are removed.

---

# 52. Security and Permissions

The Valuation Engine integrates with:

* Identity & Access
* Consent & Access
* Policy / Compliance
* Local Vault
* Professional Review Package
* Audit / Provenance

It must not create a parallel permission system.

Valuation records may contain:

* Financial information
* Ownership information
* Earnings detail
* Comparable and benchmark data
* Professional commentary
* Strategic intent

Access must be inherited from platform-level authorization.

---

# 53. Core Data Objects

The engine should use a set of related objects rather than one enormous Valuation record.

Core objects:

1. Valuation
2. Valuation Version
3. Valuation Snapshot
4. Valuation Approach
5. Valuation Method Selection
6. Valuation Assumption
7. Comparable
8. Benchmark Reference
9. Adjustment
10. Discount / Premium
11. Valuation Range
12. Valuation Reconciliation
13. Method Divergence
14. Valuation Confidence Signal
15. Valuation Status History
16. Professional Valuation Reference
17. Valuation Model Reference
18. Valuation Scenario Reference

---

# 54. Valuation Object

Conceptual structure:

```text
Valuation
├── valuation_id
├── business_id
├── valuation_name
├── valuation_purpose
├── valuation_basis
├── business_reality_reference
├── approaches
├── method_selections
├── assumptions
├── comparables
├── benchmarks
├── adjustments
├── discounts_premiums
├── ranges
├── reconciliation
├── divergences
├── confidence_signals
├── professional_references
├── model_references
├── scenario_references
├── readiness_status
├── lifecycle_status
├── created_at
├── updated_at
└── current_version_id
```

---

# 55. Valuation Approach Object

```text
ValuationApproach
-------------------------
approach_id
valuation_version_id

approach_type
selection_status
selection_rationale

metric_basis
multiple_basis
rate_basis

indicated_low
indicated_high
currency

created_at
created_by
```

---

# 56. Valuation Assumption Object

```text
ValuationAssumption
-------------------------
assumption_id
valuation_version_id

category
name
description

value_type
value
unit
currency
range_min
range_max

assumption_class
source_type
source_reference
source_version
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

# 57. Comparable Object

```text
Comparable
-------------------------
comparable_id
valuation_version_id

comparable_type
source
source_reference
as_of_date

industry
geography
size_metric
size_value

multiple_basis
multiple_value

inclusion_status
exclusion_reason

created_at
created_by
```

---

# 58. Adjustment Object

```text
Adjustment
-------------------------
adjustment_id
valuation_version_id

adjustment_type
category

direction
magnitude
unit

basis
source_reference

requires_professional_review

created_at
created_by
```

---

# 59. Valuation Result Object

```text
ValuationResult
-------------------------
result_id
valuation_version_id

result_type
result_basis

low_value
mid_value
high_value
currency

range_width_rationale

method_attribution

created_at
```

---

# 60. Valuation Reconciliation Object

```text
ValuationReconciliation
-------------------------
reconciliation_id
valuation_version_id

methods_considered
method_weights
reconciliation_basis

divergence_present
divergence_magnitude
divergence_causes

resulting_range_id

requires_professional_review

created_at
created_by
```

---

# 61. Valuation Professional Reference

```text
ValuationProfessionalReference
-------------------------
reference_id
valuation_version_id

professional_review_id
professional_identity_reference
professional_role
credential_reference

assignment_scope
standard_applied
effective_date

determination_reference
determination_status

created_at
```

The professional's authoritative record remains owned by Professional Review.

---

# 62. API / Engine Contract

Conceptual contract:

```text
createValuation()
cloneValuation()
createValuationVersion()
getValuation()
getValuationVersion()

selectApproach()
selectMethod()
setMethodWeight()

addAssumption()
updateAssumption()
removeAssumption()

addComparable()
excludeComparable()

addBenchmark()
addAdjustment()
addDiscountOrPremium()

calculateRange()
reconcileMethods()
getDivergence()

getValuationConfidence()
getValuationReadiness()
getValuationExplanation()
getValuationSnapshot()

requestValuationRefresh()
requestProfessionalValuation()
recordProfessionalValuationReference()

getValuationSupportability()
supersedeValuation()
archiveValuation()

getValuationHistory()
```

Exact implementation contracts should be finalized during technical design.

---

# 63. Events

Recommended events:

```text
ValuationCreated
ValuationVersionCreated
ValuationAssumptionAdded
ValuationAssumptionChanged
ValuationComparableAdded
ValuationComparableExcluded
ValuationBenchmarkAttached
ValuationAdjustmentApplied
ValuationRangeCalculated
ValuationMethodsReconciled
ValuationDivergenceDetected
ValuationReadinessChanged
ValuationProfessionalReviewRequested
ValuationProfessionalDeterminationReceived
ValuationRefreshRequested
ValuationSuperseded
ValuationAffectedByRealityChange
ValuationAffectedByBenchmarkChange
ValuationReferencedByModel
ValuationReferencedByScenario
ValuationSupportabilityConcernRaised
```

Events may trigger:

* scenario impact analysis
* financial model recalculation
* professional review
* confidence recalculation
* workflow
* notifications
* audit

The Valuation Engine does not directly orchestrate those downstream systems.

---

# 64. Example: Preliminary Range Before Professionals

The owner wants to know roughly what the business might be worth before paying for an appraisal.

```text
Business Reality v8: revenue $14M, normalized EBITDA $1.9M, owner-reported
Research: 7 comparable employee-owned acquisitions, 4.0x–6.0x EBITDA

Market approach:  $7.6M – $11.4M
Income approach:  $9.1M (discount rate 22%, derivation recorded)

Reconciled range: $8M – $12M
Readiness: Indicative
Confidence signals: inputs unverified; comparable set small but consistent
Professional valuation required: not yet — sufficient for exploration
```

The platform has enabled exploration without impersonating an appraiser.

---

# 65. Example: Modeling Consumes a Valuation

Financial Modeling needs a value to compute seller proceeds.

```text
Valuation v3
  range: $8M – $12M
  readiness: Indicative

Financial Modeling
  purchase_price_basis:
    valuation_id: V-001
    valuation_version: 3
    value_used: mid

  computes:
    sources and uses
    seller proceeds
    debt service
    capital gap
```

If the valuation changes to v4, the model identifies itself as affected and offers a refresh. It does not silently recalculate.

---

# 66. Example: Divergent Methods

```text
Income approach: $9.1M
Market approach: $15.5M
Divergence: 70% of the lower indication

Causes identified:
  - comparable set drawn from larger, less owner-dependent businesses
  - income approach applies a key-person discount absent from comparables
  - buyer market currently competitive

Engine response:
  - both indications displayed
  - no averaging
  - professional review required: yes
```

The divergence is the finding. The engine's job is to make it visible, not to resolve it.

---

# 67. Example: Professional Valuation Supersedes a Platform Estimate

```text
Valuation v3 (platform estimate)
  range: $8M – $12M
  readiness: Indicative

        ↓  professional engagement

Professional determination
  $9.4M, income approach, as of June 30
  standard applied: recorded
  effective date: June 30

Valuation v4
  professional_reference: present
  readiness: Professionally Determined
  platform estimate retained as a historical version
```

The platform estimate is not deleted. It is the record of what the platform indicated before the professional spoke — and comparing the two is itself informative.

---

# 68. Example: Valuation Change Ripples Into Scenarios

```text
Valuation v3 → v4
  range changed: $8M–$12M → $7M–$9.4M

Engine identifies affected objects:
  3 scenarios reference v3
  2 financial models reference v3
  1 financing request references v3

Displayed:
  **Valuation changed. 6 objects may be affected.**

Offered:
  Refresh using current valuation
  (creates new versions; historical versions unchanged)
```

The engine never silently recalculates historical scenarios.

---

# 69. Valuation Anti-Patterns

The following are prohibited.

## 69.1 Platform Opinion

Do not state or imply that the platform has determined what the business is worth.

## 69.2 Midpoint Collapse

Do not display a range's midpoint as though it were the value.

## 69.3 Silent Averaging

Do not average divergent methods to produce a tidier number.

## 69.4 Convenience Adjustment

Do not adjust a valuation to reach a desired price, financing amount, or owner expectation.

## 69.5 Discount Abuse

Do not use an unrecorded or unjustified discount or premium to move the indication.

## 69.6 Unexplained Multiple

Do not present a multiple without its comparable set, metric, and as-of date.

## 69.7 Unexplained Rate

Do not present a discount or capitalization rate without its derivation.

## 69.8 Hidden Assumption

Do not produce a valuation output without the assumption set that generated it.

## 69.9 Verification Promotion

Never convert an owner-provided estimate into a verified fact through use.

## 69.10 Historical Mutation

Never allow current Business Reality to silently rewrite a historical valuation.

## 69.11 Professional Impersonation

Do not generate text that reads as a valuation opinion, appraisal, or professional conclusion.

## 69.12 False Precision

Do not present a value to a precision the inputs do not support.

## 69.13 Financing Feedback

Do not adjust value because the financing works better at that level.

---

# 70. What the Engine Should Never Do

It should never:

* Claim to be an appraisal or a valuation opinion
* Attribute a platform estimate to a professional
* Present a valuation without its assumptions
* Present a range without its basis
* Present a multiple without its comparables
* Present a rate without its derivation
* Resolve method divergence by averaging
* Adjust value to suit financing, price expectation, or tax outcome
* Promote an assumption's authority through reuse
* Silently recalculate historical valuations
* Substitute for professional valuation where one is required
* Determine tax, legal, or accounting consequences
* Decide the purchase price
* Approve financing
* Recommend a course of action
* Create its own permission model
* Replace Policy / Compliance, Consent & Access, or Professional Review

---

# 71. Observability

The engine should expose operational telemetry for:

* Valuation creation
* Version creation
* Assumption changes
* Comparable additions and exclusions
* Divergence detection frequency
* Professional review requests
* Refresh frequency
* Stale valuations
* Broken references
* Supportability concerns raised

Operational telemetry must not become a substitute for business-domain audit history.

---

# 72. Performance Considerations

The engine should favor references and immutable snapshots over duplicating entire external datasets.

Large objects such as:

* Financial statements
* Comparable databases
* Benchmark corpora
* Research findings

remain owned by their respective systems.

Valuation stores the minimum references necessary to reproduce context.

---

# 73. Extensibility

The engine should allow future valuation methods and structures without architectural changes.

Possible future additions:

* Industry-specific valuation frameworks
* Jurisdiction-specific standards
* Formula-driven or contractual valuation bases
* ESOP-specific valuation conventions
* Multi-entity and consolidated valuations
* Partial-interest and minority-stake valuations
* Intangible and IP-specific valuations
* Machine-assisted comparable selection

The method taxonomy must remain configuration-driven rather than hard-coded.

---

# 74. Architectural Lock

The following statements are architectural invariants.

> **Business Reality says what the business's facts are.**

> **Valuation says what the business might be worth, under stated methods and assumptions.**

> **Financial Modeling calculates the numbers inside a transaction given that value.**

> **Capital / Financing describes how the value might be funded.**

> **Seller-Note Liquidity handles note economics and lifecycle.**

> **Professionals determine what the business is actually worth when a determination is required.**

> **Decision Records preserve what the owner decides to pay or accept.**

> **Transaction / Orchestration executes the chosen path.**

> **Audit / Provenance records what happened.**

This separation is mandatory.

Additional invariants:

**1. A platform estimate is never a valuation opinion.**

**2. A valuation range is the default output; a point estimate requires justification.**

**3. Every valuation output is traceable to its assumption set.**

**4. Every multiple is traceable to its comparable set.**

**5. Every rate is traceable to its derivation.**

**6. Method divergence is displayed, never averaged away.**

**7. A valuation is never adjusted to fit financing, price, or tax expectations.**

**8. Professional determinations supersede platform estimates without deleting them.**

**9. Historical valuations are immutable and reproducible.**

**10. Valuation is a decision-layer engine and never executes a transaction.**

---

# 75. Architectural Boundary Summary

| Engine                     | Owns                                                                                 | Does Not Own                                   |
| -------------------------- | ------------------------------------------------------------------------------------ | ---------------------------------------------- |
| **Valuation**              | Approaches, assumptions, comparables, adjustments, ranges, reconciliation, versions   | Professional valuation opinions                |
| **Business Reality**       | Actual/current business facts and their verification status                           | Valuation methods or indications               |
| **Financial Modeling**     | Calculations, cash flows, debt schedules, proceeds, sensitivities                     | Determining what the business is worth         |
| **Scenario**               | Potential transaction paths and which value each assumes                              | Determining the value itself                   |
| **Capital / Financing**    | Financing process, providers, structures, commitments                                 | Value determination or debt arithmetic         |
| **Seller-Note Liquidity**  | Actual seller note, servicing, liquidity                                              | Valuation or affordability arithmetic          |
| **Research**               | External evidence, benchmarks, market data                                            | Applying benchmarks to this business           |
| **Evidence Ledger**        | Provenance of external evidence                                                       | Valuation conclusions                          |
| **Professional Review**    | Professional assignments and determinations, including valuation opinions             | Valuation methodology infrastructure           |
| **Decision Record**        | Owner's price decision and rationale                                                  | Valuation or professional determination        |
| **Confidence**             | Goal-alignment and confidence signal                                                  | Valuation indications                          |
| **Transaction / Orchestration** | Transaction execution plan, milestones, dependencies                             | Performing valuation work                      |
| **Policy / Compliance**    | Rules requiring a valuation determination                                             | Determining value                              |
| **Audit / Provenance**     | Historical record of valuation activity                                               | Valuation logic itself                         |
| **Local Vault**            | Private source files and local workspace                                              | Valuation semantics                            |
| **Consent & Access**       | Who may access valuation inputs and outputs                                           | The valuation itself                           |

## Hard Boundary

Valuation tells us what the business might be worth under stated methods and assumptions. It does not tell the owner what the business is worth, what they should pay, or whether the transaction is advisable. Those are the professional's determination and the owner's decision.

---

# 76. The Engine's Place in the Platform

The simplest way to remember the architecture is:

```text
BUSINESS REALITY
     │
     │  What are the facts?
     ▼
VALUATION
     │
     │  What might it be worth?
     │
     ├───────────────┐
     ▼               ▼
EVIDENCE         RESEARCH
/BENCHMARKS      /COMPARABLES
     │               │
     └───────┬───────┘
             ▼
         SCENARIO
             │
             │  Which paths, assuming which value?
             ▼
    FINANCIAL MODELING
             │
             │  What do the numbers look like?
             │
             └──────► returns affordability and
                      debt-service evidence to VALUATION
                      as supportability information
                      (never as a reason to adjust value)
             │
             ▼
   PROFESSIONAL REVIEW
             │
             │  What is it actually worth,
             │  and should this proceed?
             ▼
      OWNER DECISION
             │
             ▼
        TRANSACTION
             │
             │  Execute
             ▼
          OUTCOME
```

This placement is deliberate and consistent with the platform's authoritative
flow diagram in [Complete Architecture](Complete%20Architecture%20-%20Decision%20Engines,%20Transaction%20Engines,%20Platform%20Infrastructure.md),
which runs Confidence → Scenario → Financial Modeling. Valuation feeds the
**Scenario**, because a scenario is what assumes a value; Financial Modeling
then computes under that scenario's value and assumptions.

**The Valuation Engine bounds the question. It does not answer it.**
