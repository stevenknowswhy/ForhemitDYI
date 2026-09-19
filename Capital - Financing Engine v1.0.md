# Capital / Financing Engine v1.0

## 1. Purpose

The Capital / Financing Engine is a standalone engine responsible for organizing and coordinating the capital required to support a potential ownership transition.

It helps answer:

> **What capital may be needed?**

> **What sources might provide it?**

> **How could those sources potentially fit together?**

> **What information does each capital provider need?**

> **Where are the financing gaps?**

> **What financing options should the owner explore with qualified professionals and capital providers?**

The engine does not make final financing determinations.

---

# 2. CORE SEPARATION OF RESPONSIBILITIES

The Capital Engine must remain separate from other engines.

### Destination Engine

**What does the owner want?**

### Business Reality Engine

**What does the business look like today?**

### Scenario Engine

**What transaction structures could potentially connect those two?**

### Financial Modeling Engine

**What do the numbers look like under each set of assumptions?**

### Capital Engine

**Where might the required capital come from, and how do we coordinate it?**

### Marketplace Engine

**Which capital providers may be relevant?**

### Professional Review Engine

**What do the qualified professionals conclude?**

No engine should absorb another engine's responsibilities.

---

# 3. CORE CAPITAL OBJECT

The engine should create:

## `CapitalPlan`

This represents the potential capital stack associated with a transaction or scenario.

It should contain:

* Capital plan ID
* Transaction ID
* Scenario ID
* Destination version
* Version
* Total estimated funding need
* Sources of capital
* Uses of capital
* Financing requirements
* Capital gaps
* Capital-provider matches
* Financing requests
* Indicative terms
* Professional review status
* Version history

---

# 4. CAPITAL STACK

The engine should represent the transaction as a potential capital stack.

Example:

```text id="v6xzih"
PURCHASE / TRANSACTION FUNDING

$5.0M Total Purchase Price
│
├── Senior Debt          $2.5M
├── Seller Note          $1.5M
├── Employee Equity      $0.5M
└── Other Capital        $0.5M
```

This is an illustrative structure only.

The Capital Engine should not determine that this is the correct structure.

It records and evaluates possible capital combinations.

---

# 5. SOURCES OF CAPITAL

The engine should support multiple capital types.

## Debt

* SBA-related financing
* Bank loans
* Credit union loans
* Acquisition loans
* Specialty lenders
* Private credit
* Seller financing
* Other debt instruments

## Equity / Ownership Capital

* Employee contributions
* Management contributions
* Existing-owner rollover where applicable
* Outside equity
* Other permitted equity sources

## Other

* Earn-outs
* Deferred consideration
* Grants or incentives where legitimately applicable
* Other transaction-specific funding

The engine should be extensible.

---

# 6. SELLER FINANCING

Seller financing should be modeled as one capital source.

The Capital Engine should capture:

* Principal
* Interest
* Term
* Amortization
* Payment frequency
* Balloon
* Security
* Priority
* Guarantees
* Subordination
* Payment conditions
* Prepayment terms where known

The detailed seller-note lifecycle belongs to the separate:

## Seller-Note Liquidity Engine

The Capital Engine simply treats the seller note as a possible source of transaction capital.

---

# 7. USES OF CAPITAL

The engine should maintain a structured view of where capital may be needed.

Potential uses include:

* Purchase price
* Refinancing existing debt
* Transaction expenses
* Professional fees
* Working capital
* Required reserves
* Capital expenditures
* Other approved transaction uses

The specific uses should be scenario-specific.

---

# 8. SOURCE / USE RECONCILIATION

Every Capital Plan should reconcile:

### Total Uses

versus:

### Total Sources

Example:

**Uses:** $5.6M

**Sources:** $5.0M

### Capital Gap

**$600K**

The platform should clearly show:

> **Current financing assumptions leave an estimated $600K funding gap.**

It should not automatically invent another source to close the gap.

---

# 9. FINANCING GAP OBJECT

Create a standalone:

## `CapitalGap`

Containing:

* Gap amount
* Currency
* Related source/use
* Related scenario
* Cause
* Priority
* Possible funding categories
* Status
* Professional review
* Owner response

Example:

> **Capital Gap: $400K**
>
> Current scenario requires more closing capital than identified sources provide.

Possible next steps:

**Explore more debt**

**Explore additional seller financing**

**Explore additional equity**

**Adjust scenario**

**Discuss with professional**

---

# 10. OWNER FINANCING PREFERENCES

The Capital Engine should also capture the owner's financing preferences.

For example:

### Would you prefer to minimize debt?

**Very important**

**Important**

**Flexible**

### Would you consider seller financing?

**Yes**

**Possibly**

**No**

**I'm not sure**

