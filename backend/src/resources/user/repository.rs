use async_trait::async_trait;

use money_app_shared::api::ListQuery;
use money_app_shared::user::User;

use crate::error::AppError;

/// Pure user data access — no authorization.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self, pagination: ListQuery) -> Result<Vec<User>, AppError>;
    async fn count_users(&self, query: ListQuery) -> Result<u64, AppError>;
    async fn get_user(&self, id: &str) -> Result<User, AppError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn create_user_record(&self, user: User) -> Result<User, AppError>;
    async fn delete_user(&self, id: &str) -> Result<User, AppError>;
    async fn update_oauth_picture_url(&self, user_id: &str, url: Option<&str>) -> Result<(), AppError>;
}
