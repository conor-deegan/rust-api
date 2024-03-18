use crate::{
    database,
    error::Error,
    types::user::{GetUserRequest, User},
};
use log::{error, info};
use sqlx::PgPool;

pub async fn get_user(pool: &PgPool, payload: GetUserRequest) -> Result<User, Error> {
    info!("looking up user id: {}", payload.id);

    let user = database::user::get_user_by_id(pool, &payload.id)
        .await?
        .ok_or_else(|| {
            error!("user with id {} not found", payload.id);
            Error::NotFound(format!("user with id {} not found", payload.id))
        })?;

    Ok(user)
}
