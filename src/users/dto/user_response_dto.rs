use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::super::user_model::User;

#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        let User {
            id,
            email,
            username,
            password_hash: _,
            role,
            created_at,
            updated_at,
        } = user;

        Self {
            id,
            email,
            username,
            role,
            created_at,
            updated_at,
        }
    }
}
