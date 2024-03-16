use log::info;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn connect_db(db_uri: &str) -> Result<PgPool, sqlx::Error> {
    info!("connecting to database...");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri)
        .await
}
