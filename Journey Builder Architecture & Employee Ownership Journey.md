# Journey Builder Architecture & Employee Ownership Journey

## Version 0.1

## 1. Product Architecture

The platform should be built around a configurable **Journey Engine**.

The Journey Engine determines:

> **What the user sees → what they can choose → what information is captured → what happens next → what scenario is generated → what professional expertise may be needed → what gets added to the Professional Review Package.**

The application should not hard-code a single questionnaire.

Instead, it should contain a reusable journey framework capable of supporting:

* Employee Ownership
* Management Buyout
* Direct Acquisition
* Staged Ownership
* Other succession journeys added later

The first published journey will be:

# Employee Ownership Journey

---

# 2. High-Level Architecture

```text
                    ┌─────────────────────────┐
                    │     JOURNEY BUILDER     │
                    │                         │
                    │ Questions               │
                    │ Choice Types            │
                    │ Branching               │
                    │ Rules                   │
                    │ Scenarios               │
                    │ Professional Roles      │
                    │ Package Templates        │
                    └────────────┬────────────┘
                                 │
                              PUBLISH
                                 │
                    ┌────────────▼────────────┐
                    │      JOURNEY ENGINE     │
                    │                         │
                    │ Runtime State           │
                    │ Answers                 │
                    │ Branches                │
                    │ Objectives              │
                    │ Scenarios               │
                    │ Progress                │
                    └───────┬─────────┬───────┘
                            │         │
                 ┌──────────▼───┐   ┌▼────────────────┐
                 │ LOCAL USER   │   │ ONLINE PLATFORM │
                 │ WORKSPACE    │   │                │
                 │              │   │ Marketplace    │
                 │ Sensitive    │   │ Professionals  │
                 │ documents   │   │ Workflow       │
                 │ Financial   │   │ Accounts       │
                 │ data        │   │ Vendors        │
                 │ Private     │   │ Coordination   │
                 │ scenarios   │   │                │
                 └──────────────┘   └─────────────────┘

                            │
                            ▼
                 ┌────────────────────────┐
                 │ PROFESSIONAL REVIEW    │
                 │ PACKAGE ENGINE         │
                 └────────────────────────┘
```

---

# 3. Journey Builder

The Journey Builder is the administrative/CMS system used to create and maintain journeys.

It should feel conceptually like:

**WordPress + workflow builder + decision tree + form builder**

but specifically designed around this platform.

## Journey Builder Components

### Journey

The overall experience.

Example:

**Employee Ownership Journey**

### Stage

A logical section of the journey.

Example:

**Owner Objectives**

### Node

One interaction or information step.

Example:

**What matters most to you about the proceeds?**

### Answer

A user's response.

### Branch

Determines what happens next.

### Rule

Determines whether a scenario, warning, professional role, document request, or additional question should appear.

### Scenario Template

Defines how a set of owner preferences and business facts produces an exploratory scenario.

### Professional Role

Defines what expertise may be relevant.

### Package Template

Defines what information is included in the Professional Review Package.

---

# 4. Journey Node Types

The Journey Builder should support the following interaction types as native components.

### Basic

* Single Select
* Yes / No
* Multi-Select
* Select Up To 3

### Priority

* Ranking
* Trade-Off / Spectrum
* Primary vs. Secondary Priority

### Quantitative

* Numeric
* Range
* Percentage Allocation
* Choice + Amount

### Timing

* Date
* Month / Year
* Timeframe

### Uncertainty

* Not Sure / Help Me Decide
* Confidence
* Flexibility

### Exploration

* Conditional Choice
* Scenario Comparison
* What-If
* Explore Another Path

### Context

* Optional Explanation
* Free Text
* Document Selection
* Local Document Upload

### Commitment

* Review
* Confirmation
* Approval
* Share With Professional

### Professional Selection

* Use My Professional
* Use Curated Professional
* Add Professional
* Change Professional

These become reusable components.

The Journey Builder should not need custom code to create a normal journey question.

---

# 5. Every Question Has Metadata

Each question should contain more than its text.

Conceptually:

