The **Consent & Access Engine** should sit between the information itself and every person, professional, service, or system that wants to see it.

Its job is not to store the information, analyze it, or decide whether a transaction is good. Its job is to answer one question, precisely and repeatedly:

> **“Is this actor allowed to access this specific information, for this specific purpose, at this specific time?”**

That makes it particularly important to the local-first architecture.

# Consent & Access Engine

## 1. Purpose

The Consent & Access Engine governs access to business, transaction, professional, financial, employee, and other sensitive information throughout the platform.

It controls:

* Who can see information
* Exactly what they can see
* Why they are being given access
* What level of access they receive
* How long access lasts
* Whether access requires explicit owner consent
* Whether access can be revoked
* Whether access applies to a document, section, field, package, or broader workspace
* Whether information is considered sensitive or highly sensitive
* What happened when information was shared
* Whether access is still valid

The engine provides the **authorization and consent policy layer**.

It does **not** become the document repository, local vault, marketplace, professional-review system, or audit system.

---

# 2. Architectural Principle

## Consent is a separate concern from storage.

A file existing in the user's local vault does not mean another person can access it.

A fact existing in Business Reality does not mean every professional can see it.

A document included in a Professional Review Package does not mean it is automatically shared.

A professional being assigned to a transaction does not grant access to the entire transaction.

Every access request must be evaluated against an explicit access policy.

### Core rule

> **Possession does not equal permission.**

This is especially important for the local-first model.

The user may have:

* payroll files
* tax returns
* bank statements
* customer lists
* employee information
* valuations
* contracts
* legal documents
* financial models
* seller-note information

stored locally without transmitting any of them to the platform.

The platform can know:

> “Document available locally.”

without knowing:

> “Document contents.”

Access is granted only when the owner authorizes the appropriate disclosure.

---

# 3. What This Engine Owns

The Consent & Access Engine owns the concepts of:

### Actor

Who is requesting or receiving access.

Examples:

* Owner
* Co-owner
* Management
* Employee
* Attorney
* CPA
* Valuation professional
* Lender
* Trustee
* Financial advisor
* Buyer
* Seller-note purchaser
* Platform administrator
* Marketplace provider
* Integration
* AI/research service

### Resource

What is being accessed.

Examples:

* Entire transaction
* Business profile
* Specific document
* Document section
* Financial statement
* Specific fact
* Scenario
* Professional Review Package
* Research finding
* Employee information
* Seller-note record
* Financial model
* Communication
* Evidence record

### Purpose

Why access is being granted.

Examples:

* Professional review
* Valuation
* Financing
* Tax analysis
* Legal review
* Employee ownership evaluation
* Seller-note liquidity review
* Transaction execution
* Administrative support
* Research
* Document preparation

### Permission

What the recipient is allowed to do.

Examples:

* View
* Download
* Print/export
* Comment
* Respond
* Add information
* Upload related material
* Share onward
* Modify
* Administrate

The default should be **least privilege**.

### Consent

The owner's authorization for a disclosure or access grant.

### Access Grant

The actual permission resulting from a consent decision or authorized system policy.

### Expiration

When access automatically ends.

### Revocation

The owner's or authorized administrator's withdrawal of access.

### Access Policy

The rules used to determine whether an access request should be allowed.

---

# 4. What This Engine Does NOT Own

This engine must remain independent.

It does not own:

**Local Vault Engine**

* File storage
* Encryption
* Local indexing
* Local document retrieval

**Document Intelligence Engine**

* OCR
* extraction
* classification
* document interpretation

**Business Reality Engine**

* current-state business facts

**Evidence Ledger**

* research-source provenance

**Professional Review Package Engine**

* determining what belongs in a professional package

**Professional Review Engine**

* managing professional assignments and determinations

**Marketplace Engine**

* professional discovery and matching

**Audit / Provenance Engine**

* platform-wide historical logging

**Policy / Compliance Engine**

* platform-wide policy rules

Instead, those systems request or consume authorization decisions from Consent & Access.

