use axum::Router;
use sqlx::PgPool;

use crate::health;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn create_app(pool: PgPool) -> Router {
    let state = AppState { db: pool };

    Router::new()
        .nest("/health", health::routes())
        .with_state(state)
}
