use crate::api::{AppError, Pagination};
use sqlx::PgPool;
use uuid::Uuid;

use super::{user_dto::UserResponse, user_repository};

pub async fn get_all(
    pool: &PgPool,
    page: u32,
    per_page: u32,
) -> Result<(Vec<UserResponse>, Pagination), AppError> {
    let limit = i64::from(per_page);
    let offset = i64::from(page.saturating_sub(1)) * limit;

    let users = user_repository::find_all(pool, limit, offset)
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();

    let total = user_repository::count(pool).await?;

    Ok((users, Pagination::from_total(total as u64, page, per_page)))
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<UserResponse, AppError> {
    match user_repository::find_by_id(pool, id).await {
        Ok(Some(user)) => Ok(UserResponse::from(user)),
        Ok(None) => Err(AppError::NotFound("User not found".to_string())),
        Err(error) => Err(AppError::Database(error)),
    }
}
