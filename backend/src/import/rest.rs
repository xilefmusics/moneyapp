use actix_web::{post, web::Bytes, web::Data, HttpResponse};

use crate::database::Database;
use crate::error::AppError;
use crate::import::import;
use crate::resources::transaction::store::TransactionStore;
use crate::resources::User;

#[utoipa::path(
    post,
    path = "/api/v1/import",
    request_body = String,
    responses((status = 201, description = "Created transactions from CSV")),
    tag = "Import",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[post("/import")]
pub async fn post_import(
    user: actix_web::web::ReqData<User>,
    db: Data<Database>,
    payload: Bytes,
) -> Result<HttpResponse, AppError> {
    let txs = import(
        std::str::from_utf8(&payload).map_err(|e| AppError::invalid_request(e.to_string()))?,
    )?;
    let created = TransactionStore::create(db.into_inner(), &user.id, txs).await?;
    Ok(HttpResponse::Created().json(created))
}
