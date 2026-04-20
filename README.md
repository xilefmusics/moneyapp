# Money App

The Money App is an app for analyzing, visualizing and predicting assets, expenses and income.
Transactions from different banks can be imported and also created manually.
These transactions can then be visualized and evaluated in dashboards.
In addition, a potential forecast can be made by creating contracts and financial targets.
In short, the aim of this app is to get control and an overview of your finances and to have all the important information about them in one place.

## Table of contents

- [Architecture](#architecture)
- [Documentation in `docs/`](#documentation-in-docs)
- [Configuration](#configuration)
- [Production (Docker image)](#production-docker-image)
- [Local development](#local-development)
- [License](#license)

## Architecture

The app is a Rust **backend** ([Actix Web](https://actix.rs/)) and a **frontend** ([Yew](https://yew.rs/)), shipped together in a single Docker image that serves the WASM UI and the API on one origin.

| Layer | Details |
|-------|---------|
| **Database** | [SurrealDB](https://surrealdb.com/) **3.x**, schema from [`backend/db-migrations/`](./backend/db-migrations/) at startup |
| **HTTP API** | Versioned under **`/api/v1`**; OpenAPI + Swagger UI under **`/api/docs`** |
| **Auth** | **OIDC** (e.g. Google) and **email OTP**, with **cookie** and optional **Bearer** sessions — no `X-Remote-User` proxy |
| **Shared types** | [`shared/`](./shared/) crate (`money_app_shared`) |
| **UI helpers** | [fancy_yew](https://github.com/xilefmusics/fancy_yew) (path dependency; the root `Dockerfile` clones a pinned release) |

How requests move through the stack (tracing, `RequireUser`, SurrealDB) is described in [**docs/architecture/backend-request-flow.md**](docs/architecture/backend-request-flow.md). Domain module layout is in [**docs/architecture/backend-resource.md**](docs/architecture/backend-resource.md).

## Documentation in `docs/`

Longer-lived rules and diagrams live next to the code, in the same pattern as sibling projects:

| Path | Purpose |
|------|---------|
| [**docs/README.md**](docs/README.md) | Index of all documentation pages |
| [**docs/architecture/**](docs/architecture/) | Request flow, resource layering |
| [**docs/business-logic-constraints/**](docs/business-logic-constraints/) | Stable BLC identifiers for auth, sessions, HTTP contract, pagination, data scoping |

Start with [authentication](docs/business-logic-constraints/authentication.md) and [data scoping](docs/business-logic-constraints/data-scoping.md) if you are integrating clients.

## Configuration

Configuration is driven by **environment variables** (names match the `Settings` struct in [`backend/src/settings.rs`](backend/src/settings.rs), loaded with [envy](https://crates.io/crates/envy)). Highlights:

- **HTTP:** `HOST`, `PORT`, `STATIC_DIR`, `ATTACHMENT_DIR`
- **Cookies / session:** `POST_LOGIN_PATH`, `COOKIE_NAME`, `COOKIE_SECURE`, `SESSION_TTL_SECONDS`
- **OTP email:** `OTP_TTL_SECONDS`, `OTP_PEPPER`, `OTP_MAX_ATTEMPTS`; optional override **`MONEYAPP_OTP_ALLOW_SELF_SIGNUP`**. Mail uses Gmail SMTP via **`GMAIL_APP_PASSWORD`** and **`GMAIL_FROM`**
- **OIDC:** `OIDC_ISSUER_URL`, `OIDC_CLIENT_ID`, `OIDC_CLIENT_SECRET`, `OIDC_REDIRECT_URL`, `OIDC_SCOPES`
- **Database:** `DB_ADDRESS`, `DB_NAMESPACE`, `DB_DATABASE`, optional `DB_USERNAME` / `DB_PASSWORD`, `DB_MIGRATION_PATH`
- **Rate limits:** `AUTH_RATE_LIMIT_RPS`, `AUTH_RATE_LIMIT_BURST`, `API_RATE_LIMIT_RPS`, `API_RATE_LIMIT_BURST`
- **OpenAPI metadata:** `OPENAPI_CONTACT_EMAIL`, `OPENAPI_IMPRINT_URL`

**Logs:** Set [`RUST_LOG`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) for verbosity; use **`LOG_FORMAT=json`** (or `compact` / `pretty`) to force format. Production-style JSON logging is also selected when **`MONEYAPP_PRODUCTION`** or **`RUST_ENV=production`** is set (see [backend request flow](docs/architecture/backend-request-flow.md)).

**Safety:** The backend **refuses to start** if `INITIAL_ADMIN_USER_TEST_SESSION` is enabled while **`MONEYAPP_PRODUCTION`** or **`RUST_ENV=production`** is set.

## Production (Docker image)

The SPA and API are built into a single image by the root [`Dockerfile`](./Dockerfile). Run **SurrealDB 3** separately, then start the app container and point it at the database (see [`backend/src/settings.rs`](backend/src/settings.rs) for all variables).

Example: SurrealDB on the host, app in Docker reaching it via `host.docker.internal` (macOS/Windows; on Linux use the host gateway IP or run both on the same Docker network):

```bash
docker run --rm -p 8000:8000 surrealdb/surrealdb:v3.0 \
  start --log warn --bind 0.0.0.0:8000 memory
```

```bash
docker build -t moneyapp .
docker run --rm -p 8080:8080 \
  -e HOST=0.0.0.0 \
  -e PORT=8080 \
  -e DB_ADDRESS=http://host.docker.internal:8000 \
  -e DB_NAMESPACE=moneyapp \
  -e DB_DATABASE=moneyapp \
  -e OIDC_CLIENT_ID=... \
  -e OIDC_CLIENT_SECRET=... \
  -e OIDC_REDIRECT_URL=https://your-host/auth/callback \
  moneyapp
```

Set mail-related variables if you use OTP email. Adjust **`OIDC_REDIRECT_URL`** for your public URL and TLS. The SPA uses **`/login`** or **`/auth/login`** for OIDC; **`POST /auth/logout`** ends the session.

## Local development

The Yew frontend uses relative URLs such as **`/api/v1/...`** and **`/auth/...`**. If you run only **`trunk serve`** on one port and the API on another, the browser will hit the wrong origin for cookies and OIDC unless you unify origins. This matches the workflow documented for [Worship Viewer](https://github.com/xilefmusics/worshipviewer): same options, same **Caddy** reverse-proxy pattern.

| Approach | What you do |
|----------|-------------|
| **A — Reverse proxy (recommended for two processes)** | Run the backend on **`8080`**, Trunk on **`8081`**, and Caddy in front on **`8082`** so **`/api*`** and **`/auth*`** go to the backend and everything else to Trunk. See [Serve backend and frontend on the same port (Caddy)](#serve-backend-and-frontend-on-the-same-port-caddy-reverse-proxy). |
| **B — Single backend process** | `trunk build --release` in `frontend/`, then run the backend with **`STATIC_DIR`** pointing at Trunk’s output (commonly **`frontend/dist`**). |
| **C — Trunk proxy** | Configure Trunk’s **`[build.proxy]`** so API and auth paths reach the backend (see Trunk docs). |

The image in this repo is built with the toolchain pinned in the root **`Dockerfile`** (Rust bookworm image, **Trunk 0.21.14**).

### Install prerequisites

You need a recent **Rust** toolchain and the **wasm32** target for the frontend. **Node.js is not required**.

**macOS** (Homebrew):

```bash
brew install rustup
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install trunk
brew install caddy
```

**Linux / Windows:** Install [rustup](https://rustup.rs/), then `rustup update stable`, `rustup target add wasm32-unknown-unknown`, and `cargo install trunk`. Install Caddy 2 or another proxy if you use option A.

Ensure **`fancy_yew`** resolves for local builds (for example a path dependency to `../fancy_yew` in `frontend/Cargo.toml`, consistent with your checkout layout).

**Trunk and `NO_COLOR`:** Some environments set `NO_COLOR=1`, which can make Trunk fail. If that happens, run for example: `env -u NO_COLOR trunk serve --port 8081`.

### Start the backend

Enough for local work: the default **`DB_ADDRESS`** is **`mem://`** (in-memory SurrealDB inside the process). You do **not** need a separate database container unless you want data to survive restarts.

```bash
cd backend && \
  INITIAL_ADMIN_USER_EMAIL="admin@example.com" \
  INITIAL_ADMIN_USER_TEST_SESSION=true \
  cargo run
```

Defaults: **`HOST=127.0.0.1`**, **`PORT=8080`**, **`DB_NAMESPACE`/`DB_DATABASE`** default to **`app`/`app`** (see [`backend/src/settings.rs`](backend/src/settings.rs)). With the flags above, an admin user is created on first run and a **dev-only** session is created with id **`admin`** and cookie name **`sso_session`** (configurable). Use that cookie value when calling the API from tools or a browser extension, or sign in through the UI as you normally would.

**Production safety:** The backend **refuses to start** if **`INITIAL_ADMIN_USER_TEST_SESSION`** is set while **`MONEYAPP_PRODUCTION`** or **`RUST_ENV=production`** is set.

### Persist data across backend restarts (optional SurrealDB)

Run SurrealDB in Docker (or elsewhere), then point the backend at it:

```bash
docker run --rm -p 8000:8000 surrealdb/surrealdb:v3.0 \
  start --log warn --bind 0.0.0.0:8000 memory
```

```bash
cd backend && \
  DB_ADDRESS="http://127.0.0.1:8000" \
  DB_NAMESPACE="moneyapp" \
  DB_DATABASE="moneyapp" \
  DB_MIGRATION_PATH="db-migrations" \
  INITIAL_ADMIN_USER_EMAIL="admin@example.com" \
  INITIAL_ADMIN_USER_TEST_SESSION=true \
  cargo run
```

Set **`OIDC_*`**, **`GMAIL_*`**, and **`OTP_PEPPER`** as needed; see [Configuration](#configuration). If you use Caddy on port **8082** (below), set **`OIDC_REDIRECT_URL=http://127.0.0.1:8082/auth/callback`** (or your chosen listen address) so the OIDC provider matches the browser origin.

### Start the frontend

```bash
cd frontend && \
  trunk serve --port 8081
```

Use this together with option **A**, **B**, or **C** so the UI and API share one origin for session cookies.

### Serve backend and frontend on the same port (Caddy reverse proxy)

Same layout as Worship Viewer: backend **`localhost:8080`**, Trunk **`localhost:8081`**, Caddy **`8082`**. Money App also serves **`/auth/*`** (OIDC and OTP) on the backend, so those paths are proxied together with **`/api*`**.

```bash
echo '{
  "apps": {
    "http": {
      "servers": {
        "srv": {
          "listen": [":8082"],
          "routes": [
            {
              "match": [{"path": ["/api*"]}],
              "handle": [{
                "handler": "reverse_proxy",
                "upstreams": [{"dial": "localhost:8080"}]
              }]
            },
            {
              "match": [{"path": ["/auth*"]}],
              "handle": [{
                "handler": "reverse_proxy",
                "upstreams": [{"dial": "localhost:8080"}]
              }]
            },
            {
              "handle": [{
                "handler": "reverse_proxy",
                "upstreams": [{"dial": "localhost:8081"}]
              }]
            }
          ]
        }
      }
    }
  }
}' | caddy run --config -
```

Then open **`http://127.0.0.1:8082`**. Verify the JSON against your installed **Caddy 2** version if the config fails to load.

Same-origin behavior and tracing notes: [**docs/architecture/backend-request-flow.md**](docs/architecture/backend-request-flow.md).

## License

[![GPL-3.0](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
