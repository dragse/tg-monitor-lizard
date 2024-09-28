use std::env;

use dotenvy::dotenv;
use env_logger::TimestampPrecision;
use log::info;
use serde_json::json;
use teloxide::prelude::*;

use crate::model::{GroupConfiguration, JoinValidation};

mod etcd;
mod model;
mod telegram;
pub mod util;
pub mod error;

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


    Ok(())
}
