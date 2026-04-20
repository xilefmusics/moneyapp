use chrono::{DateTime, Local, Months};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum State {
    #[default]
    Active,
    Terminated,
    Expired,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentKind {
    Active,
    #[default]
    Debit,
    Credit,
    PayPal,
    GooglePay,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Payment {
    pub first: DateTime<Local>,
    pub amount: u32,
    pub fix: bool,
    pub cycle: u32, // in months
    pub kind: PaymentKind,
    pub pod: String,
    pub debts: HashMap<String, u32>,
}

impl Payment {
    pub fn has_income(&self) -> bool {
        match self.kind {
            PaymentKind::Credit => true,
            _ => false,
        }
    }

    pub fn debt_sum(&self) -> u32 {
        self.debts.values().cloned().sum()
    }

    pub fn amount(&self) -> u32 {
        self.amount - self.debt_sum()
    }

    pub fn signed_amount(&self) -> i64 {
        if self.has_income() {
            self.amount() as i64
        } else {
            -(self.amount() as i64)
        }
    }

    pub fn signed_monthly_amount(&self) -> i64 {
        if self.cycle == 0 {
            self.signed_amount()
        } else {
            self.signed_amount() / (self.cycle as i64)
        }
    }

    pub fn date_of_next_payment(&self) -> DateTime<Local> {
        let current = Local::now();
        let mut result = self.first;
        let cycle = if self.cycle > 0 { self.cycle } else { 1 };
        while result < current {
            result = result + Months::new(cycle);
        }
        result
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Contract {
    pub id: Option<String>,
    pub title: String,
    pub partner: String,
    pub start: DateTime<Local>,
    pub state: State,
    pub term: u32,   // in months (Mindestvertragslaufzeit)
    pub notice: u32, // in months (Kündigungsfrist)
    pub management: String,
    pub payment: Payment,
    pub attachments: Vec<String>,
}

