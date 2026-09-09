use sqlx::PgPool;
use uuid::Uuid;

use super::user_model::User;

pub async fn find_all(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, password_hash, role, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        LIMIT $1
        OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, password_hash, role, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    id: Uuid,
    email: &str,
    username: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, username, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, username, password_hash, role, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(email)
    .bind(username)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    email: Option<&str>,
    username: Option<&str>,
    password_hash: Option<&str>,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET
            email = COALESCE($2, email),
            username = COALESCE($3, username),
            password_hash = COALESCE($4, password_hash),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, email, username, password_hash, role, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(email)
    .bind(username)
    .bind(password_hash)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
