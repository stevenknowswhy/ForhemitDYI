Yes. This engine is the **trust and governance layer** for the Marketplace Engine.

The Marketplace Engine says:

> **“Who might be useful?”**

The Vendor Administration / Vetting Engine says:

> **“Who do we allow into our marketplace, what have we verified, what are they permitted to claim, and what should happen when something changes?”**

That distinction should remain absolute.

# Vendor Administration / Vetting Engine v1.0

## Marketplace Governance, Vetting & Trust

## 1. Purpose

The Vendor Administration / Vetting Engine is a standalone back-office system responsible for controlling the quality, eligibility, verification, publication, and ongoing status of professionals and organizations participating in the curated marketplace.

Its core question is:

> **"Has this provider met our marketplace requirements, and does the information presented to users remain sufficiently verified and current?"**

The engine manages:

* Applications
* Organizations
* Professionals
* Credential verification
* Experience verification
* Vetting
* Approval
* Publication
* Visibility restrictions
* Suspension
* Re-verification
* Complaints
* Review moderation
* Administrative history

It does not provide professional advice and does not determine whether a professional is suitable for a particular client's circumstances.

---

# 2. CORE PRINCIPLE

The marketplace is:

**Curated**

**Independently vetted**

**Not pay-to-play**

The vendor cannot publish simply because they created an account.

The platform establishes eligibility through a defined vetting process.

---

# 3. SEPARATION FROM MARKETPLACE ENGINE

### Marketplace Engine

Customer-facing discovery:

> Who appears?

> Why do they appear?

> How can the user compare them?

### Vendor Administration / Vetting Engine

Back-office governance:

> Should they appear?

> What claims have we verified?

> What restrictions apply?

> Is the profile current?

> Should access be suspended?

This separation means administrators can change a provider's marketplace status without rewriting the customer-facing marketplace.

---

# 4. PRIMARY OBJECTS

The engine should maintain:

### `VendorApplication`

A provider's request to join the marketplace.

### `OrganizationReview`

Evaluation of the organization.

### `ProfessionalReview`

Evaluation of an individual professional.

### `CredentialRecord`

Evidence supporting a professional credential.

### `ExperienceClaim`

A claim about experience that may require verification.

### `VettingCase`

The complete administrative review.

### `ApprovalRecord`

The decision allowing publication.

### `PublicationRecord`

What was allowed to become public.

### `ReverificationRecord`

Periodic or event-driven re-review.

### `Complaint`

A user or stakeholder concern.

### `ReviewModerationCase`

Administrative handling of ratings/comments.

### `VisibilityAction`

Publish, restrict, hide, suspend, restore, etc.

---

# 5. APPLICATION LIFECYCLE

A provider should move through controlled states:

```text id="npv7q9"
Invited / Discovered
        ↓
Application Started
        ↓
Application Submitted
        ↓
Completeness Review
        ↓
Vetting
        ↓
Additional Information Requested
        ↓
Approved
        ↓
Published
        ↓
Active Monitoring
        ↓
Reverification / Review
        ↓
Renewed
   OR
Restricted / Suspended
   OR
Archived
```

A provider can move backward when new information requires it.

---

# 6. APPLICATION

The Vendor Portal allows the provider to submit an application.

The application should collect only information needed for marketplace evaluation.

Potential fields:

### Organization

* Legal/business name
* Public name
* Website
* Locations
* Contact information
* Organization type
* Services
* Specialties
* Industries
* Transaction sizes

### Individual Professional

* Name
* Role
* Organization
* Professional credentials
* Licenses where applicable
* Experience
* Specialties
* Services
* Geography
* Transaction experience

### Marketplace Participation

* Areas of interest
* Journey types
* Availability

---

# 7. APPLICATION COMPLETENESS

Before human vetting begins, the system checks:

### Complete

Required fields present.

### Incomplete

Information missing.

### Needs Clarification

Information present but ambiguous.

