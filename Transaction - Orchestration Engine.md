This is the engine that makes the platform move from **“we know what we intend to do”** to **“we are systematically doing it.”**

Its defining discipline should be:

> **The Transaction / Orchestration Engine coordinates work. It does not perform the specialist work.**

That distinction matters enormously. A transaction may involve valuation, tax, legal, financing, employee communication, trustee review, diligence, document preparation, and closing. The Orchestration Engine needs to know **when each thing must happen, who owns it, what it depends on, and whether it is blocking progress**, without pretending to be the engine that actually performs it.

# Transaction / Orchestration Engine

## 1. Purpose

The Transaction / Orchestration Engine coordinates the execution of an owner's professionally reviewed transaction plan.

It converts:

> **Destination + Business Reality + Research + Scenario + Professional Determinations**

into:

> **Stages + Milestones + Tasks + Dependencies + Participants + Deadlines + Closing Requirements + Transaction State**

Its purpose is to make the transaction understandable, sequenced, trackable, and executable.

It answers:

> **“Where are we in the transaction, what must happen next, who is responsible, what is blocking us, and what must be true before we can proceed?”**

---

# 2. Core Architectural Principle

## The Orchestration Engine coordinates. It does not absorb.

The engine must never become a giant transaction-management monolith that starts doing everyone else's jobs.

For example:

It does not perform valuation.

It does not determine tax treatment.

It does not give legal advice.

It does not approve financing.

It does not perform professional review.

It does not manage document permissions.

It does not become the document repository.

It does not create legal agreements.

It does not decide which transaction structure the owner should choose.

It does not replace professionals.

Instead, it maintains the **execution graph** connecting those activities.

---

# 3. The Transaction as an Execution Graph

A transaction should be represented as a network of:

**Stages**

→ **Milestones**

→ **Tasks**

→ **Dependencies**

→ **Requirements**

→ **Participants**

→ **Decisions**

→ **State changes**

A simple transaction might look like:

```text
Professionally Reviewed Structure
              ↓
        Transaction Setup
              ↓
        Diligence Ready
              ↓
      Valuation Complete
              ↓
      Financing Complete
              ↓
       Definitive Docs
              ↓
       Closing Ready
              ↓
           Closing
              ↓
      Post-Close Transition
```

But internally this is not just a linear checklist.

It is a dependency graph.

For example:

```text
Business Information Verified
          │
          ├──────────────► Valuation
          │                    │
          │                    ▼
          │              Purchase Price
          │                    │
          ├──────────────► Financing
          │                    │
          │                    ▼
          │              Capital Confirmed
          │                    │
          └──────────────► Definitive Documents
                               │
                               ▼
                        Closing Readiness
```

That is where the engine becomes genuinely useful.

---

# 4. When the Engine Begins

The engine should become primary after a viable path has been professionally reviewed and the owner has decided to proceed.

However, the system may create a **Pre-Transaction Plan** earlier.

This allows the platform to say:

> “Here is what execution would involve.”

without implying:

> “This is the structure you should use.”

Once the owner approves a scenario and the applicable professionals have completed their required determinations, the system can promote it into an active transaction.

---

# 5. Transaction Object

The central object is the **Transaction**.

It contains:

* Transaction ID
* Business
* Owner
* Selected scenario
* Transaction type
* Current stage
* Current state
* Target closing date
* Actual closing date
* Participants
* Milestones
* Tasks
* Dependencies
* Requirements
* Risks/blockers
* Decision records
* Professional review references
* Financing references
* Document references
* Version
* Created date
* Updated date

The Transaction object should reference information owned by other engines rather than copying all of it.

---

# 6. Transaction Type

The engine must not be built around ESOPs alone.

Examples:

* ESOP transition
* Direct employee purchase
* Employee-owned acquisition entity
* Management buyout
* Employee/management combination
* Staged employee ownership
* Seller-financed employee acquisition
* Hybrid transaction
* Other owner-approved structure

The transaction type influences the execution plan but does not determine it by itself.

---

# 7. Template vs Actual Transaction

This distinction is important.

The platform can have:

> **Transaction Template**

which defines the common stages and tasks for a type of transaction.

But an actual transaction becomes:

> **Transaction Plan**

which is customized for the specific business, scenario, professionals, financing, timing, and requirements.

For example:

**ESOP Template**

might contain:

* Initial structure review
* Valuation
* Trustee involvement
* Financing
* Documentation
* Regulatory/professional requirements
* Closing
* Transition

An actual transaction may add or remove many of those items.

Therefore:

> **Templates generate plans. They do not dictate reality.**

---

# 8. Stage Model

Stages provide the high-level map.

A generic transaction could use:

### 1. Transaction Setup

Confirm the selected path, participants, objectives, and execution framework.

### 2. Readiness

Ensure required facts, documents, professionals, and decisions are available.

### 3. Diligence

Validate the information necessary for the transaction.

### 4. Structure & Valuation

Complete the relevant professional work around value and transaction structure.

### 5. Financing / Capitalization

Coordinate financing and capital requirements where applicable.

### 6. Documentation

Coordinate preparation, review, revision, and approval of transaction documents.

### 7. Closing Preparation

Confirm that closing conditions and required deliverables are satisfied.

### 8. Closing

Coordinate the actual closing sequence.

### 9. Transition

Track immediate post-closing actions.

### 10. Post-Close Handoff

Transition responsibility to the Ownership Lifecycle Engine and other appropriate systems.

These are defaults, not universal requirements.

A transaction may skip stages.

A stage may also repeat.

---

# 9. Stage State

Each stage can have:

* Not Started
* Preparing
* Active
* Waiting
* Blocked
* Complete
* Skipped
* Reopened
* Cancelled

A stage should never be marked complete merely because its tasks look finished if a required external condition remains unresolved.

---

# 10. Milestones

Milestones are significant transaction events.

Examples:

* Scenario approved
* Professional team appointed
* Diligence complete
* Valuation complete
* Financing approved
* Definitive documents approved
* Closing conditions satisfied
* Closing scheduled
* Closing completed
* Ownership transferred
* Post-close handoff completed

Milestones should be meaningful enough that the owner can understand the transaction without reading every task.

---

# 11. Tasks

Tasks are executable units of work.

Every task should have:

* Task ID
* Title
* Description
* Owner
* Participant
* Related stage
* Related milestone
* Status
* Priority
* Deadline
* Dependencies
* Blocking status
* Source/trigger
* Required evidence
* Completion criteria
* Created date
* Completed date
* Notes
* Related documents
* Related professional assignment

Example:

> **Provide updated customer concentration analysis to valuation professional**

Owner:

> Business owner

Due:

> October 12

Dependency:

> 2025 customer revenue data verified

Completion requirement:

> File uploaded or locally available and marked ready for professional review.

---

# 12. Task Ownership

Every meaningful task should have a clear responsible party.

Potential task owners:

* Owner
* Co-owner
* Management
* Attorney
* CPA
* Valuation professional
* Lender
* Trustee
* Financial advisor
* Employee representative
* Buyer
* Seller-note purchaser
* Platform coordinator
* External service provider

The platform should distinguish:

### Responsible

Who is expected to do the work.

### Accountable

Who owns the outcome.

### Participant

Who is involved.

### Reviewer

Who must review.

### Approver

Who must approve.

This prevents the classic transaction problem where ten people are involved and nobody actually owns the next step.

---

# 13. Dependencies

Dependencies are fundamental.

Examples:

> Financing cannot begin until the financing request is complete.

> Final purchase price cannot be finalized until required valuation work is completed.

> Closing cannot occur until required documents are executed.

> Professional review cannot be completed until requested information is available.

Dependencies should be explicit rather than buried in task descriptions.

---

# 14. Dependency Types

The engine should support several kinds.

### Finish-to-Start

Task B cannot start until Task A finishes.

### Finish-to-Finish

Task B cannot finish until Task A finishes.

### Start-to-Start

Task B begins after Task A begins.

### Conditional

Task B is required only if a condition is true.

Example:

> Seller financing required = Yes.

### Approval Dependency

Task requires an explicit approval.

### Document Dependency

Task requires a specific document/version.

### Professional Determination Dependency

Task requires a professional determination from another engine.

### External Dependency

Task depends on a third party.

---

# 15. Conditional Branching

The orchestration plan must change when transaction facts change.

Example:

> Seller note included?

**No**

→ Seller-note servicing tasks are unnecessary.

**Yes**

→ Create note documentation, servicing, payment, and potentially liquidity-related tasks.

Another:

> External financing required?

**No**

→ Financing stage may be skipped.

**Yes**

→ Activate capital request, lender diligence, financing approval, documentation, and funding tasks.

