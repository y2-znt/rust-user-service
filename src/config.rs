use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "4000".to_string())
            .parse::<u16>()
            .expect("APP_PORT must be a valid port");

        let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            host,
            port,
            database_url,
        }
    }
}
