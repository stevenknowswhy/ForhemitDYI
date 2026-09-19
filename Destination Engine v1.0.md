Yes. The **Destination Engine** is foundational because it creates the first meaningful object in every journey: **Desired Outcome / Destination**.

I would make it completely independent from the Journey Engine. The Journey Engine asks questions and navigates. The Destination Engine owns the meaning, structure, versioning, and lifecycle of **what the owner wants**.

# Destination Engine v1.0

## Desired Outcome / Destination

## 1. Purpose

The Destination Engine is a standalone engine responsible for capturing, structuring, maintaining, versioning, and presenting the owner's desired end state.

It answers one fundamental question:

> **"When this transition is complete, what does success look like to the owner?"**

The Destination Engine does **not** determine whether the destination is realistic, advisable, legally possible, tax-efficient, financeable, or achievable.

Those questions belong to other engines and qualified professionals.

---

# 2. CORE PRINCIPLE

Every ownership-transition journey begins with a:

# Desired Outcome / Destination

The owner defines the destination **before** the platform attempts to determine how to get there.

The system then uses the destination as the persistent North Star for:

* Business Reality
* Research
* Goal-to-Reality Confidence
* Scenario development
* Professional Review
* Professional Review Packages
* Transaction planning

The destination remains editable throughout the journey.

---

# 3. DESTINATION VS. OBJECTIVE

The Destination Engine should distinguish between:

### Destination

The owner's desired end state as a whole.

> "I want to retire in 18 months, receive $3–5M at closing, receive $75K–$100K annually for 5–10 years, and have my employees and management own the company."

### Objective

An individual component of that destination.

> "Receive substantial cash at closing."

### Priority

How important one objective is relative to another.

### Nonnegotiable

A requirement the owner currently considers a must-have.

### Preference

Something the owner wants but is willing to change.

The Destination is the **whole picture**.

---

# 4. DESTINATION DATA MODEL

The primary object is:

## `DesiredOutcome`

Conceptually:

```text
DesiredOutcome
│
├── Identity
│   ├── destination_id
│   ├── owner_id
│   ├── business_id
│   ├── journey_id
│   └── version
│
├── Financial Outcome
│   ├── closing_proceeds
│   ├── future_income
│   ├── future_income_duration
│   └── other_financial_goals
│
├── Ownership Outcome
│   ├── ownership_participants
│   ├── desired_allocations
│   └── ownership_characteristics
│
├── Personal Outcome
│   ├── desired_owner_role
│   ├── desired_transition_timing
│   └── lifestyle / involvement preferences
│
├── Business Outcome
│   ├── preservation_goals
│   ├── continuity_goals
│   └── cultural goals
│
├── Avoidances
│   └── undesired_outcomes
│
├── Constraints
│   └── nonnegotiables
│
├── Preferences
│   ├── strong_preferences
│   └── flexible_preferences
│
├── Context
│   ├── owner_narrative
│   └── additional_context
│
├── Completeness
│
├── Status
│
└── Version History
```

---

# 5. DESTINATION IDENTITY

Every destination receives a permanent identifier.

It should contain:

* Destination ID
* Owner ID
* Business ID
* Journey ID
* Creation date
* Current version
* Status
* Last updated
* Created by
* Current owner confirmation

This allows the Destination to survive across the entire transaction lifecycle.

---

# 6. DESTINATION STATUS

Suggested states:

### Draft

The owner is still constructing the destination.

### Working

The owner has created a usable destination but expects it may change.

### Confirmed

The owner has explicitly confirmed:

> "This is the outcome I'm trying to create."

### Under Professional Review

A professional is evaluating the destination.

### Revised

The owner has materially changed the destination after additional information or professional feedback.

### Superseded

A newer destination version has replaced this version.

### Archived

The destination is retained historically but is no longer active.

These describe process state, not feasibility.

---

# 7. FINANCIAL OUTCOME

The financial component should support multiple forms of expression.

## Closing Proceeds

Support:

* Range
* Minimum
* Maximum
* Exact amount, if known
* Unknown

Example:

**$3M–$5M**

The owner can designate:

