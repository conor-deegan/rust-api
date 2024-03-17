use crate::{
    error::Error, services::user::create_user::create_user as create_user_service,
    types::user::CreateUserRequest, AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match create_user_service(&state.pool, payload).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(Error::ValidationError(res)) => (StatusCode::BAD_REQUEST, Json(res)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())).into_response(),
    }
}
