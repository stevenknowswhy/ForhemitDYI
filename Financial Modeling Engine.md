# Financial Modeling Engine

## 1. Purpose

The Financial Modeling Engine performs the quantitative modeling required to understand ownership-transition economics under defined assumptions.

It handles:

* Financial models
* Cash-flow projections
* Purchase-price calculations
* Sources and uses
* Capital-stack calculations
* Debt service
* Seller-note payment schedules
* Seller proceeds
* Ownership allocations
* Sensitivity analysis
* Scenario financial comparisons
* What-if analysis
* Break-even calculations
* Assumption management
* Model versions
* Calculation provenance

Its central question is:

> **“What do the numbers look like under these stated assumptions?”**

The engine calculates.

It does not decide.

---

# 2. Core Architectural Principle

## Financial Modeling is a calculation engine, not a recommendation engine.

The engine may determine:

> **Under these assumptions, annual debt service is $X.**

It may determine:

> **Under these assumptions, the owner receives $Y at closing and $Z over five years.**

It may determine:

> **Under these assumptions, the transaction produces a $500,000 capital gap.**

It must not independently conclude:

> **Therefore this is the best transaction structure.**

That conclusion belongs to the owner, supported by scenarios and qualified professionals.

---

# 3. Why This Engine Exists

The platform needs a place where its quantitative questions can be answered consistently.

Examples:

* What happens to seller proceeds under different purchase prices?
* What is annual debt service?
* How much cash remains after debt service?
* How does a seller note affect future income?
* How much outside financing is required?
* What happens if the purchase price changes?
* What happens if EBITDA declines?
* What happens if interest rates increase?
* What percentage of ownership changes under different structures?
* When does a financing structure become cash-flow constrained?

Without a dedicated Financial Modeling Engine, every other engine would begin doing its own math.

That would create duplicated formulas, inconsistent assumptions, and difficult-to-audit results.

---

# 4. What This Engine Owns

The Financial Modeling Engine owns:

* Financial model definitions
* Model assumptions
* Calculation logic
* Formula execution
* Cash-flow schedules
* Debt schedules
* Seller-note schedules
* Sources and uses calculations
* Proceeds calculations
* Ownership allocation calculations
* Sensitivity analysis
* What-if analysis
* Break-even analysis
* Model versions
* Calculation outputs
* Calculation dependencies
* Model validation
* Numeric provenance

---

# 5. What This Engine Does NOT Own

It does not own:

* Business facts
* Valuation determinations
* Scenario selection
* Financing provider decisions
* Professional advice
* Owner decisions
* Transaction execution
* Document storage
* Accounting records
* Tax determinations
* Legal conclusions
* Marketplace matching

Those systems provide inputs or consume outputs.

---

# 6. The Model as a Reproducible Calculation

Every material model should be reproducible from:

```text
Inputs + Assumptions + Formulas + Model Version = Outputs
```

The engine must preserve enough information to reproduce a historical result.

Example:

```text
Model v4
+
Revenue assumption: $10M
EBITDA margin: 15%
Purchase price: $8M
Debt: $5M
Interest rate: 7%
Term: 10 years
Seller note: $2M

=
Annual debt service
Seller cash flow
Seller proceeds
Capital gap
Cash-flow coverage
```

If an assumption later changes, the old result remains reconstructable.

---

# 7. Model Types

The engine should support multiple model families.

## Transaction Model

Purchase price, financing, proceeds, ownership, and closing economics.

## Cash-Flow Model

Revenue, expenses, EBITDA, cash flow, debt service, and liquidity.

## Capital-Stack Model

Equity, debt, seller financing, reserves, fees, and sources/uses.

## Seller Proceeds Model

Closing proceeds, future payments, taxes as externally supplied assumptions, and liquidity timing.

## Seller-Note Model

Principal, rate, term, amortization, payments, and remaining balance.

## Ownership Model

Ownership percentages, units, allocations, dilution, and staged transitions.

## Scenario Model

Quantitative comparison of multiple platform scenarios.

## Sensitivity Model

Changes in outputs resulting from changes in specified assumptions.

## What-If Model

Interactive exploration of alternative assumptions.

---

# 8. Financial Assumptions

