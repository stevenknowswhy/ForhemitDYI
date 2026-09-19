# Employee Ownership Journey

## Destination Builder v1.0

### Core Purpose

The Destination Builder is the **first functional component of the Employee Ownership Journey**.

Its sole purpose is to capture:

> **What does a successful ownership transition look like to this owner?**

It does **not** determine whether the outcome is achievable.

It does **not** recommend a transaction structure.

It does **not** provide legal, tax, investment, valuation, or financing advice.

It creates the owner's:

# Desired Outcome / Destination

This becomes the persistent **North Star** for the remainder of the journey.

---

# 1. DESTINATION BUILDER DESIGN PRINCIPLES

The Destination Builder must follow these rules:

### One meaningful decision at a time

Do not display a traditional multi-page form.

### 2–3 primary choices whenever practical

Additional choices can appear progressively.

### Use the right interaction for the information

Single select, multi-select, ranking, range, percentage allocation, date, etc.

### Every meaningful objective can become a nonnegotiable

The owner can mark:

**☐ This is nonnegotiable**

### "I'm not sure" is always a legitimate answer

Never force false certainty.

### Explain why when useful

Short contextual guidance, not paragraphs of instruction.

### The destination is editable

The owner can change it at any point.

### Changes create a new version

The original destination is retained.

### Do not ask for information we don't need yet

Revenue, EBITDA, debt, tax returns, valuation, employee count, etc. belong to later stages.

---

# 2. DESTINATION BUILDER AT A GLANCE

```text id="6khyc6"
START
  ↓
PICTURE SUCCESS
  ↓
CLOSING MONEY
  ↓
FUTURE INCOME
  ↓
WHO OWNS THE COMPANY
  ↓
OWNER'S ROLE
  ↓
TIMING
  ↓
WHAT SHOULD REMAIN TRUE
  ↓
WHAT SHOULD BE AVOIDED
  ↓
MUST-HAVES
  ↓
REVIEW YOUR DESTINATION
  ↓
CREATE DESTINATION
```

The journey should normally take only a few minutes.

Additional detail can be added later.

---

# 3. SCREEN 1: PICTURE THE FINISH LINE

## Header

# Picture the Day Your Business Transition Is Complete

### Supporting text

> Before we explore ESOPs, employee purchases, financing, or other structures, let's first understand what success looks like to you.
>
> You don't need to know how you'll get there yet.

### Primary action

**Let's Picture It**

### Secondary

**I'm Not Sure Yet**

Selecting "I'm Not Sure Yet" still creates a destination, but with fewer initial constraints.

---

# 4. SCREEN 2: YOUR MAIN FINANCIAL OUTCOME

## Header

# When the business is transferred, what would you like financially?

Present three cards:

### 💰 More Cash Now

**I want substantial cash at closing.**

### 💵 Income Over Time

**I'm comfortable receiving some of the value over time.**

### ⚖️ A Combination

**I'd like a mix of cash now and future income.**

### Fourth option

**I'm not sure**

This identifies the **financial objective**, not the transaction mechanism.

Do not mention seller notes yet.

---

# 5. SCREEN 3: CASH AT CLOSING

Only display this screen if cash at closing is relevant.

## Header

# About how much would you ideally like at closing?

Use a **range selector**.

Example:

* Under $500K
* $500K–$1M
* $1M–$2M
* $2M–$3M
* $3M–$5M
* $5M–$10M
* $10M+
* I don't know yet

For larger or unusual transactions, allow:

**Enter my own range**

### Then:

**How important is this range?**

☐ **This is nonnegotiable**

> This means you currently consider reaching this minimum a must-have.

The data model should retain:

**Desired Cash at Closing**

* Minimum
* Maximum
* Approximate / exact
* Nonnegotiable

---

# 6. IMPORTANT UX DECISION: DON'T FORCE A NUMBER

If the owner says:

**I don't know yet**

the application should respond:

> That's okay. We can explore your financial goals first and help you think through possible ranges later.

Continue.

The owner should never be blocked because they don't know the value of their business.

---

# 7. SCREEN 4: FUTURE INCOME

Only display this if relevant.

## Header

# Would you like the business transition to provide income after closing?

Choices:

**Yes**

**Maybe**

**No**

**I'm not sure**

If yes:

### About how much annual income would you ideally like?

Use ranges:

* Under $25K
* $25K–$50K
* $50K–$100K
* $100K–$250K
* $250K+
* I don't know yet

Then:

### For approximately how long?

* 1–3 years
* 3–5 years
* 5–10 years
* 10+ years
* Ongoing
* I'm not sure

Then:

☐ **This is nonnegotiable**

The system stores:

**Desired Future Income**

* Amount range
* Duration range
* Nonnegotiable
* Confidence

---

# 8. SCREEN 5: WHO OWNS THE COMPANY AT THE END?

This is one of the most important Destination fields.

## Header

# Picture the company after your transition.

### Who would you like to have ownership?

**Select all that should have ownership.**

