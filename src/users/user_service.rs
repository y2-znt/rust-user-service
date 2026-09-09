use crate::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::{user_dto::UserResponse, user_repository};

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<UserResponse, AppError> {
    match user_repository::find_by_id(pool, id).await {
        Ok(Some(user)) => Ok(UserResponse::from(user)),
        Ok(None) => Err(AppError::NotFound),
        Err(error) => Err(AppError::Database(error)),
    }
}
