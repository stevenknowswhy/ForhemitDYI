# California ESOP Vertical Slice Implementation Architecture

## 1. Purpose

This document translates the [California ESOP Exploration Vertical Slice](California%20ESOP%20Exploration%20Vertical%20Slice.md) into a buildable software shape without prematurely splitting Forhemit into services.

The first implementation is a **modular monolith with explicit trust boundaries**. Domain modules communicate through typed contracts inside one codebase. Jev is reached through a replaceable adapter. Professional collaboration receives only an authorized package copy; it does not receive the owner's private workspace.

This is an implementation architecture, not evidence that application code already exists.

---

## 2. Deployment Shape

```text
OWNER APPLICATION / MODULAR MONOLITH
│
├── Journey
├── Destination
├── Business Reality
├── Scenario
├── Review Package
├── Consent & Access
├── Policy
├── Audit / Provenance
├── Professional Review
└── Jev Adapter
       │
       └── TypeSafe API, only after consent + minimization
│
├── Private workspace database
├── Private document vault
└── Transactional outbox
       │
       └── authorized, version-specific disclosure
              │
              ▼
      PROFESSIONAL COLLABORATION SURFACE
      ├── disclosed package copy
      ├── questions / requests
      ├── attributed professional response
      └── append-only collaboration audit
```

One deployable may host the initial application modules. The private owner workspace and professional collaboration surface remain separate persistence and authorization boundaries even if they initially share a repository and release process.

Do not create one microservice per conceptual engine. An engine boundary becomes a module boundary first.

---

## 3. Module Boundaries

| Module | Owns | Must not own |
| --- | --- | --- |
| Journey | Current stage, permitted next steps, stage completion | Owner intent, business facts, professional judgment |
| Destination | Desired outcome, objectives, priorities, preferences, nonnegotiables | Scenario recommendation or professional determination |
| Business Reality | Current-state facts, provenance, status, freshness, conflicts | Destination or universal truth |
| Scenario | Immutable exploratory scenario versions, assumptions, unknowns, conflicts, readiness | Transaction execution, professional conclusions, a winning path |
| Review Package | Purpose-built package versions and manifests | Consent decision or professional response |
| Consent & Access | Consent, grants, disclosure authorization, revocation | Package content or Jev inference |
| Policy | Deterministic platform rules and hard gates | Legal-compliance determination |
| Professional Review | Assignment, questions, requests, attributed feedback/determinations, acknowledgment | Advice generation or normalization into platform truth |
| Audit / Provenance | Append-only event history, actors, causation, correlation, integrity metadata | Domain state or inferred intent |
| Jev Adapter | Typed request/response translation, minimization enforcement, evaluation persistence, fallback | Domain truth, authorization, calculations, prose generation |

Each module writes its own logical tables or aggregates through its repository interface. Direct writes across module boundaries are forbidden.

---

## 4. Persistence Boundaries

### 4.1 Private owner workspace

The owner workspace is the source of truth for:

- destination versions
- owner-authored nonnegotiables
- business-current-state versions and fact provenance
- scenario versions
- package drafts
- consent decisions
- immutable professional and external-AI disclosure records
- Jev evaluation records
- owner acknowledgments and decisions
- private audit events

For a local-first implementation, use an encrypted local transactional database such as SQLite plus the Local Vault for documents. If a hosted deployment is later approved, the same repositories may target a tenant-isolated transactional database without changing domain contracts.

### 4.2 Professional collaboration store

The collaboration surface stores only:

- the exact authorized package version
- its disclosure manifest
- recipient and review scope
- questions and information requests
- attributed professional feedback and determinations
- review status and collaboration audit events

It must not contain the owner's entire workspace or private document master.

### 4.3 Jev evaluation store

Jev evaluations are append-only decision-support records linked by reference. They do not live in source-of-truth domain fields.

A current signal is resolved by:

1. question ID/version
2. source object/version references
3. state, request, and criteria hashes
4. exact criteria/options snapshot
5. versioned external-AI consent and disclosure references
6. input fields and minimization-policy ID/version
7. invalidation status
8. threshold-policy ID/version
9. returned model version

A new evaluation supersedes but never rewrites the prior record.

---

## 5. Transaction Model

Use one transaction per aggregate command within a persistence boundary.

