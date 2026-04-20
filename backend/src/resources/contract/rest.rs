use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ReqData},
    HttpResponse,
};

use crate::database::Database;
use crate::error::AppError;
use crate::resources::User;

use money_app_shared::contract::Contract;

use super::store::ContractStore;

#[utoipa::path(get, path = "/api/v1/contracts", tag = "Contracts", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("")]
pub async fn get_contracts(
    user: ReqData<User>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(ContractStore::list(db.into_inner(), &user.id).await?))
}

#[utoipa::path(get, path = "/api/v1/contracts/{id}", tag = "Contracts", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/{id}")]
pub async fn get_contract(
    user: ReqData<User>,
    db: Data<Database>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        ContractStore::get_one(db.into_inner(), &user.id, &id.into_inner()).await?,
    ))
}

#[utoipa::path(put, path = "/api/v1/contracts", tag = "Contracts", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[put("")]
pub async fn put_contracts(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Contract>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        ContractStore::put(db.into_inner(), &user.id, body.into_inner()).await?,
    ))
}

#[utoipa::path(delete, path = "/api/v1/contracts", tag = "Contracts", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[delete("")]
pub async fn delete_contracts(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Contract>>,
) -> Result<HttpResponse, AppError> {
    ContractStore::delete(db.into_inner(), &user.id, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(post, path = "/api/v1/contracts", tag = "Contracts", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[post("")]
pub async fn post_contracts(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Contract>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        ContractStore::create(db.into_inner(), &user.id, body.into_inner()).await?,
    ))
}
