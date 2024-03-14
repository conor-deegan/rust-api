use crate::config::ConfigVars;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Result;
use std::sync::Arc;
use tracing::info;

pub async fn connect_db(config: &ConfigVars) -> Result<Arc<PgPool>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_uri)
        .await;

    match pool {
        Ok(pool) => {
            info!("connected to database");
            Ok(Arc::new(pool))
        }
        Err(e) => panic!("failed to connect to database: {}", e),
    }
}
