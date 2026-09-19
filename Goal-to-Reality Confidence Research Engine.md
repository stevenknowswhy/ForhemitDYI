# Goal-to-Reality Confidence Research Engine

## Locked Product Architecture

### Purpose

The **Goal-to-Reality Confidence Engine** continuously examines whether the owner's stated:

* Desired Outcome
* Objectives
* Preferences
* Nonnegotiables
* Assumptions
* Business information
* Scenario parameters

appear reasonably aligned with current external evidence, industry benchmarks, transaction norms, and other relevant information.

Its purpose is to help the owner:

**Catch unrealistic assumptions early.**

**Identify missing information.**

**Understand where additional research is needed.**

**Improve the quality of their Professional Review Package.**

**Enter conversations with professional advisors better prepared.**

It is **not** a prediction engine, professional advisor, valuation opinion, financing approval, tax analysis, legal determination, investment recommendation, or guarantee of transaction success.

---

# 1. PERSISTENT TOP-BAR INDICATOR

The application should maintain a visible status in the top navigation.

### Example

**Goal Alignment: 🟡 Moderate**

or:

**Goal-to-Reality Confidence: 72%**

The preferred UI should emphasize the qualitative status first.

Clicking the indicator opens the detailed assessment.

---

# 2. WHAT THE INDICATOR MEANS

The user should see:

> **What does this mean?**
>
> This indicator reflects how well your current goals and assumptions are supported by the information you've provided and the external research we've completed.
>
> It is not a prediction of whether your transaction will succeed and does not replace professional advice.

The system should always identify:

### Current status

How well-supported the current destination appears to be.

### Evidence quality

How much reliable information supports the assessment.

### Research freshness

How recently the relevant information was checked.

### Professional review status

Whether qualified professionals have reviewed the relevant assumptions.

---

# 3. THE INDICATOR SHOULD BEGIN WITH LIMITED CONFIDENCE

Immediately after creating the Destination:

### Goal Alignment

**Preliminary**

> We understand what you want, but we don't yet have enough information about your business to meaningfully compare your goals with current business and transaction conditions.

This is important.

The platform should not manufacture confidence simply because the user has completed the first screen.

---

# 4. THE RESEARCH AGENT

The platform should contain a specialized research agent responsible for investigating the owner's assumptions.

Call this:

## Goal Reality Research Agent

Its job is to investigate questions such as:

* Is the desired outcome within commonly observed ranges?
* Are the assumptions consistent with current industry conditions?
* Are there relevant transaction norms?
* Are there known financing constraints?
* Are the owner's desired timelines broadly consistent with similar transactions?
* Are there meaningful differences by industry or company size?
* Are there current regulatory or structural considerations that could materially affect the scenario?
* What important information is missing?

The agent should research the **question behind the owner's assumption**, not the owner's personal identity.

---

# 5. PRIVACY-FIRST RESEARCH INPUT

The Research Agent should receive the **minimum information necessary**.

It should not receive direct personal identifiers unless there is an independently justified reason and the user explicitly authorizes it.

Research inputs should generally be transformed into generalized characteristics such as:

**Industry:** Commercial HVAC services

**Business size:** $5M–$10M revenue

**Approximate EBITDA:** $1M–$2M

**Employees:** 40–75

**Region:** Western U.S.

**Desired transaction:** Employee ownership

**Desired closing proceeds:** $3M–$5M

**Desired future income:** $75K–$100K annually for 5–10 years

**Desired transition:** 18 months

rather than:

**Company name**

**Owner name**

**Customer names**

**Employee names**

**Exact addresses**

**Tax ID**

**Account numbers**

or unnecessary personal information.

---

# 6. DO NOT PROMISE "ANONYMIZATION"

The system should use:

**Data minimization**

**De-identification**

**Pseudonymization where necessary**

**Strict input filtering**

**Access controls**

and other privacy protections.

Do not describe the process internally or externally as guaranteeing that data is "anonymous."

Information can sometimes be re-identified when multiple attributes are combined.

The architecture should therefore assume:

> **Less sensitive information leaves the local environment in the first place.**

This follows the broader privacy-risk-management approach emphasized by NIST's Privacy Framework.

---

# 7. LOCAL DATA SHOULD REMAIN LOCAL BY DEFAULT

The Research Agent should not automatically receive:

* Tax returns
* Payroll
* Customer lists
* Employee records
* Bank statements
* Contracts
* Complete financial statements
* Other sensitive documents

unless the user explicitly authorizes an action that requires them.

The default research request should be generated from **sanitized structured information**.

---

# 8. RESEARCH REQUEST OBJECT

Each research request should become a structured object.

