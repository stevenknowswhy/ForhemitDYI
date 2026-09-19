# Blog / Publishing Engine

## 1. Purpose

The Blog / Publishing Engine manages the platform's editorial content and publishing lifecycle.

Its sole job is:

> **Turn approved ideas and source material into organized, versioned, publishable public content while keeping editorial content separate from the core transaction and private workspace systems.**

It handles:

* Blog posts
* Articles
* Authors
* Editorial workflow
* Drafts
* Reviews
* Revisions
* Categories
* Tags
* Series
* SEO metadata
* Featured content
* Publication scheduling
* Content status
* Content relationships
* Content performance references
* Content syndication references
* Publishing history

---

# 2. Core Architectural Principle

Editorial content is a product surface, not the platform's business database.

The Blog Engine may reference platform knowledge, research, and public concepts.

It should never expose private business information merely because the information exists somewhere in the platform.

The public publishing boundary must be explicit.

---

# 3. What This Engine Owns

* Editorial content
* Posts
* Articles
* Authors
* Editorial teams
* Drafts
* Review status
* Revision history
* Publishing state
* Categories
* Tags
* Series
* SEO metadata
* Slugs
* Excerpts
* Featured media references
* Editorial approval
* Scheduling
* Publication history
* Content relationships

---

# 4. What It Does Not Own

It does not own:

* WordPress infrastructure
* Domain management
* Plugins
* Themes
* Core platform identity
* Private documents
* Transaction records
* Professional reviews
* Billing
* Marketplace provider records
* Research source-of-truth

The WordPress Management Engine delivers content to WordPress.

The Research and Evidence systems remain authoritative for research claims.

---

# 5. Editorial Content Object

An Article contains:

* Article ID
* Title
* Subtitle/deck
* Slug
* Author(s)
* Status
* Body/content blocks
* Excerpt
* Categories
* Tags
* Series
* Featured media
* SEO title
* SEO description
* Canonical reference
* Publication date
* Revision history
* Source references
* Approval state
* Target channels

---

# 6. Content Types

The engine should support more than traditional blog posts.

Examples:

* Article
* Guide
* Case study
* Interview
* Research summary
* Announcement
* Resource
* FAQ
* Editorial series

The underlying editorial model remains reusable.

---

# 7. Editorial Lifecycle

Suggested states:

* Idea
* Draft
* Under Review
* Needs Revision
* Approved
* Scheduled
* Published
* Updated
* Archived
* Withdrawn

---

# 8. Idea Capture

Ideas can originate from:

* User brainstorming
* Research
* Platform discussions
* Professional insights
* Customer questions
* Analytics
* News/events
* Existing articles

An idea is not automatically publication-ready.

---

# 9. Editorial Brief

Before drafting, the platform can create a brief:

* Audience
* Topic
* Objective
* Core thesis
* Key evidence
* Sources
* Intended action
* Tone
* Target length
* Publication channel

This allows the editorial process to remain structured without forcing a giant form.

---

# 10. Source and Evidence Integrity

When editorial content uses research, the content should be able to reference:

* Evidence Ledger findings
* Research sources
* Publication dates
* Source URLs/references
* Scope
* Claims supported

The Blog Engine does not become the Evidence Ledger.

It references supporting evidence.

---

# 11. AI-Assisted Writing

AI may assist with:

* Outlines
* Drafting
* Editing
* Summarization
* Headline alternatives
* SEO suggestions
* Fact-reference checks

But AI-generated material should be distinguishable from human-approved final content.

The platform should preserve:

* AI-generated draft
* Human edits
* Final approved version

---

# 12. Editorial Approval

Publication should support explicit approval.

Possible roles:

* Author
* Editor
* Reviewer
* Publisher
* Administrator

A publisher should not automatically have permission to alter the substantive source research.

---

# 13. Publication vs Approval

These are distinct.

**Approved**

Editorial team approved content for publication.

**Published**

Content is publicly available.

WordPress Management executes the technical publication.

---

# 14. Versioning

Every material article change should create a new revision.

Example:

* Article v1 — Draft
* Article v2 — Edited
* Article v3 — Approved
* Article v4 — Updated after publication

The published version should remain identifiable.

---

# 15. Published Revisions

If a published article changes, the platform should preserve:

* Previous published version
* New published version
* What changed
* Who changed it
* Approval state
* Publication timestamp

This is particularly important for evidence-heavy content.

---