The provider sees exactly what must be corrected.

---

# 8. CREDENTIAL VERIFICATION

Credentials should be independently verified where appropriate.

Potential records:

* Credential type
* Issuing organization
* Jurisdiction
* Credential identifier where appropriate
* Status
* Effective date
* Expiration date
* Source
* Verification date
* Reviewer
* Verification method

The platform should retain evidence of verification.

---

# 9. CREDENTIAL STATUS

Each credential may have:

**Claimed**

**Submitted**

**Verified**

**Unable to Verify**

**Expired**

**Suspended / Invalid where confirmed**

**Pending Reverification**

The public profile should show an appropriate simplified status.

---

# 10. EXPERIENCE VERIFICATION

Experience is often harder to verify than credentials.

The system should distinguish:

### Provider Claim

> "We have completed 25 ESOP transactions."

from:

### Platform Verification

> **Experience claim reviewed**

The platform should record:

* Claim
* Evidence
* Source
* Reviewer
* Verification date
* Confidence/status
* Restrictions

Do not convert an unsupported provider claim into a platform-certified fact.

---

# 11. EXPERIENCE CLAIMS

Examples:

**Employee ownership experience**

**ESOP experience**

**M&A experience**

**Seller-financing experience**

**Healthcare transaction experience**

**$5M–$10M transaction experience**

Each claim can have:

**Verified**

**Partially verified**

**Provider reported**

**Unable to verify**

This is much more precise than a single "verified" badge.

---

# 12. VETTING CASE

Every provider should have a centralized:

## Vetting Case

It contains:

* Application
* Provider identity
* Organization
* Individuals
* Credentials
* Experience claims
* Evidence
* Reviewer notes
* Issues
* Decisions
* Restrictions
* Approval
* Publication
* Reverification schedule

This becomes the administrative case file.

---

# 13. VETTING CHECKLIST

Administrators should have a structured checklist.

Example:

### Organization

✓ Identity established

✓ Website confirmed

✓ Contact confirmed

### Professional

✓ Identity confirmed

✓ Credential checked

✓ Jurisdiction checked

### Experience

✓ Employee ownership experience reviewed

🟡 Transaction volume requires clarification

### Marketplace policy

✓ No known pay-to-play arrangement

### Decision

**Pending**

---

# 14. ADMINISTRATIVE REVIEW

The reviewer should be able to:

**Approve**

**Approve With Restrictions**

**Request More Information**

**Return for Correction**

**Reject**

**Escalate**

These decisions should require a reason.

---

# 15. APPROVAL WITH RESTRICTIONS

This is important.

A provider may be appropriate for:

**Business transactions**

but not yet verified for:

**Employee ownership**

The system should allow:

### Approved

**Business Transactions**

### Restricted

**Employee Ownership**

That restriction flows into the Marketplace Engine.

---

# 16. PUBLIC CLAIM CONTROL

The vendor can enter:

> "We specialize in employee ownership."

But the administrator might classify it:

**Provider-reported**

rather than:

**Platform verified**

The public profile should make that distinction visible.

This prevents the platform from unintentionally endorsing every claim a vendor enters.

---

# 17. PUBLIC VERIFICATION BADGE

Possible statuses:

### Platform Verified

Specific information has been independently reviewed.

### Provider Reported

Information comes from the provider and has not been independently verified.

### Verification Pending

Review is underway.

### Verification Expired

Information requires updating.

The actual badge taxonomy should be simple for users while retaining detailed internal records.

---

# 18. PUBLICATION CONTROL

A provider can be:

**Approved but unpublished**

for example because:

* Profile incomplete
* Category not currently needed
* Geographic capacity unavailable
* Administrative hold

Publication is an explicit administrative action.

---

# 19. VISIBILITY CONTROLS

Administrators should be able to control:

### Global visibility

Public / hidden.

### Journey visibility

Employee Ownership only.

### Specialty visibility

