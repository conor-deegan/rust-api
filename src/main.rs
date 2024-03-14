// todo:
// request validation
// SQLX: https://chat.openai.com/c/f845dda9-0b65-4061-8c20-b7c3426a5f70
// Figure out how to add tables and do SQLX migrations
// docker and kube locally. Get used to Kube again.

use axum::{
    http::{StatusCode, Uri},
    response::Json,
    routing::any,
    Extension, Router,
};
use log::{info, warn};
use serde_json::json;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod config;
mod controllers;
mod middleware;
mod routes;
mod services;
mod types;
mod utils;

async fn app(pool: Arc<PgPool>) -> Router {
    // Define routes
    let routers = vec![
        routes::user::routes(),
        // Add more routers from other modules
    ];

    // Merge the routers
    routers
        .into_iter()
        .reduce(|acc, router| acc.merge(router))
        .unwrap()
        // Fallback route 404
        .fallback(any(|uri: Uri| async move {
            let response_body = json!({ "error": format!("No route found for path {}", uri) });
            (StatusCode::NOT_FOUND, Json(response_body))
        }))
        // Add middleware
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
    // load the config
    let config = config::ConfigVars::new();

    // initialize the logger
    utils::logging::initialize_logger(config.log_level.clone());

    // connect to the database
    let pool = config::connect_db(&config).await.unwrap();

    // Load the app
    let app = app(pool).await;

    // Run the server
    let addr = config.host_and_port().parse::<SocketAddr>();

    // Use pattern matching to handle the Result
    let addr = match addr {
        Ok(a) => a,
        Err(e) => {
            warn!("Failed to parse address: {}", e);
            return;
        }
    };

    info!("server started");
    info!("listening on {}", addr);
    info!("environment: {}", config.app_env);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
