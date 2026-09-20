> **Superseded.** Replaced by two documents: [Marketplace Engine v1.0](Marketplace%20Engine%20v1.0.md) owns discovery, matching and presentation, and [Vendor Administration - Vetting Engine v1.0](Vendor%20Administration%20-%20Vetting%20Engine%20v1.0.md) owns eligibility, vetting and governance. Section 37 below bundles exactly that pair into a single engine, which is why this draft is no longer current. Kept for provenance.

I like this direction. It gives the marketplace a very different identity from a lead-generation directory.

The principle becomes:

> **We curate the professionals. We don't sell placement. The user chooses.**

And the WordPress-like vendor experience is important because we want a marketplace that can become large without requiring our staff to manually maintain every profile.

I would define it as another **standalone Marketplace Engine** with separate vendor management, organization/contact management, vetting, publishing, matching, and ratings components.

# Professional Marketplace Engine v1.0

## 1. Purpose

The Professional Marketplace is a **curated, independently vetted professional network** designed to help business owners find specialists who may be useful in their ownership-transition journey.

The marketplace is not intended to be:

* An open directory
* A paid advertising marketplace
* A lead-selling service
* A ranking-for-payment system

Its purpose is:

> **Identify qualified professionals and organizations that may be useful to the owner's particular journey and make those choices easy to understand.**

---

# 2. MARKETPLACE TRUST PRINCIPLE

The foundational marketplace rule is:

> **Professionals and vendors do not pay for inclusion, placement, ranking, or favorable treatment in the marketplace.**

The platform independently determines whether a professional or organization qualifies for inclusion.

This should be prominently disclosed to users.

### Example

> **How our marketplace works**
>
> Professionals and firms do not pay us to be listed or ranked. We independently review providers before allowing them to publish in our marketplace.
>
> Inclusion means the provider has met our marketplace requirements. It does not constitute a guarantee, endorsement, or recommendation for your particular situation.

Any future revenue relationship with a marketplace participant should be separately evaluated for conflicts and clearly disclosed.

---

# 3. CURATED, NOT OPEN

A provider cannot simply create an account and immediately appear publicly.

The process is:

```text
Provider Registration
        ↓
Profile Completion
        ↓
Credential / Experience Review
        ↓
Platform Vetting
        ↓
Approval
        ↓
Profile Published
        ↓
Ongoing Monitoring
```

The platform controls publication.

---

# 4. PROVIDER TYPES

The marketplace should support many professional categories.

Initial categories may include:

### Legal

* Business transaction attorney
* M&A attorney
* Employee-ownership attorney
* Tax attorney
* Employment attorney
* Corporate attorney
* Other specialized legal professionals

### Financial

* CPA
* Transaction tax specialist
* Financial advisor
* Wealth advisor
* Fractional CFO
* Quality-of-earnings provider

### Valuation

* Business valuation professional
* ESOP valuation professional
* Appraisal firm
* Other specialized valuation providers

### Employee Ownership

* ESOP consultant
* ESOP administrator
* ESOP trustee
* Employee-ownership advisor

### Financing

* SBA lender
* Bank
* Credit union
* Acquisition lender
* Specialty finance provider

### Seller-Note Liquidity

* Seller-note buyer
* Private credit provider
* Specialty note purchaser
* Other qualified capital provider

### Transaction / Closing

* Escrow
* Closing provider
* Insurance professional
* Other transaction specialist

The category system should be configurable so new specialties can be added without changing the platform architecture.

---

# 5. ORGANIZATION-FIRST MODEL

The marketplace should not treat every professional as a completely independent vendor.

A firm or organization should be able to contain multiple professional contacts.

For example:

## Smith & Jones LLP

### Professionals

**Jane Smith**
Employee Ownership / ESOP Attorney

**Robert Jones**
M&A Attorney

**Maria Chen**
Tax Attorney

**David Lee**
Employment Attorney

The user may choose:

**The Firm**

or

**A Specific Professional**

depending on the service.

---

# 6. ORGANIZATION OBJECT

Each organization should have:

* Organization name
* Description
* Website
* Main phone
* General email
* Headquarters
* Offices
* Service regions
* Industries
* Transaction-size ranges
* Organization categories
* Specialties
* Verification status
* Marketplace status
* Last review date
* Contacts
* Published content
* Ratings/comments
* Platform notes
* Internal vetting history

---

# 7. PROFESSIONAL CONTACT OBJECT

Each individual professional should have:

* Name
* Title
* Organization
* Photo where appropriate
* Professional specialty
* Secondary specialties
* Credentials
* Licenses where applicable
* Years of experience
* Relevant transaction experience
* Employee-ownership experience
* Industries served
* Geographic coverage
* Transaction-size experience
* Services offered
* Contact information
* Website/profile information
* Availability
* Languages where appropriate
* Verification status
* Last verified date
* User ratings/comments

