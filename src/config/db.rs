use log::info;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn connect_db(db_uri: &str) -> Result<PgPool, sqlx::Error> {
    info!("connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri)
        .await;
    match pool {
        Ok(_) => {
            info!("Connected to the database.");
            pool
        }
        Err(e) => {
            info!("Failed to connect to the database: {}", e);
            Err(e)
        }
    }
}
