use super::{History, QueryParams};

use crate::error::AppError;
use crate::resources::User;

use actix_web::{get, web::Data, web::Query, HttpResponse};

use crate::database::Database;

#[utoipa::path(
    get,
    path = "/api/v1/history/wealth",
    tag = "History",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[get("/wealth")]
pub async fn get_wealth(
    user: actix_web::web::ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        History::wealth(db.into_inner(), &user.id, q.into_inner()).await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/history/pods", tag = "History", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/pods")]
pub async fn get_pods(
    user: actix_web::web::ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        History::pod(db.into_inner(), &user.id, q.into_inner()).await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/history/budgets", tag = "History", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/budgets")]
pub async fn get_budgets(
    user: actix_web::web::ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        History::budget(db.into_inner(), &user.id, q.into_inner()).await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/history/inbudgets", tag = "History", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/inbudgets")]
pub async fn get_inbudgets(
    user: actix_web::web::ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        History::inbudget(db.into_inner(), &user.id, q.into_inner()).await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/history/debts", tag = "History", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/debts")]
pub async fn get_debts(
    user: actix_web::web::ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        History::debt(db.into_inner(), &user.id, q.into_inner()).await?,
    ))
}
