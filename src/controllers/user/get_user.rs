use crate::{services::user::get_user::get_user as get_user_service, types::user::GetUserRequest};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

pub async fn get_user(Path(params): Path<GetUserRequest>) -> impl IntoResponse {
    match get_user_service(params).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(error) => error.into_response(),
    }
}
