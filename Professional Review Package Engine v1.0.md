Exactly. This is the engine that turns the platform's internal information into **the right package for the right person at the right disclosure level**.

The key architectural principle is:

> **One source of truth, many purpose-built packages.**

The Professional Review Package Engine should never create a generic "dump everything we know" report. It should understand the recipient's role, the purpose of the review, the current journey state, and the owner's chosen disclosure level.

# Professional Review Package Engine v1.0

## Stakeholder-Specific Professional Review Packages

# 1. Purpose

The Professional Review Package Engine is a standalone engine responsible for transforming the owner's structured journey information into **purpose-built review packages for specific professionals and stakeholders**.

Its purpose is to answer:

> **"What does this particular person need to understand to help the owner move toward their desired outcome?"**

The engine creates tailored packages for:

* Attorneys
* CPAs / tax professionals
* Valuation professionals
* Lenders
* ESOP professionals
* Trustees
* Financial advisors
* Management
* Employees
* Seller-note buyers
* Other stakeholders

Every package can use one of three disclosure levels:

**Summary**

**Detailed**

**Comprehensive**

The engine must preserve owner control over sharing.

---

# 2. CORE PRINCIPLE

The Professional Review Package Engine does not create a second source of truth.

It pulls from the platform's authoritative objects:

```text
Destination
Business Reality
Research Findings
Evidence
Platform Scenarios
Capital Plans
Seller Notes
Document Readiness
Professional Feedback
Professional Determinations
Owner Decisions
```

Then creates a stakeholder-specific view.

```text
                  SOURCE OF TRUTH
                         │
       ┌─────────────────┼──────────────────┐
       │                 │                  │
       ▼                 ▼                  ▼
    Attorney            CPA              Lender
    Package            Package           Package
       │                 │                  │
       ▼                 ▼                  ▼
    Valuation          Trustee          Note Buyer
    Package            Package           Package
```

One underlying reality.

Many tailored views.

---

# 3. PACKAGE OBJECT

Create:

## `ProfessionalReviewPackage`

It should contain:

* Package ID
* Owner
* Business
* Journey
* Destination version
* Current State version
* Scenario version
* Stakeholder
* Organization
* Professional role
* Purpose
* Visibility level
* Included information
* Excluded information
* Included documents
* Owner authorization
* Package version
* Creation date
* Expiration
* Sharing status
* Review status
* Change history

---

# 4. THREE DISCLOSURE LEVELS

## LEVEL 1: SUMMARY

### Purpose

Give the stakeholder a concise understanding of the owner's objectives and the reason they are involved.

### Typical content

* Desired Outcome
* Primary objective
* Major priorities
* Major nonnegotiables
* Broad timing
* Broad ownership goals
* High-level scenarios
* Main questions for the stakeholder

### Normally exclude

* Personal identifiers
* Personal financial details
* Detailed financials
* Employee-level data
* Customer data
* Sensitive business documents

### Example

> The owner wants to transition the business to employee ownership within approximately 18 months, receive substantial cash at closing, maintain income after the transition, and retire from day-to-day operations.

---

# 5. LEVEL 2: DETAILED

### Purpose

Give the stakeholder enough information to conduct a meaningful preliminary review.

### Typical content

* Detailed Desired Outcome
* Ranked priorities
* Nonnegotiables
* Strong preferences
* Desired financial ranges
* Ownership objectives
* Approximate business financial profile
* Employee count range
* Debt range
* Management information
* Relevant scenario assumptions
* Relevant research findings
* Questions for this stakeholder
* Relevant document references
* Selected supporting documents

### Normally minimize

* Personal identifiers
* Personal financial information
* Employee-level records
* Customer-level information
* Unrelated business documents

---

# 6. LEVEL 3: COMPREHENSIVE

### Purpose

Provide the stakeholder with the complete information appropriate to their authorized role.

This may include:

* Owner identity
* Business identity
* Detailed financial information
* Detailed business information
* Detailed transaction assumptions
* Supporting documents
* Relevant personal financial information
* Detailed ownership information
* Other sensitive information specifically authorized for the recipient

