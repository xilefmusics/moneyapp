use super::Filter;

use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct QueryParams {
    #[serde(default, with = "crate::serde_custom::yyyy_mm_dd_option")]
    pub start: Option<DateTime<Local>>,
    #[serde(default, with = "crate::serde_custom::yyyy_mm_dd_option")]
    pub end: Option<DateTime<Local>>,
    pub pod: Option<String>,
    pub debt: Option<String>,
    pub budget: Option<String>,
    pub inbudget: Option<String>,
    #[serde(rename = "type")]
    pub ttype: Option<String>,
}

impl QueryParams {
    pub fn to_filter<'a>(&'a self) -> Filter<'a> {
        Filter::new(
            self.start,
            self.end,
            self.pod.as_deref(),
            self.debt.as_deref(),
            self.budget.as_deref(),
            self.inbudget.as_deref(),
            self.ttype.as_deref(),
        )
    }
}
