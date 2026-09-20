# California ESOP vertical-slice contracts

`vertical-slice-contracts.schema.json` is the first executable domain-contract bundle for the [California ESOP Exploration Vertical Slice](../California%20ESOP%20Exploration%20Vertical%20Slice.md).

`examples/valid-slice.json` is a complete synthetic **conformance aggregate**. It contains no real customer or professional data. The aggregate exists to test relationships across lifecycle objects; it is not a command payload, persistence row, or claim that all objects are created at once. Production commands validate individual object contracts and then enforce the same cross-object invariants against persisted state.

## Contract rules

- IDs are stable within an object type.
- `version` identifies an immutable domain snapshot. A material edit creates a new version; it does not rewrite a version already referenced by a scenario, package, disclosure, review, or audit event.
- Mutable lifecycle fields, such as consent status or package sharing status, change through commands that also emit append-only events.
- Owner intent has `owner_stated` authority.
- Professional determinations have `professional_attributed` authority.
- Platform-derived scenarios and readiness never become owner or professional truth.
- Sensitivity is explicit and travels with the object.
- Commands use an idempotency key; duplicate keys return the original result rather than creating another effect.
- Events use a common envelope with correlation and causation IDs.
- Additive optional fields are backward-compatible minor changes. New required fields, changed meanings, removed/renamed fields, or narrowed enum values require a new major schema version.
- Consumers reject unsupported major versions and preserve unknown information rather than coercing it.
- Jev outputs are referenced from source-of-truth objects; they are not embedded as source truth.
- Professional disclosure consent and external-AI consent are distinct, versioned records. A professional-sharing grant never authorizes Jev processing.
- Every outbound Jev request creates an immutable `ExternalAIDisclosureRecord` linking recipient, purpose, consent version, source versions, minimized fields, policy version, and state/request/criteria hashes to the resulting evaluation.
- Cross-object integrity is enforced in addition to JSON shape: references must resolve, recipient and purpose must agree, disclosures must occur during active consent, disclosed categories cannot exceed the package, and advanced readiness cannot bypass deterministic gates.
- A professional identity, claimed role, and credential-verification status are separate facts. `platform_verified` requires a verification reference; other statuses cannot imply verification.

## Validation

Install `requirements-dev.txt`, then run:

```bash
python scripts/check-domain-contracts.py
```

The validator checks the schema, validates the synthetic aggregate, applies referential and temporal invariants, and proves that 14 structural and semantic boundary violations are rejected. JSON Schema validates object shape; `check-domain-contracts.py` owns cross-object workflow truth.