Comprehensive must still be **role-specific**.

It does not mean:

> **Everything in the owner's vault.**

It means:

> **Everything this authorized recipient legitimately needs for the stated purpose.**

---

# 7. VISIBILITY IS DATA POLICY, NOT DOCUMENT LENGTH

The three levels should not simply produce:

**3 pages / 10 pages / 30 pages.**

Instead, each level represents a **data-access policy**.

For example:

### Summary

May include:

**"Revenue: $5M–$10M"**

### Detailed

May include:

**"Revenue: $8.2M, FY2025"**

### Comprehensive

May include:

**Full financial statements and related supporting documents**

This approach makes disclosure meaningful.

---

# 8. STAKEHOLDER PROFILE

Every package is generated using a:

## `StakeholderProfile`

It defines:

* Role
* Purpose
* Relevant information categories
* Relevant questions
* Default visibility
* Allowed data
* Restricted data
* Typical documents
* Typical output structure

Examples:

### Attorney

Focus:

* Legal/transaction objectives
* Ownership
* Governance preferences
* Transaction assumptions
* Relevant contracts/documents

### CPA

Focus:

* Financial objectives
* Tax questions
* Financial history
* Seller financing
* Relevant financial records

### Lender

Focus:

* Financing request
* Cash flow
* Debt
* Transaction structure
* Management
* Relevant financial documentation

---

# 9. PACKAGE TYPES

The engine should have reusable templates.

Examples:

### Legal & Transaction Objectives Package

### Tax & Financial Objectives Package

### Valuation Review Package

### Financing Review Package

### Employee Ownership Review Package

### Trustee Review Package

### Owner Liquidity Package

### Management Transition Package

### Employee Ownership Overview

### Seller-Note Opportunity Package

### General Stakeholder Brief

The template controls the **purpose and structure**.

The actual contents are populated dynamically.

---

# 10. PACKAGE PURPOSE

Every package must have an explicit purpose.

Example:

> **Purpose**
>
> Provide the attorney with the owner's stated objectives, current assumptions, and selected scenarios for preliminary legal and transaction review.

A different package might say:

> **Purpose**
>
> Provide the lender with the information necessary to evaluate the owner's financing request and identify additional underwriting requirements.

Purpose determines relevance.

---

# 11. PACKAGE SHOULD START WITH THE DESTINATION

The first substantive section should usually be:

# What the Owner Is Trying to Accomplish

This includes:

* Desired outcome
* Financial goals
* Ownership goals
* Personal goals
* Business preservation goals
* Nonnegotiables
* Preferences

That keeps the professional focused on:

> **Why does this transaction exist?**

before:

> **How are we structuring it?**

---

# 12. OWNER OBJECTIVE SECTION

Every professional package should contain some version of:

## Owner Objectives

What the owner wants.

Example:

**Primary**

Employee ownership

**Secondary**

Retire within 18 months

**Financial**

$3M–$5M at closing

**Future income**

$75K–$100K/year for 5–10 years

---

# 13. NONNEGOTIABLES SECTION

Nonnegotiables should receive prominent treatment.

# Owner's Must-Haves

🔴 Employee ownership

🔴 Minimum $3M at closing

🔴 Retirement within 18 months

The package should distinguish:

**Must-have**

from:

**Strong preference**

from:

**Flexible**

---

# 14. OWNER'S AVOIDANCES

Where relevant:

# What the Owner Wants to Avoid

Examples:

* Long-term operational involvement
* Loss of employee ownership
* Excessive debt
* Major workforce disruption
* Long transition

This gives professionals an important view of the owner's priorities.

---

# 15. CURRENT BUSINESS STATE

Packages should include only the Business Reality information relevant to the recipient.

For example:

### Valuation Professional

* Revenue
* EBITDA
* Historical trend
* Assets
* Debt
* Industry
* Business model

### Employee Stakeholder

Likely:

* Company size
* General business description
* Ownership objective
* Transition goals

Not:

* Seller's personal financial information

---

# 16. PLATFORM SCENARIOS

The package should clearly label:

# Scenarios Explored

For each:

* Scenario name
* Description
* Assumptions
* Owner interest
* Trade-offs
* Goal alignment observations
* Open questions

Example:

> **Scenario A: Employee Ownership + Seller Financing**
>
> This is an exploratory platform scenario based on the assumptions listed below.

It must never appear as:

> **Recommended Structure**

unless that statement is directly attributable to a professional and is displayed as professional feedback.

---

# 17. RESEARCH FINDINGS

Relevant external research can be included.

But the package should distinguish:

### Platform Research

from:

### Professional Determination

Example:

> **Platform research identified the following current information relevant to this scenario.**

Then sources.

The professional can decide what that information means for the transaction.

---

# 18. EVIDENCE REFERENCES

Where a research finding materially matters, the package can include:

* Source
* Publication date
* Research date
* Scope
* Applicability
* Key finding
* Link/reference

Summary packages may contain:

**Key finding + source**

Detailed packages can contain:

**Relevant evidence**

Comprehensive packages can include:

**Full applicable evidence set**

---

# 19. GOAL-TO-REALITY STATUS

The package can include the latest Goal Alignment assessment when relevant.

Example:

### Current Goal Alignment

**Moderate**

### Evidence Quality

**Good**

### Key areas requiring review

* Closing proceeds
* Financing assumptions
* Future income

The package must state that this is a platform assessment, not a professional determination.

---

# 20. QUESTIONS FOR THE PROFESSIONAL

This should be one of the most valuable sections.

### Questions for You

1. Can this owner's stated objective be pursued under the contemplated structures?
2. What assumptions need to be validated?
3. What additional information do you need?
4. What alternatives should we investigate?

Questions should be generated from:

* Owner objectives
* Research gaps
* Scenario issues
* Document gaps
* Professional role

---

# 21. DOCUMENT REFERENCES

The package can show:

### Supporting Documents

**Available locally**

2025 Financial Statements

**Shared**

2024 Tax Return

**Outstanding**

Debt Schedule

The owner decides what actually gets attached or shared.

---

# 22. DOCUMENT ATTACHMENTS

The package should distinguish:

**Referenced**

> "2025 Financial Statements are available."

from:

**Attached**

> The document is included in this package.

from:

**Shared Separately**

> The professional has controlled access through the platform.

This prevents unnecessary duplication.

---

# 23. OWNER APPROVAL BEFORE SHARING

Every package must pass through:

# Review Before Sharing

The owner sees:

### Recipient

Jane Smith
Smith & Jones LLP
Employee Ownership Attorney

### Visibility

**Detailed**

### Included

✓ Desired Outcome
✓ Nonnegotiables
✓ Scenario A
✓ Scenario B
✓ Relevant business information
✓ Legal questions

### Not Included

✗ Tax returns
✗ Employee-level information
✗ Customer data
✗ Personal financial information

### Actions

**Approve & Share**

**Edit Package**

**Change Visibility**

**Cancel**

---

# 24. OWNER CAN CHANGE VISIBILITY

At any time before sharing:

**Summary**

→ **Detailed**

or:

**Detailed**

→ **Comprehensive**

The platform recalculates the allowed information set.

---

# 25. OWNER CAN REMOVE INFORMATION

Even within a Detailed package, the owner should be able to remove optional categories where appropriate.

For example:

☑ Business financial summary

☐ Family/estate objectives

☐ Personal financial information

☑ Employee ownership goals

This creates true owner control.

---

# 26. PACKAGE GENERATION RULES

The engine should apply:

### Relevance

Does this information matter to the stakeholder?

### Sensitivity

How sensitive is it?

### Visibility

What level was selected?

### Authorization

Has the owner authorized it?

### Purpose

Why is this stakeholder receiving the package?

### Freshness

Is the information current enough to include?

### Provenance

Can we identify its source?

---

# 27. NO UNATTRIBUTED AI CLAIMS

If the package contains a statement produced by platform analysis, label it.

Examples:

**Owner reported**

**Document-supported**

**Platform research**

**Platform scenario**

**Professional feedback**

**Professional determination**

The engine must never blur these categories.

---

# 28. PROFESSIONAL DETERMINATIONS

