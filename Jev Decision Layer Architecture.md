# Jev Decision Layer Architecture

**Status:** Proposed cross-cutting architecture  
**Applies to:** Existing decision, transaction, and infrastructure engines  
**Model interface:** TypeSafe System One / Jev  
**Canonical name:** Jev (`JEV` may be used informally)

---

## 1. Purpose

Jev is a typed probabilistic decision primitive for bounded ambiguity.

It receives authorized state and caller-defined questions, then returns one of three result forms:

1. **Noul** — a yes/no probability.
2. **Choice** — one caller-defined option plus the complete probability distribution.
3. **Score** — a probability-weighted position on a caller-defined ordered rubric.

Jev does not generate prose, extract fields, calculate financial outputs, enforce authorization, or create candidate options.

The platform uses Jev to:

- classify
- score
- prioritize
- detect likely ambiguity or conflict
- choose among permitted next routes

Jev is not an engine. It is a cross-cutting capability called by an engine that continues to own the domain decision, state transition, and audit record.

---

## 2. Terminology

- **Jev** — TypeSafe System One, used here as a typed probabilistic decision primitive.
- **Noul** — a yes/no judgment represented as the probability of `true`; it is not a deterministic boolean.
- **Choice** — a selection among caller-defined options, returned with the complete option probability distribution.
- **Score** — a probability-weighted position on a caller-defined ordered rubric.
- **Probability** — Jev's relative support for an answer under the supplied state and question contract.
- **Confidence** — response metadata about the strength of a typed result; it is not business confidence, legal certainty, feasibility, or truth.
- **Probability margin** — the difference between the highest and second-highest choice probabilities, used as one abstention signal.
- **Abstention** — the platform deliberately takes no Jev-driven action because context, authorization, confidence, margin, or authority is insufficient.
- **Fallback** — the safe, versioned route used after abstention or service failure, usually preserving state and routing to a named human role.
- **Maximum authority** — the strongest effect a question's result is ever allowed to have, independent of model confidence.

Use precise language in all future documents: name the typed operation, owning engine, maximum authority, and fallback. Do not describe a Jev result as an AI decision, AI confidence, or AI recommendation.

---

## 3. Architectural Position

```text
AUTHORIZED DOMAIN STATE
        ↓
OWNING ENGINE
        ↓
DETERMINISTIC PRECONDITIONS
        ↓
MINIMIZATION + CONSENT CHECK
        ↓
VERSIONED JEV QUESTION
        ↓
TYPED RESULT + FULL PROBABILITIES
        ↓
CONFIDENCE / MARGIN POLICY
        ↓
SIGNAL | HUMAN CONFIRMATION | REVERSIBLE ROUTE | ABSTAIN
        ↓
OWNING ENGINE RECORDS THE OUTCOME
```

Jev never writes domain state directly.

The owning engine:

- chooses the question contract
- supplies the allowed options or rubric
- confirms authorization and data minimization
- interprets the result using a versioned threshold policy
- applies any permitted action
- records the result and any human override
- invalidates the result when relevant source state changes

---

## 4. Relationship to the Three Decision Layers

Jev must preserve:

```text
OWNER OBJECTIVE
≠
PLATFORM SCENARIO
≠
PROFESSIONAL DETERMINATION
```

### Owner Objective

Jev may flag incompleteness, ambiguity, or an apparent contradiction. It cannot decide what the owner wants, rewrite a destination, or relax a nonnegotiable.

### Platform Scenario

Jev may score readiness, classify conflicts, or select a next route. It cannot recommend a scenario, declare feasibility, or convert assumptions into facts.

### Professional Determination

Jev may route an issue to a professional scope, check whether a determination contains required structured elements, or identify disagreement. It cannot make, average, supersede, or normalize a professional determination.

---

## 5. Allowed Authority Levels

Every registered question declares exactly one maximum authority.

### `signal_only`

The result may be displayed, monitored, or used to sort a human queue. It cannot change domain state.

