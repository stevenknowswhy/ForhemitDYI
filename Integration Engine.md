Yes. This is the **bridge layer** between our platform and the outside world.

The central rule should be:

> **The Integration Engine connects systems. It does not become the system of record for the information flowing through them.**

That keeps QuickBooks, payroll, banks, e-signature, calendars, email, lender portals, and professional systems as replaceable external connections rather than allowing any one integration to take over the platform architecture.

# Integration Engine

## 1. Purpose

The Integration Engine provides the controlled infrastructure for connecting the platform to external systems and services.

It manages:

* External system connections
* Authentication to external systems
* API credentials and OAuth connections
* Data import
* Data export
* Synchronization
* Mapping
* Transformation
* Connection health
* Error handling
* Retry behavior
* Webhooks
* Scheduled synchronization
* Manual synchronization
* Integration permissions
* Field/data mapping
* External system identifiers
* Integration versioning
* Connection lifecycle

Its central question is:

> **“How does our platform exchange authorized information with another system without allowing that system to become the owner of our internal architecture?”**

---

# 2. Core Architectural Principle

## Connect, don't absorb.

The Integration Engine should allow:

> QuickBooks → Business Reality

without making QuickBooks the Business Reality system.

It should allow:

> Bank → Capital / Financing

without making the bank the financing engine.

It should allow:

> Calendar → Workflow

without making Google Calendar the Workflow Engine.

It should allow:

> E-signature → Closing / Transaction

without making DocuSign or another provider the Transaction Engine.

The integration moves information.

The receiving engine interprets and owns it.

---

# 3. What This Engine Owns

The Integration Engine owns:

* Connections
* Connectors
* Authentication credentials/references
* OAuth authorization state
* External account links
* External object identifiers
* Data mappings
* Synchronization rules
* Import/export jobs
* Webhook subscriptions
* Sync schedules
* Retry policies
* Integration errors
* Connection health
* Integration permissions
* Transformation pipelines
* Sync history
* Integration version compatibility

---

# 4. What It Does Not Own

It does not own:

* Business truth
* Financial truth
* Payroll truth
* Bank account truth
* Professional determinations
* Documents as a system of record
* Transaction state
* Workflow state
* Consent policy
* Identity
* Legal agreements
* Communication history

The external system remains authoritative for its own domain where appropriate.

The internal platform engine remains authoritative for the internal domain.

---

# 5. Integration vs Import

These should be separate concepts.

### Import

> Bring information from an external system into the platform.

### Integration

> Maintain an ongoing controlled relationship between two systems.

Example:

> Import 2025 financial statements once.

versus:

> Maintain a QuickBooks connection that periodically retrieves updated financial information.

---

# 6. Integration vs Synchronization

Also distinct.

### Integration

The overall connection.

### Synchronization

A specific exchange of information.

Example:

> QuickBooks connection exists.

Then:

> Financial data synchronized at 9:00 AM.

A single integration can produce many synchronization events.

---

# 7. Connector Architecture

External systems should be accessed through adapters/connectors.

Conceptually:

```text
                    INTEGRATION ENGINE
                           │
            ┌──────────────┼───────────────┐
            ▼              ▼               ▼
       Accounting       Banking         Calendar
        Adapter          Adapter          Adapter
            │              │               │
            ▼              ▼               ▼
       External          External        External
        System            System          System
```

The core platform should not contain provider-specific logic everywhere.

---

# 8. Provider Independence

A connector should be replaceable.

For example:

> Accounting Adapter

might support:

* QuickBooks
* Xero
* NetSuite
* Other systems later

The Business Reality Engine should consume a normalized interface rather than knowing how every accounting provider works.

---

# 9. Connector Types

The engine should support:

### API Connector

Direct API communication.

### OAuth Connector

User-authorized external account.

### File Connector

CSV, Excel, PDF, XML, JSON, etc.

### Webhook Connector

External system pushes events.

### Email Connector

Inbound/outbound email integration.

### SFTP / Secure File Connector

For systems that exchange files.

### Manual Connector

User-triggered import/export.

### Browser / Portal Adapter

Potential later capability where legitimate API access does not exist, subject to provider terms and security requirements.

---

# 10. Connection Object

A Connection should contain:

* Connection ID
* Connector type
* Provider
* External account
* Organization/workspace
* Authentication state
* Scope
* Authorized capabilities
* Created date
* Last sync
* Health state
* Expiration
* Owner
* Status
* Connector version