Once the professional responds, the package can incorporate:

# Professional Review

### Attorney

Reviewed September 2026

### Professional Feedback

[Attributed statement]

### Professional Determination

[Attributed determination]

This is never presented as:

**Platform recommendation.**

---

# 29. PACKAGE VERSIONING

Every package has a version.

Example:

### Attorney Package v1

Destination v2
Scenario v4

Then owner changes the destination.

### Attorney Package v2

Destination v3
Scenario v6

The old package remains archived.

---

# 30. CHANGE SUMMARY

When a new version is created:

# What Changed?

**Closing target:** $3M–$5M → $2.5M–$5M

**Timing:** 18 months → 24 months

**Seller financing:** Possible → Preferred

This allows professionals to review changes quickly.

---

# 31. PACKAGE EXPIRATION

A package can have:

* Created date
* Validity period
* Expiration
* Superseded date

This helps prevent someone from relying on an obsolete package.

---

# 32. RECIPIENT ACCESS

The package should integrate with the Consent & Access Engine.

Recipient permissions include:

**View**

**Download**

**Print**

**Comment**

**Request Information**

where supported by platform policy.

The package itself does not decide access.

---

# 33. SHARE HISTORY

The owner can see:

# Shared Packages

### CPA

Detailed
Shared Sept. 19

### Attorney

Summary
Shared Sept. 20

### Lender

Detailed
Shared Sept. 21

This becomes part of the owner's transaction history.

---

# 34. PACKAGE REVOCATION

The owner can:

**Revoke access**

**Expire package**

**Replace with newer version**

**Stop sharing**

The Access Engine handles actual permission changes.

---

# 35. STAKEHOLDER-SPECIFIC DEFAULTS

The engine can establish defaults.

Example:

| Stakeholder       | Suggested Default                             |
| ----------------- | --------------------------------------------- |
| Attorney          | Detailed                                      |
| CPA               | Detailed                                      |
| Valuation         | Detailed                                      |
| Lender            | Detailed                                      |
| ESOP Professional | Detailed                                      |
| Trustee           | Detailed / Comprehensive, situation-dependent |
| Financial Advisor | Detailed                                      |
| Management        | Summary                                       |
| Employees         | Summary                                       |
| Note Buyer        | Detailed                                      |

These are **workflow defaults**, not automatic disclosure rules.

The owner can adjust them.

---

# 36. ROLE-SPECIFIC CONTENT

## Attorney Package

Emphasize:

* Owner objectives
* Ownership
* Governance
* Timing
* Structures
* Legal questions
* Relevant legal documents

## CPA Package

Emphasize:

* Financial goals
* Income
* Proceeds
* Transaction assumptions
* Tax questions
* Financial records

## Valuation Package

Emphasize:

* Business profile
* Financial history
* Revenue
* EBITDA
* Debt
* Assets
* Valuation objectives

## Lender Package

Emphasize:

* Financing request
* Business financials
* Debt
* Cash flow
* Management
* Capital plan
* Financing questions

## ESOP Professional Package

Emphasize:

* Employee ownership objective
* Desired ownership breadth
* Owner liquidity
* Timing
* Business profile
* Structure questions

## Trustee Package

Emphasize:

* Applicable transaction structure
* Owner objectives
* Relevant transaction facts
* Valuation information
* Financing
* Documents required for trustee review
* Professional questions

## Financial Advisor Package

Emphasize:

* Liquidity goals
* Future income
* Retirement objectives
* Risk preferences
* Proceeds
* Seller-note objectives

## Management Package

Emphasize:

* Transition objectives
* Leadership role
* Employee ownership goals
* Timeline
* Expected responsibilities

## Employee Package

Emphasize:

* High-level owner objective
* General employee ownership vision
* Expected transition process
* Current status
* What employees may need to know next

Avoid unnecessary confidential financial information.

## Seller-Note Buyer Package

Emphasize:

* Note characteristics
* Payment history
* Business characteristics
* Security
* Priority
* Remaining term
* Seller's liquidity objective where relevant
* Authorized supporting documents

---

# 37. OTHER STAKEHOLDERS

