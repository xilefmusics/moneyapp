# Backend resource architecture

Money App domain code follows a consistent layout: **DTOs** in the **`shared/`** crate and **resource modules** under **`backend/src/resources/<name>/`**. Auth and infrastructure (database, mail, governors) live alongside under `backend/src/`.

---

## High-level diagram

```
┌────────────────────────────────────────────────────────────────────┐
│  shared/src/                                                       │
│  transaction.rs, contract.rs, goal.rs, attachment.rs, user/, …    │
│  Platform-agnostic types (serde); optional utoipa for OpenAPI      │
└───────────────────────────────┬────────────────────────────────────┘
                                │  money_app_shared
                                ▼
┌────────────────────────────────────────────────────────────────────┐
│  backend/src/resources/<name>/                                     │
│                                                                    │
│  rest.rs          utoipa::path handlers, RequireUser where needed  │
│  store.rs         (or surreal_repo.rs) SurrealQL against Database  │
│  query_params.rs, filter.rs  … as needed per resource             │
└───────────────────────────────┬────────────────────────────────────┘
                                │
                                ▼
┌────────────────────────────────────────────────────────────────────┐
│  backend/src/database/          migrations in backend/db-migrations/│
└────────────────────────────────────────────────────────────────────┘
```

There is **no** team ACL layer: after authentication, data is scoped by **`owner`** (or equivalent) matching the **session user’s id**.

---

## File roles (typical domain resource)

| Piece | Role |
|-------|------|
| **`shared`** | `Serialize`/`Deserialize` models shared with the Yew frontend. |
| **`rest.rs`** | Actix handlers, `utoipa::path`, `ReqData<User>`, rate limits via `resources/rest.rs` scope. |
| **`store.rs`** | Database access: parameterized SurrealQL, `Arc<Database>`, user id for scoping. |
| **`filter.rs`** / **`query_params.rs`** | List filters and query parsing (e.g. transactions). |

User and session resources use **repository traits** and **`surreal_repo.rs`** in the worshipviewer style; money-specific aggregates (transactions, contracts, goals) often use a **`Store`** type with associated functions for clarity.

---

## Related

- [Backend request flow](./backend-request-flow.md) — middleware and tracing.
- [Data scoping](../business-logic-constraints/data-scoping.md) — single-user tenancy rules.