### GoalRealityResearchRequest

Contains:

* Research ID
* Journey ID
* Destination version
* Scenario version, if applicable
* Sanitized user assumptions
* Research questions
* Industry
* Business-size category
* Geographic scope, when relevant
* Required evidence categories
* Date requested
* Research status

---

# 9. RESEARCH SHOULD BE QUESTION-DRIVEN

Do not tell an AI agent:

> "Research whether this transaction is realistic."

That is too vague.

Instead create specific research questions.

Example:

### Research Question

> "For profitable privately held businesses in this industry and approximate size range, how does the owner's desired closing proceeds compare with commonly observed valuation and transaction ranges?"

Another:

> "For employee-ownership transactions involving businesses of this approximate size, what transaction structures and financing mechanisms are commonly used?"

Another:

> "Does the desired 18-month transition period appear broadly consistent with the preparation and transaction processes commonly observed for comparable transactions?"

The agent answers **specific questions with evidence**.

---

# 10. SOURCE HIERARCHY

Research should prioritize sources according to reliability and relevance.

Potential hierarchy:

### Tier 1

Government agencies
Regulators
Official standards
Official transaction/program documentation

### Tier 2

Established industry associations
Recognized professional organizations
Major research institutions

### Tier 3

Reputable financial and business research

### Tier 4

Specialized publications and practitioner sources

### Tier 5

General web sources

Lower-quality sources may provide leads but should not independently drive major confidence changes.

---

# 11. SOURCE PROVENANCE

Every material research conclusion should retain:

* Source
* URL/reference
* Publication date
* Research date
* Relevant passage/data
* What claim it supports
* Geographic scope
* Population/sample scope
* Limitations
* Source quality

This gives the system an **Evidence Ledger**.

The user doesn't need to see all of it immediately.

But it must exist.

---

# 12. EVIDENCE LEDGER

## Example

### Claim

Comparable businesses commonly transact within a certain valuation range.

### Evidence

Source A
Industry transaction research
Published: 2026

Source B
Professional association data
Published: 2025

### Applicability

Moderate

### Limitation

Available data may not precisely match the user's business size and geography.

### Research confidence

Moderate

This prevents the Research Agent from turning one convenient web result into apparent fact.

---

# 13. CONFIDENCE SHOULD HAVE MULTIPLE DIMENSIONS

The headline indicator can remain simple.

Underneath it, use separate dimensions.

### Goal Alignment

How closely the desired outcome appears to fit current evidence.

### Evidence Quality

How strong and relevant the supporting information is.

### Data Completeness

How much necessary information has been provided.

### Research Freshness

How current the evidence is.

### Professional Review

Whether qualified professionals have reviewed the relevant assumptions.

These should not be collapsed into one mysterious AI number.

---

# 14. EXAMPLE

### Goal Alignment

**72% | Moderate**

### Evidence Quality

**Good**

### Data Completeness

**Preliminary**

### Research Freshness

**Current**

### Professional Review

**Not yet reviewed**

Then:

### Why?

🟢 Employee ownership objective is consistent with the type of transaction being explored.

🟢 Desired transition timeline is within a range worth investigating.

🟡 Desired cash at closing requires additional valuation and financing analysis.

🟡 Desired future income depends on assumptions that have not yet been validated.

🔴 One stated assumption currently appears inconsistent with available evidence.

This is far more useful than:

> **Your confidence is 72%.**

---

# 15. CONFIDENCE MUST BE EXPLAINABLE

Every meaningful increase or decrease should have a reason.

Example:

### Your Goal Alignment changed

**64% → 71%**

Why?

> You provided updated business cash-flow information.

> The Research Agent found current industry evidence more consistent with your stated target.

Or:

### 71% → 58%

Why?

> Your updated debt information reduces the apparent feasibility of the current financing assumptions.

The user should never experience an unexplained number changing.

---

# 16. CONFIDENCE SHOULD NOT MONOTONICALLY INCREASE

This is important.

The user's goal may become **less supported** as better information arrives.

That is not a failure.

For example:

**50%**

Initial information.

↓

**75%**

Basic business information added.

↓

**59%**

Financial documents reveal substantially higher debt.

↓

**68%**

Alternative scenario reduces the financing requirement.

That is exactly how the system should behave.

The purpose is **better understanding**, not always making the user feel better.

---

# 17. RESEARCH SHOULD BE RE-RUN AT MILESTONES

The platform should automatically offer or initiate research when meaningful new information changes the analysis.

Examples:

### After Destination Created

Initial benchmark research.

### After Business Snapshot

Business-specific benchmark research.

### After Financial Information

