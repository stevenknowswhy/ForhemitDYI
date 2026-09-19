Absolutely. This is the natural counterpart to the **Destination Engine**.

The Destination Engine captures:

> **Where do I want to go?**

The **Current State / Business Assessment Engine** captures:

> **Where am I today?**

That separation is foundational. It gives us a clean bridge:

**Destination → Current Reality → Research → Scenarios**

I would formally name it the **Business Reality Engine**, with **Current State / Business Assessment** as the user-facing concept.

# Business Reality Engine v1.0

## Current State / Business Assessment

## 1. Purpose

The Business Reality Engine establishes a structured, evidence-aware representation of the business as it exists today.

Its purpose is to answer:

> **What does the business look like right now?**

It should collect, organize, classify, and validate business information without making the final determination about:

* Business value
* Transaction feasibility
* Tax treatment
* Legal structure
* Financing approval
* Investment suitability
* Whether an employee-ownership structure should be pursued

Those determinations belong to other engines and qualified professionals.

---

# 2. CORE SEPARATION

The platform must maintain a clear distinction between:

### Desired Outcome

> What the owner wants.

### Current State

> What the business appears to be today based on available information.

### Research Evidence

> What external evidence says about those facts and assumptions.

### Platform Scenario

> What might potentially connect the current state to the desired outcome.

### Professional Determination

> What a qualified professional concludes should actually be done.

The Business Reality Engine owns only the **Current State**.

---

# 3. CURRENT STATE OBJECT

The engine creates a primary object:

## BusinessCurrentState

This becomes the platform's structured snapshot of the business.

It should contain:

### Company Identity

* Business name
* Legal entity type
* Industry
* Primary location
* Operating locations
* Years in business
* Ownership structure

### Financial Profile

* Revenue
* Gross profit
* EBITDA / operating cash flow where appropriate
* Expenses
* Cash
* Working capital
* Capital expenditures
* Existing debt
* Debt service
* Other material obligations

### Workforce

* Employee count
* Full-time / part-time structure
* Management team
* Key employees
* Ownership participation where applicable
* Employee tenure characteristics

### Operations

* Core products/services
* Customer concentration
* Supplier concentration
* Geographic concentration
* Recurring vs. non-recurring revenue
* Seasonality
* Capital intensity
* Major operational dependencies

### Ownership / Management

* Owner count
* Ownership percentages
* Owner responsibilities
* Key-person dependency
* Management depth
* Succession readiness

### Assets

* Equipment
* Real estate
* Intellectual property
* Vehicles
* Other material assets

### Liabilities / Risk

* Debt
* Leases
* Litigation where disclosed
* Material contractual obligations
* Regulatory dependencies
* Insurance considerations

### Transaction Readiness

* Financial records available
* Tax records available
* Corporate records available
* Contracts available
* Employee information available
* Existing valuation
* Existing financing
* Other relevant documents

---

# 4. FACTS MUST HAVE A SOURCE

Every important Current State fact should carry provenance.

For example:

### Revenue

**$8.2M**

Source:

**2025 Profit & Loss**

Evidence status:

**Document-supported**

Date:

**2025**

Or:

### EBITDA

**Approximately $1.4M**

Source:

**Owner entered**

Evidence status:

**Owner-reported**

This distinction is critical.

---

# 5. FACT CONFIDENCE

The Business Reality Engine should not determine Goal-to-Reality Confidence.

However, it should provide **fact confidence / evidence status**.

For example:

### Information Status

**Owner-reported**

**Document-supported**

**Professionally verified**

**Externally verified**

**Conflicting**

**Unknown**

**Outdated**

This gives the Confidence Engine clean inputs without duplicating its responsibility.

---

# 6. THREE INFORMATION LEVELS

Every important Current State field should ideally have:

### Stated

What the owner says.

### Supported

What available documents support.

### Verified

What an appropriate professional or authoritative source has confirmed.

Example:

```text
Revenue

Owner stated:
$8.5M

Document supported:
$8.2M

Professionally verified:
Pending
```

