use sqlx::PgPool;

pub mod config;
pub mod controllers;
pub mod database;
pub mod error;
pub mod middleware;
pub mod services;
pub mod types;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