Assumptions should be first-class objects.

An assumption contains:

* Assumption ID
* Name
* Value
* Unit
* Currency
* Period
* Source
* Source version
* Date
* Confidence/status
* Owner-entered flag
* Professional-reviewed flag
* Scenario
* Model version
* Notes

Example:

```text
EBITDA = $1.5M
Source: Business Reality v7
Status: Owner Verified
```

The model must not obscure where an assumption came from.

---

# 9. Assumption Classes

The engine should distinguish:

## Historical Fact

A known historical value.

## Current Fact

A current business reality input.

## Forecast Assumption

An assumed future value.

## Platform Scenario Assumption

A modeling assumption supplied by a scenario.

## Professional Input

An assumption supplied or validated by a professional.

## Owner Assumption

An explicit owner assumption.

This distinction is critical because:

A modeled assumption is not the same thing as a verified business fact.

---

# 10. Source Provenance

Every material model input should reference its source.

Examples:

* Business Reality fact
* Local Vault document
* Spreadsheet cell
* Professional review
* Capital provider term
* Seller-note record
* Owner-entered assumption
* Research finding

The engine should preserve source and version references.

---

# 11. Formula Provenance

Important outputs should be explainable.

Example:

```text
Annual Debt Service = $697,830
```

The user should be able to inspect:

* Principal: $5M
* Rate: 7%
* Term: 10 years
* Amortization: Monthly
* Formula/model version: Debt Schedule v3

The system should never expose a mysterious number with no path back to its inputs.

---

# 12. Calculation Model

The engine should support deterministic calculations such as:

* Addition
* Subtraction
* Multiplication
* Division
* Percentages
* Ratios
* Growth rates
* Margins
* Compound growth
* Present value
* Future value
* Payment calculations
* Amortization
* Ownership allocation
* Debt service coverage
* Break-even analysis

Specialized formulas should be modular rather than scattered through application code.

---

# 13. Currency

Every monetary value should include:

* Currency
* Precision
* Date/period
* Source
* Whether nominal or adjusted, where applicable

The engine should not silently convert currencies.

Conversions should record:

* Original currency
* Converted currency
* Exchange-rate source
* Rate date
* Conversion method

---

# 14. Periods

Financial modeling must explicitly identify time periods.

Examples:

* Monthly
* Quarterly
* Annual
* Fiscal year
* Calendar year
* Transaction date
* Payment date

The engine should distinguish:

```text
FY2025
```

from:

```text
Calendar 2025
```

where applicable.

---

# 15. Date Logic

The engine should support:

* Actual dates
* Month-end
* Quarter-end
* Year-end
* Payment dates
* Grace periods
* Interest accrual periods
* Closing dates
* Relative dates

Date assumptions should be explicit rather than hidden inside formulas.

---

# 16. Sources and Uses

The engine should calculate transaction funding requirements.

Example:

| Use | Amount |
| --- | --- |
| Purchase price | $8M |
| Refinancing | $500K |
| Fees | $250K |
| Working capital | $750K |
| **Total uses** | **$9.5M** |

| Source | Amount |
| --- | --- |
| Bank debt | $5M |
| Seller note | $2M |
| Employee/management equity | $1M |
| Other equity | $1.5M |
| **Total sources** | **$9.5M** |

The Capital / Financing Engine owns the financing process.

Financial Modeling owns the arithmetic.

---

# 17. Capital Gap

The engine should calculate:

```text
Total Uses - Total Sources = Capital Gap
```

Example:

```text
Uses: $9.5M
Sources: $8.75M
Capital gap: $750K
```

The engine reports the gap.

It does not decide how the gap should be solved.

---

# 18. Debt Modeling

The engine should support:

* Principal
* Interest rate
* Fixed/variable indication
* Term
* Amortization period
* Payment frequency
* Interest-only periods
* Balloon payments
* Fees
* Beginning balance
* Principal payment
* Interest payment
* Ending balance

Outputs should be available by period.

---

# 19. Debt Service

The engine can calculate:

* Periodic payment
* Annual debt service
* Principal/interest split
* Remaining balance
* Total interest
* Cash-flow coverage

Financial ratios should clearly identify their numerator and denominator.

---

