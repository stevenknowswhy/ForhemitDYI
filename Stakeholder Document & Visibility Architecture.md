I think this is the right next layer. The application should **never create one universal “owner packet” and send it everywhere**. It should create a **source-of-truth Owner Destination/Plan**, then generate **stakeholder-specific views** from it.

That gives us three powerful controls:

**Who gets it → What they need → How much they can see**

And I would make the three disclosure levels a native part of the document engine.

# Stakeholder Document & Visibility Architecture

## 1. Core Principle

The platform should maintain one authoritative internal representation of the owner's:

* Desired Outcome / Destination
* Objectives
* Priorities
* Nonnegotiables
* Preferences
* Assumptions
* Business facts
* Platform scenarios
* Research findings
* Professional questions
* Professional determinations

From that source of truth, the platform generates **tailored stakeholder documents**.

The owner chooses:

**WHO receives information**

and

**HOW MUCH information they receive.**

The platform determines:

**WHAT information is relevant to that stakeholder's role.**

---

# 2. THREE VISIBILITY LEVELS

Every stakeholder document should offer:

## LEVEL 1: SUMMARY

### Purpose

Provide a concise overview of the owner's desired outcome and why they are exploring employee ownership.

### Contains

* Owner's broad objective
* Desired ownership outcome
* Desired timing
* Broad financial goals
* Major priorities
* Major nonnegotiables
* Key questions
* High-level scenarios being explored

### Does NOT normally contain

* Personal identifying information
* Personal financial identifiers
* Bank/account information
* Tax identifiers
* Employee-level data
* Customer-level data
* Detailed financial records
* Sensitive transaction documents

### Example

> **Owner wants to transition the company to employee ownership within approximately 18 months, receive substantial cash at closing, retain some post-closing income, and retire from day-to-day operations.**
>
> The owner is currently exploring several employee-ownership structures and financing approaches.
>
> The owner is seeking professional review of the feasibility and implications of these alternatives.

---

# 3. LEVEL 2: DETAILED

### Purpose

Give the stakeholder enough information to meaningfully evaluate the owner's objectives and current thinking.

### May contain

* More detailed owner objectives
* Ranked priorities
* Nonnegotiables
* Desired proceeds ranges
* Desired future-income ranges
* Desired ownership percentages
* Approximate revenue
* Approximate EBITDA/cash flow
* Approximate debt
* Employee count range
* Business characteristics
* Scenario assumptions
* Platform research findings
* Questions relevant to the stakeholder
* Relevant timeline information

### Normally minimize

* Personal identifying information
* Personal financial identifiers
* Employee-level information
* Customer-level information
* Unnecessary confidential information

The Detailed package should provide **decision context without becoming a complete data room**.

---

# 4. LEVEL 3: COMPREHENSIVE

### Purpose

Provide the stakeholder with the information they legitimately need to perform their professional or transaction role.

May include:

* Owner identity
* Business identity
* Detailed financial information
* Detailed transaction assumptions
* Relevant personal financial information
* Detailed ownership information
* Detailed employee information where appropriate
* Documents specifically relevant to the stakeholder
* Financial statements
* Tax documents
* Debt information
* Supporting records
* Other authorized confidential information

The Comprehensive package must remain **role-specific**.

"Comprehensive" does not mean:

> **Give this person everything we have.**

It means:

> **Give this stakeholder the complete information appropriate to their authorized role.**

---

# 5. IMPORTANT TERMINOLOGY

The system should distinguish:

### Personal Identifying Information

Examples:

* Name
* Address
* SSN
* Tax ID
* Bank account information
* Personal contact details

### Personal Financial Information

Examples:

* Personal income
* Personal assets
* Personal liabilities
* Personal tax information

### Business Financial Information

Examples:

* Revenue
* EBITDA
* Financial statements
* Debt
* Cash flow
* Working capital

### Business Confidential Information

Examples:

* Customer lists
* Employee information
* Contracts
* Pricing
* Trade secrets
* Proprietary processes

These are not all the same thing.

The document engine should use **data classification**, not simply a binary PII/non-PII switch.

---

# 6. STAKEHOLDER CATEGORIES

The first Employee Ownership journey should anticipate the following stakeholders.

## A. Owner

The owner has access to the complete internal record.

### Owner Master Plan

This is the owner's complete working record.

It can contain:

