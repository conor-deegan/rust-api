use std::sync::Arc;

use crate::{
    services::user::create_user::create_user as create_user_service, types::user::CreateUserRequest,
};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

pub async fn create_user(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match create_user_service(payload, Extension(pool)).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(error) => error.into_response(),
    }
}
