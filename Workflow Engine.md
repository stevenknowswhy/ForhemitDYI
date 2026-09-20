This one is the **plumbing with a brain**. It should be deliberately more generic than the Transaction / Orchestration Engine so we can reuse it everywhere, not just for business sales.

The key architectural distinction is:

> **Transaction / Orchestration owns transaction meaning. Workflow owns execution mechanics.**

# Workflow Engine

## 1. Purpose

The Workflow Engine provides the reusable event, task, trigger, dependency, scheduling, escalation, and completion infrastructure used throughout the platform.

It allows one engine's meaningful change to cause controlled actions in another engine.

Example:

> **Valuation completed**
>
> → Notify lender
> → Update Document Readiness
> → Recalculate transaction dependencies
> → Refresh Goal-to-Reality Confidence

The Workflow Engine makes those transitions reliable without owning the underlying business meaning.

Its central question is:

> **“When X happens, what should happen next, under what conditions, and how do we know it actually happened?”**

---

# 2. Architectural Principle

## Workflow is infrastructure, not business logic.

The Workflow Engine should know:

* An event occurred
* A trigger matched
* A task needs to be created
* A dependency was satisfied
* A deadline was reached
* An escalation rule fired
* A recurring action became due
* A task completed
* A workflow failed and needs retry/review

It should **not** know why a valuation matters to a transaction.

That knowledge belongs to the relevant domain engine.

For example:

**Valuation Engine**

> Valuation completed.

**Workflow Engine**

> Event received.

**Transaction Engine**

> Valuation completion satisfies Milestone M-14.

**Notification Engine**

> Tell the owner.

**Capital Engine**

> Reevaluate financing inputs.

That separation is essential.

---

# 3. What This Engine Owns

The Workflow Engine owns:

* Events
* Event subscriptions
* Triggers
* Workflow definitions
* Workflow instances
* Tasks
* Task templates
* Dependencies
* Scheduling
* Recurring actions
* Escalations
* Retries
* Completion handling
* Failure handling
* Idempotency
* Execution state
* Workflow history references
* Execution telemetry

---

# 4. What It Does Not Own

It does not own:

* Transaction meaning
* Professional determinations
* Business facts
* Financial calculations
* Document contents
* Access permissions
* Notifications as a communication channel
* Marketplace matching
* Research conclusions
* Owner decisions
* Closing requirements themselves

Those remain owned by other engines.

The Workflow Engine can **invoke** them or react to their events.

It does not absorb their responsibilities.

---

# 5. Event-Driven Model

The basic architecture is:

```text id="1q0jtz"
EVENT OCCURS
      ↓
WORKFLOW ENGINE
      ↓
MATCH TRIGGERS
      ↓
CHECK CONDITIONS
      ↓
EXECUTE ACTIONS
      ↓
EMIT NEW EVENTS
      ↓
OTHER ENGINES RESPOND
```

This creates a chain rather than a hard-coded collection of direct integrations.

Example:

```text id="o7zcfa"
ValuationCompleted
      ↓
Workflow Engine
      ├──→ Notify lender
      ├──→ Update Document Readiness
      ├──→ Recalculate transaction dependencies
      └──→ Refresh Confidence
```

The Workflow Engine coordinates the sequence.

Each target engine still owns its own action.

---

# 6. Events

An **Event** means:

> Something meaningful happened.

Examples:

* DocumentImported
* DocumentVerified
* DocumentConflictDetected
* ProfessionalAssigned
* ProfessionalReviewRequested
* ProfessionalReviewCompleted
* ValuationStarted
* ValuationCompleted
* FinancingRequested
* FinancingCommitted
* ConsentGranted
* ConsentRevoked
* ScenarioChanged
* OwnerDecisionRecorded
* MilestoneCompleted
* TaskCompleted
* DeadlineApproaching
* TransactionStateChanged
* ClosingRequirementSatisfied
* ClosingCompleted

Events should describe things that **already happened**.

They should not masquerade as commands.

### Event

> ValuationCompleted

### Command

> NotifyLender

The distinction prevents the event stream from becoming a hidden task queue.

---

# 7. Event Structure

Every event should contain:

* Event ID
* Event type
* Event version
* Source engine
* Actor
* Workspace
* Related entity
* Related transaction, when applicable
* Timestamp
* Correlation ID
* Causation ID
* Payload
* Sensitivity classification
* Schema version

