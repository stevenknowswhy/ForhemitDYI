Yes. This is the **core discovery layer** for the professional ecosystem, and we should keep it separate from the Vendor Administration/Vetting Engine we already identified.

The **Marketplace Engine** answers:

> **“Who might be relevant to this owner's journey?”**

The **Vendor Administration/Vetting Engine** answers:

> **“Should this organization or professional be allowed into our curated marketplace, and are they still eligible?”**

That separation is important. One is customer-facing discovery; the other is platform governance.

# Marketplace Engine v1.0

## Curated Professional & Vendor Marketplace

## 1. Purpose

The Marketplace Engine is a standalone system for discovering, organizing, matching, and presenting curated professionals, firms, organizations, and specialized service providers that may help an owner accomplish the objectives identified in their ownership-transition journey.

The Marketplace Engine should make it possible for a user to move from:

> **“I need help with this part of my journey.”**

to:

> **“Here are several relevant professionals or organizations, I understand why they appeared, I can inspect their credentials and experience, and I can choose who I want to work with.”**

The engine must support:

* Organizations
* Professional contacts
* Specialties
* Services
* Profiles
* Search
* Matching
* Reviews
* Ratings
* Visibility
* Curation
* User selection
* Invitations
* Marketplace status

---

# 2. CORE MARKETPLACE PRINCIPLES

## Principle 1: Curated, not open

A professional cannot simply publish publicly because they created an account.

Marketplace inclusion requires platform approval through the separate Vendor Administration/Vetting Engine.

---

## Principle 2: No pay-to-play placement

The platform should not sell:

* Better ranking
* Preferred placement
* Sponsored recommendations
* Paid "top provider" status

Marketplace inclusion and relevance should be independent of payment.

Any future commercial relationships must be separately evaluated for conflicts of interest and transparently disclosed.

---

## Principle 3: User chooses

The marketplace helps the user discover options.

It does not make the professional-selection decision for them.

---

## Principle 4: Relevance over volume

The goal is not:

> **“Show the user 2,000 professionals.”**

The goal is:

> **“Show the user 2–3 professionals who appear relevant to this specific need.”**

The user can expand the search when desired.

---

## Principle 5: Organizations and people are different objects

A law firm is not one person.

A CPA firm may contain tax professionals, transaction professionals, valuation specialists, and other advisors.

The marketplace must support:

**Organization → Professionals → Specialties → Services**

---

## Principle 6: Profiles are structured

Marketing descriptions are useful, but matching must rely primarily on structured data.

---

## Principle 7: Ratings inform, not decide

User reviews and ratings provide evidence about experience.

They should not become a simplistic "best professional" leaderboard.

---

# 3. MARKETPLACE ENTITY MODEL

The Marketplace Engine should have these primary objects:

```text id="9eq4jv"
Organization
    │
    ├── Professional
    │      │
    │      ├── Specialties
    │      ├── Services
    │      ├── Experience
    │      └── Qualifications
    │
    ├── Locations
    ├── Organization Services
    └── Marketplace Profile

Specialty
Service
Review
Rating
Match
Search
Visibility Policy
Curation Record
Marketplace Listing
```

---

# 4. ORGANIZATION OBJECT

## `Organization`

Represents the company, firm, practice, lender, note buyer, or other entity.

Fields should include:

### Identity

* Organization ID
* Legal name
* Public/display name
* Organization type
* Description
* Website
* Main phone
* General email

### Location

* Headquarters
* Offices
* Service regions
* Geographic coverage

### Business Characteristics

* Industries served
* Business-size ranges
* Transaction-size ranges
* Organization specialties
* Services

### Marketplace

* Marketplace status
* Publication status
* Verification status
* Last verified date
* Profile completeness
* Marketplace categories

### Relationships

* Professionals
* Organization contacts
* Marketplace services
* Reviews
* Curation status

---

# 5. PROFESSIONAL OBJECT

## `Professional`

Represents an individual working within an organization or independently.

Fields:

* Professional ID
* Name
* Title
* Organization
* Biography
* Photo where appropriate
* Contact information
* Website/profile
* Professional credentials
* Licenses where relevant
* Years of experience
* Relevant transaction experience
* Industries
* Geographies
* Transaction sizes
* Specialties
* Services
* Availability
* Marketplace status
* Verification status
* Last verified date

---

# 6. INDEPENDENT PROFESSIONALS

Not every provider belongs to a large organization.

The engine must support:

```text id="kfywy3"
Independent Professional
      │
      ├── Profile
      ├── Specialties
      ├── Services
      └── Contact
```

The same marketplace rules apply.

---

# 7. SPECIALTY ENGINE WITHIN THE MARKETPLACE

Specialties should be hierarchical.

Example:

```text id="q1xkgd"
Legal
  └── Business Transactions
       ├── M&A
       ├── Employee Ownership
       ├── ESOP
       ├── Seller Financing
       ├── Corporate
       └── Tax

Financial
  ├── CPA
  ├── Transaction Tax
  ├── Wealth Management
  └── Fractional CFO

Valuation
  ├── Business Valuation
  └── Employee Ownership Valuation
```

This gives the Journey Engine the ability to request a general capability while the Marketplace Engine finds specific expertise.

---

# 8. SERVICE OBJECT

Specialty and service are not always the same.

Example:

**Specialty:** Employee Ownership

**Services:**

* ESOP advisory
* Transaction structuring
* ESOP implementation support
* Employee education

A professional can have several specialties and several services.

---

# 9. EXPERIENCE MODEL

The marketplace should capture structured experience.

Examples:

### Transaction Type

* Employee ownership
* ESOP
* Direct acquisition
* Management buyout
* Seller financing
* Business sale
* Seller-note transactions

### Business Size

* Under $1M
* $1M–$5M
* $5M–$10M
* $10M–$25M
* $25M+

### Transaction Size

Same concept.

### Industry

Standardized industry taxonomy.

### Geography

State / region / national / international as appropriate.

This allows meaningful matching.

---

# 10. CREDENTIAL MODEL

Credentials should not simply be typed into a bio.

They should be structured.

Potentially:

* Credential type
* Issuing authority
* Credential number where appropriate
* State/jurisdiction
* Effective date
* Expiration date
* Verification status
* Verification date

The Vetting Engine owns the actual verification process.

---

# 11. MARKETPLACE LISTING

The Organization or Professional may have a:

## `MarketplaceListing`

It contains:

* Listing ID
* Entity
* Marketplace category
* Display profile
* Visibility
* Eligibility
* Publication status
* Matching tags
* Curation status
* Last reviewed
* Marketplace metadata

This separates the actual professional record from its marketplace presentation.

---

# 12. CURATION

The marketplace should have a **Curation Layer**.

Curation should determine:

* Whether a provider belongs in a particular category
* Which specialties are verified
* Which journeys the provider is eligible for
* Which geographies they can appear in
* Which transaction sizes they can appear in
* Whether their listing is featured because of relevance, not payment
* Whether they should be included in curated results

Curation must be independently controlled by the platform.

---

# 13. CURATION IS NOT RANKING

We should distinguish:

**Eligible**

**Relevant**

**Curated**

**Potential Match**

from:

**Best**

**Top**

**#1**

The marketplace should avoid universal quality rankings.

---

# 14. CONTEXTUAL MATCHING

This is where the Marketplace Engine connects to the Journey Engine.

Suppose the owner reaches:

> **You may need legal expertise.**

The system already knows:

* Employee ownership
* Business size
* Industry
* Geography
* Scenario
* Seller financing
* Transaction size
* Stage of journey

The Marketplace Engine finds professionals whose structured profile appears relevant.

---

# 15. MATCH OBJECT

Create:

## `MarketplaceMatch`

It records:

* Match ID
* User/journey
* Need
* Provider
* Matching factors
* Match timestamp
* Visibility status
* Why matched
* User response

---

# 16. MATCH EXPLANATION

Every contextual match should be explainable.

Example:

### Why you're seeing this professional

✓ Employee ownership experience

✓ Business-size range match

✓ California coverage

