# Jev Repository Migration Checklist

**Goal:** Move Forhemit from a prose-only conceptual architecture toward a safe, testable, Jev-enabled vertical slice.

This checklist converts the findings in the repository review into an implementation sequence. Jev improves bounded judgment; it does not replace the repository's deterministic, owner-controlled, or professional boundaries.

---

## P0 — Repository truth and governance

### Correct the document roster

- [x] Add the Jev cross-cutting architecture document.
- [x] Add this migration checklist.
- [x] Update README and Document Index counts/groups.
- [x] Extend `scripts/doc-graph.py` to validate the human-readable count/group sentence in `DOCUMENT-INDEX.md`.
- [x] Add a regression fixture proving an intentionally wrong prose count fails.
- [x] Formally justify and limit generated gap-analysis archives in `analysis/README.md`.

### Fix repository automation

- [x] Fix `scripts/sync-docs.sh` so pre-staged files cannot be swept into the wrong one-file commit.
- [x] Add regression tests for:
  - [x] multiple staged files
  - [x] mixed staged/unstaged changes
  - [x] paths with spaces
  - [x] renames
  - [x] deletions
  - [x] hook-generated analysis changes
- [x] Add CI validation for the document graph and Jev registry.
- [x] Add Markdown link/anchor checking to CI.
- [x] Add ShellCheck to CI.
- [x] Make generated-report drift fail CI.

### Establish canonical terminology

- [x] Use **Jev** as the canonical product/model name.
- [x] Define Jev as a cross-cutting typed decision layer, not an engine.
- [x] State that Jev does not generate prose, extract fields, calculate outputs, or enforce authorization.
- [x] Establish a rule replacing ambiguous phrases such as “AI decides,” “AI confidence,” and “AI recommendation” with the exact typed operation and authority.
- [x] Add glossary entries for `noul`, `choice`, `score`, confidence, probability margin, abstention, fallback, and maximum authority.

---

## P0 — Buildable vertical slice

Implement one flow before expanding the engine catalog:

```text
Destination
→ Business Snapshot
→ Scenario Exploration
→ Professional Review Package
→ Professional Review
```

### Scope the slice

- [x] Select one owner persona: controlling owner of a privately held California operating business, considering transition in 1–3 years.
- [x] Select one Employee Ownership transaction shape: ESOP exploration through Professional Review.
- [x] Select the initial launch jurisdiction: California plus applicable US federal requirements.
- [x] Define explicit non-goals in `California ESOP Exploration Vertical Slice.md`.
- [x] Use bring-your-own-professional first.
- [x] Defer closing, marketplace monetization, capital-provider execution, seller-note liquidity, and ownership lifecycle.

### Define outcomes and acceptance tests

For every stage:

- [x] user outcome
- [x] minimum persisted objects
- [x] owning module/engine
- [x] deterministic gates
- [x] Jev question, if any
- [x] maximum Jev authority
- [x] human/professional authority
- [x] audit events
- [x] invalidation triggers
- [x] acceptance tests
- [x] telemetry

The complete stage contracts are in `California ESOP Exploration Vertical Slice.md`.

---

## P0 — Jev safety architecture

### Bound authority

- [x] Define `signal_only`, `human_confirmed`, `reversible_route`, `professional_only`, and `no_role`.
- [x] Prohibit Jev from access, consent, policy, calculations, professional determinations, and protected actions.
- [ ] Enforce maximum authority in code.
- [x] Require an explicit fallback for every registered question.
- [x] Require explicit `human_triage`, `human_help`, or equivalent insufficient-context behavior where ambiguity is material.
- [ ] Ensure hard rules always override Jev results.

### Protect data

Executable design contracts now model Jev as an external actor, separate professional and external-AI consent, per-question minimization allowlists, versioned source/hash references, and immutable external-AI disclosure records. The items remain unchecked until application code enforces them at runtime.