Financial feasibility research.

### After Scenario Created

Scenario-specific research.

### After Financing Assumptions

Financing-market research.

### After Professional Feedback

Research questions generated by unresolved issues.

---

# 18. USER SHOULD ALSO HAVE A RESEARCH BUTTON

Persistent action:

### 🔎 Research My Assumptions

The owner can explicitly ask:

> "Check this."

The system can then research the current assumption set.

The UI can say:

> **We'll research your assumptions using current external information and show you what supports them, what conflicts with them, and what remains uncertain.**

---

# 19. DEEP RESEARCH MODE

The owner should have a deeper option.

### Standard Check

Fast benchmark review.

### Deep Research

More comprehensive investigation using multiple current sources.

Deep Research should examine:

* Industry data
* Transaction data
* Financing environment
* Ownership structures
* Relevant professional standards
* Current regulatory information where applicable
* Comparable business characteristics
* Identified assumptions
* Contradictory evidence

The research should not simply search more websites.

It should seek **corroboration and conflicting evidence**.

---

# 20. RESEARCH AGENT SHOULD ACTIVELY LOOK FOR DISCONFIRMING EVIDENCE

This is critical.

Do not instruct the agent:

> "Find evidence that supports the owner's plan."

Instead:

> **"Determine whether the owner's assumption is supported, contradicted, or unresolved by current evidence. Specifically search for evidence that would challenge the assumption."**

This prevents confirmation bias.

---

# 21. RESEARCH OUTPUT

Each research cycle should generate:

### Research Summary

What we found.

### Supporting Evidence

Why we found it.

### Contradicting Evidence

What challenges it.

### Unknowns

What cannot currently be established.

### Assumptions

What the research depends on.

### Goal Impact

Which parts of the Desired Outcome are affected.

### Questions for Professionals

What should be reviewed by qualified advisors.

---

# 22. NO FALSE PRECISION

Do not produce:

> "Your transaction has a 73.2% chance of success."

Do not infer such a probability from benchmark data.

Likewise:

> "You are 82% likely to receive $4M."

is prohibited as a product behavior.

The platform can say:

> "Your current target appears broadly consistent with the benchmark range we found."

or:

> "Your current target currently appears outside the range supported by the evidence we found."

The confidence indicator describes **evidence alignment**, not outcome probability.

---

# 23. THE INDICATOR SHOULD HELP PREVENT BAD PROFESSIONAL PACKAGES

Before the owner sends a package:

## Package Readiness Check

### Your goals

🟢 Clearly defined

### Your assumptions

🟡 Some remain weakly supported

### Your financial target

🟡 Requires professional validation

### Your ownership objective

🟢 Clearly defined

### Your timing

🟢 Supported as an initial planning target

### Overall

**Ready for professional review, with 3 questions flagged**

The owner can then choose:

**Send Package**

**Review Flagged Items**

**Research More**

---

# 24. RESEARCH FLAGS SHOULD NOT STOP THE OWNER

A low confidence or unresolved issue should normally not block the owner.

Instead:

> **We found something your professional should review before you rely on this scenario.**

Then:

**Continue**

**Research More**

**Edit Goal**

The user retains control.

---

# 25. THE OWNER SHOULD BE ABLE TO SEE "WHAT WOULD IMPROVE CONFIDENCE?"

This is a particularly useful feature.

Example:

### What would strengthen this assessment?

**Add recent financial statements**

**Enter approximate debt**

**Provide management information**

**Research financing**

**Ask a valuation professional**

**Ask a tax professional**

The application turns uncertainty into an actionable next step.

---

# 26. RESEARCH SHOULD GENERATE QUESTIONS, NOT ADVICE

Instead of:

> "You should use an ESOP."

produce:

> **Question for your professional:**
>
> "Given the owner's goal of broad employee ownership and substantial cash at closing, should an ESOP be evaluated alongside other employee-ownership structures?"

Instead of:

> "Your seller note should be 7 years."

produce:

> **Question for your financing professional:**
>
> "What seller-note term would be sustainable given the company's projected cash flow and the owner's desired income period?"

This preserves the three-layer architecture.

---

# 27. THE THREE-LAYER MODEL BECOMES FOUR WITH RESEARCH

The product now has:

### 1. OWNER OBJECTIVE

**What I want.**

### 2. RESEARCH EVIDENCE

**What current external evidence says about my assumptions.**

### 3. PLATFORM SCENARIO

**What might potentially accomplish my goal under the current assumptions and evidence.**

### 4. PROFESSIONAL DETERMINATION

**What the qualified professional concludes should actually be done.**

This separation is critical.

Research evidence is **not professional determination**.

