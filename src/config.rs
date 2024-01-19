// config.rs

use std::env;

pub struct Config {
    pub telegram_bot_token: String,
}

impl Config {
    pub fn load() -> Self {
        Config {
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"),
            // Add other environment variables as needed
        }
    }
}
