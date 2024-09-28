use log::info;
use static_events::simple_event;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::*;
use teloxide::types::Update;

use crate::telegram::{cli, event_join_member};
use crate::telegram::cli::TelegramCli;
use crate::telegram::event_left_member;

pub async fn initial_dispatcher(bot: Bot) -> anyhow::Result<()> {
    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<TelegramCli>()
                .endpoint(cli::telegram_cli_handler),
        )
        .branch(
            Update::filter_message().enter_dialogue()
                .branch(
                    dptree::filter(|m: Message| m.new_chat_members().is_some())
                        .endpoint(event_join_member::execute),
                )
                .branch(
                    dptree::filter(|m: Message| m.left_chat_member().is_some())
                        .endpoint(event_left_member::execute),
                ).endpoint(|(bot, msg)| {

            }),
        );

    let _ = Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
