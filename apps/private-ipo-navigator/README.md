# Private IPO Navigator

A runnable reference implementation of the Employee Ownership journey described
by the ForhemitDYI design documents.

It guides a business owner through:

1. Desired destination and nonnegotiables
2. Current business reality
3. Liquidity and financial preferences
4. Illustrative ownership-transition scenarios
5. A decision dashboard
6. A controlled document locker
7. Professional handoff

The app keeps the three decision layers distinct: owner objectives, platform
scenarios, and qualified professional determinations. All modeled values are
illustrative and are not legal, tax, valuation, lending, fiduciary, or
investment advice.

## Run locally

Requires Python 3.10+ and has no third-party runtime dependencies.

```bash
cd apps/private-ipo-navigator
python3 server.py
```

Open <http://localhost:4173>. The app creates `navigator.db` beside the server.

Optional environment variables:

| Variable | Default | Purpose |
| --- | --- | --- |
| `PORT` | `4173` | HTTP port |
| `DATABASE_PATH` | `./navigator.db` | SQLite database location |
| `MAX_DOCUMENT_BYTES` | `26214400` | Per-file locker upload limit |

## Run with Docker

```bash
docker build -t private-ipo-navigator .
docker run --rm -p 4173:4173 \
  -v private-ipo-data:/data \
  -e DATABASE_PATH=/data/navigator.db \
  private-ipo-navigator
```

Health check: `GET /readyz`.

## Identity and production integration

The reference app isolates state using a trusted identity header, in this order:

- `X-PromptQL-User-Id`
- `X-PromptQL-Visitor-Id`
- `X-Forwarded-User`

When none is present, it uses the shared development identity `default`.
Therefore, a production deployment **must** sit behind an authenticated proxy
that strips untrusted client-supplied identity headers and injects the verified
user identity. Do not expose the current server directly as a multi-user
production service.

The next production integration should replace the prototype's direct SQLite
contracts with authenticated application services for:

- Journey and scenario persistence
- Document/object storage and malware scanning
- Consent, stakeholder access grants, and audit events
- Professional review packages and determinations
- Valuation and financing data supplied by qualified professionals

The browser interface can remain the frontend while these API boundaries are
introduced incrementally.

## Document locker

The locker stores files in SQLite for the reference implementation. Each file
has a category, sensitivity level, intended-recipient list, and per-visitor
ownership. Files are never distributed automatically.

Production document handling should add object storage, encryption/KMS,
malware scanning, retention rules, immutable audit history, consent records,
and expiring download grants before handling live diligence materials.

## Tests

Run the dependency-free API smoke test:

```bash
python3 tests/smoke.py
```

If Node.js is installed, also validate the browser JavaScript:

```bash
node --check static/app.js
```