---

# 5. Permission Hierarchy

The platform should support several levels of access rather than a single binary permission.

## Level 0: No Access

The actor cannot see that information.

They may see only:

> “Additional information is required.”

without seeing its contents.

## Level 1: Existence Only

The actor knows that a resource exists.

Example:

> “2025 Federal Tax Return is available locally.”

They cannot open it.

This is valuable for workflow coordination without disclosure.

## Level 2: Summary

The actor receives intentionally limited information.

Example:

> “2025 revenue: approximately $8M.”

rather than the complete financial statement.

## Level 3: Detailed

The actor receives additional relevant information.

Example:

* revenue
* EBITDA
* debt
* ownership structure
* approximate transaction assumptions

but not unrelated PII or confidential schedules.

## Level 4: Full View

The authorized actor can view the complete resource.

## Level 5: Download / Export

The actor may create a copy outside the controlled environment.

This should require separate permission from viewing.

## Level 6: Edit

The actor can modify information.

This should be rare and explicitly scoped.

## Level 7: Share / Delegate

The actor can authorize another person to access information.

This should be highly restricted.

**Viewing should never automatically imply downloading, editing, or onward sharing.**

---

# 6. Access Scope

Permissions should be hierarchical and granular.

An access grant may apply to:

* Workspace
* Transaction
* Business
* Scenario
* Professional package
* Document collection
* Individual document
* Document section
* Individual field
* Specific fact
* Specific communication

The platform should support inheritance, but inheritance must always be visible.

Example:

> Attorney Smith
> Access: Legal Review Package
> Includes: 14 documents
> Excludes: Employee medical information, banking credentials, unrelated HR records
> Expires: December 15, 2026

The user should be able to drill down and see exactly what that means.

---

# 7. Sensitive Data Model

Not all information should be treated equally.

Every resource should have a sensitivity classification.

### Standard

Ordinary business information with limited confidentiality concerns.

### Confidential

Information that should not be broadly disclosed.

Examples:

* financial statements
* business contracts
* internal strategy
* pricing information

### Sensitive

Information that requires explicit authorization and tighter controls.

Examples:

* tax information
* employee compensation
* customer information
* banking information
* detailed ownership records

### Highly Sensitive

Information requiring the strongest controls.

Examples could include:

* personally identifiable financial information
* credentials
* highly confidential transaction material
* sensitive employee records
* information whose disclosure could materially harm the business

The system should allow organizations to configure additional classifications.

---

# 8. Sensitive-Data Permissions

Sensitive information should require a separate authorization pathway.

For example:

> **CPA Request**
> Needs access to:
>
> * 2025 tax return
> * 2025 financial statements
> * Owner compensation schedule

The user should see:

**Purpose:** Tax review
**Requested by:** CPA
**Duration:** 30 days
**Sensitivity:** Sensitive
**Access:** View only

Then:

**Approve**

or

**Change what is shared**

or

**Decline**

For highly sensitive information, the platform can require an additional confirmation:

> “This information contains sensitive financial or personal data. Confirm that you authorize access for this purpose.”

---

# 9. Why Was This Shared?

Every external access grant should have a recorded purpose.

The purpose should not be generic whenever a meaningful distinction exists.

Examples:

| Recipient              | Resource              | Purpose              |
| ---------------------- | --------------------- | -------------------- |
| Attorney               | Ownership documents   | Legal review         |
| CPA                    | Tax returns           | Tax analysis         |
| Valuation professional | Financial statements  | Valuation            |
| Lender                 | Financial package     | Financing evaluation |
| Trustee                | Transaction materials | ESOP review          |
| Financial advisor      | Proceeds assumptions  | Retirement planning  |

This becomes extremely useful later when the system asks:

> “Should this person still have access?”

The answer can be evaluated against the original purpose.

---

# 10. Duration

Access must have a lifecycle.

Possible durations:

* One-time access
* 24 hours
* 7 days
* 30 days
* 90 days
* Until a specified date
* Until a milestone
* For the duration of a professional assignment
* Ongoing until revoked