**Minimum $3M = Nonnegotiable**

while treating the upper end as a preference.

---

# 8. FUTURE INCOME

Capture:

* Desired amount
* Amount range
* Frequency
* Duration
* Start timing
* Nonnegotiable status

Example:

**$75K–$100K annually**

for:

**5–10 years**

The platform should not imply that this income will come from a specific transaction mechanism.

The owner is specifying the outcome, not the financing method.

---

# 9. OWNERSHIP OUTCOME

Capture:

### Who should own the company after transition?

Support:

* Employees
* Management
* Specific employee groups
* Family
* Existing owners
* Outside investors
* Other

Allow multiple selections.

Then optionally:

### Approximate desired ownership allocation

For example:

**Employees: 60%**

**Management: 30%**

**Other: 10%**

The system must clearly label this as:

> **Owner's desired ownership outcome**

not:

> **Proposed legal ownership structure**

---

# 10. OWNERSHIP CHARACTERISTICS

The owner may care about more than percentages.

Potential attributes:

* Broad employee participation
* Meaningful employee ownership
* Management participation
* Employee voting/control preferences
* Continued family ownership
* Company independence
* Local ownership

These should be treated as desired characteristics rather than legal conclusions.

---

# 11. PERSONAL OUTCOME

Capture:

### Desired owner role

* Fully retired
* Temporary transition advisor
* Ongoing advisor
* Continued owner/operator
* Other
* Not sure

### Desired transition period

* Immediate
* Less than 6 months
* 6–18 months
* 18–36 months
* Longer
* Flexible

### Nonnegotiable

The owner can mark the desired role or transition timing as a must-have.

---

# 12. TIMING

The Destination Engine stores:

### Desired transition date

or:

### Desired timeframe

Examples:

* Within 12 months
* 1–3 years
* 3–5 years
* More than 5 years
* Flexible

It should distinguish:

**Target date**

from:

**Hard deadline**

---

# 13. BUSINESS PRESERVATION

The destination can include what the owner wants preserved.

Potential goals:

* Employees remain
* Company remains independent
* Company remains in current location
* Culture remains
* Brand remains
* Current leadership remains
* Community presence remains
* Existing customers continue being served
* Family remains involved

The owner can:

**Select up to 3**

then:

**Rank them**

when appropriate.

Each can optionally be:

**Nonnegotiable**

---

# 14. WHAT THE OWNER WANTS TO AVOID

The Destination Engine should support negative goals.

Examples:

* Selling to an outside buyer
* Losing employee ownership
* Remaining involved long-term
* Excessive debt
* Long transition period
* Major workforce disruption
* Moving the company
* Losing independence

These become:

## `DesiredAvoidance`

An avoidance can also be marked:

**Nonnegotiable**

Example:

> **Do not require the owner to remain active for more than two years.**

---

# 15. PRIORITIES

The Destination Engine must preserve relative importance.

For example:

### Primary

Employee ownership

### Secondary

Retirement within 18 months

### Secondary

$3M+ cash at closing

### Preference

Ongoing income

The engine should not assume every selected outcome has equal weight.

---

# 16. NONNEGOTIABLES

Nonnegotiability is a first-class property.

For every applicable destination component:

```text
Preference Level:
- Flexible
- Preference
- Strong Preference
- Nonnegotiable
```

The system should default to:

**Unspecified / No designation**

rather than guessing importance.

When the user explicitly selects:

**This is nonnegotiable**

the item becomes a hard owner constraint for downstream engines.

---

# 17. NONNEGOTIABLES MUST NEVER BE SILENTLY CHANGED

The Destination Engine may receive feedback from:

* Confidence Engine
* Scenario Engine
* Research Engine
* Professionals

But none may silently alter the owner's destination.

If new information creates a conflict:

> **This may be difficult to satisfy under the current assumptions.**

The owner then chooses:

**Keep it**

**Change it**

**Explore alternatives**

---

# 18. DESTINATION NARRATIVE

The engine should generate a human-readable description of the destination.

Example:

> **"When this transition is complete, I want to have transferred meaningful ownership of the company to my employees and management, receive approximately $3–5 million at closing, continue receiving approximately $75,000–$100,000 annually for 5–10 years, and retire from day-to-day operations within 18 months. I would like the company to remain independent and preserve its current workforce."**

This narrative should be generated **from structured owner data**.

It must not invent objectives.

---

# 19. OWNER CONFIRMATION

After generating the narrative:

### Does this accurately describe the outcome you're trying to create?

**Yes, this is right**

**Edit my destination**

**Something is missing**

The owner explicitly confirms the destination.

---

# 20. DESTINATION COMPLETENESS

Do not use a generic "score."

Instead, identify the areas that have been established.

Example:

### Destination Status

**Financial outcome** ✓
**Ownership outcome** ✓
**Personal outcome** ✓
**Timing** ✓
**Preservation goals** ✓
**Avoidances** ✓

### Destination

**Ready for exploration**

Another owner might have:

**Financial outcome** ✓
**Ownership outcome** ✓
**Personal outcome** ?
**Timing** ?
**Preservation goals** ?

### Destination

**Working draft**

---

# 21. DESTINATION CONFIDENCE IS NOT OWNED BY THIS ENGINE

The Destination Engine should not calculate whether the destination is realistic.

It passes the structured destination to:

**Research Engine**

and:

**Confidence Engine**

Those engines provide independent analysis.

This maintains:

> **Destination = owner truth**

rather than:

> **Destination = platform judgment**

---

# 22. DESTINATION + BUSINESS REALITY

The two should always remain separate.

### Destination

**$3M–$5M at closing**

### Business Reality

**Current business information**

The Destination Engine does not compare these.

The Confidence Engine does.

---

# 23. DESTINATION + SCENARIO ENGINE

The Scenario Engine receives the destination and asks:

> **What possible routes could potentially satisfy this desired outcome?**

The Destination Engine does not generate those routes.

---

# 24. DESTINATION + RESEARCH ENGINE

The Research Engine receives structured questions such as:

> "How does this desired closing-proceeds range compare with available evidence for businesses with these characteristics?"

The Destination Engine simply provides the owner's target.

---

# 25. DESTINATION + PROFESSIONAL REVIEW

Professionals should be able to see:

### Owner's Desired Destination

and distinguish:

**Owner wants**

from:

**Platform explored**

from:

**Professional determined**

That distinction must survive into every professional package.

---

# 26. DESTINATION VERSIONING

Every material change creates a new version.

Example:

### Destination v1

$3M–$5M closing
18-month target

↓

### Destination v2

$2.5M–$5M closing
24-month target

The previous version remains accessible.

---

# 27. VERSION CHANGE REASONS

When a material destination change occurs, ask:

### Why are you changing this?

**I learned something new**

**My priorities changed**

**My professional suggested another approach**

**The business changed**

**I want to explore a different outcome**

**Other**

Optional explanation.

This creates valuable context without forcing the user to write an essay.

---

# 28. DESTINATION CHANGE IMPACT

After a change:

> **Changing your destination may affect scenarios and research you've already completed.**

Then:

**Update My Exploration**

**Review Before Updating**

The engine sends a change event to relevant downstream systems.

---

# 29. DESTINATION CHANGE EVENT

Conceptually:

```text
DestinationChanged
{
    destination_id
    previous_version
    new_version
    changed_fields
    changed_constraints
    changed_preferences
    reason
    timestamp
}
```

Other engines subscribe to this event.

For example:

**Scenario Engine**

→ refresh affected scenarios.

**Confidence Engine**

→ recalculate Goal Alignment.

**Research Engine**

→ identify stale or affected research.

**Document Readiness Engine**

→ identify changed information requirements.

**Professional Review Package Engine**

→ mark previous package as superseded where appropriate.

---

# 30. NO CASCADE DESTRUCTION

Changing the Destination must not destroy historical data.

It should create:

**New version**

and mark affected downstream objects as potentially stale.

Example:

### Scenario v4

Based on:

**Destination v2**

If the owner creates:

**Destination v3**

the Scenario Engine can flag:

> **Scenario v4 was built against an earlier destination. Review recommended.**

---

# 31. DESTINATION SNAPSHOT

At important milestones, capture a snapshot.

For example:

### At Professional Review

**Destination v4**

### At Transaction Planning

**Destination v5**

### At Closing

**Final acknowledged destination**

This provides historical context.

---

# 32. LOCAL-FIRST REQUIREMENTS

The Destination itself is relatively low sensitivity compared with the owner's source documents, but privacy controls still apply.

Potentially sensitive destination elements may include:

* Desired proceeds
* Personal income goals
* Retirement timing
* Family objectives

The owner should control sharing.

The destination remains part of the owner's private workspace until explicitly shared or placed into a stakeholder package.

---

# 33. DESTINATION SHARING

The Destination Engine should expose structured data to the Stakeholder Document Engine.

The Document Engine determines:

**What this recipient gets.**

For example:

### Employee

High-level ownership objective.

### CPA

Financial objectives.

### Attorney

Ownership and transaction objectives.

### Financial advisor

Liquidity and income objectives.

The Destination Engine does not decide disclosure.

---

# 34. DESTINATION SEARCH

The owner should eventually be able to search:

**My Goals**

**My Destinations**

**Previous Versions**

**Scenarios Based on This Destination**

**Professional Packages**

This becomes useful across a multi-year transaction.

---

# 35. DESTINATION COMPARISON

The engine should support comparing versions.

Example:

|                    | v1            | v2              |
| ------------------ | ------------- | --------------- |
| Closing target     | $3–5M         | $2.5–5M         |
| Income             | $100K × 5 yrs | $75K × 5–10 yrs |
| Transition         | 18 mo.        | 24 mo.          |
| Employee ownership | Must-have     | Must-have       |

This allows the owner to see how their thinking evolved.

---

# 36. DESTINATION AUDITABILITY

Every meaningful change should record:

* Previous value
* New value
* Who changed it
* Date
* Reason
* Journey stage
* Related scenario
* Related research
* Related professional feedback

This belongs to the engine's audit interface, while the broader Audit Engine maintains the overall platform audit trail.

---

# 37. DESTINATION API / INTERFACE

The engine should expose operations conceptually such as:

```text
createDestination()
getDestination()
updateDestination()
confirmDestination()
cloneDestinationVersion()
compareDestinationVersions()
setNonnegotiable()
removeNonnegotiable()
addObjective()
removeObjective()
setPriority()
addAvoidance()
generateNarrative()
getDestinationSnapshot()
publishDestinationEvent()
```

The actual technology can be selected later.

The interface should remain stable even if implementation changes.

---

# 38. DESTINATION ENGINE INPUTS

Primary inputs:

* Journey responses
* Owner edits
* Explicit confirmations
* Owner notes
* Professional feedback that the owner chooses to incorporate

The engine should not independently invent destination information.

---

# 39. DESTINATION ENGINE OUTPUTS

Primary outputs:

### Structured Destination

Machine-readable owner goals.

### Human-readable Destination

Owner narrative.

### Priority Map

Relative importance.

### Constraint Map

Nonnegotiables.

### Preference Map

Strong/flexible preferences.

### Avoidance Map

Undesired outcomes.

### Destination Version

Historically traceable version.

### Destination Snapshot

Milestone-specific copy.

### Change Events

Notifications to dependent engines.

---

# 40. DESTINATION ENGINE ERROR PREVENTION

The engine should guard against:

### Contradictory owner inputs

Example:

**Must retire immediately**

and:

**Must remain active for 5 years**

The system asks:

> **These goals may conflict. Which should take priority?**

### Invalid allocation

Ownership percentages total 110%.

The system says:

> **Your allocation currently totals 110%.**

It does not fix the percentages automatically.

### Ambiguous duration

"Several years."

The system can ask:

**Would you like to leave this flexible?**

### Missing context

The owner selects:

**Income over time**

but doesn't specify amount.

Allow:

**Not decided yet**

---

# 41. DESTINATION ENGINE SHOULD NOT OVERQUESTION

If the owner has communicated enough to establish a destination, stop.

