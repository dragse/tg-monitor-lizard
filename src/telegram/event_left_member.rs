use log::info;
use teloxide::{prelude::*, types::ParseMode, utils::html};

pub(crate) async fn execute(bot: Bot, message: Message) -> ResponseResult<()> {
    info!("LEAVE");
    if let Some(user) = message.left_chat_member() {
        let username =
            user.mention().unwrap_or_else(|| html::user_mention(user.id.0 as i64, user.full_name().as_str()));

        bot.send_message(message.chat.id, format!("Goodbye {username}!")).await?;
    }

    Ok(())
}