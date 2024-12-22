use std::collections::HashMap;
use frankenstein::{BusinessConnection, BusinessMessagesDeleted, CallbackQuery, ChatBoostRemoved, ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, MessageReactionCountUpdated, MessageReactionUpdated, PaidMediaPurchased, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Update, UpdateContent};

pub struct EventManager {
    functions: HashMap<String, Box<dyn EventListener>>
}

pub trait EventListener {
    fn handle_message(&self, data: Message) -> Option<()> {
        None
    }
    fn handle_edited_message(&self, data: Message) -> Option<()> {
        None
    }
    fn handle_channel_post(&self, data: Message) -> Option<()> {
        None
    }
    fn handle_edited_channel_post(&self, data: Message) -> Option<()> {
        None
    }
    fn handle_business_connection(&self, data: BusinessConnection) -> Option<()> {
        None
    }
    fn handle_business_message(&self, data: Message) -> Option<()> {
        None
    }
    fn handle_edited_business_message(&self, data: Message) -> Option<()> {
        None
    }
    fn handle_deleted_business_message(&self, data: BusinessMessagesDeleted) -> Option<()> {
        None
    }
    fn handle_message_reaction(&self, data: MessageReactionUpdated) -> Option<()> {
        None
    }
    fn handle_message_reaction_count(&self, data: MessageReactionCountUpdated) -> Option<()> {
        None
    }
    fn handle_inline_query(&self, data: InlineQuery) -> Option<()> {
        None
    }
    fn handle_chosen_inline_result(&self, data: ChosenInlineResult) -> Option<()> {
        None
    }
    fn handle_callback_query(&self, data: CallbackQuery) -> Option<()> {
        None
    }
    fn handle_shipping_query(&self, data: ShippingQuery) -> Option<()> {
        None
    }
    fn handle_pre_checkout_query(&self, data: PreCheckoutQuery) -> Option<()> {
        None
    }
    fn handle_poll(&self, data: Poll) -> Option<()> {
        None
    }
    fn handle_poll_answer(&self, data: PollAnswer) -> Option<()> {
        None
    }
    fn handle_my_chat_member(&self, data: ChatMemberUpdated) -> Option<()> {
        None
    }
    fn handle_chat_member(&self, data: ChatMemberUpdated) -> Option<()> {
        None
    }
    fn handle_chat_join_request(&self, data: ChatJoinRequest) -> Option<()> {
        None
    }
    fn handle_chat_boost(&self, data: ChatBoostUpdated) -> Option<()> {
        None
    }
    fn handle_removed_chat_boost(&self, data: ChatBoostRemoved) -> Option<()> {
        None
    }
    fn handle_purchased_paid_media(&self, data: PaidMediaPurchased) -> Option<()> {
        None
    }
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager{
            functions: HashMap::new()
        }
    }

    pub fn add_listener(&mut self, key: &str, listener: Box<dyn EventListener>) -> anyhow::Result<()> {
        self.functions.insert(key.to_owned(), listener);

        Ok(())
    }

    pub fn rem_listener(&mut self, key: &str) -> anyhow::Result<()> {
        self.functions.remove(key);

        Ok(())
    }

    pub fn call(&self, update: Update) {
        self.functions.values().for_each(|f| match update.content.clone() {
            UpdateContent::Message(data) => {
                f.handle_message(data);
            }
            UpdateContent::EditedMessage(data) => {
                f.handle_edited_message(data);
            }
            UpdateContent::ChannelPost(data) => {
                f.handle_channel_post(data);
            }
            UpdateContent::EditedChannelPost(data) => {
                f.handle_edited_channel_post(data);
            }
            UpdateContent::BusinessConnection(data) => {
                f.handle_business_connection(data);
            }
            UpdateContent::BusinessMessage(data) => {
                f.handle_business_message(data);
            }
            UpdateContent::EditedBusinessMessage(data) => {
                f.handle_edited_business_message(data);
            }
            UpdateContent::DeletedBusinessMessages(data) => {
                f.handle_deleted_business_message(data);
            }
            UpdateContent::MessageReaction(data) => {
                f.handle_message_reaction(data);
            }
            UpdateContent::MessageReactionCount(data) => {
                f.handle_message_reaction_count(data);
            }
            UpdateContent::InlineQuery(data) => {
                f.handle_inline_query(data);
            }
            UpdateContent::ChosenInlineResult(data) => {
                f.handle_chosen_inline_result(data);
            }
            UpdateContent::CallbackQuery(data) => {
                f.handle_callback_query(data);
            }
            UpdateContent::ShippingQuery(data) => {
                f.handle_shipping_query(data);
            }
            UpdateContent::PreCheckoutQuery(data) => {
                f.handle_pre_checkout_query(data);
            }
            UpdateContent::Poll(data) => {
                f.handle_poll(data);
            }
            UpdateContent::PollAnswer(data) => {
                f.handle_poll_answer(data);
            }
            UpdateContent::MyChatMember(data) => {
                f.handle_my_chat_member(data);
            }
            UpdateContent::ChatMember(data) => {
                f.handle_chat_member(data);
            }
            UpdateContent::ChatJoinRequest(data) => {
                f.handle_chat_join_request(data);
            }
            UpdateContent::ChatBoost(data) => {
                f.handle_chat_boost(data);
            }
            UpdateContent::RemovedChatBoost(data) => {
                f.handle_removed_chat_boost(data);
            }
            UpdateContent::PurchasedPaidMedia(data) => {
                f.handle_purchased_paid_media(data);
            }
        });
    }
}