```text
Question
├── ID
├── Stage
├── Question Text
├── Why We're Asking
├── Interaction Type
├── Choices
├── Required / Optional
├── Data Field
├── Storage Scope
├── Branching Rules
├── Follow-Up Rules
├── Scenario Impact
├── Professional Role Impact
├── Package Impact
└── Version
```

This is important because the same answer can affect multiple parts of the system.

For example:

> "I want most of my money at closing."

could affect:

* Owner Objective
* Scenario generation
* Financing exploration
* Seller-note exploration
* Professional Review Package
* Questions presented to the CPA
* Questions presented to the attorney
* Questions presented to financing professionals

---

# 6. Storage Scope Is Part of the Journey Definition

Every question should indicate where its answer belongs.

### Public

Safe to display publicly.

### Online Private

Stored in the user's account but not public.

### Local Private

Stored only on the user's computer unless explicitly shared.

### Shared Transaction Data

Uploaded only after the user authorizes sharing.

### Professional Determination

Information entered or submitted by a professional.

This allows the hybrid architecture to work at the question level.

A question involving:

> "What is your annual revenue?"

could be treated differently from:

> "Select your latest tax return."

---

# 7. The Three Decision Layers Are Native Objects

Every significant piece of information should belong to one of three layers.

## Layer 1: Owner Objective

**What the owner wants.**

Examples:

* Maximize cash at closing
* Receive income over time
* Employee ownership
* Retire within two years
* Preserve jobs
* Remain involved
* Minimize disruption

---

## Layer 2: Platform Scenario

**What might potentially accomplish that objective under the assumptions provided.**

Examples:

* ESOP scenario
* Direct employee purchase
* Management buyout
* Employee-owned acquisition entity
* Staged ownership
* Seller-financing scenario
* Bank + seller-note scenario

Every scenario contains:

**Inputs → Assumptions → Outputs → Trade-offs**

and is clearly labeled exploratory.

---

## Layer 3: Professional Determination

**What the qualified professional concludes should actually be done.**

This is entered after professional review.

The platform stores:

* Professional
* Role
* Date
* Scenario reviewed
* Determination
* Guidance
* Required changes
* Follow-up tasks

The system must never automatically convert a Platform Scenario into a Professional Determination.

---

# 8. Journey Runtime

When a user starts a journey, create:

**Journey Instance**

That instance contains:

* Current stage
* Current node
* Completed nodes
* Answers
* Owner Objectives
* Scenarios
* Selected professionals
* Documents
* Package versions
* Professional feedback
* Change history

The journey is therefore a **stateful process**, not simply a form.

---

# 9. Back / Edit / Change Must Be Native

The Journey Engine must support:

**Go Back**

**Edit Answer**

**Change Objective**

**Explore Another Option**

When something changes, dependent information should be recalculated.

Example:

```text
Owner selects:
"Maximum cash at closing"

        ↓

Financing scenarios generated

        ↓

Owner changes:
"Actually, income over time is more important"

        ↓

Previous scenarios remain in history

        ↓

New scenarios are generated

        ↓

Professional Review Package is updated
```

The system must not erase the original decision.

---

# 10. Scenario Versioning

Every scenario receives a version.

Example:

**Scenario A v1**

Based on:

* $5M estimated business value
* High cash-at-closing preference
* 18-month transition

The owner changes a preference.

The system creates:

**Scenario A v2**

rather than destroying v1.

This lets the owner say:

> "Let's go back and see what we had before."

---

# 11. First Employee Ownership Journey

The first journey should not begin with:

> "Which transaction structure do you want?"

Instead:

> **"What are you trying to accomplish?"**

The structure emerges later.

---

# STAGE 0: Welcome

### Screen

# Explore Selling Your Business to Your Employees

> This guided journey will help you clarify what you want from a transition, explore potential ownership paths, and prepare a plan to discuss with your professional advisors.

### Important distinction

> We do not provide legal, tax, investment, valuation, or other professional advice. Your results are exploratory and are intended to help you prepare for conversations with qualified professionals.

### Choice

**Start My Journey**

---

# STAGE 1: OWNER OBJECTIVES

## Question 1

### What matters most to you about the outcome?

**Get substantial cash at closing**

**Create income over time**

**Preserve the company through employee ownership**

