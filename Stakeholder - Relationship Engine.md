This is the other half of the Stakeholder pair. We already defined **what a stakeholder may see** — the three visibility levels as data-permission policies, in the Stakeholder Document & Visibility Architecture. This defines **who the stakeholders are**, and why they are there at all.

The split is deliberate. *Who participates* and *how much they may see* are two different decisions, made at different times, by different actors. The owner adds a lender to the transaction. The platform sets what a lender sees by default. Collapsing those into one engine means one engine owns both the membership and the disclosure policy — and neither question gets a clean answer.

# Stakeholder / Relationship Engine

## 1. Purpose

The Stakeholder / Relationship Engine answers one question:

> **Who participates in this transaction, and why?**

It manages:

* The participant roster for a transaction
* Each participant's role *in this transaction*
* The reason that person or organization is involved
* Organization and contact hierarchy
* Relationship state and lifecycle
* The participant references every other engine points at
* Relationship history — who was added, changed, or removed, and when

It is the engine that makes a transaction a **social object** rather than a document set. Without it, "the attorney" is a name in a field. With it, the attorney is a participant with a role, a reason, an organization, and a state.

---

# 2. Core Principle — Relationship, Not Identity

The single most important distinction in this engine:

> **Identity answers "who are you?" Relationship answers "why are you here?"**

A person has **one** identity. That identity may participate in **many** transactions, in **different** roles, for **different** reasons.

The same accountant may be:

* The owner's personal CPA in one transaction
* A buyer-side advisor in another
* A minority shareholder with an ownership interest in a third

Identity & Access owns the person. This engine owns the **participation**.

That distinction is why Identity & Access explicitly forbids the assumption that *a person's role is identical in every transaction*. The role is not a property of the person. It is a property of the relationship between that person and one transaction.

---

# 3. The `StakeholderRelationship` Object

The core object:

## `StakeholderRelationship`

It defines:

* The transaction it belongs to
* The participant reference (never a duplicated identity record)
* Role in this transaction
* Reason for participation
* Organization, where the participant acts through one
* Contact routing for this transaction
* Relationship state
* Start date and end date
* Who added them, and why
* Role history, where the role changes over time

## `ParticipantReference`

Other engines must never store their own copy of a participant's identity. They store a **reference**:

* The relationship ID
* The transaction ID
* The role, where the consuming engine needs it

This is the rule Communication states directly: participants should be references to this engine rather than duplicate identity records.

## `Organization`

Where a participant acts through a firm:

* Organization identity reference
* The firm's relationship to the transaction
* Which individuals act for the firm
* Which individual is primary
* Organization-level versus individual-level contact routing

---

# 4. Stakeholder Roles

The first Employee Ownership journey anticipates these roles. The list is a starting set, not a closed enum — a transaction type may introduce roles we have not named.

## Owner

The person whose objective the transaction exists to serve. Exactly one owner in the simple case; co-owners are separate participants with their own interests.

## Employees / Employee Group

The population that may become owners. Usually represented as a **group relationship** rather than one relationship per employee until allocation is defined — the employee-level detail belongs to the ownership structure, not to this engine.

## Management

Individuals who may become owners and who carry transition responsibility. Often overlaps with Employees; the overlap is expressed as two roles on one identity, never as two identities.

## Attorney

Counsel to the owner or to the transaction.

## CPA / Tax Professional

Financial and tax adviser.

## Valuation Professional

Independent valuation.

## Lender / Financing Professional

Provider of debt.

## ESOP Professional

Specialist in employee-ownership structure.

## Trustee

Fiduciary for the employee-ownership trust, where applicable. Independence requirements make this role structurally different from the others — see §7.

## Financial / Wealth Advisor

Advises the owner on broader objectives rather than the transaction itself.

## Seller-Note Buyer

A counterparty whose participation is scoped to a specific note.

## Closing Provider

Title, escrow, or settlement.

## Broker / M&A Advisor

Where the owner chooses to involve one.

## Board / Advisory Board

Governance participants.

## Other

