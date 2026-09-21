# Release 0 local API

The Release 0 API is a thin HTTP adapter over the deterministic workflow core.
It serves the same-origin owner UI and exercises the complete manual
California ESOP exploration path.

> **Development only.** The server binds to loopback. `X-Forhemit-Actor-ID`
> is an explicit testing seam, not authentication. Do not expose this server
> to a network or use real customer data.

## Start

```bash
python -m app.api \
  --workspace-db .forhemit/workspace.sqlite \
  --collaboration-db .forhemit/collaboration.sqlite
```

The default address is `http://127.0.0.1:8765`; open it to use the owner UI.
Only `127.0.0.1` and `localhost` are accepted as bind and Host values; every
other value is rejected. Static responses use a restrictive CSP and other
browser hardening headers.

## Request rules

All command requests require:

```text
Content-Type: application/json
X-Forhemit-Actor-ID: <development actor ID>
Idempotency-Key: <stable command key>
```

An idempotency key is bound to the complete command. Repeating the same command
returns the original result; reusing the key with altered fields returns
`409 conflict`.

The owner status route requires the actor header. `/health` is the only route
that does not.

The maximum JSON body is 1 MiB. Responses are JSON, use `Cache-Control:
no-store`, and identify the development identity mode through
`X-Forhemit-Auth-Mode`.

## Five-stage flow

| Stage | Endpoint | Actor |
|---|---|---|
| Destination | `POST /v1/destinations/confirm` | owner |
| Business Snapshot | `POST /v1/business-snapshots/confirm` | owner |
| ESOP Scenario Exploration | `POST /v1/scenarios` | owner |
| Professional Review Package | `POST /v1/professional-consents`, then `POST /v1/review-packages/authorize` | owner |
| Professional Review | `POST /v1/review-packages/{package_id}/versions/{version}/responses`, then `POST /v1/professional-determinations/{determination_id}/acknowledgments` | assigned professional, then owner |

Purpose-limited consent can be revoked with:

```text
POST /v1/professional-consents/{consent_id}/revoke
```

A queued package is not delivered if the referenced consent has expired, has
been revoked, or is no longer the current consent version.

## Guided owner status

```text
GET /v1/owners/{owner_id}/businesses/{business_id}/status
```

This returns all five stages, their current object references, and the next
manual action. A completed flow explicitly reports that no automatic
transaction handoff was created.

## Development worker endpoints

Release 0 has no background worker process yet. Tests and local development can
advance the transactional outboxes explicitly:

```text
POST /v1/dev/workers/deliver-packages
POST /v1/dev/workers/import-responses
X-Forhemit-Actor-ID: local-system
```

These routes are development scaffolding and are not a production operations
model.

## Health

```bash
curl http://127.0.0.1:8765/health
```

The response reports:

- development-header identity mode;
- Jev runtime disabled;
- service availability only, not database or security readiness.

## Error shape

```json
{
  "error": {
    "code": "invariant_failed",
    "message": "deterministic scenario gates failed"
  }
}
```

Expected status classes:

- `400` malformed transport request;
- `401` missing development actor;
- `403` failed actor authority;
- `404` unknown route or object;
- `409` command conflict;
- `413` body too large;
- `415` wrong content type;
- `422` deterministic or contract invariant failure.

## Production prerequisites

See [Production identity and session design](IDENTITY-AND-SESSIONS.md) for
the concrete authentication target.

Before any networked or real-data deployment:

- replace the actor header with authenticated identities and session/token
  validation;
- add authorization middleware and secure professional invitations;
- add TLS, host/origin controls, rate limits, request correlation, and safe
  structured logging;
- use a production connection/concurrency model;
- implement key management and encryption;
- move outbox delivery to supervised workers;
- complete retention/deletion, backup/recovery, monitoring, threat modeling,
  incident response, privacy/legal review, and penetration testing.