Example:

```text id="2k4ddp"
Event:
  type: ValuationCompleted
  version: 1
  source: ValuationEngine
  entity: VAL-0024
  transaction: TX-004
  occurred_at: 2026-10-15T14:32:00Z
  correlation_id: COR-1884
  causation_id: EVT-8841
```

---

# 8. Event Immutability

Events should be treated as immutable historical facts.

If something changes, generate another event.

For example:

> FinancingCommitted

followed later by:

> FinancingCommitmentWithdrawn

Do not modify the original event.

This creates a reliable history of what happened.

---

# 9. Event Versioning

Event schemas will evolve.

Therefore:

> `ValuationCompleted v1`

may eventually become:

> `ValuationCompleted v2`

The Workflow Engine must support versioned event contracts.

Existing workflows should specify which versions they understand.

This allows engines to evolve independently.

---

# 10. Commands vs Events

This distinction should be explicitly locked.

### Event

> Something happened.

### Command

> Please perform this action.

### Task

> A piece of work that someone or something must complete.

Example:

```text id="r8z3rm"
ValuationCompleted
      ↓
Workflow Trigger
      ↓
CreateTask:
"Submit valuation to lender"
      ↓
Task Assigned
      ↓
Task Completed
      ↓
LenderPackageUpdated
```

The event does not itself become the task.

The workflow determines whether a task should result.

---

# 11. Triggers

A trigger tells the system:

> **When should this workflow run?**

Trigger types include:

### Event trigger

> When ValuationCompleted occurs.

### Schedule trigger

> Every Monday at 9 AM.

### Deadline trigger

> Three days before closing.

### State trigger

> When TransactionState becomes Closing Preparation.

### Threshold trigger

> When a document remains incomplete for 7 days.

### Condition trigger

> When financing is committed AND valuation is complete.

### Human trigger

> When owner selects “Proceed.”

### External trigger

> When lender sends a response.

---

# 12. Conditions

A trigger may not be sufficient by itself.

The workflow may need conditions.

Example:

> When ValuationCompleted

but only if:

> Transaction is Active

and:

> Financing is still outstanding

Then:

> Notify lender.

Conditions should be explicit and inspectable.

---

# 13. Workflow Definition

A **Workflow Definition** is a reusable blueprint.

It contains:

* Workflow ID
* Name
* Version
* Trigger
* Conditions
* Actions
* Dependencies
* Error policy
* Retry policy
* Escalation policy
* Completion criteria
* Enabled/disabled state

Example:

> **Valuation Completion Workflow v2**

Trigger:

> ValuationCompleted

Actions:

1. Update transaction dependency.
2. Update Document Readiness.
3. Notify assigned lender.
4. Refresh confidence inputs.

---

# 14. Workflow Instance

A Workflow Definition is reusable.

A **Workflow Instance** is an actual execution.

Example:

> Workflow Definition:
> Valuation Completion

Instance:

> TX-004 valuation completed on October 15.

The instance records:

* Start time
* Current state
* Actions
* Results
* Failures
* Retries
* Completion time

This keeps reusable rules separate from transaction history.

---

# 15. Task Infrastructure

Tasks are reusable execution objects.

A Task contains:

* Task ID
* Workflow instance
* Task template
* Title
* Description
* Responsible actor
* Assignee
* Status
* Priority
* Deadline
* Dependencies
* Required completion event
* Retry state
* Escalation state
* Related entity
* Related transaction
* Created time
* Completed time

---

# 16. Task Types

The workflow system should support:

### Human task

> Owner uploads requested document.

### Professional task

> CPA reviews requested financial information.

### System task

> Recalculate a model.

### Integration task

> Request updated lender status.

### Approval task

> Owner approves package.

### Waiting task

> Wait for external event.

### Review task

> Human reviews system-generated result.

---

# 17. Task States

A task can move through:

**Created**

**Ready**

**Assigned**

**In Progress**

**Waiting**

**Blocked**

**Completed**

**Failed**

**Cancelled**

**Expired**

**Skipped**

**Superseded**

The state machine should be explicit.

---

# 18. Dependencies

Workflow dependencies are generic.

A task can depend upon:

* Another task
* Event
* Condition
* Date
* State
* External response
* Approval
* Resource availability

