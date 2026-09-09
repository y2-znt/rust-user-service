use axum::Router;
use sqlx::PgPool;

use crate::{health, users};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn create_app(pool: PgPool) -> Router {
    let state = AppState { db: pool };

    Router::new()
        .nest("/health", health::routes())
        .nest("/users", users::routes())
        .with_state(state)
}