ESOP, M&A, tax, etc.

### Geographic visibility

California only.

### Transaction-size visibility

Certain business-size ranges.

### Individual visibility

Specific professional.

This gives the platform fine control without deleting the underlying provider.

---

# 20. SUSPENSION

Suspension should be a formal state.

Possible reasons:

* Credential issue
* Material complaint
* Misrepresentation
* Expired verification
* Regulatory concern
* Provider request
* Administrative review
* Other policy issue

Suspension should immediately affect Marketplace visibility.

---

# 21. SUSPENSION WORKFLOW

```text id="f2m8lc"
Issue Identified
      ↓
Administrative Review
      ↓
Temporary Restriction if warranted
      ↓
Provider Notified
      ↓
Information / Response
      ↓
Decision
   ┌──┴─────┐
   ▼        ▼
Restore   Suspend
   │        │
   ▼        ▼
Publish   Restricted
```

The engine should preserve every action.

---

# 22. REVERIFICATION

Vetting should be ongoing.

Each provider should have:

**Last Verified**

**Next Review**

**Verification Scope**

**Items Due**

For example:

> Credential expires in 60 days.

This generates a reverification task.

---

# 23. EVENT-DRIVEN REVERIFICATION

Re-review can also be triggered by:

* Credential change
* Material profile change
* User complaint
* Provider status change
* New experience claims
* Significant public information
* Failed response to verification request

Not every event needs immediate suspension.

The policy engine determines the required response.

---

# 24. PROVIDER PROFILE CHANGE WORKFLOW

A vendor changes:

> "We now specialize in ESOP transactions."

The system should not necessarily publish that claim immediately.

It becomes:

**New claim → Review required → Verified or provider-reported → Published according to policy**

That protects the integrity of the curated marketplace.

---

# 25. COMPLAINTS

Users need:

### Report a Concern

Categories might include:

* Misrepresentation
* Professional conduct
* Communication
* Billing concern
* Experience concern
* Credential concern
* Conflict of interest
* Other

The user should be able to describe what happened.

---

# 26. COMPLAINT OBJECT

Contains:

* Complaint ID
* Provider
* Organization
* Reporter
* Related journey
* Related transaction
* Date
* Category
* Description
* Evidence
* Status
* Administrative response
* Provider response
* Resolution
* Appeal/dispute if supported

---

# 27. COMPLAINT WORKFLOW

```text id="g1x4dv"
Complaint Submitted
        ↓
Triage
        ↓
Severity Assessment
        ↓
Information Gathering
        ↓
Provider Response
        ↓
Administrative Review
        ↓
Resolution
```

Possible outcomes:

**No Action**

**Information Corrected**

**Profile Restricted**

**Reverification Required**

**Temporary Suspension**

**Permanent Removal**

These decisions should follow marketplace policy.

---

# 28. REVIEW MODERATION

Ratings and comments must be governed separately from general complaints.

The engine should determine whether a review is:

**Pending**

**Published**

**Reported**

**Under Review**

**Removed**

**Disputed**

This should not allow providers to erase unfavorable reviews simply because they disagree with them.

---

# 29. REVIEW POLICY

The platform should define:

* What reviewers may discuss
* What prohibited content is
* What personal information may appear
* How retaliation complaints are handled
* How providers respond
* How disputed reviews are labeled
* When reviews are removed

The exact policy can evolve without rewriting the Marketplace Engine.

---

# 30. VERIFIED PLATFORM EXPERIENCE

When a user actually worked with a provider through the platform, the review can receive:

### Verified Platform Experience

This means:

> The platform can verify that the interaction occurred.

It does not mean:

> The platform guarantees the professional's performance.

---

# 31. PROVIDER RESPONSE TO REVIEWS

Providers should be able to respond to reviews.

The vendor portal could show:

**Review**

**Respond**

The provider cannot edit the customer's review.

Administrative moderation remains independent.

---