✓ Seller-financing experience

✓ Relevant transaction-size experience

This prevents the marketplace from feeling like a mysterious recommendation algorithm.

---

# 17. MATCHING SHOULD BE MULTI-DIMENSIONAL

Potential matching inputs:

### Journey

What the user is trying to accomplish.

### Transaction stage

Where they are.

### Specialty

What expertise is needed.

### Business characteristics

Industry, size, geography.

### Scenario

ESOP, direct purchase, etc.

### Owner preferences

Existing relationships, geography, timing, etc.

### Provider criteria

What the professional actually says they handle.

---

# 18. MATCHING SHOULD NEVER OVERRIDE USER PREFERENCE

If the owner says:

> **I want a professional in California.**

the engine should prioritize that.

If the owner says:

> **I already have an attorney.**

the marketplace should not continue pushing attorneys.

If the owner says:

> **Show me other options.**

the marketplace expands.

---

# 19. USER'S OWN PROFESSIONAL

Marketplace is always optional.

At any relevant point:

### Do you already have someone?

**Yes, I'll use mine**

**Show me curated professionals**

**I'm not sure**

The Marketplace Engine does not have to be involved when the owner chooses their own professional.

---

# 20. SEARCH

The marketplace should support both:

### Journey-driven search

The system finds providers based on the owner's journey.

and:

### User-driven search

The user actively searches.

Example:

> "Search employee ownership attorneys in California."

Search should understand:

* Specialty
* Service
* Industry
* Geography
* Transaction size
* Organization
* Professional name
* Experience

---

# 21. SEARCH UX

Do not present 25 filters initially.

Start with:

### What matters most?

**Experience**

**Location**

**Transaction size**

Then progressively reveal additional filters.

Advanced search can expose more.

---

# 22. PROFILE PAGE

Each profile should have a consistent structure.

## Organization

### About

### Why this provider may be relevant

### Specialties

### Services

### Experience

### Industries

### Transaction sizes

### Geography

### Professionals

### Credentials

### Website

### Contact

### Platform Verification

### User Feedback

### Actions

**Select**

**Compare**

**Contact**

**Save**

---

# 23. ORGANIZATION PROFILE

Example:

# Smith & Jones LLP

### Employee Ownership • M&A • Tax

**Why it may be relevant**

Employee-ownership experience
California coverage
$2M–$25M business experience

### Professionals

Jane Smith
Employee Ownership

Robert Jones
M&A

Maria Chen
Tax

### Website

### Contact

### Reviews

---

# 24. PROFESSIONAL PROFILE

# Jane Smith

**Partner | Employee Ownership**

### Specialties

Employee Ownership
ESOP
Business Transactions

### Experience

15+ years

### Relevant Transaction Experience

Structured employee ownership transactions

### Organizations

Smith & Jones LLP

### Contact

Email
Phone
Website

---

# 25. PROFILE OWNERSHIP

Providers can manage their own profiles through the Vendor Portal.

They can update:

* Biography
* Services
* Specialties
* Locations
* Contact information
* Experience
* Team members
* Website
* Profile content

But certain fields remain platform-controlled.

---

# 26. VENDOR VS. PLATFORM AUTHORITY

### Vendor controls

What the provider says about themselves.

### Platform controls

What the platform has independently verified.

Examples:

Vendor:

> "We specialize in ESOP transactions."

Platform:

> **Employee ownership specialty verified**

This distinction should be visually clear.

---

# 27. PROFILE STATUS

Potential states:

**Draft**

**Submitted**

**Under Review**

**Published**

**Changes Requested**

**Temporarily Hidden**

**Suspended**

**Archived**

These states come from the Vetting/Admin system.

---

# 28. VISIBILITY ENGINE

The marketplace needs a standalone visibility-policy mechanism.

Potential states:

### Public

Anyone can find the listing.

### Journey Eligible

Only relevant journeys can surface it.

### Category Limited

Only certain specialties/categories.

### Geographic Limited

Only applicable regions.

### Invite Only

Not searchable broadly.

### Hidden

Not displayed.