The system should preserve the difference rather than automatically replacing the owner's statement.

---

# 7. DO NOT FORCE PRECISION TOO EARLY

The engine should accept approximate information initially.

For example:

### Annual Revenue

**Under $1M**

**$1M–$5M**

**$5M–$10M**

**$10M–$25M**

**$25M+**

**I don't know**

Later, the owner can provide exact information.

This allows the journey to progress without forcing a business owner to dig through records just to answer the first few questions.

---

# 8. PROGRESSIVE DETAIL

Current State should have levels.

## Level 1: Business Snapshot

Enough information to begin preliminary scenario exploration.

Potentially:

* Industry
* Years operating
* Revenue range
* EBITDA/cash-flow range
* Employee count
* Debt range
* Ownership structure

## Level 2: Business Profile

Adds:

* Management
* Owner dependence
* Customer concentration
* Recurring revenue
* Assets
* Operational characteristics

## Level 3: Transaction Readiness

Adds:

* Detailed financial information
* Supporting documents
* Corporate records
* Debt schedules
* Contracts
* Employee information
* Other relevant materials

This prevents the Current State Engine from becoming a giant intake form.

---

# 9. CURRENT STATE JOURNEY

The engine should ask questions only when the answer can meaningfully improve the model.

Example:

### How large is your business?

↓

Revenue range

↓

### About how much does the business generate in operating cash flow?

↓

Cash-flow range

↓

### How many employees do you have?

↓

Employee count

↓

### Could the business operate without you?

↓

Owner dependency

↓

### Do you have a management team that could lead the business after you leave?

↓

Management readiness

Each answer creates or updates part of the BusinessCurrentState.

---

# 10. BUSINESS HISTORY

Because the owner may have owned the company for 15 years, history matters.

Capture:

* Years owned
* Years in operation
* Major acquisitions
* Major expansions
* Significant changes
* Revenue trends
* Profitability trends
* Ownership changes

But avoid collecting history simply because it is interesting.

Only capture what may materially matter to the journey.

---

# 11. FINANCIAL TREND OBJECT

Current State should not be only a single-year snapshot.

Where information is available, capture trends.

For example:

| Metric  |  2023 |  2024 |  2025 |
| ------- | ----: | ----: | ----: |
| Revenue | $6.8M | $7.5M | $8.2M |
| EBITDA  | $900K | $1.1M | $1.4M |
| Debt    | $3.0M | $2.7M | $2.2M |

The platform can then identify:

**Growing**

**Stable**

**Declining**

**Volatile**

but should be cautious about interpreting why those trends exist.

Interpretation belongs to Research and professional review.

---

# 12. OWNER DEPENDENCY ASSESSMENT

This deserves its own component.

## How dependent is the business on you?

**The business operates independently**

**I'm important, but the team can operate without me**

**Several critical functions depend on me**

**Most major decisions depend on me**

**Almost everything depends on me**

**I'm not sure**

This becomes:

### Owner Dependency Profile

Potential subcategories:

* Sales
* Customer relationships
* Operations
* Finance
* Hiring
* Vendor relationships
* Strategic decisions
* Industry expertise

The owner can answer quickly.

Later, the platform can ask for evidence where useful.

---

# 13. MANAGEMENT READINESS

The engine should capture:

* Management roles
* Years of experience
* Leadership coverage
* Decision authority
* Operational responsibility
* Potential successors
* Employee ownership interest

The platform should not label management:

**"qualified"**

or

**"unqualified."**

Instead it records facts and owner-reported assessments.

Professional determination comes later.

---

# 14. EMPLOYEE OWNERSHIP READINESS

Since this first journey is employee ownership, include a dedicated section.

Possible information:

### Employee population

* Approximate number
* Full-time/part-time
* Tenure distribution

### Management

* Existing leadership
* Potential employee leaders

### Employee interest

**Strong interest**

**Some interest**

**Unknown**

**Not yet discussed**

### Employee ownership familiarity

**High**

**Some**

**Limited**

**Unknown**

Again, these are inputs rather than a final determination.