This should use the existing Scenario and Journey logic rather than creating a second decision system inside Orchestration.

---

# 16. Deadlines

The engine should support:

* Fixed dates
* Relative deadlines
* Milestone-based deadlines
* Professional target dates
* Owner target dates
* Closing date offsets

Example:

> Valuation package due 45 days before target closing.

If the target closing date changes, dependent deadlines should recalculate where appropriate.

The system should distinguish:

**Calculated date**

from:

**Committed date**

and:

**Actual date**

---

# 17. Deadline Confidence

The engine should not pretend every date is certain.

A date might be:

> Planned

> Target

> Requested

> Committed

> Confirmed

> Actual

Example:

> Closing date: November 30
> Status: Target

Later:

> Closing date: November 30
> Status: Confirmed

This avoids false precision.

---

# 18. Critical Path

The engine should dynamically calculate the transaction's critical path.

The critical path is:

> **The chain of dependencies currently determining the earliest feasible completion of the transaction.**

Example:

```text
Verified Financials
      ↓
Valuation
      ↓
Purchase Price
      ↓
Financing
      ↓
Definitive Documents
      ↓
Closing Conditions
      ↓
Closing
```

If valuation is delayed by two weeks and it sits on the critical path, the system should tell the owner:

> **Potential closing impact: 2 weeks**

without presenting this as a certainty if downstream dates remain flexible.

---

# 19. Critical Path UI

The owner should not have to understand project-management theory.

Instead:

> **What could delay my closing?**

The system can show:

### Current Critical Path

Valuation → Financing → Definitive Documents → Closing

### Current Blocker

Valuation information package incomplete.

### Next Action

Owner must provide updated customer concentration report.

That is much more useful than a giant Gantt chart.

---

# 20. Blockers

A blocker is an unresolved condition preventing progress.

Examples:

* Missing document
* Conflicting financial fact
* Professional review outstanding
* Financing decision outstanding
* Approval outstanding
* Required participant unavailable
* Document revision required
* Nonnegotiable conflict
* External dependency overdue

Each blocker should have:

* Source
* Description
* Owner
* Severity
* Created date
* Related task
* Related milestone
* Resolution action
* Status

---

# 21. Blocker Hierarchy

A simple classification:

### Informational

Worth knowing but not delaying execution.

### Attention Required

Requires action but is not currently blocking.

### Blocking

Prevents one or more downstream actions.

### Critical

Threatens a key milestone or closing condition.

This gives the user a clean transaction health picture without inventing a numerical “probability of closing.”

---

# 22. Participant Status

The engine should track each participant's state.

Example:

**Attorney**

Assigned
Active
Waiting
Reviewing
Complete
Withdrawn

**Lender**

Invited
Requesting information
Under review
Indicative terms
Approved
Declined
Committed
Funded

**Valuation Professional**

Assigned
Information requested
Analyzing
Draft complete
Final complete

Participant status should be coordination data.

It should not reinterpret the professional's substantive conclusions.

---

# 23. Participant Responsibility Map

The engine should provide:

> **Who is doing what?**

Example:

| Participant            | Role      | Current responsibility    | Status                       |
| ---------------------- | --------- | ------------------------- | ---------------------------- |
| Owner                  | Seller    | Provide financial records | Waiting on owner             |
| CPA                    | Tax       | Tax review                | In progress                  |
| Attorney               | Legal     | Definitive documents      | Waiting on valuation         |
| Valuation professional | Valuation | Final valuation           | In progress                  |
| Lender                 | Financing | Credit review             | Waiting on financial package |

This creates a transaction-wide operating picture.

---

# 24. Requirements

A **Requirement** is something that must be true before a particular milestone or stage can complete.

Examples:

> Required valuation completed.

> Required documents executed.

> Financing committed.

> Required professional approval obtained.

> Required disclosure made.

> Owner approval recorded.

> Required funds available.

Requirements differ from tasks.

A task is:

> “Prepare financing package.”

A requirement is:

> “Financing package accepted by lender.”

The requirement can be satisfied by events from another engine.

---

# 25. Closing Requirements

Closing should have a dedicated **Closing Readiness** object.

It aggregates required conditions from the relevant systems.

Example:

### Closing Readiness

**Professional review**
Complete

**Valuation**
Complete

**Financing**
Committed

**Required documents**
Complete

**Owner approvals**
Complete

**Required funds**
Confirmed

