use dyn_clone::DynClone;
use crate::event;

pub struct PluginMetadata {
    pub key: String,
    pub name: String,
    pub description: String,
}

pub trait Plugin: DynClone {
    fn get_data(&self) -> PluginMetadata;

    fn on_load(&self);

    fn on_enable(&self) -> Box<dyn event::EventListener>;

    fn on_disable(&self);
}

dyn_clone::clone_trait_object!(Plugin);