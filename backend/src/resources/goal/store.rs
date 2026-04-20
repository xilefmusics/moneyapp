use std::sync::Arc;

use money_app_shared::goal::Goal;
use serde::Deserialize;
use surrealdb::types::{RecordId, SurrealValue};

use crate::database::Database;
use crate::database::record_id_string;
use crate::error::AppError;

fn merge(id: RecordId, content: serde_json::Value) -> Result<Goal, AppError> {
    let mut v = content;
    if let Some(o) = v.as_object_mut() {
        o.remove("id");
    }
    let mut g: Goal = serde_json::from_value(v)
        .map_err(|e| AppError::invalid_request(format!("goal content: {e}")))?;
    g.id = Some(record_id_string(&id));
    Ok(g)
}

fn to_content(g: &Goal) -> Result<serde_json::Value, AppError> {
    let mut v = serde_json::to_value(g).map_err(|e| AppError::internal_from_err("serde goal", e))?;
    if let Some(o) = v.as_object_mut() {
        o.remove("id");
    }
    Ok(v)
}

pub struct GoalStore;

impl GoalStore {
    pub async fn list(db: Arc<Database>, user_id: &str) -> Result<Vec<Goal>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut response = db
            .db
            .query("SELECT id, content FROM goals WHERE owner = $owner")
            .bind(("owner", owner))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "goals.list", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "goals.list.take", e)
        })?;
        rows.into_iter().map(|r| merge(r.id, r.content)).collect()
    }

    pub async fn get_one(db: Arc<Database>, user_id: &str, id: &str) -> Result<Goal, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let gid = RecordId::new("goals", id.to_owned());
        let mut response = db
            .db
            .query("SELECT id, content FROM $gid WHERE owner = $owner LIMIT 1")
            .bind(("gid", gid))
            .bind(("owner", owner))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "goals.one", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let row: Option<Row> = response.take::<Option<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "goals.one.take", e)
        })?;
        let row = row.ok_or_else(|| AppError::NotFound("goal not found".into()))?;
        merge(row.id, row.content)
    }

    pub async fn put(
        db: Arc<Database>,
        user_id: &str,
        items: Vec<Goal>,
    ) -> Result<Vec<Goal>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::new();
        for g in items {
            let id = g
                .id
                .as_deref()
                .ok_or_else(|| AppError::invalid_request("goal id required for put"))?;
            let gid = RecordId::new("goals", id);
            let content = to_content(&g)?;
            let mut response = db
                .db
                .query("UPDATE $gid SET content = $content WHERE owner = $owner RETURN id, content")
                .bind(("gid", gid))
                .bind(("content", content))
                .bind(("owner", owner.clone()))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "goals.put", e))?;
            #[derive(Deserialize, SurrealValue)]
            struct Row {
                id: RecordId,
                content: serde_json::Value,
            }
            let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
                crate::log_and_convert!(AppError::database, "goals.put.take", e)
            })?;
            let row = rows
                .into_iter()
                .next()
                .ok_or_else(|| AppError::NotFound("goal not found on update".into()))?;
            out.push(merge(row.id, row.content)?);
        }
        Ok(out)
    }

    pub async fn delete(
        db: Arc<Database>,
        user_id: &str,
        items: Vec<Goal>,
    ) -> Result<Vec<Goal>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::new();
        for g in items {
            let id = g
                .id
                .as_deref()
                .ok_or_else(|| AppError::invalid_request("goal id required for delete"))?;
            let full = Self::get_one(db.clone(), user_id, id).await?;
            let gid = RecordId::new("goals", id);
            let response = db
                .db
                .query("DELETE $gid WHERE owner = $owner")
                .bind(("gid", gid))
                .bind(("owner", owner.clone()))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "goals.delete", e))?;
            let _ = response
                .check()
                .map_err(|e| crate::log_and_convert!(AppError::database, "goals.delete.check", e))?;
            out.push(full);
        }
        Ok(out)
    }

    pub async fn create(
        db: Arc<Database>,
        user_id: &str,
        items: Vec<Goal>,
    ) -> Result<Vec<Goal>, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let mut out = Vec::new();
        for g in items {
            let content = to_content(&g)?;
            let mut response = db
                .db
                .query("CREATE goals SET owner = $owner, content = $content RETURN id, content")
                .bind(("owner", owner.clone()))
                .bind(("content", content))
                .await
                .map_err(|e| crate::log_and_convert!(AppError::database, "goals.create", e))?;
            #[derive(Deserialize, SurrealValue)]
            struct Row {
                id: RecordId,
                content: serde_json::Value,
            }
            let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
                crate::log_and_convert!(AppError::database, "goals.create.take", e)
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