**Closing date**
Confirmed

**Outstanding blockers**
None

The Orchestration Engine does not invent those requirements.

It aggregates them from the relevant engines and monitors their state.

---

# 26. Closing Checklist

A closing checklist should be generated dynamically.

Different transaction structures produce different requirements.

For example:

An ESOP transaction may generate one set.

A direct employee purchase may generate another.

A seller-financed transaction may generate another.

The same engine can coordinate all of them because the checklist is generated from:

**Transaction Type + Scenario + Professional Determinations + Policies + Requirements**

---

# 27. Transaction State

The transaction itself needs a clear state.

Suggested states:

**Draft**

**Planning**

**Professionally Reviewed**

**Approved to Proceed**

**Active**

**On Hold**

**Closing Preparation**

**Ready to Close**

**Closing**

**Closed**

**Transitioning**

**Complete**

**Cancelled**

**Terminated**

**Superseded**

These states should have explicit transition rules.

---

# 28. State Transitions

A transaction should not jump arbitrarily between states.

For example:

```text
Draft
  ↓
Planning
  ↓
Professionally Reviewed
  ↓
Approved to Proceed
  ↓
Active
  ↓
Closing Preparation
  ↓
Ready to Close
  ↓
Closing
  ↓
Closed
  ↓
Transitioning
  ↓
Complete
```

A transaction can be:

> **On Hold**

from several stages.

A transaction can also be:

> **Reopened**

when a material issue requires additional work.

---

# 29. Reopening a Transaction

A closed or near-closed process may need to reopen a task or stage.

Example:

> Lender requests corrected financial information.

The engine should create:

> **Reopened requirement**

rather than pretending the earlier state never existed.

Previous state remains part of transaction history.

---

# 30. Scenario Changes

The owner may change the selected scenario.

This is important.

Suppose:

> Scenario A: Direct employee purchase

becomes:

> Scenario B: Staged employee ownership

The Transaction Engine should not simply overwrite the current plan.

Instead:

1. Existing plan is versioned.
2. Dependencies are recalculated.
3. New scenario is referenced.
4. Obsolete tasks are identified.
5. New tasks are generated.
6. Participants are reevaluated.
7. Professional review requirements are checked.
8. Owner sees what changed.

Example:

> **Transaction plan changed**
>
> 11 tasks remain relevant.
> 7 tasks are no longer applicable.
> 14 new tasks were added.
> Financing assumptions changed.
> Additional professional review is required.

That is much safer than silently mutating the transaction.

---

# 31. Professional Determination Boundary

A professional can say:

> “This structure should be changed.”

The Professional Review Engine records that determination.

The Transaction Engine may then receive:

> Required change before execution.

It should create:

> **Task: Obtain revised structure**

or:

> **Requirement: Professional determination updated**

But it must never independently say:

> “The professional is correct.”

The transaction state reflects the professional determination and owner's decision, not the platform's judgment.

---

# 32. Owner Decision Boundary

The owner remains the decision-maker.

For significant decision points, the system should provide:

> **Decision Required**

Example:

> Attorney has proposed a revised transaction structure.

The platform shows:

* What changed
* Why the professional requested it
* Supporting information
* Relevant alternatives
* Impact on current plan
* Outstanding questions

Then the owner records a decision.

The Decision Record Engine owns the decision record.

Transaction Orchestration records the resulting workflow effect.

---

# 33. Task Generation

Tasks may originate from:

* Scenario
* Professional Review
* Professional Review Package
* Document Readiness
* Capital / Financing
* Seller-Note Liquidity
* Policy / Compliance
* Closing
* Owner decision
* External participant
* Transaction template

The Orchestration Engine receives these events and incorporates them into the execution graph.

---

# 34. Event-Driven Architecture

This engine should be highly event-driven.

Examples:

### Professional Review completes

→ Create next-stage tasks.

### Lender requests a document

→ Document Readiness creates requirement.

→ Orchestration creates owner action.

### Document becomes available

→ Dependency may automatically resolve.

### Financing is committed

→ Financing dependency resolves.

### Closing date changes

→ Recalculate affected deadlines.

### Owner changes scenario

→ Rebuild affected execution graph.

The engine coordinates these events without owning the underlying facts.

---

# 35. Generic Workflow Engine Relationship

We previously identified a separate **Workflow Engine**.

That engine should provide generic infrastructure:

* task triggers
* recurring tasks
* dependencies
* reminders
* escalations
* completion events
* scheduling

The Transaction / Orchestration Engine owns the **transaction-specific interpretation** of those capabilities.

So:

**Workflow Engine**

> “A task can depend on another task.”

**Transaction Engine**

> “Closing document execution depends on definitive document approval.”

This distinction keeps the generic workflow infrastructure reusable outside transactions.

---

# 36. Notifications

The Notification Engine should generate the actual alerts.

Transaction Orchestration supplies events such as:

> Task due in 3 days.

> Critical dependency blocked.

> Closing date changed.

> Professional response received.

> Requirement satisfied.

The Notification Engine decides how and when to alert the user.

Again, separation of responsibilities.

---

# 37. Document Readiness Integration

A common workflow:

> Lender requests updated debt schedule.

Document Readiness:

> Debt schedule requested.

Local Vault:

> Debt schedule available locally.

Consent & Access:

> Owner authorizes lender access.

Disclosure:

> Debt schedule shared.

Transaction Orchestration:

> Financing information requirement satisfied.

The orchestration engine does not duplicate the document workflow.

It watches the resulting state.

---

# 38. Capital / Financing Integration

Capital Engine owns:

* Capital sources
* Financing requests
* Financing requirements
* Indicative terms
* Capital gaps

Transaction Engine owns:

* When financing work must happen
* What milestone depends on it
* Who is responsible
* Whether financing is currently blocking closing

Example:

> Capital Engine:

**Financing status: committed**

Transaction Engine:

**Financing milestone: complete**

---

# 39. Seller-Note Integration

Seller-Note Liquidity Engine owns the note economics and marketplace process.

Transaction Engine may own:

> Seller note documentation completed.

> Seller note included in closing package.

> Note funding confirmed.

> Servicing handoff required.

The financial details stay with the Seller-Note engine.

---

# 40. Professional Review Integration

Professional Review Engine owns:

* Review assignments
* Requests
* Professional feedback
* Professional determinations

Transaction Engine tracks:

> Legal review outstanding.

> Valuation determination complete.

> Tax review required before proceeding.

This allows the transaction to know its readiness without becoming the repository for professional opinions.

---

# 41. Consent Integration

Before a task can complete:

> “Share employee ownership information with trustee.”

the Orchestration Engine should check that required authorization exists.

It does not grant the permission.

Consent & Access does.

---

# 42. Audit Integration

Every important orchestration state change should emit an event to Audit.

Examples:

> Stage started.

> Task assigned.

> Deadline changed.

> Dependency added.

> Milestone completed.

> Transaction placed on hold.

> Closing readiness achieved.

The Audit Engine keeps historical truth.

Transaction Engine keeps current operational state.

---

# 43. Transaction Timeline

The user should have a human-readable timeline.

Example:

> **September 19**
> Transaction approved to proceed.
>
> **September 21**
> Valuation package submitted.
>
> **September 25**
> CPA requested updated debt schedule.
>
> **September 26**
> Owner provided debt schedule.
>
> **September 30**
> Valuation completed.
>
> **October 2**
> Financing request submitted.
>
> **October 12**
> Financing commitment received.

This becomes the transaction's operational narrative.

---

# 44. Transaction Dashboard

The primary transaction screen should answer five questions immediately:

### Where are we?

**Financing & Documentation**

### What's next?

**Finalize financing documents**

### What's blocking us?

**One outstanding lender request**

### Who owns it?

**Owner**

### Are we on track?

**Current target: November 30**

The platform can additionally show:

> Critical path active

without creating a misleading transaction-success probability.

---

# 45. Progress Indicator

The system can show stage progress.

For example:

**Transaction Progress**

`████████░░`

but progress should be based on meaningful milestones rather than simply counting tasks.

Completing 20 administrative tasks should not make a transaction appear 80% complete if the valuation and financing remain unresolved.

The system should therefore have:

### Task progress

and:

### Milestone readiness

as separate concepts.

---

# 46. Readiness vs Progress

This distinction is important.

Example:

> 85% of tasks completed.

but:

> Closing readiness: Not Ready.

That is perfectly possible.

The UI should make this obvious.

The transaction might say:

> **Operational progress: High**
> **Closing readiness: Pending**

rather than giving one misleading percentage.

---

# 47. Risk Signals

The engine can surface factual execution risks such as:

* Critical task overdue
* Dependency unresolved
* Participant unresponsive
* Closing date approaching
* Required document missing
* Professional determination outstanding
* Financing not yet committed

It should not convert these into:

> “You probably won't close.”

That crosses into prediction.

Instead:

> **Closing readiness is currently blocked by financing commitment.**

That's actionable and factual.

---

# 48. Owner Experience

The owner should not see a project-management cockpit first.

The default experience should be:

> **Your Transition**

**Current stage:** Financing

**Next:** Respond to lender request

**Blocking issue:** Updated debt schedule

**Upcoming:** Financing decision

**Target closing:** November 30

Then the user can expand into detailed project management.

The complexity is there, but it arrives progressively.

---

# 49. Professional Experience

Professionals should get role-specific views.

An attorney should see:

> Legal tasks

> Legal deadlines

> Legal dependencies

> Documents requiring review

> Questions awaiting response

A lender sees:

> Financing tasks

> Requested documents

> Financing milestones

> Outstanding conditions

Nobody needs to see the entire transaction unless authorized.

Consent & Access controls the visibility.

---

# 50. Participant Portal

The platform should eventually support a simplified participant workspace.

A participant sees:

> **Your Current Responsibilities**

1. Review ownership structure.
2. Return comments.
3. Upload requested document.
4. Confirm milestone.

This prevents external participants from needing to understand the entire application.

---

# 51. Escalation

Tasks and dependencies can escalate.

Example:

> Task overdue 2 days.

→ Reminder.

> Task overdue 5 days.

→ Participant escalation.

> Task overdue 10 days and on critical path.

→ Owner alert + professional/project alert according to policy.

Escalation rules belong primarily to Workflow/Notification infrastructure, while Transaction determines whether the item is material to the transaction.

---

# 52. Deadline Changes

Changing a major deadline must show downstream effects.

Example:

> **Target closing moved from November 30 to December 15.**

The platform can show:

> 8 deadlines moved automatically.
> 2 manually committed dates were not changed.
> 1 professional date may now be unnecessary.
> 1 critical-path dependency changed.

The system should never blindly shift dates that participants have explicitly committed to.

---

# 53. Cancellation

A transaction may be cancelled.

Cancellation should require:

* Reason, when appropriate
* Effective date
* Owner authorization
* Current state capture
* Participant notification as applicable
* Open-task handling
* Document preservation
* Professional assignment status update

Cancellation does not delete history.

---

# 54. Superseded Transactions

A transaction can become superseded.

Example:

> Direct employee acquisition plan

becomes:

> ESOP transaction plan.

The old transaction is preserved.

The new transaction references it:

> **Supersedes Transaction TX-001**

This gives the owner a historical decision trail.

---

# 55. Closing Event

The closing event should be a controlled state transition.

Before entering:

> **Closing**

the engine evaluates all required closing requirements.

Once the closing event occurs:

* Actual closing date recorded
* Closing state created
* Remaining pre-close tasks resolved
* Post-close tasks generated
* Required participants notified
* Closing artifacts referenced
* Ownership Lifecycle handoff initiated

The engine does not itself create legal closing documents.

---

# 56. Post-Close Handoff

After closing:

> Transaction / Orchestration

hands ongoing responsibilities to:

**Ownership Lifecycle Engine**

which can eventually manage:

* Ownership administration
* Governance
* Annual reviews
* Seller-note servicing
* Additional ownership changes
* Succession
* Long-term transition

This prevents the transaction engine from becoming a permanent operating system for the business.

---

# 57. Core Data Objects

## Transaction

The overall execution record.

## TransactionStage

High-level phase.

## Milestone

Significant achievement or gate.

## Task

Actionable work item.

## Dependency

Relationship between tasks, milestones, requirements, or external conditions.

## Requirement

Condition that must be satisfied.

## ParticipantAssignment

Person or organization and their role in the transaction.

## Blocker

Condition preventing progress.

## Deadline

Planned, committed, or actual timing.

## CriticalPathNode

A task or milestone currently contributing to the critical path.

## ClosingReadiness

Aggregated status of closing requirements.

## TransactionState

Current lifecycle state.

## TransactionPlanVersion

Historical version of the execution plan.

---

# 58. Transaction Plan Versioning

The plan itself should be versioned.

Example:

**Plan v1**

Direct employee acquisition.

**Plan v2**

Seller financing added.

**Plan v3**

Financing structure changed.

**Plan v4**

Closing date moved.

