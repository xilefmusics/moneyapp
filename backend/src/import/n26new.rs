use crate::error::AppError;
use money_app_shared::transaction::{Transaction, Type as TType};

use std::collections::HashMap;

use chrono::{DateTime, Local};
use csv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Type {
    #[serde(rename = "Debit Transfer")]
    DebitTransfer,
    #[serde(rename = "Credit Transfer")]
    CreditTransfer,
    #[serde(rename = "Direct Debit")]
    DirectDebit,
    #[serde(rename = "Presentment")]
    MoneyBeam,
    #[serde(rename = "MoneyBeam")]
    Presentment,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Self::DebitTransfer => "Debit Transfer",
            Self::CreditTransfer => "Credit Transfer",
            Self::DirectDebit => "Direct Debit",
            Self::Presentment => "Presentment",
            Self::MoneyBeam => "MoneyBeam",
        }
        .to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct N26new {
    #[serde(rename = "Booking Date", with = "crate::serde_custom::yyyy_mm_dd")]
    booking_date: DateTime<Local>,
    #[serde(rename = "Value Date", with = "crate::serde_custom::yyyy_mm_dd")]
    value_date: DateTime<Local>,
    #[serde(rename = "Partner Name")]
    partner_name: String,
    #[serde(rename = "Partner Iban")]
    partner_iban: String,
    #[serde(rename = "Type")]
    ttype: Type,
    #[serde(rename = "Payment Reference")]
    payment_reference: String,
    #[serde(rename = "Account Name")]
    account_name: String,
    #[serde(rename = "Amount (EUR)", with = "crate::serde_custom::amount")]
    amount: usize,
    #[serde(rename = "Original Amount")]
    original_amount: Option<f64>,
    #[serde(rename = "Original Currency")]
    original_currency: Option<String>,
    #[serde(rename = "Exchange Rate")]
    exchange_rate: Option<f64>,
}

impl N26new {
    fn from_str(s: &str) -> Result<Vec<Self>, AppError> {
        Ok(csv::Reader::from_reader(s.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Self>, csv::Error>>()?)
    }

    pub fn to_tags(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("bank".into(), "n26".into());
        map.insert("booking_date".into(), self.booking_date.to_string());
        map.insert("value_date".into(), self.value_date.to_string());
        map.insert("partner_name".into(), self.partner_name);
        map.insert("partner_iban".into(), self.partner_iban);
        map.insert("type".into(), self.ttype.to_string());
        map.insert("payment_reference".into(), self.payment_reference);
        map.insert("account_name".into(), self.account_name);
        map.insert("amount".into(), self.amount.to_string());
        map.insert(
            "original_amount".into(),
            self.original_amount
                .map(|a| a.to_string())
                .unwrap_or("".to_string()),
        );
        map.insert(
            "original_currency".into(),
            self.original_currency
                .map(|a| a.to_string())
                .unwrap_or("".to_string()),
        );
        map.insert(
            "exchange_rate".into(),
            self.exchange_rate
                .map(|a| a.to_string())
                .unwrap_or("".to_string()),
        );
        map
    }

    pub fn to_transaction(self) -> Result<Transaction, AppError> {
        let (ttype, sender, receiver) = match self.ttype {
            Type::CreditTransfer => (TType::In, None, Some("N26 Hauptkonto".to_string())),
            _ => (TType::Out, Some("N26 Hauptkonto".to_string()), None),
        };

        Ok(Transaction {
            id: None,
            ttype,
            date: self.value_date,
            amount: self.amount,
            sender,
            receiver,
            budgets: HashMap::new(),
            inbudgets: HashMap::new(),
            debts: HashMap::new(),
            tags: self.to_tags(),
            attachments: Vec::new(),
        })
    }

    pub fn parse_transactions(s: &str) -> Result<Vec<Transaction>, AppError> {
        Self::from_str(s)?
            .into_iter()
            .map(|n26| n26.to_transaction())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_transactions() {
        let input = r#""Booking Date","Value Date","Partner Name","Partner Iban","Type","Payment Reference","Account Name","Amount (EUR)","Original Amount","Original Currency","Exchange Rate"
"2024-10-01","2024-10-01","Partner A","DEXXXXXXXXXXXXXXXXXX","Debit Transfer","Rent October 2024","Main Account",-980.00,,,
"2024-10-10","2024-10-10","Telecom Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference A","Main Account",-8.99,,,
"2024-10-14","2024-10-14","Utility Company","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference B","Main Account",-44.90,,,
"2024-10-15","2024-10-15","Broadcasting Service","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Broadcast Contribution","Main Account",-55.08,,,
"2024-10-17","2024-10-17","Mortgage Bank","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference C","Main Account",-50.00,,,
"2024-10-24","2024-10-24","Energy Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Direct-Debit","Main Account",-46.08,,,
"2024-10-28","2024-10-28","Employer","DEXXXXXXXXXXXXXXXXXX","Credit Transfer","Salary October","Main Account",2918.88,,,
"2024-10-31","2024-10-31","Credit Card Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Card Payment","Main Account",-2000.00,,,
"2024-11-01","2024-11-01","Partner A","DEXXXXXXXXXXXXXXXXXX","Debit Transfer","Rent November 2024","Main Account",-980.00,,,
"2024-11-01","2024-11-01","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription A","Main Account",-474.00,,,
"2024-11-04","2024-11-04","Private Transfer","DEXXXXXXXXXXXXXXXXXX","Credit Transfer","","Main Account",600.00,,,
"2024-11-04","2024-11-04","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription B","Main Account",-4.99,,,
"2024-11-05","2024-11-05","Online Store","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Order 1","Main Account",-31.83,,,
"2024-11-05","2024-11-05","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription C","Main Account",-37.99,,,
"2024-11-05","2024-11-05","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Service Payment","Main Account",-5.99,,,
"2024-11-05","2024-11-05","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription D","Main Account",-34.99,,,
"2024-11-06","2024-11-06","Online Payments","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Order 2","Main Account",-5.99,,,
"2024-11-08","2024-11-08","Telecom Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference D","Main Account",-8.99,,,
"2024-11-09","2024-11-08","Retail Store","N/A","Presentment","","Main Account",-61.86,61.86,"EUR",1.0
"#;
        println!("{:?}", N26new::parse_transactions(input).unwrap());
    }
}
