use frankenstein::{Chat, Update, UpdateContent};
use log::info;
use crate::error::LizardError;

pub fn handle_telegram_update(update: Update) -> anyhow::Result<(), LizardError> {
    match update.content {
        UpdateContent::Message(msg) |
            UpdateContent::EditedMessage(msg) |
            UpdateContent::ChannelPost(msg) |
            UpdateContent::EditedChannelPost(msg) |
            UpdateContent::BusinessMessage(msg) |
            UpdateContent::EditedBusinessMessage(msg) => {
            info!("Handling telegram message: {:?}", msg.message_id);
        }

        UpdateContent::InlineQuery(_) => {}
        UpdateContent::ChosenInlineResult(_) => {}
        UpdateContent::CallbackQuery(_) => {}

        // Polls maybe?!?
        UpdateContent::Poll(_) => {}
        UpdateContent::PollAnswer(_) => {}

        // Audit Implementation
        UpdateContent::ChatMember(_) => {}

        // Join validation check maybe?
        UpdateContent::ChatJoinRequest(_) => {}
        // =========
        UpdateContent::MessageReaction(_) => {}
        UpdateContent::MessageReactionCount(_) => {}
        UpdateContent::PreCheckoutQuery(_) => {}
        UpdateContent::ShippingQuery(_) => {}
        UpdateContent::MyChatMember(_) => {}
        UpdateContent::ChatBoost(_) => {}
        UpdateContent::RemovedChatBoost(_) => {}
        UpdateContent::PurchasedPaidMedia(_) => {}
        UpdateContent::BusinessConnection(_) => {}
        UpdateContent::DeletedBusinessMessages(_) => {}
    }

    Ok(())
}
pub fn get_chat_fom_update(update: Update) -> Option<Box<Chat>> {
    match update.content {
        UpdateContent::Message(msg) |
        UpdateContent::EditedMessage(msg) |
        UpdateContent::ChannelPost(msg) |
        UpdateContent::EditedChannelPost(msg) |
        UpdateContent::BusinessMessage(msg) |
        UpdateContent::EditedBusinessMessage(msg) => {
            Some(msg.chat)
        }

        UpdateContent::InlineQuery(_) => None,
        UpdateContent::ChosenInlineResult(_) => None,
        UpdateContent::CallbackQuery(_) => None,

        // Polls maybe?!?
        UpdateContent::Poll(_) => None,
        UpdateContent::PollAnswer(_) => None,

        // Audit Implementation
        UpdateContent::ChatMember(content) => Some(Box::new(content.chat)),

        // Join validation check maybe?
        UpdateContent::ChatJoinRequest(content) => Some(Box::new(content.chat)),
        // =========
        UpdateContent::MessageReaction(content) => Some(Box::new(content.chat)),
        UpdateContent::MessageReactionCount(content) => Some(Box::new(content.chat)),
        UpdateContent::PreCheckoutQuery(_) => None,
        UpdateContent::ShippingQuery(_) => None,
        UpdateContent::MyChatMember(content) => Some(Box::new(content.chat)),
        UpdateContent::ChatBoost(content) => Some(Box::new(content.chat)),
        UpdateContent::RemovedChatBoost(content) => Some(Box::new(content.chat)),
        UpdateContent::PurchasedPaidMedia(_) => None,
        UpdateContent::BusinessConnection(_) => None,
        UpdateContent::DeletedBusinessMessages(content) => Some(Box::new(content.chat)),
    }
}