Every material plan change should produce:

> **What changed?**

For example:

> Financing requirement added.

> 6 tasks added.

> 2 deadlines changed.

> Seller-note task removed.

> Closing target moved 15 days.

---

# 59. Decision Record Relationship

When a plan changes because the owner made a significant decision:

Decision Record Engine stores:

> What the owner chose
> Why
> Information available
> Alternatives considered

Transaction Engine stores:

> What operational changes resulted from that decision.

This keeps decision-making separate from execution.

---

# 60. Transaction Integrity Rules

The engine should enforce important structural rules.

### A transaction cannot be marked complete while required closing requirements remain unresolved.

### A milestone cannot be completed while a required dependency is unresolved.

### A task cannot be assigned to a participant who lacks the required relationship or authorization.

### A professional requirement cannot be considered satisfied merely because a task was checked off if the Professional Review Engine has not recorded the required determination.

### A document requirement cannot be considered fulfilled merely because a filename exists.

### A closing condition cannot be treated as satisfied based on stale information when freshness is required.

### A transaction plan change must be versioned.

### A cancelled or superseded transaction must remain historically recoverable.

---

# 61. What the Engine Should Never Do

It should never:

* Give legal advice
* Give tax advice
* Determine valuation
* Approve financing
* Choose an investment structure
* Override professional determinations
* Override owner nonnegotiables
* Grant document access
* Store the master document repository
* Become the communication system
* Become the professional marketplace
* Make a closing-success prediction
* Silently change transaction structure
* Silently discard historical plans
* Mark work complete solely because a checkbox was clicked

---

# 62. Engine Contract

Other engines communicate with Transaction Orchestration through structured events and references.

Examples:

```text id="0wj1yz"
createTransaction()
createStage()
createMilestone()
createTask()
addDependency()
addRequirement()
assignParticipant()
updateTaskStatus()
updateMilestoneStatus()
recordBlocker()
updateDeadline()
recalculateCriticalPath()
evaluateClosingReadiness()
changeTransactionState()
createPlanVersion()
```

External engines can also emit events:

```text id="3s0y52"
ProfessionalReviewCompleted
DocumentRequirementSatisfied
FinancingCommitted
ValuationCompleted
ConsentGranted
ConsentRevoked
ScenarioChanged
OwnerDecisionRecorded
ClosingRequirementAdded
ClosingRequirementSatisfied
```

The Orchestration Engine translates those events into transaction state changes.

---

# 63. The Central Question

Every transaction screen should ultimately help answer:

> **What needs to happen next?**

But the engine should also answer:

> Why does it need to happen?

> Who owns it?

> What does it depend on?

> What happens if it slips?

> Is it on the critical path?

> What requirement will it satisfy?

> What professional or owner decision created the requirement?

That makes the system explainable rather than merely administrative.

---

# 64. Architectural Lock

The following should be treated as requirements:

**1. Transaction / Orchestration is a standalone engine.**

**2. It coordinates execution without absorbing specialist responsibilities.**

**3. Transaction plans are generated from professionally reviewed scenarios and then customized.**

**4. Transaction type is extensible beyond ESOPs.**

**5. Stages, milestones, tasks, dependencies, requirements, participants, and deadlines are first-class objects.**

**6. Critical path is dynamically calculated from the dependency graph.**

**7. Blockers are explicit objects.**

**8. Closing requirements are aggregated from the relevant engines rather than invented by Orchestration.**

**9. Transaction state is explicit and versioned.**

**10. Material transaction-plan changes create a new plan version.**

**11. Scenario changes create controlled plan revisions rather than silent overwrites.**

**12. Owner decisions remain owner decisions.**

**13. Professional determinations remain professional determinations.**

**14. Document permissions remain owned by Consent & Access.**

**15. Source documents remain owned by Local Vault.**

**16. Professional work remains owned by Professional Review and the relevant professional.**

**17. Financing facts and financing processes remain owned by Capital / Financing.**

**18. Financial calculations remain owned by Financial Modeling.**

**19. Notifications and generic task infrastructure remain separate.**

**20. Audit records historical events; Orchestration maintains current operational state.**

**21. The engine never represents execution progress as a probability of transaction success.**

**22. The engine does not determine whether the owner should proceed.**

**23. A transaction cannot reach Closing or Complete while required conditions remain unresolved.**

**24. Cancellation, reopening, and superseding preserve historical state.**

