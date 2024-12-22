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
mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .is_test(true)
        .init();


    let mut plugin_manager = plugin::PluginManager::new();
    plugin_manager.register_plugin(Box::new(modules::demo::DemoPlugin{}));

    plugin_manager.load_plugins();
    plugin_manager.enable_plugins();

    let telegram_token =
        env::var("TELEGRAM_TOKEN").expect("'TELEGRAM_TOKEN' is an required environment variable");
    let api = Api::new(telegram_token.as_str());

    let bot_info = api.get_me()?;
    info!("Connect to Telegram with Bot: @{:?}", bot_info.result.username);

    let mut update_params = GetUpdatesParams::builder().build();
    loop {
        let result = api.get_updates(&update_params);
        match result {
            Ok(response) => {
                for update in response.result {
                    let update_id = update.update_id;

                    update_params.offset = Some(i64::from(update_id) + 1);
                    plugin_manager.call_event(update);
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }

    plugin_manager.disable_plugins();
}