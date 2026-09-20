# Jev contracts

This directory contains executable starting contracts for the cross-cutting [Jev Decision Layer](../Jev%20Decision%20Layer%20Architecture.md).

- `pilot-questions.json` — versioned typed questions and example/counterexample states for the first vertical slice.
- `pilot-question-registry.schema.json` — JSON Schema for the question registry.
- `threshold-policies.json` — independently versioned evaluation-only action thresholds.
- `threshold-policy.schema.json` — JSON Schema for threshold policies.
- `evaluation-record.schema.json` — JSON Schema for an auditable Jev result.
- `examples/valid-evaluation.json` — synthetic evaluation record used for structural and cross-registry validation.
- `VERSIONING.md` — semantic-versioning and compatibility rules.
- `../scripts/check-jev-contracts.py` — schema and cross-contract safety validation.

The registry is a design contract, not production configuration. Thresholds are evaluation hypotheses until calibrated against labeled examples. Every design policy is explicitly `production_approved: false`.

Each question has a machine-readable input contract. It allowlists structured fields, forbids raw documents, secrets, credentials, unrelated identifiers, and customer records, and pins the minimization policy. Professional-sharing consent never substitutes for the versioned external-AI consent recorded on an evaluation. Every evaluation also references an immutable external-AI disclosure record describing the minimized request that actually left the private boundary.

The validator checks schemas and cross-contract behavior: question/policy versions, consent purpose, owning module, answer type, probability coherence, selected-answer membership, input allowlists, minimization policy, and authority ceilings. It also runs adversarial records that must be rejected.

Jev question IDs and choice keys are stable API values. See `VERSIONING.md` before changing instructions, criteria, answer options, authority, consent purpose, fallback, input contracts, minimization, or thresholds.