The system must support custom stakeholder profiles.

For example:

**Insurance Professional**

**Escrow Provider**

**Benefits Advisor**

**Board Member**

**Economic Development Organization**

The administrator creates a:

**Stakeholder Profile**

that defines its information requirements.

---

# 38. TEMPLATE ENGINE

The system should support reusable templates.

Each template contains:

```text id="rdo6pj"
Stakeholder Type
       ↓
Package Purpose
       ↓
Required Sections
       ↓
Optional Sections
       ↓
Data Policies
       ↓
Document Policies
       ↓
Visibility Defaults
       ↓
Question Templates
```

This allows the platform to add new professional categories without rebuilding the package system.

---

# 39. CUSTOMIZATION WITHOUT BREAKING STANDARDIZATION

Professionals should be able to request:

**"Add this section."**

But that should be handled as a structured request.

The owner decides whether to provide the information.

The platform should not allow every recipient to arbitrarily demand the owner's entire record.

---

# 40. PROFESSIONAL REQUESTS CAN CREATE PACKAGE UPDATES

For example:

> "Please provide the detailed debt schedule."

The Document Readiness Engine receives the request.

The package engine can produce:

### Attorney Package v2

with the newly authorized information.

This preserves the relationship between:

**Professional Request → Document → Package Version.**

---

# 41. PACKAGE READINESS CHECK

Before sharing:

### Package Status

**Owner objectives:** Complete

**Destination:** Complete

**Scenario:** Complete

**Questions:** Complete

**Documents:** 2 outstanding

**Authorization:** Complete

### Result

**Ready to Share**

or:

**Ready with 2 outstanding items**

The system explains the gaps.

---

# 42. AUTOMATED QUALITY CHECK

Before generating a package, run:

### Completeness

Are required fields present?

### Consistency

Are there unresolved conflicts?

### Freshness

Are critical facts current?

### Provenance

Do material claims have sources?

### Visibility

Does any included information exceed the selected level?

### Authorization

Has the owner approved sharing?

### Role Fit

Is every included section relevant to the recipient?

---

# 43. RED FLAGS BEFORE SHARING

The engine should stop and ask for review when it detects:

### Sensitive information unexpectedly included

### Unresolved factual conflict

### Expired document

### Outdated scenario

### Professional determination incorrectly represented as platform analysis

### Data exceeding recipient visibility policy

The owner can then review.

---

# 44. PACKAGE READABILITY

The package should be useful to a busy professional.

It should begin:

# Executive Summary

then:

# Owner's Destination

then:

# Current Business Reality

then:

# Scenarios Explored

then:

# Questions for You

then:

# Supporting Information

This allows the professional to understand the story quickly and drill into details only where necessary.

---

# 45. PACKAGE NARRATIVE STYLE

Narrative should remain factual.

Prefer:

> "The owner has identified $3M as the minimum proceeds target."

rather than:

> "The owner deserves at least $3M."

Prefer:

> "The platform identified three scenarios for further investigation."

rather than:

> "The platform recommends Scenario A."

---

# 46. RESEARCH DISCLAIMER

When research is included:

> **Platform Research**
>
> The information in this section was assembled by the platform from external sources for exploratory planning. It is not legal, tax, investment, valuation, financing, fiduciary, or other professional advice.

---

# 47. SCENARIO DISCLAIMER

When scenarios are included:

> **Platform Scenario**
>
> This scenario reflects the owner's stated objectives and the assumptions currently entered into the platform. It is exploratory and has not been presented as a professional determination.

---

# 48. PROFESSIONAL DETERMINATION DISPLAY

When a professional responds:

> **Professional Determination**
>
> **Jane Smith, Smith & Jones LLP**
>
> [Professional's own statement]
>
> Date reviewed: September 2026

The platform should preserve the professional's attribution and wording.

---

# 49. PACKAGE COMPARISON

The engine should eventually let the owner see:

### What did I send each person?

For example:

**Attorney:** Detailed

**CPA:** Comprehensive

**Employees:** Summary

This makes information sharing transparent.

---

# 50. PACKAGE GENERATION EVENT

The engine should emit:

```text id="jz4yzs"
PackageCreated
PackageUpdated
PackageApproved
PackageShared
PackageRevoked
PackageExpired
PackageSuperseded
PackageRequestedByProfessional
```

Other engines can subscribe.

---

# 51. ENGINE INPUTS

The Professional Review Package Engine receives:

**Destination Engine**

Desired outcome and owner objectives.

**Business Reality Engine**

Current-state business information.

**Research Engine / Evidence Ledger**

Relevant external evidence.

**Scenario Engine**

Explored scenarios.

**Capital Engine**

Financing information.

**Seller-Note Engine**

Relevant note information.

**Document Readiness Engine**

Document status and requirements.

**Professional Review Engine**

Professional questions and feedback.

**Stakeholder Engine**

Recipient identity and role.

**Consent & Access Engine**

Authorization rules.

---

# 52. ENGINE OUTPUTS

It produces:

### Stakeholder Package

Purpose-built document/view.

### Package Sections

Relevant information grouped by purpose.

### Disclosure Manifest

Exactly what categories are included.

### Exclusion Manifest

What was intentionally excluded.

### Version

Traceable package version.

### Sharing Record

What was sent, to whom, and when.

### Review Package Status

Draft / ready / shared / superseded.

---

# 53. WHAT THIS ENGINE DOES NOT DO

It does not:

* Decide what the owner should do
* Provide professional advice
* Create legal agreements
* Make tax determinations
* Make investment recommendations
* Determine financing approval
* Alter owner objectives
* Invent facts
* Override privacy permissions
* Decide which professional the owner should select

---

# 54. THE MOST IMPORTANT RULE

> **The package should contain the minimum information necessary for the recipient to understand their role and take the next useful action, unless the owner explicitly authorizes additional information.**

This should be the governing disclosure principle.

---

# 55. END-TO-END EXAMPLE

### Owner's Destination

> Receive $3M–$5M at closing, receive $75K–$100K annually for 5–10 years, transition ownership to employees and management, and retire within 18 months.

### Scenario

> ESOP + third-party financing + seller financing.

### Attorney Package

Focuses on:

* Desired ownership
* Governance
* Transaction structure
* Seller financing
* Legal questions

### CPA Package

Focuses on:

* Cash
* Future income
* Financial assumptions
* Tax questions
* Relevant financial information

### Lender Package

Focuses on:

* Financing request
* Cash flow
* Debt
* Capital structure
* Financial documentation

### ESOP Professional Package

Focuses on:

* Employee ownership objective
* Desired ownership breadth
* Owner liquidity
* Timing
* Business profile

### Employee Package

Focuses on:

* Intent to explore employee ownership
* High-level desired outcome
* General process
* What happens next

Same owner.

Same Destination.

Different packages.

---

# 56. ARCHITECTURAL NORTH STAR

The engine should function as a:

# Controlled Information Translator

It translates:

**Owner Thinking**

into:

**Professional Context**

without changing the owner's meaning.

---

# 57. ONE-SENTENCE DEFINITION

> **The Professional Review Package Engine converts the owner's evolving destination, business reality, scenarios, research, questions, and authorized documents into versioned, stakeholder-specific packages at Summary, Detailed, or Comprehensive visibility levels, ensuring each recipient receives the information relevant to their role without unnecessarily exposing information they do not need.**

This engine gives us a really clean architecture because it **doesn't own the underlying information**. It assembles views of it.

The important chain now becomes:

**Destination** = what the owner wants
**Business Reality** = what we know today
**Scenario** = what we're exploring
**Research/Evidence** = what supports or challenges assumptions
**Professional Review Package** = what *this particular person* needs to see
**Professional Review** = what they say
**Professional Determination** = what they conclude

And I particularly like the **Exclusion Manifest** concept. The package shouldn't just know what it sent. It should know what it *deliberately did not send*. That creates a much stronger privacy and audit story:

> **"CPA received these 14 information categories. Employee group received these 5. Customer data was excluded. Personal financial information was excluded."**

That's the kind of infrastructure that will make this platform feel trustworthy rather than merely clever.
