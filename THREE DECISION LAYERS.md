# THREE DECISION LAYERS

The platform must maintain three distinct layers throughout the entire ownership-transition journey.

These layers are fundamental to the product architecture and must never be blended.

---

## 1. OWNER OBJECTIVE

### Definition

**What the owner wants.**

This represents the business owner's goals, priorities, preferences, constraints, and desired outcomes.

Examples:

* Maximize cash at closing
* Generate income over time
* Retire within two years
* Transition ownership to employees
* Preserve jobs
* Remain involved for three years
* Minimize disruption
* Maximize after-tax proceeds
* Preserve the company's independence

The Owner Objective is the **source of truth for the owner's wishes**, not a professional conclusion.

### Example

> **Owner Objective:**
> I want to transition ownership to my employees, retire within 18 months, receive substantial cash at closing, and retain some income after the transaction.

---

# 2. PLATFORM SCENARIO

### Definition

**What might potentially accomplish the owner's objective under the assumptions currently provided.**

The platform can organize information, model scenarios, compare alternatives, identify potential transaction structures, and explain trade-offs.

However, the platform must clearly identify these as **exploratory scenarios**, not professional advice.

Examples:

* ESOP scenario
* Direct employee purchase scenario
* Management buyout scenario
* Employee-owned acquisition entity
* Staged ownership scenario
* Seller-financing scenario
* Bank + seller-note scenario

### Example

> **Platform Scenario:**
> Based on the information provided, an employee ownership structure combined with significant third-party financing and a seller note could potentially provide substantial cash at closing while maintaining employee ownership.

Then clearly state:

> **This is an exploratory scenario based on user-provided information and assumptions. It has not been professionally validated.**

---

# 3. PROFESSIONAL DETERMINATION

### Definition

**What the qualified professional concludes should actually be done.**

This is outside the platform's authority.

A professional may:

* Validate a scenario
* Modify it
* Reject it
* Recommend another structure
* Identify additional requirements
* Identify risks
* Determine appropriate professional treatment
* Provide legal, tax, valuation, investment, financing, fiduciary, or other specialized advice within their role

### Example

> **Professional Determination:**
> The owner's CPA, attorney, valuation professional, and other appropriate advisors reviewed the preliminary scenarios and determined which structures warrant further development.

The platform records that determination but does not manufacture it.

---

# 4. NEVER BLEND THE THREE LAYERS

The application should always make it obvious whether the user is looking at:

🟦 **OWNER OBJECTIVE**

🟨 **PLATFORM SCENARIO**

🟩 **PROFESSIONAL DETERMINATION**

A platform-generated scenario must never appear to be a professional recommendation.

An owner's preference must never be represented as a professional conclusion.

A professional's conclusion must be attributable to the appropriate professional.

---

# 5. THE JOURNEY SHOULD MOVE THROUGH THE THREE LAYERS

The basic product flow becomes:

**OWNER OBJECTIVE**

↓

**PLATFORM SCENARIOS**

↓

**OWNER EXPLORES / MODIFIES**

↓

**PROFESSIONAL REVIEW**

↓

**PROFESSIONAL DETERMINATION**

↓

**REVISED / VALIDATED PLAN**

↓

**TRANSACTION EXECUTION**

This provides a clean separation between:

**What I want**

**What the software shows me**

and

**What my professional advises me to do**

---

# 6. OWNER OBJECTIVE SHOULD NEVER DISAPPEAR

Even after professionals become involved, the original objectives should remain visible.

For example:

## Your Objectives

**Primary:** Employee ownership

**Secondary:** Significant cash at closing

**Secondary:** Retirement within 18 months

**Preference:** Ongoing income

A professional may determine that one or more of these objectives must change.

The platform should record:

**Original Objective**

**Professional Feedback**

**Revised Objective**

This makes changes transparent.

---

# 7. PLATFORM SCENARIOS SHOULD RETAIN THEIR ASSUMPTIONS

Every scenario should carry its assumptions.

For example:

### Scenario: Employee Purchase + Seller Financing

**Assumptions**

* Estimated business value: $5M
* Estimated EBITDA: $1.2M
* Cash at closing: $2.5M
* Seller note: $2.5M
* Interest: 8%
* Term: 7 years

