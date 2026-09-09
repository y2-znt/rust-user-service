use axum::{Router, routing::get};

use crate::app::AppState;

use super::user_handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/{id}", get(user_handler::get_by_id))
}