---

# 15. CUSTOMER & REVENUE PROFILE

Potential questions:

### Is revenue concentrated among a small number of customers?

**Low concentration**

**Moderate**

**High**

**I'm not sure**

### What type of revenue does the company have?

**Mostly recurring**

**Mix of recurring and project-based**

**Mostly project-based**

**Mostly transactional**

**I'm not sure**

These are especially useful because they influence later research and scenario analysis.

The platform should not independently declare that one model is "good" or "bad."

---

# 16. BUSINESS DEPENDENCY & CONCENTRATION FLAGS

The engine can identify structured conditions such as:

🟡 **High owner dependency**

🟡 **High customer concentration**

🟡 **Key management gap**

🟡 **Significant debt**

These are **Current State flags**.

They are not feasibility determinations.

The Research and Confidence engines can later determine how relevant they are to the owner's goals.

---

# 17. BUSINESS REALITY SUMMARY

After sufficient information:

# Your Business Today

### Business

15 years operating
$5M–$10M revenue
40–75 employees

### Financial

Approx. $1M–$2M EBITDA
Moderate debt

### Ownership

Founder-owned

### Management

Existing management team

### Owner Dependency

Moderate

### Revenue

Recurring + project-based

### Employee Ownership Interest

Not yet established

### Information Quality

Several figures are owner-reported
Financial documentation partially available

This becomes the **Current State Snapshot**.

---

# 18. CURRENT STATE VS. DESTINATION

The platform should allow the owner to see the two side by side.

## Your Destination

$3M–$5M at closing
Employee ownership
18-month transition
Retirement

## Your Current Reality

$5M–$10M revenue
$1M–$2M EBITDA
Existing debt
40–75 employees
Moderate owner dependency

Then:

> **We now have enough information to begin researching how closely your destination aligns with your current business reality.**

That action triggers the Research Engine.

---

# 19. CURRENT STATE SHOULD NEVER CHANGE THE DESTINATION AUTOMATICALLY

Suppose the owner wants:

**$5M minimum at closing**

and the Business Reality Engine discovers significant debt.

It should not revise the goal.

Instead:

> **Current State information has changed. Your destination remains unchanged.**

Then the Confidence Engine evaluates what that means.

---

# 20. DOCUMENT INTEGRATION

The engine works with the Document Readiness Engine.

For example:

### Revenue

Owner reported ✓

Financial statement available locally ✓

Verified ⏳

### Debt

Owner reported ✓

Debt schedule missing 🔴

The Document Readiness Engine can now say:

> **Debt schedule would improve Current State completeness.**

---

# 21. LOCAL-FIRST DOCUMENT ANALYSIS

Sensitive documents can be analyzed locally where technically appropriate.

Example:

Owner selects:

**2025 Financial Statements**

The local system extracts relevant structured information.

Potential result:

**Revenue:** $8.2M

**EBITDA:** $1.4M

The Business Reality Engine can store:

**Extracted from local document**

without needing to upload the underlying document.

The user explicitly decides whether the source document is later shared.

---

# 22. CONFLICT DETECTION

The engine should detect inconsistent information.

Example:

Owner states:

**Revenue: $10M**

Financial statement shows:

**Revenue: $8.2M**

The system should say:

> ⚠️ **We found a difference between your stated revenue and the financial statement.**

Then:

**Update**

**Keep Both for Review**

**Investigate**

Do not silently overwrite the owner-provided figure.

---

# 23. DATA QUALITY

The engine should continuously evaluate:

### Completeness

How much is known?

### Consistency

Do sources agree?

### Freshness

Is the data current?

### Provenance

Where did it come from?

### Sensitivity

How confidential is it?

These feed the Confidence Engine.

---

# 24. BUSINESS REALITY RESEARCH REQUESTS

The Research Engine should be able to ask:

> "What additional facts would materially improve our understanding of this business?"

For example:

**Customer concentration unknown**

→ Research Engine identifies its importance.

→ Document Readiness Engine adds relevant information request.

→ Owner provides information.