---

# 11. Connection States

A connection can be:

**Available**

**Authorization Required**

**Active**

**Syncing**

**Needs Attention**

**Expired**

**Disconnected**

**Revoked**

**Failed**

**Archived**

The platform should make connection health visible.

---

# 12. Authentication

External integrations may use:

* OAuth
* API keys
* Service credentials
* Certificates
* Signed requests
* User sessions
* Secure file exchange credentials

Credentials should be scoped and stored securely.

The Integration Engine should never treat an external credential as unlimited authority.

---

# 13. Credential Separation

A user's platform authentication should remain separate from an external-system credential.

Example:

> User is authenticated to our platform.

does not mean:

> User is authenticated to QuickBooks.

The Integration Engine manages the second relationship.

---

# 14. Integration Permissions

An integration should request only what it needs.

Example:

Accounting integration requests:

> Read financial data.

rather than:

> Full accounting-account administrator access.

Banking integration may request:

> Read-only account transactions.

rather than:

> Funds transfer capability.

Least privilege should be the default.

---

# 15. Read vs Write

Integration permissions should distinguish:

**Read**

**Create**

**Update**

**Delete**

**Initiate**

**Approve**

**Execute**

The platform should prefer read-only integration when write access is unnecessary.

---

# 16. Protected Actions

Some external actions should require explicit human approval.

Examples:

* Initiating bank transfer
* Sending binding document
* Executing e-signature
* Sending external email
* Modifying accounting records
* Changing payroll
* Submitting a lender application

The Integration Engine can facilitate the action.

It should not silently execute high-impact actions.

Policy / Compliance and relevant approval engines govern those control points.

---

# 17. Accounting Integrations

Potential uses:

* Revenue
* Expenses
* General ledger
* Accounts receivable
* Accounts payable
* Financial statements
* Chart of accounts
* Historical financial data

The Accounting system may be the source of record for accounting data.

Business Reality determines how imported information becomes a candidate or verified business fact.

---

# 18. Accounting Data Flow

Example:

```text
Accounting System
       ↓
Integration Engine
       ↓
Imported Financial Dataset
       ↓
Document / Data Intelligence
       ↓
Candidate Facts
       ↓
Fact Verification / Conflict
       ↓
Business Reality
```

This prevents external data from automatically becoming unquestioned platform truth.

---

# 19. Payroll Integrations

Potential uses:

* Employee count
* Compensation
* Benefits
* Payroll totals
* Historical payroll data
* Employer taxes

Payroll data can be highly sensitive.

The integration should therefore support:

* Field minimization
* Role restrictions
* Local processing where appropriate
* Explicit authorization
* Purpose limitations

---

# 20. Banking Integrations

Potential uses:

* Account balances
* Transactions
* Debt
* Cash position
* Payment history
* Financing information

Bank connections should normally be read-only unless there is an explicit, separately governed need for write capability.

The Integration Engine should not independently make financial decisions.

---

# 21. E-Signature Integrations

Potential uses:

* Send document for signature
* Track envelope status
* Receive signature status
* Record completion
* Retrieve signed-document reference

The integration should distinguish:

> Document sent

> Recipient viewed

> Recipient signed

> All signatures complete

The definitive legal significance of the signed document remains outside the Integration Engine.

---

# 22. Calendar Integrations

Potential uses:

* Meetings
* Deadlines
* Professional appointments
* Closing dates
* Review sessions
* Reminders

Calendar integration can receive events from Workflow and Transaction Orchestration.

Example:

> Closing target updated.

→ Workflow recalculates deadlines.

→ Calendar integration offers or updates relevant calendar events.

The calendar remains the calendar.

---

# 23. Email Integrations

Email integration may support:

* Send message
* Receive message
* Import relevant communication
* Link email to transaction
* Create request from email
* Create question from email
* Archive reference

Communication remains the source of structured platform conversation.

Email is an external communication channel.

---

# 24. Email Capture

A valuable use case:

> Attorney sends an email requesting updated financial statements.

The owner can:

> **Capture as Request**

Then:

Communication:

> Request recorded.

Document Readiness:

> Financial statement requirement created.

Workflow:

> Owner task created.

Integration:

> Email preserved as external-source reference.

---

# 25. Document-System Integrations

Potential systems:

* SharePoint
* Google Drive
* OneDrive
* Dropbox
* Box
* Secure data rooms
* Document-management systems

The integration should distinguish:

> External document reference

from:

> Local Vault copy

and:

> Platform-authorized disclosure.

The existence of an external connection must not automatically give the platform permission to ingest everything.

---

# 26. Data Rooms

Later-stage transactions may use external data rooms.

The integration could:

* Track requested files
* Identify availability
* Synchronize approved metadata
* Retrieve authorized documents
* Track upload status

But the Local Vault remains the owner's private workspace unless the owner deliberately chooses another operating model.

---

# 27. Lender-System Integrations

Potential capabilities:

* Financing application status
* Document request status
* Lender messages
* Indicative terms
* Commitment status
* Closing requirements
* Funding status

The Capital / Financing Engine remains authoritative for the platform's financing model and workflow.

The lender remains authoritative for its own external status.

Integration bridges the two.

---

# 28. Professional-System Integrations

Potential systems could include:

* Professional document systems
* Practice-management platforms
* Tax systems
* Legal systems
* Valuation systems
* Trustee systems

The platform should import relevant outputs without turning those providers into internal source-of-truth engines.

---

# 29. Integration Mapping

External systems use their own schemas.

Example:

External:

> `annual_revenue`

Platform:

> `revenue`

Mapping rules should be explicit.

They should include:

* External field
* Internal field
* Transformation
* Units
* Period
* Definition
* Source system
* Mapping version

---

# 30. Transformation

Integration may need to transform data.

Examples:

> cents → dollars

> external date format → ISO date

> account categories → normalized platform categories

> vendor employee IDs → internal stakeholder references

Transformation logic should be versioned.

---

# 31. Never Hide Unit Conversion

Example:

External system:

> Revenue = 8,240,000 USD

Platform:

> Revenue = $8.24M

The transformation should retain:

> Original value

> Original unit

> Converted value

> Conversion rule

This allows auditability.

---

# 32. Data Freshness

Every imported data object should have:

* Source system
* Retrieved timestamp
* Source timestamp
* Reporting period
* Refresh status
* Data age
* Mapping version

This integrates with Business Reality and Confidence.

---

# 33. Sync Frequency

Synchronization can be:

* Real-time
* Near real-time
* Hourly
* Daily
* Weekly
* Manual
* Event-driven

The appropriate frequency depends on the data type.

---

# 34. Manual Sync

Users should be able to request:

> **Sync now**

The system should display:

> Last synchronized: 10:42 AM.

This is particularly helpful when a transaction is moving quickly.

---

# 35. Webhooks

Where supported, external systems can send events.

Example:

> E-signature completed.

→ Integration Engine receives webhook.

→ validates source.

→ emits:

`SignatureCompleted`

→ Workflow responds.

This allows the platform to react quickly without constantly polling.

---

# 36. Webhook Security

Incoming external events should be validated using mechanisms such as:

* Signature verification
* Provider authentication
* Replay protection
* Timestamp validation
* Known source verification
* Idempotency

The platform should never trust an arbitrary external request simply because it resembles a known webhook.

---

# 37. Idempotency

The same external event may arrive more than once.

Example:

> `EnvelopeCompleted`

received twice.

The Integration Engine should not create two completion events.

External event identifiers should be retained.

---

# 38. Sync Conflicts

External and internal values may disagree.

Example:

Accounting system:

> Revenue = $8.24M

Business Reality:

> Revenue = $8.31M

Integration should not silently overwrite Business Reality.

Instead:

> **External/Internal Conflict**

with:

* Source
* Values
* Period
* Retrieval date
* Mapping
* Related fact

Fact Verification / Conflict handles resolution.

---

# 39. External Source Authority

The platform should know where a particular value originated.

Example:

> Revenue
> Source: QuickBooks
> Imported: Sept 19

Another:

> Revenue
> Source: Tax return
> Imported: Sept 18

Another:

> Revenue
> Owner reported

This provides the provenance necessary for Business Reality to reconcile information.

---

# 40. External System Change Detection

If an external source changes a value:

> Revenue changed from $8.24M to $8.51M.

Integration should generate:

> ExternalDataChanged

rather than directly modifying every dependent platform object.

Workflow then propagates consequences.

---

# 41. Integration Event Model

Events can include:

```text
ConnectionCreated
ConnectionAuthorized
ConnectionExpired
ConnectionRevoked
SyncStarted
SyncCompleted
SyncFailed
ExternalObjectCreated
ExternalObjectChanged
ExternalObjectDeleted
ExternalRequestReceived
ExternalResponseReceived
WebhookReceived
MappingChanged
```