An explicit escape hatch. A role we have not modelled is a role we record honestly, not one we force into the nearest enum value.

---

# 5. Role Is Per-Transaction, and Roles Compose

Two rules that follow from §2.

**First: a participant may hold more than one role in one transaction.** An individual may be both Management and an Employee. Modelling that as two participants creates two records that must then be reconciled — and they will drift. Model it as one relationship with two roles.

**Second: a participant's role may change during the transaction.** A valuation professional who is engaged for the review phase may become a closing participant. The relationship's **role history** records the change; the relationship itself persists.

The consequence for every consuming engine:

> **Never cache a role.** Ask the relationship, or subscribe to its role-changed event.

A cached role is the failure mode that makes an attorney still appear to be a lender three stages later.

---

# 6. Organization & Contact Hierarchy

Many participants act through organizations, and the organization is often the counterparty rather than the individual.

The engine therefore distinguishes:

* **Organization relationships** — the firm participates in the transaction
* **Individual relationships** — the person participates
* **Acting-for links** — the individual acts for the organization

This matters because:

* A firm may replace the individual handling the matter without changing the relationship
* A firm may have several individuals involved with different roles
* Contact routing must reach the right individual *and* survive their replacement

The rule:

> **The relationship belongs to whoever the transaction is actually dealing with.** Where a firm is the counterparty, the firm is the relationship and the individual is an acting-for link — so replacing the individual does not end the relationship.

---

# 7. Relationship Lifecycle

A relationship has a state, and the state is not the same thing as an identity's status.

```text id="stk7life"
PROPOSED
    │
    ▼
INVITED ──────► DECLINED
    │
    ▼
ACTIVE
    │
    ├──► SUSPENDED ──► ACTIVE
    │
    ▼
ENDED
```

The states carry one load-bearing rule each:

* **Proposed** — the owner is considering involving this party. Nobody has been contacted. Nothing may be shared.
* **Invited** — an invitation exists. **An invitation is not membership.** Identity & Access forbids treating an invitation as active membership, and this engine is where that rule is enforced.
* **Active** — the participant is in the transaction and may be referenced by other engines.
* **Declined** — the invitation was refused. The record is kept; the relationship does not become active.
* **Suspended** — temporarily out of the transaction. Not the same as ended: the relationship, its history, and its role are preserved.
* **Ended** — the participation is over. **The record is not deleted.** An attorney who left the matter three months ago is still the attorney who reviewed the earlier package, and Audit / Provenance must be able to say so.

Ending a relationship is a relationship decision. It is **not** an access revocation, and it does not by itself change what anyone may see — that remains Consent & Access's decision.

---

# 8. Participants Are References, Not Copies

The rule that keeps the graph clean:

> **Exactly one engine stores the participant record. Every other engine stores a reference to it.**

If Communication, Notification, Professional Review and Document Readiness each kept their own participant list, then:

* Ending a relationship would have to be propagated to four places, and would eventually miss one
* A role change would be applied in some places and not others
* "Who is in this transaction?" would have four different answers

So the engine emits, and others consume:

* `RelationshipCreated`
* `RelationshipActivated`
* `RelationshipRoleChanged`
* `RelationshipSuspended`
* `RelationshipEnded`
* `OrganizationLinked`

Every consuming engine that needs participant context holds a reference and resolves it. The reference is stable for the life of the relationship.

---

# 9. What the Engine Does Not Own

Stated explicitly, because each of these is a boundary someone will otherwise cross:

* **Authentication, sessions, and platform roles** — Identity & Access. This engine knows a participant is an attorney; it does not know whether the person logging in is that attorney.
* **The authorization decision** — Consent & Access. Participation never implies access.
* **What each participant may see** — Stakeholder Disclosure. The three visibility levels are that engine's policy vocabulary; this engine records which relationship they attach to, not what they mean.
* **Vendor eligibility, verification and trust** — Vendor Administration. A provider becomes a stakeholder *after* vetting, not instead of it.
* **Curation and matching of providers** — Marketplace.
* **Conversations** — Communication.
* **The transaction, its stages and execution state** — Transaction / Orchestration.
* **Platform-wide rules, including role restrictions** — Policy / Compliance.
* **Documents** — Local Vault and Document Readiness.
* **The historical record of who was added or removed** — Audit / Provenance, which records what happened. This engine owns the *current* relationship state.