The default should favor **expiration**, particularly for external access.

For example:

> “CPA access expires October 31, 2026.”

The owner can extend it.

The platform should not silently convert expired access into permanent access.

---

# 11. Revocation

Revocation is a first-class operation.

The owner should be able to say:

> **Revoke Access**

and immediately invalidate the authorization.

Possible reasons:

* Professional engagement ended
* Wrong person
* Information no longer needs to be shared
* Transaction changed
* New version replaced old information
* Concern about disclosure
* Access granted accidentally
* Owner changed their mind

The system records:

* Who revoked it
* When
* What was revoked
* Why, when provided
* What permissions were removed

### Important distinction

Revoking access to a platform resource does not necessarily mean a person can be made to “forget” something they already viewed or downloaded.

Therefore the platform must distinguish:

**Platform access revoked**

from

**Previously disclosed information**

That distinction should be explicit.

---

# 12. Version-Specific Sharing

This is critical.

Suppose an owner shares:

> Financial Package v3

with a lender.

Later the owner creates:

> Financial Package v4

The lender should not automatically receive v4 unless the access policy permits it.

The system should show:

> **A newer version is available.**

Then the owner can authorize the new version.

This prevents a previously authorized recipient from receiving continuously changing information without renewed disclosure.

---

# 13. Package-Level Sharing

Because the Professional Review Package Engine already exists, Consent & Access should integrate tightly with it.

The package engine answers:

> “What should be in this professional's package?”

The Consent & Access Engine answers:

> “May this professional receive it?”

Therefore:

**Package creation ≠ sharing.**

A package can be:

* Draft
* Ready for owner review
* Approved for sharing
* Shared
* Partially shared
* Expired
* Revoked

---

# 14. Owner Approval Flow

For external sharing, the preferred flow is:

### Step 1

Professional requests information.

### Step 2

Professional Review Package Engine assembles the proposed package.

### Step 3

Consent & Access evaluates sensitivity and permissions.

### Step 4

Owner sees a clear disclosure screen.

Example:

> **Review Before Sharing**
>
> Attorney Smith is requesting access for legal review.
>
> **14 items included**
>
> 9 documents
> 3 business facts
> 2 scenarios
>
> **3 sensitive items**
>
> Employee compensation schedule
> Tax return
> Ownership records
>
> Access: View
> Duration: 30 days
>
> [Approve] [Change] [Decline]

The user should never have to hunt through a giant permissions console to make ordinary decisions.

---

# 15. Local-First Behavior

This engine is particularly important because sensitive information may remain entirely local.

Consider:

> `/BusinessVault/Tax/2025/Federal_Return.pdf`

The local device may know the file exists.

The cloud application may know:

> “2025 Federal Tax Return available locally.”

But the document itself remains local.

When a CPA needs it:

1. CPA requests access.
2. Platform creates a disclosure request.
3. Owner approves.
4. Consent & Access creates an authorization.
5. Local Vault verifies the authorization.
6. User explicitly releases the authorized document.
7. Only the permitted document/version is transmitted.
8. Access is logged.
9. Authorization expires or is revoked.

This creates a clean boundary:

**Local Vault = possession**

**Consent & Access = permission**

**Sharing mechanism = transmission**

---

# 16. No Silent Cloud Escape Hatch

The platform must never do this:

> “The AI needs the document, so we uploaded it automatically.”

Instead:

> **Information remains local unless a defined policy and authorization permit transmission.**

This applies not only to humans but also to:

* AI services
* research agents
* external integrations
* document processors
* cloud storage
* third-party APIs

An AI system should be treated as an actor.

For example:

> Research Agent
> Resource: Anonymous industry financial benchmark
> Permission: View
> Purpose: Market research
> PII permitted: No
> Expiration: Research session

---

# 17. Data Minimization

The engine should support **minimum necessary disclosure**.

The question should not be:

> “Should I give the CPA the entire business?”

It should be:

> “What does the CPA need for this specific task?”

