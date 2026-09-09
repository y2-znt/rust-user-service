use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::{
    api::{ApiResponse, ok},
    app::AppState,
};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn health(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<HealthResponse>>) {
    let database_is_up = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    if database_is_up {
        (
            StatusCode::OK,
            ok(HealthResponse {
                status: "Health check ok",
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            ok(HealthResponse {
                status: "Health check degraded",
            }),
        )
    }
}
