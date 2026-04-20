use std::sync::Arc;

use money_app_shared::attachment::Attachment;
use serde::Deserialize;
use surrealdb::types::{RecordId, SurrealValue};

use crate::database::Database;
use crate::database::record_id_string;
use crate::error::AppError;

fn merge(id: RecordId, content: serde_json::Value) -> Result<Attachment, AppError> {
    let mut v = content;
    if let Some(o) = v.as_object_mut() {
        o.remove("id");
    }
    let mut a: Attachment = serde_json::from_value(v)
        .map_err(|e| AppError::invalid_request(format!("attachment content: {e}")))?;
    a.id = Some(record_id_string(&id));
    Ok(a)
}

fn to_content(a: &Attachment) -> Result<serde_json::Value, AppError> {
    let mut v = serde_json::to_value(a).map_err(|e| AppError::internal_from_err("serde attachment", e))?;
    if let Some(o) = v.as_object_mut() {
        o.remove("id");
    }
    Ok(v)
}

pub struct AttachmentStore;

impl AttachmentStore {
    pub async fn create(
        db: Arc<Database>,
        user_id: &str,
        attachment: Attachment,
    ) -> Result<Attachment, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let content = to_content(&attachment)?;
        let mut response = db
            .db
            .query("CREATE attachments SET owner = $owner, content = $content RETURN id, content")
            .bind(("owner", owner))
            .bind(("content", content))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "attachments.create", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let rows: Vec<Row> = response.take::<Vec<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "attachments.create.take", e)
        })?;
        let row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::database("create returned no row"))?;
        merge(row.id, row.content)
    }

    pub async fn get_one(db: Arc<Database>, user_id: &str, id: &str) -> Result<Attachment, AppError> {
        let owner = RecordId::new("user", user_id.to_owned());
        let aid = RecordId::new("attachments", id.to_owned());
        let mut response = db
            .db
            .query("SELECT id, content FROM $aid WHERE owner = $owner LIMIT 1")
            .bind(("aid", aid))
            .bind(("owner", owner))
            .await
            .map_err(|e| crate::log_and_convert!(AppError::database, "attachments.one", e))?;
        #[derive(Deserialize, SurrealValue)]
        struct Row {
            id: RecordId,
            content: serde_json::Value,
        }
        let row: Option<Row> = response.take::<Option<Row>>(0).map_err(|e| {
            crate::log_and_convert!(AppError::database, "attachments.one.take", e)
        })?;
        let row = row.ok_or_else(|| AppError::NotFound("attachment not found".into()))?;
        merge(row.id, row.content)
    }
}
