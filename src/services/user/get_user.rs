use crate::{
    error::Error,
    types::user::{GetUserRequest, User},
};
use log::{error, info};
use sqlx::PgPool;

pub async fn get_user(pool: &PgPool, payload: GetUserRequest) -> Result<User, Error> {
    info!("looking up user id: {}", payload.id);

    let result = sqlx::query_as!(
        User,
        "SELECT id, name, age, gender FROM users WHERE id = $1",
        payload.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            let err = Error::NotFound(format!("user with id {} not found", payload.id));
            error!("{}", err);
            err
        }
        _ => {
            let err = Error::Sqlx(e);
            error!("database error: {}", err);
            err
        }
    })?;

    Ok(result)
}