Example:

> Notify lender

depends on:

> ValuationCompleted

Another:

> Complete transaction financing stage

depends on:

> FinancingCommitted

Workflow understands the dependency mechanics.

Transaction Orchestration gives the dependency transaction-specific meaning.

---

# 19. Dependency Resolution

When an event occurs, Workflow evaluates relevant dependencies.

Example:

```text id="q77f5m"
ValuationCompleted
        ↓
Dependency V-22 satisfied
        ↓
Task "Prepare Financing Package"
becomes Ready
```

The workflow system should emit:

> DependencySatisfied

so other engines can react without directly coupling to the dependency implementation.

---

# 20. Waiting

A workflow often needs to wait.

Examples:

> Wait for professional response.

> Wait for owner approval.

> Wait for lender decision.

> Wait until October 15.

> Wait until document becomes available.

Waiting should be an explicit state, not a dormant task nobody can explain.

---

# 21. Recurring Actions

The engine supports recurring workflows.

Examples:

> Request updated financial information every quarter.

> Remind owner about expiring professional access.

> Perform monthly transaction-readiness review.

> Generate annual ownership review task.

Recurring workflows should create independent task instances.

The platform should preserve which recurrence generated each task.

---

# 22. Recurrence Types

Examples:

* Daily
* Weekly
* Monthly
* Quarterly
* Annually
* Every N days
* Specific calendar dates
* Milestone-relative
* Event-relative

Example:

> Every 90 days after closing.

This becomes particularly useful later in Ownership Lifecycle.

---

# 23. Escalations

Escalation means:

> Something expected to happen has not happened within a defined period.

Example:

```text id="9u0txk"
Task due
   ↓
No completion
   ↓
Reminder
   ↓
Escalation
   ↓
Owner / participant notification
```

Escalations can be based on:

* Time
* Critical-path position
* Task importance
* Participant role
* Number of failed attempts
* Transaction stage

---

# 24. Escalation Levels

A simple framework:

### Level 1: Reminder

> “Task due tomorrow.”

### Level 2: Attention

> “Task is overdue.”

### Level 3: Escalation

> “Task is overdue and blocking financing.”

### Level 4: Critical

> “Critical-path task threatens the confirmed closing schedule.”

Workflow determines that escalation has occurred.

Notification determines how people are alerted.

---

# 25. Escalation Does Not Mean Blame

The system should describe state, not assign motives.

Instead of:

> “The attorney failed to respond.”

Use:

> “Professional response has not been received and the task is 5 days overdue.”

This is more accurate and keeps the system neutral.

---

# 26. Retries

System and integration actions can fail.

The Workflow Engine should support:

* Retry count
* Retry delay
* Exponential backoff where appropriate
* Maximum attempts
* Failure state
* Manual retry
* Escalation

For example:

> External notification failed.

The workflow can retry automatically before escalating.

---

# 27. Idempotency

This is an architectural requirement.

The same event may occasionally be delivered more than once.

A workflow must not:

> send five duplicate notifications

because the same event appeared five times.

Each event/action should support an idempotency key.

Example:

> `TX004-ValuationCompleted-NotifyLender`

The workflow should recognize that the action has already completed.

---

# 28. Exactly-Once Illusion

The architecture should not rely on an assumption that distributed systems deliver events exactly once.

Instead, design for:

> **At-least-once delivery + idempotent handling.**

This gives us a much more realistic foundation.

---

# 29. Correlation

Every related chain of actions should have a correlation ID.

Example:

```text id="3qj9pc"
Owner decision
      ↓
Scenario changed
      ↓
Transaction plan regenerated
      ↓
New tasks created
      ↓
Professional review requested
      ↓
New package generated
```

All these events can share a correlation identifier.

Then the system can answer:

> **“Why did this task get created?”**

---

# 30. Causation

Correlation tells us what belongs to the same overall activity.

Causation tells us:

> **What directly caused this event?**

Example:

```text id="xazlpk"
Event:
ScenarioChanged

caused:
TransactionPlanVersionCreated

which caused:
ProfessionalReviewRequired

which caused:
ProfessionalReviewTaskCreated
```

This is exceptionally useful for debugging and explainability.

---

# 31. Workflow Explainability

The owner should eventually be able to ask:

> **Why did I get this task?**

