use std::sync::Arc;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
use frankenstein::Api;
use crate::db;
use crate::db::{ModuleData, Settings};
use crate::plugin::PluginMetadata;

pub struct EventContext {
    pub api: Arc<Api>,
    metadata: PluginMetadata,
    setting: Settings,
    pool: r2d2::Pool<ConnectionManager<PgConnection>>
}

impl EventContext {
    pub fn new(api: Arc<Api>, metadata: PluginMetadata, setting: Settings, pool: r2d2::Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            api,
            metadata,
            setting,
            pool,
        }
    }

    pub fn save_data(&self, key: &str, value: serde_json::Value) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        let metadata = self.metadata.clone();

        db::upsert_module_data(&mut connection, &ModuleData {
            module_identifier: metadata.key,
            data_key: key.to_owned(),
            data: value,
        })?;

        Ok(())
    }

    pub fn get_data(&self, key: &str) -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::Value::String("".to_string()))
    }

    pub fn save_setting(&mut self, chat_id: i64, enabled: bool, config: serde_json::Value) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        let metadata = self.metadata.clone();
        let new_setting = Settings{
            chat_id,
            module_identifier: metadata.key,
            enabled,
            configuration: config,
        };

        db::upsert_module_settings(&mut connection, &new_setting)?;

        self.setting = new_setting;
        Ok(())
    }

    pub fn get_setting(&self, chat_id: i64) -> anyhow::Result<serde_json::Value> {
        Ok(self.setting.configuration.clone())
    }
}



