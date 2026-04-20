use crate::error::AppError;
use money_app_shared::transaction::{Transaction, Type as TType};

use std::collections::HashMap;

use chrono::{DateTime, Local};
use csv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Type {
    #[serde(rename = "Überweisung")]
    Transfer,
    #[serde(rename = "Gutschrift")]
    CreditNote,
    #[serde(rename = "Lastschrift")]
    Debit,
    #[serde(rename = "MasterCardZahlung")]
    CardPayment,
}

#[derive(Deserialize, Debug)]
pub struct N26 {
    #[serde(rename = "Datum", with = "crate::serde_custom::yyyy_mm_dd")]
    date: DateTime<Local>,
    #[serde(rename = "Empfänger")]
    receiver: String,
    #[serde(rename = "Kontonummer")]
    account_number: String,
    #[serde(rename = "Transaktionstyp")]
    ttype: Type,
    #[serde(rename = "Verwendungszweck")]
    purpose: String,
    #[serde(rename = "Betrag (EUR)", with = "crate::serde_custom::amount")]
    amount: usize,
    #[serde(rename = "Betrag (Fremdwährung)")]
    foreign_amount: Option<f64>,
    #[serde(rename = "Fremdwährung")]
    foreign_currency: Option<String>,
    #[serde(rename = "Wechselkurs")]
    exchange_rate: Option<f64>,
}

impl N26 {
    fn from_str(s: &str) -> Result<Vec<Self>, AppError> {
        Ok(csv::Reader::from_reader(s.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Self>, csv::Error>>()?)
    }

    pub fn to_tags(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("bank".into(), "n26".into());
        map.insert("iban".into(), self.account_number);
        map.insert("purpose_of_use".into(), self.purpose);
        map.insert("receiver".into(), self.receiver);
        map.insert(
            "foreign_currency".into(),
            self.foreign_currency.unwrap_or("".into()),
        );
        map.insert(
            "foreign_amount".into(),
            self.foreign_amount
                .map(|a| a.to_string())
                .unwrap_or("".into()),
        );
        map.insert(
            "exchange_rate".into(),
            self.exchange_rate
                .map(|a| a.to_string())
                .unwrap_or("".into()),
        );
        map
    }

    pub fn to_transaction(self) -> Result<Transaction, AppError> {
        let (ttype, sender, receiver) = if self.receiver.starts_with("Von") {
            let mut iter = self.receiver.split(" ");
            let _von = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no Von".to_string()))?;
            let sender = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no sender".to_string()))?;
            let _nach = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no nach".to_string()))?;

            let receiver = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no receiver".to_string()))?;

            Ok::<(TType, Option<String>, Option<String>), AppError>((
                TType::Move,
                Some(sender.to_string()),
                Some(receiver.to_string()),
            ))
        } else {
            Ok(match self.ttype {
                Type::CreditNote => (TType::In, None, Some("N26 Hauptkonto".to_string())),
                _ => (TType::Out, Some("N26 Hauptkonto".to_string()), None),
            })
        }?;

        Ok(Transaction {
            id: None,
            ttype,
            date: self.date,
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
        let input = "\"Datum\",\"Empfänger\",\"Kontonummer\",\"Transaktionstyp\",\"Verwendungszweck\",\"Betrag (EUR)\",\"Betrag (Fremdwährung)\",\"Fremdwährung\",\"Wechselkurs\"
\"2023-12-01\",\"Receiver 1\",\"DE123...456\",\"Überweisung\",\"Transaction 1\",\"-123.4\",\"\",\"\",\"\"
\"2023-12-02\",\"Receiver 2\",\"DE123...456\",\"Lastschrift\",\"Transaction 2\",\"-8.99\",\"\",\"\",\"\"
";
        println!("{:?}", N26::parse_transactions(input).unwrap());
    }
}
