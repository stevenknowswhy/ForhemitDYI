Yes. This is a very important engine because a transaction is not simply a sequence of system events. It is a sequence of **human decisions made with incomplete information that changes over time**.

The **Decision Record Engine** should preserve that journey without pretending that every decision was made with today's knowledge.

Its central question is:

> **“What did the owner decide, why did they decide it, what did they know at that moment, what alternatives did they consider, and what happened afterward?”**

That is fundamentally different from technical audit logging.

# Decision Record Engine

## 1. Purpose

The Decision Record Engine preserves the owner's substantive decision journey throughout the business ownership transition.

It records:

* What was decided
* Who decided
* Why it was decided
* When it was decided
* What information was available at the time
* What assumptions were being used
* What alternatives were considered
* What trade-offs were recognized
* What was uncertain
* What was explicitly rejected
* Whether the decision was nonnegotiable, preferred, or flexible
* What changed afterward
* What consequences followed
* Whether the decision was later revisited
* What new information caused reconsideration

Its purpose is not to judge decisions.

It is to preserve **decision context**.

---

# 2. Core Architectural Principle

## Audit records what the system did.

## Decision Records preserve why the owner decided what they did.

Example:

### Audit

> User changed target closing date from November 30 to December 15 at 2:14 PM.

### Decision Record

> Owner chose December 15 because the owner wanted additional time for employee communication and considered November 30 too compressed.

Those are completely different records.

Both are valuable.

---

# 3. Why This Engine Exists

Business transitions can take months or years.

During that time:

* Facts change
* Valuations change
* Financing conditions change
* Professional opinions change
* Employee circumstances change
* Owner priorities change
* Scenarios evolve
* New evidence appears

Without a decision record, the system eventually knows:

> “The owner chose Scenario B.”

but not:

> “Why did the owner choose Scenario B instead of Scenario A on September 19 when the available information looked like this?”

The Decision Record Engine preserves that context.

---

# 4. Decision-Time Reality

A decision should be understood using the information available **at the moment it was made**.

This is a critical rule.

Suppose the owner selects:

> Direct employee purchase

on September 19.

At that time:

* Valuation is preliminary.
* Financing is uncertain.
* Employee interest is not fully tested.
* Two professional reviews are pending.

Six months later, financing becomes unavailable.

The system must not rewrite the September decision as though the owner knew that would happen.

Instead:

> **Decision made September 19, 2026**
>
> Information available at time of decision:
> Preliminary valuation
> Initial financing scenarios
> Owner destination
> Current business facts
>
> Later event:
> Financing source withdrew on March 4, 2027.

That is decision history rather than hindsight.

---

# 5. What Counts as a Decision?

Not every click is a decision.

The engine should capture substantive choices.

Examples:

* Selecting a transaction scenario
* Changing a destination objective
* Setting a nonnegotiable
* Choosing whether to use an existing professional
* Selecting professional roles
* Deciding whether to pursue financing
* Choosing a seller-note strategy
* Approving a major transaction-plan change
* Changing target timing
* Choosing whether to disclose information
* Approving a major ownership-structure alternative
* Deciding to pause
* Deciding to proceed
* Choosing between materially different transaction paths

Routine interface actions should remain in Audit.

---

# 6. Decision Types

The engine should support categories.

### Destination Decision

> What outcome does the owner want?

### Strategic Decision

> Which broad path should be explored?

### Scenario Decision

> Which scenario should move forward?

### Professional-Team Decision

> Which professionals or roles should be used?

### Information Disclosure Decision

> What information should be shared?

### Financial Decision

> Which capital or proceeds approach should be pursued?

### Timing Decision

> When should a major milestone or closing be targeted?

### Transaction Decision

> Should the transaction proceed, pause, change structure, or stop?

### Exception Decision

> How should an unusual situation be handled?

### Reconsideration Decision

> Should an earlier decision be changed?

---

# 7. Decision Object

The central object is the **Decision Record**.

It contains:

* Decision ID
* Workspace
* Business
* Owner / decision-maker
* Transaction
* Decision type
* Decision statement
* Decision date
* Decision status
* Decision rationale
* Information available
* Assumptions
* Alternatives considered
* Trade-offs
* Constraints
* Nonnegotiables involved
* Professional inputs
* Research inputs
* Confidence state at time of decision
* Related documents
* Related facts
* Related scenarios
* Resulting actions
* Subsequent changes
* Review history

---

# 8. Decision Status

A decision can be:

**Draft**

**Proposed**

**Under Consideration**

**Decided**

**Implemented**

**Revisited**

**Superseded**

**Withdrawn**

**Cancelled**

The historical decision should remain intact when its status changes.

---

# 9. Decision Statement

Every substantive decision should have a concise statement.

Example:

> **Decision**
>
> Proceed with the staged employee ownership scenario as the transaction planning path.

This should be distinguishable from:

> Platform Scenario

and:

> Professional Determination.

The platform should never blur those three.

---

# 10. Owner Objective vs Decision

The engine should preserve the distinction between:

### Owner Objective

> “I want employees to eventually own the company.”

### Decision

> “I have chosen to explore staged employee ownership as the current transaction path.”

The first expresses desired outcome.

The second represents an actual choice.

That distinction becomes very useful when decisions change.

---

# 11. Professional Determination vs Owner Decision

Likewise:

### Professional Determination

> “The attorney determined that the proposed governance structure requires revision.”

### Owner Decision

> “After reviewing the attorney's determination and the alternatives, I chose to proceed with the revised structure.”

Both should be preserved.

The professional's conclusion remains attributed to the professional.

The owner's choice remains the owner's.

---

# 12. Information Available at the Time

This is arguably the most important field.

A decision record should capture the information state that was available when the decision was made.

Potential references:

* Business Reality version
* Destination version
* Scenario version
* Financial Model version
* Research version
* Evidence Ledger findings
* Professional Review versions
* Document versions
* Confidence state
* Known conflicts
* Outstanding questions

The record should preferably reference versions rather than copying entire datasets.

---

# 13. Decision Snapshot

At decision time the engine can create a logical snapshot:

> **Decision Context Snapshot**

Containing references to:

```text id="z3n8lf"
Destination v4
Business Reality v7
Scenario Set v3
Financial Model v2
Research Set v5
Professional Review v4
Confidence Status: Moderate
Known Conflicts: 2
Outstanding Questions: 3
```

This allows the platform to reconstruct the context later.

---

# 14. Why

The owner should be able to record rationale in several levels.

### Simple

> “I want more time.”

### Structured

> Reason: Preserve employee communication window.

### Detailed

> “I chose December 15 because it gives the employee group additional time to understand the proposed ownership structure, while still fitting my retirement timeline.”

The platform should not require a paragraph every time.

Progressive disclosure applies here too.

---

# 15. Alternatives Considered

A good decision record should capture meaningful alternatives.

Example:

**Chosen**

Staged employee ownership.

**Alternatives considered**

Direct employee purchase.

ESOP.

Continue operating without transition.

The platform should not force the owner to enumerate every imaginable alternative.

It should capture the alternatives that were genuinely considered.

---

# 16. Trade-Offs

Some decisions are inherently trade-offs.

The engine should support:

> **What are you accepting in exchange for what?**

Example:

| Choice                  | Benefit            | Trade-off                     |
| ----------------------- | ------------------ | ----------------------------- |
| Earlier closing         | Faster retirement  | Less employee transition time |
| Later closing           | More preparation   | Delayed liquidity             |
| Higher upfront proceeds | More liquidity now | Less future seller income     |

These are decision inputs, not platform rankings.

The engine preserves the owner's reasoning without declaring which trade-off is superior.

---

# 17. Uncertainty

The owner may not know why a choice will work.

The engine should allow:

> **Confidence in this decision**

using qualitative choices such as:

* High confidence
* Moderate confidence
* Low confidence
* Not sure

This is different from transaction-success probability.

It expresses:

> **How confident was the owner in their own decision at the time?**

---

# 18. “I’m Not Sure”

The engine should support decisions made under uncertainty.

Example:

> **Decision:** Continue exploring ESOP and direct employee purchase.

> **Confidence:** Low

> **Reason:** Need professional and financing information before narrowing the path.

