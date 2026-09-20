This is the **black box recorder** of the platform. It should be comprehensive enough to reconstruct what happened, but disciplined enough not to become a dumping ground for every internal technical event.

The key boundary is:

> **Audit / Provenance records factual history. It does not explain intent, make judgments, or become the source of domain truth.**

# Audit / Provenance Engine

## 1. Purpose

The Audit / Provenance Engine maintains the platform-wide historical record of meaningful activity.

It answers:

> **Who did what, when did they do it, what changed, what existed before, what exists now, who approved it, which engine produced it, and what information was disclosed or revoked?**

It provides a durable record of:

* Changes
* Approvals
* Decisions as actions, while Decision Record owns the reasoning
* System-generated changes
* Engine-generated outputs
* Sharing
* Revocation
* Access events
* Workflow events
* Version changes
* State transitions
* Administrative actions
* Important automated activity

Its purpose is **traceability, accountability, reconstruction, and provenance**.

---

# 2. Core Architectural Principle

## Audit records history, not meaning.

Example:

### Audit

> Target closing date changed from November 30 to December 15 by Owner at 2:14 PM.

### Decision Record

> Owner changed the date to allow additional employee preparation time.

### Workflow

> Eight dependent deadlines recalculated.

### Notification

> Relevant participants were notified.

Four different systems tell four different parts of the story.

Audit should preserve the factual event without attempting to replace those other records.

---

# 3. What This Engine Owns

The Audit / Provenance Engine owns:

* Audit events
* Change history
* Previous values
* New values
* Actors
* Timestamps
* Approvals
* Source engine
* Source object
* Event relationships
* Sharing records
* Revocation records
* Administrative actions
* System-generated actions
* Version lineage
* Causation references
* Correlation references
* Audit retention metadata
* Audit integrity mechanisms

---

# 4. What It Does Not Own

It does not own:

* Owner rationale
* Professional determinations
* Business facts
* Research evidence
* Document contents
* Permissions themselves
* Workflow state
* Transaction state
* Decisions themselves
* Communication content as the communication system of record

It records important events from those systems.

---

# 5. The Four Provenance Questions

Every meaningful audit record should help answer:

### What happened?

Example:

> Business Reality fact changed.

### Who caused it?

Example:

> Owner.

### What changed?

Example:

> Revenue: $8.2M → $8.5M.

### Why can we trust the history?

Example:

> Source engine: Fact Verification Engine
> Event ID: EVT-8831
> Previous version: FACT-17-v4
> New version: FACT-17-v5

The “why” in a decision-making sense is generally answered by Decision Record, not Audit.

---

# 6. Audit Event

The fundamental object is the **Audit Event**.

It should contain:

* Audit Event ID
* Event type
* Timestamp
* Actor
* Actor role
* Organization/workspace
* Source engine
* Source object
* Action
* Previous value/reference
* New value/reference
* Approval reference, where applicable
* Related transaction
* Related decision
* Related workflow
* Related communication
* Related consent
* Correlation ID
* Causation ID
* Event version
* Sensitivity
* Integrity metadata

---

# 7. Actor

The system should distinguish the actor responsible for an event.

Possible actors:

* Owner
* Co-owner
* Professional
* Employee
* Lender
* Trustee
* Buyer
* Platform administrator
* Service account
* Automation
* AI agent
* Integration
* System process

This is important.

The platform should never record:

> “User changed value”

when the actual actor was:

> Automated workflow.

---

# 8. Human vs System Action

Every audit event should clearly identify whether the action was:

**Human initiated**

**Human approved**

**System generated**

**Automated workflow**

**External integration**

**AI-assisted**

**AI-generated**

This prevents automation from appearing as though a human personally performed every action.

---

# 9. Who Changed What?

The engine must support field-level change records where material.

Example:

```text id="i2x6pz"
Object:
Transaction TX-004

Field:
target_closing_date

Previous:
2026-11-30

New:
2026-12-15

Actor:
Owner

Timestamp:
2026-09-19 14:14

Source:
Transaction Orchestration
```

This is much more useful than:

> Transaction modified.

---

# 10. Previous and New Values

Material state changes should preserve both:

> Previous value

and:

> New value

Examples:

**Revenue**

$8,200,000 → $8,500,000

**Transaction state**

Planning → Active

**Task owner**

Owner → CPA

**Access**

Active → Revoked

**Closing date**

November 30 → December 15

This makes historical reconstruction possible.

---

# 11. Value Types

Audit should handle:

* Text
* Number
* Currency
* Percentage
* Date
* Date/time
* Boolean
* Enumerated state
* Structured object
* Relationship
* Document/version reference

For highly sensitive values, the platform may record:

> Value changed

without storing the raw sensitive content in the audit log, depending on policy.

---

# 12. Reference vs Copy

Where possible, Audit should reference objects rather than duplicate their full contents.

Example:

> `document_version_id = DOC-184-v4`

rather than copying the entire document.

This keeps the audit store smaller and reduces unnecessary sensitive-data duplication.

---

# 13. Sensitive Data

The Audit Engine itself can become highly sensitive because it may reveal:

* Who worked on a transaction
* What documents were shared
* When permissions changed
* Financial changes
* Professional activity
* Transaction timing
* Internal decisions

Therefore:

> **The audit system must itself be protected.**

An audit log cannot be a backdoor around the privacy model.

---

# 14. Who Can View Audit?

Access should be role-controlled.

Possible levels:

### Owner

Broad visibility into their own transaction history.

### Authorized professionals

Relevant audit history for their scope.

### Platform administrator

System and operational audit access according to policy.

### External participant

Only narrowly relevant history.

A lender should not automatically see:

> Internal owner activity.

---

# 15. Immutable History

Audit records should be append-only.

Once recorded:

> An audit event should not be silently edited.

If a correction is necessary:

> Create a corrective audit event.

Example:

```text id="w3h8ac"
Original:
Revenue changed to $8.5M

Correction:
Prior event contained incorrect metadata.
Corrected source reference added.
```

The original event remains visible with its correction.

---

# 16. Audit Integrity

The system should support mechanisms that make unauthorized alteration detectable.

Potential techniques include:

* Append-only storage
* Cryptographic hashes
* Hash chaining
* Signed event records
* Immutable storage tiers
* Periodic integrity verification

The exact implementation can be determined later.

The architectural requirement is:

> **The system must be able to detect tampering with historical audit records.**

---

# 17. Chain of Events

Audit should support event lineage.

Example:

```text id="7l8yxz"
OwnerDecisionRecorded
       ↓
ScenarioChanged
       ↓
TransactionPlanVersionCreated
       ↓
ProfessionalReviewRequested
       ↓
PackageCreated
       ↓
ConsentGranted
       ↓
DocumentsShared
```

Each event can reference:

* What caused it
* What it caused

This makes the transaction reconstructable.

---

# 18. Correlation ID

Related activity receives a shared correlation identifier.

For example:

> Transaction Plan Change #COR-882.

Then the audit system can show every related event.

This answers:

> **“Show me everything that happened because of this change.”**

---

# 19. Causation ID

Each event can identify the immediate event that caused it.

Example:

```text id="f3y8tm"
ScenarioChanged
      ↓
causes
TransactionPlanRegenerated
      ↓
causes
ProfessionalReviewRequired
```

This distinguishes:

> Related activity

from:

> Direct causal chain.

---

# 20. Source Engine

Every meaningful audit record should identify the engine that produced the event.

Examples:

* Destination
* Business Reality
* Fact Verification
* Research
* Evidence Ledger
* Scenario
* Financial Modeling
* Capital / Financing
* Seller-Note Liquidity
* Marketplace
* Professional Review
* Professional Package
* Document Readiness
* Consent & Access
* Local Vault
* Transaction Orchestration
* Workflow
* Communication
* Notification
* Decision Records

This answers:

> **“Which system generated this change?”**

---

# 21. Version Provenance

Every material object should have version lineage.

Example:

```text id="70ln0j"
Business Reality Fact
v1 → v2 → v3 → v4
```