A successful command writes:

1. the new immutable aggregate version or lifecycle change
2. the append-only audit event
3. an outbox message when another boundary must be notified

These writes occur atomically.

### Idempotency

Every externally retriable command carries an idempotency key. The receiving boundary stores the key and original result. Repeating the key returns the original result and creates no second disclosure, invitation, evaluation, or professional response.

### Outbox and inbox

Use an outbox only for:

- publishing an authorized package to the collaboration surface
- receiving professional responses into the owner workspace
- requesting Jev evaluation
- recording integration delivery outcomes

Each receiving boundary uses an inbox/deduplication record. Internal same-process module calls do not need messaging merely to imitate services.

---

## 6. Command and Query Flow

### 6.1 Destination confirmation

```text
ConfirmDestination(command)
→ validate owner authority
→ validate deterministic required fields
→ preserve contradictions for owner resolution
→ create immutable DesiredOutcome version
→ append DestinationConfirmed event
→ invalidate dependent Jev evaluations
```

Jev may have supplied a displayed or human-confirmed clarification signal before this command. The command never accepts a Jev output as owner confirmation.

### 6.2 Scenario creation

```text
CreateESOPExplorationScenario(command)
→ load confirmed destination version
→ load confirmed current-state version
→ apply deterministic policy/nonnegotiable gates
→ create exploratory ScenarioVersion
→ create deterministic ScenarioReadiness
→ append ScenarioVersionCreated event
```

A later Jev readiness evaluation is stored separately and cannot change the deterministic readiness record directly.

### 6.3 Package authorization and disclosure

```text
AuthorizePackage(command)
→ load immutable package version
→ evaluate Policy
→ evaluate Consent & Access
→ confirm recipient, purpose, expiry, and manifests
→ write ConsentRecord
→ append authorization event + outbox message
→ collaboration boundary deduplicates message
→ copy exact package version
→ write DisclosureRecord
```

Jev package QA may block nothing by itself and may authorize nothing. A signal requiring attention routes to human review before the owner issues the authorization command.

### 6.4 Professional response

```text
SubmitProfessionalResponse(command)
→ authenticate professional identity/context
→ validate assignment and review scope
→ preserve exact source response
→ create attributed feedback/determination version
→ append professional event + outbox message
→ owner workspace records attributed copy
→ owner separately acknowledges or selects next route
```

No model transforms the professional's response into platform truth.

---

## 7. Deterministic Gate Precedence

Every command uses this order:

```text
Identity
→ Role / assignment
→ Policy
→ Consent & Access
→ Contract validation
→ Domain invariants
→ Jev signal, if registered and authorized
→ Human confirmation, when required
→ Command effect
```

A failure before the Jev step means Jev is not called. A Jev result cannot convert `DENY`, `REQUIRE_CONSENT`, invalid contract state, failed domain invariant, or unresolved hard nonnegotiable into an allowed command.

The code-level authority guard accepts:

- question contract
- requested action
- threshold-policy result
- current consent
- deterministic gate result

It returns either the permitted authority effect or fallback. Unknown question versions, policies, action modes, or authority levels fail closed to the manual path.

---

## 8. Jev Adapter

The domain calls a project-owned interface, never the TypeSafe API directly:

```text
evaluate(
  question_id,
  question_version,
  source_references,
  external_ai_consent_reference,
  expected_consent_purpose,
  correlation_id
) → JevEvaluationRecord
```

The adapter:

1. loads the exact registered question
2. checks its maximum authority
3. loads the independently versioned threshold policy
4. resolves a versioned, active, purpose-specific external-AI consent record; professional-sharing consent is never accepted
5. builds state exclusively from the question's machine-readable input allowlist
6. blocks forbidden fields, raw documents, secrets, credentials, unrelated identifiers, and customer records
7. records the exact input fields and minimization-policy ID/version
8. calculates canonical state, request, and criteria hashes
9. writes an immutable external-AI disclosure record for the exact recipient, purpose, consent, source versions, input fields, minimization policy, and hashes
10. invokes the configured provider
11. validates the typed response and probability coherence against the exact question version and criteria/options snapshot
12. stores full probabilities and returned model version
13. applies thresholds outside the model without exceeding the registered authority ceiling
14. records fallback, selected action, authority effect, and any human override
15. emits an audit event attributed to Jev rather than the triggering human