This is not a failed decision.

It is a legitimate decision to **continue investigating**.

---

# 19. Nonnegotiables

The existing nonnegotiable mechanism should integrate directly.

Example:

> Nonnegotiable:
> Employee ownership must be part of the eventual outcome.

A decision record can capture:

> This decision preserves the following owner nonnegotiable.

This allows later scenario changes to identify when a decision conflicts with a previously stated requirement.

---

# 20. Decision Dependencies

A decision can depend upon another decision.

Example:

> Select financing structure

depends on:

> Select transaction structure.

The engine should preserve these relationships.

It should be possible to ask:

> **What decisions led to this decision?**

---

# 21. Decision Consequences

After a decision, the platform should track resulting actions.

Example:

> Owner chose staged employee ownership.

Consequences:

* Transaction plan version created
* Professional review requested
* New financing analysis required
* New employee communication plan created

The Decision Record does not perform those actions.

Workflow and Orchestration respond to the decision event.

---

# 22. Decision Events

A decision can generate an event:

> **DecisionRecorded**

Workflow can then respond.

For example:

```text id="k8y2dr"
DecisionRecorded
      ↓
Workflow
      ↓
Transaction Plan Update
      ↓
Professional Review Request
      ↓
Document Readiness Refresh
      ↓
Notification
```

This keeps the Decision Engine clean.

---

# 23. Decision Changes

When new information arrives, an earlier decision may need reconsideration.

The engine should never silently overwrite the original.

Instead:

> **Original Decision**
>
> September 19, 2026

↓

> **Reconsideration**
>
> March 4, 2027

↓

> **New Decision**
>
> March 12, 2027

This creates an explicit decision lineage.

---

# 24. What Changed?

Every reconsideration should identify the trigger.

Examples:

* New valuation
* Financing unavailable
* New professional determination
* New business information
* Material data correction
* Change in owner objective
* Change in employee participation
* Change in timing
* New research evidence

Example:

> **Why was this decision reconsidered?**
>
> Financing terms changed materially.

---

# 25. Decision Lineage

The system should support:

```text id="7s6gdh"
Decision D-001
   ↓
Decision D-004
   ↓
Decision D-009
```

with relationships such as:

**Reconsidered Because**

**Supersedes**

**Depends On**

**Implements**

**Reverses**

This provides a longitudinal decision history.

---

# 26. Decision vs Transaction State

Transaction state may say:

> On Hold.

Decision Record explains:

> Owner decided to pause execution until financing terms improve.

Again, both systems have distinct responsibilities.

---

# 27. Decision vs Audit

This distinction should be locked very clearly.

### Audit

> Who changed the closing date?

> Stephen changed it at 2:14 PM.

### Decision Record

> Why did Stephen change the closing date?

> To provide employees additional preparation time while preserving the retirement target.

Audit is technical/historical.

Decision Records are substantive/contextual.

---

# 28. Decision vs Communication

A conversation may contain:

> “I think we should move closing.”

That is communication.

Later:

> “I have decided to move the closing target to December 15.”

That becomes a Decision Record.

The Communication Engine can link to the resulting decision.

---

# 29. Decision vs Workflow

Workflow records:

> Task created to update transaction target date.

Decision Records record:

> Owner chose the new target date.

Workflow executes the consequences.

Decision Records preserve the reason.

---

# 30. Decision vs Scenario

Scenario Engine:

> Scenario A could achieve the stated destination under these assumptions.

Decision Engine:

> Owner chose Scenario A for further pursuit.

Those should never be confused.

---

# 31. Decision vs Professional Recommendation

The platform can preserve:

> Professional suggested Option A.

But the Decision Record should distinguish:

> Owner selected Option B.

This protects the integrity of professional roles while preserving the actual owner choice.

---

# 32. Decision Review

The owner should be able to reopen an old decision.

Example:

> **Review Decision**

The platform displays:

> Original decision
> Information known then
> Alternatives considered
> Assumptions
> Professional input
> What has changed since

Then:

> **Keep decision**

or:

> **Reconsider**

This can be valuable during long transactions.

---

# 33. Decision Checkpoints

Some decisions should intentionally be revisited at milestones.