---

# 8. SPECIALTY HIERARCHY

Specialties should support parent/child relationships.

Example:

```text
Legal
  └── Business Transactions
       ├── M&A
       ├── Employee Ownership
       ├── Seller Financing
       ├── Corporate
       └── Tax
```

This lets the Journey Engine request:

**Legal expertise**

while the Marketplace Engine finds:

**Employee Ownership + M&A**

rather than just returning every attorney.

---

# 9. PROFESSIONAL SKILL TAGS

Each provider should have structured tags.

Example:

### Jane Smith

**Specialties**

Employee Ownership
ESOP
Business Acquisitions
Seller Financing

**Business Size**

$2M–$25M revenue

**Transaction Size**

$1M–$10M

**Industries**

Healthcare
Professional Services
Manufacturing

**Geography**

California
Nevada
Arizona

This structured information powers matching.

---

# 10. WORDPRESS-LIKE VENDOR EXPERIENCE

The professional should receive a simple dashboard.

Conceptually:

# My Marketplace Profile

**Profile**

**Organization**

**Professionals**

**Specialties**

**Services**

**Experience**

**Geography**

**Credentials**

**Website & Contact**

**Reviews**

**Visibility**

**Verification**

**Activity**

The professional edits their information themselves.

They should not need technical knowledge.

---

# 11. PROFILE EDITING

The vendor experience should use simple forms and progressive disclosure.

For example:

### What services do you provide?

Select from approved categories.

### What transaction sizes do you typically handle?

Select ranges.

### What industries do you serve?

Select applicable industries.

### What employee-ownership experience do you have?

Select experience categories and provide supporting information.

The system should standardize answers so profiles remain searchable and comparable.

---

# 12. VENDOR CONTENT VS. SYSTEM-CONTROLLED CONTENT

This distinction is important.

### Vendor controls

* Description
* Bio
* Services
* Experience
* Website
* Contact information
* Business locations
* Team members
* Approved profile fields

### Platform controls

* Verification badge
* Marketplace category
* Eligibility
* Publication status
* Visibility
* Specialty classification
* Matching tags
* Vetting status
* Administrative notes
* Whether the provider appears in curated results

A vendor should not be able to create their own:

**"Best Provider"**

or

**"Top Rated"**

designation.

Those are platform-controlled concepts.

---

# 13. PUBLISHING WORKFLOW

Vendor edits should not necessarily become public immediately.

Possible status:

**Draft**

**Submitted for Review**

**Approved**

**Changes Required**

**Published**

**Suspended**

**Archived**

The administrator controls transitions between these states.

---

# 14. ADMINISTRATION CONSOLE

The platform administrator needs a marketplace management interface.

### Marketplace Admin

**Organizations**

**Professionals**

**Categories**

**Specialties**

**Applications**

**Vetting Queue**

**Verification**

**Reviews**

**Reports**

**Visibility**

**Marketplace Analytics**

**Policy**

This should feel like a **CMS administration console**.

---

# 15. VETTING ENGINE

Vetting should also be a standalone component.

The platform should record:

* What was checked
* Who checked it
* Date checked
* Evidence used
* Credentials reviewed
* License status where relevant
* Experience claims reviewed
* References where applicable
* Conflicts identified
* Approval decision
* Next review date

The provider profile can display:

### Platform Verified

**Last reviewed:** September 2026

Internally, the platform maintains the full vetting record.

---

# 16. VETTING IS NOT A GUARANTEE

The marketplace should clearly distinguish:

**Verified**

from

**Guaranteed**

The platform can verify information and establish eligibility based on its process.

It cannot guarantee that a professional will be the right choice for every owner or transaction.

---

# 17. ONGOING RE-VERIFICATION

Vetting should not be a one-time event.

Profiles should have:

**Last Verified**

**Next Review**

Potential triggers for re-review:

* Credential expiration
* Significant profile change
* User complaints
* Regulatory/public-record events where appropriately monitored
* Repeated poor feedback
* Provider inactivity
* Material changes to services

The platform can request updated information from providers.

---

# 18. USER COMMENTS AND RATINGS

The marketplace should include user feedback.

Potential rating dimensions could include:

* Professional communication
* Responsiveness
* Understanding of the transaction
* Clarity
* Overall experience

But ratings should be handled carefully.

A single star number can imply more precision than it deserves.

The system should support:

### Rating

and

### Written Comment

with moderation.

---

# 19. REVIEW AUTHENTICITY

Where practical, reviews should be associated with actual platform interactions.

For example:

**Verified Platform Experience**

would indicate that the reviewer actually interacted with the provider through the platform.

Reviews obtained outside the platform can potentially be handled differently and clearly labeled.