* Destination
* Objectives
* Constraints
* Assumptions
* Scenarios
* Research
* Evidence
* Questions
* Professionals
* Professional responses
* Version history
* Shared documents
* Transaction history

This is **not normally sent externally as-is**.

---

# 7. ATTORNEY

### Purpose

Understand what the owner wants legally and structurally and identify the legal questions that need to be addressed.

### Best document

**Legal & Transaction Objectives Brief**

Potential sections:

* Owner objectives
* Desired ownership outcome
* Timing
* Nonnegotiables
* Desired governance characteristics
* Structures being explored
* Seller-financing preferences
* Questions for counsel
* Relevant business facts
* Relevant professional feedback

Visibility:

**Summary → useful for initial discussion**

**Detailed → likely primary working brief**

**Comprehensive → when counsel needs supporting transaction information**

---

# 8. CPA / TAX PROFESSIONAL

### Purpose

Understand the owner's financial and tax-related objectives.

### Document

**Tax & Financial Objectives Brief**

Potential sections:

* Desired proceeds
* Desired income
* Timing
* Owner objectives
* Nonnegotiables
* Transaction structures being explored
* Seller financing preferences
* Relevant business financial information
* Tax-related questions
* Documents authorized for tax review

The platform should **not provide tax advice**.

It should turn the owner's goals into:

> **Questions and information for the tax professional.**

---

# 9. BUSINESS VALUATION PROFESSIONAL

### Purpose

Understand what the owner hopes to accomplish financially and provide the information necessary for professional valuation work.

### Document

**Valuation Objectives & Business Profile**

Potential sections:

* Desired closing proceeds
* Desired income
* Business history
* Revenue
* Cash flow / EBITDA
* Debt
* Ownership
* Industry
* Geographic context
* Owner dependence
* Relevant business characteristics
* Owner questions
* Requested scope of valuation

The owner should be able to say:

> "I want to understand what business value would be required to achieve my desired destination."

without the platform presenting its preliminary model as the valuation.

---

# 10. LENDER / FINANCING PROFESSIONAL

### Document

**Financing Objectives & Transaction Profile**

Potential sections:

* Purchase objective
* Desired closing proceeds
* Approximate purchase price assumptions
* Business financial profile
* EBITDA/cash flow
* Debt
* Financing preferences
* Seller financing preferences
* Desired ownership
* Management information
* Timing
* Relevant documents

The lender's Detailed and Comprehensive versions would likely contain substantially more financial information than a general stakeholder version.

---

# 11. ESOP PROFESSIONAL

### Document

**Employee Ownership Objectives Brief**

Potential sections:

* Desired employee ownership outcome
* Desired ownership breadth
* Management ownership preferences
* Timing
* Owner liquidity goals
* Owner transition goals
* Business profile
* Scenarios explored
* Questions
* Relevant professional findings

The document should communicate:

> **"This is the employee-ownership outcome the owner is trying to create."**

not:

> **"This is the ESOP structure the owner has decided upon."**

---

# 12. TRUSTEE

Where applicable, the trustee may need a highly specialized information package.

### Document

**Employee Ownership Transaction Review Package**

This should be generated specifically for the trustee's role.

It should include information necessary for the trustee to perform the appropriate professional/ fiduciary function.

Because trustee independence and role requirements can be highly specialized, the platform should allow the trustee to define or request additional information.

---

# 13. FINANCIAL ADVISOR / WEALTH ADVISOR

### Document

**Owner Liquidity & Transition Objectives Brief**

Potential focus:

* Cash-at-closing target
* Future income target
* Timing
* Retirement objectives
* Liquidity preferences
* Seller-note preferences
* Risk/preferences stated by owner
* Family/estate objectives where the owner chooses to include them

This professional may be helping the owner evaluate the owner's broader financial objectives rather than structuring the transaction itself.

---

# 14. SELLER-NOTE BUYER

This is especially important.

The note buyer should **not receive the same package as an attorney**.

### Document

**Seller Note Opportunity Profile**

Potential information:

* Note amount
* Purchase price / remaining balance
* Interest rate
* Term
* Amortization
* Payment history
* Security
* Priority
* Seller-financing structure
* Relevant business financial metrics
* Business industry
* Business size
* Geography
* Transaction background
* Contact information where authorized

The package should exclude unrelated personal information unless specifically required and authorized.

This should be generated directly from the **Seller Note object + authorized business data**, rather than simply copying the Owner Master Plan.

