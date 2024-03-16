use crate::{
    error::Error,
    types::user::{GetUserRequest, User},
};
use log::info;
use sqlx::PgPool;

pub async fn get_user(pool: &PgPool, payload: GetUserRequest) -> Result<User, Error> {
    info!("Looking up user id: {}", payload.id);

    let result = sqlx::query_as!(
        User,
        "SELECT id, name, age, gender FROM users WHERE id = $1",
        payload.id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}
