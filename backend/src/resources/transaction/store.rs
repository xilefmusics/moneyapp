use std::sync::Arc;

use money_app_shared::transaction::{AssociatedTypeValues, Transaction, Type};
use serde::Deserialize;
use surrealdb::types::{RecordId, SurrealValue};

use crate::database::Database;
use crate::database::record_id_string;
use crate::error::AppError;

use super::Filter;

fn merge_id_into_transaction(id: RecordId, mut content: serde_json::Value) -> Result<Transaction, AppError> {
    if let Some(o) = content.as_object_mut() {
        o.remove("id");
    }
    let mut t: Transaction = serde_json::from_value(content).map_err(|e| {
        AppError::invalid_request(format!("transaction content: {e}"))
    })?;
    t.id = Some(record_id_string(&id));
    Ok(t)
}

fn transaction_to_content(t: &Transaction) -> Result<serde_json::Value, AppError> {
    let mut v = serde_json::to_value(t).map_err(|e| AppError::internal_from_err("serde transaction", e))?;
    if let Some(o) = v.as_object_mut() {
        o.remove("id");
    }
    Ok(v)
}

pub struct TransactionStore;

impl TransactionStore {
    pub async fn list(
        db: Arc<Database>,
        user_id: &str,
        filter: &Filter<'_>,
        reverse: bool,
    ) -> Result<Vec<Transaction>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut q = "SELECT id, content FROM transactions WHERE owner = $owner".to_string();
        for c in filter.conditions() {
            q.push_str(" AND ");
            q.push_str(&c);
        }
        q.push_str(" ORDER BY content.date");
        let mut response = db
            .db
            .query(q)
            .bind(("owner", owner))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "transactions.list", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "transactions.list.take", e)
        })?;
        let mut out: Vec<Transaction> = rows
            .into_iter()
            .map(|r| merge_id_into_transaction(r.id, r.content))
            .collect::<Result<_, _>>()?;
        if reverse {
            out.reverse();
        }
        Ok(out)
    }

    pub async fn get_one(db: Arc<Database>, user_id: &str, id: &str) -> Result<Transaction, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let tid = RecordId::new("transactions", id.to_owned());
        let mut response = db
            .db
            .query("SELECT id, content FROM $tid WHERE owner = $owner LIMIT 1")
            .bind(("tid", tid))
            .bind(("owner", owner))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "transactions.one", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let row: Option<Row> = response.take::<Option<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "transactions.one.take", e)
        })?;
        let row = row.ok_or_else(|| AppError::NotFound("transaction not found".into()))?;
        merge_id_into_transaction(row.id, row.content)
    }

    pub async fn put(
        db: Arc<Database>,
        user_id: &str,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::with_capacity(transactions.len());
        for t in transactions {
            let id = t
                .id
                .as_deref()
                .ok_or_else(|| AppError::invalid_request("transaction id required for put"))?;
            let tid = RecordId::new("transactions", id);
            let content = transaction_to_content(&t)?;
            let mut response = db
                .db
                .query(
                    "UPDATE $tid SET content = $content WHERE owner = $owner RETURN id, content",
                )
                .bind(("tid", tid))
                .bind(("content", content))
                .bind(("owner", owner.clone()))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "transactions.put", e))?;
            #[derive(Deserialize, SurrealValue)]
            struct Row {
                id: RecordId,
                content: serde_json::Value,
            }
            let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
                crate::log_and_convert!(AppError::database, "transactions.put.take", e)
            })?;
            let row = rows
                .into_iter()
                .next()
                .ok_or_else(|| AppError::NotFound("transaction not found on update".into()))?;
            out.push(merge_id_into_transaction(row.id, row.content)?);
        }
        Ok(out)
    }

    pub async fn delete(
        db: Arc<Database>,
        user_id: &str,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::new();
        for t in transactions {
            let id = t
                .id
                .as_deref()
                .ok_or_else(|| AppError::invalid_request("transaction id required for delete"))?;
            let full = Self::get_one(db.clone(), user_id, id).await?;
            let tid = RecordId::new("transactions", id);
            let response = db
                .db
                .query("DELETE $tid WHERE owner = $owner")
                .bind(("tid", tid))
                .bind(("owner", owner.clone()))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "transactions.delete", e))?;
            let _ = response
                .check()
                .map_err(|e| crate::log_and_convert!(AppError::database, "transactions.delete.check", e))?;
            out.push(full);
        }
        Ok(out)
    }

    pub async fn create(
        db: Arc<Database>,
        user_id: &str,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::with_capacity(transactions.len());
        for t in transactions {
            let content = transaction_to_content(&t)?;
            let mut response = db
                .db
                .query(
                    "CREATE transactions SET owner = $owner, content = $content RETURN id, content",
                )
                .bind(("owner", owner.clone()))
                .bind(("content", content))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "transactions.create", e))?;
            #[derive(Deserialize, SurrealValue)]
            struct Row {
                id: RecordId,
                content: serde_json::Value,
            }
            let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
                crate::log_and_convert!(AppError::database, "transactions.create.take", e)
            })?;
            let row = rows
                .into_iter()
                .next()
                .ok_or_else(|| AppError::database("create returned no row"))?;
            out.push(merge_id_into_transaction(row.id, row.content)?);
        }
        Ok(out)
    }

    pub async fn associated_keys(
        db: Arc<Database>,
        user_id: &str,
        filter: &Filter<'_>,
        associated_type: &str,
    ) -> Result<Vec<String>, AppError> {
        let txs = Self::list(db, user_id, filter, false).await?;
        Ok(if associated_type == "pods" {
            let mut keys: Vec<String> = txs
                .into_iter()
                .flat_map(|t| match t.ttype {
                    Type::In => t.receiver.into_iter().collect::<Vec<_>>(),
                    Type::Out => t.sender.into_iter().collect::<Vec<_>>(),
                    Type::Move => vec![
                        t.sender.unwrap_or_default(),
                        t.receiver.unwrap_or_default(),
                    ],
                })
                .filter(|s| !s.is_empty())
                .collect();
            keys.sort();
            keys.dedup();
            keys
        } else {
            let field = match associated_type {
                "budgets" => "budgets",
                "inbudgets" => "inbudgets",
                "debts" => "debts",
                "tags" => "tags",
                _ => associated_type,
            };
            let mut keys: Vec<String> = txs
                .into_iter()
                .flat_map(|t| match field {
                    "budgets" => t.budgets.keys().cloned().collect::<Vec<_>>(),
                    "inbudgets" => t.inbudgets.keys().cloned().collect::<Vec<_>>(),
                    "debts" => t.debts.keys().cloned().collect::<Vec<_>>(),
                    "tags" => t.tags.keys().cloned().collect::<Vec<_>>(),
                    _ => Vec::new(),
                })
                .collect();
            keys.sort();
            keys.dedup();
            keys
        })
    }

    pub async fn associated_type_values(
        db: Arc<Database>,
        user_id: &str,
        filter: &Filter<'_>,
        associated_type: &str,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        let txs = Self::list(db, user_id, filter, false).await?;
        if associated_type == "pods" {
            return Ok(txs
                .into_iter()
                .map(AssociatedTypeValues::from_transaction_pod)
                .collect());
        }
        if associated_type == "debts" {
            return Ok(txs
                .into_iter()
                .map(AssociatedTypeValues::from_transaction_debt)
                .collect());
        }
        let field = match associated_type {
            "budgets" => "budgets",
            "inbudgets" => "inbudgets",
            "debts" => "debts",
            "tags" => "tags",
            _ => associated_type,
        };
        Ok(txs
            .into_iter()
            .map(|t| {
                let data = match field {
                    "budgets" => t
                        .budgets
                        .into_iter()
                        .map(|(k, v)| (k, v as i64))
                        .collect(),
                    "inbudgets" => t
                        .inbudgets
                        .into_iter()
                        .map(|(k, v)| (k, v as i64))
                        .collect(),
                    "debts" => t.debts.into_iter().map(|(k, v)| (k, v as i64)).collect(),
                    "tags" => t
                        .tags
                        .into_iter()
                        .map(|(k, v)| (k, v.parse::<i64>().unwrap_or(0)))
                        .collect(),
                    _ => std::collections::HashMap::new(),
                };
                AssociatedTypeValues {
                    date: t.date,
                    data,
                }
            })
            .collect())
    }
}
