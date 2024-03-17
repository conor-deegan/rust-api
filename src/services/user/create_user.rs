use crate::{
    error::Error,
    types::user::{CreateUserRequest, User},
};
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

    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (name, age, gender) VALUES ($1, $2, $3) RETURNING id, name, age, gender",
        &user.name,
        user.age,
        &user.gender
    )
    .fetch_one(pool)
    .await?;

    info!("user created successfully id: {:?}", res.id);

    Ok(json!({ "message": "user created", "id": res.id }))
}
