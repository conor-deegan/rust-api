use std::sync::Arc;

use crate::{types::user::GetUserRequest, utils::error::CustomError};
use axum::Extension;
use log::info;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn get_user(
    payload: GetUserRequest,
    Extension(_pool): Extension<Arc<PgPool>>,
) -> Result<Value, CustomError> {
    info!("Looking up user id: {}", payload.id);
    Ok(json!({ "message": "hi i am the user" }))
}
