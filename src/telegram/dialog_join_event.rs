use teloxide::Bot;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::types::{MessageId, User};

type JoinDialog = Dialogue<JoinEventValidation, InMemStorage<JoinEventValidation>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum JoinEventValidation {
    #[default]
    Start, // Welcome Message. Please Validate (whatever Strategy...?) (also disallow everything except text)
    ValidateCodeReceived {
        start_message_id: MessageId,
        correct_code: String
    }, // Validation Success :D Thanks... please tell something about yourself
    ReceiveInformation {
        request_information_message_id: MessageId,
    } // Exit and  Complete set free
}


async fn start(bot: Bot, dialogue: JoinDialog, msg: Message) -> HandlerResult {
    match msg.new_chat_members() {
        None => dialogue.exit(),
        Some(member) => {
            if member.len() == 0 {
                dialogue.exit();
                return Ok(());
            }

            let send_meg = bot.send_message(msg.chat.id, "Welcome to the group. Please write 123456").await?;
            dialogue.update(JoinEventValidation::ValidateCodeReceived {
                start_message_id: send_meg.id,
                correct_code: String::from("123456"),
            }).await?;
        }
    }

    Ok(())
}

async fn validate_code_received(bot: Bot, dialogue: JoinDialog, (start_message_id, correct_code): (MessageId, String), msg: Message) -> HandlerResult {
    match msg.text() {
        Some(content) => {
            if !content.contains(correct_code.as_str()) {
                bot.send_message(msg.chat.id, "Wrong code").await?;
                return Ok(());
            }

            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.edit_message_text(msg.chat.id, start_message_id, "Welcome to the group").await?;
            let send_msg = bot.send_message(msg.chat.id, "Please tell something about yourself!").await?;
            dialogue.update(JoinEventValidation::ReceiveInformation {
                request_information_message_id: send_msg.id
            }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }
    Ok(())
}

async fn information_received(bot: Bot, dialogue: JoinDialog, (request_information_message_id): (MessageId), msg: Message) -> HandlerResult {
    match msg.text() {
        Some(_) => {
            bot.delete_message(msg.chat.id, request_information_message_id).await?;
            dialogue.exit()?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }
    Ok(())
}

