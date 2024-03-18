use crate::{database, error::Error, types::user::CreateUserRequest};
use log::{error, info};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, user: CreateUserRequest) -> Result<Value, Error> {
    info!("{:?}", user);

    if user.age < 18 {
        error!("user must be at least 18 years old");
        return Err(Error::ValidationError(
            "User must be at least 18 years old.".to_string(),
        ));
    }
    let new_user = database::user::create_user(pool, &user).await?;

    info!("user created successfully id: {:?}", new_user.id);

    Ok(json!({ "message": "user created", "id": new_user.id }))
}
