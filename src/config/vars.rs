use dotenv::dotenv;
use std::env;

pub struct ConfigVars {
    pub app_env: String,
    pub db_uri: String,
    pub log_level: String,
    pub host: String,
    pub port: u16,
}

impl ConfigVars {
    pub fn new() -> Self {
        // Load .env file
        dotenv().ok();

        // Get environment variables or default values
        let app_env = env::var("ENV").unwrap_or_else(|_| "development".into());
        let db_uri = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/db".into());
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".into());
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .unwrap_or(8080);

        ConfigVars {
            app_env,
            db_uri,
            log_level,
            host,
            port,
        }
    }

    // Method to get the combined host and port
    pub fn host_and_port(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
