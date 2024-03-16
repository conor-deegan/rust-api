use std::sync::Arc;

use crate::{types::user::CreateUserRequest, utils::error::CustomError};
use axum::{http::StatusCode, Extension};
use log::info;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn create_user(
    user: CreateUserRequest,
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<Value, CustomError> {
    info!("{:?}", user);

    let res = sqlx::query!(
        "INSERT INTO users (name, age, gender) VALUES ($1, $2, $3) RETURNING id",
        &user.name,
        user.age,
        &user.gender
    )
    .fetch_one(&*pool)
    .await;

    match res {
        Ok(record) => Ok(json!({ "message": "user created", "id": record.id })),
        Err(e) => Err(CustomError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Failed to create user: {:?}", e.to_string()),
        }),
    }
}
