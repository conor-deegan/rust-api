use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[arg(short, long, default_value = "development")]
    pub app_env: String,

    #[arg(
        short,
        long,
        default_value = "postgres://postgres:password@localhost:5432"
    )]
    pub db_uri: String,

    #[arg(short = 'H', long, default_value = "localhost")]
    pub host: String,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    #[arg(long, env = "SENDGRID_API_KEY")]
    pub sendgrid_api_key: String,
}