# 20. Seller-Note Modeling

The engine should support seller-note structures such as:

* Fixed amortization
* Interest-only
* Balloon
* Deferred payments
* Custom schedules
* Partial repayment
* Multiple tranches

Example output:

```text
Initial note: $2M
Rate: 6%
Term: 7 years
Payment frequency: Monthly
Remaining principal after year 3: $1.29M
```

The Seller-Note Liquidity Engine remains authoritative for the actual note and servicing process.

---

# 21. Seller Proceeds

The engine can model:

```text
Gross purchase price
Less debt payoff
Less transaction costs
Plus/minus modeled adjustments
= modeled closing proceeds
```

Future seller-note payments can be modeled separately.

Tax treatment should be represented only through appropriate supplied assumptions or professional determinations, not independently invented by the model.

---

# 22. Ownership Modeling

The engine may calculate ownership allocations.

Examples:

```text
Employee pool: 60%
Management: 20%
Founder: 20%
```

It can model changes:

```text
Founder transfers 5%.
```

Result:

```text
Employee pool: 60%
Management: 25%
Founder: 15%
```

The authoritative ownership structure remains with Ownership Lifecycle after closing or the relevant transaction record before closing.

---

# 23. Dilution

Where applicable, the engine can model dilution from:

* New equity
* New ownership interests
* Additional employee allocations
* Management allocations
* Conversions

The model should distinguish:

```text
Modeled dilution
```

from:

```text
Actual ownership change.
```

---

# 24. Cash-Flow Modeling

The engine can model:

* Revenue
* Growth
* Gross profit
* Operating expenses
* EBITDA
* Cash taxes as supplied assumptions
* Capital expenditures
* Working capital
* Debt service
* Seller-note payments
* Free cash flow
* Minimum cash reserve

It should clearly distinguish reported historical financials from forecasts.

---

# 25. Forecasting

Forecast assumptions should support:

* Growth rate
* Margin assumptions
* Expense growth
* Headcount assumptions
* Capex
* Working capital assumptions
* Debt changes
* Ownership-related cash flows

Forecasts should never silently become historical facts.

---

# 26. Sensitivity Analysis

The engine should calculate output changes caused by defined input changes.

Example:

| Purchase Price | Seller Proceeds | Annual Debt Service |
| --- | --- | --- |
| $7M | $X | $Y |
| $8M | $X | $Y |
| $9M | $X | $Y |

Sensitivity analysis is descriptive.

It does not identify a preferred value.

---

# 27. Multi-Variable Sensitivity

Support combinations such as:

* Purchase price × interest rate
* EBITDA × purchase price
* Growth × margin
* Debt × seller note
* Closing proceeds × future income

Outputs should identify the exact assumptions varied.

---

# 28. What-If Exploration

The user should be able to change a variable and see the mathematical effect.

Example:

> **What if purchase price were $7.5M instead of $8M?**

The model recalculates:

* Sources and uses
* Debt
* Seller note
* Annual payments
* Seller proceeds
* Ownership allocation, where applicable

The original model remains unchanged.

---

# 29. What-If Branching

What-if explorations should create branches.

Example:

```text
Base Model
   ├── What If A: Purchase Price -10%
   ├── What If B: Interest Rate +1%
   └── What If C: Seller Note +$500K
```

Branches reference a common base model rather than duplicating everything unnecessarily.

---

# 30. Break-Even Analysis

The engine can calculate defined break-even points.

Examples:

* Minimum revenue to cover debt service
* Maximum debt supported by modeled cash flow
* Purchase price corresponding to a specified payment level
* Interest rate corresponding to a payment threshold

The calculation result should always state the assumptions used.

---

# 31. Coverage Analysis

The engine may calculate ratios such as:

* Debt service coverage
* Debt-to-EBITDA
* Purchase price / EBITDA
* Seller proceeds / purchase price
* Cash reserve coverage

Each ratio should provide its formula definition.

Example:

```text
Debt Service Coverage = Cash Available for Debt Service / Debt Service
```

The engine calculates the ratio.

It does not decide whether the ratio is “good.”

Where an external threshold is needed, that threshold must be identified as an external policy, lender criterion, professional input, or scenario assumption.

