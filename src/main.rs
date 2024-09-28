use std::env;

use dotenvy::dotenv;
use env_logger::TimestampPrecision;
use frankenstein::{Api, GetUpdatesParams, ReplyParameters, SendMessageParams, TelegramApi, UpdateContent};
use log::info;
use serde_json::json;

pub mod util;
pub mod error;
pub mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .is_test(true)
        .init();

    let telegram_token =
        env::var("TELEGRAM_TOKEN").expect("'TELEGRAM_TOKEN' is an required environment variable");
    let api = Api::new(telegram_token.as_str());

    let bot_info = api.get_me()?;
    info!("Connect to Telegram with Bot: @{:?}", bot_info.result.username);

    let mut update_params = GetUpdatesParams::builder().build();
    loop {
        let result = api.get_updates(&update_params);

        println!("result: {result:?}");

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let reply_parameters = ReplyParameters::builder()
                            .message_id(message.message_id)
                            .build();
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text("hello")
                            .reply_parameters(reply_parameters)
                            .build();
                        if let Err(error) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {error:?}");
                        }
                    }
                    update_params.offset = Some(i64::from(update.update_id) + 1);
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }

    Ok(())
}
