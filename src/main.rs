use api::middleware::auth::auth;
use api::AppState;
use axum::{
    http::{StatusCode, Uri},
    middleware::from_fn,
    response::Json,
    routing::{any, get, post},
    Router,
};
use clap::Parser;
use env_logger::Env;
use log::info;
use serde_json::json;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // load the .env file and configure the environment
    dotenv::dotenv().expect("unable to load .env file");
    let config = api::config::Config::parse();

    // initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // connect to the database
    let pool = api::config::connect_db(&config.db_uri).await?;

    // Define routes
    let app = Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/", post(api::controllers::user::create_user::create_user))
                .route("/:id", get(api::controllers::user::get_user::get_user))
                .layer(from_fn(auth)),
        )
        .with_state(AppState { pool })
        .fallback(any(|uri: Uri| async move {
            let response_body = json!({ "error": format!("No route found for path {}", uri) });
            (StatusCode::NOT_FOUND, Json(response_body))
        }))
        .layer(TraceLayer::new_for_http());

    info!("server started");
    info!("listening on {}:{}", config.host, config.port);
    info!("environment: {}", config.app_env);

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .expect("could not bind to host / port");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