☐ All employees
☐ Management
☐ A specific employee group
☐ Family members
☐ Existing owners
☐ Outside investors
☐ Other
☐ I'm not sure

This is a **multi-select**.

---

# 9. SCREEN 6: HOW SHOULD EMPLOYEE OWNERSHIP LOOK?

This question only appears if employees or management were selected.

## Header

# How would you like employee ownership to work?

Present a few conceptual choices:

### Employees broadly own the company

**I want ownership to be broadly shared among employees.**

### Employees + management

**I want employees to own part of the company and management to have additional ownership.**

### Employee ownership with other owners

**I want employees to own part of the company alongside other owners.**

### I'm not sure

**I know I want employee ownership, but I don't know what the ownership mix should look like.**

The platform should not call these legal structures.

They are merely descriptions of the owner's desired outcome.

---

# 10. SCREEN 7: OPTIONAL OWNERSHIP ALLOCATION

If the owner wants to provide more detail:

## Header

# Do you have an approximate ownership split in mind?

### Example

Employees: **60%**

Management: **30%**

Other: **10%**

Use a **percentage allocation control** totaling 100%.

But make this optional.

### Choice

**Set approximate percentages**

**I only know who should participate**

**I'm not sure**

Important text:

> These percentages describe your desired ownership outcome. They are not a proposed legal ownership structure.

This prevents the owner from confusing an aspiration with a legal structure.

---

# 11. SCREEN 8: YOUR ROLE AFTER THE TRANSITION

## Header

# On the day the transition is complete, what would you like your role to be?

### Retired

**I'm ready to step away completely.**

### Transition Advisor

**I'd like to help for a limited period.**

### Ongoing Advisor

**I'd like to remain available occasionally.**

### Continuing Owner / Operator

**I'd like to remain meaningfully involved.**

### I'm not sure

Then:

☐ **This is nonnegotiable**

If the user selects a transition period, that can be handled later.

---

# 12. SCREEN 9: WHEN SHOULD THIS HAPPEN?

## Header

# When would you ideally like to reach this destination?

### Choices

**Within 12 months**

**1–3 years**

**3–5 years**

**More than 5 years**

**I'm flexible**

**I'm not sure**

Then:

☐ **This is nonnegotiable**

The system stores a **desired timeframe**, not a guaranteed closing date.

---

# 13. SCREEN 10: WHAT SHOULD REMAIN TRUE?

This is the **Preservation Goals** section.

## Header

# What would you like to remain true about the company after the transition?

**Select up to 3 initially.**

Possible choices:

* Employees remain with the company
* Company remains independent
* Company stays in the same location
* Company culture remains
* Current leadership remains
* Brand remains
* Family remains involved
* Community presence remains
* Company continues serving existing customers
* Other
* I'm not sure

Once selected:

### Which matters most?

Rank only the selected items.

Then allow:

☐ **This is nonnegotiable**

on each applicable item.

This creates:

**Preservation Priorities**

rather than an enormous ranking exercise.

---

# 14. SCREEN 11: WHAT DO YOU WANT TO AVOID?

## Header

# What would you like to avoid?

**Select any that apply.**

Examples:

* Selling to an outside buyer
* Losing employee ownership
* Remaining involved long-term
* Leaving employees behind
* Excessive debt
* Waiting many years for proceeds
* Moving the business
* Losing company independence
* Major disruption to operations
* Other
* Nothing specific
* I'm not sure

Important:

This is not merely a preference list.

If appropriate, an owner can mark:

☐ **This is nonnegotiable**

For example:

> **I do not want to remain involved for more than two years.**

That becomes a hard owner constraint.

---

# 15. SCREEN 12: ANYTHING ELSE?

## Header

# Is there anything important we haven't asked?

Optional free text.

Prompt:

> Tell us anything else you want your future outcome to include.

Examples might include:

> "I want the business to stay in San Francisco."

> "I want my children to benefit financially even though employees own the company."

> "I want the employees to have real control, not just an economic interest."

This field should be optional.

It should be called:

**Additional Owner Context**

not:

**Advice**

---

# 16. SCREEN 13: DESTINATION REVIEW

This is the most important screen in the builder.

The platform now turns the answers into a visual story.

# Your Desired Destination

### The picture you are trying to create

> **You want to transition ownership of your business to your employees within approximately 18 months. You would ideally like to receive $3–5 million at closing and continue receiving approximately $75,000–$100,000 per year for 5–10 years. You would like employees and management to have ownership after the transition, while preserving the company's workforce and independence. You would like to retire from day-to-day operations.**

Then show:

### 💰 Financial Outcome

**Closing:** $3M–$5M
🔴 Nonnegotiable minimum: $3M

**Future income:** $75K–$100K/year
**Duration:** 5–10 years

### 👥 Ownership Outcome

Employees: ✓
Management: ✓
Other: ✓

### 🧑‍💼 Your Role

Retired from day-to-day operations

### 📅 Timing

Within 18 months

### 🏢 What You Want Preserved

Employees
Independence
Company culture

### 🔴 Your Must-Haves

