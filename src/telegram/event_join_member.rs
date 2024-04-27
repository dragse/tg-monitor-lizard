use log::info;
use teloxide::{prelude::*, types::ParseMode, utils::html};


pub(crate) async fn execute(bot: Bot, message: Message) -> ResponseResult<()> {
    info!("JOIN");
    let users = message.new_chat_members().unwrap_or_default();

    let telegram_group_name = message.chat.title().unwrap_or("");

    for user in users {
        // We get a "@username" mention via `mention()` method if the user has a
        // username, otherwise we create a textual mention with "Full Name" as the
        // text linking to the user
        let username =
            user.mention().unwrap_or_else(|| html::user_mention(user.id.0 as i64, user.full_name().as_str()));

        bot.send_message(message.chat.id, format!("Welcome to {telegram_group_name} {username}!"))
            .await?;
    }

    Ok(())
}