The most dangerous of these is the second. **A participant who can see nothing is a valid state**, and it is the correct default. Adding someone to a transaction must never silently grant them access to anything.

---

# 10. Integration

## Identity & Access

Supplies the identity a relationship points at. This engine never creates identities, and never treats possession of a role as proof of qualification.

## Consent & Access

Consumes the relationship when deciding whether a specific disclosure may occur. Participation is a precondition for the decision, never the decision.

## Stakeholder Disclosure

Supplies the visibility level attached to a role. This engine records that a role carries a level; that engine defines what the level means.

## Marketplace and Vendor Administration

Marketplace selects a provider and hands the selected provider to this engine, which creates the stakeholder relationship. Vendor Administration governs eligibility before that point.

## Transaction / Orchestration

Owns the transaction; this engine owns the participants within it. Orchestration asks this engine for the participant set when building role-specific views and the responsibility map.

## Professional Review

Invites a professional into a review. The invitation references a relationship; this engine owns the relationship, Professional Review owns the review.

## Communication and Notification

Both hold participant references. Neither holds participant records. Notification asks this engine who should be told; this engine does not decide when or how.

## Audit / Provenance

Records additions, changes and removals. This engine owns current state; Audit owns the history.

---

# 11. Architectural Boundary Summary

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Stakeholder / Relationship** | Who participates in a transaction and why — the participant roster, each participant's role in this transaction, the organization/contact hierarchy, and relationship state and history | Authentication, the authorization decision, what each participant may see, vendor eligibility, provider curation, and the conversations they take part in |
| **Identity & Access** | Who a person or organization is, and their authentication and session | Why they participate in this particular transaction |
| **Consent & Access** | The authorization decision that permits a specific disclosure | Who participates, or what a disclosure level means |
| **Stakeholder Disclosure** | What each visibility level (Summary / Detailed / Comprehensive) permits | Which relationship holds which level, or the access decision |
| **Vendor Administration / Vetting** | Vendor eligibility, verification and trust | Who participates once a vendor becomes a stakeholder |
| **Marketplace** | Curation and matching of providers | The stakeholder relationship created once a provider is selected |
| **Transaction / Orchestration** | The transaction, its stages, milestones and execution state | The stakeholder relationships within it |
| **Professional Review** | Inviting a professional into a review, and their determinations | The participant record the invitation references |
| **Communication** | The conversations participants take part in | Which participants exist |
| **Notification** | Attention, reminders and delivery channels | Who should be notified — the relationship determines that |
| **Policy / Compliance** | Platform-wide rules, including role restrictions | Whether a specific person holds a role in this transaction |
| **Local Vault** | Private files, encryption, and document storage | Who may be given access to them |
| **Audit / Provenance** | The record of who was added, changed or removed, and when | The current relationship state |

## Hard Boundary

> The Stakeholder / Relationship Engine owns **who participates in a transaction and why**. It does
> not own who a person *is* (Identity & Access), whether they may see something (Consent & Access),
> what a disclosure level means (Stakeholder Disclosure), whether a vendor is eligible (Vendor
> Administration), or the transaction they participate in (Transaction / Orchestration).
> Participation is never a grant of access.

Three consequences follow, and they are the ones to test any future change against:

* Adding a participant grants them **nothing**.
* A participant's role may differ in every transaction, and may change within one.
* Ending a relationship ends the participation, not the record.

---

# 12. Final Principle

Every engine in this platform needs to know who is involved. If each one answers that question for itself, the platform has as many participant lists as it has engines — and they will disagree.

> **One engine owns the relationship. Every other engine holds a reference to it.**

That is what makes "who is in this transaction?" a question with a single answer, and what keeps a role change from having to be applied in five places.
