use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{
    get, post,
    web::{Bytes, Data, Path, ReqData},
    HttpRequest, HttpResponse,
};

use crate::database::Database;
use crate::error::AppError;
use crate::resources::User;
use crate::settings::Settings;

use money_app_shared::attachment::Attachment;

use super::store::AttachmentStore;

fn file_path(settings: &Settings, attachment: &Attachment) -> Result<PathBuf, AppError> {
    let id = attachment
        .id
        .as_deref()
        .ok_or_else(|| AppError::invalid_request("attachment id missing"))?;
    Ok(PathBuf::from(&settings.attachment_dir).join(format!("{}.{}", id, attachment.extension())))
}

#[utoipa::path(
    get,
    path = "/api/v1/attachments/{id_or_path}",
    tag = "Attachments",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[get("/attachments/{id_or_path}")]
pub async fn get_attachment(
    user: ReqData<User>,
    db: Data<Database>,
    settings: Data<Settings>,
    id_or_path: Path<String>,
) -> Result<NamedFile, AppError> {
    let raw = id_or_path.into_inner();
    let id = raw.split('.').next().unwrap_or(&raw);
    let att = AttachmentStore::get_one(db.into_inner(), &user.id, id).await?;
    NamedFile::open(file_path(settings.get_ref(), &att)?).map_err(|e| AppError::NotFound(e.to_string()))
}

#[utoipa::path(
    post,
    path = "/api/v1/attachments",
    tag = "Attachments",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[post("/attachments")]
pub async fn post_attachment(
    user: ReqData<User>,
    db: Data<Database>,
    settings: Data<Settings>,
    req: HttpRequest,
    payload: Bytes,
) -> Result<HttpResponse, AppError> {
    let content_type = req
        .headers()
        .get(actix_web::http::header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::invalid_request("missing Content-Type"))?
        .to_string();

    let attachment = Attachment {
        id: None,
        content_type,
    };

    let created = AttachmentStore::create(db.into_inner(), &user.id, attachment).await?;

    let path = file_path(settings.get_ref(), &created)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::internal_from_err("mkdir attachment", e))?;
    }
    let mut file = File::create(&path).map_err(|e| AppError::internal_from_err("attachment file", e))?;
    file
        .write_all(&payload)
        .map_err(|e| AppError::internal_from_err("attachment write", e))?;

    Ok(HttpResponse::Created().json(created))
}
