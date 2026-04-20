use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg_attr(feature = "backend", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    In,
    #[default]
    Out,
    Move,
}

#[cfg_attr(feature = "backend", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub ttype: Type,
    pub date: DateTime<Local>,
    pub amount: usize,
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub budgets: HashMap<String, usize>,
    pub inbudgets: HashMap<String, usize>,
    pub debts: HashMap<String, usize>,
    pub tags: HashMap<String, String>,
    pub attachments: Vec<String>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            id: Option::default(),
            ttype: Type::default(),
            date: Local::now(),
            amount: usize::default(),
            sender: Option::default(),
            receiver: Option::default(),
            budgets: HashMap::default(),
            inbudgets: HashMap::default(),
            debts: HashMap::default(),
            tags: HashMap::default(),
            attachments: Vec::default(),
        }
    }
}

impl Transaction {
    pub fn validate(&self) -> bool {
        match self.ttype {
            Type::In => self.amount as i64 == (self.debt_sum() + self.inbudget_sum()),
            Type::Out => self.amount as i64 == (self.debt_sum() + self.budget_sum()),
            Type::Move => {
                self.debt_sum() == 0 && self.budget_sum() == 0 && self.inbudget_sum() == 0
            }
        }
    }

    pub fn income(&self) -> i64 {
        match self.ttype {
            Type::In => (self.amount as i64) - self.debt_sum(),
            Type::Out => 0,
            Type::Move => 0,
        }
    }

    pub fn out(&self) -> i64 {
        match self.ttype {
            Type::In => 0,
            Type::Out => (self.amount as i64) - self.debt_sum(),
            Type::Move => 0,
        }
    }

    pub fn signed_amount(&self) -> i64 {
        match self.ttype {
            Type::In => self.amount as i64,
            Type::Out => -(self.amount as i64),
            Type::Move => 0,
        }
    }

    pub fn budget_sum(&self) -> i64 {
        self.budgets.values().sum::<usize>() as i64
    }

    pub fn inbudget_sum(&self) -> i64 {
        self.inbudgets.values().sum::<usize>() as i64
    }

    pub fn debt_sum(&self) -> i64 {
        self.debts.values().sum::<usize>() as i64
    }

    pub fn signed_debt_sum(&self) -> i64 {
        match self.ttype {
            Type::In => self.debt_sum(),
            Type::Out => -self.debt_sum(),
            Type::Move => 0,
        }
    }

    pub fn title(&self) -> String {
        match self.ttype {
            Type::In => self.receiver.clone().unwrap_or("".into()),
            Type::Out => self.sender.clone().unwrap_or("".into()),
            Type::Move => format!(
                "{} to {}",
                self.sender.clone().unwrap_or("".into()),
                self.receiver.clone().unwrap_or("".into())
            ),
        }
    }
}

#[cfg_attr(feature = "backend", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssociatedTypeValues {
    pub date: DateTime<Local>,
    pub data: HashMap<String, i64>,
}

impl AssociatedTypeValues {
    pub fn from_transaction_pod(transaction: Transaction) -> AssociatedTypeValues {
        let mut data = HashMap::<String, i64>::new();
        let amount = transaction.amount as i64;

        if let Some(receiver) = transaction.receiver {
            data.insert(receiver, amount);
        }

        if let Some(sender) = transaction.sender {
            data.insert(sender, -amount);
        }

        AssociatedTypeValues {
            date: transaction.date,
            data,
        }
    }

    pub fn from_transaction_debt(transaction: Transaction) -> AssociatedTypeValues {
        let multiplier = match transaction.ttype {
            Type::In => 1,
            Type::Out => -1,
            Type::Move => 0,
        };

        AssociatedTypeValues {
            date: transaction.date,
            data: transaction
                .debts
                .into_iter()
                .map(|(key, value)| (key, value as i64 * multiplier))
                .collect(),
        }
    }
}
