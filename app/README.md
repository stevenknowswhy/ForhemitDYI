# Release 0 deterministic workflow core

This package is the first runnable implementation foundation for the locked
California ESOP exploration vertical slice.

## Scope

Implemented:

- immutable versions for owner destination, business snapshot, scenario,
  readiness, consent, review package, disclosure, and professional response;
- owner and assigned-professional authority checks;
- deterministic workflow gates with no Jev dependency;
- explicit preservation of unknown facts;
- purpose- and recipient-bound professional consent;
- disclosure records committed before an outbound package is deliverable;
- separate SQLite files for the private workspace and professional
  collaboration boundary;
- transactional audit/outbox writes;
- inbox deduplication and payload-bound command idempotency;
- source-version dependency invalidation;
- hash-linked append-only audit events;
- exact professional attribution;
- owner acknowledgment that does not imply agreement or create an automatic
  transaction handoff;
- a loopback-only [development HTTP API](API.md) with explicit idempotency,
  transport errors, development worker controls, and a guided five-stage owner
  status view.

Not yet implemented:

- production authentication or a user interface;
- encryption/key management;
- authentication and credential verification;
- background outbox workers;
- document vault;
- retention/deletion and backup/recovery;
- production observability and incident response;
- Jev shadow mode.

## Run tests

```bash
python -m unittest discover -s tests -p 'test_*.py' -v
```

Start the development API with:

```bash
python -m app.api
```

It binds to `127.0.0.1:8765`. The actor header is not authentication; see the
[API documentation](API.md) before use.

The runtime depends on `jsonschema` so every persisted domain object is checked
against the canonical repository contracts.

## Persistence boundaries

`WorkspaceStore` and `CollaborationStore` deliberately use separate SQLite
databases. Only the authorized package copy and its disclosure cross into the
collaboration database. The complete destination, business facts, consent
history, and private audit trail stay in the owner workspace.

This is a testable implementation foundation, not a production security claim.
Before real customer data, the operational minimum in the implementation
architecture remains mandatory.