# List pagination (users)

List endpoints that accept **`page`**, **`page_size`**, and optional **`q`** follow [`money_app_shared::api::ListQuery`](../../shared/src/api/list_query.rs) validation.

## When / then

- **BLC-LP-001:** **`page`** is zero-based. Invalid values are rejected with **400** (`invalid_request` or `invalid_page_size`).

- **BLC-LP-002:** **`page_size`** is bounded (default and max defined in shared code). Oversized or zero values yield **400**.

- **BLC-LP-003:** List responses may include **`X-Total-Count`** and a **`Link`** header for pagination; see [`backend/src/request_link.rs`](../../backend/src/request_link.rs).

- **BLC-LP-004:** WHEN query parameters that must be non-negative integers are not parseable THEN the API responds **400**.