Audit records:

> Fact v4 created from v3 by Fact Verification Engine.

This is especially important when current values differ from historical values.

---

# 22. Approvals

Audit records approvals as actions.

Example:

> Owner approved Professional Package v5.

Audit:

**Actor:** Owner
**Action:** Approved
**Object:** Package v5
**Timestamp:** September 19
**Source:** Professional Review Package Engine

Decision Record may separately preserve:

> Why the owner approved it.

---

# 23. Approval Is Not Agreement

The audit event should distinguish:

> Approved

from:

> Acknowledged

from:

> Accepted

from:

> Agreed

Those states should not be treated as interchangeable.

The relevant domain engine defines their meaning.

Audit records the actual action.

---

# 24. Sharing Provenance

One of the most important audit functions is recording disclosures.

Example:

> Financial Statements v4 shared with ABC Bank.

Audit record:

* Actor
* Recipient
* Document/version
* Purpose
* Authorization reference
* Timestamp
* Sharing mechanism
* Related package
* Related transaction

This should tie directly to Consent & Access and Disclosure Records.

---

# 25. Revocation Provenance

Likewise:

> ABC Bank access revoked.

Audit preserves:

* Who revoked it
* What access was revoked
* What resource was affected
* When
* Authorization/grant reference
* Reason, when provided

The Consent Engine owns current authorization.

Audit preserves the historical event.

---

# 26. Access History

Audit can answer:

> Who has had access to this document?

For example:

> CPA Jones
> Access granted Sept 10
> Viewed Sept 11
> Access expired Oct 10

> ABC Bank
> Access granted Sept 14
> Access revoked Sept 20

This should be reconstructed from Consent and disclosure events rather than maintained as a second authorization system.

---

# 27. Sharing vs Viewing

These are distinct events.

### Sharing

> Platform disclosed document to recipient.

### Viewing

> Recipient accessed the document.

### Download

> Recipient created a downloadable copy.

Those should not be collapsed.

This distinction becomes important for sensitive transaction information.

---

# 28. Administrative Actions

Platform administrators may:

* Approve vendors
* Suspend vendors
* Change workflow templates
* Alter platform configuration
* Resolve system issues
* Revoke access
* Correct metadata

These actions must be auditable.

For example:

> Admin suspended Professional Organization X.

Audit records:

* Admin identity
* Organization
* Previous status
* New status
* Timestamp
* Administrative reason/reference
* Related vetting case

---

# 29. System Configuration Changes

Important platform configuration changes should also be logged.

Examples:

* Policy changed
* Notification rule changed
* Workflow definition version released
* Marketplace visibility changed
* Credential requirement changed
* Security configuration changed

This becomes important for explaining why the system behaved differently at different points in time.

---

# 30. Workflow Provenance

Workflow is highly automated.

Audit should record material automation.

Example:

> Workflow v3 created Task T-884 automatically because ValuationCompleted occurred.

That allows support staff to answer:

> “Why did the system create this task?”

---

# 31. AI Provenance

AI-generated activity deserves its own provenance fields.

For AI-assisted outputs, record where appropriate:

* AI/system identity
* Model/service identifier
* Model version
* Prompt/input reference
* Source objects
* Human approval
* Timestamp
* Output object

The system should distinguish:

> AI generated

from:

> Human approved AI output

from:

> Human authored.

This is especially important when AI contributes to business analysis, research, communication drafts, or workflow decisions.

---

# 32. AI Does Not Become the Actor of Human Decisions

If AI suggests:

> “Consider moving the closing date.”

and the owner then changes it:

Audit should show:

**AI suggestion generated**

and separately:

**Owner changed closing date**

The AI should not appear as the decision-maker.

---

# 33. Communication Provenance

Audit can record meaningful communication events:

* Conversation created
* Participant added
* Participant removed
* Important request created
* Message sent
* Message edited
* Communication access changed

It should not duplicate the entire communication database.

Communication remains the source for actual message history.

---

# 34. Decision Provenance

Audit records:

> Decision Record created.

> Decision changed.

