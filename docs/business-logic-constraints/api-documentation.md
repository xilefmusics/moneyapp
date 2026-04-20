# API documentation (OpenAPI)

## When / then

- **BLC-DOCS-001:** WHEN **`GET /api/docs/openapi.json`** runs **without** authentication THEN the API responds **200** and returns the OpenAPI schema for the HTTP API (exact wire format may follow the generator’s JSON layout).

- **BLC-DOCS-002:** Swagger UI is served under **`/api/docs`** (see [`backend/src/docs.rs`](../../backend/src/docs.rs)).

This route is **outside** the authenticated **`/api/v1`** surface except where individual operations declare security in the schema; see [authentication.md](./authentication.md).
