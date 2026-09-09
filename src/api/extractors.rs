use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use uuid::Uuid;

use super::error::AppError;
use super::validation::FieldError;

pub struct PathUuid(pub Uuid);

impl<S> FromRequestParts<S> for PathUuid
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::<Uuid>::from_request_parts(parts, state)
            .await
            .map(|Path(id)| Self(id))
            .map_err(|_| invalid_id())
    }
}

fn invalid_id() -> AppError {
    AppError::Validation(vec![FieldError {
        field: "id".to_string(),
        message: "must be a valid UUID".to_string(),
    }])
}