Example:

> Six months after selecting the transaction path, review whether the destination and major assumptions remain valid.

Workflow can create the checkpoint.

Decision Engine records the outcome.

This helps prevent “set it and forget it” behavior.

---

# 34. Decision Freshness

The engine should be able to show:

> **Decision made 8 months ago**
>
> 5 major assumptions have changed.
>
> 2 professional reviews have been updated.
>
> 1 financing assumption is outdated.

That does not mean the decision is wrong.

It means:

> **The context has changed enough that reconsideration may be useful.**

The system should present this as information, not a recommendation.

---

# 35. Decision Impact Map

The platform should be able to show:

> **What depends on this decision?**

Example:

**Decision**

Use seller financing.

**Affected areas**

* Capital structure
* Cash-flow model
* Seller-note engine
* Financing workflow
* Closing requirements

This is extremely useful when the owner changes direction.

---

# 36. Decision Conflict

The engine should detect when a new decision conflicts with an earlier one.

Example:

Earlier:

> “I will not personally guarantee transaction debt.”

Later:

> Proposed financing includes a personal guarantee.

The system should surface:

> **Decision Conflict**

not silently accept the contradiction.

The owner can then:

* Keep both with an explanation
* Change the earlier decision
* Change the new decision
* Mark one obsolete
* Ask for professional review

---

# 37. Owner Explanation

Decision rationale should be editable.

But editing history matters.

The platform should distinguish:

> **Original rationale**

from:

> **Later explanation**

because people may naturally reinterpret their reasoning after the fact.

This is another reason not to simply let users rewrite history.

---

# 38. Decision Record Versioning

A decision itself should be versioned when materially amended.

Example:

**Decision v1**

> Proceed with direct employee purchase.

**Decision v2**

> Proceed with staged employee ownership after financing review.

The original remains accessible.

---

# 39. Decision Evidence

The engine should support references to:

* Documents
* Business facts
* Research findings
* Professional reviews
* Financial models
* Scenarios
* Communications
* Questions
* Requirements
* Other decisions

The record should point to those objects rather than duplicate them.

---

# 40. Decision Evidence Does Not Equal Truth

The presence of evidence in a decision record does not mean the evidence was correct.

Example:

> Owner relied on preliminary valuation.

Later:

> Valuation revised.

The historical decision still points to the preliminary valuation version.

This preserves the actual decision context.

---

# 41. Decision Context Snapshot Integrity

The snapshot should preserve the versions used at decision time.

For example:

> Business Reality v7

should continue to identify that version even after:

> Business Reality v8

exists.

This prevents historical reconstruction from accidentally substituting today's facts for yesterday's.

---

# 42. Decision Consequence Tracking

The platform should eventually show:

> **What happened after I made this decision?**

Example:

> Decision: Move closing date.

Afterward:

* Employee communication expanded.
* Attorney timeline shifted.
* Lender deadline changed.
* Two downstream tasks moved.
* No change to purchase price.

This creates useful outcome history without pretending to evaluate whether the decision was “good.”

---

# 43. Outcome Capture

The owner may later voluntarily record:

> **How did this turn out?**

Possible responses:

* As expected
* Different than expected
* Better than expected
* Worse than expected
* Still unfolding
* Not enough information

The system should treat this as retrospective information, not a model-generated judgment.

---

# 44. Lessons Learned

After major milestones, the platform can optionally ask:

> **What did we learn?**

For example:

> “We underestimated the time required for employee communication.”

This can become a structured learning record associated with the decision.

That could eventually inform future Journey and Workflow templates, subject to governance.

---

# 45. Decision Privacy

Decision records can contain highly sensitive information.

Examples:

> Owner's retirement concerns.

> Negotiation strategy.

> Internal disagreements.

> Reasons for choosing one buyer structure over another.

Therefore Decision Records require access control.

A decision should not automatically be visible to:

* Employees
* Lenders
* Marketplace providers
* All professionals
* Other owners

Visibility should be deliberate.

---

# 46. Decision Visibility Levels

Possible visibility:

### Private

Owner only.

### Advisory Team

Owner + selected professionals.

### Transaction Team

