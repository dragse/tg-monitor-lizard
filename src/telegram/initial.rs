use anyhow::anyhow;
use log::info;
use teloxide::{prelude::*, types::ParseMode, utils::html};
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::{ChatMemberUpdated, Dispatcher};
use teloxide::types::Update;
use crate::telegram::event_join_member;
use crate::telegram::event_left_member;

pub async fn initial_dispatcher(bot: Bot) -> anyhow::Result<()> {
    let handler = Update::filter_message()
        .branch(
            Update::filter_message()
                .branch(dptree::filter(|m: Message| m.new_chat_members().is_some()).endpoint(event_join_member::execute))
                .branch(dptree::filter(|m: Message| m.left_chat_member().is_some()).endpoint(event_left_member::execute))
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}


/// Welcome Endpoint