For example, a lender may need:

* revenue
* EBITDA
* debt
* cash flow
* collateral information

but not:

* employee home addresses
* customer personal information
* unrelated legal correspondence

The platform should actively identify potentially unnecessary disclosures.

It should never silently remove a requested item, but it can explain:

> “This item appears unrelated to the stated purpose.”

---

# 18. Owner-Controlled Nonnegotiables

The owner's existing **Nonnegotiable** mechanism must integrate into access policy.

Example:

> **Nonnegotiable:** Employee identities must not be disclosed until employees have been formally notified.

If a proposed disclosure conflicts with that requirement, the platform must stop and show the conflict.

It should not override the owner's requirement because a professional requests the information.

---

# 19. Access Requests

Access should generally flow through a formal request.

A request contains:

* Requestor
* Organization
* Role
* Resource requested
* Purpose
* Requested permission level
* Requested duration
* Sensitivity
* Optional explanation
* Related journey stage
* Related professional assignment
* Related transaction/scenario

Example:

> **Lender Request**
>
> Requested by: ABC Bank
> Purpose: Acquisition financing evaluation
> Resources: 2024–2025 financial statements, debt schedule, cash-flow model
> Access: View
> Duration requested: 30 days

The owner can approve all, approve some, modify, or decline.

---

# 20. Multiple Owners / Authorized Representatives

The system should support businesses with:

* One owner
* Multiple owners
* Trusts or entities
* Authorized representatives
* Management teams

Access rights must distinguish between:

**Identity**

and

**Authority**

Someone being listed as an owner is not sufficient to infer unrestricted permission in every context.

The engine should rely on the Identity & Access Engine and Policy / Compliance Engine for formal authority rules.

---

# 21. Access Conflict Rules

When policies conflict, the engine should fail safely.

Examples:

### Too Sensitive

> Access requested, but information is classified Highly Sensitive and requires explicit owner authorization.

### Expired

> This access grant expired on September 1, 2026.

### Wrong Purpose

> Existing access was granted for valuation review. Requested use is financing.

### Wrong Resource

> Professional has access to Financial Package v3 but not v4.

### Nonnegotiable Conflict

> Requested disclosure conflicts with owner requirement.

### Missing Professional Assignment

> Professional has not been assigned to a role permitting this information.

The system should never silently broaden access.

---

# 22. Access States

An access grant can move through:

**Requested → Pending Owner Approval → Approved → Active → Expiring → Expired**

or:

**Requested → Declined**

or:

**Active → Revoked**

or:

**Active → Suspended**

The underlying consent record remains historically intact.

Revocation changes current authorization.

It does not erase history.

---

# 23. Core Data Objects

## AccessRequest

Contains:

* request_id
* requester_id
* organization_id
* role
* purpose
* requested_resources
* requested_permissions
* requested_duration
* sensitivity
* related_transaction
* related_package
* status
* created_at

## ConsentRecord

Contains:

* consent_id
* consenting_party
* scope
* purpose
* permissions
* resources
* sensitivity
* start_time
* expiration_time
* consent_method
* consent_timestamp
* version
* status
* revocation information

## AccessGrant

Contains:

* grant_id
* actor
* resource
* permission
* scope
* purpose
* effective_from
* expires_at
* source_consent
* policy_basis
* status

## DisclosureRecord

Records the actual sharing event:

* what was disclosed
* which version
* to whom
* when
* purpose
* permission level
* transmission method
* authorization used

## AccessPolicy

Defines rules for:

* sensitivity
* role
* resource
* purpose
* duration
* geographic or organizational limits
* required approval
* prohibited combinations

---

# 24. Relationship to Audit / Provenance

The Consent & Access Engine records **authorization facts**.

The Audit / Provenance Engine records **system history**.

For example:

Consent Engine:

> Owner granted Attorney Smith view access to Ownership Package v2 until October 31.

Audit Engine:

> Stephen changed access from “30 days” to “60 days” at 2:14 PM, replacing the previous policy.

Disclosure Record:

> Ownership Package v2 was actually transmitted to Attorney Smith at 2:17 PM.

These are related but not the same thing.

---

# 25. Relationship to Professional Review

The workflow becomes:

**Professional requests information**

↓

**Professional Review Package Engine assembles relevant information**

↓

**Consent & Access determines what may be shared**

↓

**Owner approves disclosure**

↓

**Local Vault releases authorized resources**

↓

**Disclosure occurs**

↓

**Audit records the transaction**

This separation keeps each engine clean.

---

# 26. Relationship to Document Readiness

When a professional requests a document:

Document Readiness Engine records:

> “2025 Tax Return requested by CPA.”

Consent & Access determines:

> “May CPA receive it?”

Document Readiness tracks the workflow.

Consent & Access controls disclosure.

Neither replaces the other.

---

# 27. Relationship to Marketplace

A marketplace relationship does **not** equal access.

Finding:

> “Potential ESOP attorney”

does not grant access to anything.

Even selecting:

> “Use this attorney”

does not grant access.

Access begins only after the appropriate professional relationship, request, purpose, and authorization exist.

---

# 28. User Interface

The everyday interface should be extremely simple.

The user should frequently see something like:

### Who needs this?

**CPA**

### Why?

**Tax review**

### What will they see?

**3 documents + 2 financial facts**

### How long?

**30 days**

### Sensitive information?

**Yes, 1 item**

### Access

**View only**

Then:

**[Approve]**

**[Change]**

**[Decline]**

Advanced users can open:

> **Review access details**

to see field-level permissions, exclusions, inherited permissions, policies, and history.

---

# 29. Persistent Access Center

The platform should also provide an **Access Center**.

A dashboard could show:

| Person / Organization | Purpose      | Access         | Expires | Status   |
| --------------------- | ------------ | -------------- | ------- | -------- |
| Attorney Smith        | Legal review | View           | Oct 31  | Active   |
| CPA Jones             | Tax review   | View           | Nov 15  | Active   |
| ABC Bank              | Financing    | View           | Oct 10  | Active   |
| Valuation Co.         | Valuation    | View + Comment | Sep 30  | Expiring |

The most important action should be:

> **Revoke**

The user should not have to navigate through five screens to remove access.

---

# 30. “What Has Been Shared?”

A particularly valuable owner-facing feature is a disclosure history.

The user can ask:

> **What have I shared?**

and see:

> Attorney Smith received Ownership Package v2 on September 12 for legal review.

> ABC Bank received Financial Package v3 on September 14 for financing evaluation.

> Valuation Co. received Financial Statements 2024–2025 on September 16 for valuation.

This gives the owner a clear picture of their disclosure footprint.

---

# 31. “Who Can See This?”

The inverse view is equally important.

From any document:

> **Who can see this document?**

The platform displays:

* Owner
* CPA
* Attorney
* Lender
* Nobody else

Along with:

* permission
* purpose
* expiration
* source of authorization

This makes permission understandable rather than mysterious.

---

# 32. “Why Can They See This?”

Every permission should be explainable.

For example:

> **ABC Bank can view this document because:**
>
> You approved access on September 14, 2026.
>
> Purpose: Acquisition financing evaluation.
>
> Permission: View only.
>
> Expiration: October 14, 2026.
>
> Source: Financial Review Package v4.

This is the **explainability requirement for authorization**.

---

# 33. No Permission Inference

The engine should never infer permission from weak signals.

For example:

* “They are my CPA.”
* “They are on the team.”
* “They were copied on an email.”
* “They saw an earlier package.”
* “They work for the same firm.”
* “They have access to the business.”
* “They reviewed another scenario.”

None of these should automatically create unrestricted access.

Permission should come from a defined authorization path.

---

# 34. Access to Facts vs Access to Source Documents

This distinction is important.

An attorney may be allowed to see:

> Owner-reported revenue: $8.2M

without being allowed to open:

> 2025 General Ledger.xlsx

Similarly, a lender might receive:

> EBITDA: $1.4M

