This should be the **private foundation** of the platform. The Consent & Access Engine decides who may see something; the Local Vault decides **where the underlying information lives and how it is handled before disclosure**.

The most important architectural rule is:

> **The Local Vault is the owner's private workspace, not a cloud drive with a local cache.**

# Local Vault / Workspace Engine

## 1. Purpose

The Local Vault / Workspace Engine provides the user's private working environment for business and transaction information.

It manages:

* Local documents
* Encrypted storage
* File indexing
* Local search
* Local document analysis
* Versioning
* Offline operation
* Synchronization
* Explicit external sharing
* Local workspace organization
* Private derived data
* Local processing state

Its primary purpose is to allow the owner to use the platform without making sensitive business information automatically leave their computer.

The engine should make the user's computer a **first-class source of truth for private information**, while the cloud platform becomes the coordination and workflow layer.

---

# 2. Core Architectural Principle

## Private by default.

A document placed into the Local Vault remains local unless a defined sharing action explicitly permits transmission.

The system must never assume:

> “The platform needs it, therefore upload it.”

Instead:

> “The platform needs it. Is the information available locally, and has the owner authorized disclosure?”

This creates the fundamental boundary:

**Local Vault = possession**

**Consent & Access = permission**

**Transmission = controlled disclosure**

---

# 3. The Vault Is Not Just File Storage

The Vault should be treated as a private workspace rather than a folder manager.

It needs to understand:

* Documents
* Facts extracted from documents
* Document relationships
* Versions
* Source provenance
* Local analysis
* User annotations
* Business entities
* Transactions
* Scenarios
* Professional requests
* Sharing state

For example, the owner might have:

```text
2025 Tax Return.pdf
2025 P&L.xlsx
2025 Balance Sheet.xlsx
Employee Roster.xlsx
Customer Contracts/
Valuation/
Ownership/
Debt/
Legal/
```

The Vault should understand that these are not merely files.

They are potentially related to:

> **Business Reality → Scenario → Professional Review → Financing**

without needing to upload the underlying files to the cloud.

---

# 4. Local-First Does Not Mean Local-Only

The architecture should support three operating modes.

### Fully Local

The owner can:

* inspect documents
* search
* organize
* analyze
* compare versions
* review extracted facts
* create scenarios
* work offline

without transmitting source documents.

### Hybrid

The cloud platform stores:

* workflow state
* approved metadata
* package definitions
* tasks
* professional assignments
* non-sensitive identifiers
* synchronization state

while underlying sensitive documents remain local.

### Explicitly Shared

A specific document, fact, package, or derived result is transmitted because the owner authorized it.

The product should make the transition between these modes visible.

---

# 5. Vault Boundary

The Vault should have a hard boundary around private information.

Conceptually:

```text
                 PLATFORM
        ┌──────────────────────────┐
        │ Journey                  │
        │ Destination              │
        │ Business Reality         │
        │ Scenarios                │
        │ Professionals            │
        │ Workflow                 │
        │ Marketplace              │
        └────────────┬─────────────┘
                     │
              Consent & Access
                     │
              Explicit Sharing
                     │
        ┌────────────▼─────────────┐
        │     LOCAL VAULT          │
        │                          │
        │ Documents                │
        │ Private Facts            │
        │ Local Analysis           │
        │ Versions                 │
        │ Search Index             │
        │ Encryption               │
        └──────────────────────────┘
```

The cloud should not become the implicit owner of the source records.

---

# 6. Vault Ownership

Every Vault belongs to a principal.

Usually:

> Owner → Business → Workspace

But the model should support:

* Individual owner
* Multiple owners
* Holding company
* Operating company
* Trust
* Authorized business representative
* Professional workspace

The vault should not assume that the individual who installed the application has unlimited authority over every business.

Identity & Access determines **who the person is and what authority they have**.

The Vault determines **what private material that authorized workspace contains**.

---

# 7. Workspace Structure

The user should not have to understand the underlying database.

The interface can organize information around the journey:

### Business

Current business information.

### Financial

Statements, tax records, debt, payroll-related documents.

### People

Ownership, management, employees, advisors.

### Legal & Contracts

Leases, customer agreements, vendor agreements, corporate records.

### Ownership Transition

Valuations, scenarios, transaction materials.

### Financing

Lender materials, debt schedules, capital plans.

### Professional Review

Professional packages and supporting documents.

### Research

Research outputs and evidence saved to the workspace.

