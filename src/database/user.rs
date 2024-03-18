use sqlx::PgPool;

use crate::{
    error::Error,
    types::user::{CreateUserRequest, User},
};

pub async fn get_user_by_id(pool: &PgPool, user_id: &i32) -> Result<Option<User>, Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id=$1", user_id,)
        .fetch_optional(pool)
        .await
        .map_err(Error::Query)
}

pub async fn create_user(pool: &PgPool, new_user: &CreateUserRequest) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, age, gender) VALUES ($1, $2, $3) RETURNING id, name, age, gender",
        &new_user.name,
        new_user.age,
        &new_user.gender
    )
    .fetch_one(pool)
    .await
    .map_err(Error::Query)
}
