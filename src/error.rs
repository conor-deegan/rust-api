#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error from SQLX: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("validation error: {0}")]
    ValidationError(String),
    #[error("query error: {0}")]
    Query(#[source] sqlx::Error),
}