**25. After closing, the transaction hands off to the Ownership Lifecycle Engine.**

**26. The engine remains independently replaceable without destabilizing the other engines.**

---

# 65. Architectural Boundary Summary

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Transaction / Orchestration** | Transaction plans, stages, milestones, tasks, dependencies, requirements, participants, deadlines, the critical path, blockers as explicit objects, and versioned transaction state | Whether the owner should proceed, or the specialist work the plan coordinates |
| **Scenario** | Potential transaction paths and their assumptions | The execution plan derived from them |
| **Destination** | The owner's desired outcome and objectives | How the transaction is executed |
| **Business Reality** | Current business facts and financial statements | Transaction stages or execution state |
| **Financial Modeling** | Calculations, cash flows, debt schedules, and projections | Financing processes or execution state |
| **Capital / Financing** | The capital plan, capital stack, and financing requests | The execution plan or transaction state |
| **Underwriting** | Lender requirement profiles and conformance to stated requirements | The transaction's execution state |
| **Professional Review** | Professional determinations and their attribution | The execution that follows them |
| **Document Readiness** | Document status, completeness, and outstanding requests | Whether a required condition is resolved |
| **Review Package** | Purpose-built packages assembled for a named recipient | The transaction plan |
| **Closing** | Closing execution and its requirements | The aggregation of conditions that permit closing |
| **Ownership Lifecycle** | Post-transaction ownership state | The execution that produced it |
| **Workflow** | Tasks, dependencies, scheduling, and execution mechanics | The transaction plan and its state |
| **Communication** | Conversations and messages | Execution progress |
| **Notification** | Delivery of attention, including follow-up alerts | The underlying work state |
| **Decision Record** | The owner's recorded reasoning and rationale | Execution of the decision |
| **Consent & Access** | Who may see which resource, for what purpose, and for how long | Transaction state |
| **Local Vault** | Private source documents, storage, encryption, and versioning | Transaction requirements |
| **Audit / Provenance** | The historical record of what happened | Current operational state |

## Hard Boundary

The Transaction / Orchestration Engine owns the execution plan and its current state: stages, milestones, tasks, dependencies, requirements, participants, deadlines, the critical path, and blockers as explicit objects. It coordinates execution without absorbing specialist responsibilities, and it does not determine whether the owner should proceed.

**The engine never represents execution progress as a probability of transaction success.** Closing requirements are aggregated from the relevant engines rather than invented here, professional determinations remain professional determinations, owner decisions remain owner decisions, and source documents remain with Local Vault. A transaction cannot reach Closing or Complete while required conditions remain unresolved, and after closing it hands off to the Ownership Lifecycle Engine.

---

# 66. The Role of This Engine in the Overall Architecture

We now have a very clean progression:

```text id="v5l4je"
OWNER'S DESTINATION
       │
       ▼
BUSINESS REALITY
       │
       ▼
RESEARCH + EVIDENCE
       │
       ▼
SCENARIOS
       │
       ▼
PROFESSIONAL REVIEW
       │
       ▼
PROFESSIONALLY REVIEWED PLAN
       │
       ▼
TRANSACTION / ORCHESTRATION
       │
       ├── Stages
       ├── Milestones
       ├── Tasks
       ├── Dependencies
       ├── Deadlines
       ├── Participants
       ├── Requirements
       ├── Critical Path
       └── Transaction State
       │
       ├────────► Professional Review
       ├────────► Document Readiness
       ├────────► Capital / Financing
       ├────────► Financial Modeling
       ├────────► Consent & Access
       ├────────► Local Vault
       ├────────► Communications
       ├────────► Notifications
       └────────► Audit
                    │
                    ▼
                  CLOSE
                    │
                    ▼
          OWNERSHIP LIFECYCLE
```

The key idea is that **Orchestration owns the relationships between the pieces, not the pieces themselves**.

That gives us the transaction equivalent of an air-traffic controller. It knows which aircraft is where, what runway is occupied, what is waiting, what is cleared, and what could cause congestion. It does not build the airplane, fly it, inspect the engine, or decide where the passengers should ultimately go. ✈️

And that is exactly the kind of separation this platform needs.

With this engine defined, the architecture now has a very strong **execution spine**: the earlier engines establish what the owner wants, what is true, what professionals determine, what can be shared, and what resources exist. **Transaction / Orchestration turns those outputs into a controlled, versioned execution plan.**