# 16. Categories

Categories should describe major editorial areas.

Examples:

* Employee Ownership
* Business Transition
* Entrepreneurship
* Financing
* Governance
* Succession

Categories should remain relatively stable.

---

# 17. Tags

Tags provide more granular relationships.

Examples:

* ESOP
* Management Buyout
* Seller Financing
* Valuation
* Succession

Tags can evolve more freely than categories.

---

# 18. Series

A series groups related content.

Example:

**Selling Your Business to Employees**

* Part 1: Understanding the destination.
* Part 2: Assessing the business.
* Part 3: Exploring structures.
* Part 4: Preparing for professionals.

This creates a coherent public knowledge path.

---

# 19. Editorial Collections

The engine can support curated collections such as:

* New owner guide
* ESOP fundamentals
* Seller-financing guide
* Succession planning
* Transaction readiness

These are editorial constructs, not transaction workflows.

---

# 20. SEO Metadata

The engine can manage:

* SEO title
* Meta description
* Slug
* Canonical URL
* Social preview text
* Structured data references

SEO optimization should never silently alter the substantive factual content.

---

# 21. Publication Scheduling

Posts can be scheduled for:

* Specific date/time
* Campaign launch
* Editorial calendar
* Series release

Workflow and Notification provide generic scheduling infrastructure.

Blog owns publication intent.

---

# 22. Editorial Calendar

The editorial calendar should show:

* Drafts
* Reviews
* Approved posts
* Scheduled publication
* Published content
* Campaigns

This becomes the content team's operational view.

---

# 23. Public/Private Boundary

A blog article can reference public platform knowledge.

It should not automatically expose:

* Business owner identity
* Private financial figures
* Professional packages
* Transaction documents
* Private communications
* Internal decisions
* Sensitive employee information

unless explicitly authorized for publication.

---

# 24. Source Authorization

Where content is based on private material, the system should identify:

> Source material is private. Publication authorization required.

Consent & Access controls the disclosure.

Blog controls editorial use.

---

# 25. Anonymous / Abstracted Content

The engine can support intentionally abstracted case studies.

Example:

> “A regional service company with approximately $8M in annual revenue...”

instead of exposing the actual business identity.

Any claim of anonymization should be careful and should not imply perfect de-identification.

---

# 26. Content Review Requirements

Policy may require specific review for certain content types.

Examples:

* Financial claims
* Professional topics
* Regulatory topics
* Case studies
* Customer testimonials

The Blog Engine routes the content for appropriate review.

It does not provide the professional determination itself.

---

# 27. Professional Review of Public Content

A professional might review an article for:

* Legal accuracy
* Tax accuracy
* Financial terminology
* Industry-specific accuracy

The professional determination remains in Professional Review.

Blog stores the publication approval reference.

---

# 28. Fact Checking

The platform can run a structured fact check using:

* Research Engine
* Evidence Ledger
* Business Reality, where explicitly authorized

A fact-check result should be distinct from the final editorial approval.

---

# 29. Claim Registry

A useful future capability is a lightweight Public Claim Registry.

Each important published claim can reference:

* Claim
* Source
* Evidence
* Publication date
* Review status
* Last reviewed date

This makes the public knowledge layer maintainable over time.

---

# 30. Staleness

Published content can become outdated.

The engine should support:

> Review by date.

Example:

> Article last reviewed September 2026.

> Recheck required September 2027.

Workflow creates the review task.

Notification alerts the editor.

---

# 31. Content Retirement

When content becomes obsolete, the platform should support:

* Update
* Redirect
* Archive
* Withdraw
* Replace

The engine should preserve the historical article version.

---

# 32. Content Analytics References

The Blog Engine may consume analytics such as:

* Views
* Engagement
* Search traffic
* Conversion reference
* Newsletter performance

Analytics remain owned by an analytics system where appropriate.

Blog uses the results for editorial management.

---

# 33. Editorial Performance

The platform can answer:

* Which topics are receiving attention?
* Which articles need updating?
* Which series are incomplete?

These are editorial observations, not business or political rankings.

---

# 34. Syndication

The engine may eventually publish to:

* WordPress
* Website
* Newsletter
* RSS
* Social platforms
* Documentation site

Each destination receives an appropriately transformed representation.

The Blog Engine remains the editorial source.

---

# 35. Canonical Content

One article can have:

* Canonical editorial version
* Multiple channel-specific representations

This prevents:

* LinkedIn copy
* Website copy
* Newsletter copy

from becoming three conflicting sources of truth.

---

# 36. Editorial Workflow Example

```text
Idea
  ↓
Editorial Brief
  ↓
Draft
  ↓
Research / Evidence Check
  ↓
Professional Review if required
  ↓
Editor Approval
  ↓
Scheduled
  ↓
WordPress Management
  ↓
Published
  ↓
Review / Update
```

This makes the publishing lifecycle explicit.

---

# 37. Content and Main Platform

The Blog Engine can reference public concepts from the core application:

* Destination methodology
* Business Reality concepts
* Ownership lifecycle concepts
* Professional review concepts

But the blog remains a public explanatory layer, not a mirror of private application data.

---

# 38. Content Permissions

* Identity & Access determines who may edit editorial content.
* Policy determines publication rules.
* Consent & Access governs use of private source material.
* Blog determines editorial state.
* WordPress Management handles publication infrastructure.

---

# 39. Core Data Objects

## Article

Editorial content.

## Draft

Unpublished working version.

## Revision

Historical content version.

## EditorialBrief

Purpose and scope of content.

## Author

Editorial author identity reference.

## Category

High-level editorial grouping.

## Tag

Granular content label.

## Series

Connected group of content.

## Publication

A channel-specific publication event.

## ContentReview

Editorial or professional review record/reference.

## ClaimReference

Reference to supporting evidence.

## ContentCollection

Curated public grouping.

---

# 40. Engine Contract

```text
createIdea()
createBrief()
createArticle()
createDraft()
createRevision()
submitForReview()
requestProfessionalReview()
approveContent()
schedulePublication()
publish()
unpublish()
archive()
withdraw()
addCategory()
addTag()
createSeries()
linkEvidence()
linkSource()
createPublication()
getEditorialCalendar()
getRevisionHistory()
getContentStatus()
```

---

# 41. Events

Examples:

```text
ArticleCreated
DraftUpdated
ContentSubmittedForReview
ProfessionalReviewRequired
ContentApproved
ContentScheduled
ContentPublished
ContentUpdated
ContentWithdrawn
ContentArchived
ContentReviewDue
```

Workflow, Notification, Audit, WordPress Management, and research-related engines can consume these events.

---

# 42. Architectural Lock

* Blog / Publishing is a standalone engine.
* It owns editorial content and publication intent.
* It does not own WordPress infrastructure.
* It does not own private platform data.
* Public publication is an explicit state transition.
* Private source material requires authorized publication use.
* Editorial versions are preserved.
* Published content can be revised without destroying prior published versions.
* Research and evidence remain authoritative for factual sourcing.
* Professional determinations remain in Professional Review.
* Policy defines publication restrictions and required review.
* WordPress Management handles technical publication.
* Identity & Access controls editorial user authority.
* Consent & Access controls use of protected source material.
* Workflow handles scheduling and recurring review mechanics.
* Notification handles alerts.
* Audit records publication history.
* The engine is independently versioned, tested, and replaceable.

---

# 43. Architectural Boundary Summary

| Engine                   | Owns                                                                              | Does Not Own              |
| ------------------------ | --------------------------------------------------------------------------------- | ------------------------- |
| **Blog / Publishing**    | Editorial content, drafts, revisions, approval, publication intent                | WordPress infrastructure  |
| **WordPress Management** | WordPress site operations, deployment, plugins, themes, synchronization            | Editorial meaning         |
| **Research**             | Research process and findings                                                      | Editorial publication state |
| **Evidence Ledger**      | Research-source provenance                                                         | Publishing workflow       |
| **Professional Review**  | Professional determinations                                                        | Editorial ownership       |
| **Policy / Compliance**  | Publishing requirements and restrictions                                           | Editorial writing         |
| **Consent & Access**     | Use of protected source material                                                   | Editorial content itself  |
| **Identity & Access**    | Editorial user identity and platform roles                                         | Content workflow          |
| **Workflow**             | Scheduling and execution mechanics                                                 | Editorial judgment        |
| **Notification**         | Editorial alerts and reminders                                                     | Publishing logic          |
| **Audit / Provenance**   | Publication activity history                                                       | Current article state     |

---

# 44. One-Sentence Definition

> **The Blog / Publishing Engine turns approved ideas, evidence, and expertise into versioned public content while maintaining a hard boundary between public publishing and the platform's private information.**