→ Current State updates.

This creates a closed-loop system.

---

# 25. CURRENT STATE VERSIONING

Business information changes.

The engine should preserve versions.

Example:

**BusinessCurrentState v1**

Owner estimates:

Revenue $8M

↓

**v2**

Financial statements added:

Revenue $8.2M

↓

**v3**

Updated 2026 financials:

Revenue $8.7M

Historical snapshots remain available.

---

# 26. CURRENT STATE SNAPSHOT DATE

Every snapshot should say:

### Current as of

**September 2026**

because a business isn't static.

This becomes particularly important when scenarios and research are repeated later.

---

# 27. PROFESSIONAL VERIFICATION

Professionals can verify or challenge specific facts.

For example:

### CPA

Revenue

**Professionally reviewed**

### Valuation Professional

Financial trend

**Professionally reviewed**

### Attorney

Ownership structure

**Professionally reviewed**

Professional verification should be attributed and should never be silently converted into generic "platform truth."

---

# 28. ENGINE OUTPUTS

The Business Reality Engine should produce:

### Current State Snapshot

High-level view.

### Business Profile

Structured business information.

### Financial Profile

Financial data and trends.

### Management Profile

Management and owner dependency.

### Ownership Profile

Current ownership.

### Risk / Attention Flags

Facts requiring further investigation.

### Data Quality Report

Completeness, consistency, freshness, provenance.

### Research Inputs

Structured context for the Research Engine.

### Scenario Inputs

Structured context for the Scenario Engine.

### Document Requirements

Information gaps sent to the Document Readiness Engine.

---

# 29. WHAT THIS ENGINE MUST NOT DO

The Business Reality Engine should not independently:

* Determine business value
* Recommend an ownership structure
* Determine tax consequences
* Approve financing
* Decide whether the transaction is viable
* Tell the owner what to do
* Change the owner's Destination
* Convert owner assumptions into facts
* Treat estimates as verified information

It establishes the **current-state evidence base**.

---

# 30. DATA MODEL

A conceptual structure:

```text id="f6m98k"
BusinessCurrentState
│
├── Company
│   ├── Identity
│   ├── Industry
│   ├── Geography
│   └── History
│
├── Financial
│   ├── Revenue
│   ├── EBITDA/Cash Flow
│   ├── Expenses
│   ├── Assets
│   ├── Debt
│   └── Trends
│
├── Workforce
│   ├── Employees
│   ├── Management
│   └── Ownership Participation
│
├── Operations
│   ├── Revenue Model
│   ├── Customers
│   ├── Suppliers
│   ├── Seasonality
│   └── Dependencies
│
├── Ownership
│   ├── Owners
│   ├── Percentages
│   └── Governance
│
├── Owner Dependency
│
├── Transaction Readiness
│
├── Evidence
│
├── Data Quality
│
└── Version History
```

---

# 31. FIELD-LEVEL PROVENANCE

Every important field should conceptually contain:

```text id="9i5xv5"
Field
├── Value
├── Unit
├── Source Type
├── Source ID
├── Date
├── Evidence Status
├── Confidence / Certainty
├── Verified By
├── Verified Date
└── Version
```

This is much more robust than storing:

`revenue = 8200000`

without knowing where the number came from.

---

# 32. USER EXPERIENCE RULE

The Business Reality Journey should follow the same product philosophy as the rest of the platform:

> **Ask only what is useful.**

For example:

The user says:

**I don't know my EBITDA.**

Don't stop.

Offer:

**Enter an estimate**

**Select financial statements**

**I'll do this later**

Then continue wherever possible.

---

# 33. CURRENT STATE COMPLETENESS

Do not create a simplistic score such as:

**Business Readiness = 83%**

Instead show:

### Current State

**Financial:** Good information

**Ownership:** Complete

**Management:** Preliminary

**Operations:** Moderate

**Documents:** Several outstanding

**Overall data quality:** Preliminary

This describes **information quality**, not transaction quality.

---

# 34. RELATIONSHIP WITH GOAL-TO-REALITY CONFIDENCE

