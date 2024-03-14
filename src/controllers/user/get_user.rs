use std::sync::Arc;

use crate::{services::user::get_user::get_user as get_user_service, types::user::GetUserRequest};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

pub async fn get_user(
    Path(params): Path<GetUserRequest>,
    Extension(pool): Extension<Arc<PgPool>>,
) -> impl IntoResponse {
    match get_user_service(params, Extension(pool)).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(error) => error.into_response(),
    }
}
