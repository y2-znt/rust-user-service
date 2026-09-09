use axum::{Json, extract::State};

use crate::{
    api::{ApiResponse, AppError, PathUuid, ok},
    app::AppState,
};

use super::{user_dto::UserResponse, user_service};

pub async fn get_by_id(
    State(state): State<AppState>,
    PathUuid(id): PathUuid,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = user_service::get_by_id(&state.db, id).await?;

    Ok(ok(user))
}