### Suspended

Temporarily unavailable.

---

# 29. VISIBILITY SHOULD BE RULE-DRIVEN

Example:

```text id="tj3p4b"
Provider:
Employee Ownership Attorney

Conditions:
California
Business size $2M–$25M
Employee Ownership Journey

Result:
Eligible for contextual matching
```

The provider doesn't get to choose to appear in unrelated categories.

---

# 30. RATINGS

Ratings should be structured.

Potential dimensions:

* Responsiveness
* Communication
* Professional experience
* Understanding of transaction
* Clarity
* Overall experience

The dimensions should be configurable by marketplace category.

---

# 31. COMMENTS

Users may submit written comments.

The platform should support:

* Comment
* Date
* Related engagement
* Verified platform interaction
* Response from provider
* Moderation status

---

# 32. VERIFIED EXPERIENCE

Where the interaction occurred through the platform:

### Verified Platform Experience

This indicates that the platform can confirm the user engaged with the provider.

It should not mean:

> The platform guarantees the quality of the experience.

---

# 33. RATINGS SHOULD NOT BECOME THE PRIMARY MATCHING SIGNAL

A provider with 4.9 stars should not automatically outrank a provider with 4.7 stars if the 4.7 provider has much more relevant employee-ownership experience for this particular journey.

Matching should prioritize:

**Relevance**

then incorporate:

**Verified experience**

**User feedback**

**Availability**

**Other appropriate factors**

without turning the system into a leaderboard.

---

# 34. REVIEW MODERATION

The Marketplace Engine should expose reviews to the separate moderation/admin system.

Possible states:

**Published**

**Pending Review**

**Reported**

**Under Review**

**Removed**

**Disputed**

---

# 35. PROVIDER RESPONSE

Professionals should be able to respond to legitimate reviews.

Example:

> **Provider response**

This may help users understand disagreements without allowing the provider to erase unfavorable feedback.

---

# 36. COMPLAINTS

Users should have:

**Report a concern**

This should feed the Vendor Administration/Vetting Engine.

A complaint may trigger:

* Review
* Temporary visibility change
* Additional verification
* Provider response
* Administrative action

---

# 37. USER SAVES

A user should be able to:

**Save Professional**

**Save Organization**

**Compare Later**

This is particularly useful when the owner isn't ready to make a selection.

---

# 38. COMPARISON

The marketplace should allow factual comparison.

For example:

|                               | Provider A | Provider B |
| ----------------------------- | ---------- | ---------- |
| Employee ownership experience | ✓          | ✓          |
| California                    | ✓          | ✓          |
| Transaction size              | $2M–$10M   | $5M–$25M   |
| M&A                           | ✓          | ✓          |
| Seller financing              | ✓          | —          |
| Availability                  | Stated     | Stated     |

Avoid:

**Best Match**

**Winner**

**#1**

The user decides.

---

# 39. PROFESSIONAL SELECTION

When the owner selects a professional:

```text id="7s4qwd"
Marketplace
    ↓
Provider Selected
    ↓
Stakeholder Created
    ↓
Transaction Role Assigned
    ↓
Document Visibility Policy Applied
    ↓
Professional Review Workspace
```

The Marketplace Engine hands the selected provider to the Stakeholder/Relationship Engine.

---

# 40. MULTIPLE PROFESSIONALS PER ORGANIZATION

A user may select:

**Organization**

and then:

> **Which professional would you like to work with?**

Or the organization may assign a professional internally.

Both workflows should be supported.

---

# 41. MULTIPLE SPECIALTIES

A single professional may have:

**M&A**

**Employee Ownership**

**Seller Financing**

The system should retain each specialty separately rather than creating one huge description field.

This is essential for matching.

---

# 42. MARKETPLACE CATEGORY CAN DIFFER FROM PROFESSIONAL CATEGORY

For example:

A firm may offer:

**Business Law**

but only one attorney may have:

**Employee Ownership Experience**

The Marketplace Engine should surface the relevant individual, not blindly surface the entire firm.

---

# 43. MARKETPLACE + PROFESSIONAL REVIEW PACKAGE