# 32. ADMIN REVIEW QUEUE

The back-office should have:

# Vetting Queue

### New Applications

12

### Credential Reviews

8

### Experience Claims

6

### Profile Changes

14

### Complaints

3

### Review Reports

5

### Reverification Due

11

Each item can have:

**Priority**

**Age**

**Assigned reviewer**

**Status**

---

# 33. ADMIN DASHBOARD

The main dashboard might show:

### Marketplace Health

**Active Providers:** 412

**Verification Current:** 94%

**Reverification Due:** 17

**Complaints Under Review:** 3

**Suspended:** 4

**Applications Pending:** 18

These are administrative operational metrics, not marketplace quality scores.

---

# 34. REVIEWER WORKSPACE

A reviewer should see one provider at a time:

# Smith & Jones LLP

### Marketplace Status

**Active**

### Verification

Organization ✓

Credentials ✓

Employee Ownership Experience 🟡

### Current Claims

[List]

### Evidence

[List]

### Complaints

[2 open / 7 historical]

### Actions

**Approve**

**Approve With Restrictions**

**Request Information**

**Suspend**

**Archive**

---

# 35. EVIDENCE FOR VETTING

Verification evidence should have provenance similar to the Research Evidence Ledger.

However, it should remain a separate domain.

Potential evidence:

* Official credential source
* Provider-supplied evidence
* Public professional registry
* Organization confirmation
* Reference
* Platform transaction history
* Other documented evidence

Each evidence item receives:

**Source**

**Date**

**Reviewer**

**Claim supported**

**Status**

---

# 36. VETTING EVIDENCE VS. RESEARCH EVIDENCE

These remain separate.

### Evidence Ledger

Supports external research and Goal-to-Reality analysis.

### Vetting Evidence

Supports the decision to allow a provider into the marketplace.

They may use similar technical infrastructure but should remain distinct objects.

---

# 37. APPROVAL RECORD

When approved:

## ApprovalRecord

Contains:

* Provider
* Scope
* Reviewer
* Date
* Decision
* Conditions
* Verification evidence
* Next review
* Marketplace categories

This becomes the authoritative administrative record.

---

# 38. REMOVAL

A provider should not simply disappear from history.

If removed:

**Marketplace status → Archived / Removed**

The system preserves:

* Previous profile
* Vetting history
* Complaints
* Reviews
* Dates
* Reason
* Administrative decision

Public visibility may disappear while the internal record remains.

---

# 39. ADMINISTRATIVE APPEAL

The system should consider a provider appeal workflow.

For example:

### Provider disputes suspension

→ Submit response/evidence

→ Independent administrative review

→ Decision

**Restore**

or

**Maintain restriction**

The marketplace should have a defined policy rather than ad hoc decisions.

---

# 40. CONFLICT OF INTEREST RECORD

The engine should track:

* Platform relationships
* Referral arrangements
* Commercial relationships
* Provider disclosures
* Potential conflicts

Any material relationship that could affect marketplace neutrality should trigger appropriate review and disclosure.

This is especially important because our core marketplace principle is:

> **No paid placement.**

---

# 41. NO PAID PLACEMENT RULE

The system should enforce:

```text id="3ip6sm"
Provider Payment
       ≠
Marketplace Eligibility
       ≠
Marketplace Visibility
       ≠
Matching Priority
```

A provider cannot purchase a better position.

This should be technically and administratively enforceable wherever possible.

---

# 42. MARKETPLACE INTEGRATION

When the Vetting Engine changes status, it sends an event.

For example:

```text id="5z4gn4"
ProviderApproved
ProviderRestricted
ProviderSuspended
ProviderReverified
ProviderCredentialExpired
ProviderProfileClaimChanged
ProviderRemoved
```

The Marketplace Engine reacts.

Example:

**ProviderSuspended**

→ Remove from contextual matching.

The Marketplace Engine does not make the suspension decision.

---

# 43. VENDOR PORTAL INTEGRATION

