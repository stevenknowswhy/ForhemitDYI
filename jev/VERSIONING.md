# Jev contract versioning

Question contracts and threshold policies are versioned independently with semantic versions.

## Question contracts

A question ID names one stable operational judgment. Within an ID:

- **Major** — change instructions, criteria meaning, choice membership, choice meaning, authority, consent purpose, fallback semantics, allowed/forbidden input fields, or the meaning of the minimization policy.
- **Minor** — add non-breaking examples, metadata, or tighter documentation without changing expected interpretation.
- **Patch** — spelling or formatting only.

Choice keys are durable API values. Never reuse a key for a different meaning. Removing, renaming, splitting, or merging a choice requires a major version.

The active registry contains at most one entry per question ID. Historical versions belong in source control or a future explicit archive, not beside the active version.

## Threshold policies

Threshold policy wording and numeric gates are not part of the question version. They live in `threshold-policies.json` and have their own semantic version.

Any numeric threshold or action-mode change increments the policy version. A production approval is a separately reviewed release event; design defaults remain `production_approved: false`.

## Compatibility

An evaluation record is reproducible only when it stores:

- question ID and version
- threshold policy ID and version
- requested model alias and returned model version
- complete answer probabilities
- nonempty versioned source references
- versioned external-AI consent and consent purpose
- state, request, and criteria hashes
- input fields, exact criteria/options snapshot, and minimization-policy ID/version
- versioned external-AI disclosure reference
- Jev actor identity, selected action, authority effect, fallback, and human override

Consumers must reject unknown major versions. Minor/patch versions may be accepted only when the consumer explicitly declares compatibility.