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
) -> ResponseResult<()> {
    let _ = match cmd {
        TelegramCli::UpdateJoin(new_join_message) => {
            let admin_check_result =
                util::check_if_user_is_admin(&bot, &msg.chat.id, &msg.from().unwrap().id).await;
            match admin_check_result {
                Ok(is_admin) => {
                    if is_admin {
                        _ = bot
                            .send_message(
                                msg.chat.id,
                                format!("new join message: '{new_join_message}'"),
                            )
                            .await?;
                    } else {
                        _ = bot.send_message(msg.chat.id, "no permissions").await?;
                    }
                }
                Err(error) => {
                    _ = bot
                        .send_message(msg.chat.id, format!("problem with admin check {error:?}"))
                        .await?;
                }
            }
            return Ok(());
        }
        TelegramCli::UpdateLeave(new_leave_message) => {
            let admin_check_result =
                util::check_if_user_is_admin(&bot, &msg.chat.id, &msg.from().unwrap().id).await;
            match admin_check_result {
                Ok(is_admin) => {
                    if is_admin {
                        _ = bot
                            .send_message(
                                msg.chat.id,
                                format!("new leave message: '{new_leave_message}'"),
                            )
                            .await?;
                    } else {
                        _ = bot.send_message(msg.chat.id, "no permissions").await?;
                    }
                }
                Err(error) => {
                    _ = bot
                        .send_message(msg.chat.id, format!("problem with admin check {error:?}"))
                        .await?;
                }
            }
            return Ok(());
        }
    };
}