When the user selects a provider:

The Professional Review Package Engine can know:

**Recipient = Employee Ownership Attorney**

and construct the right package automatically.

The Marketplace Engine itself does not create the document.

---

# 44. MARKETPLACE + DOCUMENT READINESS

The selected professional's role can trigger document requirements.

Example:

**Valuation professional selected**

↓

Document Readiness Engine

↓

**Valuation-related materials added**

The Marketplace Engine only announces:

> **Provider selected**

---

# 45. MARKETPLACE + CAPITAL

Capital providers can use the same core framework.

For example:

**Bank**

**SBA lender**

**Private lender**

**Seller-note buyer**

Each is an organization with specialized criteria.

The Marketplace Engine provides discovery.

The Capital Engine manages financing workflow.

---

# 46. MARKETPLACE + SELLER-NOTE LIQUIDITY

Seller-note buyers are a marketplace specialization.

The user might see:

### Potential Note Buyers

The Marketplace Engine supplies:

* Buyer profiles
* Criteria
* Verification
* Contact
* Website
* Experience

The Seller-Note Liquidity Engine handles:

* Matching
* Note opportunity
* Requests
* Offers
* Due diligence

---

# 47. NO PAY-TO-PLAY ARCHITECTURE

The matching engine should not accept:

> **"Pay $5,000 to appear first."**

Nor should the curation algorithm accept a hidden payment signal.

A provider's commercial relationship with the platform, if any, should not secretly alter contextual matching.

---

# 48. MATCH EXPLANATION OBJECT

Create:

## `MatchExplanation`

Contains:

* Journey
* Need
* Provider
* Matching factors
* Relevant experience
* Missing factors
* Timestamp
* Match-engine version

This means we can answer:

> **"Why did you show me this firm?"**

---

# 49. MARKETPLACE VERSIONING

The marketplace itself should be versioned.

Changes to:

* Specialty taxonomy
* Matching rules
* Visibility rules
* Curation rules

should have versions.

Example:

**Marketplace Rules v1.3**

A past match can therefore be reconstructed.

---

# 50. MATCH ENGINE TESTING

The Marketplace Engine should have test scenarios.

Example:

### Journey

California employee ownership
$5M EBITDA
ESOP exploration

Expected:

Show providers with:

* Employee ownership experience
* California coverage
* Relevant business size
* ESOP experience

Then test:

**Provider without California coverage**

Should not appear in the first contextual results.

---

# 51. SEARCH VS. MATCH

These should be separate capabilities.

### Search

User asks:

> "Show me employee-ownership attorneys."

### Match

System says:

> "Based on your journey, these professionals appear relevant."

Search returns results.

Matching explains **why these results were selected for this user**.

---

# 52. MARKETPLACE ANALYTICS

The engine should track:

* Searches
* Profile views
* Matches
* Saves
* Contacts
* Selections
* Search-to-selection conversion
* Specialty demand
* Geographic demand
* Unmet searches

This is important for discovering where the marketplace needs more providers.

---

# 53. DEMAND GAP ANALYSIS

If users frequently ask:

> "Find an employee-ownership attorney in Arizona."

and the marketplace has none:

The platform should detect:

### Marketplace Gap

**Employee Ownership Attorney**

**Arizona**

**High demand**

This feeds Vendor Administration and business development.

---

# 54. PROFILE COMPLETENESS

Vendors should have a profile completeness indicator.

Example:

### Profile completeness

**86%**

Missing:

* Transaction size
* Industry experience

This is operational guidance, not a ranking.

---

# 55. PROFILE FRESHNESS

Every profile should show:

### Last verified

**September 2026**

And internally:

**Next review**

A stale profile should become less visible or require re-verification according to marketplace policy.

---

# 56. CURATION RECORD

The platform should maintain:

## `CurationRecord`

Containing:

* Provider
* Category
* Reason included
* Evidence reviewed
* Approval date
* Reviewer
* Next review date
* Restrictions
* Current status

This is the marketplace's institutional memory.

---

# 57. MARKETPLACE TRUST RECORD