Authorized transaction participants.

### Stakeholder Summary

A deliberately limited version for broader audiences.

The full decision rationale may remain private while a participant sees:

> “Owner approved revised closing timeline.”

---

# 47. Decision Disclosure

When a decision affects another participant, the platform may create a notification or communication.

Example:

Decision Record:

> Owner changes target closing date.

Notification:

> Target closing date has changed.

Communication:

> Owner informs lender.

Transaction:

> Deadlines recalculated.

The internal rationale does not automatically travel with the operational update.

This is another important local-first and privacy principle.

---

# 48. Decision Approvals

Some decisions may require formal acknowledgment or approval by other parties.

The Decision Engine can record:

> Owner decision recorded.

Then Workflow can create:

> Professional approval task.

But the Decision Engine does not itself determine whether another party's approval is required.

Policy / Compliance or the relevant professional/process engine establishes that requirement.

---

# 49. Decision Journal

The owner should have a human-friendly view:

> **My Decision Journey**

Example:

**March 2026**
Defined employee ownership as desired long-term outcome.

**June 2026**
Selected ESOP and direct employee purchase as scenarios to research.

**September 2026**
Selected staged employee ownership for professional review.

**October 2026**
Changed target closing date.

**November 2026**
Paused execution pending financing review.

This could become one of the most valuable long-term views in the application.

---

# 50. “Why Did We Choose This?”

At any point, the owner should be able to ask:

> **Why are we pursuing this transaction structure?**

The platform can construct an answer from Decision Records:

> You selected this path on September 19.
>
> Your stated priorities were employee ownership, preserving the business, and retiring within your target timeframe.
>
> You considered direct employee purchase and ESOP.
>
> At that time, financing for the selected scenario remained uncertain.
>
> Your valuation was preliminary.
>
> Two professional reviews were still pending.

That is vastly more useful than a simple database field saying:

> `selected_scenario = scenario_3`.

---

# 51. “What Has Changed Since Then?”

Another powerful query:

> **What changed since I made this decision?**

The platform can compare:

* Destination
* Business Reality
* Research
* Professional reviews
* Financing
* Valuation
* Employee information
* Transaction timing
* Nonnegotiables

and present the material changes.

This does not make the decision for the owner.

It restores context.

---

# 52. Decision Integrity

The engine should preserve:

**Original decision**

**Original context**

**Original alternatives**

**Original rationale**

**Later information**

**Reconsideration**

**New decision**

This is essentially a time-aware decision model.

---

# 53. Core Data Objects

## DecisionRecord

The primary substantive decision.

## DecisionContext

Versioned snapshot of relevant information available at the time.

## Alternative

A meaningful option considered.

## Tradeoff

A recognized exchange or consequence.

## DecisionConstraint

A requirement affecting the decision.

## DecisionRationale

Owner's stated reasoning.

## DecisionDependency

A relationship to another decision.

## DecisionConsequence

An identified downstream effect.

## DecisionReconsideration

A later review of an earlier decision.

## DecisionLineage

Relationship among original, modified, reversed, and superseding decisions.

## DecisionOutcome

Retrospective result or owner observation.

---

# 54. Decision Contract

Core capabilities:

```text id="d4c7qe"
createDecision()
recordDecision()
addAlternative()
addTradeoff()
captureDecisionContext()
linkEvidence()
linkProfessionalInput()
linkScenario()
linkConstraint()
recordConsequence()
reconsiderDecision()
supersedeDecision()
reverseDecision()
recordOutcome()
getDecisionHistory()
getDecisionContext()
getDecisionLineage()
getAffectedObjects()
compareDecisionContext()
```

Useful explanatory operations:

```text id="r5h2vs"
whyWasThisDecided()
whatAlternativesWereConsidered()
whatWasKnownThen()
whatChangedSince()
whatDecisionsLedHere()
whatDoesThisDecisionAffect()
```

---

# 55. Events

The engine should emit events such as:

```text id="u8n4bz"
DecisionProposed
DecisionRecorded
DecisionImplemented
DecisionReconsiderationStarted
DecisionChanged
DecisionSuperseded
DecisionReversed
DecisionOutcomeRecorded
DecisionConflictDetected
```

