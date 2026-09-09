use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy(database_url)
        .expect("Invalid DATABASE_URL")
}
