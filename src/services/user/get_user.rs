use crate::{types::user::GetUserRequest, utils::error::CustomError};
use log::info;
use serde_json::{json, Value};

pub async fn get_user(payload: GetUserRequest) -> Result<Value, CustomError> {
    info!("Looking up user id: {}", payload.id);
    Ok(json!({ "message": "hi i am the user" }))
}
