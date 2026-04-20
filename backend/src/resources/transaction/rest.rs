use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query, ReqData},
    HttpResponse,
};

use crate::database::Database;
use crate::error::AppError;
use crate::resources::User;

use money_app_shared::transaction::Transaction;

use super::query_params::QueryParams;
use super::store::TransactionStore;

#[utoipa::path(
    get,
    path = "/api/v1/transactions",
    tag = "Transactions",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[get("")]
pub async fn get_transactions(
    user: ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionStore::list(
            db.into_inner(),
            &user.id,
            &q.into_inner().to_filter(),
            true,
        )
        .await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/transactions/{id}",
    tag = "Transactions",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[get("/{id}")]
pub async fn get_transaction(
    user: ReqData<User>,
    db: Data<Database>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionStore::get_one(db.into_inner(), &user.id, &id.into_inner()).await?,
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/transactions",
    tag = "Transactions",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[put("")]
pub async fn put_transactions(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Transaction>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        TransactionStore::put(db.into_inner(), &user.id, body.into_inner()).await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/transactions",
    tag = "Transactions",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[delete("")]
pub async fn delete_transactions(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Transaction>>,
) -> Result<HttpResponse, AppError> {
    TransactionStore::delete(db.into_inner(), &user.id, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    post,
    path = "/api/v1/transactions",
    tag = "Transactions",
    security(("SessionCookie" = []), ("SessionToken" = []))
)]
#[post("")]
pub async fn post_transactions(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Transaction>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        TransactionStore::create(db.into_inner(), &user.id, body.into_inner()).await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/transactions/pods", tag = "Transactions", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/pods")]
pub async fn get_pods(
    user: ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionStore::associated_keys(
            db.into_inner(),
            &user.id,
            &q.into_inner().to_filter(),
            "pods",
        )
        .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/transactions/budgets", tag = "Transactions", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/budgets")]
pub async fn get_budgets(
    user: ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionStore::associated_keys(
            db.into_inner(),
            &user.id,
            &q.into_inner().to_filter(),
            "budgets",
        )
        .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/transactions/inbudgets", tag = "Transactions", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/inbudgets")]
pub async fn get_inbudgets(
    user: ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionStore::associated_keys(
            db.into_inner(),
            &user.id,
            &q.into_inner().to_filter(),
            "inbudgets",
        )
        .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/transactions/debts", tag = "Transactions", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/debts")]
pub async fn get_debts(
    user: ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionStore::associated_keys(
            db.into_inner(),
            &user.id,
            &q.into_inner().to_filter(),
            "debts",
        )
        .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/transactions/tags", tag = "Transactions", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/tags")]
pub async fn get_tags(
    user: ReqData<User>,
    db: Data<Database>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionStore::associated_keys(
            db.into_inner(),
            &user.id,
            &q.into_inner().to_filter(),
            "tags",
        )
        .await?,
    ))
}