The Marketplace Engine should expose a simplified status to the user:

### Platform Verified

But the full:

**Curation + Vetting + Verification history**

remains internal.

This prevents the profile from becoming cluttered while preserving accountability.

---

# 58. MARKETPLACE DATA MODEL

Conceptually:

```text id="hhizco"
Marketplace
│
├── Organizations
│   ├── Profiles
│   ├── Contacts
│   ├── Services
│   └── Specialties
│
├── Professionals
│   ├── Credentials
│   ├── Experience
│   ├── Services
│   └── Specialties
│
├── Taxonomy
│   ├── Categories
│   ├── Specialties
│   └── Services
│
├── Listings
│
├── Search
│
├── Matching
│   ├── Match
│   └── MatchExplanation
│
├── Reviews
│   ├── Ratings
│   └── Comments
│
├── Visibility
│
└── Curation
```

---

# 59. ENGINE INPUTS

The Marketplace Engine receives information from:

### Journey Engine

Current journey need.

### Destination Engine

Owner objectives that may affect provider relevance.

### Business Reality Engine

Business characteristics relevant to provider matching.

### Scenario Engine

Transaction scenario being explored.

### Vendor Administration / Vetting

Provider eligibility and verification.

### User Preferences

Location, existing relationships, etc.

### Marketplace Activity

Availability and current listings.

---

# 60. ENGINE OUTPUTS

The Marketplace Engine produces:

### Search Results

What the user searched for.

### Contextual Matches

Providers relevant to the current journey.

### Match Explanations

Why they appeared.

### Organization Profiles

Structured firm information.

### Professional Profiles

Structured individual information.

### Ratings / Comments

User experience information.

### Selection Events

Who the owner selected.

### Marketplace Availability

Whether the provider can currently be considered.

### Demand Signals

Where additional providers may be needed.

---

# 61. WHAT THE MARKETPLACE ENGINE DOES NOT DO

It does not:

* Determine professional competence
* Give legal advice
* Give tax advice
* Make investment decisions
* Make financing decisions
* Determine transaction feasibility
* Select the provider for the user
* Create professional determinations
* Create contracts
* Guarantee outcomes

The separate systems handle those responsibilities.

---

# 62. THE MARKETPLACE JOURNEY

The ideal user interaction is:

> **You may need specialized legal help.**

↓

### Do you already have someone?

**Yes, I'll use mine**

**Show me curated professionals**

**I'm not sure**

↓

User chooses:

**Show Me Curated Professionals**

↓

### We found 3 professionals who appear relevant.

**Why you're seeing them**

↓

User opens profiles.

↓

**Compare**

↓

**Select**

↓

**Add to My Transaction Team**

↓

The provider becomes a Stakeholder.

---

# 63. MARKETPLACE NORTH STAR

The marketplace should answer:

> **"Who are the relevant, independently vetted professionals and organizations that could help me with this particular part of my journey?"**

It should make that discovery easy without pretending to know:

> **"Which professional is best for you?"**

The owner chooses.

The professional advises.

The platform coordinates.

---

# 64. ONE-SENTENCE DEFINITION

> **The Marketplace Engine is the curated discovery and matching system that connects owners with independently vetted professionals and service providers based on the actual needs of their ownership-transition journey, while preserving transparency, choice, and non-pay-to-play curation.**

### The architecture now has an important separation

We have:

**Marketplace Engine**
Who could help?

**Vendor Administration / Vetting Engine**
Should this provider be in our marketplace?

**Stakeholder Engine**
What role does this selected person have in this transaction?

**Professional Review Engine**
What does the professional actually say?

That is a very clean chain.

And the **multi-contact organization model** is worth emphasizing. We shouldn't think of the marketplace as a pile of vendors. It's a structured professional graph:

**Firm → People → Specialties → Experience → Services → Journey relevance.**

That becomes particularly powerful when an owner says, *“I already have a law firm,”* because the application doesn't need to throw away that relationship. It can ask which specialty is needed, see that their firm has the appropriate attorney, and potentially bring that person into the journey.
