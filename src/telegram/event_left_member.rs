use teloxide::{prelude::*, utils::html};

use crate::etcd;

pub(crate) async fn execute(bot: Bot, message: Message) -> ResponseResult<()> {
    let group_configuration = etcd::get_group_validation(message.chat.id).await.unwrap();

    if group_configuration.leave_message.len() == 0 {
        return Ok(());
    }

    if let Some(user) = message.left_chat_member() {
        let _ = user
            .mention()
            .unwrap_or_else(|| html::user_mention(user.id.0 as i64, user.full_name().as_str()));

        bot.send_message(message.chat.id, group_configuration.leave_message)
            .await?;
    }

    Ok(())
}
