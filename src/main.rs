#![allow(unused, unused_variables)]

use std::env;

use dotenvy::dotenv;
use env_logger::TimestampPrecision;
use frankenstein::{Api, Chat, GetUpdatesParams, ReplyParameters, SendMessageParams, TelegramApi, Update, UpdateContent, Voice};
use log::info;
use serde_json::json;
use crate::error::LizardError;
use crate::handler::get_chat_fom_update;

pub mod util;
pub mod error;
pub mod db;
mod handler;
mod plugin;
mod event;

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
                    let update_id = update.update_id;

                    update_params.offset = Some(i64::from(update_id) + 1);

                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }

}

fn test(update: Update, chat: Option<Box<Chat>>) -> Result<(), LizardError> {
    Ok(())
}