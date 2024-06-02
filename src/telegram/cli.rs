use log::info;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::telegram::util;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum TelegramCli {
    UpdateJoin(String),
    UpdateLeave(String),
}

pub async fn telegram_cli_handler(
    bot: Bot,
    _me: teloxide::types::Me,
    msg: Message,
    cmd: TelegramCli,
) -> Result<(), teloxide::RequestError> {
    let _ = match cmd {
        TelegramCli::UpdateJoin(new_join_message) => {
            if !util::check_if_user_is_admin(&bot, &msg.chat.id, &msg.from().unwrap().id).await? {
                // No permissions
                return Ok(());
            }

            info!("new join message: '{new_join_message}'")
        }
        TelegramCli::UpdateLeave(new_leave_message) => {}
    };

    Ok(())
}