### Would you consider outside equity?

**Yes**

**Possibly**

**No**

**I'm not sure**

These are owner preferences.

They do not become financing commitments.

---

# 11. EMPLOYEE CAPITAL

Because this is an employee-ownership platform, the engine should explicitly account for employee capital where applicable.

Potential information:

* Employee contribution
* Management contribution
* Acquisition entity capitalization
* Equity contribution
* Financing contribution
* Other employee funding mechanisms

The engine should not assume employees have capital available.

It should identify:

> **Employee capital assumption**

as an explicit assumption requiring validation.

---

# 12. ESOP CAPITAL SCENARIOS

Because Employee Ownership is the first journey, the Capital Engine should be able to represent financing associated with ESOP-related structures.

However, it must remain structure-neutral.

The engine should support questions such as:

> **What capital sources may be required for the contemplated employee-ownership structure?**

Potential categories:

* Senior debt
* Seller financing
* Other permitted financing
* Existing company cash
* Other capital sources

The exact transaction requirements should be determined by the appropriate professionals.

---

# 13. CAPITAL PROVIDER PROFILE

The Capital Engine should consume structured profiles from the Marketplace Engine.

A capital provider profile may include:

### Organization

* Name
* Type
* Website
* Headquarters
* Service region
* Contact information

### Financing Focus

* Transaction sizes
* Industries
* Geography
* Loan types
* Seller financing preferences
* Employee-ownership experience
* ESOP experience
* Acquisition experience

### Financing Preferences

* Minimum
* Maximum
* Seniority
* Security
* Typical term
* Other stated criteria

### Status

* Verified
* Active
* Accepting opportunities
* Temporarily unavailable

The Capital Engine does not own the profile.

---

# 14. CAPITAL MATCHING

The Capital Engine should take the transaction's characteristics and identify potentially relevant capital providers.

For example:

### Current Opportunity

**Transaction size:** $5M

**Business size:** $8M revenue

**Industry:** Professional services

**Employee ownership:** Yes

**Seller financing:** Possible

Then:

> **Several capital providers appear relevant to this financing profile.**

Show approximately **2–3 initial matches**.

Each can show:

**Why this provider appeared**

* Employee ownership experience
* Transaction-size fit
* Geography
* Industry fit
* Financing type

This is contextual matching, not a recommendation.

---

# 15. OWNER CHOICE

The owner chooses:

**Use My Lender**

or:

**Show Me Curated Capital Providers**

or:

**I'm Not Sure**

This follows the platform's standard 3-choice journey.

---

# 16. FINANCING REQUEST

The owner can create:

## Financing Request

The system assembles an appropriate package from:

* Destination
* Business Reality
* Selected Scenario
* Capital Plan
* Financial information
* Authorized documents
* Professional inputs

The owner reviews the information before sending.

---

# 17. CAPITAL PROVIDER PACKAGE

The Capital Engine can request a specialized:

## Financing Review Package

It may contain:

### Transaction

* Proposed transaction type
* Purchase price assumptions
* Ownership objective

### Financial

* Revenue
* EBITDA / cash flow
* Debt
* Historical performance
* Other relevant metrics

### Capital

* Requested financing
* Proposed seller financing
* Equity contribution
* Uses
* Sources
* Funding gap

### Timing

* Target closing
* Financing timeline

### Documents

Only those authorized for this provider.

---

# 18. FINANCING REQUEST STATUS

A financing request should have explicit states:

### Draft

Owner is preparing.

### Ready for Review

Owner has checked the information.

### Submitted

Sent to capital provider.

### Information Requested

Provider needs additional information.

### Under Review

Provider is evaluating.

### Indicative Terms

Provider has supplied preliminary terms.

### Conditional Approval / Other Provider Status

Provider has indicated a further stage, as applicable.

### Declined

Provider is not proceeding.

### Withdrawn

Owner stopped the request.

### Closed

Financing was completed.

The platform should not normalize every provider's terminology.

Provider-specific status can be retained alongside the platform's standardized status.

---

# 19. INDICATIVE TERMS OBJECT

When a provider submits potential terms, create:

## `FinancingProposal`

Potential fields:

* Provider
* Amount
* Rate
* Term
* Amortization
* Security
* Seniority
* Fees
* Conditions
* Required equity
* Seller financing requirements
* Expiration
* Notes
* Date submitted

Clearly label:

> **Indicative / preliminary**

unless the provider explicitly identifies a later, binding status.

---

# 20. DO NOT RANK FINANCING PROVIDERS

The platform should not say:

> "Lender A is the best."

Instead:

### Financing Options

**Provider A**

Potential terms

**Provider B**

Potential terms

**Provider C**

Potential terms

Then:

**Compare**

