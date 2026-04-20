use super::create_extrapolation;
use super::QueryParams;

use crate::database::Database;
use crate::error::AppError;
use crate::resources::User;

use actix_web::{get, web::Data, web::Query, HttpResponse};
use chrono::{Datelike, Local};

#[utoipa::path(
    get,
    path = "/api/v1/extrapolation",
    tag = "Extrapolation",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[get("")]
pub async fn get_extrapolation(
    user: actix_web::web::ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        create_extrapolation(
            db.into_inner(),
            &user.id,
            q.year.unwrap_or(Local::now().year()),
            q.month.unwrap_or(Local::now().month()),
        )
        .await?,
    ))
}