These can feed Workflow and Audit.

---

# 42. Data Import Pipeline

A standard import should be:

```text
EXTERNAL SYSTEM
      ↓
AUTHENTICATED CONNECTION
      ↓
INTEGRATION ADAPTER
      ↓
VALIDATION
      ↓
TRANSFORMATION
      ↓
NORMALIZED EXTERNAL DATA
      ↓
TARGET ENGINE
```

The Integration Engine owns the path through the middle.

The receiving engine owns the meaning.

---

# 43. Data Export Pipeline

Export should be:

```text
SOURCE ENGINE
      ↓
AUTHORIZED DATA SET
      ↓
CONSENT / POLICY CHECK
      ↓
INTEGRATION TRANSFORMATION
      ↓
EXTERNAL SYSTEM
      ↓
CONFIRMATION
```

This is especially important for:

* Bank systems
* Lenders
* E-signature
* Email
* Payroll
* External document systems

---

# 44. Consent Boundary

Integration must never bypass Consent & Access.

Example:

> Accounting connection exists.

does not mean:

> CPA may access all accounting data.

Instead:

> Integration makes data available to the platform under its connection scope.

Consent determines whether another stakeholder can receive it.

---

# 45. Policy Boundary

Integration must also obey Policy / Compliance.

Example:

> Highly sensitive employee data may not be exported through a particular provider.

Integration receives:

> EXPORT_DENIED

from policy evaluation and stops.

---

# 46. Identity Boundary

Identity & Access establishes:

> Which user authorized the external connection?

The Integration Engine records:

> Connection belongs to User/Organization X.

It should not create a second independent user-account system.

---

# 47. Audit Boundary

Audit records:

> Connection created.

> Sync occurred.

> External data imported.

> External action initiated.

> Credential revoked.

Integration remains responsible for executing the integration.

Audit remains responsible for historical record.

---

# 48. Workflow Boundary

Workflow may say:

> Sync accounting data every Monday.

Integration performs the sync.

Or:

> When lender webhook arrives, trigger workflow.

Integration emits the event.

---

# 49. Notification Boundary

Integration can emit:

> ConnectionExpired.

Notification tells the user:

> “Your accounting connection needs to be reauthorized.”

Notification does not repair the connection.

---

# 50. Local-First Boundary

This is particularly important.

An external system connection does not automatically mean cloud ingestion.

For example, the user could connect an accounting system while choosing:

> **Process financial data locally where possible.**

The Integration Engine retrieves authorized data.

Local Vault / local processing can handle sensitive data.

Only approved structured outputs may move into cloud services.

---

# 51. Local Connector Mode

Some future integrations may support:

> **Local Connector**

where the connector operates on the user's machine.

Example:

```text
QuickBooks export
      ↓
Local Connector
      ↓
Local Vault
      ↓
Local Analysis
      ↓
Authorized Platform Metadata
```

This is highly compatible with the local-first philosophy.

---

# 52. External AI Integrations

AI providers should be treated like any other external system.

The Integration Engine can manage:

* Provider connection
* Model/service identifier
* Authentication
* Data-transfer scope
* Usage limits
* Availability
* Version

Policy and Consent determine whether protected information may be sent.

---

# 53. Integration Health

Each connection should display:

**Healthy**

**Needs Reauthorization**

**Sync Delayed**

**Sync Failed**

**Provider Unavailable**

**Permission Changed**

**Disconnected**

The user should understand the problem without reading technical logs.

---

# 54. Error Handling

Errors should be classified.

### Authentication error

> External credential expired.

### Permission error

> External system denied requested access.

### Mapping error

> Field cannot be converted.

### Validation error

> Data does not meet expected structure.

### Provider error

> External system unavailable.

### Rate limit

> Provider temporarily limited requests.

### Conflict

> External data conflicts with existing platform information.

The user should see the relevant action.

---

# 55. Retry

Retryable failures should support:

* Automatic retry
* Backoff
* Maximum attempts
* Manual retry
* Escalation

Workflow and Notification handle broader task/escalation behavior.

---

# 56. Provider Outages

If a provider goes down:

> Integration status: Provider unavailable.

The platform should continue operating where possible using:

* Cached data
* Local Vault
* Existing platform state
* Offline workflows

The platform should not become unusable merely because one external service is offline.

---

# 57. External IDs

