#![allow(unused, unused_variables)]

use std::env;
use std::sync::Arc;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use env_logger::TimestampPrecision;
use envconfig::Envconfig;
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
mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .is_test(true)
        .init();

    let db_url =
        env::var("DATABASE_URL").expect("'DATABASE_URL' is an required environment variable");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)?;

    let mut plugin_manager = plugin::PluginManager::new(pool);
    plugin_manager.register_plugin(Box::new(modules::join_validation_math::DemoPlugin{}));

    plugin_manager.load_plugins();
    plugin_manager.enable_plugins();

    let telegram_token =
        env::var("TELEGRAM_TOKEN").expect("'TELEGRAM_TOKEN' is an required environment variable");
    let api = Api::new(telegram_token.as_str());

    let bot_info = api.get_me()?;
    info!("Connect to Telegram with Bot: @{:?}", bot_info.result.username);

    let mut update_params = GetUpdatesParams::builder().build();
    let arc_api = Arc::new(api);
    loop {
        let result = arc_api.get_updates(&update_params);
        match result {
            Ok(response) => {
                for update in response.result {
                    let update_id = update.update_id;

                    update_params.offset = Some(i64::from(update_id) + 1);
                    plugin_manager.call_event(arc_api.clone(), update);
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }

    plugin_manager.disable_plugins();
}