The system could answer:

> Your valuation was completed.
>
> The current transaction plan requires lender review after valuation.
>
> This task was automatically created by Workflow: “Valuation Completion v2.”
>
> Dependency: ValuationCompleted.
>
> Assigned to: Owner.
>
> Due: October 18.

This makes automation trustworthy.

---

# 32. Human Override

Automation must not eliminate human control.

Authorized users should be able to:

* Pause workflow
* Resume workflow
* Cancel workflow
* Skip a task where policy permits
* Reassign a task
* Change a deadline
* Request manual review
* Retry failed actions

Overrides should be recorded.

The workflow should distinguish:

> Automatically completed

from:

> Manually completed

---

# 33. Protected Actions

Some actions should never be automatically performed without additional authorization.

Examples:

* External disclosure
* Binding commitments
* Financial transfers
* Final owner decisions
* Legal document execution
* High-sensitivity data release
* Irreversible deletion

Workflow can create:

> **Approval Required**

rather than performing the protected action.

Consent & Access or another relevant engine then handles the authorization.

---

# 34. Workflow Permissions

Not everyone should be allowed to manipulate workflows.

The system should distinguish:

* View
* Create
* Edit
* Pause
* Resume
* Cancel
* Override
* Administer

Workflow permissions come from Identity & Access and Policy / Compliance.

---

# 35. Workflow Templates

Templates should make common processes reusable.

Examples:

### Professional Review Workflow

Request → Package → Owner Approval → Share → Review → Feedback → Completion.

### Financing Workflow

Request → Documents → Lender Review → Questions → Indicative Terms → Commitment.

### Document Request Workflow

Request → Local Availability → Owner Preparation → Authorization → Share → Confirmation.

### Closing Workflow

Requirements → Readiness Review → Outstanding Items → Final Confirmation → Closing Event.

These templates belong to Workflow infrastructure.

Domain engines supply their specific meaning and data.

---

# 36. Workflow Composition

A workflow should be able to invoke another workflow.

Example:

```text id="j6k8g7"
Transaction reaches Financing Stage
        ↓
Start Financing Workflow
        ↓
Financing Workflow completes
        ↓
Transaction Workflow resumes
```

This prevents giant workflows from becoming unmanageable.

---

# 37. Nested Workflows

A parent workflow can contain child workflows.

Example:

**Transaction Execution**

contains:

* Valuation Workflow
* Financing Workflow
* Professional Review Workflow
* Document Readiness Workflow
* Closing Workflow

The parent should see:

> Child workflow status

without owning its internal mechanics.

---

# 38. Workflow Completion

A workflow needs explicit completion criteria.

Completion should not simply mean:

> “All tasks checked.”

It might require:

* Required events received
* Required approvals obtained
* Required external confirmation
* Required system state achieved

Example:

> Financing Workflow complete only when Capital Engine reports **Committed**.

This prevents false completion.

---

# 39. Partial Completion

A workflow may be partially complete.

Example:

> 7 of 9 actions complete.

The engine should show:

* Completed
* Waiting
* Blocked
* Failed
* Not Started

rather than flattening everything into one percentage.

---

# 40. Failure Handling

A workflow should have a defined failure path.

Example:

> Document synchronization failed.

The system:

1. Retries.
2. Logs failure.
3. Attempts recovery.
4. Escalates if retries exhausted.
5. Creates manual intervention task where appropriate.

The workflow itself should not silently stop.

---

# 41. Dead-Letter / Failed Work Queue

Persistent workflow failures should go into a recoverable queue.

Example:

> **Workflow Exception**
>
> Action: Notify lender
>
> Failure: External service unavailable
>
> Attempts: 4
>
> Status: Needs attention

An authorized administrator can retry or resolve it.

The user should not need to understand infrastructure terminology.

---

# 42. Scheduling

Workflow needs a scheduling layer capable of:

* Absolute dates
* Relative dates
* Time windows
* Time zones
* Business-day calculations
* Deadline offsets
* Recurrence
* Calendar-aware actions

The scheduling system should distinguish:

> Due date

from:

> Execution time.

Example:

> Notify owner 3 business days before deadline.

---

# 43. Time Zones

Transactions may involve professionals in different locations.

Every scheduled event should retain its intended time zone.

Example:

> October 20, 9:00 AM America/Los_Angeles

rather than storing:

> 9:00 AM

with no context.

---

# 44. Time-Based Event Generation

The Workflow Engine can generate events such as:

> DeadlineApproaching

> DeadlineReached

> TaskOverdue

> AccessExpiring

> RecurrenceDue

Other engines can subscribe without having to build their own scheduling systems.

---

# 45. Example: Valuation Completed

This is the canonical example.

### Event

**ValuationCompleted**

Source:

> Valuation Engine

### Workflow rules

1. Mark valuation dependency satisfied.
2. Check whether lender review is required.
3. Create lender notification task if applicable.
4. Check Document Readiness for valuation materials.
5. Trigger transaction progress recalculation.
6. Trigger Confidence refresh.
7. Emit resulting events.

### Other engines

**Notification Engine**

> Notify lender.

**Document Readiness**

> Mark valuation requirement complete.

**Transaction Orchestration**

> Update milestone.

**Confidence Engine**

> Recalculate evidence/goal alignment.

Workflow coordinates these calls but does not perform their underlying work.

---

# 46. Example: Consent Revoked

Event:

> ConsentRevoked

Workflow may trigger:

* Notify affected participant
* Disable pending document transfer
* Cancel outstanding sharing task
* Update package status
* Create owner follow-up task
* Record transaction impact

Consent & Access remains authoritative over permission.

Workflow simply propagates the consequence.

---

# 47. Example: Scenario Changed

Event:

> ScenarioChanged

Workflow can trigger:

* Transaction Plan Review
* Professional Review requirement check
* Document Readiness refresh
* Capital requirement refresh
* Financial model refresh
* Confidence refresh

Again, Workflow coordinates.

It does not decide which scenario is preferable.

---

# 48. Example: Document Conflict Detected

Event:

> FactConflictDetected

Workflow might:

* Create verification task
* Notify owner
* Mark dependent milestone as blocked
* Notify relevant professional
* Recalculate transaction readiness

Business Reality and Fact Verification remain authoritative for the conflict itself.

---

# 49. Workflow and the Confidence Engine

This is an especially useful integration.

A material event can automatically trigger a confidence refresh:

```text id="0u1scj"
New Evidence
   ↓
Fact Verification
   ↓
Business Reality Updated
   ↓
Workflow Event
   ↓
Confidence Refresh
```

This allows confidence to evolve without the Confidence Engine monitoring every system independently.

---

# 50. Workflow and Notifications

The separation should be:

**Workflow**

> A notification is required.

**Notification**

> Send it by in-app alert + email.

This means we can change communication channels without changing workflow definitions.

---

# 51. Workflow and Audit

Workflow emits events such as:

> TaskCreated

> TaskAssigned

> TaskCompleted

> WorkflowStarted

> WorkflowFailed

Audit records the durable history.

Workflow maintains execution state.

This prevents duplicate audit implementations.

---

# 52. Workflow and Transaction Orchestration

The distinction can now be stated very simply:

### Transaction / Orchestration

> **What needs to happen in this transaction?**

### Workflow

> **What should the system do when something happens?**

Example:

Transaction says:

> Financing must be completed before closing.

Workflow says:

> When financing becomes committed, satisfy the financing dependency and notify the transaction engine.

---

# 53. Generic Reusability

The Workflow Engine should work outside business sales.

Later it could support:

* Ownership lifecycle
* Annual business reviews
* Seller-note payments
* Compliance reminders
* Professional credential renewal
* Vendor re-verification
* Document expiration
* Customer onboarding
* Platform administration
* Subscription lifecycle

That is why it must remain domain-neutral.

---

# 54. Core Data Objects

## Event

Immutable statement that something happened.

## EventSubscription

Defines which workflow or engine listens for an event.

## Trigger

Defines what starts a workflow.

## WorkflowDefinition

Reusable workflow blueprint.

## WorkflowVersion

Versioned definition of the workflow.

## WorkflowInstance

Specific execution of a workflow.

## WorkflowAction

Individual action within a workflow.

## TaskTemplate

Reusable task pattern.

## TaskInstance

Actual task.

## Dependency

Relationship determining whether work can proceed.

## Schedule

Timing and recurrence rules.

## EscalationRule

Defines what happens when expected progress does not occur.

## ExecutionAttempt