---

# 32. Assumption Sensitivity

The engine should indicate which outputs are highly dependent on which assumptions.

Example:

Seller future income is materially affected by:

* Seller-note principal
* Interest rate
* Repayment term

This is an analytical relationship, not a recommendation.

---

# 33. Model Status

A model should have states such as:

* Draft
* Preliminary
* Owner Reviewed
* Professionally Reviewed
* Scenario Model
* Superseded
* Archived

The state describes review status, not whether the model is “correct” in an absolute sense.

---

# 34. Model Versioning

Every material change creates a new model version.

Example:

```text
Model v1
Purchase price = $8M.

Model v2
Purchase price = $8.5M.

Model v3
Interest rate changed from 7% to 7.5%.
```

Historical outputs remain reproducible.

---

# 35. Calculation Reproducibility

A historical model should be reproducible using:

* Model version
* Input versions
* Formula version
* Calculation date
* Calculation engine version
* Unit/currency settings

This should be enough to explain why an earlier output differed from a later output.

---

# 36. Formula Versioning

Calculation logic itself must be versioned.

Example:

```text
Debt Schedule Formula v2.
```

If the implementation of a formula changes materially, existing models should retain the formula version that produced their historical results.

---

# 37. Model Dependencies

A model may depend upon:

* Business Reality version
* Scenario version
* Capital Plan
* Seller Note
* Ownership model
* Professional input
* Owner assumptions

Dependencies should be explicit.

---

# 38. Stale Inputs

If an important source input becomes outdated, the engine should flag the model.

Example:

```text
Model v6 uses EBITDA from Business Reality v4.
Business Reality has since changed materially.
```

The system can show:

```text
Model may need refresh.
```

It should not silently recalculate the old model and overwrite historical results.

---

# 39. Conflict Inputs

If Business Reality contains a conflict:

```text
Revenue = $8.2M vs $8.5M
```

The financial model should not silently choose one.

Instead:

```text
Input conflict requires resolution.
```

Alternatively, the model may deliberately branch:

```text
Case A: $8.2M
Case B: $8.5M
```

provided that the assumptions are clearly labeled.

---

# 40. Scenario Relationship

Scenario Engine asks:

> **What could this transaction path look like?**

Financial Modeling asks:

> **What do the numbers look like under the scenario's assumptions?**

A scenario may contain one or more financial models.

The Scenario Engine remains responsible for the scenario as a whole.

---

# 41. Scenario Comparison

The engine can calculate side-by-side outputs.

Example:

| Metric | Scenario A | Scenario B |
| --- | --- | --- |
| Closing proceeds | $X | $Y |
| Future seller income | $X | $Y |
| External financing | $X | $Y |
| Seller-note balance | $X | $Y |

This is factual comparison, not ranking.

---

# 42. Professional Inputs

Professionals may supply modeling inputs.

Examples:

* Valuation range
* Financing terms
* Tax assumptions
* Ownership structure assumptions

The engine should identify these as:

```text
Professional input.
```

It should not transform them into platform-generated conclusions.

---

# 43. Professional Determination Boundary

Suppose a valuation professional determines:

```text
Fair market value = $9M.
```

The Financial Modeling Engine may calculate:

```text
Debt service if purchase price = $9M.
```

It does not independently determine:

```text
$9M is the correct purchase price.
```

Similarly, if a tax professional supplies a tax assumption, the engine uses the assumption but does not become the tax advisor.

---

# 44. Owner Decision Boundary

The owner may ask:

> **“What happens if I take a lower purchase price for more seller-note income?”**

The engine calculates both cases.

The owner decides which path to pursue.

The Decision Record captures the choice.

---

# 45. Capital / Financing Boundary

Capital Engine owns:

* Financing requests
* Provider interactions
* Capital sources
* Financing status
* Financing commitments

Financial Modeling owns:

* Payment calculations
* Debt schedules
* Sources and uses arithmetic
* Cash-flow effects

Capital does not duplicate the math.

Financial Modeling does not negotiate with lenders.

---

# 46. Seller-Note Boundary

Seller-Note Liquidity owns:

* Actual note
* Buyer relationship
* Payment history
* Liquidity options
* Servicing

