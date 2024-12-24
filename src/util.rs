use diesel::serialize::IsNull::No;
use frankenstein::{Update, UpdateContent};
use rand::{Rng};

pub fn generate_calculation(correct: bool) -> String {
    let first_num = rand::thread_rng().gen_range(0..100);
    let second_num = rand::thread_rng().gen_range(0..100);

    let mut solution = first_num + second_num;

    while !correct && solution == first_num + second_num {
        solution = rand::thread_rng().gen_range(0..150)
    }

    format!("{} + {} = {}", first_num, second_num, solution)
}

pub fn get_chat_id_fom_update(update: Update) -> Option<i64> {
    match update.content {
        UpdateContent::Message(msg) |
        UpdateContent::EditedMessage(msg) |
        UpdateContent::ChannelPost(msg) |
        UpdateContent::EditedChannelPost(msg) |
        UpdateContent::BusinessMessage(msg) |
        UpdateContent::EditedBusinessMessage(msg) => {
            Some(msg.chat.id)
        }
        UpdateContent::InlineQuery(_) => None,
        UpdateContent::ChosenInlineResult(_) => None,
        UpdateContent::CallbackQuery(_) => None,
        // Polls maybe?!?
        UpdateContent::Poll(_) => None,
        UpdateContent::PollAnswer(_) => None,
        // Audit Implementation
        UpdateContent::ChatMember(content) => Some(content.chat.id),
        // Join validation check maybe?
        UpdateContent::ChatJoinRequest(content) => Some(content.chat.id),
        // =========
        UpdateContent::MessageReaction(content) => Some(content.chat.id),
        UpdateContent::MessageReactionCount(content) => Some(content.chat.id),
        UpdateContent::PreCheckoutQuery(_) => None,
        UpdateContent::ShippingQuery(_) => None,
        UpdateContent::MyChatMember(content) => Some(content.chat.id),
        UpdateContent::ChatBoost(content) => Some(content.chat.id),
        UpdateContent::RemovedChatBoost(content) => Some(content.chat.id),
        UpdateContent::PurchasedPaidMedia(_) => None,
        UpdateContent::BusinessConnection(_) => None,
        UpdateContent::DeletedBusinessMessages(content) => Some(content.chat.id),
    }
}