# Backend request flow

End-to-end path of an HTTP request through the Money App backend — from Actix entry to SurrealDB and back.

---

## Logging and observability

Subscriber setup lives in [`backend/src/observability.rs`](../../backend/src/observability.rs): `observability::init()` runs once from `main`, installs `tracing-subscriber` with an [`EnvFilter`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) (default `info`, overridable with **`RUST_LOG`**), and enables the **`tracing-log`** bridge so crates that use the `log` facade (Actix, SurrealDB, lettre, etc.) emit into the same sink.

**Output format:** With **`LOG_FORMAT=json`**, **`LOG_FORMAT=compact`**, or **`LOG_FORMAT=pretty`**, operators force a formatter regardless of environment. If `LOG_FORMAT` is unset, JSON is used when **`MONEYAPP_PRODUCTION`** is truthy or **`RUST_ENV=production`**, otherwise compact (human-friendly) lines.

**`RUST_LOG` examples:**

- `RUST_LOG=info` — default.
- `RUST_LOG=backend=debug,info` — debug for this crate only.
- `RUST_LOG=backend::auth=trace,surrealdb=info` — verbose auth, quieter database logs.

**Request correlation:** [`tracing-actix-web`](https://docs.rs/tracing-actix-web) builds a root span per HTTP request via [`MoneyRootSpan`](../../backend/src/request_id.rs). The request-id middleware stores the same id in request extensions (for Problem Details `instance`) and echoes it as **`X-Request-Id`**. If the client sends a W3C **`traceparent`** header, its span id is preferred as the request id; otherwise a UUID is generated. Authenticated requests record **`user_id`** on the current span from [`RequireUser`](../../backend/src/auth/middleware.rs).

### Canonical log fields

Use these names on new spans and structured log lines so aggregators and grep stay consistent:

| Field | Type | Meaning |
|-------|------|---------|
| `request_id` | string | UUID or W3C `traceparent` span id; set on the root HTTP span. |
| `user_id` | string | Authenticated user id; recorded after session validation. |
| `session_id` | string | Session id being created, validated, or revoked. |
| `route` | string | Matched Actix route pattern (e.g. `/api/v1/transactions`). |
| `method` | string | HTTP method. |
| `status` | u16 | HTTP response status code. |
| `latency_ms` | u64 | Total request latency in milliseconds. |
| `event` | string | Stable event name (`startup`, or `audit.*`). |
| `audit` | bool | `true` on audit lines emitted via `audit!`. |
| `error` | Display | Primary error message (`%err`). |

---

## Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  HTTP Request                                                                   │
└──────────┬──────────────────────────────────────────────────────────────────────┘
           │
           ▼
┌──────────────────────┐
│  Middleware stack     │  RequestId → HttpAudit → TracingLogger::<MoneyRootSpan>
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐     Public: GET /api/v1/about, /api/docs/*, /auth/* (varies)
│  Route matching       │     Protected /api/v1/*: RequireUser
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  RequireUser          │  Validates session (cookie or Bearer), injects ReqData<User>
│  (under /api/v1)      │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  Handlers             │  Domain: transactions, contracts, goals, history, …
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  SurrealDB            │  Queries scoped by authenticated user id (owner)
└──────────────────────┘
```

Unauthenticated **`/auth/*`** flows (OIDC redirect, callback, OTP, logout) and **`/api/docs/*`** bypass **`RequireUser`**. Versioned JSON API surface is **`/api/v1`** — see [api-documentation.md](../business-logic-constraints/api-documentation.md).

---

## Related

- [Backend resource architecture](./backend-resource.md) — layering of `resources/<name>/` modules.
- [Authentication](../business-logic-constraints/authentication.md) — BLC rules for callers.
- [Session](../business-logic-constraints/session.md) — cookie and bearer behavior.