### Replaceability

The interface must support:

- `TypeSafeJevAdapter`
- `DisabledDecisionAdapter`, which always returns the declared fallback
- `RecordedEvaluationAdapter` for deterministic tests
- future approved typed-decision providers without changing domain modules

No module may branch on provider-specific response fields outside the adapter.

---

## 9. Initial Jev Call Sites

| Call site | Question | Initial mode | Permitted effect |
| --- | --- | --- | --- |
| Destination review | `destination.completeness` | Shadow, then displayed | Signal only |
| Destination review | `destination.nonnegotiable_conflict` | Shadow, then displayed | Signal only |
| Destination questions | `destination.next_clarification` | Shadow | Human-confirmed prompt only |
| Journey routing | `journey.next_step` | Shadow | No route change initially |
| Journey stage review | `journey.ready_to_advance` | Shadow | Signal only |
| Scenario review | `scenario.readiness` | Shadow, then displayed | Signal only |
| Scenario review | `scenario.next_route` | Shadow | Signal only |
| Package review | `review_package.boundary_qa` | Shadow, then displayed | Signal only |

Business Snapshot and Professional Review have no Jev call site in the initial slice.

---

## 10. Suggested Code Layout

The first implementation should use a layout equivalent to:

```text
app/
  modules/
    journey/
    destination/
    business_reality/
    scenario/
    review_package/
    consent_access/
    policy/
    professional_review/
    audit/
  decision_support/
    registry/
    authority_guard/
    minimization/
    adapters/
  contracts/
  persistence/
    private_workspace/
    collaboration/
    outbox/
    inbox/
  api/
  tests/
    contract/
    domain/
    integration/
    end_to_end/
```

The exact programming language and framework remain an implementation decision. The module and contract boundaries do not depend on that choice.

---

## 11. Test Strategy

### Contract tests

- every JSON Schema validates its canonical examples
- unsupported major versions are rejected
- immutable references require object ID and version
- professional determinations require professional attribution
- scenarios remain explicitly exploratory
- package authorization requires a consent reference

### Domain tests

- only the owner can confirm owner intent
- hard nonnegotiables cannot be relaxed by Jev or system logic
- unknown values never become zero
- professional judgment never becomes platform truth
- owner acknowledgment is distinct from agreement
- a source-version change invalidates dependent evaluations

### Jev adapter tests

- minimized state excludes forbidden fields
- no consent means no outbound request
- unknown question or policy fails to fallback
- provider timeout uses fallback
- malformed response uses fallback
- low probability or margin uses fallback
- hard-gate failure prevents invocation
- actor attribution remains Jev
- complete probabilities and model version are stored

### End-to-end tests

- complete the slice with Jev disabled
- complete the slice with all Jev calls abstaining
- revoke consent before evaluation
- revoke package consent after sharing
- change destination after scenario readiness
- receive conflicting professional views
- retry every external command with the same idempotency key
- prove no automatic transaction handoff occurs

---

## 12. Operational Minimum

Before real customer data:

- encrypt private workspace and collaboration data
- manage keys, rotation, recovery, and lost-device behavior
- classify every contract field
- implement retention and deletion policies
- monitor access, disclosure, adapter failures, and authorization denials
- define backup and recovery objectives for each persistence boundary
- complete threat modeling for local workspace, synchronization, professional portal, and Jev
- obtain legal/privacy review of California/federal and external-AI flows
- test incident response and breach notification
- complete an independent penetration test

---

## 13. Service-Split Rule

Split a module into a service only when one of these is demonstrated:

- a materially different trust or data-residency boundary
- independent scaling supported by measurements
- separate deployment or availability requirements
- a team-ownership boundary that cannot be managed in the monolith
- regulatory isolation
- failure containment that cannot be achieved in process

Conceptual engine count is not a reason to create service count.

---

## 14. Architectural Lock

> Build the first Forhemit slice as a modular monolith with explicit private-workspace and professional-collaboration trust boundaries. Preserve engine ownership through modules and versioned contracts. Keep Jev behind an auditable, replaceable adapter whose output can never bypass deterministic gates, become source truth, or exceed the registered maximum authority.