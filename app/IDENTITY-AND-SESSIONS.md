# Production identity and session design

Status: **design target, not implemented**

The Release 0 development server uses `X-Forhemit-Actor-ID` only as a test seam.
A caller can choose any value, so it provides no authentication. This document
defines the minimum identity and session boundary required before a networked
pilot or real customer data.

## Security goals

The production identity layer must:

- establish who is acting without trusting caller-supplied identity fields;
- separate owner, invited professional, and system-worker identities;
- authorize every command against server-side relationships and current state;
- preserve the owner/private-workspace and professional-collaboration boundary;
- support revocation, expiration, reauthentication, and incident investigation;
- avoid exposing credentials or session material to application JavaScript;
- never treat professional identity as credential verification.

## Identity types

| Identity | Authentication | Authority source |
|---|---|---|
| Owner | OIDC Authorization Code + PKCE through a configured identity provider | Server-side owner/business membership |
| Professional | Purpose-limited invitation, then OIDC account binding | Current package assignment and invitation acceptance |
| Worker | Workload identity or narrowly scoped service credential | Explicit worker policy; never a human header |
| Support/admin | Separate privileged role with step-up authentication | Audited administrative policy; no implicit workspace access |

An authenticated professional remains **credential status unverified** until a
separate verification process records otherwise. Signing in proves control of
an identity account, not legal, tax, valuation, lending, investment, or
fiduciary qualifications.

## Browser session

Use a backend-for-frontend session:

1. Browser starts OIDC Authorization Code + PKCE.
2. Callback validates issuer, audience, state, nonce, code verifier, and token
   signatures against pinned provider configuration.
3. Server maps the external subject to an internal immutable identity.
4. Server stores provider tokens encrypted on the server, if they must be
   retained at all.
5. Browser receives only a random opaque session identifier in a cookie.

Cookie requirements:

```text
Secure; HttpOnly; SameSite=Lax; Path=/; no Domain attribute
```

Rotate the session identifier after login, reauthentication, role change, and
professional invitation acceptance. Use an idle timeout and an absolute
lifetime. Store only a hash of the session secret. Logout and security events
revoke the server-side session immediately.

The frontend must not store access tokens or session secrets in local storage,
session storage, IndexedDB, URLs, or readable cookies.

## Request authentication and CSRF

Production commands derive the actor exclusively from the validated server-side
session. Remove `X-Forhemit-Actor-ID` entirely.

For state-changing browser requests:

- require same-origin requests;
- validate `Origin` and, where needed, `Sec-Fetch-Site`;
- require a session-bound CSRF token in a custom header;
- keep strict request content types and size limits;
- continue payload-bound idempotency;
- reject identity, role, owner, or authority overrides from request bodies.

CORS is disabled by default. If a future client requires cross-origin access,
design a separate token audience and policy rather than loosening the browser
session boundary.

## Authorization model

Authentication occurs in transport middleware. Domain authority remains in the
application service.

Every request builds an immutable actor context:

```text
identity_id
session_id
identity_type
authentication_time
assurance_level
project/workspace memberships
request correlation_id
```

The server then checks:

- owner commands: actor is the current owner/member authorized for that business;
- professional response: actor is the exact assigned recipient for that package
  version and the package remains within its collaboration boundary;
- consent and disclosure: current version, recipient, purpose, resource,
  permission, status, and time window all match;
- worker actions: workload identity has only the named outbox capability;
- administrative actions: separate role plus step-up authentication.

Never infer authority from email domain, URL identifiers, UI visibility, or a
request-provided role.

## Professional invitation

A professional invitation must be single-purpose and single-recipient:

1. Owner identifies recipient and purpose.
2. Server creates a random, single-use invitation secret and stores only its
   hash, expiration, package assignment, intended recipient, and status.
3. Recipient authenticates.
4. If the invitation is email-bound, require the authenticated verified email
   to match; account for provider normalization explicitly.
5. Acceptance binds the internal professional identity to the package.
6. Rotate/revoke the invitation and record an audit event.
7. Any reassignment requires a new invitation, consent check, and disclosure
   decision.

Invitation links must not contain package contents and must be redacted from
logs and referrers.

## Reauthentication and high-risk actions

Require recent or step-up authentication for:

- granting or revoking professional-sharing consent;
- authorizing package disclosure;
- changing the professional recipient;
- exporting or deleting restricted data;
- changing identity-provider or recovery settings;
- privileged support/admin actions.

Owner acknowledgment is meaningful but is not a transaction approval and must
not become one through an authentication upgrade.

## Audit and privacy

Record:

- internal actor and session IDs;
- authentication and reauthentication events;
- invitation creation, acceptance, expiration, and revocation;
- authorization outcome and policy reason;
- consent/disclosure subject references;
- request correlation and idempotency keys;
- privileged access and session revocation.

Do not record passwords, tokens, cookies, invitation secrets, raw OIDC claims,
or unnecessary personal data. Security logs need integrity protection,
retention limits, access controls, and tested incident retrieval.

## Failure behavior

Fail closed when:

- session is absent, expired, revoked, or cannot be loaded;
- OIDC metadata, keys, issuer, audience, state, nonce, or PKCE validation fails;
- membership or package assignment is missing or stale;
- step-up authentication is required;
- consent is expired, revoked, superseded, or mismatched;
- worker identity or capability is unavailable.

Authentication-provider outage must not silently downgrade to the development
actor header.

## Recovery

Account recovery is a high-risk identity operation:

- prefer identity-provider recovery;
- require reauthentication before changing recovery factors;
- notify the account through an independent channel;
- revoke existing sessions after recovery;
- audit support involvement;
- prohibit support staff from impersonating users or reading private workspaces
  by default.

## Migration from the development API

Before enabling any non-loopback bind:

1. implement OIDC callback and opaque session store;
2. remove actor-header trust from every route;
3. add owner/business membership and professional invitation records;
4. add CSRF, Origin/Host validation, session rotation, expiration, and logout;
5. replace `local-system` with workload identity;
6. add security event logging, redaction, rate limits, and correlation IDs;
7. add negative tests for spoofing, fixation, CSRF, invitation replay, stale
   memberships, revoked sessions, and privilege escalation;
8. complete threat modeling, privacy/legal review, incident procedures, key
   management, backup/recovery, and independent security testing.

Until all eight are complete, the service remains loopback-only and uses
synthetic data.