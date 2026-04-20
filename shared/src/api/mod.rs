//! Minimal API helpers (pagination). Moneyapp does not ship the full worshipviewer API client.

mod list_query;
pub mod pagination_link;

pub use list_query::{ListQuery, PageQuery, PAGE_SIZE_DEFAULT, PAGE_SIZE_MAX};
pub use pagination_link::pagination_link_header;