---

# 15. MANAGEMENT TEAM

This is an interesting stakeholder because management may become future owners.

### Document

**Management Ownership Transition Brief**

Potential content:

* High-level owner objectives
* Desired transition timeline
* Employee ownership goals
* Management participation objectives
* Broad financial structure
* Expected transition responsibilities
* Questions for management
* Information the owner wants management to understand

This version should generally avoid sensitive seller financial information unless the owner intentionally shares it.

---

# 16. EMPLOYEES / EMPLOYEE GROUP

The employee audience should usually receive a **much more limited document**.

### Document

**Employee Ownership Overview**

Potential information:

* Owner's intention to explore employee ownership
* General reason for the transition
* High-level desired outcome
* Expected exploration process
* What employees may eventually need to understand
* Questions employees can ask
* Current status

This should generally not expose:

* Seller's personal finances
* Purchase-price negotiations
* Seller-note terms
* Personal tax information
* Confidential employee data
* Customer information

This document is about **understanding the journey**, not revealing the transaction's private financial machinery.

---

# 17. BOARD / ADVISORY BOARD

Where applicable:

### Document

**Ownership Transition Strategic Brief**

Focus:

* Owner objectives
* Desired end state
* Business continuity goals
* Timing
* Key risks/questions
* Structures being explored
* Professional team
* Major decisions requiring review

---

# 18. BROKER / M&A ADVISOR

If the owner chooses to involve one:

### Document

**Transaction Objectives & Business Profile**

Focus:

* Desired outcome
* Sale objectives
* Ownership objectives
* Timing
* Financial profile
* Buyer preferences
* Deal structure preferences
* Professional team
* Questions for advisor

Again, this is an owner instruction/goal document, not an instruction to the advisor to adopt the platform's scenario as fact.

---

# 19. THE OWNER SHOULD CHOOSE THE RECIPIENT

The sharing workflow should be:

### Who would you like to share this with?

**My attorney**

**My CPA**

**My valuation professional**

**My lender**

**My ESOP professional**

**My financial advisor**

**Someone else**

Then:

### How much should they see?

**Summary**

**Detailed**

**Comprehensive**

Then:

### What should this package focus on?

The platform can automatically tailor the package based on stakeholder role, with the owner able to modify the selection.

---

# 20. OWNER OVERRIDE

The owner should be able to adjust the package before sending.

Example:

### Attorney Package

Included:

✓ Owner objectives
✓ Nonnegotiables
✓ Desired ownership
✓ Timing
✓ Scenarios
✓ Legal questions

Optional:

☐ Financial assumptions
☐ Seller-note information
☐ Research findings
☐ Supporting documents

The owner makes the final sharing decision.

---

# 21. STAKEHOLDER-SPECIFIC DATA FILTERING

The document engine should not merely hide sections visually.

It should construct the package from **authorized data fields**.

For example:

```text id="25ex1h"
SOURCE OF TRUTH
       ↓
Stakeholder = CPA
       ↓
Visibility = Detailed
       ↓
CPA Data Policy
       ↓
Allowed Fields
       ↓
Document Generator
       ↓
CPA Review Package
```

Different stakeholder roles get different data policies.

---

# 22. DOCUMENT VISIBILITY MATRIX

The platform should maintain a configurable matrix.

| Information            |    Summary |          Detailed |          Comprehensive |
| ---------------------- | ---------: | ----------------: | ---------------------: |
| Owner objectives       |          ✓ |                 ✓ |                      ✓ |
| Desired ownership      |          ✓ |                 ✓ |                      ✓ |
| Nonnegotiables         |          ✓ |                 ✓ |                      ✓ |
| Desired proceeds range |      Broad |                 ✓ |                      ✓ |
| Future income goal     |      Broad |                 ✓ |                      ✓ |
| Approx. revenue        | No / broad |                 ✓ |                      ✓ |
| EBITDA / cash flow     |         No |           Approx. |               Detailed |
| Debt                   |         No |           Approx. |               Detailed |
| Personal identifiers   |         No |        Usually no |         Role-dependent |
| Tax documents          |         No |                No |             Authorized |
| Employee-level data    |         No |           Limited |         Role-dependent |
| Customer information   |         No |                No |         Role-dependent |
| Contracts              |         No |        Usually no |          Relevant only |
| Platform scenarios     |    Summary |          Detailed |               Detailed |
| Research evidence      | Highlights | Relevant findings | Full relevant evidence |
| Supporting documents   |         No |          Selected |          Role-specific |