The selected answer becomes:

**Primary Owner Objective**

A fourth option can be:

**I'm not sure. Help me explore**

---

## Question 2

### What else matters to you?

**Select up to 3**

Potential choices:

* Retire completely
* Transition quickly
* Preserve jobs
* Preserve company culture
* Keep the company independent
* Maintain some involvement
* Reduce debt
* Generate future income
* Consider tax implications
* Give employees meaningful ownership
* Protect family/estate interests

---

## Question 3

If multiple goals were selected:

### Put these in order from most important to least important.

The user ranks only the goals they selected.

This establishes:

**Primary**

**Secondary**

**Tertiary**

without making them rank twelve unrelated items.

---

# STAGE 2: FLEXIBILITY

## Question

### How flexible are you about your goals?

For important objectives:

**Very firm**

**Strong preference**

**Flexible**

This establishes constraints versus preferences.

---

# STAGE 3: TIMING

## Question

### When would you ideally like to transition out of the business?

**Within 1 year**

**1–3 years**

**3–5 years**

**I'm flexible**

**I'm not sure**

If appropriate, the user can later enter a specific date.

---

# STAGE 4: OWNER INVOLVEMENT

### How involved would you like to be after the transaction?

**I want to leave completely**

**I could stay for a transition period**

**I'd like to remain involved**

This can branch into questions about transition preferences.

---

# STAGE 5: EMPLOYEE OWNERSHIP

Now that the system understands the owner's objectives:

### How important is employee ownership to the outcome?

**It's essential**

**It's very important**

**I'd like to explore it**

This is a preference, not a commitment to a particular structure.

---

# STAGE 6: BUSINESS SNAPSHOT

Only now does the application begin gathering enough information to create meaningful scenarios.

Start with approximate information.

### About how large is your business?

Revenue:

**Under $1M**

**$1M–$5M**

**$5M–$10M**

**$10M–$25M**

**Over $25M**

**I'm not sure**

Then progressively ask for:

* Approximate cash flow / EBITDA
* Employee count
* Existing debt
* Ownership structure
* Number of owners
* Owner dependence
* Management depth

Exact figures can be entered later when useful.

---

# STAGE 7: FINANCIAL PREFERENCE JOURNEY

This branch is driven by the Owner Objective.

## Example: Cash-at-Closing Path

### How important is receiving cash at closing?

**Very important**

**Important**

**I'm flexible**

Then:

### Would you consider receiving some proceeds over time?

**Yes**

**Possibly**

**No**

**I'm not sure**

Only then introduce:

**Seller financing**

without assuming the owner wants it.

---

# STAGE 8: INCOME-OVER-TIME PATH

If income over time is important:

### How important is ongoing income after closing?

**Very important**

**Important**

**I'm flexible**

Then introduce potential mechanisms such as:

* Seller financing
* Staged ownership
* Other transaction structures

Again:

> **These are possibilities to explore, not recommendations.**

---

# STAGE 9: SELLER-NOTE EXPLORATION

If seller financing enters the owner's scenario:

### What would you want to do with a seller note?

**Keep the note**

**Explore selling part of it**

**Explore selling all of it**

**I'm not sure**

If the owner chooses liquidity exploration:

### Seller-Note Marketplace

The platform presents potentially relevant curated note buyers.

Each profile contains:

* Company
* Description
* Investment focus
* Typical note size
* Industries
* Geography
* Note characteristics
* Contact name
* Email
* Phone
* Website
* Verification status
* Last verified date
* Other relevant information

The platform may identify:

**Potential Matches**

but does not represent them as offers until the buyer actually submits one.

---

# STAGE 10: STRUCTURE EXPLORATION

Only after objectives and preferences have been established does the platform introduce ownership structures.

Potential candidates include:

### ESOP

### Direct Employee Purchase

### Management Buyout

### Employee-Owned Acquisition Entity

### Staged Employee Ownership

### Other / Explore Alternatives

The system should normally show **2–3 relevant paths at once**, based on the user's stated objectives and information.

The user can select:

**Explore**

**Compare**

**Ask My Professional**

**Show Me Another Path**

---

# STAGE 11: SCENARIO COMPARISON

