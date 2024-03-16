use crate::{error::Error, types::user::CreateUserRequest};
use log::info;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, user: CreateUserRequest) -> Result<Value, Error> {
    info!("{:?}", user);

    let res = sqlx::query!(
        "INSERT INTO users (name, age, gender) VALUES ($1, $2, $3) RETURNING id",
        &user.name,
        user.age,
        &user.gender
    )
    .fetch_one(pool)
    .await;

    match res {
        Ok(record) => Ok(json!({ "message": "user created", "id": record.id })),
        Err(e) => Err(e.into()),
    }
}