> Decision superseded.

But Decision Record remains the source for:

> What the owner decided and why.

This distinction should be preserved.

---

# 35. Professional Provenance

Audit can record:

> Professional review completed.

> Professional determination updated.

> Professional assigned.

But Professional Review remains the source for the substantive professional record.

---

# 36. Document Provenance

Audit can record:

> Document imported.

> Version created.

> Document shared.

> Document revoked from access.

> Document archived.

But Local Vault remains the source for the actual private file.

---

# 37. Fact Provenance

Audit records:

> Business Reality value changed.

Evidence Ledger records:

> Research source supporting a claim.

Fact Verification records:

> Conflict resolution.

These are different kinds of provenance.

This distinction should be hard-coded into the architecture.

---

# 38. Evidence Ledger Boundary

The **Evidence Ledger** is research-specific provenance.

Example:

> Industry benchmark published March 2026.

It records:

* Source
* Publication date
* Retrieval date
* Scope
* Supporting evidence
* Contradictory evidence

Audit instead records:

> Research finding was added to the workspace by Research Engine.

Audit does not replace Evidence Ledger.

---

# 39. Provenance Types

The engine should distinguish at least:

### Operational provenance

Who performed an action.

### Data provenance

Where a value came from.

### Disclosure provenance

What was shared and with whom.

### System provenance

Which engine generated an output.

### Approval provenance

Who authorized an action.

### Version provenance

How an object evolved.

This makes the audit system much more useful than a generic activity log.

---

# 40. Search

Authorized users should be able to search:

> “Everything that changed the purchase price.”

or:

> “What was shared with the lender?”

or:

> “Who revoked the attorney's access?”

or:

> “What changed after the scenario changed?”

or:

> “Which engine created this task?”

Audit search should respect the viewer's current permissions.

---

# 41. Object History

Every major object should have:

> **History**

Example:

### Transaction

Created
→ Scenario selected
→ Plan v1
→ Plan v2
→ Financing added
→ Closing date changed
→ Ready to Close

### Document

Imported
→ v1
→ v2
→ Shared with CPA
→ Access expired
→ v3

This provides a human-readable object timeline.

---

# 42. “What Changed?”

The platform should provide a change comparison.

Example:

> **Transaction Plan v4 → v5**

Added:

> 6 financing tasks

Removed:

> 3 seller-note tasks

Changed:

> Closing target

Added:

> Trustee review requirement

This can combine Audit with versioning from the relevant engine.

---

# 43. “Who Changed It?”

From any important object:

> **Who changed this?**

The system can show:

> Owner
> Sept 19
> Changed closing date.

> Workflow
> Sept 19
> Recalculated 8 dependent dates.

> Lender
> Sept 21
> Updated financing status.

This becomes a powerful transparency feature.

---

# 44. “Why Did It Change?”

Audit can answer:

> Which event caused the change?

It should then link to Decision Record, Communication, or Workflow when those systems contain the substantive reason.

Example:

> Closing date changed because of Decision Record D-014.

This prevents Audit from inventing a rationale.

---

# 45. Time Travel

The platform should eventually be able to reconstruct historical state.

Example:

> **Show transaction as it existed on September 19 at 2:00 PM.**

Using versioned records and audit events, the platform can reconstruct:

* Destination
* Business Reality
* Scenario
* Decision state
* Transaction plan
* Professional status
* Access status

This is technically demanding but architecturally valuable.

---

# 46. Historical State vs Current State

The system should always distinguish:

> **What is true now**

from:

> **What was true then.**

Audit is one of the mechanisms supporting this distinction.

---

# 47. Event Ordering

Audit records should retain:

* Event timestamp
* Server receipt time
* Sequence where required
* Causation
* Correlation

Because distributed systems can receive events out of order.

The system should not assume:

> Database timestamp = actual event sequence.

---

# 48. Clock Differences

External systems and devices may have incorrect clocks.

Therefore the platform should maintain a trusted server-side timestamp in addition to source timestamps.

Example:

> Client timestamp
> Server received timestamp

This improves forensic reconstruction.

