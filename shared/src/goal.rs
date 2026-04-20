use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GoalData {
    #[serde(rename = "realWealth")]
    RealWealth(u32),
}

impl Default for GoalData {
    fn default() -> Self {
        GoalData::RealWealth(0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Goal {
    pub id: Option<String>,
    pub due: DateTime<Local>,
    pub data: GoalData,
}

