Yes. This is one of the foundational infrastructure engines because almost every other engine needs to know **who someone is, what organization they belong to, what role they have, and what authority they possess**.

The most important distinction is:

> **Identity & Access answers “Who are you, what organizational context are you operating in, and what platform authority do you have?”**

It does **not** decide whether you should be allowed to see a particular sensitive document. That remains the job of **Consent & Access**.

# Identity & Access Engine

## 1. Purpose

The Identity & Access Engine provides the core identity infrastructure for the platform.

It manages:

* Users
* Organizations
* Roles
* Teams
* Invitations
* Authentication
* Sessions
* Platform permissions
* Membership
* Identity verification state
* Service identities
* Access credentials
* Account lifecycle

Its central question is:

> **“Who is this actor, what organization and team context are they operating within, and what platform-level authority do they have?”**

This engine establishes identity and platform authority for every other engine.

---

# 2. Core Architectural Principle

## Identity is not authorization to specific information.

This distinction should be locked.

### Identity & Access

> Jane Smith is a CPA.

> Jane belongs to ABC Advisory.

> Jane is a Professional on Transaction TX-004.

> Jane has permission to participate in Professional Review.

### Consent & Access

> Jane may view these three financial documents for tax review until October 31.

Those are different questions.

Identity establishes:

> **Who Jane is and what she is allowed to do within the platform.**

Consent establishes:

> **What Jane is allowed to access in this specific context.**

---

# 3. What This Engine Owns

The engine owns:

* User identities
* Organization identities
* Organization membership
* Roles
* Team membership
* Invitations
* Authentication
* Sessions
* Account recovery
* Platform-level permissions
* Role assignments
* Service accounts
* API credentials
* Identity verification state
* Account activation/deactivation
* Organization relationships
* Identity lifecycle
* Permission inheritance

---

# 4. What It Does Not Own

It does not own:

* Specific document disclosure
* Sensitive-data sharing
* Transaction decisions
* Professional determinations
* Workflow logic
* Communication permissions
* Vendor vetting
* Credential verification itself
* Business facts
* Document storage

Those are owned by other engines.

---

# 5. User

The fundamental identity object is the **User**.

A User represents a person who can authenticate to the platform.

A User contains:

* User ID
* Name
* Contact information
* Authentication identifiers
* Account status
* Identity verification status
* Locale
* Time zone
* Preferences reference
* Created date
* Last activity
* Security state

The platform should avoid storing unnecessary personal information.

---

# 6. User vs Person

A useful distinction:

### Person

The real-world individual.

### User

The platform identity representing that individual.

A person may eventually interact with multiple organizations while retaining one platform identity.

Example:

> Jane Smith

can participate as:

* CPA at ABC Advisory
* Advisor to a separate business
* Member of a professional team

The system should avoid creating three unrelated user identities for the same person where one identity can legitimately serve all contexts.

---

# 7. Organization

Organizations are first-class objects.

Examples:

* Business being sold
* Law firm
* CPA firm
* Valuation company
* Bank
* Trustee organization
* Marketplace provider
* Platform customer
* Holding company

An Organization contains:

* Organization ID
* Name
* Organization type
* Contact information
* Status
* Verification state
* Parent organization where applicable
* Members
* Teams
* Roles
* Settings

---

# 8. Organization vs Business

Not every organization is a business being transitioned.

The platform may have:

> **Business Organization**

and:

> **Professional Organization**

and:

> **Platform Organization**

These should be distinct organizational types.

A business owner may belong to:

> Business A

while a CPA belongs to:

> CPA Firm B

Both participate in the same transaction.

---

# 9. Organization Membership

Membership establishes:

> This user belongs to this organization.

Example:

> Jane Smith
> Member of ABC Advisory
> Role: CPA

Membership should contain:

* User
* Organization
* Membership role
* Status
* Start date
* End date
* Invited by
* Verification status

Membership can be:

**Invited**

**Active**

**Suspended**

**Revoked**

**Expired**

---

# 10. Multiple Organizations

A user may belong to multiple organizations.

Example:

> Owner personally

and:

> Holding Company

and:

> Business Entity

A professional may belong to:

> Law Firm A

and participate in:

> Transaction Team B.

The platform identity remains one user while organizational context changes by activity.

---

# 11. Organizational Context

Every important action should know:

> **In what organizational context did this actor act?**

Example:

> Jane Smith acting as CPA at ABC Advisory.

This prevents ambiguity in:

* Communication
* Professional Review
* Marketplace
* Transaction participation
* Audit
* Billing
* Permissions

---

# 12. Roles

A **Role** represents a category of platform responsibility.

Examples:

* Owner
* Co-owner
* Administrator
* Professional
* CPA
* Attorney
* Valuation Professional
* Lender
* Trustee
* Employee
* Management
* Buyer
* Seller-note Purchaser
* Vendor Administrator
* Platform Administrator

Roles should be configurable and extensible.

---

# 13. Role vs Specialty

These should remain different.

### Role

> What the person is doing in the platform.

### Specialty

> What professional capability the person or organization has.

Example:

> Jane = Professional

Specialties:

> M&A
> ESOP
> Tax

Marketplace and Vendor Administration manage specialty information.

Identity & Access manages platform role.

---

# 14. Role vs Credential

Also distinct.

### Role

> Person is acting as a valuation professional.

### Credential

> Person has a particular credential.

### Credential Verification

> Platform verified the credential.

Policy determines what credentials may be required.

Vendor Administration manages verification.

Identity records the resulting identity/role context.

---

# 15. Role Assignment

A user may have different roles in different contexts.

Example:

> Jane

At ABC Advisory:

> CPA

On Transaction TX-004:

> Assigned Professional

In another workspace:

> Advisor

Roles therefore need scope.

---

# 16. Role Scope

A role may apply to:

* Platform
* Organization
* Workspace
* Transaction
* Team
* Specific project

Example:

> Owner at Business A

does not automatically make that person:

> Owner of Business B.

This avoids broad, accidental authority.

---

# 17. Teams

Teams group people who work together.

Examples:

* Seller Team
* Legal Team
* Tax Team
* Financing Team
* Valuation Team
* Employee Transition Team
* Platform Administration Team

A team contains:

* Team ID
* Organization
* Name
* Members
* Team role
* Status
* Scope

---

# 18. Team-Based Permissions

Some permissions can be assigned to teams.

Example:

> Legal Team may participate in legal-review workflows.

But team membership should not automatically grant access to all underlying documents.

That remains subject to:

> Consent & Access.

Team membership establishes operational participation.

---

# 19. Invitations

Users should be invited into organizational and transaction contexts.

Invitation contains:

* Invitation ID
* Inviter
* Invitee
* Organization
* Proposed role
* Proposed team
* Scope
* Expiration
* Status
* Created date
* Acceptance date

States:

**Pending**

**Accepted**

**Declined**

**Expired**

**Revoked**

---

# 20. Invitation Security

Invitations should:

* Expire
* Be single-use where appropriate
* Be tied to intended identity/contact
* Require authentication
* Avoid granting permissions before acceptance
* Be auditable

An invitation is not the same thing as membership.

---

# 21. Invitation Flow

Example:

> Owner invites CPA Jane.

↓

> Invitation created.

↓

> Jane authenticates.

↓

> Jane accepts.

↓

> Membership created.

↓

> Role assigned.

↓

> Professional relationship established.

↓

> Professional Review may now assign relevant work.

Consent & Access still controls sensitive information.

---

# 22. Authentication

Authentication answers:

> **Is this user actually the identity they claim to be?**

The engine should support:

* Passwordless authentication
* Password authentication where appropriate
* Multi-factor authentication
* Federated login
* SSO
* Recovery
* Session management
* Device/session controls

The specific authentication provider should remain replaceable.

---

# 23. Authentication vs Identity Verification

These are not the same.

### Authentication

> The person successfully signed into Jane Smith's account.

### Identity Verification

> The platform has verified additional identity information.

A successful login does not automatically establish professional credentials or legal authority.

---

# 24. Identity Assurance

The system should track identity confidence/status separately.

Examples:

**Basic account**

**Email verified**

**Phone verified**

**Identity verified**

**Organization verified**

**Professional identity verified**

These should not be conflated.

---

# 25. Authentication Factors

The platform should support appropriate authentication factors such as:

* Password
* Passkey
* Authenticator
* Security key
* Verified email
* Verified phone

Higher-risk actions may require stronger authentication according to Policy.

---

# 26. Reauthentication

Certain sensitive actions may require the user to authenticate again.

Examples:

* Changing security credentials
* Adding a new device
* Changing high-level permissions
* Approving highly sensitive disclosures
* Managing organization administrators

Policy determines when this is required.

Identity performs the authentication.

---

# 27. Sessions

The engine manages:

* Session creation
* Session expiration
* Session revocation
* Device association
* Last activity
* Authentication strength
* Refresh/re-authentication