**Ask Questions**

**Select**

The owner decides.

---

# 21. FINANCING COMPARISON

The platform can standardize comparable fields.

|               | Provider A    | Provider B    |
| ------------- | ------------- | ------------- |
| Amount        | $X            | $Y            |
| Rate          | X%            | Y%            |
| Term          | X yrs         | Y yrs         |
| Equity needed | X             | Y             |
| Seller note   | Required / no | Required / no |
| Security      | Stated        | Stated        |
| Status        | Indicative    | Indicative    |

The platform can calculate differences using the Financial Modeling Engine.

It should not declare a winner.

---

# 22. CAPITAL PLAN + FINANCIAL MODELING

The separation should work like:

### Capital Engine

> "Provider A proposes $2M at X terms."

### Financial Modeling Engine

> "Under those terms, projected debt service is $Y."

### Scenario Engine

> "This creates Scenario B."

### Confidence Engine

> "This changes alignment with the owner's target because the closing proceeds assumption changed."

Each engine stays in its lane.

---

# 23. CAPITAL PLAN + DOCUMENT READINESS

When a lender requests:

> "Three years of financial statements"

the Capital Engine emits a requirement.

The Document Readiness Engine tracks it.

### Document

Three years financial statements

**Requested by:** Lender A

**Status:** Available locally

Then:

**Share**

The capital engine does not manage file permissions.

---

# 24. CAPITAL PLAN + PROFESSIONAL REVIEW

A financing professional may say:

> "This capital stack is not workable under the current assumptions."

That becomes:

**Professional Determination / Feedback**

The Capital Engine receives it and updates the plan.

It does not overrule the professional.

---

# 25. RESEARCH INTEGRATION

The Research Engine may investigate:

* Financing structures commonly used for similar transactions
* Current lending conditions
* Relevant program requirements
* Industry financing characteristics
* Common capital stack structures

The Capital Engine consumes those findings.

It does not independently conduct research.

---

# 26. FINANCING ASSUMPTIONS

Every capital proposal should distinguish:

### Owner Assumption

"I think the bank will lend $2.5M."

### Research Finding

"Comparable financing examples suggest amounts within a certain range."

### Provider Statement

"Lender submitted an indicative $2M proposal."

### Professional Determination

"Advisor concludes the financing structure should be modified."

These must never be blended.

---

# 27. CAPITAL PROVIDER QUESTIONS

The platform should help owners ask providers useful questions.

Examples:

* What business characteristics are most important to your underwriting?
* What equity contribution is expected?
* What seller financing is acceptable?
* What collateral is required?
* What conditions must be satisfied?
* What information remains outstanding?
* What could cause terms to change?

These are **questions for the provider**, not answers generated by the platform.

---

# 28. FINANCING READINESS

The Capital Engine should create:

## Financing Readiness

Rather than a simplistic approval score.

Example:

### Financing Preparation

**Business information:** Good

**Financial documentation:** Partial

**Transaction structure:** Preliminary

**Equity contribution:** Unknown

**Debt assumptions:** Preliminary

**Lender matches:** 3

**Outstanding information:** 4 items

This tells the owner what remains.

---

# 29. CAPITAL PLAN VERSIONING

Every material change creates a new version.

### Capital Plan v1

$5M purchase

$2M senior debt

$2M seller note

$1M equity

↓

### Capital Plan v2

$5M purchase

$2.5M senior debt

$1.5M seller note

$1M equity

Historical versions remain available.

---

# 30. CAPITAL EVENT MODEL

The Capital Engine should produce events.

Examples:

```text id="yqss9w"
CapitalRequestSubmitted
FinancingProposalReceived
CapitalProviderRequestedInformation
CapitalProposalUpdated
CapitalGapDetected
CapitalSourceAdded
CapitalSourceRemoved
FinancingStatusChanged
```

Other engines can react to these events without becoming coupled to the Capital Engine.

---

# 31. FINANCING DEADLINE MANAGEMENT

Some financing opportunities have expiration dates.

The engine should track:

* Submission deadline
* Proposal expiration
* Information deadline
* Closing deadline
* Required action dates

The Notification Engine handles alerts.

The Capital Engine maintains the underlying financing state.

---

# 32. CAPITAL STACK WHAT-IF

The owner should be able to experiment.

Example:

> **What if the seller note increases by $500K?**

The Capital Engine adjusts the potential capital stack.

The Financial Modeling Engine calculates the financial impact.

The Scenario Engine creates the corresponding scenario.

The Confidence Engine reassesses goal alignment.

The original plan remains available.

---

# 33. CAPITAL PLAN DASHBOARD

A user-facing view might look like:

# Financing

### Your Current Plan

**Estimated funding need:** $5.2M