Workflow can respond to these events.

---

# 56. Decision State Machine

A typical lifecycle:

```text id="m7q2pk"
Idea
  ↓
Under Consideration
  ↓
Decision Made
  ↓
Implemented
  ↓
Observed
  ↓
Reconsidered
  ↓
New Decision
```

Some decisions will never reach implementation.

Some will be superseded before implementation.

The system should support both.

---

# 57. Automatic vs Human Decisions

The engine should clearly distinguish:

### Owner Decision

Human choice.

### Professional Determination

Qualified professional conclusion.

### System Action

Automated platform behavior.

Example:

> Owner decides to change target date.

↓

Workflow automatically recalculates dependent deadlines.

The deadline changes are system actions, not owner decisions.

This distinction should be explicit.

---

# 58. AI-Assisted Decision Capture

AI can help the owner capture a decision from natural conversation.

For example, after a discussion:

> “It sounds like you've decided to pursue the staged ownership scenario because preserving employee participation is more important to you than maximizing immediate liquidity.”

The platform could ask:

> **Record this as a decision?**

The owner must confirm.

AI should **assist with capture**, not declare that a decision was made.

---

# 59. AI Decision Summaries

AI can summarize:

* Alternatives
* Trade-offs
* Rationale
* Context
* Changes since decision

But the summary must remain distinguishable from the original decision record.

Example:

> **AI-generated summary**

versus:

> **Owner's original rationale**

The original remains authoritative as the historical record.

---

# 60. Privacy and Local-First

Decision records may themselves be sensitive.

Therefore the owner should be able to maintain:

> **Private Local Decision Journal**

without automatically synchronizing every rationale to the cloud.

The same Local Vault / Consent & Access principles apply.

Structured metadata may synchronize as needed for workflow, while sensitive rationale can remain local unless explicitly shared.

---

# 61. Decision Record and Professional Packages

Professional Review Packages may include relevant owner decisions.

But the Package Engine should determine:

> Which decisions are relevant to this professional?

Consent & Access determines:

> Whether the decision may be disclosed.

Decision Record remains the source.

Example:

Attorney may need:

> Owner decided employee ownership is a nonnegotiable objective.

They may not need:

> Owner's private concerns about personal financial circumstances.

---

# 62. Decision Record and Communication

A decision can be referenced in communication:

> “Per the owner's decision recorded on September 19, the transaction will proceed under the staged ownership structure.”

The message points to the decision record.

It does not replace it.

---

# 63. Decision Record and Transaction Orchestration

When a decision affects execution:

```text id="h8n5cq"
Owner Decision
      ↓
DecisionRecorded
      ↓
Workflow
      ↓
Transaction Plan Update
      ↓
New Tasks / Dependencies
```

Transaction Orchestration owns the changed execution state.

Decision Record owns the choice that caused it.

---

# 64. Decision Record and Audit

The two engines should work together:

### Decision Record

> Owner chose December 15 because employee communication required additional time.

### Audit

> Target closing date changed from November 30 to December 15 by user ID X at timestamp Y.

### Workflow

> 8 dependent deadlines recalculated.

### Notification

> Relevant participants notified.

Four different truths about the same event.

That is exactly what we want.

---

# 65. Important Integrity Rule

The platform should never use today's information to rewrite yesterday's decision context.

This should be a hard architectural constraint.

Historical decision reconstruction must use:

> **Information versions that existed at the time.**

Not:

> Current values.

---

# 66. Another Important Integrity Rule

The platform should not automatically infer rationale.

For example:

> Owner changed closing date.

The system should not write:

> “Owner changed closing date to reduce employee stress.”

unless the owner actually said that or authorized that interpretation.

AI may suggest:

> “Possible reason: employee communication timing.”

but it must remain:

> **Unconfirmed**

until the owner records it.

---

# 67. Decision Conflicts With Current Reality

The engine should be able to surface:

> **Your previous decision may no longer fit the current context.**

But it should not say:

> “Your decision was wrong.”

Example:

> Original decision assumed financing availability of $5M.

Current Capital Engine:

> Available financing appears lower.

The platform presents the changed fact and links back to the original decision.

