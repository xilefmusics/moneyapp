use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ReqData},
    HttpResponse,
};

use crate::database::Database;
use crate::error::AppError;
use crate::resources::User;

use money_app_shared::goal::Goal;

use super::store::GoalStore;

#[utoipa::path(get, path = "/api/v1/goals", tag = "Goals", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("")]
pub async fn get_goals(
    user: ReqData<User>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(GoalStore::list(db.into_inner(), &user.id).await?))
}

#[utoipa::path(get, path = "/api/v1/goals/{id}", tag = "Goals", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[get("/{id}")]
pub async fn get_goal(
    user: ReqData<User>,
    db: Data<Database>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        GoalStore::get_one(db.into_inner(), &user.id, &id.into_inner()).await?,
    ))
}

#[utoipa::path(put, path = "/api/v1/goals", tag = "Goals", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[put("")]
pub async fn put_goals(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Goal>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        GoalStore::put(db.into_inner(), &user.id, body.into_inner()).await?,
    ))
}

#[utoipa::path(delete, path = "/api/v1/goals", tag = "Goals", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[delete("")]
pub async fn delete_goals(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Goal>>,
) -> Result<HttpResponse, AppError> {
    GoalStore::delete(db.into_inner(), &user.id, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(post, path = "/api/v1/goals", tag = "Goals", security(("SessionCookie" = []), ("SessionToken" = [])))]
#[post("")]
pub async fn post_goals(
    user: ReqData<User>,
    db: Data<Database>,
    body: Json<Vec<Goal>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        GoalStore::create(db.into_inner(), &user.id, body.into_inner()).await?,
    ))
}