The Business Reality Engine provides raw material.

The Confidence Engine decides how that information affects **Goal-to-Reality alignment**.

Example:

```text id="wrcp80"
Business Reality:
Debt = $3M
Owner goal:
$4M minimum at closing

        ↓

Confidence Engine

        ↓

Potential impact:
Closing proceeds assumption requires
additional analysis
```

The Business Reality Engine does not make that judgment.

---

# 35. CURRENT STATE → RESEARCH

Once the Business Reality Engine reaches sufficient completeness:

> **Research your assumptions**

becomes available.

The Research Engine can now investigate the owner's goals using:

**Desired Outcome**

*

**Business Current State**

without exposing unnecessary personal information.

---

# 36. CURRENT STATE → SCENARIO

The Scenario Engine receives:

**Desired Outcome**

*

**Current State**

*

**Relevant Research**

*

**Owner Constraints**

and develops potential pathways.

---

# 37. CURRENT STATE → DOCUMENT READINESS

Whenever the system identifies a meaningful gap:

**Current State needs information**

↓

Document Readiness Engine

↓

**Information/document requested**

This creates an adaptive document checklist.

---

# 38. CURRENT STATE → PROFESSIONAL REVIEW

The Professional Review Package can include:

### Current State Summary

**What we currently understand about the business**

Then distinguish:

**Owner reported**

**Document supported**

**Professionally reviewed**

This is exactly the information a professional needs to know when assessing the owner's Desired Outcome.

---

# 39. THE TWO-SNAPSHOT EXPERIENCE

Eventually the platform should provide:

# Where You Want to Go

**Desired Outcome**

versus:

# Where You Are Today

**Current State**

Then:

# What We Still Need to Learn

**Research / Information Gaps**

Then:

# What Paths Could Connect Them

**Platform Scenarios**

That's an exceptionally clean mental model.

---

# 40. NORTH STAR

The Business Reality Engine should ultimately allow the owner to say:

> **"This is what my business looks like today, and I can see how that compares with the destination I described."**

The engine should not tell them whether they can get there.

It supplies the factual foundation that allows:

**Research**

**Confidence**

**Scenario modeling**

and ultimately

**Professional Determination**

to do their respective jobs.

---

# 41. ARCHITECTURAL RULE

> **The Business Reality Engine describes reality. It does not judge the destination.**

That single sentence should govern the engine's design.

---

# 42. COMPLETE FLOW

```text id="m58vkl"
DESTINATION ENGINE
"What do I want?"
        ↓
BUSINESS REALITY ENGINE
"Where am I today?"
        ↓
RESEARCH ENGINE
"What does the evidence tell us?"
        ↓
EVIDENCE LEDGER
"What supports those findings?"
        ↓
CONFIDENCE ENGINE
"How well aligned are my goals with
the evidence currently available?"
        ↓
SCENARIO ENGINE
"What routes could potentially connect
my current reality to my destination?"
```

That is the clean architecture we want.

## The key addition is the **two-snapshot view**

I think this could become one of the most intuitive screens in the whole product:

### 🎯 Where You Want to Go

Your Destination

**$3–5M at closing**
**Employee ownership**
**$75–100K annual income**
**18-month transition**
**Retirement**

↓

### 🏢 Where You Are Today

Your Current Reality

**$8.2M revenue**
**$1.4M EBITDA**
**$2.2M debt**
**55 employees**
**Moderate owner dependency**
**Management team exists**

↓

### 🔎 What We Still Need to Understand

**Customer concentration**
**Management succession depth**
**Financing capacity**
**Ownership structure considerations**

↓

### 🛣️ Potential Routes

Scenario A
Scenario B
Scenario C

That gives the owner an incredibly intuitive mental model without pretending that the app already knows the answer.

And architecturally, **Business Reality becomes the factual counterweight to Destination**. Neither is allowed to overwrite the other. One says *“this is what I want,”* the other says *“this is what we currently know,”* and the Research, Confidence, and Scenario engines sit between them to explore the gap.