The physical storage structure can differ from the visible organization.

---

# 8. Encryption

Encryption is a foundational Vault capability.

The architecture should support:

### Encryption at rest

Private Vault data is encrypted on the user's device.

### Encryption in transit

Any approved transmission is encrypted while moving between systems.

### Key separation

The system should avoid making the cloud platform's ordinary application database a universal decryption key for private files.

Where technically practical, encryption keys or key-encryption material should be structured so that compromising ordinary application metadata does not automatically expose the Vault.

### Device security dependency

The product should clearly distinguish:

> Platform protection

from:

> Security of the user's own device.

A compromised computer can undermine local security regardless of application design.

---

# 9. Encryption Model

We should design this as a layered system.

Conceptually:

```text
User Authentication
        ↓
Workspace Authorization
        ↓
Vault Key
        ↓
Document Encryption Key
        ↓
Encrypted Document
```

This gives us the ability to protect individual resources and rotate keys without redesigning the entire system.

The exact cryptographic implementation should use established, audited primitives rather than custom encryption.

---

# 10. File Ingestion

Files can enter the Vault through:

* Drag and drop
* File picker
* Folder import
* Scanner
* Download/import
* Integration
* Watched folder
* Professional request
* Existing business system

Import should not automatically share anything.

When a document enters the Vault:

> **Imported**

does not mean:

> **Uploaded**

and does not mean:

> **Shared**

---

# 11. Document Intelligence at the Edge

The Vault should be able to invoke the Document Intelligence / Extraction Engine locally whenever possible.

Example:

User imports:

> 2025 FinancialStatements.xlsx

Local processing identifies:

* Revenue
* EBITDA
* Debt
* Cash
* Reporting period
* Worksheet names
* Formula relationships

These become **candidate facts**.

The original file remains local.

The extracted information can be sent to Business Reality only according to the broader data policy and user authorization.

---

# 12. Candidate Fact Boundary

Local extraction must not silently become platform truth.

The Vault can produce:

> Candidate Fact:
> Revenue = $8,240,000
> Source = 2025 P&L.xlsx
> Cell = B17

Business Reality can then record:

> Extracted

The owner may later verify:

> Owner Verified

A professional may eventually establish:

> Professionally Reviewed

This maintains the existing distinction between source material and validated business reality.

---

# 13. File Provenance

Every document should retain:

* Original filename
* Current filename
* Source location
* Import date
* Creation date when available
* Modified date when available
* Hash/checksum
* File type
* Size
* Version
* Source application when known
* Extraction status
* Analysis history

For structured files, retain:

* Worksheet
* Cell/range
* Header
* Unit
* Formula/value distinction

For PDFs:

* Page
* section
* extraction location

For Word documents:

* section
* heading
* paragraph location

The system should be able to answer:

> **“Where did this fact come from?”**

without the owner having to hunt through the file manually.

---

# 14. File Versioning

Versioning is mandatory.

The system should never silently replace important business records.

Example:

```text
Financial Statements
   v1  2024-12-31
   v2  2025-03-01
   v3  2025-06-15
   v4  2025-09-19
```

The user should be able to:

* Compare versions
* Restore a previous version
* Identify current version
* Mark a version superseded
* See who created a version
* See what changed

Versioning should distinguish:

**File version**

from:

**Business fact version**

because a revised file may alter several facts simultaneously.

---

# 15. Immutable Source Preservation

Original source documents should be preserved.

When a document is imported, the system should maintain an immutable original or cryptographically verifiable representation.

Subsequent changes should create new versions rather than modifying the original source invisibly.

This is especially important for:

* Tax filings
* Financial statements
* Contracts
* Valuations
* Financing documents
* Corporate records

---

# 16. Local Search

Search should operate locally first.

The user should be able to search:

> “2025 EBITDA”

and receive:

* Relevant documents
* Relevant pages
* Relevant spreadsheet cells
* Extracted facts
* Related scenarios
* Professional requests

without needing to upload the documents to a cloud search index.

Search can support:

### Filename search

### Full-text search

### Metadata search

### Semantic search

### Structured fact search

### Date / period search

### Entity search

For example:

> “All documents mentioning ABC Bank.”

---

# 17. Semantic Search

Local semantic search could eventually use a local embedding/indexing process.

The important architectural principle is:

> **The search representation should not automatically become another cloud copy of the sensitive source material.**