Employee ownership
$3M minimum at closing
Retirement within 18 months

---

# 17. DESTINATION CHECKPOINT

At the bottom:

### Does this look like the outcome you're trying to create?

**Yes, this is it**

**Edit my destination**

**I'm not sure yet**

If "I'm not sure yet":

> That's okay. We'll keep this as a working destination and refine it as we explore your options.

---

# 18. CREATE THE DESTINATION OBJECT

When the user confirms, the application creates:

## DesiredOutcome / Destination

Conceptually:

```text id="j4i1gq"
DesiredOutcome
│
├── Financial
│   ├── ClosingProceedsRange
│   ├── FutureIncomeRange
│   └── FutureIncomeDuration
│
├── Ownership
│   ├── OwnershipParticipants
│   ├── DesiredAllocation
│   └── EmployeeOwnershipPreference
│
├── Personal
│   ├── DesiredOwnerRole
│   └── DesiredTransitionTiming
│
├── Preservation
│   ├── DesiredOutcomes
│   └── RankedPriorities
│
├── Avoidances
│   └── DesiredAvoidances
│
├── Constraints
│   └── Nonnegotiables
│
├── Preferences
│   └── StrongPreferences
│
├── Context
│   └── AdditionalOwnerContext
│
└── VersionHistory
```

---

# 19. DESTINATION OBJECT VS. BUSINESS FACTS

The Destination Builder should **not** mix these.

### Destination

> "I want $3–5M at closing."

### Later Business Assessment

> "The business currently generates $1.4M EBITDA."

The first is what the owner wants.

The second is what is currently true.

That distinction is essential.

---

# 20. DESTINATION OBJECT VS. PLATFORM SCENARIO

The Destination says:

> **"I want $3–5M at closing and employee ownership."**

The Platform Scenario later says:

> **"Under the assumptions currently entered, Scenario A could potentially provide $3–5M at closing while creating employee ownership."**

Those must remain separate objects.

---

# 21. DESTINATION OBJECT VS. PROFESSIONAL DETERMINATION

Later, the professional may say:

> **"Based on the company's actual financials, valuation, tax situation, and applicable transaction requirements, we recommend investigating Structure X."**

That becomes:

**Professional Determination**

It does not overwrite the Desired Outcome.

The owner's destination remains:

> **What the owner wants.**

---

# 22. PERSISTENT DESTINATION PANEL

After the Destination Builder is complete, the destination should remain visible throughout the rest of the journey.

For example:

## Your Destination

💰 **$3M–$5M at closing**

💵 **$75K–$100K/year for 5–10 years**

👥 **Employee ownership**

🧑‍💼 **Retire from daily operations**

📅 **18-month target**

🔴 **3 must-haves**

The owner can select:

**View / Edit Destination**

at any time.

---

# 23. DESTINATION VERSIONING

If the owner changes the destination:

### Destination v1

$3M–$5M at closing
18-month transition

↓

Owner changes target

### Destination v2

$2.5M–$5M at closing
24-month transition

The system retains both.

The Professional Review Package can show:

> **Destination revised September 24, 2026**

This is important because owner objectives can evolve as information becomes available.

---

# 24. DESTINATION CHANGE SHOULD TRIGGER A REVIEW

If the owner changes a nonnegotiable, the system should say:

> **You've changed one of your must-haves.**

Then:

> Some scenarios you've already explored may be affected.

### Choices

**Update My Scenarios**

**Review Before Continuing**

The platform does not automatically discard anything.

---

# 25. DESTINATION COMPLETION SCORE

I would **not** create a numerical "quality score" for the owner's destination.

Instead, use a simple completeness indicator.

### Your Destination

**Financial outcome** ✓
**Ownership outcome** ✓
**Personal outcome** ✓
**Timing** ✓
**Preservation goals** ✓

**Destination ready for exploration**

This avoids false precision.

---

# 26. WHAT THE OWNER HAS ACHIEVED

At the end of the Destination Builder, the owner should understand:

### I know:

**What I want financially**

**Who I want to own the company**

**What I want my role to be**

**When I want it to happen**

**What I want preserved**

**What I want to avoid**

**What I will not compromise**

And the platform should tell them:

> **Now we'll work backward from your destination. We’ll ask only the questions we need to explore potential paths for getting there.**

---

# 27. NEXT JOURNEY

The next stage is **Business Discovery / Current State**.

Only now should the platform begin asking:

* How large is the business?
* How many employees?
* What is approximate cash flow?
* How dependent is the business on the owner?
* What management exists?
* What debt exists?
* What information is available?

The purpose of those questions is now clear:

> **We are not asking about your business just to collect information. We are asking because we need to understand whether and how your desired destination might potentially be approached.**

---

# 28. FINAL DESTINATION BUILDER PRINCIPLE

The owner should finish this stage thinking:

> **"I can see the outcome I want."**

Not:

> **"I have filled out a questionnaire."**

The platform's job at this stage is simply to capture the owner's picture of success.

**Destination first.
Current reality second.
Possible paths third.
Professional determination fourth.**
