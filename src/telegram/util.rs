use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::{ChatId, UserId};

pub async fn check_if_user_is_admin(
    bot: &Bot,
    chat_id: &ChatId,
    user_id: &UserId,
) -> anyhow::Result<bool> {
    //TODO later we can save all telegram admins and listen for changes so we don't need to call this everytime
    // right now, we will cache this call for 30 minutes

    Ok(get_chat_administrator_ids(bot, chat_id)
        .await?
        .into_iter()
        .find(|member_id| member_id == user_id)
        .is_some())
}

pub async fn get_chat_administrator_ids(
    bot: &Bot,
    chat_id: &ChatId,
) -> anyhow::Result<Vec<UserId>> {
    let resp = bot
        .get_chat_administrators(*chat_id)
        .await?
        .into_iter()
        .map(|member| member.user.id)
        .collect();

    return Ok(resp);
}