The owner decides what to do.

---

# 68. Retention

Decision records should generally have a long retention horizon because they explain transaction history.

But retention and deletion rules remain subject to the platform's Policy / Compliance Engine.

Historical integrity should be preserved even when operational objects are archived.

---

# 69. What This Engine Should Never Do

It should never:

* Judge whether a decision was good or bad
* Rank the owner's decisions
* Replace professional determinations
* Replace owner judgment
* Rewrite history using current information
* Invent rationale
* Treat AI-generated interpretation as owner intent
* Automatically infer consent from a decision
* Automatically disclose private rationale
* Become technical audit logging
* Become transaction orchestration
* Become a permanent archive of every interface click

---

# 70. Architectural Lock

These should now be treated as requirements:

**1. Decision Records are a standalone engine.**

**2. The engine preserves substantive owner decisions, not technical activity logs.**

**3. Every substantive decision records what was decided, when, and by whom.**

**4. Decision context is versioned.**

**5. The system preserves what information existed at the time of decision.**

**6. Alternatives considered can be recorded.**

**7. Trade-offs can be recorded.**

**8. Uncertainty can be recorded.**

**9. Owner rationale is distinguishable from platform-generated summaries.**

**10. Professional determinations remain separate from owner decisions.**

**11. Owner objectives remain separate from decisions.**

**12. Platform scenarios remain separate from decisions.**

**13. Decisions can have dependencies and consequences.**

**14. Material decision changes create new decision records or explicit reconsideration lineage rather than silently overwriting history.**

**15. The engine records what changed and why a decision was revisited.**

**16. Historical decisions reference historical versions of relevant information.**

**17. The system never uses today's information to reconstruct yesterday's decision context.**

**18. The system never invents or assumes owner rationale.**

**19. Decision records can be private, restricted, or shared according to Consent & Access.**

**20. Workflow responds to decisions; it does not own them.**

**21. Transaction Orchestration responds to decisions; it does not own their rationale.**

**22. Communication may reference decisions but does not replace them.**

**23. Audit records technical changes but does not replace decision records.**

**24. Decision outcomes can be captured retrospectively without rewriting the original decision.**

**25. Decision lineage is preserved from original choice through reconsideration and supersession.**

**26. AI may assist with decision capture and summarization but may not declare or rewrite owner intent without confirmation.**

**27. Decision records are independently versioned, tested, permissioned, and replaceable.**

---

# 71. The Four Histories

We now have a very useful distinction across the platform:

```text id="k4v1ns"
                WHAT THE OWNER WANTS
                       │
                 Destination
                       │
                       ▼
                WHAT THE OWNER
                   DECIDES
                       │
                Decision Record
                       │
                       ▼
               WHAT THE SYSTEM DOES
                       │
                  Workflow
                       │
                       ▼
             WHAT ACTUALLY HAPPENED
                       │
                    Audit
```

And there is a fifth layer:

```text id="y8q3ps"
             WHAT PROFESSIONALS
                 DETERMINED
                       │
            Professional Review
```

Those records should **never collapse into one another**.

---

# 72. The Most Valuable User Experience

Eventually the owner should be able to open:

## **My Decision Journey**

and see something like:

> **March 2026**
> Desired employee ownership established as a core objective.
>
> **June 2026**
> Direct employee purchase and ESOP explored.
>
> **September 19, 2026**
> Staged employee ownership selected for professional review.
>
> **Information available then:** preliminary valuation, current financial data, initial financing scenarios.
>
> **Alternatives considered:** direct employee purchase, ESOP.
>
> **Primary reasons recorded:** employee participation and business continuity.
>
> **October 2026**
> Attorney requested governance changes.
>
> **November 2026**
> Owner reconsidered timing after financing assumptions changed.
>
> **December 2026**
> Closing target moved.

That tells the story of the transaction in a way a raw audit log never could.

And there is a subtle but important benefit: **it protects the owner from hindsight bias.** A decision can be perfectly reasonable based on what was known at the time and still require changing later. The system should preserve both facts without turning either into a verdict. 🧭

This gives us another strong architectural layer:

**Decision Records preserve human agency across time.**