Users should be able to see:

> **Where am I signed in?**

Example:

> MacBook Pro
> Current

> Office Desktop
> Active 2 hours ago

> Unknown Device
> Active yesterday

---

# 28. Device Management

The engine should support trusted/revoked device states.

The Local Vault manages the private local environment.

Identity & Access manages:

> Whether the device/session is still an authenticated platform identity.

The two systems work together.

---

# 29. Service Identities

Not every actor is a human.

Examples:

* Workflow service
* Research agent
* Notification service
* Integration
* AI processing service

These should have explicit service identities.

Never represent them as:

> “System user.”

when knowing the specific service matters.

---

# 30. AI Identities

AI agents should have their own identities where they interact with platform systems.

Example:

> Research Agent v3

or:

> Document Analysis Service

That allows Audit to record:

> AI service created candidate fact.

rather than:

> System changed business reality.

---

# 31. Service Permissions

Service identities should receive only the permissions they require.

Example:

> Notification service may create notifications.

It should not automatically:

> Read every document.

This is least privilege applied to software agents.

---

# 32. API Credentials

The engine may support:

* API tokens
* OAuth clients
* Service credentials
* Integration identities

Credentials should be:

* Scoped
* Expirable where appropriate
* Revocable
* Rotatable
* Auditable

The platform should never treat an API credential as unlimited access.

---

# 33. Permissions

Permissions define:

> **What an identity can do within the platform.**

Examples:

* View
* Create
* Edit
* Approve
* Assign
* Administer
* Export
* Invite
* Manage users
* Manage teams
* Manage policies

Permissions are platform-level capabilities.

Resource-specific data disclosure is handled separately.

---

# 34. Permission Model

A useful structure is:

```text id="q9z5ta"
USER
  ↓
ORGANIZATION MEMBERSHIP
  ↓
ROLE
  ↓
TEAM
  ↓
PERMISSION
  ↓
CONTEXT
```

Then:

```text id="a4m7kc"
SPECIFIC RESOURCE
        ↓
Consent & Access
        ↓
Actual disclosure/access
```

---

# 35. Role-Based Access Control

The initial model should support RBAC:

> Role → Permissions

Example:

**Professional**

may:

* Participate in assigned review
* Respond to questions
* Complete assigned professional tasks

but cannot automatically:

* Access all business documents
* Manage owner permissions
* Invite arbitrary users

---

# 36. Attribute-Based Rules

As the product grows, some permissions will depend on context.

Examples:

> Professional + assigned to transaction + legal role.

or:

> Owner + current business + authenticated strongly.

This suggests eventually supporting attribute-based access alongside RBAC.

---

# 37. Permission Scope

Permissions should have scope.

Examples:

> `manage_users` at Organization A.

> `view_professional_review` at Transaction TX-004.

> `edit_vendor_profile` at Provider Organization B.

This prevents global permissions from becoming the default.

---

# 38. Permission Inheritance

Organizations and teams may inherit permissions.

But inherited permissions should be explainable.

Example:

> Jane can manage this team because she is an organization administrator.

The system should show the permission path.

---

# 39. No Silent Permission Escalation

A critical requirement:

> Adding someone to a team should not unexpectedly grant unrelated high-level permissions.

Permission inheritance must be explicit.

---

# 40. Delegation

The engine should eventually support controlled delegation.

Example:

> Owner temporarily delegates transaction administration to an authorized representative.

Delegation should have:

* Scope
* Start date
* End date
* Permissions
* Grantor
* Recipient
* Revocation

It should never become permanent by accident.

---

# 41. Temporary Roles

Roles may be temporary.

Example:

> Temporary Transaction Coordinator

valid:

> September 19–December 31.

Workflow can generate reminders before expiration.

Notification can alert the owner.

Audit records the assignment.

---

# 42. Relationship to Consent & Access

This distinction should be repeated because it is fundamental.

Identity says:

> **Jane is allowed to participate as a CPA on this transaction.**

Consent says:

> **Jane may view Financial Statements v4 for tax review until October 31.**

Therefore:

> **Role eligibility does not equal resource access.**

---

# 43. Relationship to Policy / Compliance

Policy may say:

> Only users with a designated role can perform this action.

Identity provides:

> Current role = Professional.

Policy evaluates:

> Requirement satisfied.

The action then proceeds.

Identity does not make the policy decision.

---

# 44. Relationship to Marketplace

Marketplace says:

> This organization offers ESOP services.

Identity says:

> Jane is a member of ABC Advisory and has the professional role assigned to her.

Vendor Administration says:

> ABC Advisory is approved/published.

Those are distinct facts.

---

# 45. Relationship to Professional Review

Professional Review creates:

> Jane assigned to legal review.

Identity confirms:

> Jane is an active user and member of ABC Advisory.

Policy may check:

> Required professional role/credential conditions.

Consent determines:

> What Jane can actually see.

---

# 46. Relationship to Communication

Communication uses Identity to determine:

> Who the participants are.

Consent determines:

> Which protected conversations they may access.

Communication manages:

> What they say.

---

# 47. Relationship to Workflow

Workflow uses Identity to determine:

> Who may be assigned a task.

Example:

> Only the assigned owner can complete this task.

Workflow controls task execution.

Identity supplies the actor and role.

---

# 48. Relationship to Transaction Orchestration

Transaction may say:

> Task assigned to valuation professional.

Identity confirms:

> Which user holds that participant identity.

Transaction remains responsible for transaction state.

---

# 49. Relationship to Audit

Every meaningful action should reference the authenticated actor.

Example:

> Jane Smith / ABC Advisory / Professional / Transaction TX-004.

Audit can therefore answer:

> Who performed this action?

Identity supplies the identity context.

---

# 50. Relationship to Notification

Notification needs to know:

> Where should this alert be delivered?

Identity supplies:

* User
* Email
* Device
* Notification preferences reference
* Organizational context

Notification manages the actual delivery.

---

# 51. Account Lifecycle

Users need lifecycle states:

**Invited**

**Pending Activation**

**Active**

**Suspended**

**Locked**

**Deactivated**

**Deleted / Anonymized where policy permits**

The platform should preserve historical identity references where needed for audit.

---

# 52. Organization Lifecycle

Organizations likewise:

**Draft**

**Pending Verification**

**Active**

**Restricted**

**Suspended**

**Archived**

This supports vendor and professional organizations without placing vendor governance into Identity.

---

# 53. Offboarding

When a user leaves an organization:

1. Membership becomes inactive.
2. Organization permissions are removed.
3. Transaction assignments are reviewed.
4. Communication access is evaluated.
5. Consent access is separately evaluated.
6. Open tasks are reassigned where necessary.
7. Audit history remains intact.

This should not require deleting the person's entire historical identity.

---

# 54. Deactivation vs Deletion

These must be separate.

### Deactivation

User can no longer authenticate.

Historical activity remains.

### Deletion

Account information is removed where policy allows.

Historical references may need controlled preservation.

The platform should avoid destroying audit integrity by deleting historical actor references indiscriminately.

---

# 55. Account Recovery

Recovery should support:

* Credential recovery
* MFA recovery
* Passkey recovery
* Device replacement
* Identity verification
* Recovery contacts where appropriate

Recovery procedures should be auditable.

---

# 56. High-Risk Account Changes

Some account actions deserve stronger controls.

Examples:

* Changing organization administrator
* Removing another administrator
* Changing authentication factors
* Adding a new trusted device
* Resetting credentials
* Granting high-privilege role

Policy determines when additional authentication or approval is required.

---

# 57. Organization Administration

Organization administrators may manage:

* Members
* Teams
* Roles
* Invitations
* Organization settings

But administrator authority should still have explicit boundaries.

A professional-firm administrator should not automatically have authority to read every client's business information.

---

# 58. Tenant Isolation

The system should maintain strong logical separation among organizations and workspaces.

A user's membership in Organization A must not accidentally expose Organization B data.

Cross-organization transaction participation must be deliberate.

---

# 59. Cross-Organization Transactions

This is particularly important for the platform.

Example:

```text id="w2f3mg"
Business Owner
Organization A
       │
       ▼
Transaction TX-004
       │
 ┌─────┼───────────┐
 ▼     ▼           ▼
Law    CPA       Lender
Firm B Firm C    Bank D
```

These participants operate across organizational boundaries.

Identity establishes who they are.

Relationship establishes why they are involved.

Consent establishes what they can see.

---

# 60. Identity Proofing

The platform may eventually support different levels of verification.

Example:

**Self-asserted**

> User entered their name.

**Email verified**

> Email confirmed.

**Organization verified**

> Organization information verified.

**Professional identity verified**

> Professional relationship/credential evidence validated.

**Enhanced identity verification**

> Additional identity verification completed.

The platform should clearly distinguish these levels.

---

# 61. Identity Does Not Equal Credential Verification

A user saying:

> “I am an attorney”

does not establish that the platform has verified the professional credential.

Vendor Administration / Vetting owns credential evidence and verification.

Identity can display the resulting status.

---

# 62. Role Conflicts

A user may have multiple roles.

Example:

> User is both owner and professional.

The platform should explicitly handle the conflict.

For instance:

> A professional cannot review their own transaction as though they were an independent reviewer where platform policy prohibits it.

Policy determines the restriction.

Identity exposes the roles.

---

# 63. Separation of Duties

Identity & Access should support role separation.

Example:

> Reviewer

cannot simultaneously satisfy:

> Required independent approver

where policy requires separation.

The Policy Engine establishes the rule.

Identity supplies the role relationships.

---

# 64. Permission Explainability

Users and administrators should be able to ask:

> **Why can this person perform this action?**

Example:

> Jane can assign professional review tasks because:
>
> She is an active member of ABC Advisory.
>
> She has the Professional role.
>
> She is assigned to Transaction TX-004.
>
> The role grants `review_assigned_case`.

And:

> **Why can't she see the tax return?**

> Platform role permits professional participation, but resource access requires Consent & Access authorization.

This is extremely important for trust.

---

# 65. Permission Simulator

Administrators should eventually be able to ask:

> **What could Jane access right now?**

The system should show:

* Organization permissions
* Transaction roles
* Team permissions
* Current restrictions
* Consent-based resources separately

This should clearly distinguish:

> **Platform capability**

from:

> **Actual resource access**.

---

# 66. Permission Changes

Every significant permission change should be auditable.

Example:

> Jane promoted from Professional to Organization Administrator.

Audit records:

* Previous role
* New role
* Actor
* Timestamp
* Organization
* Scope
* Reason/reference if required

---

# 67. Emergency Suspension

Authorized administrators may need to suspend an account immediately.

Example:

> Security concern detected.

Identity changes:

> User status = Suspended.

That can trigger:

* Notification
* Consent review
* Communication restrictions
* Workflow reassignment
* Audit event

The relevant engines respond to the identity event.

---

# 68. Identity Events

The engine should emit events such as:

```text id="z6x3nt"
UserCreated
UserAuthenticated
AuthenticationFailed
UserVerified
UserSuspended
UserReactivated
MembershipCreated
MembershipRevoked
RoleAssigned
RoleRemoved
TeamCreated
TeamMemberAdded
TeamMemberRemoved
InvitationCreated
InvitationAccepted
InvitationExpired
PermissionGranted
PermissionRevoked
SessionCreated
SessionRevoked
DeviceRegistered
DeviceRevoked
ServiceIdentityCreated
CredentialRotated
```

Other engines can subscribe.

---

# 69. Core Data Objects

## User

Human platform identity.

## Organization

Organizational identity.

## Membership

Relationship between user and organization.

## Role

Defined platform responsibility.

## RoleAssignment

A role granted to a specific user in a specific scope.

## Team

Group of users.

## TeamMembership

User's membership in a team.

## Permission

Discrete platform capability.

## Invitation

Pending request to join an organization/team/context.

## Session

Authenticated platform session.

## Device

Recognized access device.

## ServiceIdentity

Non-human platform identity.

## Credential

Authentication or integration credential.

## IdentityVerification

Record of verification state.

## Delegation

Temporary transfer of authority.

---

# 70. Engine Contract

Core capabilities:

```text id="8z4mfw"
createUser()
authenticate()
verifyIdentity()
createOrganization()
inviteUser()
acceptInvitation()
createMembership()
removeMembership()
assignRole()
removeRole()
createTeam()
addTeamMember()
removeTeamMember()
grantPermission()
revokePermission()
createSession()
revokeSession()
registerDevice()
revokeDevice()
createServiceIdentity()
rotateCredential()
createDelegation()
revokeDelegation()
getUserContext()
getOrganizationContext()
getEffectivePermissions()
explainPermission()
```

Administrative capabilities:

```text id="f1p8sd"
suspendUser()
reactivateUser()
manageOrganization()
verifyOrganization()
manageRoles()
manageTeams()
simulatePermissions()
```

---

# 71. Effective Identity Context

Other engines should be able to request a normalized identity context.

For example:

```text id="w6f4tr"
User:
Jane Smith

Organizations:
ABC Advisory

Current Organization:
ABC Advisory

Role:
Professional

Specialty references:
Tax
ESOP

Transaction:
TX-004

Transaction Role:
Assigned Professional

Teams:
Tax Review Team

Identity State:
Active

Authentication State:
Verified
```

