# Sessions

Sessions identify the caller after login. The server stores session rows in SurrealDB; the client may send the session id either as a **cookie** (default name configurable via **`COOKIE_NAME`**, often `sso_session`) or as **`Authorization: Bearer <session_id>`**.

## Cookie

- **BLC-SESS-001:** On successful OIDC callback or OTP verify, the API may **`Set-Cookie`** for the session id. Cookie flags (**`Secure`**, **`SameSite`**) follow **`COOKIE_SECURE`** and server defaults in [`CookieConfig`](../../backend/src/settings.rs).

- **BLC-SESS-002:** **`POST /auth/logout`** clears the session cookie and invalidates the session server-side when the session id is known.

## Bearer token

- **BLC-SESS-003:** For APIs and non-browser clients, **`Authorization: Bearer <session_id>`** is accepted where **`RequireUser`** runs, subject to the same validation as the cookie value.

## User-facing session APIs

Administrative and “current user” session listing under **`/api/v1/users`** follows the handlers in **`backend/src/resources/user/session/rest.rs`**. Admins may list or revoke sessions for other users; normal users see only their own sessions where enforced by handlers.
