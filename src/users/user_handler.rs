use axum::{
    Json,
    extract::{Query, State},
};

use crate::{
    api::{ApiResponse, AppError, PaginatedResponse, PaginationQuery, PathUuid, ok, paginated},
    app::AppState,
};

use super::{user_dto::UserResponse, user_service};

pub async fn get_all(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Vec<UserResponse>>>, AppError> {
    let (page, per_page) = query.parse()?;

    let (users, pagination) = user_service::get_all(&state.db, page, per_page).await?;

    Ok(paginated(users, pagination))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    PathUuid(id): PathUuid,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = user_service::get_by_id(&state.db, id).await?;

    Ok(ok(user))
}
