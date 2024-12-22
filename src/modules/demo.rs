use dyn_clone::DynClone;
use frankenstein::Message;
use log::info;
use crate::event::EventListener;
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
    fn handle_message(&self, data: Message) -> Option<()> {
        info!("demo plugin received message: {:?}", data.text.unwrap_or("".to_string()));

        Some(())
    }
}