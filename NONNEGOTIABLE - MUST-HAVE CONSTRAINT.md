# NONNEGOTIABLE / MUST-HAVE CONSTRAINT

Every applicable journey question should have an optional toggle:

### **☐ This is nonnegotiable**

**This is a must-have for me to move forward.**

This allows the owner to distinguish between:

**Preference**
"I would like this."

**Strong Preference**
"This is very important to me."

**Nonnegotiable**
"I do not want to proceed with a path that does not satisfy this."

---

# 1. THE TOGGLE IS QUESTION-SPECIFIC

The nonnegotiable designation belongs to the **owner's answer**, not simply to the question.

For example:

### When would you ideally like to retire?

**Within 1 year**
☑ **This is nonnegotiable**

Another owner could answer:

**1–3 years**
☐ **This is nonnegotiable**

The same question supports very different constraints for different owners.

---

# 2. USE IT WHERE IT ACTUALLY MAKES SENSE

The toggle should be available for applicable questions involving:

* Goals
* Timing
* Ownership objectives
* Cash requirements
* Income requirements
* Continued involvement
* Employee participation
* Business preservation
* Other transaction preferences
* Constraints

It should not necessarily appear on every trivial factual question.

For example:

**How many employees do you have?**

does not need a nonnegotiable toggle.

But:

**Do you want to remain involved after the transaction?**

might.

---

# 3. NONNEGOTIABLE BECOMES A HARD CONSTRAINT

Once the owner explicitly marks something as nonnegotiable, the Scenario Engine must treat it as a **hard constraint**.

For example:

**Owner Objective**

Employee ownership

☑ **Nonnegotiable**

The platform should not generate a scenario that results in an external buyer owning the company and quietly present it as satisfying the owner's stated objective.

---

# 4. NONNEGOTIABLES SHOULD FILTER SCENARIOS

Example:

### Owner Requirements

**Employee ownership:** Nonnegotiable

**Retire within 12 months:** Nonnegotiable

**Maximum cash at closing:** Strong preference

The Scenario Engine evaluates potential paths against those conditions.

It may produce:

### Scenario A

**Appears compatible with your must-haves**

### Scenario B

**May not satisfy your retirement timeline**

### Scenario C

**May not satisfy employee ownership requirement**

This gives the owner useful information without the platform declaring a universal winner.

---

# 5. DO NOT SILENTLY COMPROMISE A NONNEGOTIABLE

This is one of the most important rules.

The system must **never quietly trade away a nonnegotiable** to make a scenario work.

Instead, say:

> **This scenario appears to conflict with one of your must-haves.**

Then identify the conflict.

### Example

**Your must-have**

Retire within 12 months.

**Scenario**

May require a longer owner transition period.

Then:

**Keep This Must-Have**

**Explore a Different Scenario**

**Change My Requirement**

The owner decides.

---

# 6. SHOW CONFLICTS CLEARLY

If two nonnegotiables conflict, the platform should surface that immediately.

For example:

**Must have #1:** Maximum possible cash at closing

**Must have #2:** Minimal business debt

The platform could say:

> **Potential conflict detected**
>
> These two requirements may be difficult to satisfy simultaneously under the current assumptions.
>
> Let's explore the trade-off.

Then the owner can investigate rather than having the application make the compromise for them.

---

# 7. NONNEGOTIABLES CAN CHANGE

Nothing about the journey is permanently locked unless the owner explicitly progresses to a legally or operationally binding stage outside the exploratory system.

The owner can select:

**Edit Requirement**

and change:

**Nonnegotiable → Strong Preference**

or:

**Nonnegotiable → Flexible**

The previous version should remain in history.

---

# 8. PRESERVE THE HISTORY

Example:

### Original

**Retire within 12 months**
🔴 Nonnegotiable

### Later

Owner changes it to:

**Retire within 24 months**
🟠 Strong Preference

The platform records:

* Original value
* Original status
* New value
* New status
* Date changed

This can be useful in the Professional Review Package.

---

# 9. PROFESSIONAL REVIEW PACKAGE

Nonnegotiables should appear prominently.

## Owner's Must-Haves