---

# 49. Corrections

Sometimes source information is corrected.

Audit should record both:

> Original entry

and:

> Correction.

Example:

> Document originally classified Confidential.

Later:

> Classification corrected to Sensitive.

The system should not erase the original classification history.

---

# 50. Bulk Actions

Bulk changes should be auditable.

Example:

> Owner shared 12 documents with lender.

Audit should be able to represent the operation as:

> Bulk Share Operation #B-118

with individual resource references beneath it.

This avoids either:

* losing detail, or
* creating an unreadable flood of duplicate records.

---

# 51. Automated Batches

The same applies to automated operations.

Example:

> Workflow refreshed 18 Document Readiness statuses.

The audit history can show:

> Batch operation

with links to each affected object.

---

# 52. Audit Retention

Audit records often require longer retention than ordinary operational data.

Retention should support:

* Active
* Archived
* Long-term retained
* Restricted
* Policy-governed deletion where permitted

Policy / Compliance controls retention requirements.

The Audit Engine implements retention mechanics.

---

# 53. Audit Export

Authorized users may eventually need to export a transaction history.

Examples:

> Professional history

> Transaction activity report

> Disclosure history

> Regulatory or legal response package

> Internal review

Exports should themselves be audited.

Example:

> Owner exported audit history on September 20.

---

# 54. Audit Integrity Export

For highly sensitive or formal uses, the platform could eventually provide:

> **Integrity-verified audit export**

with:

* Event identifiers
* Timestamps
* Hashes
* Chain information
* Export generation metadata

This provides evidence that the exported history corresponds to a specific recorded audit state.

---

# 55. Privacy-Preserving Audit

The audit trail should be comprehensive without becoming an accidental copy of everything.

Examples:

Instead of storing an entire tax return:

> Document DOC-184 v3 shared with CPA.

Instead of storing the entire message:

> Message MSG-8841 sent.

Instead of storing a whole professional report:

> Professional Determination PR-221 recorded.

The authoritative content remains in its owning engine.

---

# 56. Audit Dashboard

The owner should have a simple view:

## Recent Activity

> Financial statements updated.

> CPA access granted.

> Valuation completed.

> Closing date changed.

> Lender package shared.

Each item can expand into:

> **Who / What / When / Source / Related Objects**

---

# 57. Advanced Audit View

Authorized advanced users can see:

```text id="5h0rqs"
Event ID
Timestamp
Actor
Source Engine
Object
Previous State
New State
Causation
Correlation
Approval
Related Decision
Related Workflow
Related Disclosure
```

This provides the technical depth needed without forcing ordinary users to navigate it.

---

# 58. Audit Alerts

Audit itself should not normally generate notifications.

Important audit events can emit events to Workflow or Notification.

Example:

> Unexpected bulk export detected.

Workflow:

> Review security event.

Notification:

> Security-related activity requires attention.

This preserves the separation.

---

# 59. Security Events

Some events should be flagged for security monitoring:

* Repeated failed access
* Unusual downloads
* Bulk export
* Permission escalation
* Administrative access
* Device revocation
* Unexpected deletion
* Key-management changes

The Security/Policy layer may consume audit events to enforce controls.

---

# 60. No Hidden Administrative Activity

Administrative actions should not disappear behind:

> “System.”

Whenever technically meaningful, the system should identify:

> Human administrator

or:

> Named automated service.

This is important for trust.

---

# 61. Core Data Objects

## AuditEvent

Individual historical event.

## ChangeRecord

Specific before/after value change.

## ActorRecord

Human, service, automation, or external actor.

## ApprovalRecord

Reference to an authorization or approval event.

## DisclosureRecord

Record of information shared.

## RevocationRecord

Record of access withdrawal.

## VersionReference

Relationship among object versions.

## EventRelationship

Causation/correlation relationship.

## AuditExport

Exported historical record and its integrity metadata.

## AuditRetentionPolicy

Retention and archival configuration.

---

# 62. Engine Contract

Core capabilities:

