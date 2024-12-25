use std::collections::HashMap;
use std::sync::Arc;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
use frankenstein::{Api, Update, UpdateContent};
use log::error;
use crate::{db, util};
use crate::db::Settings;
use crate::plugin::context::EventContext;
use crate::plugin::listener::EventListener;
use crate::plugin::plugin::Plugin;
use crate::plugin::PluginMetadata;

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
    plugins: HashMap<String, BotPlugin>,
    plugin_metadata: HashMap<String, PluginMetadata>,
    functions: HashMap<String, Box<dyn EventListener>>,
    pool: r2d2::Pool<ConnectionManager<PgConnection>>
}

impl PluginManager{
    pub fn new(pool: r2d2::Pool<ConnectionManager<PgConnection>>)-> Self{
        Self{
            functions: HashMap::new(),
            plugins: HashMap::new(),
            plugin_metadata: HashMap::new(),
            pool
        }
    }

    pub fn register_plugin(&mut self,plugin: Box<dyn Plugin>){
        let metadata = plugin.get_data();

        self.plugins.insert(metadata.key.clone(),
            BotPlugin{
                state: PluginState::REGISTERED,
                interface: plugin
            }
        );
        self.plugin_metadata.insert(metadata.key.clone(), metadata);
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
        self.functions.insert(plugin_identifier.to_owned(), handler);

        Ok(())
    }

    pub fn disable_plugin(&mut self, plugin_identifier: &str) -> anyhow::Result<()> {
        let plugin = self.plugins.get_mut(plugin_identifier).ok_or(anyhow::anyhow!("plugin '{plugin_identifier}' not found"))?;

        plugin.interface.on_disable();
        plugin.state = PluginState::DISABLED;
        self.functions.remove(plugin_identifier);

        Ok(())
    }

    pub fn call_event(&self, api: Arc<Api>, update: Update) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        let chat = util::get_chat_id_fom_update(update.clone());

        self.functions.iter().filter_map(|(module_key, f)| {
            let setting = Settings{
                chat_id: -1,
                module_identifier: module_key.to_owned(),
                enabled: false,
                configuration: Default::default(),
            };

            if chat.is_none() {
                return Some((setting, f))
            }

            let chat_id = chat.clone().unwrap();

            let setting_result = db::get_chat_module_setting(&mut connection, chat_id, module_key);

            match setting_result {
                Ok(setting) => {
                    Some((setting, f))
                }
                Err(error) => {
                    None
                }
            }
        }).for_each(|(setting, f)| {
            let metadata = self.plugin_metadata[setting.module_identifier.as_str()].clone();
            let ctx = EventContext::new(api.clone(), metadata, setting, self.pool.clone());
            match update.content.clone() {
                UpdateContent::Message(data) => {
                    f.handle_message(ctx, data);
                }
                UpdateContent::EditedMessage(data) => {
                    f.handle_edited_message(ctx, data);
                }
                UpdateContent::ChannelPost(data) => {
                    f.handle_channel_post(ctx, data);
                }
                UpdateContent::EditedChannelPost(data) => {
                    f.handle_edited_channel_post(ctx, data);
                }
                UpdateContent::BusinessConnection(data) => {
                    f.handle_business_connection(ctx, data);
                }
                UpdateContent::BusinessMessage(data) => {
                    f.handle_business_message(ctx, data);
                }
                UpdateContent::EditedBusinessMessage(data) => {
                    f.handle_edited_business_message(ctx, data);
                }
                UpdateContent::DeletedBusinessMessages(data) => {
                    f.handle_deleted_business_message(ctx, data);
                }
                UpdateContent::MessageReaction(data) => {
                    f.handle_message_reaction(ctx, data);
                }
                UpdateContent::MessageReactionCount(data) => {
                    f.handle_message_reaction_count(ctx, data);
                }
                UpdateContent::InlineQuery(data) => {
                    f.handle_inline_query(ctx, data);
                }
                UpdateContent::ChosenInlineResult(data) => {
                    f.handle_chosen_inline_result(ctx, data);
                }
                UpdateContent::CallbackQuery(data) => {
                    f.handle_callback_query(ctx, data);
                }
                UpdateContent::ShippingQuery(data) => {
                    f.handle_shipping_query(ctx, data);
                }
                UpdateContent::PreCheckoutQuery(data) => {
                    f.handle_pre_checkout_query(ctx, data);
                }
                UpdateContent::Poll(data) => {
                    f.handle_poll(ctx, data);
                }
                UpdateContent::PollAnswer(data) => {
                    f.handle_poll_answer(ctx, data);
                }
                UpdateContent::MyChatMember(data) => {
                    f.handle_my_chat_member(ctx, data);
                }
                UpdateContent::ChatMember(data) => {
                    f.handle_chat_member(ctx, data);
                }
                UpdateContent::ChatJoinRequest(data) => {
                    f.handle_chat_join_request(ctx, data);
                }
                UpdateContent::ChatBoost(data) => {
                    f.handle_chat_boost(ctx, data);
                }
                UpdateContent::RemovedChatBoost(data) => {
                    f.handle_removed_chat_boost(ctx, data);
                }
                UpdateContent::PurchasedPaidMedia(data) => {
                    f.handle_purchased_paid_media(ctx, data);
                }
            }
        });
        Ok(())
    }
}