Examples:

- destination completeness
- scenario readiness
- package language QA
- communication urgency

### `human_confirmed`

Jev may prepare a proposed action. A named human role must confirm before the owning engine changes state.

Examples:

- owner destination clarification
- professional candidate selection
- request for missing evidence

### `reversible_route`

Jev may take a low-risk, reversible routing or prioritization action above a calibrated threshold.

Examples:

- next permitted journey question
- notification digest placement
- internal review queue

The action must not:

- grant access
- skip a mandatory gate
- alter source truth
- bind the owner
- create a professional conclusion
- move money
- execute a transaction

### `professional_only`

Jev may triage or prepare the state, but a qualified professional remains the decision authority.

Examples:

- legal/tax/valuation/lending review routing
- unresolved factual conflict with professional significance
- professional package review

### `no_role`

The work must be deterministic or directly human-controlled.

Examples:

- identity and authorization
- consent and revocation
- financial calculations
- formula execution
- policy `ALLOW` / `DENY`
- audit history
- signatures, funding, and closing state

---

## 6. Prohibited Uses

Jev must never be the final authority for:

1. Identity, roles, permissions, tenant isolation, or authentication.
2. Consent, access grants, sharing, expiration, or revocation.
3. Policy and compliance enforcement.
4. Financial calculations, valuation outputs, taxes, DSCR, proceeds, or financing gaps.
5. Legal, tax, securities, lending, fiduciary, credential, or professional determinations.
6. Vendor approval, suspension, credential verification, or appeals.
7. Relaxing or overriding a nonnegotiable.
8. Selecting a winning scenario, professional, lender, buyer, financing proposal, or closing path.
9. Converting a message, model output, or AI result into domain truth.
10. Creating or modifying factual provenance.
11. Sending external communications or taking protected integration actions without the required confirmation.
12. Determining whether private data may leave the Local Vault.

A Jev result never overrides a hard rule. When they conflict, the deterministic rule wins and the conflict is audited.

---

## 7. Data and Privacy Boundary

Jev is an external AI processing destination under Local Vault, Consent & Access, Policy, Identity & Access, and Audit / Provenance.

Before every evaluation, the owning engine must establish:

- the actor is known
- the workspace and organizational context are explicit
- external AI processing is allowed for this purpose
- the state is minimized to what the question requires
- secrets and unrelated identifiers are excluded
- source objects and versions are recorded
- sensitivity and retention rules are known

Preferred state order:

1. derived features and categories
2. structured facts and references
3. selected excerpts only under a separately registered future contract
4. raw documents only under a separately registered future contract, explicit authorization, and security review

All initial pilot contracts prohibit selected excerpts and raw documents. They accept only their machine-readable structured-field allowlists.

Possession is not permission to transmit.

---

## 8. Question Contract

Every question is registered and versioned.

Required fields:

```json
{
  "id": "scenario.next_route",
  "version": "1.0.0",
  "owner_engine": "Scenario",
  "type": "choice",
  "instructions": "Choose the safest next route from the supplied options.",
  "criteria": {
    "request_evidence": "Required factual support is missing.",
    "clarify_owner_intent": "Owner intent is ambiguous or contradictory.",
    "resolve_conflict": "Material inputs conflict.",
    "professional_review": "Qualified professional judgment is required.",
    "owner_review": "The scenario is ready for owner exploration.",
    "human_triage": "No route is sufficiently supported."
  },
  "maximum_authority": "signal_only",
  "fallback": "human_triage",
  "consent_purpose": "scenario_readiness_evaluation"
}
```

Contract rules:

- IDs are stable and namespaced by owning engine or capability.
- Instructions contain the complete question; the ID is not used for inference.
- Choice options are caller-defined and have stable semantic meanings.
- Score rubrics use ordered, mutually distinguishable levels.
- Criteria do not contain hidden policy that belongs in the Policy Engine.
- Every breaking instruction, criterion, or meaning change increments the major version.
- Threshold changes are separately versioned.
- Unknown, insufficient-context, or human-triage routes are explicit where appropriate.

