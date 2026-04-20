# Data scoping (single-user tenancy)

Money App does **not** implement teams or shared collections. After **`RequireUser`** succeeds, all domain data (transactions, contracts, goals, attachments, and derived history) is scoped to **the authenticated user’s id**.

## When / then

- **BLC-SCOPE-001:** Rows in SurrealDB store an **`owner`** (or equivalent) pointing at the **`user`** record for the session’s user. Handlers pass **`user.id`** into store/query layers; callers cannot impersonate another user by request fields alone.

- **BLC-SCOPE-002:** **`GET`** by id returns **404** when the record does not exist **or** is not owned by the current user (avoid leaking existence across users).

- **BLC-SCOPE-003:** Mutations (**`PUT`**, **`POST`**, **`DELETE`**) apply only to resources owned by the current user.

See [authentication.md](./authentication.md) for how the session is established.