Financial Modeling owns:

* Modeled payment schedule
* Present value
* Remaining balance calculations
* Sensitivity analysis

Actual note data should feed models where authorized.

---

# 47. Ownership Lifecycle Boundary

Ownership Lifecycle owns the established post-close ownership state.

Financial Modeling can model:

> **“What would ownership look like if allocation changes occurred?”**

That is a modeled state.

It is not the actual ownership record until the appropriate ownership process establishes it.

---

# 48. Local-First Architecture

Financial models should support local execution where sensitive inputs are involved.

For example:

```text
Owner loads financial statements into Local Vault.
Local analysis produces candidate inputs.
Financial Modeling calculates scenarios locally.
```

Sensitive financial information does not need to leave the device merely to calculate debt service or proceeds.

---

# 49. Cloud vs Local Models

The engine should support:

## Local model

Sensitive inputs and calculations remain on the owner's device.

## Cloud model

Authorized structured information is processed remotely.

## Hybrid model

Sensitive source material stays local while authorized structured inputs are sent to a remote calculation service.

The choice follows Local Vault, Consent & Access, and Policy rules.

---

# 50. Model Output Sharing

A model output can itself be sensitive.

Examples:

* Seller proceeds
* Debt capacity
* Ownership allocations
* Internal negotiation assumptions

Therefore calculated outputs require the same permission model as other business information.

---

# 51. Calculation Output Provenance

Every important output should be traceable to:

* Model version
* Input versions
* Formula version
* Source references
* Calculation timestamp

Example:

```text
Seller proceeds = $5.7M
Model: TX-004 Financial Model v8
Inputs: Business Reality v7, Capital Plan v4, Seller Note v2
```

---

# 52. Rounding

Rounding should be explicit.

The engine should distinguish:

```text
Stored precision
```

from:

```text
Display precision.
```

Example:

```text
Stored: $697,829.83
Display: $697,830
```

Calculations should generally use stored precision rather than repeatedly rounded display values.

---

# 53. Negative Values and Edge Cases

The engine must handle:

* Negative cash flow
* Zero revenue
* Zero debt
* Zero interest
* Negative growth
* Interest-only periods
* Balloon balances
* Ownership percentages exceeding 100% due to bad input
* Capital sources not matching uses
* Missing inputs

Invalid states should be surfaced rather than producing plausible-looking numbers.

---

# 54. Model Validation

Before a model is marked ready, validate:

* Required inputs exist
* Units match
* Currency matches
* Ownership totals are valid
* Sources equal uses where required
* Debt schedules reconcile
* Beginning and ending balances reconcile
* Payment schedules reconcile
* Formulas produce finite values
* No prohibited circular references exist
* Required professional inputs are present

---

# 55. Circular Dependencies

The engine should detect circular calculations.

Example:

```text
Debt depends on cash flow.
Cash flow depends on debt service.
Debt service depends on debt.
```

Circular logic should be explicit and handled through supported iterative methods only where deliberately configured.

The user should never receive a number from an invisible circular calculation.

---

# 56. Model Scenarios

A model may contain:

* Base Case
* Conservative Case
* Upside Case
* Stress Case

These labels describe assumptions.

They are not platform recommendations.

---

# 57. Stress Testing

The engine should support defined stress assumptions.

Examples:

* Revenue -20%
* EBITDA margin -300 bps
* Interest rate +2%
* Closing delayed 6 months

The engine calculates the effect.

It does not conclude what the owner should do.

---

# 58. Model Comparison

The engine should allow comparison of model versions.

Example:

```text
Model v7 → v8

Changes:
Purchase price +$500K
Debt +$250K
Interest rate +50 bps
Seller note unchanged

Outputs changed accordingly.
```

---

# 59. “Why Did This Number Change?”

A key user capability should be:

```text
Why did seller proceeds change?
```

The system can answer:

```text
Purchase price increased $500K.
Debt payoff increased $200K.
Transaction costs increased $50K.
Net modeled proceeds increased $250K.
```

This makes modeling explainable.

---

# 60. “What Is Driving This Model?”

The platform should identify material drivers.

Example:

```text
Model output is most sensitive to:
Purchase price
Interest rate
EBITDA
Seller-note term
```