- [ ] Model Jev as an external AI actor/destination.
- [ ] Require a purpose-specific consent/disclosure reference.
- [ ] Implement state minimization before invocation.
- [ ] Block secrets and unrelated identifiers.
- [ ] Prefer derived features and structured facts over raw documents.
- [ ] Record source object/version references and state hash.
- [ ] Invalidate evaluations when source state or consent changes.
- [ ] Provide a local/manual path when Jev is unavailable or unauthorized.

### Audit every evaluation

- [x] Add an evaluation-record JSON Schema.
- [x] Require requested alias and returned model version in the evaluation-record contract.
- [x] Require question and threshold-policy IDs/versions in the evaluation-record contract.
- [x] Require complete typed answer probabilities, confidence where returned, action, fallback, and override fields.
- [x] Require correlation and causation IDs.
- [x] Record whether the result was displayed, human-confirmed, used for a reversible route, professional-only, or had no effect.
- [x] Require a criteria/options snapshot and state/request/criteria hashes for replay and audit.
- [x] Require a versioned external-AI disclosure reference distinct from consent.
- [x] Validate probability coherence, answer membership, consent purpose, input allowlists, minimization policy, and authority ceilings across registries.
- [ ] Implement persistence and enforce that Jev output is never attributed to the triggering human.

---

## P1 — Executable question contracts

### Registry

- [x] Add a machine-readable pilot question registry.
- [x] Add a registry validator.
- [x] Add JSON Schema validation for the registry.
- [x] Define semantic-versioning rules.
- [x] Require stable caller-defined choice meanings.
- [x] Reject duplicate active question IDs.
- [x] Reject automation without fallback and threshold policy.
- [x] Reject prohibited owner/engine combinations.
- [x] Add example and counterexample states for every question.

### Threshold policies

- [x] Version threshold policy separately from question wording.
- [x] Begin with conservative offline hypotheses:
  - [x] Noul automation only at `≤ 0.10` or `≥ 0.90`
  - [x] Choice top probability `≥ 0.80`
  - [x] Choice top-to-runner-up margin `≥ 0.25`
  - [x] Scores remain display-only until calibrated
- [ ] Calibrate each question separately.
- [x] Keep all initial thresholds explicitly unapproved for production pending benchmark evidence.

---

## P1 — First Jev pilots

### Pilot 1: Destination QA

- [x] Register completeness, conflict, and next-clarification questions.
- [ ] Build synthetic owner-destination examples.
- [ ] Test vague, contradictory, and fully specified destinations.
- [ ] Confirm Jev cannot rewrite owner intent.
- [ ] Confirm nonnegotiables cannot be relaxed.
- [ ] Require owner confirmation for any proposed change.

### Pilot 2: Journey next-step routing

- [x] Register next-step and advance-readiness questions.
- [ ] Supply only steps permitted by the journey definition.
- [ ] Test missing-context and ambiguous cases.
- [ ] Preserve a “not sure / human help” route.
- [ ] Begin in shadow mode.
- [ ] Allow reversible routing only after calibration.

### Pilot 3: Scenario readiness

- [x] Register readiness and next-route questions.
- [ ] Test unsupported assumptions, stale evidence, conflicts, and required professional review.
- [ ] Separate readiness from feasibility and recommendation.
- [ ] Display competing route probabilities when materially close.
- [ ] Keep all routing signal-only initially.

### Pilot 4: Review-package QA

- [x] Register boundary and language checks.
- [ ] Test unsupported claims and missing attribution.
- [ ] Test scenario language that sounds like advice.
- [ ] Test omitted material conflicts.
- [ ] Test recipient-purpose mismatch.
- [ ] Keep access and disclosure authorization deterministic.

---

## P1 — Executable domain contracts

The repository review found no JSON Schema, OpenAPI, AsyncAPI, protobuf, migrations, or contract tests.

Add machine-readable contracts for the vertical slice:

- [x] `DesiredOutcome`
- [x] `Nonnegotiable`
- [x] `BusinessCurrentState`
- [x] `Fact` and provenance reference
- [x] `ScenarioVersion`
- [x] `ScenarioReadiness`
- [x] `ProfessionalReviewPackage`
- [x] `ProfessionalDetermination`
- [x] `ConsentRecord`
- [x] `DisclosureRecord`
- [x] `ExternalAIDisclosureRecord`
- [x] common event envelope
- [x] Jev evaluation reference

For each contract:

- [x] stable ID/version
- [x] required and optional fields
- [x] mutable vs. immutable fields
- [x] source authority
- [x] sensitivity classification
- [x] idempotency behavior
- [x] compatibility rules
- [x] example payloads
- [x] validation tests

The initial executable bundle is in `contracts/`. It is a design contract for implementation, not evidence that persistence or APIs already exist. JSON Schema owns shape; semantic validation owns referential, temporal, recipient, purpose, disclosure, deterministic-gate, professional-attribution, and external-AI audit invariants.

---

## P1 — Implementation shape

- [x] Specify a modular monolith as the initial deployment shape.
- [x] Keep engine boundaries as modules and contracts.
- [x] Specify transactional persistence per trust boundary.
- [x] Specify append-only audit records.
- [x] Use outbox/inbox only where asynchronous boundary delivery is needed.
- [x] Keep Jev behind a replaceable adapter.
- [x] Keep Jev outputs outside source-of-truth tables.
- [x] Split services only for demonstrated scaling, security, ownership, regulatory, or deployment needs.

These architecture decisions are defined in `California ESOP Vertical Slice Implementation Architecture.md`; application implementation remains outstanding.

---

## P1 — Security and compliance workstream

Before real customer data:

- [ ] Data classification matrix
- [ ] Retention/deletion matrix
- [ ] Threat model:
  - [ ] owner device
  - [ ] Local Vault
  - [ ] synchronization boundary
  - [ ] Jev/external AI processing
  - [ ] professional portal
  - [ ] sharing links
  - [ ] integrations
- [ ] Key generation, storage, rotation, recovery, and lost-device behavior
- [ ] Account-recovery model
- [ ] Incident-response plan
- [ ] Breach-notification plan
- [ ] Vulnerability management
- [ ] Security monitoring
- [ ] Data-residency policy
- [ ] Recovery objectives
- [ ] Independent legal/privacy review
- [ ] Penetration testing before production

---

## P2 — Evaluation and release process

### Labeled benchmark

For each question:

- [ ] 100–300 representative states
- [ ] synthetic examples first
- [ ] independently labeled outcomes
- [ ] relevant professional labels separated by role
- [ ] ambiguous cases
- [ ] missing-context cases
- [ ] adversarial wording
- [ ] privacy/minimization cases
- [ ] hard-boundary counterexamples

### Metrics

- [ ] per-class precision/recall
- [ ] calibration
- [ ] abstention rate
- [ ] fallback rate
- [ ] false-advancement rate
- [ ] hard-boundary violation rate
- [ ] human override rate
- [ ] professional disagreement rate
- [ ] time saved
- [ ] model-version drift

### Rollout

- [ ] offline replay
- [ ] shadow mode
- [ ] internal review
- [ ] small synthetic pilot
- [ ] explicitly consented pilot
- [ ] reversible low-risk automation only
- [ ] rollback/disable switch
- [ ] version-pinned evaluation report

---

## Definition of Done for “Jev-ready”

The repository is Jev-ready when:

- [x] Jev is represented as a cross-cutting capability, not a domain authority.
- [x] Every Jev question is registered, versioned, and validated.
- [x] Every question has a named owning engine and maximum authority.
- [x] No access, policy, calculation, or professional determination is delegated to Jev.
- [ ] External AI consent and minimization are enforced.
- [ ] Evaluations are reproducible and auditable.
- [x] Low confidence always has a safe fallback in the executable question contracts; runtime enforcement remains part of implementation.
- [ ] A labeled benchmark supports every automated route.
- [x] The vertical slice has executable structural and semantic domain-contract tests.
- [x] CI verifies document, contract, and Jev-registry integrity.
- [ ] The model can be disabled or replaced without corrupting domain truth.