### Potential Sources

Senior debt: $2.5M
Seller note: $1.5M
Equity: $750K

### Current Gap

**$450K**

### Capital Providers

3 potentially relevant providers

### Outstanding

2 documents
1 financing assumption
1 professional question

### Next Step

**Explore Funding Gap**

---

# 34. SELLER-NOTE LIQUIDITY CONNECTION

When the owner's capital plan contains seller financing:

The Capital Engine records:

**Seller Note = Capital Source**

Then:

**Seller-Note Liquidity Engine**

can separately manage:

> What happens if the seller later wants liquidity from that note?

This keeps origination and secondary-market activity separate.

---

# 35. CAPITAL ENGINE SHOULD SUPPORT MULTIPLE BUYER TYPES

The engine should not assume the employee buyers have one financing profile.

Potential buyer structures may include:

* Employee group
* Management group
* Acquisition entity
* ESOP
* Other employee-ownership structure

The Capital Engine cares about:

**Capital requirement**

not legal structure ownership details.

---

# 36. CAPITAL PROVIDER DATA MODEL

Conceptually:

```text id="a56e45"
CapitalProvider
│
├── Organization
├── Contacts
├── FinancingTypes
├── TransactionSizes
├── Industries
├── Geography
├── EmployeeOwnershipExperience
├── ESOPExperience
├── FinancingCriteria
├── Verification
├── Availability
└── MarketplaceStatus
```

This is consumed from the Marketplace Engine.

---

# 37. CAPITAL PLAN DATA MODEL

```text id="ddvdd1"
CapitalPlan
│
├── Transaction
├── Scenario
├── TotalUses
├── CapitalSources
│   ├── SeniorDebt
│   ├── SellerNote
│   ├── Equity
│   └── Other
├── CapitalGap
├── FinancingRequests
├── FinancingProposals
├── Requirements
├── ProfessionalFeedback
├── Status
└── VersionHistory
```

---

# 38. SECURITY AND PRIVACY

Capital requests may contain highly sensitive business information.

The engine must respect:

**Local-first storage**

**Explicit authorization**

**Stakeholder-specific disclosure**

**Document-level permissions**

**Audit trail**

The Capital Engine should never automatically expose the owner's full business record to every lender.

---

# 39. NO AUTOMATIC FINANCING DECISIONS

The platform can say:

> **"Provider A's stated criteria appear relevant to this transaction."**

It can display:

> **"Provider A submitted indicative terms."**

It should not say:

> **"Provider A will approve you."**

or:

> **"You qualify for this loan."**

unless that statement is directly attributable to the provider and accurately reflects its status.

---

# 40. CAPITAL ENGINE OUTPUTS

The engine should produce:

### Capital Plan

Potential capital stack.

### Funding Requirements

What capital may be needed.

### Capital Gaps

What remains unfunded.

### Capital Provider Matches

Potentially relevant providers.

### Financing Requests

Requests sent to providers.

### Financing Proposals

Provider-submitted potential terms.

### Financing Status

Where each request stands.

### Capital Requirements

Outstanding information needed by providers.

### What-If Capital Scenarios

Alternative capital-stack structures.

---

# 41. CAPITAL ENGINE NORTH STAR

The engine should help the owner move from:

> **"I think I need money to buy my business."**

to:

> **"Here's the capital this potential transaction appears to require, here are the sources we're exploring, here are the current gaps, here are the people who may be able to provide capital, and here is what each one needs from me."**

It should turn financing from an opaque event into a **visible, coordinated process**.

---

# 42. ARCHITECTURAL RULE

> **The Capital Engine coordinates capital. It does not become the lender, underwriter, financial advisor, or transaction decision-maker.**

---

# 43. CAPITAL ENGINE RELATIONSHIPS

```text id="gy5vpf"
DESTINATION
"What do I want?"
       ↓
BUSINESS REALITY
"What do I have today?"
       ↓
SCENARIO
"What could the transaction look like?"
       ↓
FINANCIAL MODELING
"What are the numbers?"
       ↓
CAPITAL ENGINE
"How might we fund it?"
       ↓
MARKETPLACE
"Who might provide the capital?"
       ↓
PROFESSIONAL REVIEW
"What do qualified professionals think?"
       ↓
TRANSACTION ENGINE
"Let's coordinate execution."
```

---

# 44. FINAL PRINCIPLE

The Capital Engine should make financing understandable without pretending that financing is simple.

The owner should be able to see:

**How much capital may be needed.**

**Where it might come from.**

**What the current gaps are.**

**What assumptions we're making.**

**What providers are potentially relevant.**

**What providers have actually said.**

**What remains for professionals to determine.**

The platform organizes the capital conversation.

**Capital providers make financing decisions.**
