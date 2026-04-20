use std::sync::Arc;

use money_app_shared::contract::Contract;
use serde::Deserialize;
use surrealdb::types::{RecordId, SurrealValue};

use crate::database::Database;
use crate::database::record_id_string;
use crate::error::AppError;

fn merge(id: RecordId, content: serde_json::Value) -> Result<Contract, AppError> {
    let mut v = content;
    if let Some(o) = v.as_object_mut() {
        o.remove("id");
    }
    let mut c: Contract = serde_json::from_value(v)
        .map_err(|e| AppError::invalid_request(format!("contract content: {e}")))?;
    c.id = Some(record_id_string(&id));
    Ok(c)
}

fn to_content(c: &Contract) -> Result<serde_json::Value, AppError> {
    let mut v = serde_json::to_value(c).map_err(|e| AppError::internal_from_err("serde contract", e))?;
    if let Some(o) = v.as_object_mut() {
        o.remove("id");
    }
    Ok(v)
}

pub struct ContractStore;

impl ContractStore {
    pub async fn list(db: Arc<Database>, user_id: &str) -> Result<Vec<Contract>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut response = db
            .db
            .query("SELECT id, content FROM contracts WHERE owner = $owner")
            .bind(("owner", owner))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "contracts.list", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "contracts.list.take", e)
        })?;
        rows.into_iter()
            .map(|r| merge(r.id, r.content))
            .collect()
    }

    pub async fn get_one(db: Arc<Database>, user_id: &str, id: &str) -> Result<Contract, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let cid = RecordId::new("contracts", id.to_owned());
        let mut response = db
            .db
            .query("SELECT id, content FROM $cid WHERE owner = $owner LIMIT 1")
            .bind(("cid", cid))
            .bind(("owner", owner))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "contracts.one", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let row: Option<Row> = response.take::<Option<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "contracts.one.take", e)
        })?;
        let row = row.ok_or_else(|| AppError::NotFound("contract not found".into()))?;
        merge(row.id, row.content)
    }

    pub async fn put(
        db: Arc<Database>,
        user_id: &str,
        items: Vec<Contract>,
    ) -> Result<Vec<Contract>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::new();
        for c in items {
            let id = c
                .id
                .as_deref()
                .ok_or_else(|| AppError::invalid_request("contract id required for put"))?;
            let cid = RecordId::new("contracts", id);
            let content = to_content(&c)?;
            let mut response = db
                .db
                .query("UPDATE $cid SET content = $content WHERE owner = $owner RETURN id, content")
                .bind(("cid", cid))
                .bind(("content", content))
                .bind(("owner", owner.clone()))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "contracts.put", e))?;
            #[derive(Deserialize, SurrealValue)]
            struct Row {
                id: RecordId,
                content: serde_json::Value,
            }
            let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
                crate::log_and_convert!(AppError::database, "contracts.put.take", e)
            })?;
            let row = rows
                .into_iter()
                .next()
                .ok_or_else(|| AppError::NotFound("contract not found on update".into()))?;
            out.push(merge(row.id, row.content)?);
        }
        Ok(out)
    }

    pub async fn delete(
        db: Arc<Database>,
        user_id: &str,
        items: Vec<Contract>,
    ) -> Result<Vec<Contract>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::new();
        for c in items {
            let id = c
                .id
                .as_deref()
                .ok_or_else(|| AppError::invalid_request("contract id required for delete"))?;
            let full = Self::get_one(db.clone(), user_id, id).await?;
            let cid = RecordId::new("contracts", id);
            let response = db
                .db
                .query("DELETE $cid WHERE owner = $owner")
                .bind(("cid", cid))
                .bind(("owner", owner.clone()))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "contracts.delete", e))?;
            let _ = response
                .check()
                .map_err(|e| crate::log_and_convert!(AppError::database, "contracts.delete.check", e))?;
            out.push(full);
        }
        Ok(out)
    }

    pub async fn create(
        db: Arc<Database>,
        user_id: &str,
        items: Vec<Contract>,
    ) -> Result<Vec<Contract>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::new();
        for c in items {
            let content = to_content(&c)?;
            let mut response = db
                .db
                .query("CREATE contracts SET owner = $owner, content = $content RETURN id, content")
                .bind(("owner", owner.clone()))
                .bind(("content", content))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "contracts.create", e))?;
            #[derive(Deserialize, SurrealValue)]
            struct Row {
                id: RecordId,
                content: serde_json::Value,
            }
            let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
                crate::log_and_convert!(AppError::database, "contracts.create.take", e)
            })?;
            let row = rows
                .into_iter()
                .next()
                .ok_or_else(|| AppError::database("create returned no row"))?;
            out.push(merge(row.id, row.content)?);
        }
        Ok(out)
    }
}
