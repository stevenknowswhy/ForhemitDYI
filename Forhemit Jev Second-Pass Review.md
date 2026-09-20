# Forhemit Jev Second-Pass Review

**Review date:** 2026-09-20  
**Scope:** Updated California ESOP vertical slice, Jev decision layer, executable contracts, validators, CI, and migration checklist.

## Executive assessment

The repository is materially stronger after the second pass. The first migration established sound architecture and structural JSON Schemas. This pass found that the original validators could still accept internally contradictory workflows because JSON Schema validated each field's shape without proving relationships across objects, time, consent, authority, and disclosure.

Those gaps have now been converted into executable semantic checks.

The design is now a credible implementation specification for a safe Jev-enabled vertical slice. It is **not production-ready application code**. The remaining work is primarily runtime enforcement, labeled evaluation, security/compliance, persistence, API/database design, and operating controls.

## Material findings

### 1. Shape validation did not prove workflow integrity — fixed

The original domain contracts accepted all of these invalid states:

- package recipient differed from the consent/disclosure recipient
- consent expired before grant or disclosure
- the same category was included and excluded
- a scenario was ready for professional review while deterministic gates failed
- stable IDs were duplicated
- an `unknown` fact still carried a concrete value

`check-domain-contracts.py` now combines JSON Schema with referential, temporal, recipient, purpose, category, deterministic-gate, identity, credential, chronology, and disclosure invariants.

Current result:

- one valid synthetic conformance aggregate
- 18 rejected adversarial boundary cases

### 2. Jev records were not independently reproducible — fixed

The original evaluation schema accepted records with:

- no source references
- selected answers absent from returned probabilities
- probabilities totaling more than 1
- an authority effect above the question's declared ceiling
- no exact runtime criteria/options snapshot
- no machine-readable proof of minimized inputs

Evaluation records now preserve:

- exact question and threshold-policy versions
- requested alias and returned model version
- nonempty versioned source references
- exact input fields
- exact criteria/options snapshot
- state, request, and criteria hashes
- minimization-policy ID/version
- selected action, authority effect, fallback, and human override
- Jev actor identity
- versioned external-AI consent and disclosure references

The validator cross-checks these values against the active question and policy registries and rejects seven adversarial record classes.

### 3. Professional-sharing and external-AI consent were conflated — fixed

The contracts now require distinct, versioned consent records for:

- disclosure to a named professional
- processing by the external Jev service

A professional-sharing grant cannot authorize Jev processing.

### 4. Consent did not prove what actually left the private boundary — fixed

Consent records describe what may be sent. Evaluation records describe what came back. Neither alone proves the payload that was disclosed.

A new immutable `ExternalAIDisclosureRecord` now links:

- recipient
- purpose
- consent version
- source object versions
- minimized fields
- minimization-policy version
- state, request, and criteria hashes
- disclosure time
- resulting evaluation

### 5. Minimization rules were prose-only — fixed at contract level

Every registered question now contains a machine-readable input contract:

- allowed structured fields
- forbidden fields
- raw-document prohibition
- minimization-policy ID/version

Initial pilots prohibit raw documents, excerpts, secrets, credentials, unrelated identifiers, and customer records.

Runtime code still needs to enforce these contracts before any network request.

### 6. Some threshold policies exceeded declared authority — fixed

`scenario.next_route` and `review_package.boundary_qa` were `signal_only` questions but referenced a human-confirmation policy. Both now use shadow policies consistent with their authority ceilings.

All current policies remain evaluation-only and `production_approved: false`.

### 7. Professional attribution lacked credential semantics — fixed at contract level

A professional determination now distinguishes:

- professional identity
- claimed/assigned role
- credential status: unverified, self-attested, or platform-verified
- verification evidence when platform-verified

The platform must not imply verification when evidence is absent.

## Additional optimizations made

- Schema IDs now use stable project-owned URNs instead of the placeholder `forhemit.example` domain.
- The complete synthetic slice is explicitly documented as a **conformance aggregate**, not an API payload or persistence row.
- Adapter documentation now requires external-AI disclosure before invocation and exact response/authority validation afterward.
- Architecture documents now state that the initial pilots accept structured allowlisted fields only.
- Checklist completion was corrected: design/contract requirements are checked only where proven, while runtime items remain open.
- CI continues to validate document topology, Markdown links, generated-report drift, shell scripts, domain contracts, Jev contracts, and repository automation.

## Remaining priorities

### P0 — before invoking Jev with real data

1. Implement the adapter and deterministic gate precedence.
2. Resolve and enforce active versioned external-AI consent.
3. Build request state only from registered allowlists.
4. Write the immutable external-AI disclosure record before transmission.
5. Persist the append-only evaluation record as Jev—not the triggering human.
6. Invalidate evaluations when source, consent, question, policy, minimization, or model state changes.
7. Preserve the full manual path when Jev is disabled or unavailable.
8. Add runtime tests proving no Jev result can bypass a hard rule.

### P1 — before an external pilot

1. Define command/API contracts and lifecycle-specific object validation.
2. Create persistence migrations and database constraints.
3. Build 100–300 labeled synthetic states per question.
4. Measure calibration, false advancement, abstention, fallback, and boundary violations separately per question.
5. Keep all pilots in offline/shadow mode until their individual thresholds are approved.
6. Complete data-classification, retention/deletion, threat-model, key-management, recovery, monitoring, and incident-response work.
7. Obtain California/federal legal and privacy review of the external-AI flow.

### P2 — before production automation

1. Add a model/policy disable switch and tested rollback.
2. Add version-pinned evaluation reports and model-drift replay.
3. Permit only calibrated, reversible, low-risk routing.
4. Complete penetration testing and security review.
5. Define operational ownership for consent, disclosure, overrides, incidents, and model changes.

## Recommended implementation order

1. **Release 0:** deterministic manual vertical slice with no Jev calls.
2. **Release 1:** persisted Jev disclosure/evaluation records in offline replay.
3. **Release 2:** shadow calls with synthetic or explicitly consented minimized data.
4. **Release 3:** displayed signals only for independently calibrated questions.
5. **Release 4:** human-confirmed prompts.
6. **Release 5:** reversible routing only after benchmark, rollback, and policy approval.

## Current validation status

All repository checks pass:

- Jev registry, policy, record, and seven adversarial checks
- domain shape plus semantic integrity and 18 adversarial checks
- all 13 document-graph checks
- Markdown links across 98 files
- generated report drift
- sync-docs regression suite
- document-roster regression
- Python compilation
- Bash syntax
- ShellCheck

## Conclusion

No additional engine should be added for Jev. The optimal design remains:

> Jev is a replaceable typed-decision adapter inside existing modules. Deterministic rules authorize actions; versioned consent authorizes external processing; immutable disclosure records prove what left the private boundary; Jev returns bounded signals; humans and qualified professionals retain their existing authority.

The highest-value next step is application implementation of the manual vertical slice and Jev audit boundary—not additional architecture prose.