Embeddings themselves can leak information depending on implementation and threat model, so the system should treat local embeddings as protected workspace data rather than assuming they are harmless metadata.

---

# 18. Offline Operation

The platform should remain useful without an internet connection.

Offline users should be able to:

* Open local files
* Search local files
* Review extracted information
* Make annotations
* Create or edit local facts
* Compare versions
* Review scenarios that do not require remote services
* Prepare sharing packages
* Continue journey work supported by locally cached state

The UI should clearly distinguish:

> **Available offline**

from:

> **Requires connection**

The system should not simply break when connectivity disappears.

---

# 19. Offline Changes

Offline changes need a synchronization model.

Each local change should receive:

* Device ID
* Workspace ID
* Timestamp
* Local sequence number
* Object version
* Previous version reference

When connectivity returns, the Sync Engine reconciles changes.

---

# 20. Conflict Resolution

Synchronization conflicts should be first-class objects.

Example:

Laptop:

> Revenue = $8.2M

Desktop:

> Revenue = $8.5M

The system should not simply choose the newest timestamp.

It should show:

> **Sync Conflict**
>
> Revenue was changed in two locations.
>
> Laptop: $8.2M
>
> Desktop: $8.5M
>
> [Keep Laptop]
> [Keep Desktop]
> [Review Differences]
> [I'm Not Sure]

This follows the same philosophy as Business Reality conflicts.

---

# 21. Sync Architecture

Synchronization should be selective.

Not everything in the Vault needs to synchronize to the cloud.

We can divide data into:

### Local-only

Never synchronized unless explicitly shared.

### Syncable metadata

Safe enough and necessary for workflow.

### User-authorized synchronized content

Explicitly approved for cloud availability.

### Shared external content

Content disclosed to a specific recipient under Consent & Access.

This becomes an important storage classification system.

---

# 22. Selective Sync

A user might have:

> 40 GB of business documents

but only need:

> 200 MB of active transaction material

on another device.

The Vault should support selective synchronization.

Examples:

> “Sync Financial folder to laptop.”

> “Keep HR folder local only.”

> “Sync current transaction package.”

This avoids turning every device into a complete copy of the business.

---

# 23. Explicit Sharing

Sharing should be a deliberate action.

A user can select:

> **Share**

The system then passes the request to Consent & Access.

The Vault should not make the authorization decision itself.

Conceptually:

```text
User selects document
        ↓
Local Vault
        ↓
Consent & Access
        ↓
Owner approval / existing authorization
        ↓
Approved transmission
        ↓
Share selected version
```

---

# 24. Sharing Should Be Version-Specific

A user should know:

> You are sharing Financial Statements v4.

not:

> Financial Statements.

That distinction matters.

The owner might update the local file tomorrow without intending to disclose the new information.

---

# 25. Share a Copy, Not the Private Master

External recipients should generally receive an authorized representation or copy of the selected content.

The recipient should not receive uncontrolled access to the Vault itself.

Therefore:

**Vault access ≠ folder sharing**

The Vault remains private.

The platform creates a controlled disclosure.

---

# 26. Local Analysis

A major product advantage can come from performing sensitive analysis locally.

Examples:

> “Analyze my five years of financial statements.”

> “Find unusual changes in gross margin.”

> “Compare revenue growth against my destination.”

> “Identify missing transaction documents.”

These tasks can potentially be performed locally using a local model or local processing pipeline.

The system can return results without transmitting the source documents.

---

# 27. Cloud AI Boundary

When a cloud AI service is needed, the system should explicitly determine what is being sent.

For example:

### Level 1

Send only a question.

### Level 2

Send anonymized/abstracted structured information.

### Level 3

Send selected facts.

### Level 4

Send selected document excerpts.

### Level 5

Send an authorized document.

The user should know which category applies.

---

# 28. AI Processing Consent

The same Consent & Access principles should apply to AI.

Example:

> **This analysis uses an external AI service.**
>
> Information to be sent:
>
> * 2025 revenue
> * EBITDA
> * industry
>
> Documents:
> None
>
> Customer names:
> No
>
> Employee information:
> No

Then:

**Approve**

This turns AI privacy into an understandable decision rather than a hidden implementation detail.

---

# 29. Workspace Snapshots

The Vault should eventually support snapshots.

Example:

> **Transaction Snapshot**
>
> Created September 19, 2026
>
> Includes:
>
> * Destination v3
> * Business Reality v7
> * Scenario Set v4
> * Financial Model v2
> * Professional Package v5
> * 18 source documents

A snapshot creates a reproducible state for review.

This can become very valuable for professional review and transaction history.

---

# 30. Workspace Branching

The architecture should allow controlled branching.

For example:

> Scenario A documents

and:

> Scenario B documents

may use the same underlying source information without copying everything unnecessarily.

The model should support references rather than duplicate files wherever practical.

---

# 31. Document Collections

Users should be able to create logical collections.

Examples:

**CPA Review**

**Attorney Review**

**Lender Package**

**Valuation Package**

**ESOP Package**

These collections are not permissions.

They are organizational constructs.

Consent & Access still determines whether the collection may be disclosed.

---

# 32. Retention

The Vault should allow lifecycle policies for:

* Active files
* Superseded files
* Archived files
* Temporary files
* Generated outputs
* Shared copies
* Deleted files

Deletion should be deliberate.

For important records, the system should distinguish:

> Remove from active workspace

from:

> Permanently delete.

Legal or professional retention requirements, where applicable, should be handled through the Policy / Compliance Engine rather than hard-coded into the Vault.

---

# 33. Trash and Recovery

Accidental deletion should be recoverable.

The system should support:

> Trash → Restore

with retention settings.

Important files should not disappear because of a single mistaken click.

---

# 34. Local Device Management

The engine should understand multiple devices.

Example:

```text
Stephen's MacBook
Active
Last sync: Today

Office Desktop
Active
Last sync: Yesterday

Backup Drive
Encrypted
Last verified: Sept 10
```

The user should be able to revoke a device.

When a device is revoked:

* synchronization stops
* future credentials are invalidated
* cached authorization should expire according to policy
* the event is recorded

---

# 35. Backup

The Vault should support encrypted backups.

Potential destinations:

* External encrypted drive
* User-controlled cloud storage
* Managed backup service
* Secondary computer

The architecture should distinguish:

**Backup**

from:

**Sync**

A backup is not another active workspace.

---

# 36. Recovery

The owner needs a recovery path when:

* device is lost
* hard drive fails
* application is reinstalled
* encryption key is unavailable
* account credentials change

Recovery should be deliberately designed because a strong local-security architecture creates a corresponding key-management risk:

> **The better we protect the vault, the more carefully we must design legitimate recovery.**

There should never be a hidden universal backdoor.

---

# 37. Health and Integrity Checks

The Vault should periodically verify:

* File integrity
* Encryption status
* Index health
* Database consistency
* Version chain integrity
* Backup status
* Sync status
* Key availability

The user should get a simple status:

> **Vault Health: Good**

and be able to inspect details.

---

# 38. Local Workspace Database

The local application will need a protected local database containing metadata such as:

* File identities
* Versions
* Tags
* Relationships
* Extraction results
* Candidate facts
* Search index references
* Local analysis results
* Sharing state
* Sync state
* Device state

This database itself should be treated as sensitive.

It may contain information about the business even when the original documents are encrypted separately.

---

# 39. Cloud Metadata Minimization

The cloud should store only the minimum information required to operate the platform.

For example, the cloud may know:

> `Financial Statements 2025`
> Available locally: Yes
> Last local version: v4
> Shared with: CPA
> Share expires: October 31

without possessing the underlying financial statement.

The cloud should not need the file simply to coordinate its workflow.

---

# 40. Local-to-Cloud Fact Promotion

An important future concept is **promotion**.

A local fact begins as:

> Private Local Fact

It can become:

> Workspace Fact

Then, when authorized:

> Platform Fact

Then potentially:

> Shared Professional Fact

The same underlying value can therefore move through controlled trust boundaries.

Example:

```text
Local document
      ↓
Local extraction
      ↓
Candidate local fact
      ↓
Owner verification
      ↓
Business Reality
      ↓
Professional package
      ↓
Authorized disclosure
```

This is extremely powerful because the entire business does not have to be uploaded merely to get the benefits of structured analysis.

---

# 41. Local Vault and Evidence Ledger

The Evidence Ledger should record provenance for research findings.

The Vault stores the source material and associated local artifacts.

Example:

> Research finding references 2025 financial data.

The Evidence Ledger can say:

> Source: Owner's 2025 P&L
> Local source identifier: FIN-2025-004

without requiring the cloud research system to contain the complete P&L.

---

# 42. Local Vault and Business Reality

Business Reality should never assume:

> “Because a document exists in the Vault, its contents are confirmed.”

Instead:

```text
Vault
  ↓
Extraction
  ↓
Candidate Fact
  ↓
Verification / Conflict
  ↓
Business Reality
```

This preserves the distinction between:

**file storage**

and:

**business truth**.

---

# 43. Local Vault and Document Readiness

Document Readiness can say:

> 2025 Tax Return: Available Locally

without receiving the tax return.

This is an important local-first workflow pattern.

The platform can therefore manage:

> What exists

without automatically receiving:

> What it contains.

---

# 44. Local Vault and Consent

Consent & Access is the gatekeeper for every external disclosure.

The Vault supplies the requested material only after authorization.

The Vault itself should not contain business logic that says:

> “CPAs can always see tax returns.”

That belongs in permission and policy layers.

---

# 45. Local Vault and Audit

The Audit / Provenance Engine should receive events such as:

> Document imported

> Version created

> Document analyzed

> Document shared

> Document restored

> Device revoked

> Backup completed

The Vault owns the underlying operation.

Audit owns the historical record.

---

# 46. Local Vault and Professional Packages

The Professional Review Package Engine can request:

> Build Legal Review Package v4.

The package engine identifies relevant materials.

The Vault finds the local sources.

Consent & Access determines what may be disclosed.

The Vault then packages or transmits the authorized material.

Again:

**Package selection ≠ access**

**Access ≠ storage**

**Storage ≠ transmission**

---

# 47. Core Vault Objects

## Vault

Contains:

* vault_id
* owner/principal
* workspace
* devices
* encryption state
* sync configuration
* health state

## Document

Contains:

* document_id
* vault_id
* local path/reference
* filename
* type
* size
* hash
* sensitivity
* status
* current_version_id

## DocumentVersion

Contains:

* version_id
* document_id
* created_at
* source_hash
* local location
* supersedes_version
* extraction state
* sharing state

## LocalFact

Contains:

* fact_id
* value
* source_document
* source_location
* extraction method
* confidence
* verification state
* created/updated timestamps

## WorkspaceCollection

Logical group of documents and related resources.

## SyncRecord

Contains:

* object
* local version
* remote version
* device
* status
* conflict state

## LocalAnalysis

Contains:

* analysis_id
* inputs
* model/tool
* execution timestamp
* output
* dependencies
* whether external services were used

---

# 48. Vault States

A document can have states such as:

**Imported**

**Indexed**

**Analyzed**

**Verified**

**Current**

**Superseded**

**Archived**

**Prepared for Sharing**

**Shared**

**Locally Restricted**

**Deleted / Recoverable**

These states should not be confused with access permissions.

---

# 49. Explicit Sharing UX

The key interaction should remain extremely simple.

When the user chooses **Share**, show:

> **Share Financial Statements v4**
>
> **Recipient:** ABC Bank
> **Purpose:** Financing review
> **What they will receive:** Financial Statements v4
> **Access:** View
> **Duration:** 30 days
> **Sensitive data:** Yes
>
> [Review & Share]
> [Cancel]

A link to:

> **See exactly what is included**

can reveal page, worksheet, or field-level detail.

---

# 50. Local-First Failure Modes

The product should be designed for failure rather than pretending systems never fail.

### Internet unavailable

Local work continues.

### Cloud unavailable

Local vault remains accessible.

### Sync conflict

User resolves it.

### Corrupted file

Integrity check identifies the problem.

### Failed backup

User receives a warning.

### Expired access

External sharing stops.

### Lost device

Device authorization can be revoked.

### Broken local index

The Vault can rebuild it from the underlying encrypted data.

### AI service unavailable

Local analysis remains available where supported.

This is a disaster-planning mindset applied to software architecture.

---

# 51. Privacy Modes

The user should eventually be able to choose a workspace privacy posture.

### Local Maximum

Almost everything remains local unless explicitly shared.

### Balanced

Workflow metadata synchronizes normally; sensitive documents stay local by default.

### Collaborative

Selected transaction documents may synchronize to the cloud for ongoing collaboration.

The platform should clearly explain the consequences of each mode.

These modes should never bypass explicit authorization requirements.

---

# 52. Security Events

The Vault should detect and report unusual events where technically feasible.

Examples:

* New device
* Unexpected bulk export
* Repeated failed unlock attempts
* Encryption configuration change
* Large-scale deletion
* Unusual synchronization activity

These events can feed the Notification and Audit engines.

---

# 53. Engine Contract

Other engines should interact with the Vault through defined interfaces.

Examples:

```text
getDocument()
getDocumentVersion()
searchLocal()
getDocumentMetadata()
getCandidateFacts()
createLocalVersion()
prepareForSharing()
exportAuthorizedContent()
syncMetadata()
restoreVersion()
```

The Vault should not expose unrestricted filesystem access to every other engine.

---

# 54. Sharing Contract

The central boundary should look approximately like:

```text
requestShare(resource, recipient, purpose)
        ↓
Consent & Access
        ↓
authorization decision
        ↓
Local Vault
        ↓
retrieve authorized version
        ↓
Transmission Layer
        ↓
Disclosure Record
```

No engine should be able to bypass that sequence for convenience.

---

# 55. What the Vault Should Never Do

It should never:

* Automatically upload private documents
* Automatically share an entire folder
* Treat possession as authorization
* Treat extraction as verification
* Silently overwrite source documents
* Silently resolve conflicting versions
* Allow another engine unrestricted filesystem access
* Give AI agents unrestricted access to the workspace
* Automatically inherit external permissions from a parent folder
* Promise that revocation retrieves previously copied information

---

# 56. Minimum Viable Local Vault

The first usable version needs:

**1. Encrypted local workspace**

**2. Document import**

**3. File indexing**

**4. Local search**

**5. Document metadata**

**6. Versioning**

**7. Basic local analysis**

**8. Offline operation**

**9. Explicit sharing workflow**

**10. Consent & Access integration**

**11. Basic encrypted backup**

**12. Sync/conflict framework**

The sophisticated pieces such as semantic search, local AI orchestration, device trust, selective sync, advanced snapshots, and branching can evolve independently.

---

# 57. Architectural Lock

The Local Vault / Workspace Engine should be treated as the platform's **private information boundary**.

The following should be locked:

**1. Private documents remain local by default.**

**2. Importing a document does not upload it.**

**3. Possession does not equal permission.**

**4. The Vault and Consent & Access are separate engines.**

**5. Every externally shared document is explicitly authorized.**

**6. Sharing is version-specific.**

**7. Source documents are preserved and versioned.**

**8. Local analysis is supported without requiring cloud upload where technically feasible.**

**9. Offline operation is a supported operating mode, not an exception.**

**10. Synchronization is selective and conflict-aware.**

**11. Local search does not require cloud indexing.**

**12. Local derived facts retain provenance back to their source material.**

**13. Extraction does not equal verification.**

**14. Cloud metadata should be minimized.**

**15. External AI and integrations are treated as disclosure destinations and subject to authorization.**

**16. The private master remains in the Vault even when an external copy is shared.**

**17. Backup and synchronization are distinct functions.**

**18. Revocation controls future platform-controlled access but does not pretend to retract copies already disclosed.**

**19. No other engine receives unrestricted Vault access.**

**20. The Vault must remain independently replaceable and evolvable without becoming the owner of the entire transaction.**

---

# 58. The Bigger Architecture

This gives us an increasingly clean separation:

```text
                 OWNER
                   │
                   ▼
            ┌───────────────┐
            │   DESTINATION │
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │ BUSINESS      │
            │ REALITY       │
            └───────┬───────┘
                    │
          ┌─────────┴──────────┐
          ▼                    ▼
   LOCAL VAULT             RESEARCH
          │                    │
          │                    ▼
          │              EVIDENCE LEDGER
          │
          ▼
   DOCUMENT / FACT
      ANALYSIS
          │
          ▼
       SCENARIOS
          │
          ▼
 PROFESSIONAL REVIEW
          │
          ▼
 PROFESSIONAL PACKAGE
          │
          ▼
   CONSENT & ACCESS
          │
          ▼
 EXPLICIT DISCLOSURE
```

And the critical boundary is:

> **The owner can use the intelligence of the platform without surrendering possession of the underlying business records.**

That is not just a security feature. It is a **product architecture decision** that can shape the entire platform.

---

## The strongest conceptual distinction

We now have two engines that should always be discussed together but **never merged**:

### Local Vault / Workspace

> **“What do I privately possess and work with?”**

### Consent & Access

> **“Who am I allowing to see it, what exactly are they allowed to see, and for how long?”**

That separation gives us a very clean local-first architecture and prevents the platform from quietly turning into a centralized repository of someone's entire business.

This one is especially important to lock early because nearly every later engine touches it, but none of them should be allowed to absorb its responsibilities.
