use std::sync::Arc;

use crate::{
    types::user::{GetUserRequest, User},
    utils::error::CustomError,
};
use axum::Extension;
use log::info;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn get_user(
    payload: GetUserRequest,
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<Value, CustomError> {
    info!("Looking up user id: {}", payload.id);

    let result = sqlx::query_as!(
        User,
        "SELECT id, name, age, gender FROM users WHERE id = $1",
        payload.id
    )
    .fetch_one(&*pool)
    .await;

    match result {
        Ok(user) => Ok(json!(user)),
        Err(e) => {
            info!("Failed to fetch user: {:?}", e);
            Err(CustomError {
                status_code: axum::http::StatusCode::NOT_FOUND,
                message: format!("No user found with ID {}", payload.id),
            })
        }
    }
}