without receiving the underlying payroll records.

Therefore the engine should support permissions on both:

**Source material**

and

**Derived platform facts**

with provenance preserved.

---

# 35. Research and AI Access

Research should follow the same model.

The Research Engine should not be allowed to rummage through the owner's vault.

Instead:

> Research request created.

↓

> Required information identified.

↓

> Consent & Access evaluates requested data.

↓

> Owner approves permitted inputs.

↓

> Research agent receives only authorized information.

This makes the platform's AI architecture fundamentally different from:

> “Upload your entire business to our AI.”

The default philosophy becomes:

> **Give the AI only what it needs, for only as long as it needs it.**

---

# 36. External Sharing Guardrails

Before external disclosure, the system should check:

* Resource exists
* Correct version
* Recipient identity verified
* Professional/organization relationship exists where applicable
* Purpose exists
* Sensitivity rules satisfied
* Owner consent satisfied
* Nonnegotiables satisfied
* Expiration exists
* Required professional review conditions satisfied
* Required policy acknowledgements satisfied

Only then should the transmission layer proceed.

---

# 37. Local-First Security Model

The architecture should ultimately allow a user to operate with:

### Local information

Fully controlled by the user's machine.

### Cloud metadata

Minimal information needed for workflow.

### Authorized cloud disclosure

Only specifically approved information.

### Temporary external access

Time-limited and purpose-limited.

This creates a three-zone architecture:

**Private Local Vault**

↓

**Controlled Platform Metadata**

↓

**Explicitly Authorized External Disclosure**

That should become one of the defining architectural characteristics of the product.

---

# 38. Security Philosophy

The engine should follow these principles:

### Least privilege

Give the smallest amount of access necessary.

### Explicit consent

Sensitive external disclosure requires affirmative authorization.

### Purpose limitation

Access must be associated with a reason.

### Time limitation

Access should normally expire.

### Version limitation

Approval applies to defined information/version unless explicitly broadened.

### Revocability

Access can be withdrawn.

### Explainability

Every access decision should be understandable.

### Separation of duties

No single engine controls storage, permission, professional determination, and transaction execution.

### No silent escalation

Permissions must never broaden without an explicit policy or authorization event.

---

# 39. Important Boundary: Revocation vs Copying

The system should clearly state that:

> Revoking platform access stops further platform-controlled access.

It cannot guarantee deletion of copies already downloaded, printed, photographed, emailed, or otherwise retained outside platform control.

That should be represented honestly rather than implying the system can magically retract information from the physical world.

---

# 40. Initial Engine Contract

Every system that wants to retrieve protected information should effectively ask:

```text
Can actor A
perform action P
on resource R
for purpose U
at time T
under context C?
```

The Consent & Access Engine returns:

```text
ALLOW
ALLOW_WITH_RESTRICTIONS
REQUIRE_CONSENT
DENY
EXPIRED
REVOKED
POLICY_CONFLICT
```

along with an explanation and the applicable authorization.

This is the engine's central contract with the rest of the platform.

---

# 41. Example End-to-End Scenario

An ESOP attorney requests:

* ownership information
* organizational documents
* financial statements
* employee ownership-related information

The system identifies:

**Owner Objective:** Sell the company to employees.

**Professional Purpose:** Legal review.

**Package:** Legal Review Package v3.

**Sensitive Items:** 4.

The owner sees:

> **Attorney Request**
>
> Legal review of proposed employee ownership structure.
>
> **Included:** 17 items
> **Sensitive:** 4
> **Access:** View only
> **Duration:** 30 days
>
> [Approve] [Change] [Decline]

The owner removes one employee file.

The Consent Engine creates a grant covering the remaining 16 items.

The Local Vault releases only those authorized resources.

The disclosure is recorded.

Thirty days later:

> **Access expired**
>
> Attorney Smith no longer has access to Legal Review Package v3.

Nothing else in the owner's vault changes.

That is exactly the separation we want.

---

# 42. Architectural Boundary Summary

