mod api;
mod app;
mod auth;
mod banner;
mod config;
mod db;
mod health;
mod users;

use config::Config;
use db::create_pool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env();
    let pool = create_pool(&config.database_url);

    let app = app::create_app(pool);

    let address = format!("{}:{}", config.host, config.port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind TCP listener");

    banner::print(&config.host, config.port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