The engine reports mathematical sensitivity rather than recommending an action.

---

# 61. API / Engine Contract

Core capabilities:

```text
createModel()
createModelVersion()
addAssumption()
updateAssumption()
calculate()
calculateScenario()
calculateSensitivity()
createWhatIfBranch()
calculateBreakEven()
calculateDebtSchedule()
calculateSellerNoteSchedule()
calculateSourcesAndUses()
calculateSellerProceeds()
calculateOwnershipAllocation()
calculateCashFlow()
compareModels()
validateModel()
getModelInputs()
getModelOutputs()
getFormulaTrace()
getInputProvenance()
getModelDependencies()
identifyStaleInputs()
refreshModel()
archiveModel()
```

---

# 62. Events

The engine can emit:

```text
ModelCreated
ModelVersionCreated
ModelCalculated
ModelValidationFailed
ModelValidationPassed
ModelInputChanged
ModelOutputChanged
ModelMarkedStale
SensitivityCalculated
WhatIfCreated
ModelSuperseded
ModelArchived
```

Workflow, Transaction Orchestration, Confidence, Notification, Audit, and other engines can subscribe.

---

# 63. Example End-to-End Flow

A clean example:

```text
Business Reality
      ↓
Verified Revenue / EBITDA
      ↓
Scenario Engine
      ↓
Transaction Scenario
      ↓
Financial Modeling
      ↓
Sources & Uses
Debt Schedule
Seller Proceeds
Cash Flow
Sensitivity
      ↓
Capital / Financing
      ↓
Professional Review
      ↓
Owner Decision
      ↓
Transaction Orchestration
```

Financial Modeling is the quantitative middle layer.

---

# 64. Example: Seller Wants $5M at Closing

Owner destination contains:

```text
Desired closing proceeds: $5M.
```

Financial Modeling can calculate:

```text
Purchase price required under specified debt, note, fee, and payoff assumptions.
```

The model does not state:

> **“The business is worth that amount.”**

Valuation and professional review remain separate.

---

# 65. Example: Employee Ownership

Scenario:

```text
Employees acquire 60%.
```

Financial Modeling calculates:

* Amount employees must fund
* External financing requirement
* Seller financing requirement
* Seller proceeds
* Payment schedule
* Cash-flow impact

It does not determine whether the employee ownership structure is advisable.

---

# 66. Example: Financing Stress Test

Base assumptions:

```text
Debt: $5M
Interest: 7%
EBITDA: $1.5M
```

Stress case:

```text
Interest: 9%
EBITDA: $1.2M
```

The engine calculates the new debt service and coverage.

Capital / Financing and professionals interpret the financing implications.

---

# 67. Relationship to Confidence

A model result may provide evidence to the Confidence Engine.

Example:

```text
Financial model demonstrates that the stated proceeds objective is
mathematically achievable under the current scenario assumptions.
```

Confidence may incorporate that evidence.

But Financial Modeling does not assign the overall Goal-to-Reality Confidence score.

---

# 68. Relationship to Research

Research may supply external assumptions such as:

* Industry growth rates
* Market ranges
* Financing assumptions
* Benchmark margins

The Evidence Ledger preserves research provenance.

Financial Modeling uses authorized assumptions from those sources.

Research does not become financial truth simply because the model uses it.

---

# 69. Relationship to Decision Records

Decision Record stores:

```text
Owner chose Scenario B.
```

Financial Modeling stores:

```text
Scenario B produces these modeled outcomes under these assumptions.
```

A later decision can reference the model version that was available at the time.

---

# 70. Relationship to Audit

Audit records:

```text
Model v8 created.
Purchase price changed.
Model recalculated.
```

Financial Modeling preserves:

```text
Formula and assumption structure.
```

Audit preserves:

```text
Historical activity.
```

---

# 71. What the Engine Should Never Do

It should never:

* Decide which scenario is best
* Recommend a purchase price
* Determine fair market value
* Give tax advice
* Approve financing
* Determine legal consequences
* Treat forecast assumptions as historical facts
* Silently overwrite model versions
* Hide model inputs
* Present unsupported precision
* Resolve conflicting source data silently
* Replace professional judgment
* Replace owner decisions
* Become the accounting system
* Become the lender's underwriting system

