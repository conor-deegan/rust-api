use axum::{response::IntoResponse, Json, http::StatusCode};
use crate::{services::user::create_user::create_user as create_user_service, types::user::CreateUserRequest};

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> impl IntoResponse {
    match create_user_service(payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(error) => error.into_response(),
    }
}