Records a specific attempt to execute an action.

## WorkflowException

Persistent execution problem requiring resolution.

---

# 55. Workflow State

Workflow instances should support:

**Draft**

**Ready**

**Running**

**Waiting**

**Paused**

**Blocked**

**Completed**

**Failed**

**Cancelled**

**Superseded**

Tasks have their own state.

Do not collapse workflow state and task state into one model.

---

# 56. Engine Contract

A clean API/event contract might include:

```text id="bdby6x"
publishEvent()
subscribeToEvent()
createWorkflowInstance()
startWorkflow()
pauseWorkflow()
resumeWorkflow()
cancelWorkflow()
createTask()
assignTask()
completeTask()
failTask()
addDependency()
resolveDependency()
scheduleAction()
createRecurringWorkflow()
evaluateTrigger()
evaluateCondition()
retryAction()
escalateTask()
getWorkflowStatus()
getExecutionHistory()
```

The engine should also expose read operations such as:

```text
whyIsTaskBlocked()
whyWasTaskCreated()
whatTriggeredWorkflow()
whatDependsOnThis()
whatWillHappenNext()
```

Those explainability methods are worth designing early.

---

# 57. Data Boundary

The Workflow Engine should mostly store **references and state**, not duplicate domain records.

For example:

Bad:

> Copy entire valuation into Workflow database.

Good:

> `valuation_id = VAL-0024`
> status = Completed

Likewise:

Bad:

> Store the entire lender document.

Good:

> `document_id = DOC-184`
> required_by = Financing Workflow
> status = Available

This reduces duplication and keeps ownership clear.

---

# 58. Security Boundary

Workflow payloads may contain sensitive information.

Therefore the system should avoid putting unnecessary sensitive data directly into event payloads.

Prefer:

> `document_id = DOC-184`

over:

> `[entire confidential document contents]`

Then the receiving engine requests the permitted resource through its proper authorization path.

This integrates naturally with the Local Vault and Consent & Access architecture.

---

# 59. Event Payload Minimization

An event should generally contain enough information to identify what happened and what needs to be evaluated.

Not necessarily everything associated with it.

Example:

```text
ValuationCompleted
valuation_id
transaction_id
completed_at
status
```

Rather than embedding:

* full valuation report
* owner PII
* financial schedules
* confidential notes

This is especially important if events travel through infrastructure outside the Local Vault.

---

# 60. Transactional Reliability

A major architectural concern is ensuring that:

> “The database says the valuation completed”

and:

> “The ValuationCompleted event was published”

do not accidentally diverge.

The implementation should therefore use a durable event-publishing pattern such as an **outbox/inbox approach** where appropriate.

The exact technology can be decided later.

The architectural requirement is:

> **State changes and their associated events must be reliably coordinated.**

---

# 61. Ordering

Some event chains require ordering.

For example:

> ScenarioChanged

should be processed before:

> TransactionPlanRegenerated

which should precede:

> ProfessionalReviewRequired

The workflow infrastructure should preserve ordering where a dependency exists.

It should not globally force every event in the entire platform into one serial queue.

Ordering should be scoped where required.

---

# 62. Duplicate and Out-of-Order Events

The engine should assume distributed systems can produce:

* duplicates
* delayed events
* retries
* out-of-order delivery

Therefore workflows need:

* event version
* timestamps
* sequence where necessary
* idempotency
* current-state checks
* causation metadata

This is foundational infrastructure, not an edge case.

---

# 63. Manual Intervention

Not every workflow problem can be automated.

The engine should be able to produce:

> **Manual Intervention Required**

with:

* What failed
* What was attempted
* Why it failed
* What remains unresolved
* Recommended operational next step, where appropriate
* Assigned owner

This is better than silently abandoning an execution chain.

---

# 64. Testing Requirements

Because this engine will sit beneath nearly everything, it needs strong automated testing.

Test categories should include:

### Trigger tests

Does the correct event start the workflow?

### Condition tests

Does the workflow branch correctly?

### Dependency tests

Does work remain blocked until required conditions are satisfied?

### Retry tests

Does a failed action retry safely?

### Idempotency tests

Does duplicate event delivery avoid duplicate actions?

### Timing tests

Are scheduled and recurring actions generated correctly?

### Escalation tests

Does an overdue task escalate correctly?

