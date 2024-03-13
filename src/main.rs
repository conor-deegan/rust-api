// todo:
// lint to order things etc?
// request validation
// SQLX
// docker

use log::{info, warn};
use std::net::SocketAddr;
use axum::{
    http::{ StatusCode, Uri }, response::Json, routing::any, Router
};
use serde_json::json;
use tower_http::trace::TraceLayer;

mod routes;
mod services;
mod controllers;
mod utils;
mod types;
mod middleware;

async fn app() -> Router {
    // Load the .env file
    dotenv::dotenv().ok();

    // initialize the logger
    utils::logging::initialize_logger();

    // Define routes
    let routers = vec![
        routes::user::routes(),
        // Add more routers from other modules
    ];

    // Merge the routers
    let app = routers.into_iter()
        .reduce(|acc, router| acc.merge(router))
        .unwrap()
        // Fallback route 404
        .fallback(any(|uri: Uri| async move {
            let response_body = json!({ "error": format!("No route found for path {}", uri) });
            (StatusCode::NOT_FOUND, Json(response_body))
        }))
        // Add middleware
        .layer(TraceLayer::new_for_http());
    app
}

#[tokio::main]
async fn main() {
    // Load the app
    let app = app().await;

    // Run the server
    let addr = "127.0.0.1:3000".parse::<SocketAddr>();

    // Use pattern matching to handle the Result
    let addr = match addr {
        Ok(a) => a,
        Err(e) => {
            warn!("Failed to parse address: {}", e);
            return;
        },
    };

    info!("Server started");
    info!("Listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}