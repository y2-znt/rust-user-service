mod banner;
mod config;
mod db;

use axum::{Json, Router, extract::State, routing::get};
use config::Config;
use db::create_pool;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env();

    let pool = create_pool(&config.database_url).await;

    let state: AppState = AppState { db: pool };

    let app = Router::new()
        .route("/health", get(health))
        .with_state(state);

    let address = format!("{}:{}", config.host, config.port);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    banner::print(&config.host, config.port);

    axum::serve(listener, app).await.unwrap();
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    sqlx::query("SELECT 1").execute(&state.db).await.unwrap();

    Json(HealthResponse {
        status: "health check ok",
    })
}