### Cancellation tests

Does stopping a workflow prevent future actions?

### Version tests

Does an older workflow definition continue to behave predictably?

### Failure recovery tests

Can an interrupted workflow resume safely?

---

# 65. Workflow Observability

Administrators need an operational view.

Example:

> **Workflow Health**

Running: 42
Waiting: 17
Blocked: 6
Failed: 2
Escalated: 3

Drill into:

> Failed Workflow #884

and see:

> Action: Notify lender
> Attempt 1: Failed
> Attempt 2: Failed
> Attempt 3: Failed
> Reason: External service unavailable
> Next action: Manual retry

The user-facing application should expose only the relevant portion.

---

# 66. Workflow Explainability for Users

Three questions should always be answerable:

### Why did this happen?

> Triggered by ValuationCompleted.

### Why is this waiting?

> Waiting for lender response.

### What happens next?

> When lender response is received, financing review will resume.

This is the human-readable side of workflow infrastructure.

---

# 67. Automation Transparency

Automated actions should be visible.

Example:

> **Automatically updated**
>
> Valuation completion satisfied the valuation dependency.

> **Automatically created**
>
> Task for lender review.

> **Awaiting human action**
>
> Owner approval required.

> **Automatically scheduled**
>
> Follow-up in 5 days.

The system should never make significant workflow changes feel like ghosts rearranging the furniture.

---

# 68. Workflow Templates and Versioning

A template change must not unexpectedly rewrite existing workflow instances.

Example:

**Financing Workflow v1**

used by Transaction TX-001.

Later:

**Financing Workflow v2**

is released.

TX-001 should continue using v1 unless deliberately migrated.

New transactions use v2.

This is another important expression of the overall:

> **independently evolving engines** principle.

---

# 69. Workflow Migration

For long-running workflows, migration may eventually be required.

For example:

> Regulatory or platform change requires new workflow behavior.

Migration should be explicit:

* Current workflow version
* Target version
* Migration rules
* Affected instances
* Owner/administrator approval where required
* History retained

Never silently transform a running workflow.

---

# 70. Protected Workflow Categories

Some workflows should receive additional safeguards.

Examples:

* Highly sensitive data
* External disclosure
* Financial transactions
* Ownership transfer
* Closing
* Legal execution
* Irreversible deletion

These can require:

* Explicit authorization
* Additional confirmation
* Professional determination
* Multiple approvals
* Enhanced audit

Policy / Compliance defines those requirements.

Workflow enforces the resulting control points.

---

# 71. Owner Experience

The owner does not need to see:

> EventSubscription #7741

They should see:

> **Automatic updates**
>
> Valuation completed.
>
> ✓ Valuation requirement updated
> ✓ Lender notified
> ✓ Transaction milestone updated
> ✓ Confidence refreshed

Then:

> **Your next action**
>
> Provide updated debt schedule.

The underlying event machinery remains invisible unless the user asks for an explanation.

---

# 72. Architectural Lock

These should now be treated as requirements:

**1. Workflow is a standalone infrastructure engine.**

**2. Workflow is reusable across the entire platform, not transaction-specific.**

**3. Events represent things that happened.**

**4. Commands represent requested actions.**

**5. Tasks represent work that needs to be performed.**

**6. Workflow definitions are versioned and reusable.**

**7. Workflow instances preserve the actual execution history.**

**8. Triggers and conditions are explicit.**

**9. Dependencies are first-class objects.**

**10. Recurring actions are supported.**

**11. Escalations are supported.**

**12. Retries and failure handling are built in.**

**13. Event processing is idempotent.**

**14. The architecture assumes at-least-once event delivery rather than relying on exactly-once delivery.**

**15. Correlation and causation identifiers are preserved.**

**16. Workflow state and task state remain separate.**

**17. Workflow does not own domain meaning.**

**18. Workflow does not duplicate entire domain records.**

**19. Sensitive data should not be unnecessarily embedded in event payloads.**

**20. External disclosure and other protected actions require the appropriate authorization engine.**

**21. Workflow can pause, resume, cancel, retry, and escalate.**

**22. Failed workflows remain recoverable.**

**23. Significant automated actions are explainable to the user.**

**24. Workflow templates do not silently alter existing workflow instances.**

**25. Other engines remain authoritative over their own data and decisions.**