The matrix should be configurable rather than permanently hard-coded.

---

# 23. EACH DOCUMENT SHOULD HAVE A PURPOSE STATEMENT

Every generated document should start with:

### Purpose of This Package

> This package summarizes the owner's objectives and current exploration for the purpose of professional/stakeholder review.

Then:

### Information Level

**Summary / Detailed / Comprehensive**

Then:

### Intended Recipient

**Attorney**

Then:

### Prepared

**Date**

### Destination Version

**Version 3**

### Journey Version

**Employee Ownership Journey v1.0**

This provides provenance.

---

# 24. DOCUMENTS SHOULD SHOW WHAT IS NOT INCLUDED

This is useful for trust.

For example:

> **This package does not include personal financial information, tax returns, employee-level information, or customer information.**

Or:

> **This Comprehensive package includes the documents specifically authorized for this recipient.**

The recipient should understand the boundaries.

---

# 25. EVERY DOCUMENT SHOULD BE TRACEABLE TO THE OWNER'S DESTINATION

For example:

### Owner Goal

Receive $3M–$5M at closing.

### Scenario

Three potential financing/ownership paths were explored.

### Question for this stakeholder

Can any of these structures reasonably support the owner's stated objective?

That means the professional can immediately see:

**Goal → Scenario → Question**

rather than receiving disconnected information.

---

# 26. PROFESSIONAL FEEDBACK RETURNS TO THE PLATFORM

A recipient should eventually be able to submit:

### Professional Feedback

**Supports**

**Needs Modification**

**Not Feasible Under Current Assumptions**

**Needs More Information**

**Explore Alternative**

The platform stores this as **Professional Determination / Professional Feedback**, never as an AI-generated conclusion.

---

# 27. DOCUMENT VERSIONING

Every package should be versioned.

Example:

**Attorney Package v1.0**

Owner Destination v2

Scenario v3

Generated September 2026

Then:

**Attorney Package v1.1**

Updated after owner changed the closing target.

The recipient sees what changed.

---

# 28. SHARE CONTROL

Every package should support:

**View**

**Download**

**Share**

**Revoke**

**Expire**

**Replace with New Version**

The owner should be able to see:

> **Who has access to what**

at any time.

---

# 29. LOCAL-FIRST DOCUMENT GENERATION

The source documents should remain local by default.

The application can generate a **sanitized stakeholder package** locally.

The owner reviews it.

Only after explicit authorization does it get:

**Shared**

with the selected recipient.

This aligns the document system with the platform's privacy architecture.

---

# 30. THE "ONE SOURCE, MANY VIEWS" MODEL

The central design principle is:

```text id="x6eqtt"
                  OWNER MASTER RECORD
                         │
          ┌──────────────┼───────────────┐
          │              │               │
          ▼              ▼               ▼
      ATTORNEY          CPA          VALUATION
      PACKAGE          PACKAGE         PACKAGE
          │              │               │
          ▼              ▼               ▼
        LENDER        ESOP/TRUSTEE     NOTE BUYER
        PACKAGE          PACKAGE        PACKAGE
          │              │               │
          └──────────────┼───────────────┘
                         ▼
                  MANAGEMENT / EMPLOYEE
                         PACKAGE
```

All packages come from the same Owner Destination and current plan.

But **each package is purpose-built.**

---

# 31. NEW STANDALONE ENGINE

This suggests another standalone engine:

# Stakeholder Document Engine

Its job is to:

1. Identify recipient role.
2. Determine applicable information.
3. Apply requested visibility level.
4. Apply authorization rules.
5. Generate stakeholder-specific content.
6. Identify included/excluded information.
7. Version the document.
8. Record what was shared.
9. Track revocation/expiration.
10. Accept professional feedback.

The engine should be independent from the Journey Engine and Scenario Engine.

---

# 32. STAKEHOLDER PROFILE ENGINE

We should also have a separate:

# Stakeholder Profile Engine

Each stakeholder type has:

* Role
* Information needs
* Data sensitivity
* Typical documents
* Default visibility level
* Allowed fields
* Optional fields
* Professional review questions
* Output template

For example:

**CPA**

Default: Detailed

**Employee**

Default: Summary

**Attorney**

Default: Detailed