Every synchronized object should retain its external identifier.

Example:

> Platform document: DOC-1004

> Google Drive file: `1aB3...`

> Lender request: `REQ-8841`

This allows later reconciliation and prevents duplicate objects.

---

# 58. Integration Object Mapping

A mapping record can store:

```text
Platform object
External provider
External object type
External object ID
Last synchronized version
Last synchronized timestamp
Status
```

This makes integrations reversible and debuggable.

---

# 59. Duplicate Detection

When importing information, the Integration Engine should detect potential duplicates.

Example:

> This external file appears to match Financial Statements v4 already in the Vault.

The system can present:

> Existing document found.

rather than creating an unnecessary duplicate.

---

# 60. Sync Deletion

An external object being deleted should not automatically delete the platform copy.

Example:

> External file deleted.

The platform should report:

> External source no longer contains this file.

The Local Vault or platform record remains governed by its own retention rules.

This prevents external systems from silently destroying internal records.

---

# 61. External Updates

Similarly, an external update should not automatically destroy internal edits.

Where conflict exists:

> External version differs from platform version.

show both.

Let the appropriate engine resolve the issue.

---

# 62. Connection Ownership

A connection should identify:

* Who authorized it
* Which organization owns it
* Which workspace uses it
* What permissions were granted
* When authorization expires

This becomes important when employees or professionals leave an organization.

---

# 63. Reauthorization

When a connection expires:

> Reauthorize

should be simple.

But reauthorization should not silently increase permissions.

If the provider now requests broader access:

> The user sees the changed scope.

---

# 64. Disconnect

Users should be able to disconnect an integration.

Disconnect should:

* Revoke/disable future synchronization
* Preserve imported platform data
* Preserve audit history
* Mark external connection inactive
* Stop future webhook processing

It should not automatically delete locally retained information.

---

# 65. Integration Data Retention

Disconnecting an external service should not automatically erase previously imported information.

Retention remains governed by:

> Local Vault
> Business Reality
> Policy / Compliance
> applicable workspace rules

---

# 66. Integration Marketplace

The platform could eventually support a connector catalog:

> Accounting

> Payroll

> Banking

> E-signature

> Calendar

> Email

> Document systems

> Lender systems

Each connector could show:

* Provider
* Capabilities
* Permissions requested
* Supported data
* Sync direction
* Security information
* Current status

The Marketplace Engine used for professionals should not necessarily be reused as the technical integration catalog.

A separate connector registry can live within Integration.

---

# 67. Connector Versioning

External APIs change.

Therefore every connector should have:

* Connector version
* Provider API version
* Compatibility state
* Upgrade path
* Deprecation date

Existing connections should not silently break because the connector was upgraded.

---

# 68. Integration Migrations

When a provider changes API versions:

> Connector v2 → Connector v3

The platform should support controlled migration.

Existing connections should be evaluated before migration.

Failures should be surfaced rather than silently producing incomplete data.

---

# 69. Integration Testing

Each connector needs:

### Authentication tests

### Permission-scope tests

### Import tests

### Export tests

### Mapping tests

### Error tests

### Retry tests

### Webhook tests

### Duplicate-event tests

### Conflict tests

### Provider-version tests

### Security tests

---

# 70. Sandbox Environments

Where external providers support them, connectors should use sandbox/test accounts.

This prevents development testing from touching production business data.

---

# 71. Connection Audit

Audit should record:

* Connected
* Authorized
* Reauthorized
* Permission scope changed
* Sync started
* Sync completed
* Sync failed
* External action performed
* Disconnected
* Credential revoked

This provides complete integration history.

---

# 72. Integration Security

The engine should follow:

**Least privilege**

**Explicit authorization**

**Credential isolation**

**Encryption**

**Credential rotation**

**Revocation**

**Data minimization**

**Provider verification**

**Scoped access**

**Auditability**

External integrations should be considered part of the platform's attack surface.

---

# 73. Secrets Management

API keys, refresh tokens, certificates, and other credentials should not be stored as ordinary application data.

The implementation should use an appropriate secrets-management mechanism.

The Integration Engine stores references and metadata needed to manage credentials.

---

# 74. Rate Limits and Cost Controls

External providers may impose:

* API limits
* Request quotas
* Usage costs

The engine should track:

* Requests
* Rate limits
* Backoff
* Quotas
* Usage

Workflow should not accidentally create runaway synchronization loops.

---

# 75. Infinite Loop Prevention

