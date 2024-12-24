use frankenstein::{BusinessConnection, BusinessMessagesDeleted, CallbackQuery, ChatBoostRemoved, ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, MessageReactionCountUpdated, MessageReactionUpdated, PaidMediaPurchased, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery};
use crate::plugin::context::EventContext;

pub trait EventListener {
    fn handle_message(&self, ctx: EventContext, data: Message) -> Option<()> {
        None
    }
    fn handle_edited_message(&self, ctx: EventContext, data: Message) -> Option<()> {
        None
    }
    fn handle_channel_post(&self, ctx: EventContext, data: Message) -> Option<()> {
        None
    }
    fn handle_edited_channel_post(&self, ctx: EventContext, data: Message) -> Option<()> {
        None
    }
    fn handle_business_connection(&self, ctx: EventContext, data: BusinessConnection) -> Option<()> {
        None
    }
    fn handle_business_message(&self, ctx: EventContext, data: Message) -> Option<()> {
        None
    }
    fn handle_edited_business_message(&self, ctx: EventContext, data: Message) -> Option<()> {
        None
    }
    fn handle_deleted_business_message(&self, ctx: EventContext, data: BusinessMessagesDeleted) -> Option<()> {
        None
    }
    fn handle_message_reaction(&self, ctx: EventContext, data: MessageReactionUpdated) -> Option<()> {
        None
    }
    fn handle_message_reaction_count(&self, ctx: EventContext, data: MessageReactionCountUpdated) -> Option<()> {
        None
    }
    fn handle_inline_query(&self, ctx: EventContext, data: InlineQuery) -> Option<()> {
        None
    }
    fn handle_chosen_inline_result(&self, ctx: EventContext, data: ChosenInlineResult) -> Option<()> {
        None
    }
    fn handle_callback_query(&self, ctx: EventContext, data: CallbackQuery) -> Option<()> {
        None
    }
    fn handle_shipping_query(&self, ctx: EventContext, data: ShippingQuery) -> Option<()> {
        None
    }
    fn handle_pre_checkout_query(&self, ctx: EventContext, data: PreCheckoutQuery) -> Option<()> {
        None
    }
    fn handle_poll(&self, ctx: EventContext, data: Poll) -> Option<()> {
        None
    }
    fn handle_poll_answer(&self, ctx: EventContext, data: PollAnswer) -> Option<()> {
        None
    }
    fn handle_my_chat_member(&self, ctx: EventContext, data: ChatMemberUpdated) -> Option<()> {
        None
    }
    fn handle_chat_member(&self, ctx: EventContext, data: ChatMemberUpdated) -> Option<()> {
        None
    }
    fn handle_chat_join_request(&self, ctx: EventContext, data: ChatJoinRequest) -> Option<()> {
        None
    }
    fn handle_chat_boost(&self, ctx: EventContext, data: ChatBoostUpdated) -> Option<()> {
        None
    }
    fn handle_removed_chat_boost(&self, ctx: EventContext, data: ChatBoostRemoved) -> Option<()> {
        None
    }
    fn handle_purchased_paid_media(&self, ctx: EventContext, data: PaidMediaPurchased) -> Option<()> {
        None
    }
}