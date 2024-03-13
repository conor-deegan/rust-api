use std::env;

pub fn initialize_logger() {
    if env::var("RUST_LOG").is_ok() {
        let log_level = env::var("RUST_LOG").unwrap();
        match log_level.as_str() {
            "info" => {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::INFO)
                    .init();
            }
            "debug" => {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::DEBUG)
                    .init();
            }
            "warn" => {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::WARN)
                    .init();
            }
            "error" => {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::ERROR)
                    .init();
            }
            _ => {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::INFO)
                    .init();
            }
        }
    }
}