The platform should clearly identify that these are **assumptions supplied by the user or generated for exploratory modeling**, not final transaction terms.

When an assumption changes, the scenario updates.

---

# 8. PROFESSIONAL DETERMINATIONS SHOULD BE ATTRIBUTED

Where the platform records professional input, it should identify:

**Professional**

**Firm**

**Role**

**Date**

**Determination / Guidance**

**Related Scenario**

For example:

> **CPA Review**
> Smith Transaction Advisory
> September 2026
>
> Reviewed Scenario B. Recommended further analysis of the tax consequences before proceeding.

This creates a clear record without turning the platform into the advisor.

---

# 9. PROFESSIONAL REVIEW PACKAGES SHOULD CONTAIN ALL THREE

This should become a major output of the platform.

## Professional Review Package

### PART I: OWNER OBJECTIVE

What I am trying to accomplish.

### PART II: PLATFORM SCENARIOS

What I have explored.

### PART III: QUESTIONS FOR PROFESSIONALS

What I need validated.

### PART IV: PROFESSIONAL DETERMINATIONS

What my advisors have concluded.

### PART V: CURRENT WORKING PLAN

The current direction after professional review.

---

# 10. CREATE A "STATUS" FOR THE PLAN

Every plan should have a clear status.

### 🟦 Exploring

Owner objectives have been entered.

### 🟨 Scenario Development

Potential structures are being explored.

### 🟧 Professional Review

The owner has sent the plan to professionals.

### 🟩 Professionally Reviewed

Relevant professionals have reviewed the applicable portions.

### 🟢 Transaction Planning

The professional team has established a transaction direction.

### 🔵 Execution

The transaction is being implemented.

These labels describe **process status**, not a judgment about whether a transaction is good or bad.

---

# 11. CHANGE MANAGEMENT

When a user changes an objective, the system should not erase history.

Example:

**Original Objective**

> Maximum cash at closing

↓

**New Objective**

> Higher long-term income

The platform creates a new scenario rather than destroying the old one.

Professionals can see what changed and why.

This allows:

**Scenario A**

**Scenario B**

**Scenario C**

to coexist.

The owner can return to any scenario.

---

# 12. PROFESSIONAL FEEDBACK SHOULD CREATE NEW SCENARIOS

Suppose the platform generated:

### Scenario A

Direct employee purchase

A professional reviews it and says:

> Consider an ESOP structure as an alternative.

The platform should not simply overwrite Scenario A.

Instead:

**Professional feedback → New Scenario**

### Scenario B

ESOP

Now the owner can compare both paths.

This preserves the exploratory nature of the application.

---

# 13. THE PLATFORM IS A TRANSLATION LAYER

The three-layer model means the application translates between:

### Owner

**"This is what I want."**

### Platform

**"Here are some ways that might potentially work."**

### Professional

**"Given the actual facts and my professional responsibilities, here's what I believe should happen."**

Then the platform brings that professional determination back into the owner's journey in an understandable form.

---

# 14. CORE DATA MODEL

These should eventually become explicit objects in the application.

### OwnerObjective

* Goal
* Priority
* Preference
* Constraint
* Timeline
* Desired outcome

### PlatformScenario

* Scenario type
* Assumptions
* Inputs
* Outputs
* Trade-offs
* Date created
* Owner modifications
* Status

### ProfessionalDetermination

* Professional
* Role
* Date
* Related scenario
* Guidance
* Determination
* Supporting documents
* Follow-up actions

### ProfessionalReviewPackage

* Owner objectives
* Selected scenarios
* Assumptions
* Questions
* Professional responses
* Current working plan
* Version history

This structure should exist in the product even if the underlying implementation changes.

---

# 15. USER EXPERIENCE RULE

Every major screen should answer:

> **Whose information am I looking at?**

The interface should visually and verbally distinguish:

**What you told us**

from

**What the platform calculated**

from

**What your professional told you**

That clarity is a core trust feature.

---

# 16. NORTH STAR

The product should ultimately produce this progression:

> **I know what I want.**

→ **I understand the possible paths.**

→ **I know what questions to ask.**

→ **My professionals have reviewed those paths.**

→ **We agree on the direction.**

→ **We can execute the transaction.**

The platform does not replace professional judgment.

**It makes professional judgment more useful by ensuring everyone starts from the same clearly documented owner objectives.**
