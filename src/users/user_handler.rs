use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    api::{ApiResponse, AppError, PaginatedResponse, PaginationQuery, PathUuid, ok, paginated},
    app::AppState,
};

use super::{
    dto::{CreateUserDto, UpdateUserDto, UserResponseDto},
    user_service,
};

pub async fn get_all(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Vec<UserResponseDto>>>, AppError> {
    let (page, per_page) = query.parse()?;

    let (users, pagination) = user_service::get_all(&state.db, page, per_page).await?;

    Ok(paginated(users, pagination))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    PathUuid(id): PathUuid,
) -> Result<Json<ApiResponse<UserResponseDto>>, AppError> {
    let user = user_service::get_by_id(&state.db, id).await?;

    Ok(ok(user))
}

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponseDto>>), AppError> {
    let user = user_service::create(&state.db, dto).await?;

    Ok((StatusCode::CREATED, ok(user)))
}

pub async fn update(
    State(state): State<AppState>,
    PathUuid(id): PathUuid,
    Json(dto): Json<UpdateUserDto>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponseDto>>), AppError> {
    let user = user_service::update(&state.db, id, dto).await?;

    Ok((StatusCode::OK, ok(user)))
}

pub async fn delete(
    State(state): State<AppState>,
    PathUuid(id): PathUuid,
) -> Result<StatusCode, AppError> {
    user_service::delete(&state.db, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
