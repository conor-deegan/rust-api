pub fn initialize_logger(log_level: String) {
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
