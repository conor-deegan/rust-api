#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error from SQLX: {0}")]
    Sqlx(#[from] sqlx::Error),
}