---

# 72. Architectural Lock

These should be treated as requirements:

1. Financial Modeling is a standalone engine.
2. Its sole purpose is quantitative calculation and exploration under explicit assumptions.
3. It is not a recommendation engine.
4. Assumptions are first-class objects.
5. Historical facts are distinguished from forecasts and scenario assumptions.
6. Material inputs retain provenance.
7. Outputs retain formula and input provenance.
8. Model versions are preserved.
9. Formula versions are preserved.
10. Historical models remain reproducible.
11. What-if analysis creates branches rather than mutating the base model.
12. Sensitivity analysis is supported.
13. Stress testing is supported.
14. Sources and uses are supported.
15. Debt and seller-note schedules are supported.
16. Ownership allocation can be modeled without replacing actual ownership records.
17. Financial models can operate locally where sensitive inputs require local processing.
18. Cloud modeling is subject to Policy and Consent rules.
19. Conflicting inputs are surfaced rather than silently resolved.
20. Stale inputs are identified.
21. Invalid models fail visibly rather than generating plausible-looking results.
22. Rounding and precision are explicit.
23. Currency and time periods are explicit.
24. Model outputs can be compared without ranking scenarios.
25. Professional inputs remain attributed to professionals.
26. Valuation remains separate from modeling.
27. Financing decisions remain separate from modeling.
28. Tax and legal determinations remain separate from modeling.
29. Owner decisions remain separate from modeling.
30. Model outputs are available to Confidence, Scenario, Capital, Transaction, Workflow, and other engines through explicit contracts.
31. The engine is independently versioned, tested, auditable, and replaceable.

---

# 73. Architectural Boundary Summary

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Financial Modeling** | Calculations, assumptions, formulas, cash flows, debt schedules, proceeds, sensitivities, what-if models | Whether a scenario is advisable |
| **Business Reality** | Actual/current business facts | Forecast mathematics |
| **Scenario** | Potential transaction paths and assumptions | Choosing the preferred path |
| **Valuation** | Professional/value determination | Generic financial modeling |
| **Capital / Financing** | Financing process, providers, commitments | Debt-payment arithmetic |
| **Seller-Note Liquidity** | Actual seller note, servicing, liquidity | Modeled note calculations |
| **Ownership Lifecycle** | Actual post-close ownership state | Hypothetical ownership modeling |
| **Professional Review** | Professional determinations | Calculation infrastructure |
| **Decision Record** | Owner's choice and rationale | Financial calculations |
| **Confidence** | Goal-alignment/evidence signal | Underlying financial formulas |
| **Transaction / Orchestration** | Execution plan and transaction state | Financial model construction |
| **Audit / Provenance** | Historical record of model activity | Model logic itself |
| **Local Vault** | Private source files and local workspace | Financial-model semantics |
| **Consent & Access** | Who may access model inputs/outputs | Calculations |

## Hard Boundary

Financial Modeling tells us what the numbers are under stated assumptions. It does not tell the owner what those numbers mean for the decision they should make.

---

# 74. The Engine's Place in the Platform

The broader architecture becomes:

```text
OWNER DESTINATION
        ↓
BUSINESS REALITY
        ↓
SCENARIO
        ↓
FINANCIAL MODELING
        ↓
PROFESSIONAL REVIEW
        ↓
OWNER DECISION
        ↓
TRANSACTION ORCHESTRATION
        ↓
CLOSING
        ↓
OWNERSHIP LIFECYCLE
```

With Financial Modeling supporting multiple points in the journey:

```text
                    FINANCIAL MODELING
                           │
          ┌────────────────┼─────────────────┐
          ▼                ▼                 ▼
      SCENARIOS        FINANCING       SELLER NOTE
          │                │                 │
          ▼                ▼                 ▼
       PROCEEDS         DEBT/CASH        PAYMENTS
          │                │                 │
          └────────────────┼─────────────────┘
                           ▼
                    OWNER UNDERSTANDING
```

The engine therefore becomes the platform's quantitative laboratory: a controlled place to calculate, stress, compare, and understand financial consequences without letting the mathematics quietly become a recommendation.

---