```text id="a6q8fy"
recordEvent()
recordChange()
recordApproval()
recordDisclosure()
recordRevocation()
recordSystemAction()
recordVersionChange()
linkCausation()
linkCorrelation()
getObjectHistory()
getChangeHistory()
getDisclosureHistory()
getAccessHistory()
getDecisionRelatedActivity()
getTransactionTimeline()
reconstructHistoricalState()
searchAudit()
exportAudit()
verifyIntegrity()
```

Administrative capabilities:

```text id="p8v2wh"
archiveAudit()
restoreAudit()
verifyAuditChain()
manageRetention()
createIntegrityExport()
```

---

# 63. Event Contract

Events received from other engines should contain:

```text id="b7p4rt"
event_id
event_type
event_version
source_engine
source_object
actor
timestamp
workspace_id
transaction_id
correlation_id
causation_id
payload_reference
```

Audit enriches and preserves the historical representation.

---

# 64. Audit vs Workflow

### Workflow

> What should happen next?

### Audit

> What actually happened?

Workflow may say:

> Retry notification.

Audit says:

> Notification attempt failed at 10:41 AM.

---

# 65. Audit vs Communication

### Communication

> Actual message conversation.

### Audit

> Message was sent at 10:41 AM by the owner.

Audit should not become a duplicate chat history.

---

# 66. Audit vs Consent

### Consent

> Attorney currently has access.

### Audit

> Owner granted access September 10, changed duration September 12, and revoked access September 20.

Consent tells us:

> Current permission.

Audit tells us:

> Permission history.

---

# 67. Audit vs Decision Records

### Decision Record

> Why owner chose staged ownership.

### Audit

> Decision Record D-014 was created on September 19.

This distinction should remain absolute.

---

# 68. Audit vs Evidence Ledger

### Evidence Ledger

> Why a research claim is supported or contradicted.

### Audit

> Research finding RF-883 was added to the workspace by Research Engine.

Research provenance and platform activity provenance are separate.

---

# 69. Audit vs Local Vault

### Local Vault

> Financial Statements v4 exists locally.

### Audit

> Financial Statements v4 was imported on September 19 and shared with ABC Bank on September 22.

The Vault owns the file.

Audit owns the activity history.

---

# 70. Audit vs Transaction Orchestration

### Transaction

> Financing milestone is complete.

### Audit

> Financing milestone changed from In Progress to Complete at 4:03 PM after Capital Engine emitted FinancingCommitted.

Again, current transaction state and historical event history remain separate.

---

# 71. What the Engine Should Never Do

It should never:

* Rewrite history silently
* Invent intent
* Judge decisions
* Become the source of business truth
* Replace Decision Records
* Replace Evidence Ledger
* Replace Communication history
* Replace Consent authorization
* Store unnecessary copies of sensitive documents
* Pretend an automated action was performed by a human
* Hide administrator actions behind “system”
* Delete an event simply because the underlying object changed
* Assume current state explains historical state

---

# 72. Architectural Lock

These should now be treated as requirements:

**1. Audit / Provenance is a standalone engine.**

**2. It records meaningful platform history rather than every low-level technical operation.**

**3. Every material change identifies who or what caused it.**

**4. Material changes preserve previous and new values or references.**

**5. Human, automated, AI, integration, and system actions are distinguishable.**

**6. Every material event identifies its source engine.**

**7. Causation and correlation relationships are preserved.**

**8. Approvals are auditable.**

**9. Sharing is auditable.**

**10. Revocation is auditable.**

**11. Version lineage is auditable.**

**12. Audit history is append-only and tamper-evident.**

**13. Corrections create additional events rather than rewriting history.**

**14. Sensitive information is minimized within audit records.**

**15. Audit references authoritative objects rather than duplicating their contents.**

**16. Audit itself is subject to access control.**

**17. Audit never grants access merely because it records that access once existed.**

**18. Audit records technical facts, not the owner's rationale.**

**19. Decision Records preserve substantive human reasoning.**

**20. Evidence Ledger preserves research provenance.**

**21. Communication preserves actual conversation.**

**22. Consent & Access preserves current authorization.**

