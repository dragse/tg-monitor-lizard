use std::env;

use dotenvy::dotenv;
use env_logger::TimestampPrecision;
use log::info;
use teloxide::prelude::*;

use crate::model::{GroupConfiguration, JoinValidation};

mod etcd;
mod model;
mod telegram;
pub mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .is_test(true)
        .init();

    let telegram_token =
        env::var("TELEGRAM_TOKEN").expect("'TELEGRAM_TOKEN' is an required environment variable");
    let telegram_bot = Bot::new(telegram_token);

    etcd::save_group_validation(
        -1001299964433,
        GroupConfiguration {
            join_message: "".to_string(),
            join_validation: JoinValidation::InlineKeyboardButtonMath,
            leave_message: "".to_string(),
        },
    )
    .await
    .expect("Problem while saving Group configuration");

    let bot_info = telegram_bot.get_me().await?;
    info!("Connect to Telegram with Bot: @{}", bot_info.username());

    telegram::initial_dispatcher(telegram_bot).await?;

    Ok(())
}