This is important.

Example:

```text
Platform update
   ↓
External system
   ↓
Webhook
   ↓
Platform update
   ↓
External system
```

The Integration Engine needs:

* Source tracking
* Change origin
* Idempotency
* Loop detection

so a synchronization does not bounce forever between systems.

---

# 76. Source-of-Change

Every synchronized update should identify:

> Source = External

or:

> Source = Platform

or:

> Source = User

This becomes valuable for conflict resolution.

---

# 77. Integration Event Contract

A normalized integration event might look like:

```text
ExternalDataChanged
  provider
  connection_id
  external_object_type
  external_object_id
  change_type
  source_timestamp
  received_timestamp
  mapping_version
  correlation_id
```

The target engine determines what that change means.

---

# 78. Example: Accounting Update

```text
QuickBooks
   ↓
ExternalDataChanged
   ↓
Integration Engine
   ↓
Financial Dataset Updated
   ↓
Business Reality / Fact Verification
   ↓
Conflict Check
   ↓
Business Reality Updated
   ↓
Workflow Event
   ↓
Confidence Refresh
```

The Integration Engine is the bridge, not the accountant.

---

# 79. Example: E-Signature Completion

```text
DocuSign
   ↓
Webhook
   ↓
Integration Engine
   ↓
SignatureCompleted
   ↓
Workflow
   ↓
Transaction milestone updated
   ↓
Notification
```

The Integration Engine does not decide whether the signed document satisfies legal requirements.

---

# 80. Example: Calendar Change

```text
Owner changes closing date
       ↓
Transaction Orchestration
       ↓
Workflow recalculates deadlines
       ↓
Integration Engine
       ↓
Calendar adapter
       ↓
Calendar event updated
```

The calendar remains an external scheduling service.

---

# 81. Example: Lender Response

```text
Lender System
      ↓
Webhook / API
      ↓
Integration Engine
      ↓
FinancingResponseReceived
      ↓
Capital / Financing
      ↓
Workflow
      ↓
Transaction
      ↓
Notification
```

This gives us the same clean event chain used elsewhere.

---

# 82. Integration and Professional Systems

A professional might use:

> Practice-management system

The platform could synchronize:

* Assignment status
* Review request
* Document request
* Completed review

But Professional Review remains the platform's authoritative workflow for the professional engagement.

---

# 83. Integration and Closing

Later, Closing Engine may connect to:

* E-signature
* Payment/escrow providers
* Legal document systems
* Banking
* Filing systems

Integration coordinates the technical exchange.

Closing coordinates closing state.

Neither should absorb the other.

---

# 84. Later-Phase Integrations

The engine should be designed to support future systems such as:

* Payroll
* Insurance
* Benefits
* HR systems
* CRM
* Inventory
* ERP
* Property-management systems
* Government filing systems
* Cap-table systems
* Trust administration
* Employee ownership administration
* Seller-note servicing
* Accounting firms
* Data rooms
* Valuation platforms

The core connector architecture should not need to change.

---

# 85. Core Data Objects

## Integration

Overall external relationship.

## Connector

Provider-specific adapter.

## Connection

Authorized instance of a connector.

## CredentialReference

Secure reference to authentication material.

## ExternalObject

Reference to an object in an external system.

## ObjectMapping

Relationship between external and internal objects.

## FieldMapping

Mapping between external and internal fields.

## SyncJob

A particular synchronization execution.

## SyncSchedule

Rules governing recurring synchronization.

## WebhookSubscription

External event subscription.

## IntegrationEvent

Normalized external event.

## IntegrationError

Recorded integration failure.

## IntegrationHealth

Current connection status.

## IntegrationPermission

Scope of external-system authority.

---

# 86. Engine Contract

Core capabilities:

```text
registerConnector()
createConnection()
authorizeConnection()
reauthorizeConnection()
disconnectConnection()
getConnectionHealth()
requestExternalData()
importData()
exportData()
sync()
scheduleSync()
registerWebhook()
processWebhook()
mapExternalObject()
mapField()
resolveExternalObject()
getExternalObject()
retrySync()
handleConflict()
getSyncHistory()
```

Administrative capabilities:

```text
enableConnectorVersion()
disableConnector()
migrateConnection()
testConnection()
inspectIntegrationHealth()
```

---

# 87. Architectural Lock

These should now be treated as requirements:

**1. Integration is a standalone engine.**

**2. Integration connects external systems without absorbing their domain responsibilities.**

