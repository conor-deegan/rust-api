use axum::{Router, routing::{post, get}};
use crate::controllers;

pub fn routes() -> Router {
    let users_router = Router::new()
        .route("/", post(controllers::user::create_user::create_user))
        .route("/:id", get(controllers::user::get_user::get_user));

    Router::new()
        .nest("/users", users_router)
}