The platform presents a small number of scenarios.

Example:

## Your Current Exploration

### Scenario A

**Employee ownership through an ESOP**

### Scenario B

**Direct employee purchase**

### Scenario C

**Staged employee ownership**

Each displays:

**What it is**

**Why it appeared**

**What assumptions we're using**

**Potential benefits**

**Potential trade-offs**

**Questions for your professionals**

The platform does not identify a winner.

---

# STAGE 12: PROFESSIONAL TEAM

Once a scenario reaches sufficient maturity:

> **Your current exploration suggests several areas where specialized professional input may be useful.**

The platform identifies functional needs.

Potential roles:

* Attorney
* CPA / tax professional
* Valuation professional
* Financing professional
* ESOP professional
* Trustee
* Financial advisor
* Other specialist

For each:

### Do you already have someone?

**Yes, I'll use mine**

**Show me your curated professionals**

**I'm not sure**

---

# STAGE 13: "I'LL USE MINE" ROLE-FIT CHECK

When the user chooses their own professional:

> **Your professional may be an excellent advisor. Specialized transactions often require specialized experience. Let's make sure they're being used in the role that best fits their expertise.**

Then ask approximately 2–3 questions about relevant experience.

The result may be:

**Primary Professional**

or

**Advisor + Specialized Professional**

or

**Needs Further Review**

This is guidance, not a determination of competence.

---

# STAGE 14: CURATED PROFESSIONALS

If the user chooses the platform network:

Show approximately 2–3 relevant professionals initially.

Each profile includes:

* Company
* Individual
* Specialty
* Relevant experience
* Geographic coverage
* Transaction size
* Credentials
* Services
* Contact information
* Website
* Availability
* Verification status

The user can:

**Select**

**Compare**

**View Profile**

**Show More**

---

# STAGE 15: BUILD THE OWNER'S PLAN

The platform now compiles everything.

## Your Preliminary Ownership Transition Plan

### Owner Objective

What the owner wants.

### Priorities

Ranked objectives.

### Preferences

Timing, involvement, liquidity, ownership, etc.

### Business Snapshot

Current information provided.

### Scenarios Explored

The scenarios the owner reviewed.

### Current Preferences

What the owner currently favors.

### Financing Concepts Explored

Including seller financing where applicable.

### Professional Needs

The roles that may be relevant.

### Open Questions

Questions requiring professional review.

### Documents Available

Only documents the owner has actually provided or authorized for sharing.

---

# STAGE 16: PROFESSIONAL REVIEW PACKAGE

The user receives:

# Professional Review Package

The package should contain:

## 1. Owner Objectives

> What I am trying to accomplish.

## 2. Owner Priorities

> What matters most to me.

## 3. Constraints & Preferences

> What I consider important, flexible, or non-negotiable.

## 4. Business Snapshot

> Information I have provided.

## 5. Scenarios Explored

> Potential approaches generated from the information provided.

## 6. Assumptions

> Information used to produce the scenarios.

## 7. Trade-Offs

> The differences the owner wants the professional to evaluate.

## 8. Questions for Professionals

> What the owner wants validated or investigated.

## 9. Professional Review

> Space for professional feedback.

## 10. Professional Determination

> What the qualified professional ultimately concludes should be done.

---

# STAGE 17: PROFESSIONAL REVIEW

The owner can:

**Download Package**

**Share Package**

**Invite Professional**

**Continue Exploring**

The owner can provide the entire package or selected portions.

---

# STAGE 18: PROFESSIONAL RESPONSE

A professional can review and respond.

They may:

**Validate**

**Modify**

**Reject**

**Request More Information**

**Recommend Another Path**

The platform records this as:

**Professional Determination**

or professional guidance associated with a scenario.

---

# STAGE 19: REVISED JOURNEY

Professional feedback can produce:

**New Questions**

**New Assumptions**

**New Scenario**

**Changed Owner Objective**

**Additional Professional**

or

**Revised Professional Review Package**

Nothing is destroyed.

Everything is versioned.

---

# 19. Journey State Model

The Employee Ownership Journey can move through:

```text
EXPLORING
    ↓
OBJECTIVES DEFINED
    ↓
SCENARIOS DEVELOPED
    ↓
PRELIMINARY PLAN
    ↓
PROFESSIONAL REVIEW
    ↓
PLAN REVISED
    ↓
TRANSACTION PLANNING
    ↓
TRANSACTION EXECUTION
```

The first MVP should probably stop primarily at:

**Professional Review**

rather than attempting to automate the entire closing process immediately.

---

# 20. Journey Builder: Administrative Experience

The administrator should be able to create a journey without writing application code.

Example:

## Add Question

**Question:**

"What matters most to you?"

**Interaction:**

Single Select

**Choices:**

Cash now
Income over time
Employee ownership

**Stores as:**

Owner Objective.Primary

**Next:**

Branch based on selection

**Scenario impact:**

Cash weighting / income weighting / ownership weighting

**Package impact:**

Include in Owner Objectives

---

# 21. Visual Journey Canvas

The Journey Builder should provide a visual map:

```text
WELCOME
   ↓
OWNER OBJECTIVE
   ↓
SECONDARY OBJECTIVES
   ↓
RANK PRIORITIES
   ↓
TIMING
   ↓
EMPLOYEE OWNERSHIP
   ↓
BUSINESS SNAPSHOT
   ↓
      ┌───────────────┐
      │ Goal Branch   │
      └───────┬───────┘
              │
       ┌──────┼──────┐
       ↓      ↓      ↓
     CASH   INCOME  OWNERSHIP
       │      │      │
       └──────┼──────┘
              ↓
       SCENARIO ENGINE
              ↓
       PROFESSIONAL TEAM
              ↓
       REVIEW PACKAGE
              ↓
       PROFESSIONAL REVIEW
```

The builder should allow administrators to inspect every branch.

---

# 22. Journey Testing

Before publishing a journey, administrators should be able to simulate:

**Test User A**

"I want maximum cash."

**Test User B**

"I want income."

**Test User C**

"I want employee ownership."

**Test User D**

"I'm not sure."

**Test User E**

"I have my own CPA and attorney."

**Test User F**

"I want seller financing and want to explore selling the note."

The builder should show exactly which path each test user takes.

This is essential because a decision tree can look sensible until actual branching begins multiplying like rabbits.

---

# 23. Journey Versioning

Every published journey receives a version.

Example:

**Employee Ownership Journey v1.0**

An update creates:

**v1.1**

Existing users remain on the version appropriate to their transaction unless migration is deliberately performed.

This is important for:

* Auditability
* Reproducibility
* Professional Review Packages
* Scenario history
* Regulatory/legal review
* Debugging

A package should record:

> **Journey Version Used: Employee Ownership v1.1**

---

# 24. Professional Review Package Versioning

The same concept applies to packages.

Example:

**Package v1**
Owner's initial objectives

↓

**Package v2**
Additional financial information

↓

**Package v3**
Professional feedback incorporated

↓

**Package v4**
Revised owner preferences

This creates a transparent evolution of the owner's thinking.

---

# 25. The Most Important Design Rule

The Journey Engine should always be asking:

> **What is the smallest useful next decision?**

Not:

> **What information can we collect right now?**

This distinction should govern the entire application.

---

# 26. Employee Ownership Journey North Star

The user should finish the first journey feeling:

> **"I understand what I want."**

> **"I understand the major paths I could investigate."**

> **"I understand the major trade-offs."**

> **"I know what questions I need to ask my professionals."**

> **"I have something useful I can hand to my advisors."**

The user should **not** finish believing:

> "The software told me which transaction to do."

---

# 27. First MVP Boundary

The first Employee Ownership release should concentrate on:

**Goal discovery**

→ **Preference capture**

→ **Business snapshot**

→ **Scenario exploration**

→ **Professional team identification**

→ **Professional selection**

→ **Professional Review Package**

→ **Professional feedback**

The deeper transaction execution system can then be layered onto the same journey framework.

---

# 28. Product Architecture in One Sentence

> **The Journey Engine converts an owner's goals into structured decisions, structured decisions into exploratory scenarios, exploratory scenarios into Professional Review Packages, and professional feedback back into an evolving ownership plan.**

That is the core application.
