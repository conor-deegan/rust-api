use crate::{
    services::user::get_user::get_user as get_user_service, types::user::GetUserRequest, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_user(
    State(state): State<AppState>,
    Path(params): Path<GetUserRequest>,
) -> impl IntoResponse {
    match get_user_service(&state.pool, params).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("shits fucked: {error:?}")),
        )
            .into_response(),
    }
}