---

## 9. Evaluation Record

Every call records:

- evaluation ID
- workspace and actor references
- owning engine
- object and version references
- question ID and version
- threshold-policy version
- requested model alias
- returned model version
- nonempty source object/version references
- exact minimized input fields and minimization-policy version
- state, request, and criteria hashes
- exact criteria/options snapshot
- versioned external-AI consent reference
- immutable external-AI disclosure reference
- typed answer
- full probabilities
- confidence
- selected action
- fallback reason
- human confirmation or override
- correlation and causation IDs
- creation and invalidation timestamps

The audit record identifies Jev as the decision service. It does not falsely attribute Jev output to the triggering human.

---

## 10. Confidence, Abstention, and Fallback

A selected answer is not certainty.

Initial evaluation hypotheses:

- **Noul:** automate only at `≤ 0.10` or `≥ 0.90`.
- **Choice:** automate a reversible route only when the top probability is `≥ 0.80` and exceeds the runner-up by `≥ 0.25`.
- **Score:** display as a signal until calibrated against labeled human judgments.

These values are not production defaults. Each question is calibrated independently.

The platform abstains when:

- required context is missing
- confidence or probability margin is below policy
- the model is unavailable
- the question version is unknown
- consent is absent or stale
- source state changed
- a deterministic rule conflicts with the result
- the result proposes authority beyond the registered maximum

Fallback must be safe and explicit:

- show the ambiguity
- route to a human
- preserve the current state
- never silently choose a more consequential path

---

## 11. Initial Pilot Questions

The first vertical slice uses four pilots.

### Destination quality

- completeness score
- apparent nonnegotiable conflict
- next clarification dimension

Maximum authority: `human_confirmed`

### Journey next step

- best permitted next question or stage
- sufficient state to advance
- uncertainty-reduction score

Maximum authority: `reversible_route`, only after calibration

### Scenario readiness

- readiness score
- unresolved hard-constraint conflict
- next route: evidence, clarification, conflict, professional review, owner review, or human triage

Maximum authority: `signal_only` initially

### Review-package QA

- unsupported claim
- unattributed professional language
- exploratory scenario presented as recommendation
- omitted material conflict
- recipient-purpose mismatch

Maximum authority: `signal_only`

The machine-readable starting registry is in `jev/pilot-questions.json`.

---

## 12. Evaluation Requirements

Before a question can influence production behavior:

- [ ] 100–300 representative labeled states exist.
- [ ] Ambiguous and missing-context cases are included.
- [ ] Relevant human and professional labels are separated.
- [ ] Calibration is measured, not only accuracy.
- [ ] False-advancement and hard-boundary violations are measured.
- [ ] Data-minimization and consent behavior are tested.
- [ ] Model-version drift can be replayed.
- [ ] Low-confidence fallback is verified.
- [ ] Human override is captured.
- [ ] A rollback path exists.

Primary safety metrics:

- nonnegotiable override rate: **0**
- unauthorized external-AI disclosure rate: **0**
- access/policy decision delegation rate: **0**
- professional determination impersonation rate: **0**
- silent low-confidence advancement rate: **0**

---

## 13. Architectural Lock

1. Jev is a typed probabilistic decision primitive, not a generative author.
2. Jev is cross-cutting and does not become a new domain engine.
3. The owning engine retains authority and state ownership.
4. Deterministic rules, calculations, access controls, and provenance remain deterministic.
5. Owner intent and nonnegotiables remain owner-controlled.
6. Professional determinations remain attributed to qualified professionals.
7. Every question, rubric, threshold policy, and result is versioned.
8. Full probability distributions are preserved.
9. Low confidence causes abstention or human review.
10. External AI processing requires explicit authorization and minimization.
11. Jev never writes domain state directly.
12. Jev can be removed or replaced without changing domain truth.