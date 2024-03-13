use axum::http::StatusCode;
use crate::{types::user::CreateUserRequest, utils::error::CustomError};
use serde_json::{json, Value};
use log::info;

pub async fn create_user(user: CreateUserRequest) -> Result<Value, CustomError> {
    info!("Creating user with name: {} and age {}", user.name, user.age);
    if some_condition_that_fails() {
        Err(CustomError {
            status_code: StatusCode::BAD_REQUEST,
            message: "an error occurred".into(),
        })
    } else {
        Ok(json!({ "message": "user created" }))
    }
}

fn some_condition_that_fails() -> bool {
    true
}