### 1. Employee ownership

**Nonnegotiable**

### 2. Retirement within 18 months

**Nonnegotiable**

### 3. Meaningful cash at closing

**Strong preference**

### 4. Ongoing income

**Preference**

This allows the professional team to immediately understand:

> **What can be negotiated and what the owner currently considers essential.**

---

# 10. PROFESSIONALS CAN CHALLENGE A NONNEGOTIABLE

The platform should not prevent a qualified professional from saying:

> "This requirement may not be achievable under the facts of the transaction."

That becomes:

**Professional Determination / Guidance**

rather than a platform override.

The user can then decide:

**Maintain requirement**

**Modify requirement**

**Explore alternatives**

This preserves the distinction between:

**Owner Objective**

and

**Professional Determination.**

---

# 11. NONNEGOTIABLE ≠ GUARANTEED OUTCOME

The interface should be careful with language.

When an owner selects:

> **"This is nonnegotiable."**

the system should interpret that as:

**A requirement the owner wants satisfied**

not:

**A promise that the platform can satisfy it.**

For example:

> **Your must-have:** $3M cash at closing.
>
> We will use this as a constraint while exploring scenarios. Whether that amount is achievable depends on valuation, financing, transaction structure, and professional review.

---

# 12. THREE PREFERENCE STATES

The platform can therefore model preferences as:

### 🔴 Nonnegotiable

Must-have

### 🟠 Strong Preference

Very important, but potentially changeable

### 🟢 Preference / Flexible

Desirable but negotiable

The owner should not be required to classify everything.

The default should be:

**No designation**

unless the owner explicitly indicates a level of importance.

---

# 13. CONSTRAINT ENGINE

The Scenario Engine should evaluate:

```text id="z9w81w"
Owner Objectives
        +
Preferences
        +
Nonnegotiable Constraints
        +
Business Facts
        ↓
Scenario Engine
        ↓
Potential Scenarios
        ↓
Constraint Check
        ↓
Compatible
Potential Conflict
Unable to Satisfy
```

Every scenario should carry its constraint status.

---

# 14. SCENARIO OUTPUT

A scenario should therefore say something like:

### Scenario A: Employee Ownership + Seller Financing

**Must-haves**

✓ Employee ownership

✓ Retirement target

⚠ Cash-at-closing goal may require additional financing analysis

**Strong preferences**

✓ Ongoing income

**Questions for professionals**

* Is the proposed structure feasible?
* What financing is available?
* What valuation is appropriate?
* What tax consequences should be evaluated?

Again, these are **questions and scenario observations**, not professional advice.

---

# 15. THE USER EXPERIENCE

The actual control should be extremely simple.

After a meaningful answer:

> **How important is this to you?**

**☐ This is nonnegotiable**

The user taps it.

A short explanation can appear:

> **We'll treat this as a must-have when exploring scenarios. If a scenario conflicts with it, we'll tell you rather than quietly changing the requirement.**

That is enough.

---

# 16. IMPORTANT DESIGN PRINCIPLE

The application should distinguish:

**What I want**

from

**What I strongly prefer**

from

**What I absolutely require**

This is critical because real-world transactions involve trade-offs.

The platform should help the owner identify those trade-offs without deciding which ones the owner should sacrifice.

---

# 17. UPDATED DECISION MODEL

We now have a richer model:

### FACT

What is true about the business.

### OWNER OBJECTIVE

What the owner wants.

### PRIORITY

What matters more than something else.

### PREFERENCE

What the owner would like.

### NONNEGOTIABLE

What the owner currently considers a must-have.

### PLATFORM SCENARIO

What might potentially satisfy those objectives and constraints under stated assumptions.

### PROFESSIONAL DETERMINATION

What qualified professionals conclude should actually be done.

That gives the Journey Engine a much more complete understanding of the owner.

---

# PRODUCT RULE

**A nonnegotiable is never silently relaxed, ignored, or overridden.**

If a scenario conflicts with it, the platform surfaces the conflict and lets the owner decide whether to:

**Keep the requirement**

**Explore another path**

or

**Change the requirement.**

The owner remains in control of the decision.
