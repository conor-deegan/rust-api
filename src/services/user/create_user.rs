use std::sync::Arc;

use crate::{types::user::CreateUserRequest, utils::error::CustomError};
use axum::{http::StatusCode, Extension};
use log::info;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn create_user(user: CreateUserRequest, Extension(pool): Extension<Arc<PgPool>>) -> Result<Value, CustomError> {
    info!("{:?}", user);
    let res = sqlx::query("INSERT INTO users (name, age) VALUES ($1, $2)")
        .bind(&user.name)
        .bind(user.age)
        .execute(&*pool)
        .await;

    match res {
        Ok(_) => {
            Ok(json!({ "message": "user created" }))
        },
        Err(e) => {
            Err(CustomError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to create user: {:?}", e.to_string())
            })
        },
    }
}