This becomes the stable identity context that other engines consume.

---

# 72. The Authorization Boundary

The overall access decision should work like this:

```text id="n8j4qp"
WHO ARE YOU?
     │
     ▼
IDENTITY & ACCESS
     │
     ▼
WHAT PLATFORM ROLE DO YOU HAVE?
     │
     ▼
POLICY / COMPLIANCE
     │
     ▼
MAY THIS TYPE OF ACTION OCCUR?
     │
     ▼
CONSENT & ACCESS
     │
     ▼
MAY YOU SEE THIS SPECIFIC RESOURCE?
```

That separation is one of the strongest architectural decisions in the entire platform.

---

# 73. What the Engine Should Never Do

It should never:

* Treat authentication as proof of professional qualification
* Treat organization membership as unrestricted data access
* Grant sensitive document access merely because someone has a role
* Replace Consent & Access
* Replace Vendor Vetting
* Replace Policy / Compliance
* Replace Professional Review
* Assume a person's role is identical in every transaction
* Silently escalate permissions
* Hide service or AI actors behind a generic “system” identity
* Delete historical identity information merely because a user was deactivated
* Treat an invitation as active membership
* Treat credential claims as credential verification

---

# 74. Architectural Lock

These should now be treated as requirements:

**1. Identity & Access is a standalone foundational engine.**

**2. It manages users, organizations, roles, teams, invitations, authentication, sessions, and platform permissions.**

**3. One person can have one platform identity across multiple organizational contexts where appropriate.**

**4. Organizational membership is separate from transaction participation.**

**5. Roles are scoped and versionable.**

**6. Teams are separate from roles.**

**7. Invitations are separate from membership.**

**8. Authentication is separate from identity verification.**

**9. Identity verification is separate from credential verification.**

**10. Professional role is separate from professional specialty and credential.**

**11. Service and AI actors have explicit identities.**

**12. Permissions are separate from specific resource disclosure.**

**13. Identity & Access establishes platform authority.**

**14. Consent & Access governs specific information access.**

**15. Policy / Compliance establishes platform rules and restrictions.**

**16. Role assignment never silently grants unrestricted access to sensitive information.**

**17. Permissions are scoped and explainable.**

**18. Temporary roles and delegations are supported.**

**19. Authentication sessions and devices are manageable and revocable.**

**20. High-risk account and permission changes can require stronger authentication or additional controls.**

**21. Offboarding removes current authority while preserving appropriate historical identity references.**

**22. Cross-organization participation is explicitly supported.**

**23. Separation-of-duty constraints can be enforced through Policy.**

**24. All material identity and permission changes are available to Audit.**

**25. Identity events are available to Workflow and Notification.**

**26. The engine does not determine professional qualification or legal authority.**

**27. Missing or ambiguous identity context does not silently produce broader permissions.**

**28. The engine is independently versioned, tested, secure, and replaceable.**

---

# 75. Architectural Boundary Summary

| Engine                              | Owns                                                                                                       | Does Not Own                              |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| **Identity & Access**               | Who the actor is, organization membership, roles, teams, invitations, authentication, platform permissions | Specific sensitive-data access            |
| **Policy / Compliance**             | Platform rules, restrictions, requirements, gates                                                          | Identity itself or professional advice    |
| **Consent & Access**                | Who may see a specific resource, purpose, duration, revocation                                             | Authentication or organizational identity |
| **Vendor Administration / Vetting** | Credential evidence, vendor verification, provider status                                                  | Login identity                            |
| **Stakeholder / Relationship**      | Why a person or organization participates in a transaction                                                 | Authentication or permission mechanics    |
| **Professional Review**             | Professional assignments and determinations                                                                | Identity infrastructure                   |
| **Audit / Provenance**              | History of identity and permission changes                                                                 | Current authorization                     |
| **Communication**                   | Conversations among authorized participants                                                                | Who a person is                           |
| **Workflow**                        | Task execution and event mechanics                                                                         | User identity and permissions             |
| **Local Vault**                     | Private local workspace and files                                                                          | User authentication                       |
| **Notification**                    | Alerts and delivery                                                                                        | Identity authority                        |

## Hard Boundary

> **Identity & Access establishes who you are and what you are permitted to do as a platform participant. Consent & Access determines what specific information you may actually see.**

That gives us a clean chain:

**Identity → Role → Policy → Permission → Consent → Resource**

And importantly:

> **Being the right person is not the same as being authorized to see everything.**
