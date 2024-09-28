use std::vec;
use teloxide::{prelude::*};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::{etcd, util};
use crate::model::JoinValidation;

pub(crate) async fn execute(bot: Bot, message: Message) -> ResponseResult<()> {

    let group_configuration = etcd::get_group_validation(message.chat.id).await.unwrap();

    if group_configuration.join_message.len() == 0 {
        return Ok(());
    }

    match group_configuration.join_validation {
        JoinValidation::Disabled => Ok(()),
        JoinValidation::InlineKeyboardButtonMath {
            question_size,
            allow_retry
        } => {
            let mut markup = InlineKeyboardMarkup::default();

            for i in 0..question_size {
                markup = markup.append_row(vec![InlineKeyboardButton::callback(util::generate_calculation(i == 0), format!("UserValidation: {}", i == 0))])
            }

            bot.send_message(message.chat.id, group_configuration.join_message).reply_markup(markup).await?;
            Ok(())
        }
    }
}
