# WordPress Management Engine

## 1. Purpose

The WordPress Management Engine manages the platform's WordPress-powered websites and their operational relationship to the broader platform.

Its purpose is not to be another general CMS inside the core application.

Its sole job is:

> **Keep connected WordPress sites configured, synchronized, secure, publishable, recoverable, and operational without making WordPress the source of truth for the core platform.**

It handles:

* WordPress site connections
* Site registration
* Domains and environments
* Users and administrator references
* Themes
* Plugins
* Configuration
* Content synchronization
* Media synchronization
* Publishing/deployment state
* Backups
* Updates
* Health checks
* Connection health
* Site versioning
* Environment management
* WordPress-specific operational automation

---

# 2. Core Architectural Principle

## WordPress is a connected publishing system, not the platform's core application database.

The platform should be able to use WordPress for:

* Marketing websites
* Public content
* Blog publishing
* Landing pages
* Documentation
* Public resources

But the core platform's:

* Users
* Transactions
* Business data
* Professional relationships
* Sensitive documents
* Decisions
* Workflow state
* Billing

remain owned by their respective engines.

WordPress is an external content surface.

---

# 3. What This Engine Owns

* WordPress site records
* Site connections
* Environment mappings
* Deployment state
* Theme/plugin inventory
* WordPress configuration metadata
* Content synchronization state
* Media synchronization state
* Backup state
* Site health
* WordPress update state
* Publishing connection
* WordPress integration credentials/references
* Site version references

---

# 4. What It Does Not Own

It does not own:

* Core platform identity
* Platform authentication
* Transaction data
* Private business documents
* Professional review records
* Billing
* Marketplace data
* Platform workflow
* Core blog editorial meaning

The Blog Engine owns structured editorial content.

WordPress Management handles how that content is delivered through WordPress.

---

# 5. Site Object

A WordPress Site contains:

* Site ID
* Site URL
* Domain
* Environment
* WordPress version
* Connection status
* Theme
* Plugins
* Health state
* Backup status
* Last sync
* Last deployment
* Owner organization
* Security state

---

# 6. Environments

The engine should support:

* Development
* Staging
* Production

A change should not automatically go straight from development to production where a controlled deployment process is required.

---

# 7. Site Lifecycle

Suggested states:

* Registered
* Connected
* Healthy
* Needs Attention
* Disconnected
* Maintenance
* Suspended
* Archived

---

# 8. WordPress Connection

The engine should support secure connection methods appropriate to the environment.

Credentials should be:

* Scoped
* Protected
* Rotatable
* Revocable
* Auditable

The engine should not store unnecessary raw secrets.

---

# 9. WordPress Roles vs Platform Roles

These should remain separate.

A WordPress administrator is not automatically a platform administrator.

A platform editor is not automatically a WordPress administrator.

Identity & Access owns platform identity.

WordPress Management maps authorized users to site roles where required.

---

# 10. Theme Management

The engine can track:

* Active theme
* Theme version
* Parent/child relationships
* Update state
* Deployment status

Theme files should not become the source of truth for core platform logic.

---

# 11. Plugin Management

The engine should track:

* Plugin
* Version
* Enabled/disabled state
* Security/update status
* Dependencies
* Environment

It should support:

> “Plugin update available.”

rather than blindly auto-updating every production site.

---

# 12. Update Policy

Updates may be classified:

**Routine**

Normal maintenance.

**Security**

Security-sensitive update requiring timely review.

**Breaking**

Potential compatibility impact.

**Emergency**

Immediate intervention may be required.

Policy determines the required review and workflow.

---

# 13. Deployment

A deployment should record:

* Site
* Environment
* Version
* Changes
* Initiator
* Approval if required
* Start time
* Completion time
* Result

The engine should support rollback where the deployment technology allows it.

---

# 14. Content Synchronization

The WordPress engine should synchronize content using stable identifiers.

Example:

```text
Blog Engine
    ↓
Content Published
    ↓
WordPress Management
    ↓
WordPress Site
```

WordPress does not become the master record for editorial content unless the product deliberately chooses a WordPress-authoritative operating mode.

---

# 15. Media Synchronization

Images and other public media can be synchronized.

The system should retain:

* Original asset ID
* WordPress media ID
* URL
* Alt text
* Caption
* Licensing/rights metadata where applicable
* Publication state

Private transaction documents should never be mixed into the public WordPress media pipeline.

---

# 16. Domain Management

The engine can track:

* Domain
* DNS status reference
* SSL status
* Environment
* Primary/secondary designation
* Redirect configuration

It should not become a general registrar or DNS provider unless deliberately expanded.

---

# 17. Site Health

Health checks should include, where technically feasible:

* Availability
* WordPress version
* Plugin status
* Theme status
* SSL
* Backup status
* Sync status
* Integration status

The user can see:

> Site Health: Good

with details underneath.

---

# 18. Backups

The engine should coordinate:

* Backup schedules
* Last successful backup
* Backup integrity status
* Restore points
* Backup failures

Backup data may live in an external backup system.

The engine manages state and coordination.

---

# 19. Restore

A restore should be explicit.

Example:

> Restore Production to backup from September 18.

The system should show:

* What will be restored
* Which environment
* Expected impact
* Backup timestamp
* Approval requirement

High-impact restore actions can require explicit authorization.

---

# 20. Staging First

Where a safe deployment path exists:

```text
Development
     ↓
Staging
     ↓
Validation
     ↓
Production
```

The engine should encourage controlled promotion rather than direct production modification.

---

# 21. WordPress and Blog Engine

The Blog Engine owns:

* Posts
* Authors
* Editorial workflow
* Categories
* Tags
* SEO metadata
* Publication intent

WordPress Management owns:

* WordPress delivery
* Site state
* Synchronization
* Deployment
* Theme/plugin environment

This is a critical separation.

---

# 22. Public vs Private Content

The WordPress engine should enforce a strong publication boundary.

Public content may include:

* Blog posts
* Marketing pages
* Public guides
* Public resources

Private platform information must not enter the WordPress publishing path without explicit authorization.

Examples that should remain private by default:

* Transaction records
* Professional packages
* Sensitive financial data
* Employee information
* Owner decisions
* Private communications

---

# 23. Preview

Before publication, users should be able to preview:

* Desktop
* Mobile
* Staging URL
* SEO metadata
* Images

Publication should remain a deliberate action.

---

# 24. Publishing

Publishing should support states such as:

* Draft
* Staged
* Scheduled
* Published
* Unpublished
* Archived

The Blog Engine determines editorial state.

WordPress Management executes or synchronizes the publication.

---

# 25. Security

The engine should monitor:

* Failed authentication
* Unexpected plugin changes
* Unauthorized administrator changes
* Connection failures
* Vulnerable component states
* Backup failures

Security events should emit events to Audit, Workflow, and Notification.

---

# 26. WordPress User Provisioning

Where appropriate, the platform may provision WordPress users.

Example:

> Approved content editor joins publishing team.

Identity & Access confirms platform identity.

Policy checks eligibility.

WordPress Management provisions the WordPress role.

Audit records the provisioning.

---

# 27. No Identity Duplication

WordPress credentials should not become a second platform identity system.

The core platform remains the authoritative identity layer.

---

# 28. Site Configuration Versioning

Material configuration changes should be versioned.

Example:

> Production configuration v14 → v15.

The system should preserve:

* Previous configuration reference
* New configuration
* Actor
* Date
* Environment

---

# 29. Integration Events

Examples:

```text
WordPressConnected
WordPressDisconnected
SiteHealthChanged
PluginUpdateAvailable
SecurityUpdateAvailable
BackupCompleted
BackupFailed
DeploymentStarted
DeploymentCompleted
DeploymentFailed
ContentPublished
ContentUnpublished
SiteRestored
```

---

# 30. Core Data Objects

## WordPressSite

Connected website.

## WordPressEnvironment

Development, staging, or production context.

## Connector

WordPress-specific connection adapter.

## ThemeRecord

Theme identity/version/status.

## PluginRecord

Plugin identity/version/status/dependencies.

## Deployment

A site change promoted to an environment.

## BackupRecord

Backup state and restore point.

## SiteHealth

Current operational condition.

## ContentSyncRecord

Mapping between platform content and WordPress content.

## MediaSyncRecord

Mapping between platform assets and WordPress media.

---

# 31. Engine Contract

```text
registerSite()
connectSite()
disconnectSite()
getSiteHealth()
listThemes()
listPlugins()
checkUpdates()
scheduleUpdate()
deploy()
rollback()
createBackup()
verifyBackup()
restoreBackup()
publishContent()
unpublishContent()
syncContent()
syncMedia()
provisionWordPressUser()
revokeWordPressUser()
getDeploymentHistory()
getSiteHistory()
```

---

# 32. Architectural Lock

* WordPress Management is a standalone engine.
* WordPress is an external publishing surface, not the core platform database.
* Platform identity remains authoritative for platform users.
* WordPress roles do not automatically equal platform roles.
* The Blog Engine owns editorial meaning; WordPress Management owns WordPress delivery.
* Private platform data does not enter the public publishing pipeline by default.
* Themes and plugins are independently tracked and versioned.
* Production changes should support controlled deployment.
* Backups and restores are explicit and auditable.
* Site health is continuously observable where feasible.
* External connections use scoped and revocable credentials.
* WordPress failures should not disable the core platform.
* WordPress events feed Workflow, Notification, and Audit.
* The engine is independently versioned, tested, and replaceable.

---

# 33. Architectural Boundary Summary

| Engine                | Owns                                                                                          | Does Not Own                            |
| --------------------- | --------------------------------------------------------------------------------------------- | --------------------------------------- |
| **WordPress Management** | Site operations, WordPress connections, themes, plugins, deployment, backups, synchronization | Core business data or editorial meaning |
| **Blog Engine**       | Editorial content, publishing workflow, posts, authors, SEO metadata                          | WordPress infrastructure                |
| **Identity & Access** | Platform identity and roles                                                                   | WordPress content delivery              |
| **Integration**       | External connectivity infrastructure                                                          | Site-specific operational state         |
| **Policy / Compliance** | Platform rules for publishing/security                                                      | WordPress administration itself         |
| **Audit / Provenance** | Site and deployment history                                                                  | Current site configuration              |
| **Notification**      | Site-health and deployment alerts                                                             | Site repair or publishing logic         |
| **Workflow**          | Update/deployment automation mechanics                                                        | WordPress source of truth               |

---

# 34. One-Sentence Definition

> **The WordPress Management Engine keeps connected WordPress sites operational, synchronized, secure, and publishable without allowing WordPress to become the core application or private-data system.**
