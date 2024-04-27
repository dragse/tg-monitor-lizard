mod telegram;

use std::env;
use dotenvy::dotenv;
use env_logger::TimestampPrecision;
use log::{info};
use teloxide::{prelude::*, utils::command::BotCommands};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .is_test(true)
        .init();

    let telegram_token = env::var("TELEGRAM_TOKEN").expect("'TELEGRAM_TOKEN' is an required environment variable");
    let telegram_bot = Bot::new(telegram_token);

    let bot_info = telegram_bot.get_me().await?;
    info!("Connect to Telegram with Bot: @{}", bot_info.username());

    telegram::initial_dispatcher(telegram_bot).await?;

    Ok(())
}