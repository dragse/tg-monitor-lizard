use dyn_clone::DynClone;
use frankenstein::{ChatId, Message, SendMessageParams, SendMessageParamsBuilder, TelegramApi};
use log::info;
use crate::plugin::{Command, EventContext, EventListener};
use crate::plugin;
use crate::plugin::PluginMetadata;

#[derive(Clone)]
pub struct DemoPlugin {

}

impl plugin::Plugin for DemoPlugin {
    fn get_data(&self) -> PluginMetadata {
        PluginMetadata{
            key: "demo_plugin".to_string(),
            name: "Demo Plugin".to_string(),
            description: "A Demo plugin to test everything".to_string(),
        }
    }

    fn get_cmd(&self) -> Vec<Command> {
        vec![]
    }

    fn on_load(&self) {
        info!("demo plugin initialized.");
    }

    fn on_enable(&self) -> Box<dyn EventListener> {
        info!("demo plugin enabled.");

        Box::new(DemoListener{})
    }

    fn on_disable(&self) {
        info!("demo plugin disabled.");
    }
}

struct DemoListener {

}

impl EventListener for DemoListener {
    fn handle_message(&self,ctx: EventContext, data: Message) -> Option<()> {
        info!("demo plugin received message: {:?}", data.text.clone().unwrap_or("".to_string()));

        let params = SendMessageParams::builder().chat_id(ChatId::Integer(data.chat.id)).text(data.text.unwrap_or("".to_string())).build();

        ctx.api.send_message(&params);

        Some(())
    }
}