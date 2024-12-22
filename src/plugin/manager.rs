use std::collections::HashMap;
use frankenstein::Update;
use crate::event;
use crate::plugin::plugin::Plugin;

#[derive(Clone)]
pub enum PluginState {
    REGISTERED,
    LOADED,
    ENABLED,
    DISABLED,
}

#[derive(Clone)]
pub struct BotPlugin {
    pub interface: Box<dyn Plugin>,
    pub state : PluginState,
}

pub struct PluginManager{
    event_manager: event::EventManager,
    plugins: HashMap<String, BotPlugin>,
}

impl PluginManager{
    pub fn new()-> Self{
        Self{
            event_manager: event::EventManager::new(),
            plugins: HashMap::new()
        }
    }

    pub fn register_plugin(&mut self,plugin: Box<dyn Plugin>){
        let metadata = plugin.get_data();

        self.plugins.insert(metadata.key,
            BotPlugin{
                state: PluginState::REGISTERED,
                interface: plugin
            }
        );
    }

    pub fn load_plugins(&mut self) -> anyhow::Result<()> {
        let plugins = self.plugins.clone();

        let result = plugins.iter()
            .map(|(identifier, _)| self.load_plugin(identifier))
            .filter_map(|result| result.err())
            .fold("".to_string(), |sum, line| format!("{sum}, {line}"));

        if result.is_empty() {
            return Ok(())
        }

        Err(anyhow::anyhow!("plugin errors: {result}"))
    }

    pub fn enable_plugins(&mut self) -> anyhow::Result<()> {
        let plugins = self.plugins.clone();

        let result = plugins.iter()
            .map(|(identifier, _)| self.enable_plugin(identifier))
            .filter_map(|result| result.err())
            .fold("".to_string(), |sum, line| format!("{sum}, {line}"));

        if result.is_empty() {
            return Ok(())
        }

        Err(anyhow::anyhow!("plugin errors: {result}"))
    }

    pub fn disable_plugins(&mut self) -> anyhow::Result<()> {
        let plugins = self.plugins.clone();

        let result = plugins.iter()
            .map(|(identifier, _)| self.disable_plugin(identifier))
            .filter_map(|result| result.err())
            .fold("".to_string(), |sum, line| format!("{sum}, {line}"));

        if result.is_empty() {
            return Ok(())
        }

        Err(anyhow::anyhow!("plugin errors: {result}"))
    }

    pub fn load_plugin(&mut self, plugin_identifier: &str) -> anyhow::Result<()> {
        let plugin = self.plugins.get_mut(plugin_identifier).ok_or(anyhow::anyhow!("plugin '{plugin_identifier}' not found"))?;

        plugin.interface.on_load();
        plugin.state = PluginState::LOADED;

        Ok(())
    }

    pub fn enable_plugin(&mut self, plugin_identifier: &str) -> anyhow::Result<()> {
        let plugin = self.plugins.get_mut(plugin_identifier).ok_or(anyhow::anyhow!("plugin '{plugin_identifier}' not found"))?;

        let handler = plugin.interface.on_enable();
        plugin.state = PluginState::ENABLED;
        self.event_manager.add_listener(plugin_identifier, handler);

        Ok(())
    }

    pub fn disable_plugin(&mut self, plugin_identifier: &str) -> anyhow::Result<()> {
        let plugin = self.plugins.get_mut(plugin_identifier).ok_or(anyhow::anyhow!("plugin '{plugin_identifier}' not found"))?;

        plugin.interface.on_disable();
        plugin.state = PluginState::DISABLED;

        Ok(())
    }

    pub fn call_event(&self, update: Update) -> anyhow::Result<()> {
        self.event_manager.call(update);

        Ok(())
    }
}