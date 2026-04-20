# Cross-resource HTTP contract

Rules that apply across several **`/api/v1`** resources. Resource-specific validation stays in handler and store code; errors are mapped to **`AppError`** and serialized as **RFC 7807 Problem Details** (`application/problem+json`).

## Problem JSON

- **BLC-HTTP-000:** Error bodies use **`application/problem+json`** with fields aligned to [`money_app_shared::error::Problem`](../../shared/src/error/mod.rs) (`type`, `title`, `status`, `code`, `detail`, optional `instance`).

## Path parameters

- **BLC-HTTP-001:** WHEN a path segment that must match the API’s resource **id** format is syntactically invalid THEN the API responds **400** (same class of validation as list query integers in **BLC-LP-004**; see [list-pagination.md](./list-pagination.md)).

## Validation status codes

- **BLC-HTTP-003:** The API does **not** use **422 Unprocessable Entity**. Invalid JSON, unknown fields (`deny_unknown_fields`), and other request validation failures are **400 Bad Request** with `application/problem+json`.

## API rate limiting (`/api/v1/*`)

- **BLC-HTTP-004:** Versioned **`/api/v1/*`** routes are rate-limited **per client IP** (see **`API_RATE_LIMIT_RPS`** and **`API_RATE_LIMIT_BURST`** in [`backend/src/settings.rs`](../../backend/src/settings.rs)). WHEN the limit is exceeded THEN the API responds **429 Too Many Requests** with **`Retry-After`** and **`X-RateLimit-*`** headers ([`actix-governor`](https://docs.rs/actix-governor/latest/actix_governor/)).

## Auth rate limiting (`/auth/*`)

- **BLC-HTTP-005:** Sensitive **`/auth/*`** routes use **`AUTH_RATE_LIMIT_RPS`** and **`AUTH_RATE_LIMIT_BURST`**.
