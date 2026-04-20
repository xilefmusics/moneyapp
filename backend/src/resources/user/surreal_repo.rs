use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::types::{RecordId, SurrealValue};

use money_app_shared::api::ListQuery;
use money_app_shared::user::User;

use crate::database::{Database, surreal_take_errors};
use crate::error::AppError;

use super::model::{UserRecord, user_resource};
use super::repository::UserRepository;

#[derive(Clone)]
pub struct SurrealUserRepo {
    db: Arc<Database>,
}

impl SurrealUserRepo {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    fn inner(&self) -> &Database {
        &self.db
    }
}

#[async_trait]
impl UserRepository for SurrealUserRepo {
    async fn get_users(&self, pagination: ListQuery) -> Result<Vec<User>, AppError> {
        let (offset, limit) = pagination.effective_offset_limit();
        let needle = pagination.q.as_ref().and_then(|s| {
            let t = s.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_lowercase())
            }
        });
        let mut response = if let Some(needle) = needle {
            self.inner()
                .db
                .query(
                    "SELECT * FROM user WHERE string::contains(string::lowercase(email), $needle) \
                     LIMIT $limit START $start",
                )
                .bind(("needle", needle))
                .bind(("limit", limit))
                .bind(("start", offset))
                .await?
        } else {
            self.inner()
                .db
                .query("SELECT * FROM user LIMIT $limit START $start")
                .bind(("limit", limit))
                .bind(("start", offset))
                .await?
        };
        Ok(response
            .take::<Vec<UserRecord>>(0)?
            .into_iter()
            .map(UserRecord::into_user)
            .collect())
    }

    async fn count_users(&self, query: ListQuery) -> Result<u64, AppError> {
        #[derive(Deserialize, SurrealValue)]
        struct CountResult {
            count: u64,
        }
        let needle = query.q.as_ref().and_then(|s| {
            let t = s.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_lowercase())
            }
        });
        let mut response = if let Some(needle) = needle {
            self.inner()
                .db
                .query(
                    "SELECT count() FROM user WHERE string::contains(string::lowercase(email), $needle) GROUP ALL",
                )
                .bind(("needle", needle))
                .await?
        } else {
            self.inner()
                .db
                .query("SELECT count() FROM user GROUP ALL")
                .await?
        };
        Ok(response
            .take::<Vec<CountResult>>(0)?
            .into_iter()
            .next()
            .map(|r| r.count)
            .unwrap_or(0))
    }

    async fn get_user(&self, id: &str) -> Result<User, AppError> {
        self.inner()
            .db
            .select(user_resource(id)?)
            .await?
            .map(UserRecord::into_user)
            .ok_or(AppError::NotFound("user not found".into()))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        Ok(self
            .inner()
            .db
            .query("SELECT * FROM user WHERE email = $email LIMIT 1")
            .bind(("email", email.to_lowercase()))
            .await?
            .take::<Option<UserRecord>>(0)?
            .map(UserRecord::into_user))
    }

    async fn create_user_record(&self, user: User) -> Result<User, AppError> {
        self.inner()
            .db
            .create("user")
            .content(UserRecord::from_user(user))
            .await?
            .map(UserRecord::into_user)
            .ok_or_else(|| AppError::database("failed to create user"))
    }

    async fn delete_user(&self, id: &str) -> Result<User, AppError> {
        self.inner()
            .db
            .delete(user_resource(id)?)
            .await?
            .map(UserRecord::into_user)
            .ok_or(AppError::NotFound("user not found".into()))
    }

    async fn update_oauth_picture_url(&self, user_id: &str, url: Option<&str>) -> Result<(), AppError> {
        let mut response = if let Some(u) = url {
            self.inner()
                .db
                .query("UPDATE $user SET oauth_picture_url = $url")
                .bind(("user", RecordId::new("user", user_id)))
                .bind(("url", u.to_owned()))
                .await?
        } else {
            self.inner()
                .db
                .query("UPDATE $user SET oauth_picture_url = NONE")
                .bind(("user", RecordId::new("user", user_id)))
                .await?
        };
        surreal_take_errors("user.update_oauth_picture_url", &mut response)?;
        let _ = response.check().map_err(|e| {
            crate::log_and_convert!(AppError::database, "user.update_oauth_picture_url.check", e)
        })?;
        Ok(())
    }
}