The Vendor Portal allows:

**Edit Profile**

**Submit Credential**

**Add Experience**

**Respond to Review**

**Respond to Complaint**

**Complete Reverification**

The Vetting Engine determines what requires administrative review.

---

# 44. MARKETPLACE PROFILE INTEGRATION

The Marketplace displays approved information.

For example:

### Employee Ownership

**Platform Verified**

### Seller Financing

**Provider Reported**

This distinction should be generated from Vetting status.

---

# 45. ADMINISTRATOR OVERRIDE

Administrators need explicit override capability for exceptional situations.

Any override should require:

* Reason
* Administrator identity
* Date
* Scope
* Duration
* Supporting evidence

This prevents hidden changes.

---

# 46. AUDITABILITY

Every important administrative action should be recorded:

* Application received
* Credential verified
* Claim changed
* Provider approved
* Profile published
* Complaint received
* Suspension applied
* Suspension removed
* Reverification completed
* Review moderated

The Audit Engine can maintain the broader platform record.

---

# 47. VETTING RULES SHOULD BE CONFIGURABLE

The engine should not hard-code everything.

For example:

### Employee Ownership Attorney

Required:

* Legal credential
* Jurisdiction
* Employee ownership experience

### CPA

Required:

* Appropriate credential
* Jurisdiction where applicable
* Financial/business transaction experience

### Seller-Note Buyer

Different requirements.

The platform should be able to configure the vetting checklist by category.

---

# 48. DIFFERENT PROVIDERS, DIFFERENT VETTING

A lawyer and a seller-note buyer should not have identical vetting requirements.

The Vendor Administration Engine should ask:

> **What type of provider is this?**

Then load the appropriate:

**Application**

**Credential requirements**

**Experience requirements**

**Review process**

**Reverification schedule**

**Publication policy**

---

# 49. ORGANIZATION + PROFESSIONAL REVIEW

For organizations with multiple professionals:

The engine should vet:

### Organization

and:

### Individuals

separately.

Example:

**Firm approved**

but:

**Attorney A pending verification**

The firm can remain in the marketplace while Attorney A remains unpublished.

---

# 50. SPECIALTY-SPECIFIC APPROVAL

Likewise:

### Organization

Approved

### Specialty

M&A ✓

### Specialty

Employee Ownership 🟡 Pending

This gives the marketplace granular control.

---

# 51. PROFILE COMPLETENESS VS. VETTING

These must remain separate.

### Profile completeness

Has the provider supplied sufficient information?

### Vetting status

Has the platform independently reviewed the relevant information?

A profile can be:

**100% complete**

but:

**Not yet vetted.**

---

# 52. VERIFICATION FRESHNESS

A verified item should have a freshness state:

**Current**

**Due Soon**

**Expired**

**Needs Reverification**

This applies independently to:

* Credentials
* Experience claims
* Organization information
* Contact information

---

# 53. ADMIN "WHAT CHANGED?" VIEW

When a profile is modified:

### Previous

Employee Ownership Experience: 10 transactions

### New

Employee Ownership Experience: 25 transactions

The administrator sees:

> **Material claim increased from 10 to 25. Review required.**

This prevents important changes from slipping through automatically.

---

# 54. ADMIN WORK QUEUE PRIORITIZATION

The system can prioritize tasks based on:

* Credential expiration
* Complaint severity
* Material profile change
* Provider importance
* Number of affected users
* Time since submission

The platform should make this prioritization explainable.

---

# 55. PROVIDER STATUS MODEL

The provider's overall marketplace status could be:

**Applicant**

**Pending**

**Approved**

**Published**

**Restricted**

**Suspended**

**Archived**

But detailed specialty/credential status exists underneath.

---

# 56. VETTING DECISION MODEL

The system should support:

### Approved

Eligible.

### Approved with restrictions

Eligible only for specified categories.

### Conditional approval

Additional information required after limited approval.