| Engine | Owns | Does Not Own |
| --- | --- | --- |
| **Consent & Access** | Who may see which resource, for what purpose, and for how long — actors, resources, permissions, access scopes, grants, expiration, revocation, and the explainability of every authorization decision | The resource itself, the identity of the actor, or the rules that make a request eligible |
| **Identity & Access** | Who a party is and what they may do on the platform | Whether a specific disclosure is permitted for a specific purpose |
| **Policy / Compliance** | The rules used to determine whether an access request should be allowed | The access decision itself |
| **Local Vault** | File storage, encryption, local indexing, and local document retrieval | Who may receive a document |
| **Document Intelligence** | OCR, extraction, classification, and document interpretation | Who may see an extracted fact |
| **Business Reality** | Current business facts and financial statements | Which facts a given actor may see |
| **Evidence Ledger** | Research-source provenance | Disclosure of research evidence |
| **Review Package** | Determining what belongs in a professional package | Who the package may be shared with |
| **Professional Review** | Managing professional assignments and determinations | The access a professional holds |
| **Document Readiness** | Document status, completeness, and outstanding requests | Whether an outstanding document may be disclosed |
| **Marketplace** | Professional discovery and matching | A professional's access to transaction information |
| **Audit / Provenance** | Platform-wide historical logging | Current authorization |
| **Transaction / Orchestration** | Transaction stages and current execution state | Who may observe them |
| **Workflow** | Tasks, dependencies, and execution mechanics | Whether an assignee may see the underlying document |
| **Communication** | Conversations and messages | Who may take part in a conversation |
| **Notification** | Delivery of attention, including follow-up alerts | What the recipient is permitted to see |
| **Stakeholder / Relationship** | Who participates in the transaction and what they may see | The authorization decision that permits it |

## Hard Boundary

The Consent & Access Engine owns the authorization decision: who may see which resource, for what purpose, and for how long. It does not own the resource, the identity of the actor, or the rules that make a request eligible.

**Storage and permission are separate. Local possession never implies external access, and no engine is allowed to silently broaden another engine's permissions.** Every external access grant carries a purpose and an expiration, every authorization decision is explainable, and revocation stops controlled access without falsely promising retrieval of copies already disclosed.

---

# 43. What We Should Lock In

The following should be treated as architectural requirements:

**1. Consent & Access is a standalone engine.**

**2. Storage and permission are separate.**

**3. Local possession never implies external access.**

**4. Every external access grant has a purpose.**

**5. Sensitive information requires stronger authorization.**

**6. Access is granular.**

**7. Viewing, downloading, editing, and onward sharing are separate permissions.**

**8. Access normally has an expiration.**

**9. Access can be revoked.**

**10. Version-specific disclosure is supported.**

**11. Package creation does not equal sharing.**

**12. Professional assignment does not equal unrestricted access.**

**13. AI systems and integrations are treated as actors subject to permission rules.**

**14. Nonnegotiables can block disclosures.**

**15. Every authorization decision is explainable.**

**16. Revocation stops controlled access but does not falsely promise retrieval of copies already disclosed.**

**17. The owner remains the primary authority over their information, subject to explicit organizational and policy rules.**

**18. No engine is allowed to silently broaden another engine's permissions.**

---

## The Conceptual Model

The cleanest way to think about the entire architecture is:

> **Destination decides where the owner wants to go.**
> **Business Reality decides what is known about where they are.**
> **Professional Review determines what qualified professionals conclude.**
> **Professional Review Packages determine what information is relevant to each professional.**
> **Consent & Access determines who is actually allowed to see that information.**
> **Local Vault determines where the private information physically resides.**
> **Audit records what happened.**

That gives us a strong security spine for the whole platform:

**Know → Package → Authorize → Disclose → Review → Revoke**

rather than:

**Upload Everything → Hope the Right People See the Right Things.**

This engine is now conceptually distinct from **Local Vault**, **Professional Review Package**, **Document Readiness**, and **Audit**, which is exactly the separation we need for the broader architecture.