The platform should not fabricate or import reviews without a documented source.

---

# 20. REVIEW MODERATION

The marketplace should have a separate moderation process.

Administrators should be able to:

* Review reported comments
* Remove prohibited content
* Request clarification
* Mark disputed reviews
* Suspend abusive reviewers
* Respond to complaints
* Temporarily disable reviews for a provider
* Preserve audit history

Providers should have a mechanism to respond to reviews.

---

# 21. DO NOT TURN REVIEWS INTO A SIMPLE LEADERBOARD

We should avoid:

> #1 Attorney

> #2 Attorney

> #3 Attorney

That risks turning a complex professional decision into a simplistic score.

Instead, the marketplace should emphasize:

**Relevant Experience**

**Specialty**

**Verified Information**

**User Feedback**

**Fit for This Journey**

---

# 22. CONTEXTUAL MATCHING

This is where the marketplace becomes integrated with the Journey Engine.

The user reaches:

### You may need legal expertise.

The system already knows:

* Employee ownership
* Business size
* Industry
* Geography
* Timing
* Potential structures
* Seller financing
* Other relevant factors

The Marketplace Engine uses that information to identify appropriate candidates.

The user sees:

### Professionals relevant to your journey

**2–3 initial options**

Each showing why they appeared.

Example:

> **Why you're seeing this professional**
>
> Employee ownership experience
> Business size match
> California coverage
> Seller-financing experience

---

# 23. MATCHING IS NOT RECOMMENDATION

The platform should phrase this carefully.

Instead of:

> **"We recommend Jane Smith."**

say:

> **"Jane Smith appears relevant based on the information you've provided."**

Then:

**View Profile**

**Select**

**Compare**

**Show More**

This preserves user agency.

---

# 24. USER CAN BRING THEIR OWN PROFESSIONAL

The marketplace must remain optional.

At every relevant stage:

**Use My Professional**

or

**Browse Curated Professionals**

The user is never required to use the marketplace.

---

# 25. STAKEHOLDER DOCUMENT INTEGRATION

When the user chooses a professional from the marketplace, that person can become part of the transaction workspace.

The platform can know:

**Who they are**

**What role they have**

**What specialty they provide**

**What information they should receive**

This connects directly to the Stakeholder Document Engine.

---

# 26. MULTIPLE CONTACTS WITHIN ONE ORGANIZATION

A law firm example:

```text
Smith & Jones LLP
       │
       ├── Jane Smith
       │   Employee Ownership
       │
       ├── Robert Jones
       │   M&A
       │
       ├── Maria Chen
       │   Tax
       │
       └── David Lee
           Employment
```

The owner might initially select:

**Smith & Jones LLP**

Then the platform can say:

> **Which specialty do you need?**

**Employee Ownership**

**Tax**

**Transaction Law**

or allow the firm to assign internally.

---

# 27. ORGANIZATION-LEVEL VS. INDIVIDUAL-LEVEL SELECTION

The Journey Engine should support:

### Select Organization

> "I want to work with Smith & Jones LLP."

### Select Individual

> "I specifically want Jane Smith."

### Invite Existing Contact

> "I already work with Maria Chen."

This accommodates different professional business models.

---

# 28. VISIBILITY CONTROLS

The administrator determines:

### Public

Visible to all users.

### Journey-Matched

Visible only when relevant to a journey.

### Invite-Only

Visible to selected users or transaction types.

### Hidden

Profile exists but isn't publicly shown.

### Suspended

Unavailable while under review.

This prevents the marketplace from becoming an uncontrolled directory.

---

# 29. MARKETPLACE DATA SHOULD BE STRUCTURED

The platform shouldn't store every vendor as a blob of marketing copy.

It should maintain structured fields.

This allows us to ask:

> Find California professionals who have employee-ownership experience with $2M–$10M EBITDA businesses and seller-financing experience.

The system can then return meaningful matches.

---

# 30. VENDOR ANALYTICS

Vendors can eventually see:

* Profile views
* Journey matches
* Contact requests
* Invitations
* Profile completeness
* User feedback

But vendors should not necessarily see sensitive transaction information about users who viewed them.

---

# 31. ADMINISTRATOR ANALYTICS

The platform should track:

* Most searched specialties
* Unmet professional needs
* Search-to-contact conversion
* Profile completeness
* Marketplace usage
* Provider response rates
* Complaints
* Review trends
* Geographic gaps
* Specialty gaps

This can show where the marketplace needs expansion.

---

# 32. MARKETPLACE REVENUE PRINCIPLE

The initial marketplace model should be:

> **No paid placement.**

The brainstorm should separately explore whether the platform can generate revenue through mechanisms that do not compromise neutrality.

Potential future options can be evaluated carefully, such as:

* Platform subscription
* Transaction-management fees
* Professional software tools
* Vendor workflow tools
* Administrative services
* Optional premium functionality

Any compensation tied to referrals, financing, transactions, or placement should undergo explicit conflict-of-interest and regulatory review before being adopted.

---

# 33. SELLER-NOTE BUYERS USE THE SAME PLATFORM FOUNDATION

Seller-note buyers should be represented as specialized marketplace participants.

They can have:

### Organization

Company information

### Buyer Profile

Investment criteria

### Note Preferences

* Size
* Duration
* Industry
* Security
* Priority
* Geography

### Contact

Name
Email
Phone
Website

### Verification

Platform review status

### Activity

Active / inactive

The marketplace can then identify:

> **Potential buyers whose stated criteria appear compatible with your seller note.**

---

# 34. MARKETPLACE FILTERS

The user should be able to narrow results using simple choices.

For example:

### What matters most?

**Experience**

**Location**

**Transaction size**

Then:

**Show me matches**

Do not dump 50 filters on the screen.

The Journey Engine can handle most filtering automatically.

---

# 35. MARKETPLACE PROFILE EXAMPLE

## Smith & Jones LLP

**Employee Ownership • M&A • Tax**

### Why this profile appeared

✓ Employee-ownership experience
✓ Business-size range matches
✓ California coverage
✓ Seller-financing experience

### Firm

San Francisco, California

### Website

smithjones.com

### Professionals

4 relevant professionals

### Experience

[Structured information]

### Verification

**Platform Verified**

Last reviewed: September 2026

### User Feedback

[Ratings + comments]

### Actions

**View Professionals**

**Select Firm**

**Compare**

---

# 36. CURATION PRINCIPLE

The marketplace should optimize for:

**Relevance + Quality + Transparency**

not:

**Maximum number of vendors.**

A marketplace containing 20 highly relevant professionals may be more useful than a directory containing 20,000 names.

---

# 37. THE MARKETPLACE ENGINE

The marketplace should operate as its own standalone engine with responsibilities for:

* Organizations
* Professionals
* Specialties
* Profiles
* Credentials
* Vetting
* Publication
* Visibility
* Search
* Matching
* User feedback
* Reviews
* Moderation
* Marketplace analytics

It should expose clean interfaces to:

**Journey Engine**

**Professional Review Package Engine**

**Document Engine**

**Transaction Workspace**

and **Identity / Permissions**.

---

# 38. MARKETPLACE TRUST LEDGER

A provider should have an auditable internal record:

```text id="4oj9a5"
Provider
   ↓
Credential Evidence
   ↓
Vetting Decision
   ↓
Publication
   ↓
User Interactions
   ↓
Reviews / Complaints
   ↓
Reverification
   ↓
Current Status
```

This creates accountability for the marketplace itself.

---

# 39. CORE USER FLOW

The ideal experience is:

> **You may need an attorney for this part of your journey.**

↓

**Do you have one?**

**Yes, I'll use mine**

**Show me curated professionals**

↓

User chooses curated.

↓

**Here are 3 professionals who appear relevant to your situation.**

↓

**Why you're seeing them**

↓

User selects one.

↓

**Add to My Transaction Team**

↓

The professional becomes part of the appropriate workflow and stakeholder-document system.

---

# 40. CORE MARKETPLACE PRINCIPLE

The marketplace should feel like:

> **A trusted professional shelf curated for your particular journey.**

Not:

> **An internet directory with ads.**

---

# 41. NORTH STAR

The marketplace should answer:

> **"Who might be qualified and relevant to help with this specific part of my journey?"**

while preserving:

**User choice**

**Independent curation**

**Transparent qualification**

**No paid placement**

**Clear conflicts**

**Appropriate disclosure**

**Professional independence**

The marketplace connects the owner to potential professionals.

**The owner chooses. The professional advises. The platform coordinates.**

## One thing I would lock especially hard

I would make **“no pay-to-play placement”** a foundational marketplace rule from the beginning.

That creates a very different relationship with the vendor community. A firm isn't paying us to appear first. Its profile gets there because we independently decided it met our criteria.

That also means the **vendor CMS and the admin CMS need to be separate authorities**:

**Vendor says:**
*"Here's who we are and what we do."*

**Platform says:**
*"We've verified these claims, classified your specialties, and decided where you're eligible to appear."*

That separation is excellent for trust.

And the organization/contact model is important too. A law firm isn't one generic "vendor." It might be **one organization with six lawyers and five different specialties**, while a CPA firm might have transaction tax, valuation, and advisory specialists. The Journey Engine can ask for the *function* needed, and the Marketplace Engine can find the right person inside the right organization.

That gets us much closer to the platform's overarching promise:

**The owner doesn't have to know who they need. They just have to tell us where they're trying to go.**