**26. Workflow coordinates engines through structured contracts and events rather than creating hidden direct dependencies.**

**27. Transaction / Orchestration uses Workflow infrastructure but remains responsible for transaction-specific meaning.**

**28. Workflow must be independently replaceable and evolvable without requiring the other engines to be rewritten.**

---

# 73. Architectural Boundary Summary

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Workflow** | Events and subscriptions, triggers, workflow definitions and instances, tasks and task templates, dependencies, scheduling, recurring actions, escalations, retries, completion and failure handling, idempotency, execution state, and execution telemetry | Domain meaning, or the data a workflow operates on |
| **Transaction / Orchestration** | Transaction plans, stages, milestones, and transaction-specific meaning | The execution mechanics that carry them out |
| **Professional Review** | Professional determinations and their attribution | The workflow that requests or routes them |
| **Business Reality** | Current business facts and financial statements | The work performed on them |
| **Financial Modeling** | Calculations, cash flows, debt schedules, and projections | The workflow that schedules them |
| **Document Readiness** | Document status, completeness, and outstanding requests | The task that requests a document |
| **Review Package** | Purpose-built packages assembled for a named recipient | The workflow that assembles them |
| **Closing** | Closing execution and its requirements | The workflow steps that satisfy them |
| **Ownership Lifecycle** | Post-transaction ownership state | The workflows that maintain it |
| **Decision Record** | The owner's recorded reasoning and rationale | The workflow a decision triggers |
| **Communication** | Conversations and messages | Task state and execution mechanics |
| **Notification** | Delivery of attention, including follow-up alerts | The workflow event that prompts an alert |
| **Consent & Access** | Who may see which resource, for what purpose, and for how long | Whether a protected action may proceed |
| **Identity & Access** | Who a party is and what they may do on the platform | Task assignment mechanics |
| **Local Vault** | Private source documents, storage, encryption, and versioning | Workflow state |
| **Audit / Provenance** | The historical record of what happened | Current execution state |
| **Policy / Compliance** | The rules governing how engines may operate | Execution mechanics |
| **Integration** | Connectivity to external systems | Workflow orchestration |
| **Billing / Commercial** | Subscription entitlements and billing state | Recurring workflow execution |

## Hard Boundary

The Workflow Engine owns execution mechanics: events and subscriptions, triggers, workflow definitions and instances, tasks and dependencies, scheduling, recurring actions, escalations, retries, failure handling, idempotency, execution state, and telemetry. It does not own domain meaning. It can invoke other engines or react to their events; it does not absorb their responsibilities.

**Workflow does not own domain meaning and does not duplicate entire domain records.** Other engines remain authoritative over their own data and decisions, protected actions require the appropriate authorization engine, and Transaction / Orchestration uses Workflow infrastructure while remaining responsible for transaction-specific meaning. Workflow coordinates engines through structured contracts and events rather than hidden direct dependencies.

---

# 74. The Architecture We Have Now

The distinction among the three layers is particularly clean:

```text id="aq8qzp"
                    WHAT
                     │
                     ▼
        TRANSACTION / ORCHESTRATION
        "What needs to happen?"
                     │
                     ▼
                    HOW
                     │
                     ▼
               WORKFLOW ENGINE
        "When and how does it happen?"
                     │
                     ▼
                  WHO / WHERE
                     │
          ┌──────────┼───────────┐
          ▼          ▼           ▼
       PEOPLE      SYSTEMS    OTHER ENGINES
```

And underneath that:

```text id="m1w1ta"
EVENT
  ↓
TRIGGER
  ↓
CONDITION
  ↓
WORKFLOW
  ↓
TASK / ACTION
  ↓
DEPENDENCY
  ↓
COMPLETION
  ↓
NEW EVENT
```

That creates the platform's **circulatory system**.

A professional completes a review, a document arrives, financing changes, a permission expires, an owner makes a decision, or a milestone is reached. The Workflow Engine carries those changes through the system so every affected engine can respond without being tightly welded to every other engine.

The beauty of this architecture is that **adding a new engine later does not require rewriting the transaction system**. The new engine subscribes to the events it understands and publishes the events that matter to others.

That is exactly what we want from the architecture we've been building: **small engines with very clear jobs, connected by predictable contracts rather than one giant application pretending to be everything.** 🔧
