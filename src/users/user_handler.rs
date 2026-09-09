use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{app::AppState, error::AppError};

use super::{user_dto::UserResponse, user_service};

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_service::get_by_id(&state.db, id).await?;

    Ok(Json(user))
}