Do not force:

* Exact proceeds
* Exact percentages
* Exact date
* Exact income
* Exact post-close role

when the owner does not know them.

Use:

**Unknown**

**Flexible**

**Approximate**

rather than manufacturing precision.

---

# 42. DESTINATION BUILDER CONFIGURATION

The Destination Engine should work with the Journey Builder.

The Journey Builder defines:

**What questions to ask.**

The Destination Engine defines:

**How the answers map into the DesiredOutcome object.**

Example:

```text
Journey Question
"What would you like to receive at closing?"

        ↓

Destination Mapping

financial.closing_proceeds
```

Another:

```text
Journey Question
"Who should own the company?"

        ↓

Destination Mapping

ownership.participants
```

This keeps the engines separate.

---

# 43. DESTINATION ENGINE SHOULD BE REUSABLE

The structure should work for future journeys.

For example:

### Employee Ownership

Destination:

Employees own the company.

### Management Buyout

Destination:

Management owns the company.

### Family Succession

Destination:

Family members continue ownership.

The Destination Engine does not need to understand the transaction structure.

It only understands:

> **What does the owner want the future state to look like?**

---

# 44. DESTINATION REVIEW CARD

The owner should always be able to open:

# My Destination

### Financial

$3M–$5M at closing

$75K–$100K/year for 5–10 years

### Ownership

Employees + management

### Personal

Retired from daily operations

### Timing

Within 18 months

### Preserve

Employees
Culture
Independence

### Avoid

Long-term owner involvement

### Must-Haves

Employee ownership
$3M minimum at closing
18-month retirement target

### Version

v3

**Edit Destination**

---

# 45. DESTINATION SHOULD BE THE NORTH STAR THROUGHOUT THE APP

At every subsequent stage the user should be able to ask:

> **"How does this affect the destination I described?"**

For example:

### Research

Does new evidence affect an assumption?

### Scenario

Could this path potentially address the destination?

### Professional

What does my professional think about this path?

### Documents

What information is needed to evaluate it?

### Transaction

Are we still pursuing the owner's intended outcome?

---

# 46. FINAL ARCHITECTURAL RULES

The Destination Engine must follow these rules:

### Rule 1

**Owner goals belong to the owner.**

### Rule 2

**The platform never silently changes the owner's destination.**

### Rule 3

**A destination is not a recommendation.**

### Rule 4

**A destination is not a feasibility determination.**

### Rule 5

**Nonnegotiables are explicit owner constraints.**

### Rule 6

**Unknown is a valid state.**

### Rule 7

**Approximation is preferable to false precision.**

### Rule 8

**Every material change is versioned.**

### Rule 9

**Historical destinations remain available.**

### Rule 10

**Downstream engines consume the destination but do not own it.**

---

# 47. DESTINATION ENGINE NORTH STAR

The Destination Engine exists to help the owner say:

> **"This is what I want the finish line to look like."**

It then preserves that vision in a structured, editable, versioned form.

The rest of the platform works from that starting point:

**Business Reality asks where the owner is today.**

**Research asks what the evidence says.**

**Confidence asks how well the destination is currently supported.**

**Scenario asks what paths might connect today's reality to the desired destination.**

**Professionals determine what should actually be done.**

The Destination Engine simply keeps the destination clear.

---

# 48. ONE-SENTENCE DEFINITION

> **The Destination Engine is the system of record for the owner's desired future state, capturing what the owner wants, how important each element is, what is nonnegotiable, what is flexible, and how those objectives evolve over time.**

### One architectural distinction I especially want to preserve

The **Destination Engine owns the owner's intent**, while the **Journey Engine owns the conversation that discovers it**.

That means the Journey Engine can change completely someday, but the Destination object remains stable.

And because we made it reusable, an eventual **Family Succession Journey**, **Management Buyout Journey**, or another transition journey can use the same Destination Engine. The owner will still start with:

> **“Picture the day this is complete. What do you want to be true?”**

The engine doesn't care whether the eventual road is an ESOP, direct acquisition, staged transition, or something we haven't invented yet. It simply preserves the finish line.