Platform modeling is **not professional determination**.

---

# 28. RESEARCH SHOULD BE VERSIONED

Every assessment should record:

**Research version**

**Date**

**Sources**

**Assumptions**

**Journey version**

**Destination version**

**Scenario version**

This allows the platform to answer:

> "Why did the system show this confidence level on September 19?"

That is essential for trust and auditability.

---

# 29. CURRENT INFORMATION SHOULD EXPIRE

Research shouldn't remain "current" forever.

Each finding receives a freshness status:

**Current**

**Aging**

**Needs Refresh**

**Outdated**

A significant change in:

* Market conditions
* Financing conditions
* Regulation
* Industry data
* Transaction norms

could trigger re-research.

This is particularly important because NIST treats AI risk management and evaluation as an ongoing process rather than a one-time certification.

---

# 30. PROFESSIONAL REVIEW SHOULD IMPROVE THE SYSTEM, NOT REPLACE RESEARCH

When a professional reviews a scenario, their feedback should become a separate layer.

Example:

**Research Evidence**

→ "Comparable transactions suggest X."

**Platform Scenario**

→ "Under these assumptions, Scenario A potentially fits."

**Professional Determination**

→ "Given the actual facts, investigate Scenario B instead."

The platform should preserve all three.

---

# 31. RESEARCH AGENT SAFETY RULE

The Research Agent should always ask:

1. What exactly are we testing?
2. What evidence would support it?
3. What evidence would contradict it?
4. How applicable is the evidence?
5. How current is it?
6. What information is missing?
7. Is this within the platform's role?
8. Does this require professional determination?

The answer to #8 should be surfaced rather than hidden.

---

# 32. CONFIDENCE UI

Recommended top bar:

```text id="qjm0q5"
┌───────────────────────────────────────────────────────────────┐
│ 🎯 Your Destination    Goal Alignment 🟡 72%    Journey 48% │
└───────────────────────────────────────────────────────────────┘
```

Click:

```text id="nq6o4c"
GOAL ALIGNMENT

72%  MODERATE

Evidence Quality       GOOD
Data Completeness      PRELIMINARY
Research Freshness     CURRENT
Professional Review    NOT YET REVIEWED

What's affecting this?

🟢 Employee ownership goal
🟢 Transition objective
🟡 Closing proceeds
🟡 Future income
🟡 Timeline

[Research My Assumptions]
[View Evidence]
```

---

# 33. THE MOST IMPORTANT UX DETAIL

The indicator should **never simply say**:

> "You're getting closer."

Instead:

> **"Your plan is becoming better supported by the information currently available."**

That is much more truthful.

Sometimes the best result of research will be:

> **"Your original goal needs to change."**

That is valuable.

The product succeeds when it helps the owner discover that **before** they spend money or send an unrealistic package to a professional.

---

# 34. RESEARCH BEFORE PROFESSIONAL ENGAGEMENT

The ideal sequence becomes:

**Owner states destination**

↓

**Platform performs initial reality check**

↓

**Owner sees gaps**

↓

**Owner adjusts assumptions or investigates**

↓

**Platform performs deeper research**

↓

**Scenario is generated**

↓

**Goal-to-Reality Confidence updated**

↓

**Professional Review Package created**

↓

**Professional reviews**

This makes the professional's time much more valuable.

---

# 35. THE DEEP-RESEARCH BUTTON SHOULD BE A MAJOR FEATURE

The user should eventually be able to tap:

### 🔎 Deep Research My Plan

and receive:

> **We researched your current assumptions against current industry, transaction, financing, and ownership evidence.**

Then:

### What appears supported

### What appears uncertain

### What appears inconsistent

### What we couldn't determine

### What your professionals should review

### What information would improve the analysis

This could become one of the platform's signature capabilities.

---

# 36. CORE PRODUCT RULE

**The Goal-to-Reality Confidence Engine exists to improve the owner's understanding, not to manufacture reassurance.**

It should be willing to move:

**up**

**down**

or remain:

**uncertain**

as evidence changes.

Its greatest value may sometimes be telling the owner:

> **"Before you go any further, there is an assumption here worth examining."**

---

# 37. NORTH STAR

The final experience should feel like:

> **I know what I want.**

→ **The platform checked my assumptions.**

→ **I understand what appears realistic, what is uncertain, and what needs professional review.**

→ **I explored possible paths.**

→ **I changed my assumptions where appropriate.**

→ **I generated a professional-ready package.**

→ **My professionals now have a much clearer starting point.**

The platform doesn't tell the owner what to do.

**It helps the owner arrive at the professional conversation better informed and better prepared.**