**23. Transaction Orchestration preserves current execution state.**

**24. Workflow preserves execution mechanics.**

**25. Audit preserves the historical chain connecting those systems.**

**26. Historical reconstruction must use versioned information from the relevant point in time.**

**27. AI assistance and AI-generated actions are explicitly attributable.**

**28. Audit exports are themselves auditable.**

**29. Integrity verification is supported.**

**30. The engine is independently versioned, tested, and replaceable.**

---

# 73. Architectural Boundary Summary

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Audit / Provenance** | The append-only historical record: what happened, who or what caused it, previous and new values, approvals, sharing and revocation records, version lineage, integrity mechanisms | The reasoning behind a decision, the truth of a business fact, or the authority to act |
| **Decision Record** | The owner's recorded reasoning and rationale | The technical history of what changed |
| **Evidence Ledger** | The provenance and authority of research evidence | The platform-wide change history |
| **Communication** | The actual conversation and what was said | The immutable record of platform events |
| **Professional Review** | Professional determinations and their attribution | The record of how a determination was produced |
| **Business Reality** | Current business facts and financial statements | The history of how those facts changed |
| **Document Readiness** | Document status, completeness, and outstanding requests | The audit history of document access |
| **Workflow** | Execution mechanics and workflow state | The immutable record of completed steps |
| **Transaction / Orchestration** | Current execution state and the execution plan | The historical chain of execution |
| **Consent & Access** | Current authorization — who may see what now | The historical record of what was once permitted |
| **Identity & Access** | Who a party is and what they may do on the platform | The audit trail of privileged actions |
| **Local Vault** | Private source documents, storage, encryption, and versioning | The audit record of who accessed them |
| **Policy / Compliance** | The rules governing how engines may operate | The historical record of policy decisions |
| **Notification** | Delivery of attention, including follow-up alerts | The record that attention was delivered |
| **Integration** | Connectivity to external systems | The audit record of integration actions |

## Hard Boundary

The Audit / Provenance Engine owns the platform's history: what happened, who or what caused it, what changed, and whether the record can be trusted. It does not own meaning. It records the owner's choice without judging it, a professional's determination without endorsing it, and a business fact without becoming its source of truth.

**Audit records history. It does not own the systems it observes, and it does not grant access merely because it records that access once existed.** Decision Record preserves the owner's reasoning, Evidence Ledger preserves research provenance, Communication preserves actual conversation, Consent & Access preserves current authorization, Transaction / Orchestration preserves current execution state, and Workflow preserves execution mechanics. Audit preserves the historical chain connecting all of them.

---

# 74. The Platform's History Stack

We now have a particularly strong distinction among the major historical records:

```text id="q3m8yc"
             WHY THE OWNER CHOSE IT
                    │
              DECISION RECORD
                    │
                    ▼
            WHAT PROFESSIONALS
                DETERMINED
                    │
            PROFESSIONAL REVIEW
                    │
                    ▼
            WHAT PEOPLE SAID
                    │
              COMMUNICATION
                    │
                    ▼
             WHAT SHOULD HAPPEN
                    │
                 WORKFLOW
                    │
                    ▼
              WHAT IS SUPPOSED
                 TO HAPPEN
                    │
          TRANSACTION ORCHESTRATION
                    │
                    ▼
             WHAT ACTUALLY
               HAPPENED
                    │
                  AUDIT
```

And alongside that:

```text id="o8r4xt"
WHAT SUPPORTS A RESEARCH CLAIM
            │
      EVIDENCE LEDGER
```

and:

```text id="y5x7mj"
WHO MAY SEE INFORMATION NOW
            │
      CONSENT & ACCESS
```

and:

```text id="p6k2ns"
WHERE THE PRIVATE SOURCE LIVES
            │
        LOCAL VAULT
```

That is a remarkably clean separation.

The Audit Engine becomes the platform's **historical spine**, but not its brain. It tells us what happened without pretending to know what someone intended, whether a decision was wise, whether a professional was correct, or whether a transaction should proceed.

That keeps the architecture explainable all the way down. 🧭
