use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::app::AppState;

use super::user_handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user_handler::get_all))
        .route("/", post(user_handler::create))
        .route("/{id}", get(user_handler::get_by_id))
        .route("/{id}", patch(user_handler::update))
        .route("/{id}", delete(user_handler::delete))
}
