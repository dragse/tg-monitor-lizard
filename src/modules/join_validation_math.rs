use dyn_clone::DynClone;
use frankenstein::{ChatId, Message, SendMessageParams, SendMessageParamsBuilder, TelegramApi};
use log::info;
use crate::plugin::{Command, EventContext, EventListener};
use crate::plugin;
use crate::plugin::PluginMetadata;

#[derive(Clone)]
pub struct JoinValidationMathPlugin {

}

impl plugin::Plugin for JoinValidationMathPlugin {
    fn get_data(&self) -> PluginMetadata {
        PluginMetadata{
            key: "join_validation_math".to_string(),
            name: "Join Validator (math)".to_string(),
            description: "This Plugin validates every person, that joins the group".to_string(),
        }
    }

    fn get_cmd(&self) -> Vec<Command> {
        vec![]
    }

    fn on_load(&self) {
        info!("Join Validator (math) Plugin initialized.");
    }

    fn on_enable(&self) -> Box<dyn EventListener> {
        info!("Join Validator (math) Plugin enabled.");

        Box::new(JoinValidationMathListener{})
    }

    fn on_disable(&self) {
        info!("Join Validator (math) Plugin disabled.");
    }
}

struct JoinValidationMathListener {

}

impl EventListener for JoinValidationMathListener {
    fn handle_message(&self,ctx: EventContext, data: Message) -> Option<()> {
        if let Some(new_users) = data.new_chat_members {
            for new_user in new_users {
                //TODO Math captcha
            }
        }

        Some(())
    }
}

