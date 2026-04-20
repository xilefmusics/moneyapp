# Authentication for `/api/v1`

Applies to routes under **`/api/v1`** that require an authenticated user session (most reads and mutations after login). The OpenAPI document is described in [api-documentation.md](./api-documentation.md) (**BLC-DOCS-001**). Public routes such as **`GET /api/v1/about`** and **`GET /api/docs/*`** are documented separately.

## When / then

- **BLC-AUTH-001:** WHEN a caller uses a route that **requires authentication** without a valid **session** (neither **`Authorization: Bearer <session_id>`** nor the configured **session cookie**) THEN the API responds **401** with `application/problem+json`.

- **BLC-AUTH-002:** WHEN **`Authorization: Bearer <token>`** is present but **`<token>`** does not identify a valid, active session THEN the API responds **401** before evaluating resource rules that would yield **403** or **404**.

## Relation to sessions

Session lifecycle and session management under **`/api/v1/users/.../sessions`** are summarized in [session.md](./session.md).

## Email OTP

- **BLC-AUTH-OTP-001:** **`POST /auth/otp/request`** stores a short-lived hashed code and sends it out-of-band. Per-IP rate limits apply (server **`AUTH_RATE_LIMIT_RPS`**, **`AUTH_RATE_LIMIT_BURST`**).

- **BLC-AUTH-OTP-002:** **`POST /auth/otp/verify`** validates the code; after too many failures the code is invalidated (see server **`OTP_MAX_ATTEMPTS`**).

- **BLC-AUTH-OTP-003:** When **`MONEYAPP_OTP_ALLOW_SELF_SIGNUP`** is unset or true, a successful verify **may** create a user for a previously unknown email. When set to **`false`** / **`0`** / **`no`**, verify succeeds only if the user already exists; otherwise **400** (`invalid_request`).

## OIDC

- **BLC-AUTH-OIDC-001:** **`GET /auth/login`** starts the provider redirect flow (e.g. Google). **`GET /auth/callback`** completes the flow and establishes a session.

Configuration: **`OIDC_ISSUER_URL`**, **`OIDC_CLIENT_ID`**, **`OIDC_CLIENT_SECRET`**, **`OIDC_REDIRECT_URL`**, **`OIDC_SCOPES`** — see [`backend/src/settings.rs`](../../backend/src/settings.rs).
