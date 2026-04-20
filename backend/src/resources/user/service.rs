use std::sync::Arc;

use money_app_shared::api::ListQuery;
use money_app_shared::user::User;
use tracing::instrument;

use crate::database::Database;
use crate::error::AppError;

use super::CreateUser;
use super::repository::UserRepository;
use super::surreal_repo::SurrealUserRepo;

#[derive(Clone)]
pub struct UserService<R> {
    pub repo: R,
}

impl<R> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: UserRepository> UserService<R> {
    pub async fn get_users(&self, pagination: ListQuery) -> Result<Vec<User>, AppError> {
        self.repo.get_users(pagination).await
    }

    pub async fn count_users(&self, query: ListQuery) -> Result<u64, AppError> {
        self.repo.count_users(query).await
    }

    pub async fn get_user(&self, id: &str) -> Result<User, AppError> {
        self.repo.get_user(id).await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.repo.get_user_by_email(email).await
    }

    #[instrument(level = "debug", err, skip(self, user))]
    pub async fn create_user(&self, user: User) -> Result<User, AppError> {
        let created = self.repo.create_user_record(user).await?;
        crate::audit!(
            "audit.user.created",
            user_id = tracing::field::display(&created.id),
            email = tracing::field::display(&created.email),
            role = tracing::field::debug(&created.role)
            ; "user created"
        );
        Ok(created)
    }

    pub async fn create_user_from_request(&self, request: CreateUser) -> Result<User, AppError> {
        self.create_user(request.into_user()).await
    }

    pub async fn get_user_by_email_or_create(&self, email: &str) -> Result<User, AppError> {
        if let Some(user) = self.repo.get_user_by_email(email).await? {
            return Ok(user);
        }
        self.create_user(User::new(email.to_lowercase())).await
    }

    pub async fn delete_user(&self, id: &str) -> Result<User, AppError> {
        self.repo.delete_user(id).await
    }

    pub async fn touch_oauth_picture_url(
        &self,
        user_id: &str,
        picture_url: Option<&str>,
    ) -> Result<(), AppError> {
        self.repo.update_oauth_picture_url(user_id, picture_url).await
    }
}

pub type UserServiceHandle = UserService<SurrealUserRepo>;

impl UserServiceHandle {
    pub fn build(db: Arc<Database>) -> Self {
        UserService::new(SurrealUserRepo::new(db))
    }
}