### Deferred

Insufficient information.

### Rejected

Does not meet marketplace criteria.

### Suspended

Previously approved, temporarily restricted.

### Removed

No longer eligible.

---

# 57. PROVIDER NOTIFICATION

The vendor should know:

* Application status
* Missing information
* Verification requests
* Profile changes requiring review
* Approval
* Restrictions
* Suspension
* Reverification due
* Complaint response requests
* Review moderation outcomes

Communication should be factual and specific.

---

# 58. ADMINISTRATOR SECURITY

Because this engine controls marketplace trust, administrative access should be highly restricted.

Potential controls:

* Role-based admin permissions
* Audit logs
* Dual approval for certain actions
* Sensitive evidence restrictions
* Separation of duties
* Administrative authentication
* Session controls

The exact implementation belongs to the broader security architecture.

---

# 59. ENGINE INPUTS

The Vendor Administration / Vetting Engine receives:

**Vendor applications**

**Vendor profile changes**

**Credential submissions**

**Experience claims**

**Complaint reports**

**Review reports**

**Marketplace policy**

**Reverification events**

**Administrative findings**

---

# 60. ENGINE OUTPUTS

It produces:

### Provider Status

Approved, restricted, suspended, etc.

### Verified Claims

What the platform has independently verified.

### Publication Permissions

What may appear publicly.

### Category Eligibility

What journeys/categories the provider can participate in.

### Reverification Requirements

What needs to be checked again.

### Complaint Outcomes

Administrative resolution.

### Review Moderation Status

What user feedback is published.

### Governance Events

Status changes consumed by other engines.

---

# 61. WHAT THIS ENGINE DOES NOT DO

It does not:

* Determine who is the best professional
* Give professional advice
* Guarantee outcomes
* Guarantee provider performance
* Make legal conclusions
* Make tax conclusions
* Make financing decisions
* Provide investment advice
* Replace professional licensing authorities
* Automatically treat provider claims as verified facts

It manages **marketplace eligibility and trust information**.

---

# 62. COMPLETE MARKETPLACE TRUST FLOW

```text id="x8os1o"
PROVIDER
   ↓
APPLICATION
   ↓
CREDENTIALS
   ↓
EXPERIENCE CLAIMS
   ↓
VETTING
   ↓
APPROVAL
   ↓
PUBLICATION
   ↓
USER INTERACTION
   ↓
REVIEWS / COMPLAINTS
   ↓
MONITORING
   ↓
REVERIFICATION
   ↓
CONTINUE / RESTRICT / SUSPEND
```

---

# 63. NORTH STAR

The Vendor Administration / Vetting Engine should ensure that when an owner sees:

> **"These are professionals who may be useful to your journey."**

the platform can support that presentation with:

**A documented curation process**

**Independent verification**

**Current information**

**Transparent provider claims**

**User feedback**

**Ongoing monitoring**

The system doesn't promise perfection.

It creates a **repeatable, auditable process for maintaining marketplace trust**.

---

# 64. ONE-SENTENCE DEFINITION

> **The Vendor Administration / Vetting Engine is the governance system that independently vets, verifies, publishes, monitors, and, when necessary, restricts or removes marketplace participants so the public marketplace remains curated, transparent, current, and independent of paid placement.**

## One distinction I'd make especially explicit

We now have a clean three-part marketplace architecture:

**Vendor Portal**
The professional says: **“Here is who we are.”**

**Vetting Engine**
We say: **“Here is what we've independently verified and what you're permitted to claim in our marketplace.”**

**Marketplace Engine**
The owner sees: **“Here are providers who appear relevant to your journey.”**

That separation is excellent for trust.

I also think **“Verified” should never be a single yes/no property internally**. We should know *what* was verified, *when*, *against what evidence*, and *whether that verification applies to the organization, individual, credential, specialty, or experience claim*. That will become important when the marketplace grows and the profiles start getting complicated.