**3. External systems remain sources of record for their own domains where appropriate.**

**4. Internal engines remain sources of record for their internal domains.**

**5. Connectors are provider-specific adapters behind a normalized interface.**

**6. Provider integrations are independently replaceable.**

**7. Credentials are scoped, protected, rotatable, and revocable.**

**8. Read and write permissions are distinct.**

**9. High-impact external actions require appropriate authorization.**

**10. Integrations cannot bypass Policy / Compliance.**

**11. Integrations cannot bypass Consent & Access.**

**12. External authentication is separate from platform authentication.**

**13. External credential ownership is separate from user identity.**

**14. Imported data does not automatically become platform truth.**

**15. External/internal conflicts are preserved rather than silently overwritten.**

**16. Data mappings and transformations are versioned.**

**17. External object IDs are retained for reconciliation.**

**18. Synchronization is idempotent.**

**19. Webhooks are authenticated and replay-resistant.**

**20. Sync loops are prevented.**

**21. External deletions do not automatically delete internal records.**

**22. Connection failure does not unnecessarily disable the rest of the platform.**

**23. Local-first operation can be supported through local connectors and local processing.**

**24. External AI services are treated as integrations subject to data policy and consent.**

**25. Integration events are available to Workflow.**

**26. Integration activity is recorded by Audit / Provenance.**

**27. Connection health and authorization state are visible to users.**

**28. Connector versions can evolve independently from the core platform.**

**29. Existing synchronized data remains under the platform's own retention rules after an integration is disconnected.**

**30. The Integration Engine is independently versioned, tested, secured, and replaceable.**

---

# 88. Architectural Boundary Summary

| Engine                          | Owns                                                                                              | Does Not Own                      |
| ------------------------------- | ------------------------------------------------------------------------------------------------- | --------------------------------- |
| **Integration**                 | Connections, adapters, synchronization, mappings, external IDs, webhooks, integration credentials | Meaning of imported business data |
| **Identity & Access**           | Platform identity, organization membership, roles, authentication                                 | External-system connection itself |
| **Policy / Compliance**         | Rules governing whether an integration/action is permitted                                        | Executing the connection          |
| **Consent & Access**            | Authorization to disclose specific information                                                    | API connection mechanics          |
| **Local Vault**                 | Private local data, files, local processing                                                       | External provider relationships   |
| **Business Reality**            | Current business facts                                                                            | Retrieval from external systems   |
| **Capital / Financing**         | Financing state and meaning                                                                       | Lender API mechanics              |
| **Communication**               | Conversations and requests                                                                        | Email infrastructure              |
| **Workflow**                    | Triggers and execution mechanics                                                                  | Provider-specific integration     |
| **Transaction / Orchestration** | Transaction execution state                                                                       | External-system synchronization   |
| **Audit / Provenance**          | Historical integration activity                                                                   | Current connection health         |
| **Notification**                | Alerts about integration conditions                                                               | Repairing integrations            |

## Hard Boundary

> **Integration moves authorized information between systems. It does not decide what that information means, who is allowed to see it, whether the transaction should proceed, or whether an external action is professionally or legally appropriate.**

---

# 89. The Integration Pattern

The platform can now use one reusable pattern across virtually every external system:

```text
EXTERNAL SYSTEM
       │
       ▼
   INTEGRATION
       │
       ▼
VALIDATE / MAP
       │
       ▼
  TARGET ENGINE
       │
       ▼
      EVENT
       │
       ▼
    WORKFLOW
       │
   ┌───┴────────────┐
   ▼                ▼
NOTIFICATION     TRANSACTION
```

For outbound activity:

```text
SOURCE ENGINE
       │
       ▼
     EVENT
       │
       ▼
   WORKFLOW
       │
       ▼
POLICY / CONSENT
       │
       ▼
  INTEGRATION
       │
       ▼
EXTERNAL SYSTEM
       │
       ▼
 CONFIRMATION
```

That pattern means adding **a bank, payroll provider, lender, accounting platform, e-signature service, calendar, or professional system** becomes a connector problem rather than an architectural rewrite.

And that is exactly where we want to end up: **the platform owns the intelligence and workflow; integrations provide controlled bridges to the outside world.** 🔌

The architecture now has a particularly strong separation between **internal truth, external data, permission, policy, execution, and connectivity**. That will make the later integrations much easier to add without turning any external vendor into a hidden dependency of the core platform.