**Note Buyer**

Default: Detailed

**Trustee**

Potentially Comprehensive where authorized

These are defaults, not automatic disclosure rules.

---

# 33. THE OWNER ALWAYS CONTROLS DISCLOSURE

The platform can recommend:

> **Recommended level: Detailed**

because that is typically enough for this professional's initial review.

But the owner can choose:

**Summary**

**Detailed**

**Comprehensive**

subject to situations where a professional or transaction process genuinely requires specific information.

The application should explain those requirements rather than silently sending information.

---

# 34. PROFESSIONAL DOCUMENT PACKAGES SHOULD BE ACTION-ORIENTED

The package should not simply dump data.

Every document should answer:

### What is the owner trying to accomplish?

### What matters most?

### What is nonnegotiable?

### What has been explored?

### What does the owner want this professional to review?

### What decisions are still open?

That keeps the document useful.

---

# 35. DOCUMENT DESIGN NORTH STAR

Every stakeholder should receive:

> **The least amount of information necessary to understand their role and take the next useful action, with the owner explicitly controlling disclosure.**

Not:

> "Give everyone a copy of everything."

---

# 36. ARCHITECTURAL BOUNDARY SUMMARY

The Stakeholder / Relationship Engine owns who participates in the transaction and what each
stakeholder may see. Its three visibility levels — Summary, Detailed, Comprehensive — are not
document lengths but **data-permission policies**: the owner controls disclosure explicitly, and
every stakeholder package is generated from the Destination and Owner Master Record as the single
source of truth.

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Stakeholder / Relationship** | Who participates in the transaction and what each stakeholder may see — the three visibility levels (Summary / Detailed / Comprehensive) expressed as data-permission policies | The authorization decision that permits access (Consent & Access); the identities themselves (Identity & Access); the source content it translates |
| **Consent & Access** | The authorization decision that permits a stakeholder to see specific data | The visibility levels Stakeholder defines as policies |
| **Identity & Access** | Who a stakeholder is and their authentication / session | What each stakeholder may see once admitted |
| **Destination** | The owner's stated intent that stakeholder documents translate | The stakeholder-facing views Stakeholder generates |
| **Professional Review** | Review of a stakeholder package before it is shared | The package content Stakeholder assembles |
| **Professional Determination** | The professional's conclusion surfaced to stakeholders | The stakeholder views Stakeholder presents |
| **Communication** | Delivery of stakeholder documents to the right people | Which people may see what — Stakeholder / Relationship defines that |
| **Document Readiness** | Whether documents are complete and ready to share | The visibility policy that gates sharing |
| **Transaction / Orchestration** | The transaction the stakeholders participate in | The stakeholder relationships within it (Stakeholder owns those) |
| **Audit / Provenance** | The record of who saw what and when | The visibility decision Stakeholder / Relationship defines |

## Hard Boundary

> The Stakeholder / Relationship Engine owns **who participates and what they may see**. It does
> not own the authorization decision (Consent & Access), the identities (Identity & Access), or
> the source content it translates from the Destination and Owner Master Record. The three
> visibility levels are data-permission policies, not document lengths — the owner controls
> disclosure explicitly.

---

# 37. FINAL PRODUCT MODEL

The platform now has another clean chain:

**Owner Destination**

↓

**Owner Objectives**

↓

**Platform Scenarios**

↓

**Research & Evidence**

↓

**Stakeholder-Specific Document Engine**

↓

**Summary / Detailed / Comprehensive**

↓

**Owner Approval**

↓

**Secure Sharing**

↓

**Professional Review**

↓

**Professional Determination**

The application becomes a **controlled information translator**.

The owner says:

> **"This is where I want to go."**

The platform translates that into the language and information requirements of each person who can help determine how to get there.

### I would lock in two ideas here

**First: “one source, many views.”** We should never maintain separate versions of the owner's goals manually for different people. The **Destination and Owner Master Record** are the source of truth. Every stakeholder package is generated from them.

**Second: the three visibility levels are not merely document lengths.** They're **data-permission policies**. Summary, Detailed, and Comprehensive should determine what categories of information the document engine is even allowed to pull into that package.

That gives us a very strong privacy model:

> **The owner doesn't ask, “Which 20-page document should I send?”**

They ask:

> **“Who needs to know, and how much should they know?”**

And the platform handles the rest. That is exactly the sort of complexity the software should hide.
