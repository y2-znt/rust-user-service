use crate::api::{AppError, Pagination};
use crate::auth::password::hash_password;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    dto::{CreateUserDto, UpdateUserDto, UserResponseDto},
    user_repository,
};

pub async fn get_all(
    pool: &PgPool,
    page: u32,
    per_page: u32,
) -> Result<(Vec<UserResponseDto>, Pagination), AppError> {
    let limit = i64::from(per_page);
    let offset = i64::from(page.saturating_sub(1)) * limit;

    let users = user_repository::find_all(pool, limit, offset)
        .await?
        .into_iter()
        .map(UserResponseDto::from)
        .collect();

    let total = user_repository::count(pool).await?;

    Ok((users, Pagination::from_total(total as u64, page, per_page)))
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<UserResponseDto, AppError> {
    match user_repository::find_by_id(pool, id).await {
        Ok(Some(user)) => Ok(UserResponseDto::from(user)),
        Ok(None) => Err(AppError::NotFound("User not found".to_string())),
        Err(error) => Err(AppError::Database(error)),
    }
}

pub async fn create(pool: &PgPool, dto: CreateUserDto) -> Result<UserResponseDto, AppError> {
    dto.validate()?;

    let id = Uuid::new_v4();

    let password_hash = hash_password_blocking(dto.password).await?;

    let user =
        match user_repository::create(pool, id, &dto.email, &dto.username, &password_hash).await {
            Ok(user) => user,
            Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
                return Err(AppError::Conflict("User already exists".to_string()));
            }
            Err(error) => return Err(AppError::Database(error)),
        };

    Ok(UserResponseDto::from(user))
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    dto: UpdateUserDto,
) -> Result<UserResponseDto, AppError> {
    dto.validate()?;

    let password_hash = match dto.password {
        Some(password) => Some(hash_password_blocking(password).await?),
        None => None,
    };

    let user = match user_repository::update(
        pool,
        id,
        dto.email.as_deref(),
        dto.username.as_deref(),
        password_hash.as_deref(),
    )
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::NotFound("User not found".to_string()));
        }
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            return Err(AppError::Conflict("User already exists".to_string()));
        }
        Err(error) => {
            return Err(AppError::Database(error));
        }
    };

    Ok(UserResponseDto::from(user))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let deleted = user_repository::delete(pool, id).await?;

    if !deleted {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(())
}

async fn hash_password_blocking(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(|_| AppError::Internal)?
        .map_err(|_| AppError::Internal)
}
