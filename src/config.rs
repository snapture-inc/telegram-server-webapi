// config.rs
use std::env;

pub struct Config {
    pub aws_region: String,
    pub s3_bucket_name: String,
    pub telegram_bot_token: String,
    // pub server_url: String,
}

// TODO: get telegram bot token from database
impl Config {
    pub fn load() -> Self {
        Config {
            aws_region: env::var("AWS_REGION").expect("AWS_REGION not set"),
            s3_bucket_name: env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME not set"),
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"),
            // server_url: env::var("SERVER_URL").expect("SERVER_URL not set"),
            // Add other environment variables as needed
        }
    }
}
